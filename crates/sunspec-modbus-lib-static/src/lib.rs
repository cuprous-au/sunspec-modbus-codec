#![no_std]
use core::{
    ffi::{CStr, c_char, c_uint},
    panic::PanicInfo,
    slice,
};

use sunspec_modbus_lib_rs::{
    ModbusRequest, handle_request, sunspec::adapters::SunspecExternalAdapters,
};

unsafe extern "C" {
    pub fn printf(format: *const c_char, ...) -> i32;
    pub fn handle_panic(message: *const c_char) -> !;
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    let message = panic_info
        .message()
        .as_str()
        .unwrap_or("Unknown Rust panic");
    unsafe { handle_panic(message.as_ptr() as *const i8) }
}

pub fn log(str: &CStr) {
    unsafe {
        printf(str.as_ptr());
    }
}

#[unsafe(no_mangle)]
/// Handles a SunSpec Modbus register read request and writes the encoded response.
///
/// # Safety
/// - `adapters` must be a valid, non-null pointer to a live `SunspecExternalAdapters`.
/// - `response_buffer` must be a valid, non-null, writable buffer of at least
///   `length * 2` bytes.
/// - Both pointers must remain valid for the duration of this call.
pub unsafe extern "C" fn sunspec_service_handle_request(
    adapters: *const SunspecExternalAdapters,
    address: u16,
    length: u16,
    response_buffer: *mut u8,
) -> i32 {
    log(c"sunspec_service_handle_request\n");
    if adapters.is_null() {
        log(c"adapters is null\n");
        return -1;
    }
    if response_buffer.is_null() {
        log(c"response_buffer is null\n");
        return -1;
    }
    unsafe {
        printf(c"service and response buffer are not null\n".as_ptr());
        printf(c"%d -> %d\n".as_ptr(), address as c_uint, length as c_uint);
    }
    let adapter_ref = unsafe { &*adapters };
    let buf = unsafe { slice::from_raw_parts_mut(response_buffer, (length * 2) as usize) };

    let res = match handle_request(
        adapter_ref,
        ModbusRequest::ReadRegister(address, length),
        buf,
    ) {
        Ok(()) => 0,
        Err(_) => -2,
    };

    unsafe { printf(c"res %d\n".as_ptr(), res as c_uint) };

    res
}
