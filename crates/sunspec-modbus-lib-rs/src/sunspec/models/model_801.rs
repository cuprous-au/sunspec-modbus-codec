use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 3;

static POINTS: [PointDetails<()>; 3] = [
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
        point: |()| Point::DeprecatedModel,
        size: 1,
        start_address: 2,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    DeprecatedModel,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    3
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
            buffer::write_u16(801, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::DeprecatedModel => {
            buffer::write_u16(model.deprecated_model(), buffer);
        }
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
