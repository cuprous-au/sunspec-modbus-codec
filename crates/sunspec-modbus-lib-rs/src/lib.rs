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
    use core::ffi::CStr;

    use crate::sunspec::models::model_1::{self, Model1StatefulAdapter, ReadAdapter};

    use super::*;

    #[test]
    fn simple_common_adapter() -> Result<(), ModbusException> {
        let mut adapter = Model1StatefulAdapter {
            manufacturer: c_char_array!("Cuprous"),
            model: c_char_array!("Inverter 1"),
            options: c_char_array!("opt_a_b_c"),
            version: c_char_array!("v0.1"),
            serial_number: c_char_array!("I-1"),
            device_address: 0,
        };

        impl ModelList for model_1::Model1 {
            type ReadAdapters<'a> = &'a Model1StatefulAdapter;
        
            type WriteAdapters<'a> = &'a mut Model1StatefulAdapter;
        
            fn map_length(&self) -> u16 {
                self.model_length()
            }
        
            fn traverse_read(
                &self,
                adapter: Self::ReadAdapters<'_>,
                cursor: &mut cursor::Cursor<ModbusException>,
                buffer: &mut buffer::WritableRegisterBuffer<'_>,
            ) {
                visit_model_read(cursor, buffer, self, adapter);
            }
        
            fn traverse_write(
                &self,
                adapter: Self::WriteAdapters<'_>,
                cursor: &mut cursor::Cursor<ModbusException>,
                buffer: &buffer::ReadableRegisterBuffer<'_>,
            ) {
                visit_model_write(cursor, buffer, self, adapter);
            }
        }

        let sunspec = Sunspec::new(model_1::Model1);

        const WORDS_TO_READ: u16 = 72;

        let mut init_buf = [0_u8; WORDS_TO_READ as usize * 2];

        sunspec.read_registers(
            STARTING_REGISTER_OFFSET,
            init_buf.as_mut_slice(),
            &adapter,
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
            &mut adapter,
        )?;

        assert_eq!(ReadAdapter::device_address(&adapter), Some(1234));

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
}
