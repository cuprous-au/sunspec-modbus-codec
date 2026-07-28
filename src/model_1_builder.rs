use core::{
    ffi::{CStr, c_char, c_uint}, ptr, slice,
};

use heapless::String;

use crate::{ModbusRequest, SunspecModelAdapters, SunspecService, sunspec::models::model_1};

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}

#[repr(C)]
pub struct Model1Adapter {
    manufacturer_callback: extern "C" fn() -> *const c_char,
    model_callback: extern "C" fn() -> *const c_char,
    options_callback: Option<extern "C" fn() -> *const c_char>,
    version_callback: Option<extern "C" fn() -> *const c_char>,
    serial_number_callback: extern "C" fn() -> *const c_char,
    device_address_callback: Option<extern "C" fn() -> u16>,
    set_device_address_callback: Option<extern "C" fn(value: u16)>,
}

fn callback_to_string(callback: extern "C" fn() -> *const c_char) -> String<32> {
    let ptr = callback();
    if ptr.is_null() {
        log(c"ptr is null");
        return String::new();
    }

    let str = unsafe { CStr::from_ptr(ptr) };
    let Ok(str) = str.to_str() else {
        log(c"str can't be cast");
        return String::new();
    };

    String::<32>::try_from(str).unwrap_or_default()
}

impl model_1::ModelAdapter for Model1Adapter {
    fn manufacturer(&self) -> String<32> {
        callback_to_string(self.manufacturer_callback)
    }

    fn model(&self) -> String<32> {
        callback_to_string(self.model_callback)
    }

    fn serial_number(&self) -> String<32> {
        callback_to_string(self.manufacturer_callback)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sunspec_service_init(
    service: *mut SunspecService,
    adapter: *const Model1Adapter,
) {
    unsafe {
        printf(c"%s\n".as_ptr(), c"Init".as_ptr());
    }
    unsafe {
        let adapter_ref: &'static Model1Adapter = &*adapter;

        ptr::write(
            service,
            SunspecService::new(SunspecModelAdapters {
                model_1_adapter: Some(adapter_ref),
                // model_103_adapter: None,
            }),
        );
    }
}

pub fn log(str: &CStr) {
    unsafe {
        printf(str.as_ptr());
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sunspec_service_handle_request(
    service: *const SunspecService,
    address: u16,
    length: u16,
    response_buffer: *mut u16
) -> i32 {
    log(c"sunspec_service_handle_request\n");
    if service.is_null() {
        log(c"service is null\n");
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
    let service = unsafe { &*service };
    let buf = unsafe { slice::from_raw_parts_mut(response_buffer, length as usize) };
    match service.handle_request(ModbusRequest::ReadRegister(address, length), buf) {
        Ok(()) => 0,
        Err(_) => -2,
    }
}
