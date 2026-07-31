use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

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
        reference: PointReference::Model18 {
            point: Point::Number,
        },
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Name => {
            if let Some(value) = model.name() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Imei => {
            if let Some(value) = model.imei() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::Apn => {
            if let Some(value) = model.apn() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Number => {
            if let Some(value) = model.number() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Pin => {
            if let Some(value) = model.pin() {
                serialisation::write_string(value, buffer, offset, limit);
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
    fn set_name(&mut self, value: &CStr) {}

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn imei(&self) -> Option<u32> {
        None
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn set_imei(&mut self, value: u32) {}

    /// APN
    ///
    /// Access Point Name for the interface
    fn apn(&self) -> Option<&CStr> {
        None
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn set_apn(&mut self, value: &CStr) {}

    /// Number
    ///
    /// Phone number for the interface
    fn number(&self) -> Option<&CStr> {
        None
    }

    /// Number
    ///
    /// Phone number for the interface
    fn set_number(&mut self, value: &CStr) {}

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn pin(&self) -> Option<&CStr> {
        None
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn set_pin(&mut self, value: &CStr) {}
}

#[repr(C)]
pub struct Model18CallbackAdapter {
    name_callback: Option<extern "C" fn() -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char)>,
    imei_callback: Option<extern "C" fn() -> u32>,
    set_imei_callback: Option<extern "C" fn(u32)>,
    apn_callback: Option<extern "C" fn() -> *const c_char>,
    set_apn_callback: Option<extern "C" fn(*const c_char)>,
    number_callback: Option<extern "C" fn() -> *const c_char>,
    set_number_callback: Option<extern "C" fn(*const c_char)>,
    pin_callback: Option<extern "C" fn() -> *const c_char>,
    set_pin_callback: Option<extern "C" fn(*const c_char)>,
}

impl ModelAdapter for Model18CallbackAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        self.name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_name_callback {
            (callback)(value.as_ptr());
        };
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn imei(&self) -> Option<u32> {
        self.imei_callback.map(|callback| (callback)())
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn set_imei(&mut self, value: u32) {
        if let Some(callback) = self.set_imei_callback {
            (callback)(value);
        };
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn apn(&self) -> Option<&CStr> {
        self.apn_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn set_apn(&mut self, value: &CStr) {
        if let Some(callback) = self.set_apn_callback {
            (callback)(value.as_ptr());
        };
    }

    /// Number
    ///
    /// Phone number for the interface
    fn number(&self) -> Option<&CStr> {
        self.number_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Number
    ///
    /// Phone number for the interface
    fn set_number(&mut self, value: &CStr) {
        if let Some(callback) = self.set_number_callback {
            (callback)(value.as_ptr());
        };
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn pin(&self) -> Option<&CStr> {
        self.pin_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn set_pin(&mut self, value: &CStr) {
        if let Some(callback) = self.set_pin_callback {
            (callback)(value.as_ptr());
        };
    }
}
