#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::c_void;
use core::slice;

#[cfg(not(feature = "std"))]
use core::{ffi::c_char, panic::PanicInfo};

use sunspec_modbus_lib_rs::Sunspec;
use sunspec_modbus_lib_rs::model::{ModelList, StaticModelSpec};
use sunspec_modbus_lib_rs::sunspec::adapters::{ReadBinding, WriteBinding};

#[cfg(not(feature = "std"))]
unsafe extern "C" {
    pub fn handle_panic(message: *const c_char) -> !;
}

#[cfg(not(feature = "std"))]
const MAX_PANIC_MESSAGE_LENGTH: usize = 128;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let message = panic_info
        .message()
        .as_str()
        .unwrap_or("Unknown Rust panic");

    let mut c_str = [0; MAX_PANIC_MESSAGE_LENGTH + 1];

    for (char, c_char) in message
        .as_bytes()
        .iter()
        .take(MAX_PANIC_MESSAGE_LENGTH)
        .zip(c_str.iter_mut())
    {
        *c_char = (*char) as c_char;
    }
    unsafe { handle_panic(c_str.as_ptr()) }
}

/// [`SunspecAdapter::kind`]: no adapter for this direction; the block reads as `0xffff` and
/// rejects writes.
pub const SUNSPEC_ADAPTER_NONE: u8 = 0;
/// [`SunspecAdapter::kind`]: `adapter` points to a `Model<id>StatefulAdapter`.
pub const SUNSPEC_ADAPTER_STATEFUL: u8 = 1;
/// [`SunspecAdapter::kind`]: `adapter` points to a `Model<id>CallbackAdapter`.
pub const SUNSPEC_ADAPTER_CALLBACK: u8 = 2;

/// `sunspec_service_*` return code: success.
pub const SUNSPEC_RC_OK: i32 = 0;
/// `sunspec_service_*` return code: a required pointer argument was null.
pub const SUNSPEC_RC_NULL_ARG: i32 = -1;
/// `sunspec_service_*` return code: the adapter array length does not match the model map —
/// every model for a read, only the writable models for a write.
pub const SUNSPEC_RC_ADAPTER_COUNT_MISMATCH: i32 = -2;
/// `sunspec_service_*` return code: an adapter's `model` does not match the model map entry it
/// lines up with (a misordered adapter array), or a map entry has a null `model`.
pub const SUNSPEC_RC_ADAPTER_MODEL_MISMATCH: i32 = -3;

/// A C-safe representation of a Model specification
/// One model in the device's register map: which model, and its repeat counts. Pure layout —
/// no adapters, so the array can be `static const` and built once.
///
/// Build an array in the order the device publishes the models and pass it, with a matching
/// [`SunspecAdapter`] array, to the service functions.
#[repr(C)]
pub struct CModelSpec {
    /// The model's dispatch descriptor: the codec's `SUNSPEC_MODEL_<id>` static, e.g.
    /// `&SUNSPEC_MODEL_103`.
    pub model_spec: *const StaticModelSpec,
    /// First repeat count for repeating-group models; `0` for non-repeating models.
    pub repeat_count_0: u16,
    /// Second repeat count for models with two nested repeating groups; `0` otherwise.
    pub repeat_count_1: u16,
}

/// The adapter backing one model for a single request. Read requests supply one per model,
/// index-aligned with the [`CModelSpec`] array; write requests supply one per
/// writable model, index-aligned with the writable subsequence of that array.
#[repr(C)]
pub struct SunspecAdapter {
    /// Must equal the `model` of the model this entry lines up with. This is the alignment
    /// guard: a mismatch (misordered or short adapter array) fails the request with
    /// [`SUNSPEC_RC_ADAPTER_MODEL_MISMATCH`] before any registers are touched.
    pub model_spec: *const StaticModelSpec,
    /// `SUNSPEC_ADAPTER_NONE` / `_STATEFUL` / `_CALLBACK`. Repeating-group models support only
    /// `_CALLBACK`.
    pub kind: u8,
    /// Pointer to the `Model<id>{Stateful,Callback}Adapter` selected by `kind`, or null when
    /// `kind` is `SUNSPEC_ADAPTER_NONE`.
    pub adapter: *mut c_void,
}

/// Fails with [`SUNSPEC_RC_ADAPTER_MODEL_MISMATCH`] if any model has a null `model`.
fn check_no_null_models(models: &[CModelSpec]) -> Result<(), i32> {
    for model in models {
        if model.model_spec.is_null() {
            return Err(SUNSPEC_RC_ADAPTER_MODEL_MISMATCH);
        }
    }
    Ok(())
}

/// Checks that `adapters` is index-aligned with `models` for a read: no null map entries,
/// equal length, every `model` matching.
fn check_read_alignment(models: &[CModelSpec], adapters: &[SunspecAdapter]) -> Result<(), i32> {
    check_no_null_models(models)?;
    if adapters.len() != models.len() {
        return Err(SUNSPEC_RC_ADAPTER_COUNT_MISMATCH);
    }
    for (model, adapter) in models.iter().zip(adapters) {
        if adapter.model_spec != model.model_spec {
            return Err(SUNSPEC_RC_ADAPTER_MODEL_MISMATCH);
        }
    }
    Ok(())
}

