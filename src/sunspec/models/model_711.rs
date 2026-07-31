use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 14;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 711 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 12 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::DerFrequencyDroopModuleEnable,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::SetActiveControlRequest,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::SetActiveControlResult,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::StoredControlCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::ReversionTimeout,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::ReversionTimeLeft,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::ReversionControl,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::DeadbandScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::FrequencyChangeScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model711 {
            point: Point::OpenLoopScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    DerFrequencyDroopModuleEnable,
    SetActiveControlRequest,
    SetActiveControlResult,
    StoredControlCount,
    ReversionTimeout,
    ReversionTimeLeft,
    ReversionControl,
    DeadbandScaleFactor,
    FrequencyChangeScaleFactor,
    OpenLoopScaleFactor,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::DerFrequencyDroopModuleEnable => {
            serialisation::write_u16(model.der_frequency_droop_module_enable() as u16, buffer)
        }
        Point::SetActiveControlRequest => {
            serialisation::write_u16(model.set_active_control_request(), buffer)
        }
        Point::SetActiveControlResult => {
            serialisation::write_u16(model.set_active_control_result() as u16, buffer)
        }
        Point::StoredControlCount => serialisation::write_u16(model.stored_control_count(), buffer),
        Point::ReversionTimeout => {
            if let Some(value) = model.reversion_timeout() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ReversionTimeLeft => {
            if let Some(value) = model.reversion_time_left() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ReversionControl => {
            if let Some(value) = model.reversion_control() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DeadbandScaleFactor => {
            serialisation::write_u16(model.deadband_scale_factor(), buffer)
        }
        Point::FrequencyChangeScaleFactor => {
            serialisation::write_u16(model.frequency_change_scale_factor(), buffer)
        }
        Point::OpenLoopScaleFactor => {
            serialisation::write_u16(model.open_loop_scale_factor(), buffer)
        }
    }
}

pub trait ModelAdapter {
    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn der_frequency_droop_module_enable(&self) -> Ena;

    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn set_der_frequency_droop_module_enable(&mut self, value: Ena);

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn set_active_control_request(&self) -> u16;

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn set_set_active_control_request(&mut self, value: u16);

    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    fn set_active_control_result(&self) -> AdptCtlRslt;

    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    fn stored_control_count(&self) -> u16;

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn reversion_timeout(&self) -> Option<u32> {
        None
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {}

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
        None
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn reversion_control(&self) -> Option<u16> {
        None
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn set_reversion_control(&mut self, value: u16) {}

    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    fn deadband_scale_factor(&self) -> u16;

    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    fn frequency_change_scale_factor(&self) -> u16;

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16;
}

pub enum Ena {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum AdptCtlRslt {
    /// Update In Progress
    ///
    /// Control update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Control update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Control update failed.
    Failed = 2,
}

#[repr(C)]
pub struct Model711CallbackAdapter {
    der_frequency_droop_module_enable_callback: extern "C" fn() -> Ena,
    set_der_frequency_droop_module_enable_callback: extern "C" fn(Ena),
    set_active_control_request_callback: extern "C" fn() -> u16,
    set_set_active_control_request_callback: extern "C" fn(u16),
    set_active_control_result_callback: extern "C" fn() -> AdptCtlRslt,
    stored_control_count_callback: extern "C" fn() -> u16,
    reversion_timeout_callback: Option<extern "C" fn() -> u32>,
    set_reversion_timeout_callback: Option<extern "C" fn(u32)>,
    reversion_time_left_callback: Option<extern "C" fn() -> u32>,
    reversion_control_callback: Option<extern "C" fn() -> u16>,
    set_reversion_control_callback: Option<extern "C" fn(u16)>,
    deadband_scale_factor_callback: extern "C" fn() -> u16,
    frequency_change_scale_factor_callback: extern "C" fn() -> u16,
    open_loop_scale_factor_callback: extern "C" fn() -> u16,
}

impl ModelAdapter for Model711CallbackAdapter {
    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn der_frequency_droop_module_enable(&self) -> Ena {
        (self.der_frequency_droop_module_enable_callback)()
    }

    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn set_der_frequency_droop_module_enable(&mut self, value: Ena) {
        (self.set_der_frequency_droop_module_enable_callback)(value);
    }

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn set_active_control_request(&self) -> u16 {
        (self.set_active_control_request_callback)()
    }

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn set_set_active_control_request(&mut self, value: u16) {
        (self.set_set_active_control_request_callback)(value);
    }

    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    fn set_active_control_result(&self) -> AdptCtlRslt {
        (self.set_active_control_result_callback)()
    }

    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    fn stored_control_count(&self) -> u16 {
        (self.stored_control_count_callback)()
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn reversion_timeout(&self) -> Option<u32> {
        self.reversion_timeout_callback.map(|callback| (callback)())
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {
        if let Some(callback) = self.set_reversion_timeout_callback {
            (callback)(value);
        };
    }

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
        self.reversion_time_left_callback
            .map(|callback| (callback)())
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn reversion_control(&self) -> Option<u16> {
        self.reversion_control_callback.map(|callback| (callback)())
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn set_reversion_control(&mut self, value: u16) {
        if let Some(callback) = self.set_reversion_control_callback {
            (callback)(value);
        };
    }

    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    fn deadband_scale_factor(&self) -> u16 {
        (self.deadband_scale_factor_callback)()
    }

    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    fn frequency_change_scale_factor(&self) -> u16 {
        (self.frequency_change_scale_factor_callback)()
    }

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16 {
        (self.open_loop_scale_factor_callback)()
    }
}
