use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 5;

static POINTS: [PointDetails<()>; 5] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::Format,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::N,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::RepeatingCert,
        size: 1,
        start_address: 4,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    Format,
    N,
    RepeatingCert,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    5
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(8, buffer);
        }
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
