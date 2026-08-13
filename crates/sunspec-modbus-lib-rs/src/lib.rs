#![no_std]
pub mod buffer;
pub mod cursor;
pub mod sunspec;

use crate::{
    ModbusRequest::{ReadRegister, Unknown},
    buffer::ModbusBuffer,
    sunspec::adapters::{SunspecAdapterProvider, read_into_buffer},
};

pub enum ModbusRequest {
    ReadRegister(u16, u16),
    Unknown,
}

#[derive(Debug)]
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
    adapter_provider: &'a dyn SunspecAdapterProvider<'a>,
    request: R,
    response_buffer: B,
) -> Result<(), ModbusException> {
    let mut buffer = response_buffer.into();
    match request.into() {
        ReadRegister(address, count) => {
            if address >= STARTING_REGISTER_OFFSET && address <= u16::MAX - count {
                read_into_buffer(
                    adapter_provider,
                    &mut buffer,
                    address - STARTING_REGISTER_OFFSET,
                    count,
                );
                Ok(())
            } else {
                Err(ModbusException::IllegalDataAddress)
            }
        }
        Unknown => Err(ModbusException::IllegalFunction),
    }
}
