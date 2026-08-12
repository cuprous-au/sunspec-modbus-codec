use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 6;

static POINTS: [PointDetails<()>; 6] = [
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
        point: |()| Point::InterfaceStatus,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::InterfaceControl,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::PhysicalAccessType,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::Pad,
        size: 1,
        start_address: 5,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    InterfaceStatus,
    InterfaceControl,
    PhysicalAccessType,
    Pad,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    6
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
            buffer::write_u16(10, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::InterfaceStatus => {
            buffer::write_u16(model.interface_status() as u16, buffer);
        }
        Point::InterfaceControl => {
            if let Some(value) = model.interface_control() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::PhysicalAccessType => {
            if let Some(value) = model.physical_access_type() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Pad => {
            buffer::write_u16(0, buffer);
        }
    }
}

pub trait ModelAdapter {
    /// Interface Status
    ///
    /// Overall interface status
    fn interface_status(&self) -> St;

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn interface_control(&self) -> Option<u16> {
        None
    }

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn set_interface_control(&mut self, value: u16) {}

    /// Physical Access Type
    ///
    /// Type of physical media
    fn physical_access_type(&self) -> Option<Typ> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum St {
    Down = 0,
    Up = 1,
    Fault = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Typ {
    Unknown = 0,
    Internal = 1,
    TwistedPair = 2,
    Fiber = 3,
    Wireless = 4,
}

#[repr(C)]
pub struct Model10CallbackAdapter {
    context: *mut c_void,
    interface_status_callback: extern "C" fn(*const c_void) -> St,
    interface_control_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_interface_control_callback: Option<extern "C" fn(u16, *mut c_void)>,
    physical_access_type_callback: Option<extern "C" fn(*const c_void) -> Typ>,
}

impl ModelAdapter for Model10CallbackAdapter {
    /// Interface Status
    ///
    /// Overall interface status
    fn interface_status(&self) -> St {
        (self.interface_status_callback)(self.context)
    }

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn interface_control(&self) -> Option<u16> {
        self.interface_control_callback
            .map(|callback| (callback)(self.context))
    }

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn set_interface_control(&mut self, value: u16) {
        if let Some(callback) = self.set_interface_control_callback {
            (callback)(value, self.context);
        };
    }

    /// Physical Access Type
    ///
    /// Type of physical media
    fn physical_access_type(&self) -> Option<Typ> {
        self.physical_access_type_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model10StatefulAdapter {
    interface_status: St,
    interface_control: u16,
    physical_access_type: Typ,
}

impl ModelAdapter for Model10StatefulAdapter {
    /// Interface Status
    ///
    /// Overall interface status
    fn interface_status(&self) -> St {
        self.interface_status
    }

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn interface_control(&self) -> Option<u16> {
        Some(self.interface_control)
    }

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn set_interface_control(&mut self, value: u16) {
        self.interface_control = value;
    }

    /// Physical Access Type
    ///
    /// Type of physical media
    fn physical_access_type(&self) -> Option<Typ> {
        Some(self.physical_access_type)
    }
}
