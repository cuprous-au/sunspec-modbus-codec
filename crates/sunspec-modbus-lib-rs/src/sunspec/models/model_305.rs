use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 38;

static POINTS: [PointDetails<()>; 8] = [
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
        point: |()| Point::Tm,
        size: 6,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::Date,
        size: 4,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::Location,
        size: 20,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::Lat,
        size: 2,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::Long,
        size: 2,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::Altitude,
        size: 2,
        start_address: 36,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    Tm,
    Date,
    Location,
    Lat,
    Long,
    Altitude,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    38
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
            buffer::write_u16(305, buffer);
        }
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
