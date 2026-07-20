use crate::serialisation;
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::IvLength => if let Some(value) = model.iv_length() { serialisation::write_u16(value, buffer); },
        Point::PoaIrradiance => if let Some(value) = model.poa_irradiance() { serialisation::write_u16(value, buffer); },
        Point::IrrSf => if let Some(value) = model.irr_sf() { serialisation::write_u16(value, buffer); },
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