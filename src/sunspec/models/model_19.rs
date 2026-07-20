use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 32;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 19 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 30 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Rate },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Bits },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Parity },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Duplex },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::FlowControl },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Authentication },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Username },
        size: 12,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Password },
        size: 6,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Name,
    Rate,
    Bits,
    Parity,
    Duplex,
    FlowControl,
    Authentication,
    Username,
    Password,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Name => if let Some(value) = model.name() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Rate => serialisation::write_u32(model.rate(), buffer, offset, limit),
        Point::Bits => serialisation::write_u16(model.bits(), buffer),
        Point::Parity => serialisation::write_u16(model.parity() as u16, buffer),
        Point::Duplex => if let Some(value) = model.duplex() { serialisation::write_u16(value as u16, buffer); },
        Point::FlowControl => if let Some(value) = model.flow_control() { serialisation::write_u16(value as u16, buffer); },
        Point::Authentication => if let Some(value) = model.authentication() { serialisation::write_u16(value as u16, buffer); },
        Point::Username => if let Some(value) = model.username() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Password => if let Some(value) = model.password() { serialisation::write_string(value, buffer, offset, limit); },
    }
}

pub trait ModelAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<String<8>> {
        None
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: String<8>) {
    }

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn rate(&self) -> u32;

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn set_rate(&mut self, value: u32);

    /// Bits
    ///
    /// Number of data bits per character
    fn bits(&self) -> u16;

    /// Bits
    ///
    /// Number of data bits per character
    fn set_bits(&mut self, value: u16);

    /// Parity
    ///
    /// Parity setting
    fn parity(&self) -> Pty;

    /// Parity
    ///
    /// Parity setting
    fn set_parity(&mut self, value: Pty);

    /// Duplex
    ///
    /// Duplex mode
    fn duplex(&self) -> Option<Dup> {
        None
    }

    /// Duplex
    ///
    /// Duplex mode
    fn set_duplex(&mut self, value: Dup) {
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn flow_control(&self) -> Option<Flw> {
        None
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn set_flow_control(&mut self, value: Flw) {
    }

    /// Authentication
    ///
    /// Authentication method
    fn authentication(&self) -> Option<Auth> {
        None
    }

    /// Username
    ///
    /// Username for authentication
    fn username(&self) -> Option<String<24>> {
        None
    }

    /// Password
    ///
    /// Password for authentication
    fn password(&self) -> Option<String<12>> {
        None
    }
}

pub enum Pty {
    None = 0,
    Odd = 1,
    Even = 2,
}

pub enum Dup {
    Full = 0,
    Half = 1,
}

pub enum Flw {
    None = 0,
    Hw = 1,
    Xonxoff = 2,
}

pub enum Auth {
    None = 0,
    Pap = 1,
    Chap = 2,
}