#![doc = include_str!("../README.md")]
#![no_std]
pub mod sunspec;
pub mod util;

use heapless::Vec;

use crate::{
    ModbusRequest::{ReadRegister, Unknown},
    sunspec::{
        core::{PointReference, PointType, ReadablePoint},
        model_1, model_103,
    },
};

pub struct SunspecServiceAdapters {
    pub model_1_adapter: Option<&'static dyn model_1::ModelAdapter>,
    pub model_103_adapter: Option<&'static dyn model_103::ModelAdapter>,
}

pub struct SunspecService {
    points: Vec<(u16, &'static [ReadablePoint]), 114>,
    adapters: SunspecServiceAdapters,
}

pub enum ModbusRequest {
    ReadRegister(u16, u16),
    Unknown,
}

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

impl SunspecService {
    pub fn new(adapters: SunspecServiceAdapters) -> Self {
        let mut points: Vec<(u16, &'static [ReadablePoint]), 114> = Vec::new();

        points.push((2, &HEADER_POINTS)).unwrap();

        if adapters.model_1_adapter.is_some() {
            points.push((model_1::SIZE, &model_1::POINTS)).unwrap();
        }

        Self {
            points: points,
            adapters: adapters,
        }
    }

    pub fn handle_request<R: Into<ModbusRequest>>(
        &self,
        request: R,
        response_buffer: &mut [u16],
    ) -> Result<(), ModbusException> {
        match request.into() {
            ReadRegister(address, count) => {
                if address >= 40000 {
                    let mut offset: u16 = address - 40000;
                    let mut words_written: u16 = 0;
                    for (size, points) in &self.points {
                        if offset > *size {
                            offset -= *size;
                        } else {
                            for point in points.iter() {
                                if offset < point.size {
                                    //write
                                    let bytes_to_write =
                                        core::cmp::min(point.size, count - words_written);

                                    match &point.reference {
                                        PointReference::Static { value } => {
                                            response_buffer[words_written as usize] = *value;
                                        }
                                        PointReference::Model1 { point } => {
                                            model_1::write_point(
                                                self.adapters.model_1_adapter.unwrap(),
                                                point,
                                                &mut response_buffer[words_written as usize
                                                    ..(words_written + bytes_to_write) as usize],
                                                offset,
                                                bytes_to_write,
                                            );
                                        }
                                        _ => (),
                                    }
                                    words_written += point.size;
                                    offset = 0;
                                } else {
                                    offset -= point.size;
                                }

                                if words_written >= count {
                                    return Ok(());
                                }
                            }
                        };
                    }

                    if words_written < count {
                        response_buffer[words_written as usize] = 0xffff;
                    };

                    Ok(())
                } else {
                    Err(ModbusException::IllegalDataAddress)
                }
            }
            Unknown => Err(ModbusException::IllegalFunction),
        }
    }
}
