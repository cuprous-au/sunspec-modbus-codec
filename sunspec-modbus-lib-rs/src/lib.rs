#![no_std]
pub mod buffer;
pub mod sunspec;

use crate::{
    ModbusRequest::{ReadRegister, Unknown},
    buffer::{ModbusBuffer, write_u16},
    sunspec::{
        PointType, ReadablePoint,
        adapters::{self, SunspecAdapterProvider, points_array_and_offset},
        points::PointReference,
    },
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

const HEADER_POINTS: [ReadablePoint; 2] = [
    ReadablePoint {
        reference: PointReference::Static { value: 0x5375 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0x6E53 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

pub fn handle_request<'a, R: Into<ModbusRequest>, B: Into<ModbusBuffer<'a>>>(
    adapter_provider: &'a dyn SunspecAdapterProvider<'a>,
    request: R,
    response_buffer: B,
) -> Result<(), ModbusException> {
    let mut buffer  = response_buffer.into();
    match request.into() {
        ReadRegister(address, count) => {
            let mut words_written: u16 = 0;

            if address >= 40000 {
                while words_written < count
                    && let Some((points, mut offset)) =
                        points_array_and_offset(adapter_provider, address + words_written - 40000)
                {
                    for point in points.iter() {
                        if offset < point.size {
                            //write
                            let limit = core::cmp::min(point.size - offset, count - words_written);

                            adapters::write_point(
                                adapter_provider,
                                &point.reference,
                                buffer.slice(words_written, limit),
                                offset,
                                limit,
                            );
                            words_written += point.size;
                            offset = 0;
                        } else {
                            offset -= point.size;
                        }

                        if words_written >= count {
                            return Ok(());
                        }
                    }
                }

                if words_written < count {
                    write_u16(0xffff, buffer.slice(words_written, 1))
                };
                Ok(())
            } else {
                Err(ModbusException::IllegalDataAddress)
            }
        }
        Unknown => Err(ModbusException::IllegalFunction),
    }
}
