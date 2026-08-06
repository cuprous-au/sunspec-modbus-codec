use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 4;

pub static POINTS: [ReadablePoint; 4] = [
    ReadablePoint {
        reference: PointReference::Static { value: 806 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model806 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model806 {
            point: Point::BatteryPointsToBeDetermined,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model806 {
            point: Point::BatteryStringBatteryStringPointsToBeDetermined,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    BatteryPointsToBeDetermined,
    BatteryStringBatteryStringPointsToBeDetermined,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    4
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::BatteryPointsToBeDetermined => {
            buffer::write_u16(model.battery_points_to_be_determined(), buffer);
        }
        Point::BatteryStringBatteryStringPointsToBeDetermined => {
            buffer::write_u16(
                model.battery_string_battery_string_points_to_be_determined(),
                buffer,
            );
        }
    }
}

pub trait ModelAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16;

    /// Battery String Points To Be Determined
    fn battery_string_battery_string_points_to_be_determined(&self) -> u16;
}

#[repr(C)]
pub struct Model806CallbackAdapter {
    context: *mut c_void,
    battery_points_to_be_determined_callback: extern "C" fn(*const c_void) -> u16,
    battery_string_battery_string_points_to_be_determined_callback:
        extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model806CallbackAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16 {
        (self.battery_points_to_be_determined_callback)(self.context)
    }

    /// Battery String Points To Be Determined
    fn battery_string_battery_string_points_to_be_determined(&self) -> u16 {
        (self.battery_string_battery_string_points_to_be_determined_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model806StatefulAdapter {
    battery_points_to_be_determined: u16,
    battery_string_battery_string_points_to_be_determined: u16,
}

impl ModelAdapter for Model806StatefulAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16 {
        self.battery_points_to_be_determined
    }

    /// Battery String Points To Be Determined
    fn battery_string_battery_string_points_to_be_determined(&self) -> u16 {
        self.battery_string_battery_string_points_to_be_determined
    }
}
