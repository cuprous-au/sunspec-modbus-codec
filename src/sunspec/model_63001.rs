use core::net::{Ipv4Addr, Ipv6Addr};
use heapless::String;

pub struct Model63001 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    sunssf_1: Option<u16>,
    sunssf_2: Option<u16>,
    sunssf_3: Option<u16>,
    sunssf_4: Option<u16>,
    int16_1: Option<i16>,
    int16_2: Option<i16>,
    int16_3: Option<i16>,
    int16_4: Option<i16>,
    int16_5: Option<i16>,
    int16_u: Option<i16>,
    uint16_1: Option<u16>,
    uint16_2: Option<u16>,
    uint16_3: Option<u16>,
    uint16_4: Option<u16>,
    uint16_5: Option<u16>,
    uint16_u: Option<u16>,
    acc16: Option<u16>,
    acc16_u: Option<u16>,
    enum16: Option<Enum16>,
    enum16_u: Option<Enum16U>,
    bitfield16: Option<u16>,
    bitfield16_u: Option<u16>,
    int32_1: Option<i32>,
    int32_2: Option<i32>,
    int32_3: Option<i32>,
    int32_4: Option<i32>,
    int32_5: Option<i32>,
    int32_u: Option<i32>,
    uint32_1: Option<u32>,
    uint32_2: Option<u32>,
    uint32_3: Option<u32>,
    uint32_4: Option<u32>,
    uint32_5: Option<u32>,
    uint32_u: Option<u32>,
    acc32: Option<u32>,
    acc32_u: Option<u32>,
    enum32: Option<Enum32>,
    enum32_u: Option<Enum32U>,
    bitfield32: Option<u32>,
    bitfield32_u: Option<u32>,
    ipaddr: Option<Ipv4Addr>,
    ipaddr_u: Option<Ipv4Addr>,
    int64: Option<i64>,
    int64_u: Option<i64>,
    acc64: Option<u64>,
    acc64_u: Option<u64>,
    ipv6addr: Option<Ipv6Addr>,
    ipv6addr_u: Option<Ipv6Addr>,
    float32: Option<f32>,
    float32_u: Option<f32>,
    string: Option<String<32>>,
    string_u: Option<String<32>>,
    sunssf_5: Option<u16>,
    sunssf_6: Option<u16>,
    sunssf_7: Option<u16>,
}

trait Model63001Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

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

    fn set_ipaddr(&mut self, value: Ipv4Addr) {}

    fn ipaddr_u(&self) -> Option<Ipv4Addr> {
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

    fn ipv6addr(&self) -> Option<Ipv6Addr> {
        None
    }

    fn ipv6addr_u(&self) -> Option<Ipv6Addr> {
        None
    }

    fn float32(&self) -> Option<f32> {
        None
    }

    fn set_float32(&mut self, value: f32) {}

    fn float32_u(&self) -> Option<f32> {
        None
    }

    fn string(&self) -> Option<String<32>> {
        None
    }

    fn set_string(&mut self, value: String<32>) {}

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

pub enum Enum16 {}

pub enum Enum16U {}

pub enum Enum32 {}

pub enum Enum32U {}
