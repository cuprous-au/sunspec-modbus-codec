use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 4;

pub static POINTS: [ReadablePoint; 4] = [
    ReadablePoint {
        reference: PointReference::Static { value: 809 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model809 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model809 {
            point: Point::StackPointsToBeDetermined,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model809 {
            point: Point::CellCellPointsToBeDetermined,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    StackPointsToBeDetermined,
    CellCellPointsToBeDetermined,
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
        Point::StackPointsToBeDetermined => {
            buffer::write_u16(model.stack_points_to_be_determined(), buffer);
        }
        Point::CellCellPointsToBeDetermined => {
            buffer::write_u16(model.cell_cell_points_to_be_determined(), buffer);
        }
    }
}

pub trait ModelAdapter {
    /// Stack Points To Be Determined
    fn stack_points_to_be_determined(&self) -> u16;

    /// Cell Points To Be Determined
    fn cell_cell_points_to_be_determined(&self) -> u16;
}

#[repr(C)]
pub struct Model809CallbackAdapter {
    context: *mut c_void,
    stack_points_to_be_determined_callback: extern "C" fn(*const c_void) -> u16,
    cell_cell_points_to_be_determined_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model809CallbackAdapter {
    /// Stack Points To Be Determined
    fn stack_points_to_be_determined(&self) -> u16 {
        (self.stack_points_to_be_determined_callback)(self.context)
    }

    /// Cell Points To Be Determined
    fn cell_cell_points_to_be_determined(&self) -> u16 {
        (self.cell_cell_points_to_be_determined_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model809StatefulAdapter {
    stack_points_to_be_determined: u16,
    cell_cell_points_to_be_determined: u16,
}

impl ModelAdapter for Model809StatefulAdapter {
    /// Stack Points To Be Determined
    fn stack_points_to_be_determined(&self) -> u16 {
        self.stack_points_to_be_determined
    }

    /// Cell Points To Be Determined
    fn cell_cell_points_to_be_determined(&self) -> u16 {
        self.cell_cell_points_to_be_determined
    }
}
