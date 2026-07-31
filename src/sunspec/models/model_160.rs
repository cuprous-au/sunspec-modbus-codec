use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 10;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 160 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::CurrentScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::PowerScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::EnergyScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::GlobalEvents,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::NumberOfModules,
        },
        size: 1,
        data_type: PointType::Count,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::TimestampPeriod,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    CurrentScaleFactor,
    VoltageScaleFactor,
    PowerScaleFactor,
    EnergyScaleFactor,
    GlobalEvents,
    NumberOfModules,
    TimestampPeriod,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::CurrentScaleFactor => {
            if let Some(value) = model.current_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageScaleFactor => {
            if let Some(value) = model.voltage_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PowerScaleFactor => {
            if let Some(value) = model.power_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::EnergyScaleFactor => {
            if let Some(value) = model.energy_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::GlobalEvents => {
            if let Some(value) = model.global_events() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::NumberOfModules => {
            if let Some(value) = model.number_of_modules() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TimestampPeriod => {
            if let Some(value) = model.timestamp_period() {
                serialisation::write_u16(value, buffer);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Current Scale Factor
    fn current_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Voltage Scale Factor
    fn voltage_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Power Scale Factor
    fn power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Energy Scale Factor
    fn energy_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Global Events
    fn global_events(&self) -> Option<u32> {
        None
    }

    /// Number of Modules
    fn number_of_modules(&self) -> Option<u16> {
        None
    }

    /// Timestamp Period
    fn timestamp_period(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model160CallbackAdapter {
    current_scale_factor_callback: Option<extern "C" fn() -> u16>,
    voltage_scale_factor_callback: Option<extern "C" fn() -> u16>,
    power_scale_factor_callback: Option<extern "C" fn() -> u16>,
    energy_scale_factor_callback: Option<extern "C" fn() -> u16>,
    global_events_callback: Option<extern "C" fn() -> u32>,
    number_of_modules_callback: Option<extern "C" fn() -> u16>,
    timestamp_period_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model160CallbackAdapter {
    /// Current Scale Factor
    fn current_scale_factor(&self) -> Option<u16> {
        self.current_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Voltage Scale Factor
    fn voltage_scale_factor(&self) -> Option<u16> {
        self.voltage_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Power Scale Factor
    fn power_scale_factor(&self) -> Option<u16> {
        self.power_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Energy Scale Factor
    fn energy_scale_factor(&self) -> Option<u16> {
        self.energy_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Global Events
    fn global_events(&self) -> Option<u32> {
        self.global_events_callback.map(|callback| (callback)())
    }

    /// Number of Modules
    fn number_of_modules(&self) -> Option<u16> {
        self.number_of_modules_callback.map(|callback| (callback)())
    }

    /// Timestamp Period
    fn timestamp_period(&self) -> Option<u16> {
        self.timestamp_period_callback.map(|callback| (callback)())
    }
}
