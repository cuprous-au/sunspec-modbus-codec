use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 13;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 307 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::AmbientTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::RelativeHumidity,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::BarometricPressure,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::WindSpeed,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::WindDirection,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::Rainfall,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::SnowDepth,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::PrecipitationType,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::ElectricField,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::SurfaceWetness,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model307 {
            point: Point::SoilWetness,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    AmbientTemperature,
    RelativeHumidity,
    BarometricPressure,
    WindSpeed,
    WindDirection,
    Rainfall,
    SnowDepth,
    PrecipitationType,
    ElectricField,
    SurfaceWetness,
    SoilWetness,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    13
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::AmbientTemperature => {
            if let Some(value) = model.ambient_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RelativeHumidity => {
            if let Some(value) = model.relative_humidity() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::BarometricPressure => {
            if let Some(value) = model.barometric_pressure() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WindSpeed => {
            if let Some(value) = model.wind_speed() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WindDirection => {
            if let Some(value) = model.wind_direction() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Rainfall => {
            if let Some(value) = model.rainfall() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SnowDepth => {
            if let Some(value) = model.snow_depth() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrecipitationType => {
            if let Some(value) = model.precipitation_type() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ElectricField => {
            if let Some(value) = model.electric_field() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SurfaceWetness => {
            if let Some(value) = model.surface_wetness() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SoilWetness => {
            if let Some(value) = model.soil_wetness() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        None
    }

    /// Relative Humidity
    fn relative_humidity(&self) -> Option<i16> {
        None
    }

    /// Barometric Pressure
    fn barometric_pressure(&self) -> Option<i16> {
        None
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<i16> {
        None
    }

    /// Wind Direction
    fn wind_direction(&self) -> Option<i16> {
        None
    }

    /// Rainfall
    fn rainfall(&self) -> Option<i16> {
        None
    }

    /// Snow Depth
    fn snow_depth(&self) -> Option<i16> {
        None
    }

    /// Precipitation Type
    ///
    /// Precipitation Type (WMO 4680 SYNOP code reference)
    fn precipitation_type(&self) -> Option<i16> {
        None
    }

    /// Electric Field
    fn electric_field(&self) -> Option<i16> {
        None
    }

    /// Surface Wetness
    fn surface_wetness(&self) -> Option<i16> {
        None
    }

    /// Soil Wetness
    fn soil_wetness(&self) -> Option<i16> {
        None
    }
}

#[repr(C)]
pub struct Model307CallbackAdapter {
    context: *mut c_void,
    ambient_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    relative_humidity_callback: Option<extern "C" fn(*const c_void) -> i16>,
    barometric_pressure_callback: Option<extern "C" fn(*const c_void) -> i16>,
    wind_speed_callback: Option<extern "C" fn(*const c_void) -> i16>,
    wind_direction_callback: Option<extern "C" fn(*const c_void) -> i16>,
    rainfall_callback: Option<extern "C" fn(*const c_void) -> i16>,
    snow_depth_callback: Option<extern "C" fn(*const c_void) -> i16>,
    precipitation_type_callback: Option<extern "C" fn(*const c_void) -> i16>,
    electric_field_callback: Option<extern "C" fn(*const c_void) -> i16>,
    surface_wetness_callback: Option<extern "C" fn(*const c_void) -> i16>,
    soil_wetness_callback: Option<extern "C" fn(*const c_void) -> i16>,
}

impl ModelAdapter for Model307CallbackAdapter {
    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        self.ambient_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Relative Humidity
    fn relative_humidity(&self) -> Option<i16> {
        self.relative_humidity_callback
            .map(|callback| (callback)(self.context))
    }

    /// Barometric Pressure
    fn barometric_pressure(&self) -> Option<i16> {
        self.barometric_pressure_callback
            .map(|callback| (callback)(self.context))
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<i16> {
        self.wind_speed_callback
            .map(|callback| (callback)(self.context))
    }

    /// Wind Direction
    fn wind_direction(&self) -> Option<i16> {
        self.wind_direction_callback
            .map(|callback| (callback)(self.context))
    }

    /// Rainfall
    fn rainfall(&self) -> Option<i16> {
        self.rainfall_callback
            .map(|callback| (callback)(self.context))
    }

    /// Snow Depth
    fn snow_depth(&self) -> Option<i16> {
        self.snow_depth_callback
            .map(|callback| (callback)(self.context))
    }

    /// Precipitation Type
    ///
    /// Precipitation Type (WMO 4680 SYNOP code reference)
    fn precipitation_type(&self) -> Option<i16> {
        self.precipitation_type_callback
            .map(|callback| (callback)(self.context))
    }

    /// Electric Field
    fn electric_field(&self) -> Option<i16> {
        self.electric_field_callback
            .map(|callback| (callback)(self.context))
    }

    /// Surface Wetness
    fn surface_wetness(&self) -> Option<i16> {
        self.surface_wetness_callback
            .map(|callback| (callback)(self.context))
    }

    /// Soil Wetness
    fn soil_wetness(&self) -> Option<i16> {
        self.soil_wetness_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model307StatefulAdapter {
    ambient_temperature: i16,
    relative_humidity: i16,
    barometric_pressure: i16,
    wind_speed: i16,
    wind_direction: i16,
    rainfall: i16,
    snow_depth: i16,
    precipitation_type: i16,
    electric_field: i16,
    surface_wetness: i16,
    soil_wetness: i16,
}

impl ModelAdapter for Model307StatefulAdapter {
    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        Some(self.ambient_temperature)
    }

    /// Relative Humidity
    fn relative_humidity(&self) -> Option<i16> {
        Some(self.relative_humidity)
    }

    /// Barometric Pressure
    fn barometric_pressure(&self) -> Option<i16> {
        Some(self.barometric_pressure)
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<i16> {
        Some(self.wind_speed)
    }

    /// Wind Direction
    fn wind_direction(&self) -> Option<i16> {
        Some(self.wind_direction)
    }

    /// Rainfall
    fn rainfall(&self) -> Option<i16> {
        Some(self.rainfall)
    }

    /// Snow Depth
    fn snow_depth(&self) -> Option<i16> {
        Some(self.snow_depth)
    }

    /// Precipitation Type
    ///
    /// Precipitation Type (WMO 4680 SYNOP code reference)
    fn precipitation_type(&self) -> Option<i16> {
        Some(self.precipitation_type)
    }

    /// Electric Field
    fn electric_field(&self) -> Option<i16> {
        Some(self.electric_field)
    }

    /// Surface Wetness
    fn surface_wetness(&self) -> Option<i16> {
        Some(self.surface_wetness)
    }

    /// Soil Wetness
    fn soil_wetness(&self) -> Option<i16> {
        Some(self.soil_wetness)
    }
}
