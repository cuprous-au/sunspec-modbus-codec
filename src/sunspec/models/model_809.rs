use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 3;

pub static POINTS: [ReadablePoint; 3] = [
    ReadablePoint {
        reference: PointReference::Static { value: 809 },
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
        reference: PointReference::Model809 {
            point: Point::StackPointsToBeDetermined,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    StackPointsToBeDetermined,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::StackPointsToBeDetermined => {
            serialisation::write_u16(model.stack_points_to_be_determined(), buffer)
        }
    }
}

pub trait ModelAdapter {
    /// Stack Points To Be Determined
    fn stack_points_to_be_determined(&self) -> u16;
}

#[repr(C)]
pub struct Model809CallbackAdapter {
    stack_points_to_be_determined_callback: extern "C" fn() -> u16,
}

impl ModelAdapter for Model809CallbackAdapter {
    /// Stack Points To Be Determined
    fn stack_points_to_be_determined(&self) -> u16 {
        (self.stack_points_to_be_determined_callback)()
    }
}
