use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 13;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 307 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 11 },
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::AmbientTemperature => {
            if let Some(value) = model.ambient_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::RelativeHumidity => {
            if let Some(value) = model.relative_humidity() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::BarometricPressure => {
            if let Some(value) = model.barometric_pressure() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WindSpeed => {
            if let Some(value) = model.wind_speed() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WindDirection => {
            if let Some(value) = model.wind_direction() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Rainfall => {
            if let Some(value) = model.rainfall() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::SnowDepth => {
            if let Some(value) = model.snow_depth() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PrecipitationType => {
            if let Some(value) = model.precipitation_type() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::ElectricField => {
            if let Some(value) = model.electric_field() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::SurfaceWetness => {
            if let Some(value) = model.surface_wetness() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::SoilWetness => {
            if let Some(value) = model.soil_wetness() {
                serialisation::write_i16(value, buffer);
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
    ambient_temperature_callback: Option<extern "C" fn() -> i16>,
    relative_humidity_callback: Option<extern "C" fn() -> i16>,
    barometric_pressure_callback: Option<extern "C" fn() -> i16>,
    wind_speed_callback: Option<extern "C" fn() -> i16>,
    wind_direction_callback: Option<extern "C" fn() -> i16>,
    rainfall_callback: Option<extern "C" fn() -> i16>,
    snow_depth_callback: Option<extern "C" fn() -> i16>,
    precipitation_type_callback: Option<extern "C" fn() -> i16>,
    electric_field_callback: Option<extern "C" fn() -> i16>,
    surface_wetness_callback: Option<extern "C" fn() -> i16>,
    soil_wetness_callback: Option<extern "C" fn() -> i16>,
}

impl ModelAdapter for Model307CallbackAdapter {
    /// Ambient Temperature
    fn ambient_temperature(&self) -> Option<i16> {
        self.ambient_temperature_callback
            .map(|callback| (callback)())
    }

    /// Relative Humidity
    fn relative_humidity(&self) -> Option<i16> {
        self.relative_humidity_callback.map(|callback| (callback)())
    }

    /// Barometric Pressure
    fn barometric_pressure(&self) -> Option<i16> {
        self.barometric_pressure_callback
            .map(|callback| (callback)())
    }

    /// Wind Speed
    fn wind_speed(&self) -> Option<i16> {
        self.wind_speed_callback.map(|callback| (callback)())
    }

    /// Wind Direction
    fn wind_direction(&self) -> Option<i16> {
        self.wind_direction_callback.map(|callback| (callback)())
    }

    /// Rainfall
    fn rainfall(&self) -> Option<i16> {
        self.rainfall_callback.map(|callback| (callback)())
    }

    /// Snow Depth
    fn snow_depth(&self) -> Option<i16> {
        self.snow_depth_callback.map(|callback| (callback)())
    }

    /// Precipitation Type
    ///
    /// Precipitation Type (WMO 4680 SYNOP code reference)
    fn precipitation_type(&self) -> Option<i16> {
        self.precipitation_type_callback
            .map(|callback| (callback)())
    }

    /// Electric Field
    fn electric_field(&self) -> Option<i16> {
        self.electric_field_callback.map(|callback| (callback)())
    }

    /// Surface Wetness
    fn surface_wetness(&self) -> Option<i16> {
        self.surface_wetness_callback.map(|callback| (callback)())
    }

    /// Soil Wetness
    fn soil_wetness(&self) -> Option<i16> {
        self.soil_wetness_callback.map(|callback| (callback)())
    }
}
