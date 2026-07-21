use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 68;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 1 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 66 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Manufacturer },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Model },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Options },
        size: 8,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Version },
        size: 8,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::SerialNumber },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::DeviceAddress },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
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
    Manufacturer,
    Model,
    Options,
    Version,
    SerialNumber,
    DeviceAddress,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Manufacturer => serialisation::write_string(model.manufacturer(), buffer, offset, limit),
        Point::Model => serialisation::write_string(model.model(), buffer, offset, limit),
        Point::Options => if let Some(value) = model.options() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Version => if let Some(value) = model.version() { serialisation::write_string(value, buffer, offset, limit); },
        Point::SerialNumber => serialisation::write_string(model.serial_number(), buffer, offset, limit),
        Point::DeviceAddress => if let Some(value) = model.device_address() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    fn manufacturer(&self) -> String<32>;

    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    fn model(&self) -> String<32>;

    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    fn options(&self) -> Option<String<16>> {
        None
    }

    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    fn version(&self) -> Option<String<16>> {
        None
    }

    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    fn serial_number(&self) -> String<32>;

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn device_address(&self) -> Option<u16> {
        None
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn set_device_address(&mut self, value: u16) {
    }
}
