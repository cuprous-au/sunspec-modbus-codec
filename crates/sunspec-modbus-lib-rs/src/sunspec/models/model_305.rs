use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 38;

pub static POINTS: [ReadablePoint; 8] = [
    ReadablePoint {
        reference: PointReference::Static { value: 305 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model305 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model305 { point: Point::Tm },
        size: 6,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model305 { point: Point::Date },
        size: 4,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model305 {
            point: Point::Location,
        },
        size: 20,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model305 { point: Point::Lat },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model305 { point: Point::Long },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model305 {
            point: Point::Altitude,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    Tm,
    Date,
    Location,
    Lat,
    Long,
    Altitude,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    38
}

pub fn write_point<'a, 'b>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Tm => {
            if let Some(value) = model.tm() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Date => {
            if let Some(value) = model.date() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Location => {
            if let Some(value) = model.location() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Lat => {
            if let Some(value) = model.lat() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Long => {
            if let Some(value) = model.long() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Altitude => {
            if let Some(value) = model.altitude() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Tm
    ///
    /// UTC 24 hour time stamp to millisecond hhmmss.sssZ format
    fn tm(&self) -> Option<&CStr> {
        None
    }

    /// Date
    ///
    /// UTC Date string YYYYMMDD format
    fn date(&self) -> Option<&CStr> {
        None
    }

    /// Location
    ///
    /// Location string (40 chars max)
    fn location(&self) -> Option<&CStr> {
        None
    }

    /// Lat
    ///
    /// Latitude with seven degrees of precision
    fn lat(&self) -> Option<i32> {
        None
    }

    /// Long
    ///
    /// Longitude with seven degrees of precision
    fn long(&self) -> Option<i32> {
        None
    }

    /// Altitude
    ///
    /// Altitude measurement in meters
    fn altitude(&self) -> Option<i32> {
        None
    }
}

#[repr(C)]
pub struct Model305CallbackAdapter {
    context: *mut c_void,
    tm_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    date_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    location_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    lat_callback: Option<extern "C" fn(*const c_void) -> i32>,
    long_callback: Option<extern "C" fn(*const c_void) -> i32>,
    altitude_callback: Option<extern "C" fn(*const c_void) -> i32>,
}

impl ModelAdapter for Model305CallbackAdapter {
    /// Tm
    ///
    /// UTC 24 hour time stamp to millisecond hhmmss.sssZ format
    fn tm(&self) -> Option<&CStr> {
        self.tm_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Date
    ///
    /// UTC Date string YYYYMMDD format
    fn date(&self) -> Option<&CStr> {
        self.date_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Location
    ///
    /// Location string (40 chars max)
    fn location(&self) -> Option<&CStr> {
        self.location_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Lat
    ///
    /// Latitude with seven degrees of precision
    fn lat(&self) -> Option<i32> {
        self.lat_callback.map(|callback| (callback)(self.context))
    }

    /// Long
    ///
    /// Longitude with seven degrees of precision
    fn long(&self) -> Option<i32> {
        self.long_callback.map(|callback| (callback)(self.context))
    }

    /// Altitude
    ///
    /// Altitude measurement in meters
    fn altitude(&self) -> Option<i32> {
        self.altitude_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model305StatefulAdapter {
    tm: [c_char; 12],
    date: [c_char; 8],
    location: [c_char; 40],
    lat: i32,
    long: i32,
    altitude: i32,
}

impl ModelAdapter for Model305StatefulAdapter {
    /// Tm
    ///
    /// UTC 24 hour time stamp to millisecond hhmmss.sssZ format
    fn tm(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.tm.as_ptr()) })
    }

    /// Date
    ///
    /// UTC Date string YYYYMMDD format
    fn date(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.date.as_ptr()) })
    }

    /// Location
    ///
    /// Location string (40 chars max)
    fn location(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.location.as_ptr()) })
    }

    /// Lat
    ///
    /// Latitude with seven degrees of precision
    fn lat(&self) -> Option<i32> {
        Some(self.lat)
    }

    /// Long
    ///
    /// Longitude with seven degrees of precision
    fn long(&self) -> Option<i32> {
        Some(self.long)
    }

    /// Altitude
    ///
    /// Altitude measurement in meters
    fn altitude(&self) -> Option<i32> {
        Some(self.altitude)
    }
}
