use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 24;

pub static POINTS: [ReadablePoint; 7] = [
    ReadablePoint {
        reference: PointReference::Static { value: 18 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 22 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Imei },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Apn },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Number },
        size: 6,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model18 { point: Point::Pin },
        size: 6,
        data_type: PointType::String,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    Name,
    Imei,
    Apn,
    Number,
    Pin,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Name => if let Some(value) = model.name() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Imei => if let Some(value) = model.imei() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Apn => if let Some(value) = model.apn() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Number => if let Some(value) = model.number() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Pin => if let Some(value) = model.pin() { serialisation::write_string(value, buffer, offset, limit); },
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

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn imei(&self) -> Option<u32> {
        None
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn set_imei(&mut self, value: u32) {
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn apn(&self) -> Option<String<8>> {
        None
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn set_apn(&mut self, value: String<8>) {
    }

    /// Number
    ///
    /// Phone number for the interface
    fn number(&self) -> Option<String<12>> {
        None
    }

    /// Number
    ///
    /// Phone number for the interface
    fn set_number(&mut self, value: String<12>) {
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn pin(&self) -> Option<String<12>> {
        None
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn set_pin(&mut self, value: String<12>) {
    }
}