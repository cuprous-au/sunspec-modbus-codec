use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 5;

pub static POINTS: [ReadablePoint; 5] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64413 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64413 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64413 {
            point: Point::IvLength,
        },
        size: 1,
        data_type: PointType::Count,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64413 {
            point: Point::PoaIrradiance,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64413 {
            point: Point::IrrSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    IvLength,
    PoaIrradiance,
    IrrSf,
    IvPower { iv_index: u16 },
    IvCurrent { iv_index: u16 },
    IvVoltage { iv_index: u16 },
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    5 + model.iv_length().unwrap_or(0) * (6)
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
        Point::IvLength => {
            if let Some(value) = model.iv_length() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PoaIrradiance => {
            if let Some(value) = model.poa_irradiance() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IrrSf => {
            if let Some(value) = model.irr_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IvPower { iv_index } => {
            if let Some(value) = model.iv_power(*iv_index) {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IvCurrent { iv_index } => {
            if let Some(value) = model.iv_current(*iv_index) {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IvVoltage { iv_index } => {
            if let Some(value) = model.iv_voltage(*iv_index) {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// IV length
    ///
    /// Number of points in the IV curve.
    fn iv_length(&self) -> Option<u16> {
        None
    }

    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    fn poa_irradiance(&self) -> Option<u16> {
        None
    }

    fn irr_sf(&self) -> Option<u16> {
        None
    }

    /// Power
    ///
    /// Power
    fn iv_power(&self, iv_index: u16) -> Option<f32> {
        None
    }

    /// Current
    ///
    /// Current
    fn iv_current(&self, iv_index: u16) -> Option<f32> {
        None
    }

    /// Voltage
    ///
    /// Voltage
    fn iv_voltage(&self, iv_index: u16) -> Option<f32> {
        None
    }
}

#[repr(C)]
pub struct Model64413CallbackAdapter {
    context: *mut c_void,
    iv_length_callback: Option<extern "C" fn(*const c_void) -> u16>,
    poa_irradiance_callback: Option<extern "C" fn(*const c_void) -> u16>,
    irr_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    iv_power_callback: Option<extern "C" fn(*const c_void, u16) -> f32>,
    iv_current_callback: Option<extern "C" fn(*const c_void, u16) -> f32>,
    iv_voltage_callback: Option<extern "C" fn(*const c_void, u16) -> f32>,
}

impl ModelAdapter for Model64413CallbackAdapter {
    /// IV length
    ///
    /// Number of points in the IV curve.
    fn iv_length(&self) -> Option<u16> {
        self.iv_length_callback
            .map(|callback| (callback)(self.context))
    }

    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    fn poa_irradiance(&self) -> Option<u16> {
        self.poa_irradiance_callback
            .map(|callback| (callback)(self.context))
    }

    fn irr_sf(&self) -> Option<u16> {
        self.irr_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power
    ///
    /// Power
    fn iv_power(&self, iv_index: u16) -> Option<f32> {
        self.iv_power_callback
            .map(|callback| (callback)(self.context, iv_index))
    }

    /// Current
    ///
    /// Current
    fn iv_current(&self, iv_index: u16) -> Option<f32> {
        self.iv_current_callback
            .map(|callback| (callback)(self.context, iv_index))
    }

    /// Voltage
    ///
    /// Voltage
    fn iv_voltage(&self, iv_index: u16) -> Option<f32> {
        self.iv_voltage_callback
            .map(|callback| (callback)(self.context, iv_index))
    }
}

#[repr(C)]
pub struct Model64413StatefulAdapter<const IV_LENGTH: usize> {
    iv_length: u16,
    poa_irradiance: u16,
    irr_sf: u16,
    iv: [Model64413Iv; IV_LENGTH],
}

#[repr(C)]
pub struct Model64413Iv {
    iv_power: f32,
    iv_current: f32,
    iv_voltage: f32,
}

impl<const IV_LENGTH: usize> ModelAdapter for Model64413StatefulAdapter<IV_LENGTH> {
    /// IV length
    ///
    /// Number of points in the IV curve.
    fn iv_length(&self) -> Option<u16> {
        Some(self.iv_length)
    }

    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    fn poa_irradiance(&self) -> Option<u16> {
        Some(self.poa_irradiance)
    }

    fn irr_sf(&self) -> Option<u16> {
        Some(self.irr_sf)
    }

    /// Power
    ///
    /// Power
    fn iv_power(&self, iv_index: u16) -> Option<f32> {
        Some(self.iv[iv_index as usize].iv_power)
    }

    /// Current
    ///
    /// Current
    fn iv_current(&self, iv_index: u16) -> Option<f32> {
        Some(self.iv[iv_index as usize].iv_current)
    }

    /// Voltage
    ///
    /// Voltage
    fn iv_voltage(&self, iv_index: u16) -> Option<f32> {
        Some(self.iv[iv_index as usize].iv_voltage)
    }
}
