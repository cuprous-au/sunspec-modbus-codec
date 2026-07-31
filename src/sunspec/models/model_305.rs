use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 38;

pub static POINTS: [ReadablePoint; 8] = [
    ReadablePoint {
        reference: PointReference::Static { value: 305 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 36 },
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
    Tm,
    Date,
    Location,
    Lat,
    Long,
    Altitude,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Tm => {
            if let Some(value) = model.tm() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Date => {
            if let Some(value) = model.date() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Location => {
            if let Some(value) = model.location() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Lat => {
            if let Some(value) = model.lat() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::Long => {
            if let Some(value) = model.long() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::Altitude => {
            if let Some(value) = model.altitude() {
                serialisation::write_i32(value, buffer, offset, limit);
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
    tm_callback: Option<extern "C" fn() -> *const c_char>,
    date_callback: Option<extern "C" fn() -> *const c_char>,
    location_callback: Option<extern "C" fn() -> *const c_char>,
    lat_callback: Option<extern "C" fn() -> i32>,
    long_callback: Option<extern "C" fn() -> i32>,
    altitude_callback: Option<extern "C" fn() -> i32>,
}

impl ModelAdapter for Model305CallbackAdapter {
    /// Tm
    ///
    /// UTC 24 hour time stamp to millisecond hhmmss.sssZ format
    fn tm(&self) -> Option<&CStr> {
        self.tm_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Date
    ///
    /// UTC Date string YYYYMMDD format
    fn date(&self) -> Option<&CStr> {
        self.date_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Location
    ///
    /// Location string (40 chars max)
    fn location(&self) -> Option<&CStr> {
        self.location_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Lat
    ///
    /// Latitude with seven degrees of precision
    fn lat(&self) -> Option<i32> {
        self.lat_callback.map(|callback| (callback)())
    }

    /// Long
    ///
    /// Longitude with seven degrees of precision
    fn long(&self) -> Option<i32> {
        self.long_callback.map(|callback| (callback)())
    }

    /// Altitude
    ///
    /// Altitude measurement in meters
    fn altitude(&self) -> Option<i32> {
        self.altitude_callback.map(|callback| (callback)())
    }
}
