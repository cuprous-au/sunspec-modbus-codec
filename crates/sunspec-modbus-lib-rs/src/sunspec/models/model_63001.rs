use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 154;

pub static POINTS: [ReadablePoint; 72] = [
    ReadablePoint {
        reference: PointReference::Static { value: 63001 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Sunssf1,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Sunssf2,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Sunssf3,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Sunssf4,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int161,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int162,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int163,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int164,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int165,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int16U,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint161,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint162,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint163,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint164,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint165,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint16U,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Acc16,
        },
        size: 1,
        data_type: PointType::Acc16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Acc16U,
        },
        size: 1,
        data_type: PointType::Acc16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Enum16,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Enum16U,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Bitfield16,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Bitfield16U,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int321,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int322,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int323,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int324,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int325,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int32U,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint321,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint322,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint323,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint324,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint325,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Uint32U,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Acc32,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Acc32U,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Enum32,
        },
        size: 2,
        data_type: PointType::Enum32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Enum32U,
        },
        size: 2,
        data_type: PointType::Enum32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Bitfield32,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Bitfield32U,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Ipaddr,
        },
        size: 2,
        data_type: PointType::Ipaddr,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::IpaddrU,
        },
        size: 2,
        data_type: PointType::Ipaddr,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int64,
        },
        size: 4,
        data_type: PointType::Int64,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Int64U,
        },
        size: 4,
        data_type: PointType::Int64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Acc64,
        },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Acc64U,
        },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Ipv6addr,
        },
        size: 8,
        data_type: PointType::Ipv6addr,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Ipv6addrU,
        },
        size: 8,
        data_type: PointType::Ipv6addr,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Float32,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Float32U,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::String,
        },
        size: 16,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::StringU,
        },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Sunssf5,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Sunssf6,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::Sunssf7,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingSunssf8,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingInt1611,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingInt1612,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingInt16U,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingUint1611,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingUint1612,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingUint1613,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingUint16U,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingInt32,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingInt32U,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingUint32,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingUint32U,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 {
            point: Point::RepeatingSunssf9,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    Sunssf1,
    Sunssf2,
    Sunssf3,
    Sunssf4,
    Int161,
    Int162,
    Int163,
    Int164,
    Int165,
    Int16U,
    Uint161,
    Uint162,
    Uint163,
    Uint164,
    Uint165,
    Uint16U,
    Acc16,
    Acc16U,
    Enum16,
    Enum16U,
    Bitfield16,
    Bitfield16U,
    Int321,
    Int322,
    Int323,
    Int324,
    Int325,
    Int32U,
    Uint321,
    Uint322,
    Uint323,
    Uint324,
    Uint325,
    Uint32U,
    Acc32,
    Acc32U,
    Enum32,
    Enum32U,
    Bitfield32,
    Bitfield32U,
    Ipaddr,
    IpaddrU,
    Int64,
    Int64U,
    Acc64,
    Acc64U,
    Ipv6addr,
    Ipv6addrU,
    Float32,
    Float32U,
    String,
    StringU,
    Sunssf5,
    Sunssf6,
    Sunssf7,
    RepeatingSunssf8,
    RepeatingInt1611,
    RepeatingInt1612,
    RepeatingInt16U,
    RepeatingUint1611,
    RepeatingUint1612,
    RepeatingUint1613,
    RepeatingUint16U,
    RepeatingInt32,
    RepeatingInt32U,
    RepeatingUint32,
    RepeatingUint32U,
    RepeatingSunssf9,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    154
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Sunssf1 => {
            if let Some(value) = model.sunssf_1() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Sunssf2 => {
            if let Some(value) = model.sunssf_2() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Sunssf3 => {
            if let Some(value) = model.sunssf_3() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Sunssf4 => {
            if let Some(value) = model.sunssf_4() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int161 => {
            if let Some(value) = model.int16_1() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int162 => {
            if let Some(value) = model.int16_2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int163 => {
            if let Some(value) = model.int16_3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int164 => {
            if let Some(value) = model.int16_4() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int165 => {
            if let Some(value) = model.int16_5() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int16U => {
            if let Some(value) = model.int16_u() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint161 => {
            if let Some(value) = model.uint16_1() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint162 => {
            if let Some(value) = model.uint16_2() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint163 => {
            if let Some(value) = model.uint16_3() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint164 => {
            if let Some(value) = model.uint16_4() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint165 => {
            if let Some(value) = model.uint16_5() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint16U => {
            if let Some(value) = model.uint16_u() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Acc16 => {
            if let Some(value) = model.acc16() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Acc16U => {
            if let Some(value) = model.acc16_u() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Enum16 => {
            if let Some(value) = model.enum16() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Enum16U => {
            if let Some(value) = model.enum16_u() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Bitfield16 => {
            if let Some(value) = model.bitfield16() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Bitfield16U => {
            if let Some(value) = model.bitfield16_u() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int321 => {
            if let Some(value) = model.int32_1() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int322 => {
            if let Some(value) = model.int32_2() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int323 => {
            if let Some(value) = model.int32_3() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int324 => {
            if let Some(value) = model.int32_4() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int325 => {
            if let Some(value) = model.int32_5() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int32U => {
            if let Some(value) = model.int32_u() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint321 => {
            if let Some(value) = model.uint32_1() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint322 => {
            if let Some(value) = model.uint32_2() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint323 => {
            if let Some(value) = model.uint32_3() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint324 => {
            if let Some(value) = model.uint32_4() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint325 => {
            if let Some(value) = model.uint32_5() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Uint32U => {
            if let Some(value) = model.uint32_u() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Acc32 => {
            if let Some(value) = model.acc32() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Acc32U => {
            if let Some(value) = model.acc32_u() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Enum32 => {
            if let Some(value) = model.enum32() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Enum32U => {
            if let Some(value) = model.enum32_u() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Bitfield32 => {
            if let Some(value) = model.bitfield32() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Bitfield32U => {
            if let Some(value) = model.bitfield32_u() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Ipaddr => {
            if let Some(value) = model.ipaddr() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IpaddrU => {
            if let Some(value) = model.ipaddr_u() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int64 => {
            if let Some(value) = model.int64() {
                buffer::write_i64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Int64U => {
            if let Some(value) = model.int64_u() {
                buffer::write_i64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Acc64 => {
            if let Some(value) = model.acc64() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Acc64U => {
            if let Some(value) = model.acc64_u() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Ipv6addr => {
            if let Some(value) = model.ipv6addr() {
                buffer::write_ipv6_addr(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Ipv6addrU => {
            if let Some(value) = model.ipv6addr_u() {
                buffer::write_ipv6_addr(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Float32 => {
            if let Some(value) = model.float32() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Float32U => {
            if let Some(value) = model.float32_u() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::String => {
            if let Some(value) = model.string() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringU => {
            if let Some(value) = model.string_u() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Sunssf5 => {
            if let Some(value) = model.sunssf_5() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Sunssf6 => {
            if let Some(value) = model.sunssf_6() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Sunssf7 => {
            if let Some(value) = model.sunssf_7() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingSunssf8 => {
            if let Some(value) = model.repeating_sunssf_8() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingInt1611 => {
            if let Some(value) = model.repeating_int16_11() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingInt1612 => {
            if let Some(value) = model.repeating_int16_12() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingInt16U => {
            if let Some(value) = model.repeating_int16_u() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingUint1611 => {
            if let Some(value) = model.repeating_uint16_11() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingUint1612 => {
            if let Some(value) = model.repeating_uint16_12() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingUint1613 => {
            if let Some(value) = model.repeating_uint16_13() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingUint16U => {
            if let Some(value) = model.repeating_uint16_u() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingInt32 => {
            if let Some(value) = model.repeating_int32() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingInt32U => {
            if let Some(value) = model.repeating_int32_u() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingUint32 => {
            if let Some(value) = model.repeating_uint32() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingUint32U => {
            if let Some(value) = model.repeating_uint32_u() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingSunssf9 => {
            if let Some(value) = model.repeating_sunssf_9() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    fn sunssf_1(&self) -> Option<u16> {
        None
    }

    fn sunssf_2(&self) -> Option<u16> {
        None
    }

    fn sunssf_3(&self) -> Option<u16> {
        None
    }

    fn sunssf_4(&self) -> Option<u16> {
        None
    }

    fn int16_1(&self) -> Option<i16> {
        None
    }

    fn int16_2(&self) -> Option<i16> {
        None
    }

    fn int16_3(&self) -> Option<i16> {
        None
    }

    fn int16_4(&self) -> Option<i16> {
        None
    }

    fn set_int16_4(&mut self, value: i16) {}

    fn int16_5(&self) -> Option<i16> {
        None
    }

    fn int16_u(&self) -> Option<i16> {
        None
    }

    fn uint16_1(&self) -> Option<u16> {
        None
    }

    fn uint16_2(&self) -> Option<u16> {
        None
    }

    fn uint16_3(&self) -> Option<u16> {
        None
    }

    fn uint16_4(&self) -> Option<u16> {
        None
    }

    fn set_uint16_4(&mut self, value: u16) {}

    fn uint16_5(&self) -> Option<u16> {
        None
    }

    fn uint16_u(&self) -> Option<u16> {
        None
    }

    fn acc16(&self) -> Option<u16> {
        None
    }

    fn acc16_u(&self) -> Option<u16> {
        None
    }

    fn enum16(&self) -> Option<u16> {
        None
    }

    fn enum16_u(&self) -> Option<u16> {
        None
    }

    fn bitfield16(&self) -> Option<u16> {
        None
    }

    fn bitfield16_u(&self) -> Option<u16> {
        None
    }

    fn int32_1(&self) -> Option<i32> {
        None
    }

    fn int32_2(&self) -> Option<i32> {
        None
    }

    fn int32_3(&self) -> Option<i32> {
        None
    }

    fn set_int32_3(&mut self, value: i32) {}

    fn int32_4(&self) -> Option<i32> {
        None
    }

    fn int32_5(&self) -> Option<i32> {
        None
    }

    fn int32_u(&self) -> Option<i32> {
        None
    }

    fn uint32_1(&self) -> Option<u32> {
        None
    }

    fn uint32_2(&self) -> Option<u32> {
        None
    }

    fn uint32_3(&self) -> Option<u32> {
        None
    }

    fn set_uint32_3(&mut self, value: u32) {}

    fn uint32_4(&self) -> Option<u32> {
        None
    }

    fn uint32_5(&self) -> Option<u32> {
        None
    }

    fn uint32_u(&self) -> Option<u32> {
        None
    }

    fn acc32(&self) -> Option<u32> {
        None
    }

    fn acc32_u(&self) -> Option<u32> {
        None
    }

    fn enum32(&self) -> Option<u32> {
        None
    }

    fn enum32_u(&self) -> Option<u32> {
        None
    }

    fn bitfield32(&self) -> Option<u32> {
        None
    }

    fn bitfield32_u(&self) -> Option<u32> {
        None
    }

    fn ipaddr(&self) -> Option<u32> {
        None
    }

    fn set_ipaddr(&mut self, value: u32) {}

    fn ipaddr_u(&self) -> Option<u32> {
        None
    }

    fn int64(&self) -> Option<i64> {
        None
    }

    fn set_int64(&mut self, value: i64) {}

    fn int64_u(&self) -> Option<i64> {
        None
    }

    fn acc64(&self) -> Option<u64> {
        None
    }

    fn acc64_u(&self) -> Option<u64> {
        None
    }

    fn ipv6addr(&self) -> Option<&[u16; 8]> {
        None
    }

    fn ipv6addr_u(&self) -> Option<&[u16; 8]> {
        None
    }

    fn float32(&self) -> Option<f32> {
        None
    }

    fn set_float32(&mut self, value: f32) {}

    fn float32_u(&self) -> Option<f32> {
        None
    }

    fn string(&self) -> Option<&CStr> {
        None
    }

    fn set_string(&mut self, value: &CStr) {}

    fn string_u(&self) -> Option<&CStr> {
        None
    }

    fn sunssf_5(&self) -> Option<u16> {
        None
    }

    fn sunssf_6(&self) -> Option<u16> {
        None
    }

    fn sunssf_7(&self) -> Option<u16> {
        None
    }

    fn repeating_sunssf_8(&self) -> Option<u16> {
        None
    }

    fn repeating_int16_11(&self) -> Option<i16> {
        None
    }

    fn set_repeating_int16_11(&mut self, value: i16) {}

    fn repeating_int16_12(&self) -> Option<i16> {
        None
    }

    fn repeating_int16_u(&self) -> Option<i16> {
        None
    }

    fn repeating_uint16_11(&self) -> Option<u16> {
        None
    }

    fn set_repeating_uint16_11(&mut self, value: u16) {}

    fn repeating_uint16_12(&self) -> Option<u16> {
        None
    }

    fn repeating_uint16_13(&self) -> Option<u16> {
        None
    }

    fn repeating_uint16_u(&self) -> Option<u16> {
        None
    }

    fn repeating_int32(&self) -> Option<i32> {
        None
    }

    fn set_repeating_int32(&mut self, value: i32) {}

    fn repeating_int32_u(&self) -> Option<i32> {
        None
    }

    fn repeating_uint32(&self) -> Option<u32> {
        None
    }

    fn set_repeating_uint32(&mut self, value: u32) {}

    fn repeating_uint32_u(&self) -> Option<u32> {
        None
    }

    fn repeating_sunssf_9(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model63001CallbackAdapter {
    context: *mut c_void,
    sunssf_1_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sunssf_2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sunssf_3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sunssf_4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    int16_1_callback: Option<extern "C" fn(*const c_void) -> i16>,
    int16_2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    int16_3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    int16_4_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_int16_4_callback: Option<extern "C" fn(i16, *mut c_void)>,
    int16_5_callback: Option<extern "C" fn(*const c_void) -> i16>,
    int16_u_callback: Option<extern "C" fn(*const c_void) -> i16>,
    uint16_1_callback: Option<extern "C" fn(*const c_void) -> u16>,
    uint16_2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    uint16_3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    uint16_4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_uint16_4_callback: Option<extern "C" fn(u16, *mut c_void)>,
    uint16_5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    uint16_u_callback: Option<extern "C" fn(*const c_void) -> u16>,
    acc16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    acc16_u_callback: Option<extern "C" fn(*const c_void) -> u16>,
    enum16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    enum16_u_callback: Option<extern "C" fn(*const c_void) -> u16>,
    bitfield16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    bitfield16_u_callback: Option<extern "C" fn(*const c_void) -> u16>,
    int32_1_callback: Option<extern "C" fn(*const c_void) -> i32>,
    int32_2_callback: Option<extern "C" fn(*const c_void) -> i32>,
    int32_3_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_int32_3_callback: Option<extern "C" fn(i32, *mut c_void)>,
    int32_4_callback: Option<extern "C" fn(*const c_void) -> i32>,
    int32_5_callback: Option<extern "C" fn(*const c_void) -> i32>,
    int32_u_callback: Option<extern "C" fn(*const c_void) -> i32>,
    uint32_1_callback: Option<extern "C" fn(*const c_void) -> u32>,
    uint32_2_callback: Option<extern "C" fn(*const c_void) -> u32>,
    uint32_3_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_uint32_3_callback: Option<extern "C" fn(u32, *mut c_void)>,
    uint32_4_callback: Option<extern "C" fn(*const c_void) -> u32>,
    uint32_5_callback: Option<extern "C" fn(*const c_void) -> u32>,
    uint32_u_callback: Option<extern "C" fn(*const c_void) -> u32>,
    acc32_callback: Option<extern "C" fn(*const c_void) -> u32>,
    acc32_u_callback: Option<extern "C" fn(*const c_void) -> u32>,
    enum32_callback: Option<extern "C" fn(*const c_void) -> u32>,
    enum32_u_callback: Option<extern "C" fn(*const c_void) -> u32>,
    bitfield32_callback: Option<extern "C" fn(*const c_void) -> u32>,
    bitfield32_u_callback: Option<extern "C" fn(*const c_void) -> u32>,
    ipaddr_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_ipaddr_callback: Option<extern "C" fn(u32, *mut c_void)>,
    ipaddr_u_callback: Option<extern "C" fn(*const c_void) -> u32>,
    int64_callback: Option<extern "C" fn(*const c_void) -> i64>,
    set_int64_callback: Option<extern "C" fn(i64, *mut c_void)>,
    int64_u_callback: Option<extern "C" fn(*const c_void) -> i64>,
    acc64_callback: Option<extern "C" fn(*const c_void) -> u64>,
    acc64_u_callback: Option<extern "C" fn(*const c_void) -> u64>,
    ipv6addr_callback: Option<extern "C" fn(*const c_void) -> *const u16>,
    ipv6addr_u_callback: Option<extern "C" fn(*const c_void) -> *const u16>,
    float32_callback: Option<extern "C" fn(*const c_void) -> f32>,
    set_float32_callback: Option<extern "C" fn(f32, *mut c_void)>,
    float32_u_callback: Option<extern "C" fn(*const c_void) -> f32>,
    string_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_string_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    string_u_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sunssf_5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sunssf_6_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sunssf_7_callback: Option<extern "C" fn(*const c_void) -> u16>,
    repeating_sunssf_8_callback: Option<extern "C" fn(*const c_void) -> u16>,
    repeating_int16_11_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_repeating_int16_11_callback: Option<extern "C" fn(i16, *mut c_void)>,
    repeating_int16_12_callback: Option<extern "C" fn(*const c_void) -> i16>,
    repeating_int16_u_callback: Option<extern "C" fn(*const c_void) -> i16>,
    repeating_uint16_11_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_repeating_uint16_11_callback: Option<extern "C" fn(u16, *mut c_void)>,
    repeating_uint16_12_callback: Option<extern "C" fn(*const c_void) -> u16>,
    repeating_uint16_13_callback: Option<extern "C" fn(*const c_void) -> u16>,
    repeating_uint16_u_callback: Option<extern "C" fn(*const c_void) -> u16>,
    repeating_int32_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_int32_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_int32_u_callback: Option<extern "C" fn(*const c_void) -> i32>,
    repeating_uint32_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_repeating_uint32_callback: Option<extern "C" fn(u32, *mut c_void)>,
    repeating_uint32_u_callback: Option<extern "C" fn(*const c_void) -> u32>,
    repeating_sunssf_9_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model63001CallbackAdapter {
    fn sunssf_1(&self) -> Option<u16> {
        self.sunssf_1_callback
            .map(|callback| (callback)(self.context))
    }

    fn sunssf_2(&self) -> Option<u16> {
        self.sunssf_2_callback
            .map(|callback| (callback)(self.context))
    }

    fn sunssf_3(&self) -> Option<u16> {
        self.sunssf_3_callback
            .map(|callback| (callback)(self.context))
    }

    fn sunssf_4(&self) -> Option<u16> {
        self.sunssf_4_callback
            .map(|callback| (callback)(self.context))
    }

    fn int16_1(&self) -> Option<i16> {
        self.int16_1_callback
            .map(|callback| (callback)(self.context))
    }

    fn int16_2(&self) -> Option<i16> {
        self.int16_2_callback
            .map(|callback| (callback)(self.context))
    }

    fn int16_3(&self) -> Option<i16> {
        self.int16_3_callback
            .map(|callback| (callback)(self.context))
    }

    fn int16_4(&self) -> Option<i16> {
        self.int16_4_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_int16_4(&mut self, value: i16) {
        if let Some(callback) = self.set_int16_4_callback {
            (callback)(value, self.context);
        };
    }

    fn int16_5(&self) -> Option<i16> {
        self.int16_5_callback
            .map(|callback| (callback)(self.context))
    }

    fn int16_u(&self) -> Option<i16> {
        self.int16_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint16_1(&self) -> Option<u16> {
        self.uint16_1_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint16_2(&self) -> Option<u16> {
        self.uint16_2_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint16_3(&self) -> Option<u16> {
        self.uint16_3_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint16_4(&self) -> Option<u16> {
        self.uint16_4_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_uint16_4(&mut self, value: u16) {
        if let Some(callback) = self.set_uint16_4_callback {
            (callback)(value, self.context);
        };
    }

    fn uint16_5(&self) -> Option<u16> {
        self.uint16_5_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint16_u(&self) -> Option<u16> {
        self.uint16_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn acc16(&self) -> Option<u16> {
        self.acc16_callback.map(|callback| (callback)(self.context))
    }

    fn acc16_u(&self) -> Option<u16> {
        self.acc16_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn enum16(&self) -> Option<u16> {
        self.enum16_callback
            .map(|callback| (callback)(self.context))
    }

    fn enum16_u(&self) -> Option<u16> {
        self.enum16_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn bitfield16(&self) -> Option<u16> {
        self.bitfield16_callback
            .map(|callback| (callback)(self.context))
    }

    fn bitfield16_u(&self) -> Option<u16> {
        self.bitfield16_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn int32_1(&self) -> Option<i32> {
        self.int32_1_callback
            .map(|callback| (callback)(self.context))
    }

    fn int32_2(&self) -> Option<i32> {
        self.int32_2_callback
            .map(|callback| (callback)(self.context))
    }

    fn int32_3(&self) -> Option<i32> {
        self.int32_3_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_int32_3(&mut self, value: i32) {
        if let Some(callback) = self.set_int32_3_callback {
            (callback)(value, self.context);
        };
    }

    fn int32_4(&self) -> Option<i32> {
        self.int32_4_callback
            .map(|callback| (callback)(self.context))
    }

    fn int32_5(&self) -> Option<i32> {
        self.int32_5_callback
            .map(|callback| (callback)(self.context))
    }

    fn int32_u(&self) -> Option<i32> {
        self.int32_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint32_1(&self) -> Option<u32> {
        self.uint32_1_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint32_2(&self) -> Option<u32> {
        self.uint32_2_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint32_3(&self) -> Option<u32> {
        self.uint32_3_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_uint32_3(&mut self, value: u32) {
        if let Some(callback) = self.set_uint32_3_callback {
            (callback)(value, self.context);
        };
    }

    fn uint32_4(&self) -> Option<u32> {
        self.uint32_4_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint32_5(&self) -> Option<u32> {
        self.uint32_5_callback
            .map(|callback| (callback)(self.context))
    }

    fn uint32_u(&self) -> Option<u32> {
        self.uint32_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn acc32(&self) -> Option<u32> {
        self.acc32_callback.map(|callback| (callback)(self.context))
    }

    fn acc32_u(&self) -> Option<u32> {
        self.acc32_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn enum32(&self) -> Option<u32> {
        self.enum32_callback
            .map(|callback| (callback)(self.context))
    }

    fn enum32_u(&self) -> Option<u32> {
        self.enum32_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn bitfield32(&self) -> Option<u32> {
        self.bitfield32_callback
            .map(|callback| (callback)(self.context))
    }

    fn bitfield32_u(&self) -> Option<u32> {
        self.bitfield32_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn ipaddr(&self) -> Option<u32> {
        self.ipaddr_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_ipaddr(&mut self, value: u32) {
        if let Some(callback) = self.set_ipaddr_callback {
            (callback)(value, self.context);
        };
    }

    fn ipaddr_u(&self) -> Option<u32> {
        self.ipaddr_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn int64(&self) -> Option<i64> {
        self.int64_callback.map(|callback| (callback)(self.context))
    }

    fn set_int64(&mut self, value: i64) {
        if let Some(callback) = self.set_int64_callback {
            (callback)(value, self.context);
        };
    }

    fn int64_u(&self) -> Option<i64> {
        self.int64_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn acc64(&self) -> Option<u64> {
        self.acc64_callback.map(|callback| (callback)(self.context))
    }

    fn acc64_u(&self) -> Option<u64> {
        self.acc64_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn ipv6addr(&self) -> Option<&[u16; 8]> {
        self.ipv6addr_callback
            .map(|callback| unsafe { &*((callback)(self.context) as *const [u16; 8]) })
    }

    fn ipv6addr_u(&self) -> Option<&[u16; 8]> {
        self.ipv6addr_u_callback
            .map(|callback| unsafe { &*((callback)(self.context) as *const [u16; 8]) })
    }

    fn float32(&self) -> Option<f32> {
        self.float32_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_float32(&mut self, value: f32) {
        if let Some(callback) = self.set_float32_callback {
            (callback)(value, self.context);
        };
    }

    fn float32_u(&self) -> Option<f32> {
        self.float32_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn string(&self) -> Option<&CStr> {
        self.string_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    fn set_string(&mut self, value: &CStr) {
        if let Some(callback) = self.set_string_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    fn string_u(&self) -> Option<&CStr> {
        self.string_u_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    fn sunssf_5(&self) -> Option<u16> {
        self.sunssf_5_callback
            .map(|callback| (callback)(self.context))
    }

    fn sunssf_6(&self) -> Option<u16> {
        self.sunssf_6_callback
            .map(|callback| (callback)(self.context))
    }

    fn sunssf_7(&self) -> Option<u16> {
        self.sunssf_7_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_sunssf_8(&self) -> Option<u16> {
        self.repeating_sunssf_8_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_int16_11(&self) -> Option<i16> {
        self.repeating_int16_11_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_repeating_int16_11(&mut self, value: i16) {
        if let Some(callback) = self.set_repeating_int16_11_callback {
            (callback)(value, self.context);
        };
    }

    fn repeating_int16_12(&self) -> Option<i16> {
        self.repeating_int16_12_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_int16_u(&self) -> Option<i16> {
        self.repeating_int16_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_uint16_11(&self) -> Option<u16> {
        self.repeating_uint16_11_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_repeating_uint16_11(&mut self, value: u16) {
        if let Some(callback) = self.set_repeating_uint16_11_callback {
            (callback)(value, self.context);
        };
    }

    fn repeating_uint16_12(&self) -> Option<u16> {
        self.repeating_uint16_12_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_uint16_13(&self) -> Option<u16> {
        self.repeating_uint16_13_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_uint16_u(&self) -> Option<u16> {
        self.repeating_uint16_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_int32(&self) -> Option<i32> {
        self.repeating_int32_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_repeating_int32(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_int32_callback {
            (callback)(value, self.context);
        };
    }

    fn repeating_int32_u(&self) -> Option<i32> {
        self.repeating_int32_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_uint32(&self) -> Option<u32> {
        self.repeating_uint32_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_repeating_uint32(&mut self, value: u32) {
        if let Some(callback) = self.set_repeating_uint32_callback {
            (callback)(value, self.context);
        };
    }

    fn repeating_uint32_u(&self) -> Option<u32> {
        self.repeating_uint32_u_callback
            .map(|callback| (callback)(self.context))
    }

    fn repeating_sunssf_9(&self) -> Option<u16> {
        self.repeating_sunssf_9_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model63001StatefulAdapter {
    sunssf_1: u16,
    sunssf_2: u16,
    sunssf_3: u16,
    sunssf_4: u16,
    int16_1: i16,
    int16_2: i16,
    int16_3: i16,
    int16_4: i16,
    int16_5: i16,
    int16_u: i16,
    uint16_1: u16,
    uint16_2: u16,
    uint16_3: u16,
    uint16_4: u16,
    uint16_5: u16,
    uint16_u: u16,
    acc16: u16,
    acc16_u: u16,
    enum16: u16,
    enum16_u: u16,
    bitfield16: u16,
    bitfield16_u: u16,
    int32_1: i32,
    int32_2: i32,
    int32_3: i32,
    int32_4: i32,
    int32_5: i32,
    int32_u: i32,
    uint32_1: u32,
    uint32_2: u32,
    uint32_3: u32,
    uint32_4: u32,
    uint32_5: u32,
    uint32_u: u32,
    acc32: u32,
    acc32_u: u32,
    enum32: u32,
    enum32_u: u32,
    bitfield32: u32,
    bitfield32_u: u32,
    ipaddr: u32,
    ipaddr_u: u32,
    int64: i64,
    int64_u: i64,
    acc64: u64,
    acc64_u: u64,
    ipv6addr: [u16; 8],
    ipv6addr_u: [u16; 8],
    float32: f32,
    float32_u: f32,
    string: [c_char; 32],
    string_u: [c_char; 32],
    sunssf_5: u16,
    sunssf_6: u16,
    sunssf_7: u16,
    repeating_sunssf_8: u16,
    repeating_int16_11: i16,
    repeating_int16_12: i16,
    repeating_int16_u: i16,
    repeating_uint16_11: u16,
    repeating_uint16_12: u16,
    repeating_uint16_13: u16,
    repeating_uint16_u: u16,
    repeating_int32: i32,
    repeating_int32_u: i32,
    repeating_uint32: u32,
    repeating_uint32_u: u32,
    repeating_sunssf_9: u16,
}

impl ModelAdapter for Model63001StatefulAdapter {
    fn sunssf_1(&self) -> Option<u16> {
        Some(self.sunssf_1)
    }

    fn sunssf_2(&self) -> Option<u16> {
        Some(self.sunssf_2)
    }

    fn sunssf_3(&self) -> Option<u16> {
        Some(self.sunssf_3)
    }

    fn sunssf_4(&self) -> Option<u16> {
        Some(self.sunssf_4)
    }

    fn int16_1(&self) -> Option<i16> {
        Some(self.int16_1)
    }

    fn int16_2(&self) -> Option<i16> {
        Some(self.int16_2)
    }

    fn int16_3(&self) -> Option<i16> {
        Some(self.int16_3)
    }

    fn int16_4(&self) -> Option<i16> {
        Some(self.int16_4)
    }

    fn set_int16_4(&mut self, value: i16) {
        self.int16_4 = value;
    }

    fn int16_5(&self) -> Option<i16> {
        Some(self.int16_5)
    }

    fn int16_u(&self) -> Option<i16> {
        Some(self.int16_u)
    }

    fn uint16_1(&self) -> Option<u16> {
        Some(self.uint16_1)
    }

    fn uint16_2(&self) -> Option<u16> {
        Some(self.uint16_2)
    }

    fn uint16_3(&self) -> Option<u16> {
        Some(self.uint16_3)
    }

    fn uint16_4(&self) -> Option<u16> {
        Some(self.uint16_4)
    }

    fn set_uint16_4(&mut self, value: u16) {
        self.uint16_4 = value;
    }

    fn uint16_5(&self) -> Option<u16> {
        Some(self.uint16_5)
    }

    fn uint16_u(&self) -> Option<u16> {
        Some(self.uint16_u)
    }

    fn acc16(&self) -> Option<u16> {
        Some(self.acc16)
    }

    fn acc16_u(&self) -> Option<u16> {
        Some(self.acc16_u)
    }

    fn enum16(&self) -> Option<u16> {
        Some(self.enum16)
    }

    fn enum16_u(&self) -> Option<u16> {
        Some(self.enum16_u)
    }

    fn bitfield16(&self) -> Option<u16> {
        Some(self.bitfield16)
    }

    fn bitfield16_u(&self) -> Option<u16> {
        Some(self.bitfield16_u)
    }

    fn int32_1(&self) -> Option<i32> {
        Some(self.int32_1)
    }

    fn int32_2(&self) -> Option<i32> {
        Some(self.int32_2)
    }

    fn int32_3(&self) -> Option<i32> {
        Some(self.int32_3)
    }

    fn set_int32_3(&mut self, value: i32) {
        self.int32_3 = value;
    }

    fn int32_4(&self) -> Option<i32> {
        Some(self.int32_4)
    }

    fn int32_5(&self) -> Option<i32> {
        Some(self.int32_5)
    }

    fn int32_u(&self) -> Option<i32> {
        Some(self.int32_u)
    }

    fn uint32_1(&self) -> Option<u32> {
        Some(self.uint32_1)
    }

    fn uint32_2(&self) -> Option<u32> {
        Some(self.uint32_2)
    }

    fn uint32_3(&self) -> Option<u32> {
        Some(self.uint32_3)
    }

    fn set_uint32_3(&mut self, value: u32) {
        self.uint32_3 = value;
    }

    fn uint32_4(&self) -> Option<u32> {
        Some(self.uint32_4)
    }

    fn uint32_5(&self) -> Option<u32> {
        Some(self.uint32_5)
    }

    fn uint32_u(&self) -> Option<u32> {
        Some(self.uint32_u)
    }

    fn acc32(&self) -> Option<u32> {
        Some(self.acc32)
    }

    fn acc32_u(&self) -> Option<u32> {
        Some(self.acc32_u)
    }

    fn enum32(&self) -> Option<u32> {
        Some(self.enum32)
    }

    fn enum32_u(&self) -> Option<u32> {
        Some(self.enum32_u)
    }

    fn bitfield32(&self) -> Option<u32> {
        Some(self.bitfield32)
    }

    fn bitfield32_u(&self) -> Option<u32> {
        Some(self.bitfield32_u)
    }

    fn ipaddr(&self) -> Option<u32> {
        Some(self.ipaddr)
    }

    fn set_ipaddr(&mut self, value: u32) {
        self.ipaddr = value;
    }

    fn ipaddr_u(&self) -> Option<u32> {
        Some(self.ipaddr_u)
    }

    fn int64(&self) -> Option<i64> {
        Some(self.int64)
    }

    fn set_int64(&mut self, value: i64) {
        self.int64 = value;
    }

    fn int64_u(&self) -> Option<i64> {
        Some(self.int64_u)
    }

    fn acc64(&self) -> Option<u64> {
        Some(self.acc64)
    }

    fn acc64_u(&self) -> Option<u64> {
        Some(self.acc64_u)
    }

    fn ipv6addr(&self) -> Option<&[u16; 8]> {
        Some(unsafe { &*(self.ipv6addr.as_ptr() as *const [u16; 8]) })
    }

    fn ipv6addr_u(&self) -> Option<&[u16; 8]> {
        Some(unsafe { &*(self.ipv6addr_u.as_ptr() as *const [u16; 8]) })
    }

    fn float32(&self) -> Option<f32> {
        Some(self.float32)
    }

    fn set_float32(&mut self, value: f32) {
        self.float32 = value;
    }

    fn float32_u(&self) -> Option<f32> {
        Some(self.float32_u)
    }

    fn string(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.string.as_ptr()) })
    }

    fn set_string(&mut self, value: &CStr) {
        for (dest, src) in self.string.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    fn string_u(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.string_u.as_ptr()) })
    }

    fn sunssf_5(&self) -> Option<u16> {
        Some(self.sunssf_5)
    }

    fn sunssf_6(&self) -> Option<u16> {
        Some(self.sunssf_6)
    }

    fn sunssf_7(&self) -> Option<u16> {
        Some(self.sunssf_7)
    }

    fn repeating_sunssf_8(&self) -> Option<u16> {
        Some(self.repeating_sunssf_8)
    }

    fn repeating_int16_11(&self) -> Option<i16> {
        Some(self.repeating_int16_11)
    }

    fn set_repeating_int16_11(&mut self, value: i16) {
        self.repeating_int16_11 = value;
    }

    fn repeating_int16_12(&self) -> Option<i16> {
        Some(self.repeating_int16_12)
    }

    fn repeating_int16_u(&self) -> Option<i16> {
        Some(self.repeating_int16_u)
    }

    fn repeating_uint16_11(&self) -> Option<u16> {
        Some(self.repeating_uint16_11)
    }

    fn set_repeating_uint16_11(&mut self, value: u16) {
        self.repeating_uint16_11 = value;
    }

    fn repeating_uint16_12(&self) -> Option<u16> {
        Some(self.repeating_uint16_12)
    }

    fn repeating_uint16_13(&self) -> Option<u16> {
        Some(self.repeating_uint16_13)
    }

    fn repeating_uint16_u(&self) -> Option<u16> {
        Some(self.repeating_uint16_u)
    }

    fn repeating_int32(&self) -> Option<i32> {
        Some(self.repeating_int32)
    }

    fn set_repeating_int32(&mut self, value: i32) {
        self.repeating_int32 = value;
    }

    fn repeating_int32_u(&self) -> Option<i32> {
        Some(self.repeating_int32_u)
    }

    fn repeating_uint32(&self) -> Option<u32> {
        Some(self.repeating_uint32)
    }

    fn set_repeating_uint32(&mut self, value: u32) {
        self.repeating_uint32 = value;
    }

    fn repeating_uint32_u(&self) -> Option<u32> {
        Some(self.repeating_uint32_u)
    }

    fn repeating_sunssf_9(&self) -> Option<u16> {
        Some(self.repeating_sunssf_9)
    }
}
