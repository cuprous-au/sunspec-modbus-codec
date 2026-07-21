use core::{
    ffi::{CStr, c_char},
    ptr, slice,
};

use heapless::String;

use crate::{ModbusRequest, SunspecModelAdapters, SunspecService, sunspec::models::model_1};

#[repr(C)]
pub struct Model1Adapter {
    manufacturer_callback: extern "C" fn() -> *const c_char,
    model_callback: extern "C" fn() -> *const c_char,
    options_callback: Option<extern "C" fn() -> *const c_char>,
    version_callback: Option<extern "C" fn() -> *const c_char>,
    serial_number_callback: extern "C" fn() -> *const c_char,
    device_address_callback: Option<extern "C" fn() -> u16>,
    set_device_address_callback: extern "C" fn(value: u16),
}

fn callback_to_string(callback: extern "C" fn() -> *const c_char) -> String<32> {
    let ptr = callback();
    if ptr.is_null() {
        return String::new();
    }

    let str = unsafe { CStr::from_ptr(ptr) };
    let Ok(str) = str.to_str() else {
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
        callback_to_string(self.serial_number_callback)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn sunspec_service_init(
    adapter: *const Model1Adapter,
) -> SunspecService<'static> {
    let adapter_ref: &'static Model1Adapter = unsafe { &*adapter };
    let service = SunspecService::new(SunspecModelAdapters {
        model_1_adapter: Some(adapter_ref),
        model_103_adapter: None,
    });

    service
}

#[unsafe(no_mangle)]
pub extern "C" fn sunspec_service_handle_request(
    service: *const SunspecService<'static>,
    register: u16,
    length: u16,
    response_buffer: *mut u16,
    buffer_len: usize,
) -> i32 {
    if service.is_null() || response_buffer.is_null() {
        return -1;
    }

    let service = unsafe { &*service };
    let buf = unsafe { slice::from_raw_parts_mut(response_buffer, buffer_len) };
    match service.handle_request(ModbusRequest::ReadRegister(register, length), buf) {
        Ok(()) => 0,
        Err(_) => -2,
    }
}
