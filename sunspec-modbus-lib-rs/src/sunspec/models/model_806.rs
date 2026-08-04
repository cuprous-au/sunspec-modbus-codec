use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
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

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::BatteryPointsToBeDetermined => {
            buffer::write_u16(model.battery_points_to_be_determined(), buffer);
        },
    }
}

pub trait ModelAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16;
}

#[repr(C)]
pub struct Model806CallbackAdapter {
    context: *mut c_void,
    battery_points_to_be_determined_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model806CallbackAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16 {
        (self.battery_points_to_be_determined_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model806StatefulAdapter {
    battery_points_to_be_determined: u16,
}

impl ModelAdapter for Model806StatefulAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16 {
        self.battery_points_to_be_determined
    }
}