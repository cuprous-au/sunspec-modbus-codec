use core::{ffi::CStr, net::Ipv4Addr};

#[derive(Clone, Copy)]
pub enum ModbusWordByteOrder {
    System,
    BigEndian,
}

pub struct ModbusBuffer<'a> {
    buffer: &'a mut [[u8; 2]],
    byte_order: ModbusWordByteOrder,
}

impl<'a> ModbusBuffer<'a> {
    pub fn slice<'b>(&'b mut self, word_offset: u16, length: u16) -> ModbusBuffer<'b> {
        ModbusBuffer {
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

    pub fn write_u16(self: &mut ModbusBuffer<'a>, value: u16) {
        self.buffer[0] = match self.byte_order {
            ModbusWordByteOrder::BigEndian => value.to_be_bytes(),
            ModbusWordByteOrder::System => value.to_ne_bytes(),
        }
    }

    pub fn write_u32(self: &mut ModbusBuffer<'a>, value: u32, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_u64(self: &mut ModbusBuffer<'a>, value: u64, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_u128(self: &mut ModbusBuffer<'a>, value: u128, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_i16(self: &mut ModbusBuffer<'a>, value: i16) {
        self.buffer[0] = match self.byte_order {
            ModbusWordByteOrder::BigEndian => value.to_be_bytes(),
            ModbusWordByteOrder::System => value.to_ne_bytes(),
        }
    }

    pub fn write_i32(self: &mut ModbusBuffer<'a>, value: i32, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_i64(self: &mut ModbusBuffer<'a>, value: i64, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_f32(self: &mut ModbusBuffer<'a>, value: f32, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_f64(self: &mut ModbusBuffer<'a>, value: f64, offset: u16) {
        self.write_bytes(&value.to_be_bytes(), offset)
    }

    pub fn write_ipv4_addr(self: &mut ModbusBuffer<'a>, value: Ipv4Addr, offset: u16) {
        self.write_bytes(&value.octets(), offset)
    }

    pub fn write_ipv6_addr(self: &mut ModbusBuffer<'a>, value: &[u16; 8], offset: u16) {
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

    pub fn write_eui48(self: &mut ModbusBuffer<'a>, value: &[u8; 6], offset: u16) {
        self.write_bytes(value, offset)
    }

    pub fn write_string(self: &mut ModbusBuffer<'a>, str: &CStr, offset: u16) {
        self.buffer.fill([0, 0]);
        self.write_bytes(str.to_bytes_with_nul(), offset);
    }

    pub fn fill(self: &mut ModbusBuffer<'a>, word: &[u8; 2]) {
        self.buffer.fill(order_bytes(word, self.byte_order))
    }

    pub fn zero(self: &mut ModbusBuffer<'a>) {
        self.buffer.fill([0, 0]);
    }

    pub fn write_bytes(self: &mut ModbusBuffer<'a>, bytes: &[u8], offset: u16) {
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
