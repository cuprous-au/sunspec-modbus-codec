use core::{
    ffi::CStr,
    net::Ipv4Addr,
    ops::{Add, Shl},
};

use crate::ModbusException;

#[derive(Clone, Copy)]
pub enum ModbusWordByteOrder {
    System,
    BigEndian,
}

/// A mutable view over a Modbus register block, used to encode a response (or any other
/// register data flowing out of a model and into the wire buffer).
pub struct WritableRegisterBuffer<'a> {
    buffer: &'a mut [[u8; 2]],
    byte_order: ModbusWordByteOrder,
}

impl<'a> WritableRegisterBuffer<'a> {
    pub fn slice<'b>(&'b mut self, word_offset: u16, length: u16) -> WritableRegisterBuffer<'b> {
        WritableRegisterBuffer {
            buffer: &mut self.buffer[word_offset as usize..(word_offset + length) as usize],
            byte_order: self.byte_order,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn len(&self) -> u16 {
        self.buffer.len() as u16
    }

    pub fn fill(self: &mut WritableRegisterBuffer<'a>, word: &[u8; 2]) {
        self.buffer.fill(order_bytes(word, self.byte_order))
    }

    pub fn zero(self: &mut WritableRegisterBuffer<'a>) {
        self.buffer.fill([0, 0]);
    }

    pub fn write_u16(self: &mut WritableRegisterBuffer<'a>, value: u16) {
        self.buffer[0] = match self.byte_order {
            ModbusWordByteOrder::BigEndian => value.to_be_bytes(),
            ModbusWordByteOrder::System => value.to_ne_bytes(),
        }
    }

    pub fn write_u32(self: &mut WritableRegisterBuffer<'a>, value: u32, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_u64(self: &mut WritableRegisterBuffer<'a>, value: u64, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_u128(self: &mut WritableRegisterBuffer<'a>, value: u128, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_i16(self: &mut WritableRegisterBuffer<'a>, value: i16) {
        self.buffer[0] = match self.byte_order {
            ModbusWordByteOrder::BigEndian => value.to_be_bytes(),
            ModbusWordByteOrder::System => value.to_ne_bytes(),
        }
    }

    pub fn write_i32(self: &mut WritableRegisterBuffer<'a>, value: i32, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_i64(self: &mut WritableRegisterBuffer<'a>, value: i64, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_f32(self: &mut WritableRegisterBuffer<'a>, value: f32, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_f64(self: &mut WritableRegisterBuffer<'a>, value: f64, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_ipv4_addr(self: &mut WritableRegisterBuffer<'a>, value: Ipv4Addr, offset: u16) {
        self.write_bytes(&value.octets(), offset)
    }

    pub fn write_ipv6_addr(self: &mut WritableRegisterBuffer<'a>, value: &[u16; 8], offset: u16) {
        value
            .iter()
            .skip(offset as usize)
            .zip(self.buffer.iter_mut())
            .for_each(|(v, buf_word)| {
                *buf_word = match self.byte_order {
                    ModbusWordByteOrder::BigEndian => v.to_be_bytes(),
                    ModbusWordByteOrder::System => v.to_ne_bytes(),
                }
            });
    }

    pub fn write_eui48(self: &mut WritableRegisterBuffer<'a>, value: &[u8; 6], offset: u16) {
        self.write_bytes(value, offset)
    }

    pub fn write_string(self: &mut WritableRegisterBuffer<'a>, str: &CStr, offset: u16) {
        self.buffer.fill([0, 0]);
        self.write_bytes(str.to_bytes_with_nul(), offset);
    }

    pub fn write_bytes(self: &mut WritableRegisterBuffer<'a>, bytes: &[u8], offset: u16) {
        let (chunks, remainder) = bytes.as_chunks();

        let last_chunk = remainder.first().map(|byte| [*byte, 0_u8]);

        chunks
            .iter()
            .chain(&last_chunk)
            .skip(offset as usize)
            .zip(self.buffer.iter_mut())
            .for_each(|(chunk, buf_word)| *buf_word = order_bytes(chunk, self.byte_order));
    }
}

impl<'a> From<&'a mut [u16]> for WritableRegisterBuffer<'a> {
    fn from(value: &'a mut [u16]) -> Self {
        Self {
            buffer: bytemuck::cast_slice_mut(value),
            byte_order: ModbusWordByteOrder::System,
        }
    }
}

impl<'a> From<&'a mut [u8]> for WritableRegisterBuffer<'a> {
    fn from(value: &'a mut [u8]) -> Self {
        let (pairs, _) = value.as_chunks_mut::<2>();
        Self {
            buffer: pairs,
            byte_order: ModbusWordByteOrder::BigEndian,
        }
    }
}

/// An immutable view over a Modbus register block, used to decode a request (register data
/// flowing in from the wire buffer and into a model).
pub struct ReadableRegisterBuffer<'a> {
    buffer: &'a [[u8; 2]],
    byte_order: ModbusWordByteOrder,
}

