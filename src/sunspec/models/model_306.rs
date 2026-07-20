use crate::serialisation;
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Ghi => if let Some(value) = model.ghi() { serialisation::write_u16(value, buffer); },
        Point::Amps => if let Some(value) = model.amps() { serialisation::write_u16(value, buffer); },
        Point::Voltage => if let Some(value) = model.voltage() { serialisation::write_u16(value, buffer); },
        Point::Temperature => if let Some(value) = model.temperature() { serialisation::write_u16(value, buffer); },
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