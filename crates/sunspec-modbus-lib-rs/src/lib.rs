#![no_std]
pub mod buffer;
pub mod cursor;
#[macro_use]
pub mod macros;
pub mod sunspec;

use crate::{
    ModbusRequest::{ReadRegister, Unknown},
    buffer::ModbusBuffer,
    sunspec::adapters::{SunspecAdapterProvider, traverse_adapters},
};

pub enum ModbusRequest {
    ReadRegister(u16, u16),
    WriteSingleRegister(u16, u16),
    WriteMultipleRegisters(u16, u16),
    Unknown,
}

pub enum RegisterAction {
    ReadToBuffer,
    SetFromBuffer,
}

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

pub fn handle_request<'a, R: Into<ModbusRequest>, B: Into<ModbusBuffer<'a>>>(
    adapter_provider: &mut dyn SunspecAdapterProvider<'a>,
    request: R,
    response_buffer: B,
) -> Result<(), ModbusException> {
    match request.into() {
        ReadRegister(address, count) => {
            let mut buffer = response_buffer.into();
            if address >= STARTING_REGISTER_OFFSET && address <= u16::MAX - count {
                if let Some(remainder_offset) = traverse_adapters(
                    adapter_provider,
                    &mut buffer.slice(0, count),
                    address - STARTING_REGISTER_OFFSET,
                    count,
                    RegisterAction::ReadToBuffer,
                ) {
                    buffer
                        .slice(remainder_offset, count - remainder_offset)
                        .fill(&[0xff, 0xff])
                }
                Ok(())
            } else {
                Err(ModbusException::IllegalDataAddress)
            }
        }
        Unknown => Err(ModbusException::IllegalFunction),
        ModbusRequest::WriteMultipleRegisters(address, count) => {
            traverse_adapters(
                adapter_provider,
                &mut response_buffer.into(),
                address - STARTING_REGISTER_OFFSET,
                count,
                RegisterAction::SetFromBuffer,
            );
            Ok(())
        }
        ModbusRequest::WriteSingleRegister(address, value) => {
            let mut slice: [u16; 1] = [value];
            let slice_ref: &mut [u16] = &mut slice;
            let mut buffer = ModbusBuffer::from(slice_ref);

            traverse_adapters(
                adapter_provider,
                &mut buffer,
                address - STARTING_REGISTER_OFFSET,
                1,
                RegisterAction::SetFromBuffer,
            );
            Ok(())
        },
    }
}
