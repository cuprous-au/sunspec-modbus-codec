#![no_std]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(auto_cfg))]

// `#[derive(ModelList)]`'s expansion refers to this crate by name (it has to: it's an external
// proc-macro crate with no other way to name the crate it's invoked from), which only resolves
// when the derive is used from a *different* crate. This self-alias makes it resolve here too,
// for the derive's own use in this crate's tests.
extern crate self as sunspec_modbus_lib_rs;

pub mod buffer;
pub mod cursor;
pub mod not_implemented;
pub mod sunspec;

use core::ffi::c_void;

use crate::buffer::{ReadableRegisterBuffer, WritableRegisterBuffer};
use crate::cursor::{Cursor, CursorResult};
use crate::sunspec::adapters::{ReadBinding, WriteBinding, read_model, write_model};
pub use sunspec_modbus_derive::ModelList;

/// Represents a standard Modbus error
///
/// Discriminant value (accessed by casting to u16) is the correct error code.
#[derive(Debug, Copy, Clone)]
#[repr(u16)]
pub enum ModbusException {
    IllegalFunction = 0x01,
    IllegalDataAddress = 0x02,
    IllegalDataValue = 0x03,
    ServerDeviceFailure = 0x04,
    Acknowledge = 0x05,
    ServerDeviceBusy = 0x06,
    MemoryParityError = 0x08,
    GatewayPathUnavailable = 0x0A,
    GatewayTargetDevice = 0x0B,
}

/// The conventional Modbus holding-register address a SunSpec register map begins at.
pub const DEFAULT_BASE_ADDRESS: u16 = 40000;

/// The `SunS` identifier that precedes every SunSpec model in the map.
const SUNS_HEADER_WORDS: u16 = 2;

/// The SunSpec end-model header: model id `0xFFFF` with a model length of `0`.
const SUNS_END_MODEL: [u8; 4] = [0xff, 0xff, 0x00, 0x00];
const SUNS_END_MODEL_WORDS: u16 = (SUNS_END_MODEL.len() / 2) as u16;

/// A single SunSpec model: how long its register block is, and how to encode/decode its
/// points against a model-specific read or write adapter.
///
/// Generated once per model by `sunspec-gen`. `ReadAdapter` / `WriteAdapter` are that
/// model's own adapter traits; a model with no writable points still has an (empty)
/// `WriteAdapter` and a `traverse_points_write` that rejects every address.
pub trait ModelSpec<'a> {
    /// The SunSpec model id (e.g. `1` for the common model).
    const MODEL_ID: u16;

    /// This model's read adapter trait. Bounded by `'a` so a request can pass an adapter
    /// that borrows non-`'static` data; the generated impls set it to `dyn ReadAdapter + 'a`.
    type ReadAdapter: ?Sized + 'a;

    /// This model's write adapter trait. Bounded by `'a` for the same reason as
    /// [`ReadAdapter`](ModelSpec::ReadAdapter).
    type WriteAdapter: ?Sized + 'a;

    /// Length of this model's register block, in words, excluding the `SunS` header but
    /// including the model id / length header words. For models with a repeating group
    /// this depends on the repeat count carried by `self`.
    fn model_length(&self) -> u16;

    /// Encode this model's points into `buffer`, starting `offset` words into the model. An
    /// optional point the adapter doesn't provide reads as its [`not_implemented`] value.
    fn traverse_points_read(
        &self,
        adapter: &Self::ReadAdapter,
        buffer: &mut WritableRegisterBuffer<'_>,
        offset: u16,
    ) -> Result<(), ModbusException>;

    /// Decode `buffer` (which starts `offset` words into the model) into `adapter`.
    fn traverse_points_write(
        &self,
        adapter: &mut Self::WriteAdapter,
        buffer: &ReadableRegisterBuffer<'_>,
        offset: u16,
    ) -> Result<(), ModbusException>;
}

/// An ordered, heterogeneous list of [`ModelSpec`]s describing one device's register map.
///
/// - [`ReadAdapters`](ModelList::ReadAdapters) — a ReadBinding binding per model in the list. Each will hold a reference
///   to a ReadAdapter implementation specific to that model.
/// - [`WriteAdapters`](ModelList::WriteAdapters) — a WriteBinding per model in the list. Only those for writeable models
///   will hold a mutable reference to a WriteAdapter implementation specific to that model.
pub trait ModelList {
    /// A set of ReadAdapters - one for each of the models defined in this ModelList.
    type ReadAdapters<'a>
    where
        Self: 'a;
    /// A set of WriteAdapters - one for each of the models defined in this ModelList.
    type WriteAdapters<'a>
    where
        Self: 'a;

