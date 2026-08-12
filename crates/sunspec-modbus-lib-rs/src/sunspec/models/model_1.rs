use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 68;

static POINTS: [PointDetails<()>; 9] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::Manufacturer,
        size: 16,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::Model,
        size: 16,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::Options,
        size: 8,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::Version,
        size: 8,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::SerialNumber,
        size: 16,
        start_address: 50,
    },
    PointDetails {
        point: |()| Point::DeviceAddress,
        size: 1,
        start_address: 66,
    },
    PointDetails {
        point: |()| Point::Pad,
        size: 1,
        start_address: 67,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    Manufacturer,
    Model,
    Options,
    Version,
    SerialNumber,
    DeviceAddress,
    Pad,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    68
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(1, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Manufacturer => {
            buffer::write_string(model.manufacturer(), buffer, offset, limit);
        }
        Point::Model => {
            buffer::write_string(model.model(), buffer, offset, limit);
        }
        Point::Options => {
            if let Some(value) = model.options() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 8);
            }
        }
        Point::Version => {
            if let Some(value) = model.version() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 8);
            }
        }
        Point::SerialNumber => {
            buffer::write_string(model.serial_number(), buffer, offset, limit);
        }
        Point::DeviceAddress => {
            if let Some(value) = model.device_address() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Pad => {
            buffer::write_u16(0, buffer);
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
    context: *mut c_void,
    manufacturer_callback: extern "C" fn(*const c_void) -> *const c_char,
    model_callback: extern "C" fn(*const c_void) -> *const c_char,
    options_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    version_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    serial_number_callback: extern "C" fn(*const c_void) -> *const c_char,
    device_address_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_device_address_callback: Option<extern "C" fn(u16, *mut c_void)>,
}

impl ModelAdapter for Model1CallbackAdapter {
    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    fn manufacturer(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.manufacturer_callback)(self.context)) }
    }

    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    fn model(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.model_callback)(self.context)) }
    }

    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    fn options(&self) -> Option<&CStr> {
        self.options_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    fn version(&self) -> Option<&CStr> {
        self.version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    fn serial_number(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.serial_number_callback)(self.context)) }
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn device_address(&self) -> Option<u16> {
        self.device_address_callback
            .map(|callback| (callback)(self.context))
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn set_device_address(&mut self, value: u16) {
        if let Some(callback) = self.set_device_address_callback {
            (callback)(value, self.context);
        };
    }
}

#[repr(C)]
pub struct Model1StatefulAdapter {
    manufacturer: [c_char; 32],
    model: [c_char; 32],
    options: [c_char; 16],
    version: [c_char; 16],
    serial_number: [c_char; 32],
    device_address: u16,
}

impl ModelAdapter for Model1StatefulAdapter {
    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    fn manufacturer(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.manufacturer.as_ptr()) }
    }

    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    fn model(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.model.as_ptr()) }
    }

    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    fn options(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.options.as_ptr()) })
    }

    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    fn version(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.version.as_ptr()) })
    }

    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    fn serial_number(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.serial_number.as_ptr()) }
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn device_address(&self) -> Option<u16> {
        Some(self.device_address)
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn set_device_address(&mut self, value: u16) {
        self.device_address = value;
    }
}
