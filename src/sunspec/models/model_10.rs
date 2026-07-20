use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 6;

pub static POINTS: [ReadablePoint; 6] = [
    ReadablePoint {
        reference: PointReference::Static { value: 10 },
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
        reference: PointReference::Model10 { point: Point::InterfaceStatus },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model10 { point: Point::InterfaceControl },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model10 { point: Point::PhysicalAccessType },
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
];

#[derive(Debug)]
pub enum Point {
    InterfaceStatus,
    InterfaceControl,
    PhysicalAccessType,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::InterfaceStatus => serialisation::write_u16(model.interface_status() as u16, buffer),
        Point::InterfaceControl => if let Some(value) = model.interface_control() { serialisation::write_u16(value, buffer); },
        Point::PhysicalAccessType => if let Some(value) = model.physical_access_type() { serialisation::write_u16(value as u16, buffer); },
    }
}

pub trait ModelAdapter {
    /// Interface Status
    ///
    /// Overall interface status
    fn interface_status(&self) -> St;

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn interface_control(&self) -> Option<u16> {
        None
    }

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn set_interface_control(&mut self, value: u16) {
    }

    /// Physical Access Type
    ///
    /// Type of physical media
    fn physical_access_type(&self) -> Option<Typ> {
        None
    }
}

pub enum St {
    Down = 0,
    Up = 1,
    Fault = 2,
}

pub enum Typ {
    Unknown = 0,
    Internal = 1,
    TwistedPair = 2,
    Fiber = 3,
    Wireless = 4,
}