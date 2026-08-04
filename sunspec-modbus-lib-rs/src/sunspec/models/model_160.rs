use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

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
        reference: PointReference::Model160 { point: Point::CurrentScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 { point: Point::VoltageScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 { point: Point::PowerScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 { point: Point::EnergyScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 { point: Point::GlobalEvents },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 { point: Point::NumberOfModules },
        size: 1,
        data_type: PointType::Count,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 { point: Point::TimestampPeriod },
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

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::CurrentScaleFactor => {
            if let Some(value) = model.current_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageScaleFactor => {
            if let Some(value) = model.voltage_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerScaleFactor => {
            if let Some(value) = model.power_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnergyScaleFactor => {
            if let Some(value) = model.energy_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::GlobalEvents => {
            if let Some(value) = model.global_events() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NumberOfModules => {
            if let Some(value) = model.number_of_modules() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TimestampPeriod => {
            if let Some(value) = model.timestamp_period() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
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
    context: *mut c_void,
    current_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    voltage_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    energy_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    global_events_callback: Option<extern "C" fn(*const c_void) -> u32>,
    number_of_modules_callback: Option<extern "C" fn(*const c_void) -> u16>,
    timestamp_period_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model160CallbackAdapter {
    /// Current Scale Factor
    fn current_scale_factor(&self) -> Option<u16> {
        self.current_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Voltage Scale Factor
    fn voltage_scale_factor(&self) -> Option<u16> {
        self.voltage_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Power Scale Factor
    fn power_scale_factor(&self) -> Option<u16> {
        self.power_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Energy Scale Factor
    fn energy_scale_factor(&self) -> Option<u16> {
        self.energy_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Global Events
    fn global_events(&self) -> Option<u32> {
        self.global_events_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Number of Modules
    fn number_of_modules(&self) -> Option<u16> {
        self.number_of_modules_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Timestamp Period
    fn timestamp_period(&self) -> Option<u16> {
        self.timestamp_period_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model160StatefulAdapter {
    current_scale_factor: u16,
    voltage_scale_factor: u16,
    power_scale_factor: u16,
    energy_scale_factor: u16,
    global_events: u32,
    number_of_modules: u16,
    timestamp_period: u16,
}

impl ModelAdapter for Model160StatefulAdapter {
    /// Current Scale Factor
    fn current_scale_factor(&self) -> Option<u16> {
        Some(
        self.current_scale_factor
        )
    }

    /// Voltage Scale Factor
    fn voltage_scale_factor(&self) -> Option<u16> {
        Some(
        self.voltage_scale_factor
        )
    }

    /// Power Scale Factor
    fn power_scale_factor(&self) -> Option<u16> {
        Some(
        self.power_scale_factor
        )
    }

    /// Energy Scale Factor
    fn energy_scale_factor(&self) -> Option<u16> {
        Some(
        self.energy_scale_factor
        )
    }

    /// Global Events
    fn global_events(&self) -> Option<u32> {
        Some(
        self.global_events
        )
    }

    /// Number of Modules
    fn number_of_modules(&self) -> Option<u16> {
        Some(
        self.number_of_modules
        )
    }

    /// Timestamp Period
    fn timestamp_period(&self) -> Option<u16> {
        Some(
        self.timestamp_period
        )
    }
}