impl<'a> ReadableRegisterBuffer<'a> {
    pub fn slice<'b>(&'b self, word_offset: u16, length: u16) -> ReadableRegisterBuffer<'b> {
        ReadableRegisterBuffer {
            buffer: &self.buffer[word_offset as usize..(word_offset + length) as usize],
            byte_order: self.byte_order,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn len(&self) -> u16 {
        self.buffer.len() as u16
    }

    pub fn read_integer<A>(
        self: &ReadableRegisterBuffer<'a>,
        len: u16,
    ) -> Result<A, ModbusException>
    where
        A: Shl<usize, Output = A> + Add<A, Output = A> + Default + From<u8>,
    {
        if self.len() != len {
            Err(ModbusException::IllegalDataAddress)
        } else {
            Ok(self
                .buffer
                .iter()
                .flat_map(|word| order_bytes(word, self.byte_order))
                .fold(A::default(), |acc: A, byte: u8| (acc << 8) + A::from(byte)))
        }
    }

    pub fn read_u16(self: &ReadableRegisterBuffer<'a>) -> Result<u16, ModbusException> {
        if self.len() != 1 {
            Err(ModbusException::IllegalDataAddress)
        } else {
            Ok(match self.byte_order {
                ModbusWordByteOrder::BigEndian => u16::from_be_bytes(self.buffer[0]),
                ModbusWordByteOrder::System => u16::from_ne_bytes(self.buffer[0]),
            })
        }
    }

    pub fn read_u32(self: &ReadableRegisterBuffer<'a>) -> Result<u32, ModbusException> {
        self.read_integer(2)
    }

    pub fn read_u64(self: &ReadableRegisterBuffer<'a>) -> Result<u64, ModbusException> {
        self.read_integer(4)
    }

    pub fn read_u128(self: &ReadableRegisterBuffer<'a>) -> Result<u128, ModbusException> {
        self.read_integer(8)
    }

    pub fn read_i16(self: &ReadableRegisterBuffer<'a>) -> Result<i16, ModbusException> {
        if self.len() != 1 {
            Err(ModbusException::IllegalDataAddress)
        } else {
            Ok(match self.byte_order {
                ModbusWordByteOrder::BigEndian => i16::from_be_bytes(self.buffer[0]),
                ModbusWordByteOrder::System => i16::from_ne_bytes(self.buffer[0]),
            })
        }
    }

    pub fn read_i32(self: &ReadableRegisterBuffer<'a>) -> Result<i32, ModbusException> {
        self.read_integer(2)
    }

    pub fn read_i64(self: &ReadableRegisterBuffer<'a>) -> Result<i64, ModbusException> {
        self.read_integer(4)
    }

    pub fn read_f32(self: &ReadableRegisterBuffer<'a>) -> Result<f32, ModbusException> {
        if self.len() != 2 {
            Err(ModbusException::IllegalDataAddress)
        } else {
            let mut bytes = [0; 4];
            self.buffer
                .iter()
                .flat_map(|word| order_bytes(word, self.byte_order))
                .zip(bytes.iter_mut())
                .for_each(|(a, b)| *b = a);
            Ok(f32::from_be_bytes(bytes))
        }
    }

    pub fn read_f64(self: &ReadableRegisterBuffer<'a>) -> Result<f64, ModbusException> {
        if self.len() != 4 {
            Err(ModbusException::IllegalDataAddress)
        } else {
            let mut bytes = [0; 8];
            self.buffer
                .iter()
                .flat_map(|word| order_bytes(word, self.byte_order))
                .zip(bytes.iter_mut())
                .for_each(|(a, b)| *b = a);
            Ok(f64::from_be_bytes(bytes))
        }
    }

    pub fn read_ipv4_addr(self: &ReadableRegisterBuffer<'a>) -> Result<Ipv4Addr, ModbusException> {
        if self.len() != 2 {
            Err(ModbusException::IllegalDataAddress)
        } else {
            let mut bytes = [0; 4];
            self.buffer
                .iter()
                .flat_map(|word| order_bytes(word, self.byte_order))
                .zip(bytes.iter_mut())
                .for_each(|(a, b)| *b = a);
            Ok(Ipv4Addr::from_octets(bytes))
        }
    }

    pub fn read_ipv6_addr(self: &ReadableRegisterBuffer<'a>) -> Result<[u16; 8], ModbusException> {
        if self.len() != 8 {
            Err(ModbusException::IllegalDataAddress)
        } else {
            let mut words = [0; 8];
            self.buffer.iter().zip(words.iter_mut()).for_each(|(a, b)| {
                *b = match self.byte_order {
                    ModbusWordByteOrder::BigEndian => u16::from_be_bytes(*a),
                    ModbusWordByteOrder::System => u16::from_ne_bytes(*a),
                }
            });
            Ok(words)
        }
    }

    pub fn read_eui48(self: &ReadableRegisterBuffer<'a>) -> Result<[u8; 6], ModbusException> {
        if self.len() != 3 {
            Err(ModbusException::IllegalDataAddress)
        } else {
            let mut bytes = [0; 6];
            self.buffer
                .iter()
                .flat_map(|word| order_bytes(word, self.byte_order))
                .zip(bytes.iter_mut())
                .for_each(|(a, b)| *b = a);
            Ok(bytes)
        }
    }

    pub fn read_string<const N: usize>(
        self: &ReadableRegisterBuffer<'a>,
    ) -> Result<[u8; N], ModbusException> {
        if self.len() * 2 != N as u16 {
            Err(ModbusException::IllegalDataAddress)
        } else {
            let mut str = [0; N];
            self.buffer
                .iter()
                .flat_map(|word| order_bytes(word, self.byte_order))
                .zip(str.iter_mut())
                .for_each(|(a, b)| *b = a);
            Ok(str)
        }
    }
}

impl<'a> From<&'a [u16]> for ReadableRegisterBuffer<'a> {
    fn from(value: &'a [u16]) -> Self {
        Self {
            buffer: bytemuck::cast_slice(value),
            byte_order: ModbusWordByteOrder::System,
        }
    }
}

impl<'a> From<&'a [u8]> for ReadableRegisterBuffer<'a> {
    fn from(value: &'a [u8]) -> Self {
        let (pairs, _) = value.as_chunks::<2>();
        Self {
            buffer: pairs,
            byte_order: ModbusWordByteOrder::BigEndian,
        }
    }
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
