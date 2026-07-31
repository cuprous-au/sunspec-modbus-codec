use core::{
    ffi::{CStr, c_char, c_uint},
    ptr, slice,
};

use crate::{
    ModbusRequest, SunspecService, sunspec::{adapters::SunspecCallbackAdapters, models::model_1::{self, Model1CallbackAdapter}},
};

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn sunspec_service_init(
    service: *mut SunspecService,
    adapter: *const Model1CallbackAdapter,
) {
    unsafe {
        printf(c"%s\n".as_ptr(), c"Init".as_ptr());
    }
    unsafe {
        let adapter_ref: &'static Model1CallbackAdapter = &*adapter;

        ptr::write(
            service,
            SunspecService::new(SunspecCallbackAdapters {
                model_1_adapter: Some(adapter_ref),
                model_2_adapter: None,
                model_3_adapter: None,
                model_4_adapter: None,
                model_5_adapter: None,
                model_6_adapter: None,
                model_7_adapter: None,
                model_8_adapter: None,
                model_10_adapter: None,
                model_11_adapter: None,
                model_12_adapter: None,
                model_13_adapter: None,
                model_15_adapter: None,
                model_16_adapter: None,
                model_17_adapter: None,
                model_18_adapter: None,
                model_19_adapter: None,
                model_101_adapter: None,
                model_102_adapter: None,
                model_103_adapter: None,
                model_111_adapter: None,
                model_112_adapter: None,
                model_113_adapter: None,
                model_120_adapter: None,
                model_121_adapter: None,
                model_122_adapter: None,
                model_123_adapter: None,
                model_124_adapter: None,
                model_125_adapter: None,
                model_126_adapter: None,
                model_127_adapter: None,
                model_128_adapter: None,
                model_129_adapter: None,
                model_130_adapter: None,
                model_131_adapter: None,
                model_132_adapter: None,
                model_133_adapter: None,
                model_134_adapter: None,
                model_135_adapter: None,
                model_136_adapter: None,
                model_137_adapter: None,
                model_138_adapter: None,
                model_139_adapter: None,
                model_140_adapter: None,
                model_141_adapter: None,
                model_142_adapter: None,
                model_143_adapter: None,
                model_144_adapter: None,
                model_145_adapter: None,
                model_160_adapter: None,
                model_201_adapter: None,
                model_202_adapter: None,
                model_203_adapter: None,
                model_204_adapter: None,
                model_211_adapter: None,
                model_212_adapter: None,
                model_213_adapter: None,
                model_214_adapter: None,
                model_220_adapter: None,
                model_305_adapter: None,
                model_306_adapter: None,
                model_307_adapter: None,
                model_308_adapter: None,
                model_401_adapter: None,
                model_402_adapter: None,
                model_403_adapter: None,
                model_404_adapter: None,
                model_501_adapter: None,
                model_502_adapter: None,
                model_701_adapter: None,
                model_703_adapter: None,
                model_704_adapter: None,
                model_705_adapter: None,
                model_706_adapter: None,
                model_707_adapter: None,
                model_708_adapter: None,
                model_709_adapter: None,
                model_710_adapter: None,
                model_711_adapter: None,
                model_712_adapter: None,
                model_713_adapter: None,
                model_714_adapter: None,
                model_715_adapter: None,
                model_801_adapter: None,
                model_802_adapter: None,
                model_803_adapter: None,
                model_804_adapter: None,
                model_805_adapter: None,
                model_806_adapter: None,
                model_807_adapter: None,
                model_808_adapter: None,
                model_809_adapter: None,
                model_63001_adapter: None,
                model_64001_adapter: None,
                model_64020_adapter: None,
                model_64101_adapter: None,
                model_64111_adapter: None,
                model_64112_adapter: None,
                model_64410_adapter: None,
                model_64411_adapter: None,
                model_64412_adapter: None,
                model_64413_adapter: None,
                model_64414_adapter: None,
                model_64415_adapter: None,
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
    response_buffer: *mut u16,
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
