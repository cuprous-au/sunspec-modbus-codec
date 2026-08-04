use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 6;

pub static POINTS: [ReadablePoint; 6] = [
    ReadablePoint {
        reference: PointReference::Static { value: 306 },
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
        reference: PointReference::Model306 { point: Point::Ghi },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model306 { point: Point::Amps },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model306 { point: Point::Voltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model306 { point: Point::Temperature },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Ghi,
    Amps,
    Voltage,
    Temperature,
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
        Point::Amps => {
            if let Some(value) = model.amps() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Voltage => {
            if let Some(value) = model.voltage() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Temperature => {
            if let Some(value) = model.temperature() {
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

    /// Amps
    ///
    /// Current measurement at reference point
    fn amps(&self) -> Option<u16> {
        None
    }

    /// Voltage
    ///
    /// Voltage measurement at reference point
    fn voltage(&self) -> Option<u16> {
        None
    }

    /// Temperature
    ///
    /// Temperature measurement at reference point
    fn temperature(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model306CallbackAdapter {
    context: *mut c_void,
    ghi_callback: Option<extern "C" fn(*const c_void) -> u16>,
    amps_callback: Option<extern "C" fn(*const c_void) -> u16>,
    voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    temperature_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model306CallbackAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        self.ghi_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Amps
    ///
    /// Current measurement at reference point
    fn amps(&self) -> Option<u16> {
        self.amps_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Voltage
    ///
    /// Voltage measurement at reference point
    fn voltage(&self) -> Option<u16> {
        self.voltage_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Temperature
    ///
    /// Temperature measurement at reference point
    fn temperature(&self) -> Option<u16> {
        self.temperature_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model306StatefulAdapter {
    ghi: u16,
    amps: u16,
    voltage: u16,
    temperature: u16,
}

impl ModelAdapter for Model306StatefulAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        Some(
        self.ghi
        )
    }

    /// Amps
    ///
    /// Current measurement at reference point
    fn amps(&self) -> Option<u16> {
        Some(
        self.amps
        )
    }

    /// Voltage
    ///
    /// Voltage measurement at reference point
    fn voltage(&self) -> Option<u16> {
        Some(
        self.voltage
        )
    }

    /// Temperature
    ///
    /// Temperature measurement at reference point
    fn temperature(&self) -> Option<u16> {
        Some(
        self.temperature
        )
    }
}