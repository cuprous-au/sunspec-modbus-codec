//! The SunSpec "not implemented" value for each point type.
//!
//! A read of an optional point the device doesn't provide - its adapter returns `None` -
//! encodes the value for its type here, so a client can tell an absent point from a real
//! reading. Defined by section 6.4 of
//! the SunSpec Device Information Model Specification.
//!
//! `acc*` and `ipaddr` points are not implemented when all-zero, so the generator writes zero
//! directly. `string` and `ipv6addr` use zero-valued constants below for type-correct fallbacks.

use core::ffi::CStr;

/// `int16`: `0x8000`.
pub const INT16: i16 = i16::MIN;
/// `uint16`: `0xFFFF`.
pub const UINT16: u16 = u16::MAX;
/// `count`: `0xFFFF`.
pub const COUNT: u16 = u16::MAX;
/// `enum16`: `0xFFFF`.
pub const ENUM16: u16 = u16::MAX;
/// `bitfield16`: `0xFFFF`.
pub const BITFIELD16: u16 = u16::MAX;
/// `sunssf`: `0x8000`.
pub const SUNSSF: i16 = i16::MIN;
/// `pad`: `0x8000`. Pad registers carry no data, so they always read as this.
pub const PAD: u16 = 0x8000;

/// `int32`: `0x80000000`.
pub const INT32: i32 = i32::MIN;
/// `uint32`: `0xFFFFFFFF`.
pub const UINT32: u32 = u32::MAX;
/// `enum32`: `0xFFFFFFFF`.
pub const ENUM32: u32 = u32::MAX;
/// `bitfield32`: `0xFFFFFFFF`.
pub const BITFIELD32: u32 = u32::MAX;

/// `int64`: `0x8000000000000000`.
pub const INT64: i64 = i64::MIN;
/// `uint64`: `0xFFFFFFFFFFFFFFFF`.
pub const UINT64: u64 = u64::MAX;
/// `bitfield64`: `0xFFFFFFFFFFFFFFFF`.
pub const BITFIELD64: u64 = u64::MAX;

/// `float32`: the quiet NaN `0x7FC00000`. Compare with [`f32::to_bits`], not `==`.
pub const FLOAT32: f32 = f32::from_bits(0x7FC0_0000);
/// `float64`: the quiet NaN `0x7FF8000000000000`. Compare with [`f64::to_bits`], not `==`.
pub const FLOAT64: f64 = f64::from_bits(0x7FF8_0000_0000_0000);

/// `eui48`: `FF:FF:FF:FF:FF:FF`. The specification doesn't define one for `eui48`; this is
/// the value SunSpec's reference library, pysunspec2, uses.
pub const EUI48: [u8; 6] = [0xFF; 6];

/// `ipv61ddr`: all zero
pub const IPV6ADDR: [u16; 8] = [0; 8];

/// `string`: an empty string
pub const STRING: &CStr = c"";