    /// Total length in words of every model's block, model headers included - the register
    /// map's extent between the `SunS` identifier and the end model. Lets
    /// [`strict`](SunspecConfig::strict) mode bounds-check a request before touching any adapter.
    fn models_length(&self) -> u32;

    /// Traverse in order the models defined by this ModelList with the provided set of adapters, providing exactly one
    /// [ReadBinding] for each model.
    fn read_iter<'a>(
        &'a self,
        adapters: Self::ReadAdapters<'a>,
    ) -> impl Iterator<Item = ReadBinding<'a>>;

    /// Traverse in order the models defined by this ModelList with the provided set of adapters, providing exactly one
    /// [WriteBinding] for each model.
    fn write_iter<'a>(
        &'a self,
        adapters: Self::WriteAdapters<'a>,
    ) -> impl Iterator<Item = WriteBinding<'a>>;
}

/// C-FFI dispatch descriptor for one SunSpec model.
///
/// One `#[unsafe(no_mangle)] pub static SUNSPEC_MODEL_<id>: StaticModelSpec` is generated per
/// model into `sunspec-modbus-lib-static`'s mirrored module. A C `SunspecModelBinding` holds a
/// `*const StaticModelSpec` pointing at that static, so the service functions dispatch straight
/// through these function pointers with no model-id lookup.
///
/// The `dyn` adapters never cross the C boundary; the concrete `Model<id>CallbackAdapter`
/// pointer is cast back to a reference inside `visit_read` / `visit_write`.
pub struct StaticModelSpec {
    /// The SunSpec model id this descriptor dispatches, for diagnostics and wire cross-checks.
    pub id: u16,

    /// Register block length in words for the given repeat counts (model header included,
    /// `SunS` excluded). Repeat counts are ignored by non-repeating models.
    pub length: fn(repeat_count_0: u16, repeat_count_1: u16) -> u16,

    /// Whether this model has any writable points. A non-writable model rejects every write
    /// regardless of adapter; `sunspec-modbus-lib-static` uses this to let a C caller pass a
    /// `write_adapters` array covering only the writable models.
    pub writable: bool,

    /// Encode one model block on a read. A null `adapter` is a caller bug (it can't happen
    /// through `sunspec-modbus-lib-static`, which rejects them up front); a read touching the
    /// block then fails with [`ModbusException::ServerDeviceFailure`].
    ///
    /// # Safety
    /// `adapter` must be null or point to a live `Model<id>CallbackAdapter` for this model,
    /// valid for the duration of the call.
    pub visit_read: unsafe fn(
        adapter: *const c_void,
        repeat_count_0: u16,
        repeat_count_1: u16,
        cursor: &mut Cursor<ModbusException>,
        buffer: &mut WritableRegisterBuffer<'_>,
    ),

    /// Decode one model block on a write. A non-writable model ignores `adapter` and rejects
    /// the write with [`ModbusException::IllegalDataAddress`]; for a writable model, a null
    /// `adapter` is treated as for [`visit_read`](StaticModelSpec::visit_read).
    ///
    /// # Safety
    /// As for [`visit_read`](StaticModelSpec::visit_read), and `adapter` must be null or
    /// uniquely borrowable for the duration of the call.
    pub visit_write: unsafe fn(
        adapter: *mut c_void,
        repeat_count_0: u16,
        repeat_count_1: u16,
        cursor: &mut Cursor<ModbusException>,
        buffer: &ReadableRegisterBuffer<'_>,
    ),
}

/// Optional configuration for a [`Sunspec`] service.
///
/// Every setting has a default, so start from [`SunspecConfig::DEFAULT`] (or
/// [`Default::default`]) and override only what's needed:
///
/// ```ignore
/// const CONFIG: SunspecConfig = SunspecConfig::DEFAULT.with_base_address(50000);
/// ```
///
/// `#[repr(C)]` so `sunspec-modbus-lib-static` can take it straight from C callers.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
#[non_exhaustive]
pub struct SunspecConfig {
    /// The Modbus holding-register address of the `SunS` identifier that begins the map.
    /// Defaults to [`DEFAULT_BASE_ADDRESS`] (`40000`); SunSpec also allows `0` and `50000`.
    pub base_address: u16,
    /// Reject any request that reaches outside the register map - from the `SunS` identifier
    /// through the end model - and any write to the read-only `SunS` identifier or end model,
    /// with [`ModbusException::IllegalDataAddress`], before any adapter is called. Defaults to
    /// `true`.
    ///
    /// When `false`, registers read past the end of the map are filled with `0xffff`, and
    /// writes to the `SunS` identifier, end model, or past the end are ignored. Requests starting below
    /// [`base_address`](SunspecConfig::base_address) are rejected either way.
    pub strict: bool,
}