/// As [`check_read_alignment`], but `adapters` covers only the writable models: a
/// non-writable model takes no write adapter, so the array is index-aligned with the
/// writable models, in map order.
fn check_write_alignment(models: &[CModelSpec], adapters: &[SunspecAdapter]) -> Result<(), i32> {
    check_no_null_models(models)?;
    // SAFETY: `check_no_null_models` above rejected every null `model`.
    let writable = || {
        models
            .iter()
            .filter(|model| unsafe { &*model.model_spec }.writable)
    };
    if adapters.len() != writable().count() {
        return Err(SUNSPEC_RC_ADAPTER_COUNT_MISMATCH);
    }
    for (model, adapter) in writable().zip(adapters) {
        if adapter.model_spec != model.model_spec {
            return Err(SUNSPEC_RC_ADAPTER_MODEL_MISMATCH);
        }
    }
    Ok(())
}

/// A [`ModelList`] view over a borrowed slice of C descriptors. Per-request adapters are
/// threaded through [`ModelList::ReadAdapters`] / [`ModelList::WriteAdapters`].
struct CModelList<'a> {
    models: &'a [CModelSpec],
}

impl CModelList<'_> {
    /// Dispatch descriptor for `model`.
    ///
    /// # Safety
    /// Every `model.model_spec` must be non-null — guaranteed by [`check_read_alignment`] /
    /// [`check_write_alignment`], which the service functions run before constructing a
    /// `CModelList`.
    unsafe fn descriptor(model: &CModelSpec) -> &StaticModelSpec {
        unsafe { &*model.model_spec }
    }
}

impl ModelList for CModelList<'_> {
    type ReadAdapters<'a>
        = &'a [SunspecAdapter]
    where
        Self: 'a;
    type WriteAdapters<'a>
        = &'a [SunspecAdapter]
    where
        Self: 'a;

    /// One [`ReadBinding::Extern`] per model, in map order — the codec drives each through
    /// its `StaticModelSpec` vtable. `adapters` is index-aligned with the models (checked by
    /// [`check_read_alignment`]), so each `kind` / pointer passes straight through.
    fn read_iter<'a>(
        &'a self,
        adapters: Self::ReadAdapters<'a>,
    ) -> impl Iterator<Item = ReadBinding<'a>> {
        self.models
            .iter()
            .zip(adapters)
            .map(|(model, adapter)| ReadBinding::Extern {
                // SAFETY: `check_read_alignment` rejected every null `model` before the
                // service function built this `CModelList`.
                descriptor: unsafe { Self::descriptor(model) },
                kind: adapter.kind,
                adapter: adapter.adapter,
                repeat_count_0: model.repeat_count_0,
                repeat_count_1: model.repeat_count_1,
            })
    }

    /// One [`WriteBinding::Extern`] per model, in map order. `adapters` carries an entry
    /// only for the writable models (see [`check_write_alignment`]); it is consumed in order
    /// for those, and every other block — plus any writable block past the end of a short
    /// array — gets `SUNSPEC_ADAPTER_NONE` and rejects the write.
    fn write_iter<'a>(
        &'a self,
        adapters: Self::WriteAdapters<'a>,
    ) -> impl Iterator<Item = WriteBinding<'a>> {
        let mut write_adapters = adapters.iter();
        self.models.iter().map(move |model| {
            // SAFETY: as for `read_iter`.
            let descriptor = unsafe { Self::descriptor(model) };
            let entry = if descriptor.writable {
                write_adapters.next()
            } else {
                None
            };
            let (kind, adapter) = match entry {
                Some(entry) => (entry.kind, entry.adapter),
                None => (SUNSPEC_ADAPTER_NONE, core::ptr::null_mut()),
            };
            WriteBinding::Extern {
                descriptor,
                kind,
                adapter,
                repeat_count_0: model.repeat_count_0,
                repeat_count_1: model.repeat_count_1,
            }
        })
    }
}

