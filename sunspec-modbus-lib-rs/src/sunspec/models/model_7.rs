use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 12;

pub static POINTS: [ReadablePoint; 11] = [
    ReadablePoint {
        reference: PointReference::Static { value: 7 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 10 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model7 { point: Point::RequestSequence },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model7 { point: Point::Status },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model7 { point: Point::Timestamp },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model7 { point: Point::Milliseconds },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model7 { point: Point::Sequence },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model7 { point: Point::Alarm },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model7 { point: Point::Algorithm },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model7 { point: Point::N },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    RequestSequence,
    Status,
    Timestamp,
    Milliseconds,
    Sequence,
    Alarm,
    Algorithm,
    N,
}

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::RequestSequence => {
            buffer::write_u16(model.request_sequence(), buffer);
        },
        Point::Status => {
            buffer::write_u16(model.status() as u16, buffer);
        },
        Point::Timestamp => {
            buffer::write_u32(model.timestamp(), buffer, offset, limit);
        },
        Point::Milliseconds => {
            buffer::write_u16(model.milliseconds(), buffer);
        },
        Point::Sequence => {
            buffer::write_u16(model.sequence(), buffer);
        },
        Point::Alarm => {
            buffer::write_u16(model.alarm() as u16, buffer);
        },
        Point::Algorithm => {
            buffer::write_u16(model.algorithm() as u16, buffer);
        },
        Point::N => {
            buffer::write_u16(model.n(), buffer);
        },
    }
}

pub trait ModelAdapter {
    /// Request Sequence
    ///
    /// Sequence number from the request
    fn request_sequence(&self) -> u16;

    /// Status
    ///
    /// Status of last write operation
    fn status(&self) -> Sts;

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32;

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16;

    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Shall be advanced for each response
    fn sequence(&self) -> u16;

    /// Alarm
    ///
    /// Bitmask alarm code
    fn alarm(&self) -> Alm;

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg;

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16;

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn set_n(&mut self, value: u16);
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Sts {
    Success = 0,
    /// The signature was not valid
    Ds = 1,
    /// One or more registers were not writable by this role
    Acl = 2,
    /// Offset out of range or missing from multi-register value
    Off = 3,
    /// Value is out of acceptable range
    Val = 4,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Alm {
    None = 0,
    /// Tampered
    Alm = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}

#[repr(C)]
pub struct Model7CallbackAdapter {
    context: *mut c_void,
    request_sequence_callback: extern "C" fn(*const c_void) -> u16,
    status_callback: extern "C" fn(*const c_void) -> Sts,
    timestamp_callback: extern "C" fn(*const c_void) -> u32,
    milliseconds_callback: extern "C" fn(*const c_void) -> u16,
    sequence_callback: extern "C" fn(*const c_void) -> u16,
    alarm_callback: extern "C" fn(*const c_void) -> Alm,
    algorithm_callback: extern "C" fn(*const c_void) -> Alg,
    n_callback: extern "C" fn(*const c_void) -> u16,
    set_n_callback: extern "C" fn(u16, *mut c_void),
}

impl ModelAdapter for Model7CallbackAdapter {
    /// Request Sequence
    ///
    /// Sequence number from the request
    fn request_sequence(&self) -> u16 {
        (self.request_sequence_callback)(self.context)
    }

    /// Status
    ///
    /// Status of last write operation
    fn status(&self) -> Sts {
        (self.status_callback)(self.context)
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        (self.timestamp_callback)(self.context)
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        (self.milliseconds_callback)(self.context)
    }

    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Shall be advanced for each response
    fn sequence(&self) -> u16 {
        (self.sequence_callback)(self.context)
    }

    /// Alarm
    ///
    /// Bitmask alarm code
    fn alarm(&self) -> Alm {
        (self.alarm_callback)(self.context)
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg {
        (self.algorithm_callback)(self.context)
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        (self.n_callback)(self.context)
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn set_n(&mut self, value: u16) {
        (self.set_n_callback)(value, self.context);
    }
}

#[repr(C)]
pub struct Model7StatefulAdapter {
    request_sequence: u16,
    status: Sts,
    timestamp: u32,
    milliseconds: u16,
    sequence: u16,
    alarm: Alm,
    algorithm: Alg,
    n: u16,
}

impl ModelAdapter for Model7StatefulAdapter {
    /// Request Sequence
    ///
    /// Sequence number from the request
    fn request_sequence(&self) -> u16 {
        self.request_sequence
    }

    /// Status
    ///
    /// Status of last write operation
    fn status(&self) -> Sts {
        self.status
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Shall be advanced for each response
    fn sequence(&self) -> u16 {
        self.sequence
    }

    /// Alarm
    ///
    /// Bitmask alarm code
    fn alarm(&self) -> Alm {
        self.alarm
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg {
        self.algorithm
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        self.n
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn set_n(&mut self, value: u16) {
        self.n = value;
    }
}