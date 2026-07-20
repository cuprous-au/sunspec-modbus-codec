use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 4;

pub static POINTS: [ReadablePoint; 4] = [
    ReadablePoint {
        reference: PointReference::Static { value: 8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 2 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model8 { point: Point::Format },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model8 { point: Point::N },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Format,
    N,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Format => serialisation::write_u16(model.format() as u16, buffer),
        Point::N => serialisation::write_u16(model.n(), buffer),
    }
}

pub trait ModelAdapter {
    /// Format
    ///
    /// X.509 format of the certificate. DER or PEM.
    fn format(&self) -> Fmt;

    /// N
    ///
    /// Number of registers to follow for the certificate
    fn n(&self) -> u16;
}

pub enum Fmt {
    None = 0,
    X509Pem = 1,
    X509Der = 2,
}