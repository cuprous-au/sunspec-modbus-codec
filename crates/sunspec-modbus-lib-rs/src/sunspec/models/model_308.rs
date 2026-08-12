use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 6;

static POINTS: [PointDetails<()>; 6] = [
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
        point: |()| Point::Ghi,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::Temp,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::AmbientTemperature,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::WindSpeed,
        size: 1,
        start_address: 5,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    Ghi,
    Temp,
    AmbientTemperature,
    WindSpeed,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    6
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
            buffer::write_u16(308, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Ghi => {
            if let Some(value) = model.ghi() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::AmbientTemperature => {
            if let Some(value) = model.ambient_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::WindSpeed => {
            if let Some(value) = model.wind_speed() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
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
        self.ghi_callback.map(|callback| (callback)(self.context))
    }

    /// Temp
    ///
    /// Back of module temperature measurement
    fn temp(&self) -> Option<i16> {
        self.temp_callback.map(|callback| (callback)(self.context))
    }

    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        self.ambient_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<u16> {
        self.wind_speed_callback
            .map(|callback| (callback)(self.context))
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
        Some(self.ghi)
    }

    /// Temp
    ///
    /// Back of module temperature measurement
    fn temp(&self) -> Option<i16> {
        Some(self.temp)
    }

    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        Some(self.ambient_temperature)
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<u16> {
        Some(self.wind_speed)
    }
}
