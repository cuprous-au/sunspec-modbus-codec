use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 9;

pub static POINTS: [ReadablePoint; 7] = [
    ReadablePoint {
        reference: PointReference::Static { value: 715 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 7 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model715 {
            point: Point::ControlMode,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model715 {
            point: Point::DerHeartbeat,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model715 {
            point: Point::ControllerHeartbeat,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model715 {
            point: Point::AlarmReset,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model715 {
            point: Point::Operation,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    ControlMode,
    DerHeartbeat,
    ControllerHeartbeat,
    AlarmReset,
    Operation,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    9
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ControlMode => {
            if let Some(value) = model.control_mode() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DerHeartbeat => {
            if let Some(value) = model.der_heartbeat() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ControllerHeartbeat => {
            if let Some(value) = model.controller_heartbeat() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AlarmReset => {
            if let Some(value) = model.alarm_reset() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Operation => {
            if let Some(value) = model.operation() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Control Mode
    ///
    /// DER control mode. Enumeration.
    fn control_mode(&self) -> Option<LocRemCtl> {
        None
    }

    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    fn der_heartbeat(&self) -> Option<u32> {
        None
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn controller_heartbeat(&self) -> Option<u32> {
        None
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn set_controller_heartbeat(&mut self, value: u32) {}

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn alarm_reset(&self) -> Option<u16> {
        None
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn set_alarm_reset(&mut self, value: u16) {}

    /// Set Operation
    ///
    /// Commands to PCS.
    fn operation(&self) -> Option<OpCtl> {
        None
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn set_operation(&mut self, value: OpCtl) {}
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum LocRemCtl {
    /// Remote Control
    Remote = 0,
    /// Local Control
    ///
    /// Local mode is required for manual/maintenance operations. Once invoked, it must be explicitly exited for the inverter to be controlled remotely.
    Local = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum OpCtl {
    /// Stop the DER
    Stop = 0,
    /// Start the DER
    Start = 1,
    /// Enter Standby Mode
    EnterStandby = 2,
    /// Exit Standby Mode
    ExitStandby = 3,
}

#[repr(C)]
pub struct Model715CallbackAdapter {
    context: *mut c_void,
    control_mode_callback: Option<extern "C" fn(*const c_void) -> LocRemCtl>,
    der_heartbeat_callback: Option<extern "C" fn(*const c_void) -> u32>,
    controller_heartbeat_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_controller_heartbeat_callback: Option<extern "C" fn(u32, *mut c_void)>,
    alarm_reset_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_alarm_reset_callback: Option<extern "C" fn(u16, *mut c_void)>,
    operation_callback: Option<extern "C" fn(*const c_void) -> OpCtl>,
    set_operation_callback: Option<extern "C" fn(OpCtl, *mut c_void)>,
}

impl ModelAdapter for Model715CallbackAdapter {
    /// Control Mode
    ///
    /// DER control mode. Enumeration.
    fn control_mode(&self) -> Option<LocRemCtl> {
        self.control_mode_callback
            .map(|callback| (callback)(self.context))
    }

    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    fn der_heartbeat(&self) -> Option<u32> {
        self.der_heartbeat_callback
            .map(|callback| (callback)(self.context))
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn controller_heartbeat(&self) -> Option<u32> {
        self.controller_heartbeat_callback
            .map(|callback| (callback)(self.context))
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn set_controller_heartbeat(&mut self, value: u32) {
        if let Some(callback) = self.set_controller_heartbeat_callback {
            (callback)(value, self.context);
        };
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn alarm_reset(&self) -> Option<u16> {
        self.alarm_reset_callback
            .map(|callback| (callback)(self.context))
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn set_alarm_reset(&mut self, value: u16) {
        if let Some(callback) = self.set_alarm_reset_callback {
            (callback)(value, self.context);
        };
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn operation(&self) -> Option<OpCtl> {
        self.operation_callback
            .map(|callback| (callback)(self.context))
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn set_operation(&mut self, value: OpCtl) {
        if let Some(callback) = self.set_operation_callback {
            (callback)(value, self.context);
        };
    }
}

#[repr(C)]
pub struct Model715StatefulAdapter {
    control_mode: LocRemCtl,
    der_heartbeat: u32,
    controller_heartbeat: u32,
    alarm_reset: u16,
    operation: OpCtl,
}

impl ModelAdapter for Model715StatefulAdapter {
    /// Control Mode
    ///
    /// DER control mode. Enumeration.
    fn control_mode(&self) -> Option<LocRemCtl> {
        Some(self.control_mode)
    }

    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    fn der_heartbeat(&self) -> Option<u32> {
        Some(self.der_heartbeat)
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn controller_heartbeat(&self) -> Option<u32> {
        Some(self.controller_heartbeat)
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn set_controller_heartbeat(&mut self, value: u32) {
        self.controller_heartbeat = value;
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn alarm_reset(&self) -> Option<u16> {
        Some(self.alarm_reset)
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn set_alarm_reset(&mut self, value: u16) {
        self.alarm_reset = value;
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn operation(&self) -> Option<OpCtl> {
        Some(self.operation)
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn set_operation(&mut self, value: OpCtl) {
        self.operation = value;
    }
}
