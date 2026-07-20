use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 3;

pub static POINTS: [ReadablePoint; 3] = [
    ReadablePoint {
        reference: PointReference::Static { value: 806 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 1 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model806 { point: Point::BatteryPointsToBeDetermined },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    BatteryPointsToBeDetermined,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::BatteryPointsToBeDetermined => serialisation::write_u16(model.battery_points_to_be_determined(), buffer),
    }
}

pub trait ModelAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16;
}