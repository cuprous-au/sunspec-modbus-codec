use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

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
        reference: PointReference::Model801 { point: Point::DeprecatedModel },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    DeprecatedModel,
}

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::DeprecatedModel => {
            buffer::write_u16(model.deprecated_model(), buffer);
        },
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
    context: *mut c_void,
    deprecated_model_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model801CallbackAdapter {
    /// Deprecated Model
    ///
    /// This model has been deprecated.
    fn deprecated_model(&self) -> u16 {
        (self.deprecated_model_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model801StatefulAdapter {
    deprecated_model: u16,
}

impl ModelAdapter for Model801StatefulAdapter {
    /// Deprecated Model
    ///
    /// This model has been deprecated.
    fn deprecated_model(&self) -> u16 {
        self.deprecated_model
    }
}