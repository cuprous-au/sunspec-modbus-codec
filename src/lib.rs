#![doc = include_str!("../README.md")]
#![no_std]
pub mod sunspec;
pub mod util;

use crate::{
    ModbusRequest::{ReadRegister, Unknown},
    sunspec::{model_1, model_103},
};

pub struct SunspecService {
    pub model_1_adapter: Option<&'static dyn model_1::ModelAdapter>,
    pub model_103_adapter: Option<&'static dyn model_103::ModelAdapter>,
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

impl SunspecService {
    pub fn handle_request<R: Into<ModbusRequest>>(
        &self,
        request: R,
        response_buffer: &mut [u8],
    ) -> Result<(), ModbusException> {
        match request.into() {
            ReadRegister(address, count) => {
                if address >= 40000 {
                    let mut offset: usize = (address - 40000) as usize * 2;
                    let buffer_bytes = (count as usize) * 2;
                    let mut bytes_written: usize = 0;
                    if offset == 0 {
                        response_buffer[..4].copy_from_slice("SunS".as_bytes());
                        bytes_written += 4;
                    } else {
                        offset -= 4;
                    }

                    if bytes_written >= buffer_bytes {
                        return Ok(());
                    }

                    if let Some(model_1_adapter) = self.model_1_adapter {
                        let size_bytes = (model_1::size() as usize) * 2;
                        if offset > size_bytes {
                            offset -= size_bytes;
                        } else {
                            for (len, writer) in model_1::register_readers() {
                                let available_bytes = len * 2;
                                if offset < available_bytes {
                                    //write
                                    let bytes_to_write = core::cmp::min(
                                        available_bytes,
                                        buffer_bytes - bytes_written,
                                    );

                                    writer(
                                        model_1_adapter,
                                        &mut response_buffer
                                            [bytes_written..bytes_written + bytes_to_write],
                                        offset,
                                        bytes_to_write,
                                    );
                                    bytes_written += available_bytes;
                                    offset = 0;
                                } else {
                                    offset -= available_bytes;
                                }

                                if bytes_written >= buffer_bytes {
                                    return Ok(());
                                }
                            }
                        }
                    };

                    if let Some(model_103_adapter) = self.model_103_adapter {
                        let size_bytes = (model_103::size() as usize) * 2;
                        if offset > size_bytes {
                            offset -= size_bytes;
                        } else {
                            for (len, writer) in model_103::register_readers() {
                                let available_bytes = len * 2;
                                if offset < available_bytes {
                                    //write
                                    let bytes_to_write = core::cmp::min(
                                        available_bytes,
                                        buffer_bytes - bytes_written,
                                    );

                                    writer(
                                        model_103_adapter,
                                        &mut response_buffer
                                            [bytes_written..bytes_written + bytes_to_write],
                                        offset,
                                        bytes_to_write,
                                    );
                                    bytes_written += available_bytes;
                                    offset = 0;
                                } else {
                                    offset -= available_bytes;
                                }

                                if bytes_written >= buffer_bytes {
                                    return Ok(());
                                }
                            }
                        }
                    };

                    if bytes_written < buffer_bytes {
                        response_buffer[bytes_written] = 0xff;
                        response_buffer[bytes_written + 1] = 0xff;
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
