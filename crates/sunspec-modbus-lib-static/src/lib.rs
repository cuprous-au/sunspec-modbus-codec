#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::c_void;
use core::slice;

#[cfg(not(feature = "std"))]
use core::{ffi::c_char, panic::PanicInfo};

use sunspec_modbus_lib_rs::buffer::{ReadableRegisterBuffer, WritableRegisterBuffer};
use sunspec_modbus_lib_rs::cursor::Cursor;
use sunspec_modbus_lib_rs::model::{CModel, ModelList};
use sunspec_modbus_lib_rs::{ModbusException, Sunspec};

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
/// `sunspec_service_*` return code: the adapter array is not the same length as the model map.
pub const SUNSPEC_RC_ADAPTER_COUNT_MISMATCH: i32 = -2;
/// `sunspec_service_*` return code: an adapter's `model` does not match the model map entry at
/// the same index (a misordered adapter array), or a map entry has a null `model`.
pub const SUNSPEC_RC_ADAPTER_MODEL_MISMATCH: i32 = -3;

/// One model in the device's register map: which model, and its repeat counts. Pure layout —
/// no adapters, so the array can be `static const` and built once.
///
/// Build an array in the order the device publishes the models and pass it, with a matching
/// [`SunspecAdapter`] array, to the service functions.
#[repr(C)]
pub struct SunspecModelBinding {
    /// The model's dispatch descriptor: the codec's `SUNSPEC_MODEL_<id>` static, e.g.
    /// `&SUNSPEC_MODEL_103`.
    pub model: *const CModel,
    /// First repeat count for repeating-group models; `0` for non-repeating models.
    pub repeat_count_0: u16,
    /// Second repeat count for models with two nested repeating groups; `0` otherwise.
    pub repeat_count_1: u16,
}

/// The adapter backing one model for a single request, supplied index-aligned with the
/// [`SunspecModelBinding`] array.
#[repr(C)]
pub struct SunspecAdapter {
    /// Must equal the `model` of the binding at the same index. This is the alignment guard:
    /// a mismatch (misordered or short adapter array) fails the request with
    /// [`SUNSPEC_RC_ADAPTER_MODEL_MISMATCH`] before any registers are touched.
    pub model: *const CModel,
    /// `SUNSPEC_ADAPTER_NONE` / `_STATEFUL` / `_CALLBACK`. Repeating-group models support only
    /// `_CALLBACK`.
    pub kind: u8,
    /// Pointer to the `Model<id>{Stateful,Callback}Adapter` selected by `kind`, or null when
    /// `kind` is `SUNSPEC_ADAPTER_NONE`.
    pub adapter: *mut c_void,
}

/// Checks that `adapters` is index-aligned with `bindings`: equal length, every `model`
/// matching, and no null map entries.
fn check_alignment(bindings: &[SunspecModelBinding], adapters: &[SunspecAdapter]) -> Result<(), i32> {
    if adapters.len() != bindings.len() {
        return Err(SUNSPEC_RC_ADAPTER_COUNT_MISMATCH);
    }
    for (binding, adapter) in bindings.iter().zip(adapters) {
        if binding.model.is_null() || adapter.model != binding.model {
            return Err(SUNSPEC_RC_ADAPTER_MODEL_MISMATCH);
        }
    }
    Ok(())
}

/// A [`ModelList`] view over a borrowed slice of C descriptors. Per-request adapters are
/// threaded through [`ModelList::ReadAdapters`] / [`ModelList::WriteAdapters`].
struct CModelList<'a> {
    bindings: &'a [SunspecModelBinding],
}

impl CModelList<'_> {
    /// Dispatch descriptor for `binding`.
    ///
    /// # Safety
    /// Every `binding.model` must be non-null — guaranteed by [`check_alignment`], which the
    /// service functions run before constructing a `CModelList`.
    unsafe fn descriptor(binding: &SunspecModelBinding) -> &CModel {
        unsafe { &*binding.model }
    }
}

