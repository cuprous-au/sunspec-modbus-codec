use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 3;

pub static POINTS: [ReadablePoint; 3] = [
    ReadablePoint {
        reference: PointReference::Static { value: 801 },
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
        reference: PointReference::Model801 {
            point: Point::DeprecatedModel,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    DeprecatedModel,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::DeprecatedModel => serialisation::write_u16(model.deprecated_model(), buffer),
    }
}

pub trait ModelAdapter {
    /// Deprecated Model
    ///
    /// This model has been deprecated.
    fn deprecated_model(&self) -> u16;
}

#[repr(C)]
pub struct Model801CallbackAdapter {
    deprecated_model_callback: extern "C" fn() -> u16,
}

impl ModelAdapter for Model801CallbackAdapter {
    /// Deprecated Model
    ///
    /// This model has been deprecated.
    fn deprecated_model(&self) -> u16 {
        (self.deprecated_model_callback)()
    }
}
