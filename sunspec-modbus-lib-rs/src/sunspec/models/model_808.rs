use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 3;

pub static POINTS: [ReadablePoint; 3] = [
    ReadablePoint {
        reference: PointReference::Static { value: 808 },
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
        reference: PointReference::Model808 { point: Point::ModulePointsToBeDetermined },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModulePointsToBeDetermined,
}

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::ModulePointsToBeDetermined => {
            buffer::write_u16(model.module_points_to_be_determined(), buffer);
        },
    }
}

pub trait ModelAdapter {
    /// Module Points To Be Determined
    fn module_points_to_be_determined(&self) -> u16;
}

#[repr(C)]
pub struct Model808CallbackAdapter {
    context: *mut c_void,
    module_points_to_be_determined_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model808CallbackAdapter {
    /// Module Points To Be Determined
    fn module_points_to_be_determined(&self) -> u16 {
        (self.module_points_to_be_determined_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model808StatefulAdapter {
    module_points_to_be_determined: u16,
}

impl ModelAdapter for Model808StatefulAdapter {
    /// Module Points To Be Determined
    fn module_points_to_be_determined(&self) -> u16 {
        self.module_points_to_be_determined
    }
}