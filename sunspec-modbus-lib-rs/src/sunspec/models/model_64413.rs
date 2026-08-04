use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 5;

pub static POINTS: [ReadablePoint; 5] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64413 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 3 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64413 { point: Point::IvLength },
        size: 1,
        data_type: PointType::Count,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64413 { point: Point::PoaIrradiance },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64413 { point: Point::IrrSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    IvLength,
    PoaIrradiance,
    IrrSf,
}

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::IvLength => {
            if let Some(value) = model.iv_length() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PoaIrradiance => {
            if let Some(value) = model.poa_irradiance() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IrrSf => {
            if let Some(value) = model.irr_sf() {
                buffer::write_u16(value, buffer);
            }
            else {
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
}

#[repr(C)]
pub struct Model64413CallbackAdapter {
    context: *mut c_void,
    iv_length_callback: Option<extern "C" fn(*const c_void) -> u16>,
    poa_irradiance_callback: Option<extern "C" fn(*const c_void) -> u16>,
    irr_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model64413CallbackAdapter {
    /// IV length
    ///
    /// Number of points in the IV curve.
    fn iv_length(&self) -> Option<u16> {
        self.iv_length_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    fn poa_irradiance(&self) -> Option<u16> {
        self.poa_irradiance_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    fn irr_sf(&self) -> Option<u16> {
        self.irr_sf_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model64413StatefulAdapter {
    iv_length: u16,
    poa_irradiance: u16,
    irr_sf: u16,
}

impl ModelAdapter for Model64413StatefulAdapter {
    /// IV length
    ///
    /// Number of points in the IV curve.
    fn iv_length(&self) -> Option<u16> {
        Some(
        self.iv_length
        )
    }

    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    fn poa_irradiance(&self) -> Option<u16> {
        Some(
        self.poa_irradiance
        )
    }

    fn irr_sf(&self) -> Option<u16> {
        Some(
        self.irr_sf
        )
    }
}