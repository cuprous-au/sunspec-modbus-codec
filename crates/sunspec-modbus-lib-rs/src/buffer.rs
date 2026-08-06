use core::{
    ffi::{CStr, c_int},
    net::Ipv4Addr,
};

#[derive(Clone, Copy)]
pub enum ModbusWordByteOrder {
    System,
    BigEndian,
}
unsafe extern "C" {
    pub fn printf(format: *const core::ffi::c_char, ...) -> i32;
}

pub struct ModbusBuffer<'a> {
    buffer: &'a mut [[u8; 2]],
    byte_order: ModbusWordByteOrder,
}

impl<'a> ModbusBuffer<'a> {
    pub fn slice<'b>(&'b mut self, word_offset: u16, length: u16) -> ModbusBuffer<'b> {
        unsafe {
            printf(
                c"slicing %d starting from %d\n".as_ptr(),
                word_offset as c_int,
                length as c_int,
            )
        };
        ModbusBuffer {
            buffer: &mut self.buffer[word_offset as usize..(word_offset + length) as usize],
            byte_order: self.byte_order,
        }
    }
}

impl<'a> From<&'a mut [u16]> for ModbusBuffer<'a> {
    fn from(value: &'a mut [u16]) -> Self {
        Self {
            buffer: bytemuck::cast_slice_mut(value),
            byte_order: ModbusWordByteOrder::System,
        }
    }
}

impl<'a> From<&'a mut [u8]> for ModbusBuffer<'a> {
    fn from(value: &'a mut [u8]) -> Self {
        let (pairs, _) = value.as_chunks_mut::<2>();
        Self {
            buffer: pairs,
            byte_order: ModbusWordByteOrder::BigEndian,
        }
    }
}

pub fn write_u16<'a>(value: u16, buffer: ModbusBuffer<'a>) {
    buffer.buffer[0] = match buffer.byte_order {
        ModbusWordByteOrder::BigEndian => value.to_be_bytes(),
        ModbusWordByteOrder::System => value.to_ne_bytes(),
    }
}

pub fn write_u32<'a>(value: u32, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    write_bytes(&value.to_be_bytes(), buffer, offset, limit)
}

pub fn write_u64<'a>(value: u64, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    write_bytes(&value.to_be_bytes(), buffer, offset, limit)
}

pub fn write_u128<'a>(value: u128, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    write_bytes(&value.to_be_bytes(), buffer, offset, limit)
}

pub fn write_i16<'a>(value: i16, buffer: ModbusBuffer<'a>) {
    buffer.buffer[0] = value.to_ne_bytes();
}

pub fn write_i32<'a>(value: i32, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    write_bytes(&value.to_be_bytes(), buffer, offset, limit)
}

pub fn write_i64<'a>(value: i64, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    write_bytes(&value.to_be_bytes(), buffer, offset, limit)
}

pub fn write_f32<'a>(value: f32, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    write_bytes(&value.to_be_bytes(), buffer, offset, limit)
}

pub fn write_f64<'a>(value: f64, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    write_bytes(&value.to_be_bytes(), buffer, offset, limit)
}

pub fn write_ipv4_addr<'a>(
    value: Ipv4Addr,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    write_bytes(&value.octets(), buffer, offset, limit)
}

pub fn write_ipv6_addr<'a>(
    value: &[u16; 8],
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    value
        .iter()
        .skip(offset as usize)
        .take(limit as usize)
        .zip(buffer.buffer)
        .for_each(|(v, buf_word)| *buf_word = v.to_ne_bytes());
}

pub fn write_eui48<'a>(value: &[u8; 6], buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    write_bytes(value, buffer, offset, limit)
}

pub fn write_string<'a>(str: &CStr, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    buffer.buffer.fill([0, 0]);
    write_bytes(str.to_bytes_with_nul(), buffer, offset, limit);
}

pub fn zero<'a>(buffer: ModbusBuffer<'a>, limit: u16) {
    buffer.buffer[..limit as usize].fill([0, 0]);
}

#[cfg(target_endian = "little")]
fn order_bytes(word: &[u8; 2], byte_order: ModbusWordByteOrder) -> [u8; 2] {
    match byte_order {
        ModbusWordByteOrder::BigEndian => *word,
        ModbusWordByteOrder::System => [word[1], word[0]],
    }
}

#[cfg(target_endian = "big")]
fn order_bytes(word: &[u8; 2], byte_order: ModbusWordByteOrder) -> [u8; 2] {
    *word
}

pub fn write_bytes<'a>(bytes: &[u8], buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    let (chunks, remainder) = bytes.as_chunks();

    let last_chunk = remainder.first().map(|byte| [*byte, 0_u8]);

    chunks
        .iter()
        .chain(&last_chunk)
        .skip(offset as usize)
        .take(limit as usize)
        .zip(buffer.buffer)
        .for_each(|(chunk, buf_word)| *buf_word = order_bytes(chunk, buffer.byte_order));
}
