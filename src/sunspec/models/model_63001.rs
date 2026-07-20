use heapless::String;
use core::net::{Ipv6Addr, Ipv4Addr};
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 136;

pub static POINTS: [ReadablePoint; 58] = [
    ReadablePoint {
        reference: PointReference::Static { value: 63001 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 134 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Sunssf1 },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Sunssf2 },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Sunssf3 },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Sunssf4 },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int161 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int162 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int163 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int164 },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int165 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int16U },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint161 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint162 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint163 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint164 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint165 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint16U },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Acc16 },
        size: 1,
        data_type: PointType::Acc16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Acc16U },
        size: 1,
        data_type: PointType::Acc16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Enum16 },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Enum16U },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Bitfield16 },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Bitfield16U },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int321 },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int322 },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int323 },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int324 },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int325 },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int32U },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint321 },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint322 },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint323 },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint324 },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint325 },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Uint32U },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Acc32 },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Acc32U },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Enum32 },
        size: 2,
        data_type: PointType::Enum32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Enum32U },
        size: 2,
        data_type: PointType::Enum32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Bitfield32 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Bitfield32U },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Ipaddr },
        size: 2,
        data_type: PointType::Ipaddr,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::IpaddrU },
        size: 2,
        data_type: PointType::Ipaddr,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int64 },
        size: 4,
        data_type: PointType::Int64,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Int64U },
        size: 4,
        data_type: PointType::Int64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Acc64 },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Acc64U },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Ipv6addr },
        size: 8,
        data_type: PointType::Ipv6addr,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Ipv6addrU },
        size: 8,
        data_type: PointType::Ipv6addr,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Float32 },
        size: 2,
        data_type: PointType::Float32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Float32U },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::String },
        size: 16,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::StringU },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Sunssf5 },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Sunssf6 },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model63001 { point: Point::Sunssf7 },
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
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Sunssf1 => if let Some(value) = model.sunssf_1() { serialisation::write_u16(value, buffer); },
        Point::Sunssf2 => if let Some(value) = model.sunssf_2() { serialisation::write_u16(value, buffer); },
        Point::Sunssf3 => if let Some(value) = model.sunssf_3() { serialisation::write_u16(value, buffer); },
        Point::Sunssf4 => if let Some(value) = model.sunssf_4() { serialisation::write_u16(value, buffer); },
        Point::Int161 => if let Some(value) = model.int16_1() { serialisation::write_i16(value, buffer); },
        Point::Int162 => if let Some(value) = model.int16_2() { serialisation::write_i16(value, buffer); },
        Point::Int163 => if let Some(value) = model.int16_3() { serialisation::write_i16(value, buffer); },
        Point::Int164 => if let Some(value) = model.int16_4() { serialisation::write_i16(value, buffer); },
        Point::Int165 => if let Some(value) = model.int16_5() { serialisation::write_i16(value, buffer); },
        Point::Int16U => if let Some(value) = model.int16_u() { serialisation::write_i16(value, buffer); },
        Point::Uint161 => if let Some(value) = model.uint16_1() { serialisation::write_u16(value, buffer); },
        Point::Uint162 => if let Some(value) = model.uint16_2() { serialisation::write_u16(value, buffer); },
        Point::Uint163 => if let Some(value) = model.uint16_3() { serialisation::write_u16(value, buffer); },
        Point::Uint164 => if let Some(value) = model.uint16_4() { serialisation::write_u16(value, buffer); },
        Point::Uint165 => if let Some(value) = model.uint16_5() { serialisation::write_u16(value, buffer); },
        Point::Uint16U => if let Some(value) = model.uint16_u() { serialisation::write_u16(value, buffer); },
        Point::Acc16 => if let Some(value) = model.acc16() { serialisation::write_u16(value, buffer); },
        Point::Acc16U => if let Some(value) = model.acc16_u() { serialisation::write_u16(value, buffer); },
        Point::Enum16 => if let Some(value) = model.enum16() { serialisation::write_u16(value as u16, buffer); },
        Point::Enum16U => if let Some(value) = model.enum16_u() { serialisation::write_u16(value as u16, buffer); },
        Point::Bitfield16 => if let Some(value) = model.bitfield16() { serialisation::write_u16(value, buffer); },
        Point::Bitfield16U => if let Some(value) = model.bitfield16_u() { serialisation::write_u16(value, buffer); },
        Point::Int321 => if let Some(value) = model.int32_1() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::Int322 => if let Some(value) = model.int32_2() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::Int323 => if let Some(value) = model.int32_3() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::Int324 => if let Some(value) = model.int32_4() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::Int325 => if let Some(value) = model.int32_5() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::Int32U => if let Some(value) = model.int32_u() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::Uint321 => if let Some(value) = model.uint32_1() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Uint322 => if let Some(value) = model.uint32_2() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Uint323 => if let Some(value) = model.uint32_3() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Uint324 => if let Some(value) = model.uint32_4() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Uint325 => if let Some(value) = model.uint32_5() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Uint32U => if let Some(value) = model.uint32_u() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Acc32 => if let Some(value) = model.acc32() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Acc32U => if let Some(value) = model.acc32_u() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Enum32 => if let Some(value) = model.enum32() { serialisation::write_u32(value as u32, buffer, offset, limit); },
        Point::Enum32U => if let Some(value) = model.enum32_u() { serialisation::write_u32(value as u32, buffer, offset, limit); },
        Point::Bitfield32 => if let Some(value) = model.bitfield32() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Bitfield32U => if let Some(value) = model.bitfield32_u() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Ipaddr => if let Some(value) = model.ipaddr() { serialisation::write_ipaddr(value, buffer, offset, limit); },
        Point::IpaddrU => if let Some(value) = model.ipaddr_u() { serialisation::write_ipaddr(value, buffer, offset, limit); },
        Point::Int64 => if let Some(value) = model.int64() { serialisation::write_i64(value, buffer, offset, limit); },
        Point::Int64U => if let Some(value) = model.int64_u() { serialisation::write_i64(value, buffer, offset, limit); },
        Point::Acc64 => if let Some(value) = model.acc64() { serialisation::write_u64(value, buffer, offset, limit); },
        Point::Acc64U => if let Some(value) = model.acc64_u() { serialisation::write_u64(value, buffer, offset, limit); },
        Point::Ipv6addr => if let Some(value) = model.ipv6addr() { serialisation::write_ipv6addr(value, buffer, offset, limit); },
        Point::Ipv6addrU => if let Some(value) = model.ipv6addr_u() { serialisation::write_ipv6addr(value, buffer, offset, limit); },
        Point::Float32 => if let Some(value) = model.float32() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::Float32U => if let Some(value) = model.float32_u() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::String => if let Some(value) = model.string() { serialisation::write_string(value, buffer, offset, limit); },
        Point::StringU => if let Some(value) = model.string_u() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sunssf5 => if let Some(value) = model.sunssf_5() { serialisation::write_u16(value, buffer); },
        Point::Sunssf6 => if let Some(value) = model.sunssf_6() { serialisation::write_u16(value, buffer); },
        Point::Sunssf7 => if let Some(value) = model.sunssf_7() { serialisation::write_u16(value, buffer); },
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

    fn set_int16_4(&mut self, value: i16) {
    }

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

    fn set_uint16_4(&mut self, value: u16) {
    }

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

    fn enum16(&self) -> Option<Enum16> {
        None
    }

    fn enum16_u(&self) -> Option<Enum16U> {
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

    fn set_int32_3(&mut self, value: i32) {
    }

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

    fn set_uint32_3(&mut self, value: u32) {
    }

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

    fn enum32(&self) -> Option<Enum32> {
        None
    }

    fn enum32_u(&self) -> Option<Enum32U> {
        None
    }

    fn bitfield32(&self) -> Option<u32> {
        None
    }

    fn bitfield32_u(&self) -> Option<u32> {
        None
    }

    fn ipaddr(&self) -> Option<Ipv4Addr> {
        None
    }

    fn set_ipaddr(&mut self, value: Ipv4Addr) {
    }

    fn ipaddr_u(&self) -> Option<Ipv4Addr> {
        None
    }

    fn int64(&self) -> Option<i64> {
        None
    }

    fn set_int64(&mut self, value: i64) {
    }

    fn int64_u(&self) -> Option<i64> {
        None
    }

    fn acc64(&self) -> Option<u64> {
        None
    }

    fn acc64_u(&self) -> Option<u64> {
        None
    }

    fn ipv6addr(&self) -> Option<Ipv6Addr> {
        None
    }

    fn ipv6addr_u(&self) -> Option<Ipv6Addr> {
        None
    }

    fn float32(&self) -> Option<f32> {
        None
    }

    fn set_float32(&mut self, value: f32) {
    }

    fn float32_u(&self) -> Option<f32> {
        None
    }

    fn string(&self) -> Option<String<32>> {
        None
    }

    fn set_string(&mut self, value: String<32>) {
    }

    fn string_u(&self) -> Option<String<32>> {
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
}

pub enum Enum16 {
}

pub enum Enum16U {
}

pub enum Enum32 {
}

pub enum Enum32U {
}