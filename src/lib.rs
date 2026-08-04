#![doc = include_str!("../README.md")]
#![no_std]
pub mod external;
pub mod serialisation;
pub mod sunspec;
use core::{
    ffi::{c_uint},
    panic::PanicInfo,
};

use crate::{
    ModbusRequest::{ReadRegister, Unknown},
    sunspec::{
        PointType, ReadablePoint,
        adapters::{self, SunspecAdapterProvider, points_array_and_offset},
        points::PointReference,
    },
};

#[cfg(not(feature = "nopanic"))]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let message = panic_info.message().as_str().unwrap_or("Unknown Rust panic");
    unsafe { external::handle_panic(message.as_ptr() as *const i8) };
}

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

pub fn handle_request<'a, R: Into<ModbusRequest>>(
    adapter_provider: &'a dyn SunspecAdapterProvider<'a>,
    request: R,
    response_buffer: &mut [u16],
) -> Result<(), ModbusException> {
    unsafe {
        external::printf(c"handle_request\n".as_ptr());
    };

    match request.into() {
        ReadRegister(address, count) => {
            unsafe {
                external::printf(c"reading a register\n".as_ptr());
            }
            let mut words_written: u16 = 0;

            if address >= 40000 {
                while words_written < count && let Some((points, mut offset)) =
                    points_array_and_offset(adapter_provider, address + words_written - 40000)
                {
                    for point in points.iter() {
                        unsafe {
                            external::printf(
                                c"goin through points %d\n".as_ptr(),
                                words_written as c_uint,
                            );
                        }
                        if offset < point.size {
                            //write
                            let bytes_to_write =
                                core::cmp::min(point.size - offset, count - words_written);

                            let buffer_slice = &mut response_buffer
                                [words_written as usize..(words_written + bytes_to_write) as usize];
                            unsafe {
                                external::printf(
                                    c"writing %d (offset %d)\n".as_ptr(),
                                    bytes_to_write as c_uint,
                                    offset as c_uint,
                                );
                                external::printf(
                                    c"slicing from %d to %d of length %d\n".as_ptr(),
                                    words_written as c_uint,
                                    (words_written + bytes_to_write) as c_uint,
                                    66 as c_uint,
                                );
                            }

                            adapters::write_point(
                                adapter_provider,
                                &point.reference,
                                buffer_slice,
                                offset,
                                bytes_to_write,
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
                };

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
