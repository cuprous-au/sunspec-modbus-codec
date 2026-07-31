use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 15;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 705 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 13 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::DerVoltVarModuleEnable,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::AdoptCurveRequest,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::AdoptCurveResult,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::NumberOfPoints,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::StoredCurveCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::ReversionTimeout,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::ReversionTimeRemaining,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::ReversionCurve,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::VarScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model705 {
            point: Point::OpenLoopScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    DerVoltVarModuleEnable,
    AdoptCurveRequest,
    AdoptCurveResult,
    NumberOfPoints,
    StoredCurveCount,
    ReversionTimeout,
    ReversionTimeRemaining,
    ReversionCurve,
    VoltageScaleFactor,
    VarScaleFactor,
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
        Point::DerVoltVarModuleEnable => {
            serialisation::write_u16(model.der_volt_var_module_enable() as u16, buffer)
        }
        Point::AdoptCurveRequest => serialisation::write_u16(model.adopt_curve_request(), buffer),
        Point::AdoptCurveResult => {
            serialisation::write_u16(model.adopt_curve_result() as u16, buffer)
        }
        Point::NumberOfPoints => serialisation::write_u16(model.number_of_points(), buffer),
        Point::StoredCurveCount => serialisation::write_u16(model.stored_curve_count(), buffer),
        Point::ReversionTimeout => {
            if let Some(value) = model.reversion_timeout() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ReversionTimeRemaining => {
            if let Some(value) = model.reversion_time_remaining() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ReversionCurve => {
            if let Some(value) = model.reversion_curve() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageScaleFactor => serialisation::write_u16(model.voltage_scale_factor(), buffer),
        Point::VarScaleFactor => serialisation::write_u16(model.var_scale_factor(), buffer),
        Point::OpenLoopScaleFactor => {
            serialisation::write_u16(model.open_loop_scale_factor(), buffer)
        }
    }
}

pub trait ModelAdapter {
    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
    fn der_volt_var_module_enable(&self) -> Ena;

    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
    fn set_der_volt_var_module_enable(&mut self, value: Ena);

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn adopt_curve_request(&self) -> u16;

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn set_adopt_curve_request(&mut self, value: u16);

    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
    fn adopt_curve_result(&self) -> AdptCrvRslt;

    /// Number Of Points
    ///
    /// Number of curve points supported.
    fn number_of_points(&self) -> u16;

    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    fn stored_curve_count(&self) -> u16;

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

    /// Reversion Time Remaining
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_remaining(&self) -> Option<u32> {
        None
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn reversion_curve(&self) -> Option<u16> {
        None
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn set_reversion_curve(&mut self, value: u16) {}

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn voltage_scale_factor(&self) -> u16;

    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    fn var_scale_factor(&self) -> u16;

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

pub enum AdptCrvRslt {
    /// Update In Progress
    ///
    /// Curve update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Curve update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Curve update failed.
    Failed = 2,
}

#[repr(C)]
pub struct Model705CallbackAdapter {
    der_volt_var_module_enable_callback: extern "C" fn() -> Ena,
    set_der_volt_var_module_enable_callback: extern "C" fn(Ena),
    adopt_curve_request_callback: extern "C" fn() -> u16,
    set_adopt_curve_request_callback: extern "C" fn(u16),
    adopt_curve_result_callback: extern "C" fn() -> AdptCrvRslt,
    number_of_points_callback: extern "C" fn() -> u16,
    stored_curve_count_callback: extern "C" fn() -> u16,
    reversion_timeout_callback: Option<extern "C" fn() -> u32>,
    set_reversion_timeout_callback: Option<extern "C" fn(u32)>,
    reversion_time_remaining_callback: Option<extern "C" fn() -> u32>,
    reversion_curve_callback: Option<extern "C" fn() -> u16>,
    set_reversion_curve_callback: Option<extern "C" fn(u16)>,
    voltage_scale_factor_callback: extern "C" fn() -> u16,
    var_scale_factor_callback: extern "C" fn() -> u16,
    open_loop_scale_factor_callback: extern "C" fn() -> u16,
}

impl ModelAdapter for Model705CallbackAdapter {
    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
    fn der_volt_var_module_enable(&self) -> Ena {
        (self.der_volt_var_module_enable_callback)()
    }

    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
    fn set_der_volt_var_module_enable(&mut self, value: Ena) {
        (self.set_der_volt_var_module_enable_callback)(value);
    }

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn adopt_curve_request(&self) -> u16 {
        (self.adopt_curve_request_callback)()
    }

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn set_adopt_curve_request(&mut self, value: u16) {
        (self.set_adopt_curve_request_callback)(value);
    }

    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
    fn adopt_curve_result(&self) -> AdptCrvRslt {
        (self.adopt_curve_result_callback)()
    }

    /// Number Of Points
    ///
    /// Number of curve points supported.
    fn number_of_points(&self) -> u16 {
        (self.number_of_points_callback)()
    }

    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    fn stored_curve_count(&self) -> u16 {
        (self.stored_curve_count_callback)()
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

    /// Reversion Time Remaining
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_remaining(&self) -> Option<u32> {
        self.reversion_time_remaining_callback
            .map(|callback| (callback)())
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn reversion_curve(&self) -> Option<u16> {
        self.reversion_curve_callback.map(|callback| (callback)())
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn set_reversion_curve(&mut self, value: u16) {
        if let Some(callback) = self.set_reversion_curve_callback {
            (callback)(value);
        };
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        (self.voltage_scale_factor_callback)()
    }

    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    fn var_scale_factor(&self) -> u16 {
        (self.var_scale_factor_callback)()
    }

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16 {
        (self.open_loop_scale_factor_callback)()
    }
}
