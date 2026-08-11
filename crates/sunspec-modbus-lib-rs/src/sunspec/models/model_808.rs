use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 4;

static POINTS: [PointDetails<()>; 4] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::ModulePointsToBeDetermined,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::StackStackPointsToBeDetermined,
        size: 1,
        start_address: 3,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    ModulePointsToBeDetermined,
    StackStackPointsToBeDetermined,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    4
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(808, buffer);
        }
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