#[unsafe(no_mangle)]
/// Handle a SunSpec Modbus holding-register read and write the encoded response.
///
/// Registers past the end of the model map are filled with `0xffff`. Returns
/// [`SUNSPEC_RC_OK`], a negative `SUNSPEC_RC_*` argument error, or a positive Modbus
/// exception code (`0x01`..=`0x0B`) to reply with.
///
/// # Safety
/// - `models` must point to `model_count` valid, live [`CModelSpec`]s, each
///   `model` pointing at a codec `SUNSPEC_MODEL_<id>` static.
/// - `read_adapters` must point to `adapter_count` valid [`SunspecAdapter`]s, index-aligned
///   with `models`, each `adapter` pointer matching its `kind`.
/// - `response_buffer` must be a valid, writable buffer of at least `buffer_length * 2` bytes.
/// - All pointers must remain valid for the duration of the call.
pub unsafe extern "C" fn sunspec_read_registers(
    models: *const CModelSpec,
    model_count: usize,
    address: u16,
    response_buffer: *mut u8,
    buffer_length: u16,
    read_adapters: *const SunspecAdapter,
    adapter_count: usize,
) -> i32 {
    if models.is_null() || read_adapters.is_null() || response_buffer.is_null() {
        return SUNSPEC_RC_NULL_ARG;
    }

    let models = unsafe { slice::from_raw_parts(models, model_count) };
    let adapters = unsafe { slice::from_raw_parts(read_adapters, adapter_count) };
    if let Err(rc) = check_read_alignment(models, adapters) {
        return rc;
    }

    let buffer = unsafe { slice::from_raw_parts_mut(response_buffer, (buffer_length as usize) * 2) };

    let codec = Sunspec::new(CModelList { models });
    match codec.read_registers(address, buffer, adapters) {
        Ok(()) => SUNSPEC_RC_OK,
        Err(exception) => exception as i32,
    }
}

#[unsafe(no_mangle)]
/// Handle a SunSpec Modbus write of multiple holding registers, decoding into the relevant
/// models. A write that touches a block whose `kind` is `SUNSPEC_ADAPTER_NONE`, or a model
/// with no writable points, is rejected with `IllegalDataAddress`. Return codes as for
/// [`sunspec_read_registers`].
///
/// # Safety
/// - `models` must point to `model_count` valid, live [`CModelSpec`]s, each
///   `model` pointing at a codec `SUNSPEC_MODEL_<id>` static.
/// - `write_adapters` must point to `adapter_count` valid [`SunspecAdapter`]s, one per
///   **writable** model (a model whose `StaticModelSpec::writable` is set), index-aligned with the
///   writable subsequence of `models` and in the same order; each `adapter` pointer must
///   match its `kind`. Non-writable models take no entry.
/// - `request_buffer` must be a valid buffer of at least `buffer_length * 2` bytes.
/// - All pointers must remain valid for the duration of the call.
pub unsafe extern "C" fn sunspec_write_multiple_registers(
    models: *const CModelSpec,
    model_count: usize,
    address: u16,
    request_buffer: *const u8,
    buffer_length: u16,
    write_adapters: *const SunspecAdapter,
    adapter_count: usize,
) -> i32 {
    if models.is_null() || write_adapters.is_null() || request_buffer.is_null() {
        return SUNSPEC_RC_NULL_ARG;
    }

    let models = unsafe { slice::from_raw_parts(models, model_count) };
    let adapters = unsafe { slice::from_raw_parts(write_adapters, adapter_count) };
    if let Err(rc) = check_write_alignment(models, adapters) {
        return rc;
    }

    let buffer = unsafe { slice::from_raw_parts(request_buffer, (buffer_length as usize) * 2) };

    let codec = Sunspec::new(CModelList { models });
    match codec.write_multiple_registers(address, buffer, adapters) {
        Ok(()) => SUNSPEC_RC_OK,
        Err(exception) => exception as i32,
    }
}

#[unsafe(no_mangle)]
/// Handle a SunSpec Modbus write to a single holding register, decoding into the relevant
/// models. A write that touches a block whose `kind` is `SUNSPEC_ADAPTER_NONE`, or a model
/// with no writable points, is rejected with `IllegalDataAddress`. Return codes as for
/// [`sunspec_read_registers`].
///
/// # Safety
/// - `models` must point to `model_count` valid, live [`CModelSpec`]s, each
///   `model` pointing at a codec `SUNSPEC_MODEL_<id>` static.
/// - `write_adapters` must point to `adapter_count` valid [`SunspecAdapter`]s, one per
///   **writable** model (a model whose `StaticModelSpec::writable` is set), index-aligned with the
///   writable subsequence of `models` and in the same order; each `adapter` pointer must
///   match its `kind`. Non-writable models take no entry.
/// - All pointers must remain valid for the duration of the call.
pub unsafe extern "C" fn sunspec_write_single_register(
    models: *const CModelSpec,
    model_count: usize,
    address: u16,
    value: u16,
    write_adapters: *const SunspecAdapter,
    adapter_count: usize,
) -> i32 {
    if models.is_null() || write_adapters.is_null() {
        return SUNSPEC_RC_NULL_ARG;
    }

    let models = unsafe { slice::from_raw_parts(models, model_count) };
    let adapters = unsafe { slice::from_raw_parts(write_adapters, adapter_count) };
    if let Err(rc) = check_write_alignment(models, adapters) {
        return rc;
    }

    let codec = Sunspec::new(CModelList { models });
    match codec.write_single_register(address, value, adapters) {
        Ok(()) => SUNSPEC_RC_OK,
        Err(exception) => exception as i32,
    }
}
