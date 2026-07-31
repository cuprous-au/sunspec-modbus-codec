use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 68;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 1 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 66 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 {
            point: Point::Manufacturer,
        },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 {
            point: Point::Model,
        },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 {
            point: Point::Options,
        },
        size: 8,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 {
            point: Point::Version,
        },
        size: 8,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 {
            point: Point::SerialNumber,
        },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 {
            point: Point::DeviceAddress,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Manufacturer,
    Model,
    Options,
    Version,
    SerialNumber,
    DeviceAddress,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Manufacturer => {
            serialisation::write_string(model.manufacturer(), buffer, offset, limit)
        }
        Point::Model => serialisation::write_string(model.model(), buffer, offset, limit),
        Point::Options => {
            if let Some(value) = model.options() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Version => {
            if let Some(value) = model.version() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::SerialNumber => {
            serialisation::write_string(model.serial_number(), buffer, offset, limit)
        }
        Point::DeviceAddress => {
            if let Some(value) = model.device_address() {
                serialisation::write_u16(value, buffer);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    fn manufacturer(&self) -> &CStr;

    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    fn model(&self) -> &CStr;

    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    fn options(&self) -> Option<&CStr> {
        None
    }

    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    fn version(&self) -> Option<&CStr> {
        None
    }

    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    fn serial_number(&self) -> &CStr;

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn device_address(&self) -> Option<u16> {
        None
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn set_device_address(&mut self, value: u16) {}
}

#[repr(C)]
pub struct Model1CallbackAdapter {
    manufacturer_callback: extern "C" fn() -> *const c_char,
    model_callback: extern "C" fn() -> *const c_char,
    options_callback: Option<extern "C" fn() -> *const c_char>,
    version_callback: Option<extern "C" fn() -> *const c_char>,
    serial_number_callback: extern "C" fn() -> *const c_char,
    device_address_callback: Option<extern "C" fn() -> u16>,
    set_device_address_callback: Option<extern "C" fn(u16)>,
}

impl ModelAdapter for Model1CallbackAdapter {
    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    fn manufacturer(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.manufacturer_callback)()) }
    }

    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    fn model(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.model_callback)()) }
    }

    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    fn options(&self) -> Option<&CStr> {
        self.options_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    fn version(&self) -> Option<&CStr> {
        self.version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    fn serial_number(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.serial_number_callback)()) }
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn device_address(&self) -> Option<u16> {
        self.device_address_callback.map(|callback| (callback)())
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn set_device_address(&mut self, value: u16) {
        if let Some(callback) = self.set_device_address_callback {
            (callback)(value);
        };
    }
}
