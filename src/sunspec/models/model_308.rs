use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

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
        reference: PointReference::Model308 {
            point: Point::AmbientTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model308 {
            point: Point::WindSpeed,
        },
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Ghi => {
            if let Some(value) = model.ghi() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::AmbientTemperature => {
            if let Some(value) = model.ambient_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WindSpeed => {
            if let Some(value) = model.wind_speed() {
                serialisation::write_u16(value, buffer);
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
    ghi_callback: Option<extern "C" fn() -> u16>,
    temp_callback: Option<extern "C" fn() -> i16>,
    ambient_temperature_callback: Option<extern "C" fn() -> i16>,
    wind_speed_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model308CallbackAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        self.ghi_callback.map(|callback| (callback)())
    }

    /// Temp
    ///
    /// Back of module temperature measurement
    fn temp(&self) -> Option<i16> {
        self.temp_callback.map(|callback| (callback)())
    }

    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        self.ambient_temperature_callback
            .map(|callback| (callback)())
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<u16> {
        self.wind_speed_callback.map(|callback| (callback)())
    }
}