impl SunspecConfig {
    /// The default configuration, usable in `const` contexts.
    pub const DEFAULT: Self = Self {
        base_address: DEFAULT_BASE_ADDRESS,
        strict: true,
    };

    /// Set [`base_address`](SunspecConfig::base_address).
    pub const fn with_base_address(mut self, base_address: u16) -> Self {
        self.base_address = base_address;
        self
    }

    /// Set [`strict`](SunspecConfig::strict).
    pub const fn with_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }
}

impl Default for SunspecConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Which way a request goes, for [`Sunspec::check_bounds`].
enum Access {
    Read,
    Write,
}

/// A SunSpec register-map codec bound to a fixed [`ModelList`].
///
/// Build one from the ordered models the device exposes, then serve Modbus reads and
/// writes against it, passing the matching adapter tuple per request:
///
/// ```ignore
/// static SUNSPEC: Sunspec<(model_1::Model1, model_103::Model103)> =
///     Sunspec::new((model_1::Model1, model_103::Model103), SunspecConfig::DEFAULT);
///
/// SUNSPEC.read_registers(addr, &mut buf[..], (&common, &inverter))?;
/// SUNSPEC.write_multiple_registers(addr, req, (Some(&mut common), None))?;
/// ```
pub struct Sunspec<L> {
    models: L,
    config: SunspecConfig,
}

impl<L: ModelList> Sunspec<L> {
    /// Bind the codec to `models`. The list is fixed for the life of the value and is
    /// used identically for reads and writes. Pass [`SunspecConfig::DEFAULT`] for the
    /// standard behaviour.
    pub const fn new(models: L, config: SunspecConfig) -> Self {
        Self { models, config }
    }

    /// Encode a holding-register read of `response_buffer.len()` words starting at
    /// `address` into `response_buffer`. A read past the end of the model map is rejected in
    /// [`strict`](SunspecConfig::strict) mode, and otherwise filled with `0xffff`.
    pub fn read_registers<'a, B: Into<WritableRegisterBuffer<'a>>>(
        &'a self,
        address: u16,
        response_buffer: B,
        adapters: L::ReadAdapters<'a>,
    ) -> Result<(), ModbusException> {
        let mut buffer = response_buffer.into();
        let count = buffer.len();

        let offset = self.check_bounds(address, count, Access::Read)?;
        let mut cursor: Cursor<ModbusException> = Cursor::new(offset, count);

        let _ = cursor.visit_source_block(SUNS_HEADER_WORDS, |offset, from, len| {
            buffer.slice(from, len).write_string(c"SunS", offset);
            Ok(())
        });

        for model in self.models.read_iter(adapters) {
            read_model(model, &mut cursor, &mut buffer);
        }

        let _ = cursor.visit_source_block(SUNS_END_MODEL_WORDS, |offset, from, len| {
            buffer.slice(from, len).write_bytes(&SUNS_END_MODEL, offset);
            Ok(())
        });

        match cursor.result() {
            CursorResult::Error(exception) => Err(exception),
            CursorResult::Incomplete(remainder) => {
                buffer
                    .slice(remainder, count - remainder)
                    .fill(&[0xff, 0xff]);
                Ok(())
            }
            CursorResult::Complete => Ok(()),
        }
    }

