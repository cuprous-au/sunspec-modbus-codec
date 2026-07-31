use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

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
            point: Point::SetOperation,
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
    SetOperation,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ControlMode => {
            if let Some(value) = model.control_mode() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::DerHeartbeat => {
            if let Some(value) = model.der_heartbeat() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ControllerHeartbeat => {
            if let Some(value) = model.controller_heartbeat() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::AlarmReset => {
            if let Some(value) = model.alarm_reset() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::SetOperation => {
            if let Some(value) = model.set_operation() {
                serialisation::write_u16(value as u16, buffer);
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
    fn set_operation(&self) -> Option<OpCtl> {
        None
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn set_set_operation(&mut self, value: OpCtl) {}
}

pub enum LocRemCtl {
    /// Remote Control
    Remote = 0,
    /// Local Control
    ///
    /// Local mode is required for manual/maintenance operations. Once invoked, it must be explicitly exited for the inverter to be controlled remotely.
    Local = 1,
}

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
    control_mode_callback: Option<extern "C" fn() -> LocRemCtl>,
    der_heartbeat_callback: Option<extern "C" fn() -> u32>,
    controller_heartbeat_callback: Option<extern "C" fn() -> u32>,
    set_controller_heartbeat_callback: Option<extern "C" fn(u32)>,
    alarm_reset_callback: Option<extern "C" fn() -> u16>,
    set_alarm_reset_callback: Option<extern "C" fn(u16)>,
    set_operation_callback: Option<extern "C" fn() -> OpCtl>,
    set_set_operation_callback: Option<extern "C" fn(OpCtl)>,
}

impl ModelAdapter for Model715CallbackAdapter {
    /// Control Mode
    ///
    /// DER control mode. Enumeration.
    fn control_mode(&self) -> Option<LocRemCtl> {
        self.control_mode_callback.map(|callback| (callback)())
    }

    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    fn der_heartbeat(&self) -> Option<u32> {
        self.der_heartbeat_callback.map(|callback| (callback)())
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn controller_heartbeat(&self) -> Option<u32> {
        self.controller_heartbeat_callback
            .map(|callback| (callback)())
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn set_controller_heartbeat(&mut self, value: u32) {
        if let Some(callback) = self.set_controller_heartbeat_callback {
            (callback)(value);
        };
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn alarm_reset(&self) -> Option<u16> {
        self.alarm_reset_callback.map(|callback| (callback)())
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn set_alarm_reset(&mut self, value: u16) {
        if let Some(callback) = self.set_alarm_reset_callback {
            (callback)(value);
        };
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn set_operation(&self) -> Option<OpCtl> {
        self.set_operation_callback.map(|callback| (callback)())
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn set_set_operation(&mut self, value: OpCtl) {
        if let Some(callback) = self.set_set_operation_callback {
            (callback)(value);
        };
    }
}
