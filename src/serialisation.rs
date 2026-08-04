use core::{
    cmp::min,
    ffi::CStr,
    net::{Ipv4Addr},
};

pub fn write_u16(value: u16, buf: &mut [u16]) -> () {
    buf[0] = value.to_be();
}

pub fn write_u32(value: u32, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_u64(value: u64, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_u128(value: u128, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_i16(value: i16, buf: &mut [u16]) -> () {
    buf[0] = (value.to_be()) as u16;
}

pub fn write_i32(value: i32, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_i64(value: i64, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_f32(value: f32, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_f64(value: f64, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_ipv4_addr(value: Ipv4Addr, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.octets(), buf, offset, limit)
}

pub fn write_ipv6_addr(value: &[u16; 8], buf: &mut [u16], offset: u16, limit: u16) -> () {
    let word_limit = min(8, (offset + limit) as usize);
    buf.copy_from_slice(&value[offset as usize..word_limit])
}

pub fn write_eui48(value: &[u8; 6], buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(value, buf, offset, limit)
}

pub fn write_string(str: &CStr, buf: &mut [u16], offset: u16, limit: u16) -> () {
    buf.fill(0);
    write_bytes(&str.to_bytes_with_nul(), buf, offset, limit);
}

pub fn write_bytes(bytes: &[u8], buf: &mut [u16], offset: u16, limit: u16) {
    let (chunks, remainder) = bytes.as_chunks();

    let last_chunk = remainder.first().map(|byte| [*byte, 0_u8]);

    chunks
        .into_iter()
        .chain(&last_chunk)
        .skip(offset as usize)
        .take(limit as usize)
        .zip(buf)
        .for_each(|(chunk, buf_word)| {
            *buf_word = u16::from_le_bytes(*chunk);
        });
}
