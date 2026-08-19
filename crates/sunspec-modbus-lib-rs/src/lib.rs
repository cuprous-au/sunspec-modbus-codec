#![no_std]
pub mod buffer;
pub mod cursor;
#[macro_use]
pub mod macros;
pub mod sunspec;

use crate::{
    buffer::{ReadableRegisterBuffer, WritableRegisterBuffer},
    sunspec::adapters::{SunspecAdapterProvider, traverse_adapters_read, traverse_adapters_write},
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

const STARTING_REGISTER_OFFSET: u16 = 40000;

pub fn read_registers<'a, 'b, B: Into<WritableRegisterBuffer<'b>>>(
    adapter_provider: &dyn SunspecAdapterProvider<'a>,
    address: u16,
    response_buffer: B,
) -> Result<(), ModbusException> {
    let mut buffer = response_buffer.into();
    let count = buffer.len();

    if address >= STARTING_REGISTER_OFFSET && address <= u16::MAX - count {
        if let Some(remainder_offset) = traverse_adapters_read(
            adapter_provider,
            &mut buffer.slice(0, count),
            address - STARTING_REGISTER_OFFSET,
            count,
        ) {
            buffer
                .slice(remainder_offset, count - remainder_offset)
                .fill(&[0xff, 0xff]);
        }
        Ok(())
    } else {
        Err(ModbusException::IllegalDataAddress)
    }
}

pub fn write_multiple_registers<'a, 'b, B: Into<ReadableRegisterBuffer<'b>>>(
    adapter_provider: &mut dyn SunspecAdapterProvider<'a>,
    address: u16,
    request_buffer: B,
) -> Result<(), ModbusException> {
    let buffer = request_buffer.into();
    let count = buffer.len();

    traverse_adapters_write(
        adapter_provider,
        &buffer,
        address - STARTING_REGISTER_OFFSET,
        count,
    )?;
    Ok(())
}

pub fn write_single_register<'a>(
    adapter_provider: &mut dyn SunspecAdapterProvider<'a>,
    address: u16,
    value: u16,
) -> Result<(), ModbusException> {
    let words: [u16; 1] = [value];
    let buffer = ReadableRegisterBuffer::from(&words[..]);

    traverse_adapters_write(
        adapter_provider,
        &buffer,
        address - STARTING_REGISTER_OFFSET,
        1,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use core::ffi::CStr;

    use crate::sunspec::{adapters::SunspecAdapters, models::model_1::Model1StatefulAdapter};

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

        let mut adapters = SunspecAdapters {
            model_1_adapter: Some(&mut adapter),
            ..SunspecAdapters::default()
        };

        const WORDS_TO_READ: u16 = 72;

        let mut init_buf = [0_u8; WORDS_TO_READ as usize * 2];

        read_registers(&adapters, STARTING_REGISTER_OFFSET, init_buf.as_mut_slice())?;

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

        write_single_register(&mut adapters, STARTING_REGISTER_OFFSET + 68, 1234)?;

        assert_eq!(
            adapters.model_1_adapter.as_ref().unwrap().device_address(),
            Some(1234)
        );

        let mut after_buf = [0_u8; 40];

        read_registers(
            &adapters,
            STARTING_REGISTER_OFFSET + 52,
            after_buf.as_mut_slice(),
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
