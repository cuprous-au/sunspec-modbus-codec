use core::ffi::c_void;
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
        Point::Format => {
            serialisation::write_u16(model.format() as u16, buffer);
        },
        Point::N => {
            serialisation::write_u16(model.n(), buffer);
        },
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

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Fmt {
    None = 0,
    X509Pem = 1,
    X509Der = 2,
}

#[repr(C)]
pub struct Model8CallbackAdapter {
    context: *mut c_void,
    format_callback: extern "C" fn(*const c_void) -> Fmt,
    n_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model8CallbackAdapter {
    /// Format
    ///
    /// X.509 format of the certificate. DER or PEM.
    fn format(&self) -> Fmt {
        (self.format_callback)(self.context)
    }

    /// N
    ///
    /// Number of registers to follow for the certificate
    fn n(&self) -> u16 {
        (self.n_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model8StatefulAdapter {
    format: Fmt,
    n: u16,
}

impl ModelAdapter for Model8StatefulAdapter {
    /// Format
    ///
    /// X.509 format of the certificate. DER or PEM.
    fn format(&self) -> Fmt {
        self.format
    }

    /// N
    ///
    /// Number of registers to follow for the certificate
    fn n(&self) -> u16 {
        self.n
    }
}