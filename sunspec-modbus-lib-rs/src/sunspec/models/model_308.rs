use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 6;

pub static POINTS: [ReadablePoint; 6] = [
    ReadablePoint {
        reference: PointReference::Static { value: 308 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 4 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model308 { point: Point::Ghi },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model308 { point: Point::Temp },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model308 { point: Point::AmbientTemperature },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model308 { point: Point::WindSpeed },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Ghi,
    Temp,
    AmbientTemperature,
    WindSpeed,
}

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::Ghi => {
            if let Some(value) = model.ghi() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                buffer::write_i16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AmbientTemperature => {
            if let Some(value) = model.ambient_temperature() {
                buffer::write_i16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WindSpeed => {
            if let Some(value) = model.wind_speed() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        None
    }

    /// Temp
    ///
    /// Back of module temperature measurement
    fn temp(&self) -> Option<i16> {
        None
    }

    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        None
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model308CallbackAdapter {
    context: *mut c_void,
    ghi_callback: Option<extern "C" fn(*const c_void) -> u16>,
    temp_callback: Option<extern "C" fn(*const c_void) -> i16>,
    ambient_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    wind_speed_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model308CallbackAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        self.ghi_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Temp
    ///
    /// Back of module temperature measurement
    fn temp(&self) -> Option<i16> {
        self.temp_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        self.ambient_temperature_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<u16> {
        self.wind_speed_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model308StatefulAdapter {
    ghi: u16,
    temp: i16,
    ambient_temperature: i16,
    wind_speed: u16,
}

impl ModelAdapter for Model308StatefulAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        Some(
        self.ghi
        )
    }

    /// Temp
    ///
    /// Back of module temperature measurement
    fn temp(&self) -> Option<i16> {
        Some(
        self.temp
        )
    }

    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        Some(
        self.ambient_temperature
        )
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<u16> {
        Some(
        self.wind_speed
        )
    }
}