    /// Decode a write of `request_buffer.len()` words starting at `address` into the
    /// relevant models. A write that touches a model with no writable points is rejected, as
    /// is (in
    /// [`strict`](SunspecConfig::strict) mode) one that touches the `SunS` identifier or the
    /// end model, or reaches past the end of the map.
    pub fn write_multiple_registers<'a, 'buf, B: Into<ReadableRegisterBuffer<'buf>>>(
        &'a self,
        address: u16,
        request_buffer: B,
        adapters: L::WriteAdapters<'a>,
    ) -> Result<(), ModbusException> {
        let buffer = request_buffer.into();
        let count = buffer.len();

        let offset = self.check_bounds(address, count, Access::Write)?;
        let mut cursor: Cursor<ModbusException> = Cursor::new(offset, count);

        let _ = cursor.visit_source_block(SUNS_HEADER_WORDS, |_, _, _| Ok(()));

        for model in self.models.write_iter(adapters) {
            write_model(model, &mut cursor, &buffer);
        }

        match cursor.result() {
            CursorResult::Error(exception) => Err(exception),
            CursorResult::Incomplete(_) | CursorResult::Complete => Ok(()),
        }
    }

    /// Offset of `address` from the base of the map, checking that the `count` words from it
    /// lie within the addressable range - and, in [`strict`](SunspecConfig::strict) mode,
    /// within the map itself for a read, or within the model blocks for a write (the `SunS`
    /// identifier and end model are read-only).
    fn check_bounds(
        &self,
        address: u16,
        count: u16,
        access: Access,
    ) -> Result<u16, ModbusException> {
        let base_address = self.config.base_address;
        let request_end = u32::from(address) + u32::from(count);
        if address < base_address || request_end > u32::from(u16::MAX) + 1 {
            return Err(ModbusException::IllegalDataAddress);
        }

        // Total model length must be u16 addressable
        if u32::from(base_address)
            + self.models.models_length()
            + u32::from(SUNS_HEADER_WORDS)
            + u32::from(SUNS_END_MODEL_WORDS)
            > u32::from(u16::MAX)
        {
            return Err(ModbusException::ServerDeviceFailure);
        };

        let offset = address - base_address;

        if self.config.strict {
            let models_end = SUNS_HEADER_WORDS + self.models.models_length() as u16;
            let (start, end) = match access {
                Access::Read => (0, models_end + SUNS_END_MODEL_WORDS),
                Access::Write => (SUNS_HEADER_WORDS, models_end),
            };
            let offset_end = u32::from(offset) + u32::from(count);
            if offset < start || offset_end > u32::from(end) {
                return Err(ModbusException::IllegalDataAddress);
            }
        }

        Ok(offset)
    }

    /// Decode a single-register write. Equivalent to [`write_multiple_registers`] with a
    /// one-word buffer.
    ///
    /// [`write_multiple_registers`]: Sunspec::write_multiple_registers
    pub fn write_single_register<'a>(
        &'a self,
        address: u16,
        value: u16,
        adapters: L::WriteAdapters<'a>,
    ) -> Result<(), ModbusException> {
        self.write_multiple_registers(address, [value].as_slice(), adapters)
    }
}

#[cfg(test)]
mod tests {
    use core::ffi::CStr;

    #[cfg(feature = "test-models")]
    use crate::sunspec::models::{model_701, model_704};
    use crate::sunspec::{
        adapters::{ReadBinding, WriteBinding},
        models::model_1,
    };

    use super::*;

    /// A plain-Rust stand-in for the common model (1) adapter, implementing
    /// [`model_1::ReadAdapter`]/[`model_1::WriteAdapter`] directly rather than through any
    /// library-provided helper - which is exactly how a Rust consumer of this crate is expected
    /// to back a model: `Model<id>CallbackAdapter`, the library's one concrete adapter, is
    /// `sunspec-modbus-lib-static`'s C-FFI type, not for Rust use.
    struct CommonModelAdapter {
        manufacturer: &'static CStr,
        model: &'static CStr,
        options: &'static CStr,
        version: &'static CStr,
        serial_number: &'static CStr,
        device_address: u16,
    }

    impl model_1::ReadAdapter for CommonModelAdapter {
        fn manufacturer(&self) -> &CStr {
            self.manufacturer
        }

        fn model(&self) -> &CStr {
            self.model
        }

        fn options(&self) -> Option<&CStr> {
            Some(self.options)
        }

        fn version(&self) -> Option<&CStr> {
            Some(self.version)
        }

        fn serial_number(&self) -> &CStr {
            self.serial_number
        }

        fn device_address(&self) -> Option<u16> {
            Some(self.device_address)
        }
    }

    impl model_1::WriteAdapter for CommonModelAdapter {
        fn set_device_address(&mut self, value: u16) {
            self.device_address = value;
        }
    }

    #[test]
    fn simple_common_adapter() -> Result<(), ModbusException> {
        struct SunspecModel {
            model: model_1::Model1,
        }
        let mut adapter = CommonModelAdapter {
            manufacturer: c"Cuprous",
            model: c"Inverter 1",
            options: c"opt_a_b_c",
            version: c"v0.1",
            serial_number: c"I-1",
            device_address: 0,
        };

        impl ModelList for SunspecModel {
            type ReadAdapters<'a> = &'a CommonModelAdapter;

            type WriteAdapters<'a> = &'a mut CommonModelAdapter;

            fn models_length(&self) -> u32 {
                self.model.model_length().into()
            }

            fn read_iter<'a>(
                &'a self,
                adapter: Self::ReadAdapters<'a>,
            ) -> impl Iterator<Item = ReadBinding<'a>> {
                Some(ReadBinding::Model1(&self.model, adapter)).into_iter()
            }

