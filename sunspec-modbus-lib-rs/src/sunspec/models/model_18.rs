use core::ffi::{CStr, c_char, c_void};
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 24;

pub static POINTS: [ReadablePoint; 7] = [
    ReadablePoint {
        reference: PointReference::Static { value: 18 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 22 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Imei },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Apn },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Number },
        size: 6,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Pin },
        size: 6,
        data_type: PointType::String,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    Name,
    Imei,
    Apn,
    Number,
    Pin,
}

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::Name => {
            if let Some(value) = model.name() {
                buffer::write_string(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Imei => {
            if let Some(value) = model.imei() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Apn => {
            if let Some(value) = model.apn() {
                buffer::write_string(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Number => {
            if let Some(value) = model.number() {
                buffer::write_string(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Pin => {
            if let Some(value) = model.pin() {
                buffer::write_string(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        None
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: &CStr) {
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn imei(&self) -> Option<u32> {
        None
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn set_imei(&mut self, value: u32) {
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn apn(&self) -> Option<&CStr> {
        None
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn set_apn(&mut self, value: &CStr) {
    }

    /// Number
    ///
    /// Phone number for the interface
    fn number(&self) -> Option<&CStr> {
        None
    }

    /// Number
    ///
    /// Phone number for the interface
    fn set_number(&mut self, value: &CStr) {
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn pin(&self) -> Option<&CStr> {
        None
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn set_pin(&mut self, value: &CStr) {
    }
}

#[repr(C)]
pub struct Model18CallbackAdapter {
    context: *mut c_void,
    name_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    imei_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_imei_callback: Option<extern "C" fn(u32, *mut c_void)>,
    apn_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_apn_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    number_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_number_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    pin_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_pin_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
}

impl ModelAdapter for Model18CallbackAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        self.name_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_name_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn imei(&self) -> Option<u32> {
        self.imei_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn set_imei(&mut self, value: u32) {
        if let Some(callback) = self.set_imei_callback {
        (callback)(value, self.context);
        };
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn apn(&self) -> Option<&CStr> {
        self.apn_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn set_apn(&mut self, value: &CStr) {
        if let Some(callback) = self.set_apn_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// Number
    ///
    /// Phone number for the interface
    fn number(&self) -> Option<&CStr> {
        self.number_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// Number
    ///
    /// Phone number for the interface
    fn set_number(&mut self, value: &CStr) {
        if let Some(callback) = self.set_number_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn pin(&self) -> Option<&CStr> {
        self.pin_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn set_pin(&mut self, value: &CStr) {
        if let Some(callback) = self.set_pin_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }
}

#[repr(C)]
pub struct Model18StatefulAdapter {
    name: [c_char; 8],
    imei: u32,
    apn: [c_char; 8],
    number: [c_char; 12],
    pin: [c_char; 12],
}

impl ModelAdapter for Model18StatefulAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.name.as_ptr()) }
        )
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: &CStr) {
        for (dest, src) in self.name.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn imei(&self) -> Option<u32> {
        Some(
        self.imei
        )
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn set_imei(&mut self, value: u32) {
        self.imei = value;
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn apn(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.apn.as_ptr()) }
        )
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn set_apn(&mut self, value: &CStr) {
        for (dest, src) in self.apn.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Number
    ///
    /// Phone number for the interface
    fn number(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.number.as_ptr()) }
        )
    }

    /// Number
    ///
    /// Phone number for the interface
    fn set_number(&mut self, value: &CStr) {
        for (dest, src) in self.number.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn pin(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.pin.as_ptr()) }
        )
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn set_pin(&mut self, value: &CStr) {
        for (dest, src) in self.pin.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }
}