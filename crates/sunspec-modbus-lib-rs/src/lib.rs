#![no_std]
pub mod buffer;
pub mod cursor;
#[macro_use]
pub mod macros;
pub mod model;
pub mod sunspec;

pub use crate::model::{
    CModel, ModelList, ModelSpec, STARTING_REGISTER_OFFSET, Sunspec, reject_model_write,
    visit_absent_read, visit_model_read, visit_model_write,
};

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
    use core::{cell::RefCell, ffi::CStr};

    use crate::sunspec::{
        adapters::{ReadAdapter, WriteAdapter},
        models::{
            model_1::{self, Model1StatefulAdapter},
            model_701, model_704,
        },
    };

    use super::*;

    #[test]
    fn simple_common_adapter() -> Result<(), ModbusException> {
        struct SunspecModel {
            model: model_1::Model1,
        }
        let adapter = RefCell::new(Model1StatefulAdapter {
            manufacturer: c_char_array!("Cuprous"),
            model: c_char_array!("Inverter 1"),
            options: c_char_array!("opt_a_b_c"),
            version: c_char_array!("v0.1"),
            serial_number: c_char_array!("I-1"),
            device_address: 0,
        });

        impl ModelList for SunspecModel {
            type ReadAdapters<'a> = &'a Model1StatefulAdapter;

            type WriteAdapters<'a> = &'a RefCell<Model1StatefulAdapter>;

            fn map_length(&self) -> u16 {
                self.model.model_length()
            }

            fn read_iter<'a>(
                &'a self,
                adapters: Self::ReadAdapters<'a>,
            ) -> impl Iterator<Item = ReadAdapter<'a>> {
                Some(ReadAdapter::Model1(&self.model, adapters)).into_iter()
            }

            fn write_iter<'a>(
                &'a self,
                adapter: Self::WriteAdapters<'a>,
            ) -> impl Iterator<Item = WriteAdapter<'a>> {
                Some(WriteAdapter::Model1(&self.model, adapter.borrow_mut())).into_iter()
            }
        }

        let sunspec = Sunspec::new(SunspecModel {
            model: model_1::Model1,
        });

        const WORDS_TO_READ: u16 = 72;

        let mut init_buf = [0_u8; WORDS_TO_READ as usize * 2];

        sunspec.read_registers(
            STARTING_REGISTER_OFFSET,
            init_buf.as_mut_slice(),
            &adapter.borrow(),
        )?;

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
            &adapter,
        )?;

        assert_eq!(
            model_1::ReadAdapter::device_address(&adapter.borrow() as &Model1StatefulAdapter),
            Some(1234)
        );

        let mut after_buf = [0_u8; 40];

        sunspec.read_registers(
            STARTING_REGISTER_OFFSET + 52,
            after_buf.as_mut_slice(),
            &adapter.borrow(),
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
    fn write_model_704_sets_active_power_enable() -> Result<(), ModbusException> {
        struct SunspecModel {
            model_1: model_1::Model1,
            model_701: model_701::Model701,
            model_704: model_704::Model704,
        }

        struct SunspecReadAdapters<'a> {
            model_1: &'a dyn model_1::ReadAdapter,
            model_701: &'a dyn model_701::ReadAdapter,
            model_704: &'a dyn model_704::ReadAdapter,
        }

        struct ReadAdapterIter<'a> {
            model: &'a SunspecModel,
            adapters: &'a SunspecReadAdapters<'a>,
            state: usize,
        }

        impl<'a> Iterator for ReadAdapterIter<'a> {
            type Item = ReadAdapter<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                let result = match self.state {
                    0 => Some(ReadAdapter::Model1(
                        &self.model.model_1,
                        self.adapters.model_1,
                    )),
                    1 => Some(ReadAdapter::Model701(
                        &self.model.model_701,
                        self.adapters.model_701,
                    )),
                    2 => Some(ReadAdapter::Model704(
                        &self.model.model_704,
                        self.adapters.model_704,
                    )),
                    _ => None,
                };
                self.state += 1;
                result
            }
        }

        struct SunspecWriteAdapters<'a> {
            model_1: &'a RefCell<dyn model_1::WriteAdapter>,
            model_704: &'a RefCell<dyn model_704::WriteAdapter>,
        }
        struct WriteAdapterIter<'a> {
            model: &'a SunspecModel,
            adapters: &'a SunspecWriteAdapters<'a>,
            state: usize,
        }
        impl<'a> Iterator for WriteAdapterIter<'a> {
            type Item = WriteAdapter<'a>;

            fn next(&mut self) -> Option<Self::Item> {
                let result: Option<WriteAdapter<'a>> = match self.state {
                    0 => Some(WriteAdapter::Model1(
                        &self.model.model_1,
                        self.adapters.model_1.borrow_mut(),
                    )),
                    1 => Some(WriteAdapter::Model701(&self.model.model_701)),
                    2 => Some(WriteAdapter::Model704(
                        &self.model.model_704,
                        self.adapters.model_704.borrow_mut(),
                    )),
                    _ => None,
                };
                self.state += 1;
                result
            }
        }

        impl ModelList for SunspecModel {
            type ReadAdapters<'a> = &'a SunspecReadAdapters<'a>;

            type WriteAdapters<'a> = &'a SunspecWriteAdapters<'a>;

            fn map_length(&self) -> u16 {
                self.model_1.model_length()
                    + self.model_701.model_length()
                    + self.model_704.model_length()
            }

            fn read_iter<'a>(
                &'a self,
                adapters: Self::ReadAdapters<'a>,
            ) -> impl Iterator<Item = ReadAdapter<'a>> {
                ReadAdapterIter {
                    model: self,
                    adapters,
                    state: 0,
                }
            }

            fn write_iter<'a>(
                &'a self,
                adapters: Self::WriteAdapters<'a>,
            ) -> impl Iterator<Item = WriteAdapter<'a>> {
                WriteAdapterIter {
                    model: self,
                    adapters,
                    state: 0,
                }
            }
        }

        let common_model = Model1StatefulAdapter {
            manufacturer: c_char_array!("Cuprous"),
            model: c_char_array!("Inverter 1"),
            options: c_char_array!("opt_a_b_c"),
            version: c_char_array!("v0.1"),
            serial_number: c_char_array!("I-1"),
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

        let der_ac_controls = RefCell::new(DerAcControlsModel {
            active_power_enable: false,
        });

        let mut adapters = SunspecWriteAdapters {
            model_1: &RefCell::new(common_model),
            model_704: &der_ac_controls,
        };

        sunspec.write_multiple_registers(
            40247,
            hex::decode("0001").unwrap().as_slice(),
            &mut adapters,
        )?;

        assert!(der_ac_controls.borrow().active_power_enable);

        Ok(())
    }
}