impl ModelList for CModelList<'_> {
    type ReadAdapters<'a> = &'a [SunspecAdapter];
    type WriteAdapters<'a> = &'a [SunspecAdapter];

    fn map_length(&self) -> u16 {
        let mut total: u16 = 0;
        for binding in self.bindings {
            let model = unsafe { Self::descriptor(binding) };
            total = total
                .wrapping_add((model.length)(binding.repeat_count_0, binding.repeat_count_1));
        }
        total
    }

    fn traverse_read<'a>(
        &self,
        adapters: &'a [SunspecAdapter],
        cursor: &mut Cursor<ModbusException>,
        buffer: &mut WritableRegisterBuffer<'a>,
    ) {
        for (binding, adapter) in self.bindings.iter().zip(adapters) {
            let model = unsafe { Self::descriptor(binding) };
            unsafe {
                (model.visit_read)(
                    adapter.kind,
                    adapter.adapter,
                    binding.repeat_count_0,
                    binding.repeat_count_1,
                    cursor,
                    buffer,
                );
            }
        }
    }

    fn traverse_write<'a, 'buf>(
        &self,
        adapters: &'a [SunspecAdapter],
        cursor: &mut Cursor<ModbusException>,
        buffer: &ReadableRegisterBuffer<'buf>,
    ) {
        for (binding, adapter) in self.bindings.iter().zip(adapters) {
            let model = unsafe { Self::descriptor(binding) };
            unsafe {
                (model.visit_write)(
                    adapter.kind,
                    adapter.adapter,
                    binding.repeat_count_0,
                    binding.repeat_count_1,
                    cursor,
                    buffer,
                );
            }
        }
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
/// - `bindings` must point to `binding_count` valid, live [`SunspecModelBinding`]s, each
///   `model` pointing at a codec `SUNSPEC_MODEL_<id>` static.
/// - `read_adapters` must point to `adapter_count` valid [`SunspecAdapter`]s, index-aligned
///   with `bindings`, each `adapter` pointer matching its `kind`.
/// - `response_buffer` must be a valid, writable buffer of at least `length * 2` bytes.
/// - All pointers must remain valid for the duration of the call.
pub unsafe extern "C" fn sunspec_service_read_registers(
    bindings: *const SunspecModelBinding,
    binding_count: usize,
    read_adapters: *const SunspecAdapter,
    adapter_count: usize,
    address: u16,
    length: u16,
    response_buffer: *mut u8,
) -> i32 {
    if bindings.is_null() || read_adapters.is_null() || response_buffer.is_null() {
        return SUNSPEC_RC_NULL_ARG;
    }

    let bindings = unsafe { slice::from_raw_parts(bindings, binding_count) };
    let adapters = unsafe { slice::from_raw_parts(read_adapters, adapter_count) };
    if let Err(rc) = check_alignment(bindings, adapters) {
        return rc;
    }

    let buffer = unsafe { slice::from_raw_parts_mut(response_buffer, (length as usize) * 2) };

    let codec = Sunspec::new(CModelList { bindings });
    match codec.read_registers(address, buffer, adapters) {
        Ok(()) => SUNSPEC_RC_OK,
        Err(exception) => exception as i32,
    }
}

#[unsafe(no_mangle)]
/// Handle a SunSpec Modbus write of multiple holding registers, decoding into the relevant
/// models. A write that touches a block whose `kind` is `SUNSPEC_ADAPTER_NONE`, or a model
/// with no writable points, is rejected with `IllegalDataAddress`. Return codes as for
/// [`sunspec_service_read_registers`].
///
/// # Safety
/// - `bindings` must point to `binding_count` valid, live [`SunspecModelBinding`]s, each
///   `model` pointing at a codec `SUNSPEC_MODEL_<id>` static.
/// - `write_adapters` must point to `adapter_count` valid [`SunspecAdapter`]s, index-aligned
///   with `bindings`, each `adapter` pointer matching its `kind`.
/// - `request_buffer` must be a valid buffer of at least `length * 2` bytes.
/// - All pointers must remain valid for the duration of the call.
pub unsafe extern "C" fn sunspec_service_write_registers(
    bindings: *const SunspecModelBinding,
    binding_count: usize,
    write_adapters: *const SunspecAdapter,
    adapter_count: usize,
    address: u16,
    length: u16,
    request_buffer: *const u8,
) -> i32 {
    if bindings.is_null() || write_adapters.is_null() || request_buffer.is_null() {
        return SUNSPEC_RC_NULL_ARG;
    }

    let bindings = unsafe { slice::from_raw_parts(bindings, binding_count) };
    let adapters = unsafe { slice::from_raw_parts(write_adapters, adapter_count) };
    if let Err(rc) = check_alignment(bindings, adapters) {
        return rc;
    }

    let buffer = unsafe { slice::from_raw_parts(request_buffer, (length as usize) * 2) };

    let codec = Sunspec::new(CModelList { bindings });
    match codec.write_multiple_registers(address, buffer, adapters) {
        Ok(()) => SUNSPEC_RC_OK,
        Err(exception) => exception as i32,
    }
}
