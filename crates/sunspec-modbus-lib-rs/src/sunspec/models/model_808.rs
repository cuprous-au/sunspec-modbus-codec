use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 4;

pub static POINTS: [ReadablePoint; 4] = [
    ReadablePoint {
        reference: PointReference::Static { value: 808 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model808 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model808 {
            point: Point::ModulePointsToBeDetermined,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model808 {
            point: Point::StackStackPointsToBeDetermined,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    ModulePointsToBeDetermined,
    StackStackPointsToBeDetermined,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    4
}

pub fn write_point<'a, 'b>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::ModulePointsToBeDetermined => {
            buffer::write_u16(model.module_points_to_be_determined(), buffer);
        }
        Point::StackStackPointsToBeDetermined => {
            buffer::write_u16(model.stack_stack_points_to_be_determined(), buffer);
        }
    }
}

pub trait ModelAdapter {
    /// Module Points To Be Determined
    fn module_points_to_be_determined(&self) -> u16;

    /// Stack Points To Be Determined
    fn stack_stack_points_to_be_determined(&self) -> u16;
}

#[repr(C)]
pub struct Model808CallbackAdapter {
    context: *mut c_void,
    module_points_to_be_determined_callback: extern "C" fn(*const c_void) -> u16,
    stack_stack_points_to_be_determined_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model808CallbackAdapter {
    /// Module Points To Be Determined
    fn module_points_to_be_determined(&self) -> u16 {
        (self.module_points_to_be_determined_callback)(self.context)
    }

    /// Stack Points To Be Determined
    fn stack_stack_points_to_be_determined(&self) -> u16 {
        (self.stack_stack_points_to_be_determined_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model808StatefulAdapter {
    module_points_to_be_determined: u16,
    stack_stack_points_to_be_determined: u16,
}

impl ModelAdapter for Model808StatefulAdapter {
    /// Module Points To Be Determined
    fn module_points_to_be_determined(&self) -> u16 {
        self.module_points_to_be_determined
    }

    /// Stack Points To Be Determined
    fn stack_stack_points_to_be_determined(&self) -> u16 {
        self.stack_stack_points_to_be_determined
    }
}
