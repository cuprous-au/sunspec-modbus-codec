use core::net::{Ipv4Addr, Ipv6Addr};

use heapless::{String, c_string};

unsafe extern "C" {
    fn printf(format: *const core::ffi::c_char, ...) -> i32;
}

pub fn write_u16(value: u16, buf: &mut [u16]) -> () {
    buf[0] = value.to_be();
}

pub fn write_u32(value: u32, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_u64(value: u64, buf: &mut [u16], offset: u16, limit: u16) -> () {
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

pub fn write_ipaddr(value: Ipv4Addr, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.octets(), buf, offset, limit)
}

pub fn write_ipv6addr(value: Ipv6Addr, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.octets(), buf, offset, limit)
}

pub fn write_eui48(value: [u8; 6], buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value, buf, offset, limit)
}

pub fn write_string<const N: usize>(
    str: String<N>,
    buf: &mut [u16],
    offset: u16,
    limit: u16,
) -> () {
    let bytes = &str.into_bytes();
    unsafe {
        printf(c"String value: '%s'\n".as_ptr(), heapless::CString::<32>::from_bytes_truncating_at_nul(bytes).unwrap().as_ptr());
    }
    // write_bytes(bytes, buf, offset, limit)
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
            *buf_word = u16::from_be_bytes([chunk[0], *chunk.get(1).unwrap_or(&0_u8)]);
        });
}