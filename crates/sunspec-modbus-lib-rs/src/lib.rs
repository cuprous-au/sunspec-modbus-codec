#![no_std]

// `#[derive(ModelList)]`'s expansion refers to this crate by name (it has to: it's an external
// proc-macro crate with no other way to name the crate it's invoked from), which only resolves
// when the derive is used from a *different* crate. This self-alias makes it resolve here too,
// for the derive's own use in this crate's tests.
extern crate self as sunspec_modbus_lib_rs;

pub mod buffer;
pub mod cursor;
#[macro_use]
pub mod macros;
pub mod model;
pub mod sunspec;

pub use crate::model::{ModelList, ModelSpec, STARTING_REGISTER_OFFSET, StaticModelSpec, Sunspec};
pub use sunspec_modbus_derive::ModelList;

#[derive(Debug, Copy, Clone)]
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

        let sunspec = Sunspec::new(SunspecModel {
            model: model_1::Model1,
        });

        const WORDS_TO_READ: u16 = 72;

        let mut init_buf = [0_u8; WORDS_TO_READ as usize * 2];

        sunspec.read_registers(STARTING_REGISTER_OFFSET, init_buf.as_mut_slice(), &adapter)?;

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

        sunspec.write_multiple_registers(
            STARTING_REGISTER_OFFSET + 68,
            [1234u16].as_slice(),
            &mut adapter,
        )?;

        assert_eq!(model_1::ReadAdapter::device_address(&adapter), Some(1234));

        let mut after_buf = [0_u8; 40];

        sunspec.read_registers(
            STARTING_REGISTER_OFFSET + 52,
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

        let sunspec = Sunspec::new(SunspecModel {
            model_1: model_1::Model1,
            model_701: model_701::Model701,
            model_704: model_704::Model704,
        });

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

        let sunspec = Sunspec::new(model_list);
        sunspec.write_multiple_registers(
            40247,
            hex::decode("0001").unwrap().as_slice(),
            &mut TrioWriteAdapters(&mut common_model, &mut der_ac_controls),
        )?;

        assert!(der_ac_controls.active_power_enable);

        Ok(())
    }
}
