use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 5;

pub static POINTS: [ReadablePoint; 5] = [
    ReadablePoint {
        reference: PointReference::Static { value: 8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model8 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model8 {
            point: Point::Format,
        },
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
    ReadablePoint {
        reference: PointReference::Model8 {
            point: Point::RepeatingCert,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    Format,
    N,
    RepeatingCert,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    5
}

pub fn write_point<'a, 'b>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Format => {
            buffer::write_u16(model.format() as u16, buffer);
        }
        Point::N => {
            buffer::write_u16(model.n(), buffer);
        }
        Point::RepeatingCert => {
            buffer::write_u16(model.repeating_cert(), buffer);
        }
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

    /// Cert
    ///
    /// X.509 Certificate of the device
    fn repeating_cert(&self) -> u16;
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
    repeating_cert_callback: extern "C" fn(*const c_void) -> u16,
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

    /// Cert
    ///
    /// X.509 Certificate of the device
    fn repeating_cert(&self) -> u16 {
        (self.repeating_cert_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model8StatefulAdapter {
    format: Fmt,
    n: u16,
    repeating_cert: u16,
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

    /// Cert
    ///
    /// X.509 Certificate of the device
    fn repeating_cert(&self) -> u16 {
        self.repeating_cert
    }
}
