#![cfg_attr(not(feature = "std"), no_std)]

use core::slice;

#[cfg(not(feature = "std"))]
use core::{ffi::c_char, panic::PanicInfo};

use sunspec_modbus_lib_rs::{
    read_registers, sunspec::adapters::SunspecExternalAdapters, write_multiple_registers,
};

#[cfg(not(feature = "std"))]
unsafe extern "C" {
    pub fn handle_panic(message: *const c_char) -> !;
}

#[cfg(not(feature = "std"))]
const MAX_PANIC_MESSAGE_LENGTH: usize = 128;

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let message = panic_info
        .message()
        .as_str()
        .unwrap_or("Unknown Rust panic");

    let mut c_str = [0; MAX_PANIC_MESSAGE_LENGTH + 1];

    for (char, c_char) in message
        .as_bytes()
        .iter()
        .take(MAX_PANIC_MESSAGE_LENGTH)
        .zip(c_str.iter_mut())
    {
        *c_char = (*char) as c_char;
    }
    unsafe { handle_panic(c_str.as_ptr()) }
}

#[unsafe(no_mangle)]
/// Handles a SunSpec Modbus register read request and writes the encoded response.
///
/// # Safety
/// - `adapters` must be a valid, non-null pointer to a live `SunspecExternalAdapters`.
/// - `response_buffer` must be a valid, non-null, writable buffer of at least
///   `length * 2` bytes.
/// - Both pointers must remain valid for the duration of this call.
pub unsafe extern "C" fn sunspec_service_read_registers(
    adapters: *const SunspecExternalAdapters,
    address: u16,
    length: u16,
    response_buffer: *mut u8,
) -> i32 {
    if adapters.is_null() {
        return -1;
    }
    if response_buffer.is_null() {
        return -1;
    }
    let adapter_ref = unsafe { &*adapters };
    let buf = unsafe { slice::from_raw_parts_mut(response_buffer, (length as usize) * 2) };

    match read_registers(adapter_ref, address, buf) {
        Ok(()) => 0,
        Err(_) => -2,
    }
}

#[unsafe(no_mangle)]
/// Handles a SunSpec Modbus register write request and invokes the relevant updates in the Sunspec model.
///
/// # Safety
/// - `adapters` must be a valid, non-null pointer to a live `SunspecExternalAdapters`.
/// - `request_buffer` must be a valid, non-null, buffer of at least `length * 2` bytes.
/// - Both pointers must remain valid for the duration of this call.
pub unsafe extern "C" fn sunspec_service_write_registers(
    adapters: *mut SunspecExternalAdapters,
    address: u16,
    length: u16,
    request_buffer: *const u8,
) -> i32 {
    if adapters.is_null() {
        return -1;
    }
    if request_buffer.is_null() {
        return -1;
    }
    let adapter_ref = unsafe { &mut *adapters };
    let buf = unsafe { slice::from_raw_parts(request_buffer, (length as usize) * 2) };

    match write_multiple_registers(adapter_ref, address, buf) {
        Ok(()) => 0,
        Err(_) => -2,
    }
}
