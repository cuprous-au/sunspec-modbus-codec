use heapless::String;

pub type RegisterReader<A> =
    fn(model: &A, buf: &mut [u8], offset_bytes: usize, limit_bytes: usize) -> ();

pub fn write_u16(value: u16, buf: &mut [u16]) -> () {
    buf[0] = value;
}

pub fn write_i16(value: i16, buf: &mut [u16]) -> () {
    buf[0] = value as u16;
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

pub fn write_u32(value: u32, buf: &mut [u16], offset: u16, limit: u16) -> () {
    write_bytes(&value.to_be_bytes(), buf, offset, limit)
}

pub fn write_string<const N: usize>(
    str: String<N>,
    buf: &mut [u16],
    offset: u16,
    limit: u16,
) -> () {
    write_bytes(&str.into_bytes(), buf, offset, limit)
}
