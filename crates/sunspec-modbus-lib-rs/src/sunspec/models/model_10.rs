use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 6;

pub static POINTS: [ReadablePoint; 6] = [
    ReadablePoint {
        reference: PointReference::Static { value: 10 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model10 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model10 {
            point: Point::InterfaceStatus,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model10 {
            point: Point::InterfaceControl,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model10 {
            point: Point::PhysicalAccessType,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    InterfaceStatus,
    InterfaceControl,
    PhysicalAccessType,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    6
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
        Point::InterfaceStatus => {
            buffer::write_u16(model.interface_status() as u16, buffer);
        }
        Point::InterfaceControl => {
            if let Some(value) = model.interface_control() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhysicalAccessType => {
            if let Some(value) = model.physical_access_type() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
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