            fn write_iter<'a>(
                &'a self,
                adapter: Self::WriteAdapters<'a>,
            ) -> impl Iterator<Item = WriteBinding<'a>> {
                Some(WriteBinding::Model1(&self.model, adapter)).into_iter()
            }
        }

        let sunspec = Sunspec::new(
            SunspecModel {
                model: model_1::Model1,
            },
            SunspecConfig::DEFAULT,
        );

        const WORDS_TO_READ: u16 = 72;

        let mut init_buf = [0_u8; WORDS_TO_READ as usize * 2];

        sunspec.read_registers(DEFAULT_BASE_ADDRESS, init_buf.as_mut_slice(), &adapter)?;

        assert_eq!(&init_buf[..4], b"SunS");
        assert_eq!(
            u16::from_be_bytes(init_buf[4..6].try_into().expect("Unexpected slice length")),
            1
        );
        assert_eq!(
            u16::from_be_bytes(init_buf[6..8].try_into().expect("Unexpected slice length")),
            66
        );
        assert_eq!(CStr::from_bytes_until_nul(&init_buf[8..40]), Ok(c"Cuprous"));
        assert_eq!(
            CStr::from_bytes_until_nul(&init_buf[40..72]),
            Ok(c"Inverter 1")
        );
        assert_eq!(
            CStr::from_bytes_until_nul(&init_buf[72..88]),
            Ok(c"opt_a_b_c")
        );
        assert_eq!(CStr::from_bytes_until_nul(&init_buf[88..104]), Ok(c"v0.1"));
        assert_eq!(CStr::from_bytes_until_nul(&init_buf[104..136]), Ok(c"I-1"));
        assert_eq!(
            u16::from_be_bytes(
                init_buf[136..138]
                    .try_into()
                    .expect("Unexpected slice length")
            ),
            0
        );
        assert_eq!(
            u16::from_be_bytes(
                init_buf[138..140]
                    .try_into()
                    .expect("Unexpected slice length")
            ),
            crate::not_implemented::PAD
        );

        sunspec.write_multiple_registers(
            DEFAULT_BASE_ADDRESS + 68,
            [1234u16].as_slice(),
            &mut adapter,
        )?;

        assert_eq!(model_1::ReadAdapter::device_address(&adapter), Some(1234));

        let mut after_buf = [0_u8; 40];

        sunspec.read_registers(
            DEFAULT_BASE_ADDRESS + 52,
            after_buf.as_mut_slice(),
            &adapter,
        )?;

        assert_eq!(CStr::from_bytes_until_nul(&after_buf[0..32]), Ok(c"I-1"));
        assert_eq!(
            u16::from_be_bytes(
                after_buf[32..34]
                    .try_into()
                    .expect("Unexpected slice length")
            ),
            1234
        );
        Ok(())
    }

    #[test]
    fn configured_base_address_moves_the_map() -> Result<(), ModbusException> {
        struct SunspecModel {
            model: model_1::Model1,
        }

        impl ModelList for SunspecModel {
            type ReadAdapters<'a> = &'a CommonModelAdapter;

            type WriteAdapters<'a> = &'a mut CommonModelAdapter;

            fn models_length(&self) -> u32 {
                self.model.model_length().into()
            }

            fn read_iter<'a>(
                &'a self,
                adapter: Self::ReadAdapters<'a>,
            ) -> impl Iterator<Item = ReadBinding<'a>> {
                Some(ReadBinding::Model1(&self.model, adapter)).into_iter()
            }

            fn write_iter<'a>(
                &'a self,
                adapter: Self::WriteAdapters<'a>,
            ) -> impl Iterator<Item = WriteBinding<'a>> {
                Some(WriteBinding::Model1(&self.model, adapter)).into_iter()
            }
        }

        let mut adapter = CommonModelAdapter {
            manufacturer: c"Cuprous",
            model: c"Inverter 1",
            options: c"opt_a_b_c",
            version: c"v0.1",
            serial_number: c"I-1",
            device_address: 0,
        };

        const BASE_ADDRESS: u16 = 0;
        let sunspec = Sunspec::new(
            SunspecModel {
                model: model_1::Model1,
            },
            SunspecConfig::DEFAULT.with_base_address(BASE_ADDRESS),
        );

        let mut buf = [0_u8; 4];
        sunspec.read_registers(BASE_ADDRESS, buf.as_mut_slice(), &adapter)?;
        assert_eq!(&buf, b"SunS");

        sunspec.write_multiple_registers(BASE_ADDRESS + 68, [1234u16].as_slice(), &mut adapter)?;
        assert_eq!(model_1::ReadAdapter::device_address(&adapter), Some(1234));

        // The default base is now just an address past the end of the map.
        assert!(matches!(
            sunspec.read_registers(DEFAULT_BASE_ADDRESS, buf.as_mut_slice(), &adapter),
            Err(ModbusException::IllegalDataAddress)
        ));

        Ok(())
    }

    /// A map of just the common model: `SunS` (2) + model 1 (68) + end model (2) = 72 words.
    struct CommonOnly {
        model: model_1::Model1,
    }

    impl ModelList for CommonOnly {
        type ReadAdapters<'a> = &'a CommonModelAdapter;

        type WriteAdapters<'a> = &'a mut CommonModelAdapter;

        fn models_length(&self) -> u32 {
            self.model.model_length().into()
        }

        fn read_iter<'a>(
            &'a self,
            adapter: Self::ReadAdapters<'a>,
        ) -> impl Iterator<Item = ReadBinding<'a>> {
            Some(ReadBinding::Model1(&self.model, adapter)).into_iter()
        }

        fn write_iter<'a>(
            &'a self,
            adapter: Self::WriteAdapters<'a>,
        ) -> impl Iterator<Item = WriteBinding<'a>> {
            Some(WriteBinding::Model1(&self.model, adapter)).into_iter()
        }
    }

    const COMMON_ONLY_MAP_WORDS: u16 = 72;

    fn common_adapter() -> CommonModelAdapter {
        CommonModelAdapter {
            manufacturer: c"Cuprous",
            model: c"Inverter 1",
            options: c"opt_a_b_c",
            version: c"v0.1",
            serial_number: c"I-1",
            device_address: 0,
        }
    }

    #[test]
    fn strict_mode_rejects_reads_outside_the_map() -> Result<(), ModbusException> {
        let sunspec = Sunspec::new(
            CommonOnly {
                model: model_1::Model1,
            },
            SunspecConfig::DEFAULT,
        );
        let adapter = common_adapter();

        // The whole map, end model included, is readable.
        let mut whole = [0_u8; COMMON_ONLY_MAP_WORDS as usize * 2];
        sunspec.read_registers(DEFAULT_BASE_ADDRESS, whole.as_mut_slice(), &adapter)?;
        assert_eq!(whole[whole.len() - 4..], [0xff, 0xff, 0x00, 0x00]);

        // One word more straddles the end; a read wholly past it is out of bounds too.
        let mut straddling = [0_u8; (COMMON_ONLY_MAP_WORDS as usize + 1) * 2];
        assert!(matches!(
            sunspec.read_registers(DEFAULT_BASE_ADDRESS, straddling.as_mut_slice(), &adapter),
            Err(ModbusException::IllegalDataAddress)
        ));
        let mut past = [0_u8; 2];
        assert!(matches!(
            sunspec.read_registers(
                DEFAULT_BASE_ADDRESS + COMMON_ONLY_MAP_WORDS,
                past.as_mut_slice(),
                &adapter
            ),
            Err(ModbusException::IllegalDataAddress)
        ));

        Ok(())
    }

    #[test]
    fn strict_mode_rejects_writes_outside_the_map_before_writing() {
        let sunspec = Sunspec::new(
            CommonOnly {
                model: model_1::Model1,
            },
            SunspecConfig::DEFAULT,
        );
        let mut adapter = common_adapter();

        // Starts at the (writable) device address but runs past the end model.
        let request = [1234_u16; 5];
        assert!(matches!(
            sunspec.write_multiple_registers(
                DEFAULT_BASE_ADDRESS + 68,
                request.as_slice(),
                &mut adapter
            ),
            Err(ModbusException::IllegalDataAddress)
        ));
        assert_eq!(model_1::ReadAdapter::device_address(&adapter), Some(0));
    }

    #[test]
    fn strict_mode_rejects_writes_to_the_suns_identifier_and_end_model() {
        let sunspec = Sunspec::new(
            CommonOnly {
                model: model_1::Model1,
            },
            SunspecConfig::DEFAULT,
        );
        let mut adapter = common_adapter();

        for (address, words) in [
            // `SunS` alone, and running on into model 1.
            (DEFAULT_BASE_ADDRESS, 1),
            (DEFAULT_BASE_ADDRESS + 1, 2),
            // The end model alone.
            (DEFAULT_BASE_ADDRESS + COMMON_ONLY_MAP_WORDS - 2, 2),
        ] {
            let request = [1234_u16; 2];
            assert!(
                matches!(
                    sunspec.write_multiple_registers(address, &request[..words], &mut adapter),
                    Err(ModbusException::IllegalDataAddress)
                ),
                "write of {words} word(s) at {address}"
            );
        }
        assert!(matches!(
            sunspec.write_single_register(DEFAULT_BASE_ADDRESS, 0, &mut adapter),
            Err(ModbusException::IllegalDataAddress)
        ));
        assert_eq!(model_1::ReadAdapter::device_address(&adapter), Some(0));
    }

    #[test]
    fn lenient_mode_fills_reads_and_ignores_writes_outside_the_models()
    -> Result<(), ModbusException> {
        let sunspec = Sunspec::new(
            CommonOnly {
                model: model_1::Model1,
            },
            SunspecConfig::DEFAULT.with_strict(false),
        );
        let mut adapter = common_adapter();

        let mut straddling = [0_u8; (COMMON_ONLY_MAP_WORDS as usize + 1) * 2];
        sunspec.read_registers(DEFAULT_BASE_ADDRESS, straddling.as_mut_slice(), &adapter)?;
        assert_eq!(straddling[straddling.len() - 2..], [0xff, 0xff]);

        sunspec.write_multiple_registers(
            DEFAULT_BASE_ADDRESS + COMMON_ONLY_MAP_WORDS,
            [1234_u16; 2].as_slice(),
            &mut adapter,
        )?;
        assert_eq!(model_1::ReadAdapter::device_address(&adapter), Some(0));

        // ...as are writes to the `SunS` identifier and the end model.
        sunspec.write_multiple_registers(
            DEFAULT_BASE_ADDRESS,
            [1234_u16; 2].as_slice(),
            &mut adapter,
        )?;
        sunspec.write_multiple_registers(
            DEFAULT_BASE_ADDRESS + COMMON_ONLY_MAP_WORDS - 2,
            [1234_u16; 2].as_slice(),
            &mut adapter,
        )?;

        // Below the base address is out of range even when lenient.
        assert!(matches!(
            sunspec.read_registers(
                DEFAULT_BASE_ADDRESS - 1,
                straddling.as_mut_slice(),
                &adapter
            ),
            Err(ModbusException::IllegalDataAddress)
        ));

        Ok(())
    }

    /// Big-endian register `index` of an encoded buffer.
    #[cfg(feature = "test-models")]
    fn register(buffer: &[u8], index: usize) -> u16 {
        u16::from_be_bytes([buffer[index * 2], buffer[index * 2 + 1]])
    }

    /// Model 701 (DER AC measurement): only its mandatory `ACType` is provided, so every other
    /// point must read as its type's SunSpec "not implemented" value.
    #[test]
    #[cfg(feature = "test-models")]
    fn absent_optional_points_read_as_not_implemented() -> Result<(), ModbusException> {
        struct AcTypeOnly;

        impl model_701::ReadAdapter for AcTypeOnly {
            fn ac_wiring_type(&self) -> model_701::AcType {
                model_701::AcType::ThreePhase
            }
        }

        let model = model_701::Model701;
        let mut buf = [0xAA_u8; 155 * 2];
        model.traverse_points_read(&AcTypeOnly, &mut buf.as_mut_slice().into(), 0)?;

        assert_eq!(register(&buf, 0), 701);
        assert_eq!(register(&buf, 1), 153);
        assert_eq!(register(&buf, 2), 2, "ACType (enum16, provided)");
        assert_eq!(register(&buf, 3), 0xFFFF, "St (enum16)");
        assert_eq!(register(&buf, 6), 0xFFFF, "Alrm (bitfield32) high");
        assert_eq!(register(&buf, 7), 0xFFFF, "Alrm (bitfield32) low");
        assert_eq!(register(&buf, 10), 0x8000, "W (int16)");
        assert_eq!(register(&buf, 15), 0xFFFF, "LLV (uint16)");
        assert_eq!(
            [register(&buf, 17), register(&buf, 18)],
            [0xFFFF; 2],
            "Hz (uint32)"
        );
        assert!(
            (19..23).all(|i| register(&buf, i) == 0xFFFF),
            "TotWhInj (uint64)"
        );
        assert_eq!(register(&buf, 113), 0x8000, "A_SF (sunssf)");
        assert!(
            (123..155).all(|i| register(&buf, i) == 0),
            "MnAlrmInfo (string)"
        );

        // A read starting part-way into a multi-register point gets the rest of its value.
        let mut tail = [0_u8; 2 * 2];
        model.traverse_points_read(&AcTypeOnly, &mut tail.as_mut_slice().into(), 21)?;
        assert_eq!([register(&tail, 0), register(&tail, 1)], [0xFFFF; 2]);

        Ok(())
    }

    #[test]
    #[cfg(feature = "test-models")]
    fn write_model_704_sets_active_power_enable() -> Result<(), ModbusException> {
        #[derive(ModelList)]
        struct SunspecModel {
            model_1: model_1::Model1,
            model_701: model_701::Model701,
            model_704: model_704::Model704,
        }

        let mut common_model = CommonModelAdapter {
            manufacturer: c"Cuprous",
            model: c"Inverter 1",
            options: c"opt_a_b_c",
            version: c"v0.1",
            serial_number: c"I-1",
            device_address: 0,
        };

        struct DerAcControlsModel {
            active_power_enable: bool,
        }

        impl model_704::WriteAdapter for DerAcControlsModel {
            fn set_active_power_enable(&mut self, value: model_704::WSetEna) {
                self.active_power_enable = value == model_704::WSetEna::Enabled;
            }
        }

        let sunspec = Sunspec::new(
            SunspecModel {
                model_1: model_1::Model1,
                model_701: model_701::Model701,
                model_704: model_704::Model704,
            },
            SunspecConfig::DEFAULT,
        );

        let mut der_ac_controls = DerAcControlsModel {
            active_power_enable: false,
        };

        let mut adapters = SunspecModelWriteAdapters {
            model_1: &mut common_model,
            model_704: &mut der_ac_controls,
        };

        sunspec.write_multiple_registers(
            40247,
            hex::decode("0001").unwrap().as_slice(),
            &mut adapters,
        )?;

        assert!(der_ac_controls.active_power_enable);

        Ok(())
    }

    /// `#[derive(ModelList)]` on a tuple struct: `model_701` (no writable points) sits between
    /// the two writable models, so a correct `WriteAdapters` must renumber its fields around the
    /// writable subset (positions 0, 1) rather than reusing the original struct's positions
    /// (0, 2) - otherwise this would either fail to compile or write through the wrong adapter.
    #[test]
    #[cfg(feature = "test-models")]
    fn derived_tuple_struct_model_list_write_indexes_the_writable_subset() {
        #[derive(ModelList)]
        struct Trio(model_1::Model1, model_701::Model701, model_704::Model704);

        let model_list = Trio(model_1::Model1, model_701::Model701, model_704::Model704);

        let mut common_model = CommonModelAdapter {
            manufacturer: c"Cuprous",
            model: c"Inverter 1",
            options: c"opt_a_b_c",
            version: c"v0.1",
            serial_number: c"I-1",
            device_address: 0,
        };

        struct DerAcControlsModel {
            active_power_enable: bool,
        }

        impl model_704::WriteAdapter for DerAcControlsModel {
            fn set_active_power_enable(&mut self, value: model_704::WSetEna) {
                self.active_power_enable = value == model_704::WSetEna::Enabled;
            }
        }

        let mut der_ac_controls = DerAcControlsModel {
            active_power_enable: false,
        };

        let mut adapters = TrioWriteAdapters(&mut common_model, &mut der_ac_controls);
        let mut iter = model_list.write_iter(&mut adapters);
        match iter.next() {
            Some(WriteBinding::Model1(_, _)) => (),
            _ => panic!("Expected first binding to be model 1"),
        };
        match iter.next() {
            Some(WriteBinding::Model701(_)) => (),
            _ => panic!("Expected second binding to be model 701"),
        };
        match iter.next() {
            Some(WriteBinding::Model704(_, _)) => (),
            _ => panic!("Expected third binding to be model 704"),
        };
        assert!(iter.next().is_none());
    }

    /// Companion to the indexing test above: exercises the actual register write through a
    /// derived `ModelList`'s `Sunspec` wrapper, rather than just the binding order `write_iter`
    /// produces.
    #[test]
    #[cfg(feature = "test-models")]
    fn derived_tuple_struct_model_list_write_reaches_the_right_adapter()
    -> Result<(), ModbusException> {
        #[derive(ModelList)]
        struct Trio(model_1::Model1, model_701::Model701, model_704::Model704);

        let model_list = Trio(model_1::Model1, model_701::Model701, model_704::Model704);

        let mut common_model = CommonModelAdapter {
            manufacturer: c"Cuprous",
            model: c"Inverter 1",
            options: c"opt_a_b_c",
            version: c"v0.1",
            serial_number: c"I-1",
            device_address: 0,
        };

        struct DerAcControlsModel {
            active_power_enable: bool,
        }

        impl model_704::WriteAdapter for DerAcControlsModel {
            fn set_active_power_enable(&mut self, value: model_704::WSetEna) {
                self.active_power_enable = value == model_704::WSetEna::Enabled;
            }
        }

        let mut der_ac_controls = DerAcControlsModel {
            active_power_enable: false,
        };

        let sunspec = Sunspec::new(model_list, SunspecConfig::DEFAULT);
        sunspec.write_multiple_registers(
            40247,
            hex::decode("0001").unwrap().as_slice(),
            &mut TrioWriteAdapters(&mut common_model, &mut der_ac_controls),
        )?;

        assert!(der_ac_controls.active_power_enable);

        Ok(())
    }
}
