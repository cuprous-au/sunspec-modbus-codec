#![doc = include_str!("../README.md")]
#![no_std]
pub mod model_1_builder;
pub mod serialisation;
pub mod sunspec;

use core::ffi::{c_char, c_uint};

use heapless::Vec;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

use crate::{
    ModbusRequest::{ReadRegister, Unknown},
    sunspec::{
        PointType, ReadablePoint,
        models::{model_1, model_103},
        points::PointReference,
    },
};

#[repr(C)]
pub struct SunspecModelAdapters {
    pub model_1_adapter: Option<&'static model_1_builder::Model1Adapter>,
    // pub model_103_adapter: Option<&'static dyn model_103::ModelAdapter>,
}

#[repr(C)]
pub struct SunspecService {
    adapters: SunspecModelAdapters,
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

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}

impl SunspecService {
    pub fn new(adapters: SunspecModelAdapters) -> Self {
        Self { adapters: adapters }
    }

    pub fn handle_request<R: Into<ModbusRequest>>(
        &self,
        request: R,
        response_buffer: &mut [u16],
    ) -> Result<(), ModbusException> {
        unsafe {
            printf(c"handle_request\n".as_ptr());
        }
        let point_groups: Vec<(u16, &'static [ReadablePoint]), 3> =
            [(2, &HEADER_POINTS as &'static [ReadablePoint])]
                .into_iter()
                .chain(
                    [
                        self.adapters
                            .model_1_adapter
                            .map(|_| (model_1::SIZE, &model_1::POINTS as &'static [ReadablePoint])),
                        // self.adapters.model_103_adapter.map(|_| {
                        //     (
                        //         model_103::SIZE,
                        //         &model_103::POINTS as &'static [ReadablePoint],
                        //     )
                        // }),
                    ]
                    .into_iter()
                    .flatten(),
                )
                .collect();

        match request.into() {
            ReadRegister(address, count) => {
                if address >= 40000 {
                    let mut offset: u16 = address - 40000;
                    let mut words_written: u16 = 0;

                    for (size, points) in &point_groups {
                        if offset > *size {
                            offset -= *size;
                        } else {
                            for point in points.iter() {
                                unsafe {
                                    printf(
                                        c"goin through points %d\n".as_ptr(),
                                        words_written as c_uint,
                                    );
                                }
                                if offset < point.size {
                                    //write
                                    let bytes_to_write =
                                        core::cmp::min(point.size - offset, count - words_written);

                                    let buffer_slice = &mut response_buffer[words_written as usize
                                        ..(words_written + bytes_to_write) as usize];
                                    unsafe {
                                        printf(
                                            c"writing %d (offset %d)\n".as_ptr(),
                                            bytes_to_write as c_uint,
                                            offset as c_uint,
                                        );
                                        printf(
                                            c"slicing from %d to %d of length %d\n".as_ptr(),
                                            words_written as c_uint,
                                            (words_written + bytes_to_write) as c_uint,
                                            66 as c_uint,
                                        );
                                    }

                                    match &point.reference {
                                        PointReference::Static { value } => {
                                            response_buffer[words_written as usize] = value.to_be();
                                        }
                                        PointReference::Model1 { point } => {
                                            model_1::write_point(
                                                self.adapters.model_1_adapter.unwrap(),
                                                point,
                                                buffer_slice,
                                                offset,
                                                bytes_to_write,
                                            );
                                        }
                                        // PointReference::Model103 { point } => {
                                        //     model_103::write_point(
                                        //         self.adapters.model_103_adapter.unwrap(),
                                        //         point,
                                        //         buffer_slice,
                                        //         offset,
                                        //         bytes_to_write,
                                        //     );
                                        // }
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
