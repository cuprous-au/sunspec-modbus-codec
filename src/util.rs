use heapless::String;

pub type RegisterReader<A> =
    fn(model: &A, buf: &mut [u8], offset_bytes: usize, limit_bytes: usize) -> ();

pub fn write_u16(value: u16, buf: &mut [u8]) -> () {
    buf.copy_from_slice(&value.to_be_bytes());
}

pub fn write_i16(value: i16, buf: &mut [u8]) -> () {
    buf.copy_from_slice(&value.to_be_bytes());
}

pub fn write_bytes(bytes: &[u8], buf: &mut [u8], offset: usize, len: usize) {
    let bytes_to_copy = core::cmp::min(len, bytes.len() - offset);
    buf[..bytes_to_copy].copy_from_slice(&bytes[offset..offset + bytes_to_copy]);
}

pub fn write_u32(value: u32, buf: &mut [u8], offset: usize, len: usize) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, len)
}

pub fn write_string<const N: usize>(
    str: String<N>,
    buf: &mut [u8],
    offset: usize,
    len: usize,
) -> () {
    write_bytes(&str.into_bytes(), buf, offset, len)
}
