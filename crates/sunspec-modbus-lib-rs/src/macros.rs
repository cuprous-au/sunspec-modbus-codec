use core::ffi::c_char;

/// Builds a fixed-length `[c_char; N]` from a `&str`, zero-padding any
/// remaining elements. `N` is inferred from the context the macro is used in.
///
/// Panics (at compile time) if the string is longer than `N-1`, ensuring the string is null-terminated.
#[macro_export]
macro_rules! c_char_array {
    ($s:expr) => {
        $crate::macros::c_char_array::<_>($s)
    };
}

#[doc(hidden)]
pub const fn c_char_array<const N: usize>(s: &str) -> [c_char; N] {
    let bytes = s.as_bytes();
    assert!(bytes.len() < N, "string does not fit in array");

    let mut array = [0 as c_char; N];
    let mut i = 0;
    while i < bytes.len() {
        array[i] = bytes[i] as c_char;
        i += 1;
    }
    array
}

#[cfg(test)]
mod tests {
    use core::ffi::c_char;

    #[test]
    fn pads_remaining_elements_with_zero() {
        let array: [c_char; 5] = c_char_array!("ab");
        assert_eq!(array, [b'a' as c_char, b'b' as c_char, 0, 0, 0]);
    }

    #[test]
    fn fills_array_exactly() {
        let array: [c_char; 3] = c_char_array!("abc");
        assert_eq!(array, [b'a' as c_char, b'b' as c_char, b'c' as c_char]);
    }
}
