use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 26;

pub static POINTS: [ReadablePoint; 15] = [
    ReadablePoint {
        reference: PointReference::Static { value: 15 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 24 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::Clear },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::InputCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::InputUnicastCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::InputNonUnicastCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::InputDiscardedCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::InputErrorCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::InputUnknownCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::OutputCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::OutputUnicastCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::OutputNonUnicastCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::OutputDiscardedCount },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 { point: Point::OutputErrorCount },
        size: 2,
        data_type: PointType::Acc32,
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
    Clear,
    InputCount,
    InputUnicastCount,
    InputNonUnicastCount,
    InputDiscardedCount,
    InputErrorCount,
    InputUnknownCount,
    OutputCount,
    OutputUnicastCount,
    OutputNonUnicastCount,
    OutputDiscardedCount,
    OutputErrorCount,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Clear => if let Some(value) = model.clear() { serialisation::write_u16(value, buffer); },
        Point::InputCount => if let Some(value) = model.input_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::InputUnicastCount => if let Some(value) = model.input_unicast_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::InputNonUnicastCount => if let Some(value) = model.input_non_unicast_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::InputDiscardedCount => if let Some(value) = model.input_discarded_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::InputErrorCount => if let Some(value) = model.input_error_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::InputUnknownCount => if let Some(value) = model.input_unknown_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::OutputCount => if let Some(value) = model.output_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::OutputUnicastCount => if let Some(value) = model.output_unicast_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::OutputNonUnicastCount => if let Some(value) = model.output_non_unicast_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::OutputDiscardedCount => if let Some(value) = model.output_discarded_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::OutputErrorCount => if let Some(value) = model.output_error_count() { serialisation::write_u32(value, buffer, offset, limit); },
    }
}

pub trait ModelAdapter {
    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn clear(&self) -> Option<u16> {
        None
    }

    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn set_clear(&mut self, value: u16) {
    }

    /// Input Count
    ///
    /// Number of bytes received
    fn input_count(&self) -> Option<u32> {
        None
    }

    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    fn input_unicast_count(&self) -> Option<u32> {
        None
    }

    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    fn input_non_unicast_count(&self) -> Option<u32> {
        None
    }

    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    fn input_discarded_count(&self) -> Option<u32> {
        None
    }

    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    fn input_error_count(&self) -> Option<u32> {
        None
    }

    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    fn input_unknown_count(&self) -> Option<u32> {
        None
    }

    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    fn output_count(&self) -> Option<u32> {
        None
    }

    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    fn output_unicast_count(&self) -> Option<u32> {
        None
    }

    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    fn output_non_unicast_count(&self) -> Option<u32> {
        None
    }

    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    fn output_discarded_count(&self) -> Option<u32> {
        None
    }

    /// Output Error Count
    ///
    /// Number of outbound error packets
    fn output_error_count(&self) -> Option<u32> {
        None
    }
}