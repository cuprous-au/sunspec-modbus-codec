use core::ffi::c_void;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 14;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 712 },
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
        reference: PointReference::Model712 { point: Point::DerWattVarModuleEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::ActiveCurveRequest },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::SetActiveCurveResult },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::NumberOfPoints },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::StoredCurveCount },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::ReversionTimeout },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::ReversionTimeLeft },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::ReversionCurve },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::ActivePowerScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model712 { point: Point::VarScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    DerWattVarModuleEnable,
    ActiveCurveRequest,
    SetActiveCurveResult,
    NumberOfPoints,
    StoredCurveCount,
    ReversionTimeout,
    ReversionTimeLeft,
    ReversionCurve,
    ActivePowerScaleFactor,
    VarScaleFactor,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::DerWattVarModuleEnable => {
            serialisation::write_u16(model.der_watt_var_module_enable() as u16, buffer);
        },
        Point::ActiveCurveRequest => {
            serialisation::write_u16(model.active_curve_request(), buffer);
        },
        Point::SetActiveCurveResult => {
            serialisation::write_u16(model.set_active_curve_result() as u16, buffer);
        },
        Point::NumberOfPoints => {
            serialisation::write_u16(model.number_of_points(), buffer);
        },
        Point::StoredCurveCount => {
            serialisation::write_u16(model.stored_curve_count(), buffer);
        },
        Point::ReversionTimeout => {
            if let Some(value) = model.reversion_timeout() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::ReversionTimeLeft => {
            if let Some(value) = model.reversion_time_left() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::ReversionCurve => {
            if let Some(value) = model.reversion_curve() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::ActivePowerScaleFactor => {
            serialisation::write_u16(model.active_power_scale_factor(), buffer);
        },
        Point::VarScaleFactor => {
            serialisation::write_u16(model.var_scale_factor(), buffer);
        },
    }
}

pub trait ModelAdapter {
    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    fn der_watt_var_module_enable(&self) -> Ena;

    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    fn set_der_watt_var_module_enable(&mut self, value: Ena);

    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    fn active_curve_request(&self) -> u16;

    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    fn set_active_curve_request(&mut self, value: u16);

    /// Set Active Curve Result
    ///
    /// Result of last set active curve operation.
    fn set_active_curve_result(&self) -> AdptCrvRslt;

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
    fn set_reversion_timeout(&mut self, value: u32) {
    }

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
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
    fn set_reversion_curve(&mut self, value: u16) {
    }

    /// Active Power Scale Factor
    ///
    /// Scale factor for curve active power points.
    fn active_power_scale_factor(&self) -> u16;

    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    fn var_scale_factor(&self) -> u16;
}

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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
pub struct Model712CallbackAdapter {
    context: *mut c_void,
    der_watt_var_module_enable_callback: extern "C" fn(*const c_void) -> Ena,
    set_der_watt_var_module_enable_callback: extern "C" fn(Ena, *mut c_void),
    active_curve_request_callback: extern "C" fn(*const c_void) -> u16,
    set_active_curve_request_callback: extern "C" fn(u16, *mut c_void),
    set_active_curve_result_callback: extern "C" fn(*const c_void) -> AdptCrvRslt,
    number_of_points_callback: extern "C" fn(*const c_void) -> u16,
    stored_curve_count_callback: extern "C" fn(*const c_void) -> u16,
    reversion_timeout_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_reversion_timeout_callback: Option<extern "C" fn(u32, *mut c_void)>,
    reversion_time_left_callback: Option<extern "C" fn(*const c_void) -> u32>,
    reversion_curve_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_reversion_curve_callback: Option<extern "C" fn(u16, *mut c_void)>,
    active_power_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    var_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model712CallbackAdapter {
    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    fn der_watt_var_module_enable(&self) -> Ena {
        (self.der_watt_var_module_enable_callback)(self.context)
    }

    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    fn set_der_watt_var_module_enable(&mut self, value: Ena) {
        (self.set_der_watt_var_module_enable_callback)(value, self.context);
    }

    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    fn active_curve_request(&self) -> u16 {
        (self.active_curve_request_callback)(self.context)
    }

    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    fn set_active_curve_request(&mut self, value: u16) {
        (self.set_active_curve_request_callback)(value, self.context);
    }

    /// Set Active Curve Result
    ///
    /// Result of last set active curve operation.
    fn set_active_curve_result(&self) -> AdptCrvRslt {
        (self.set_active_curve_result_callback)(self.context)
    }

    /// Number Of Points
    ///
    /// Number of curve points supported.
    fn number_of_points(&self) -> u16 {
        (self.number_of_points_callback)(self.context)
    }

    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    fn stored_curve_count(&self) -> u16 {
        (self.stored_curve_count_callback)(self.context)
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn reversion_timeout(&self) -> Option<u32> {
        self.reversion_timeout_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {
        if let Some(callback) = self.set_reversion_timeout_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
        self.reversion_time_left_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn reversion_curve(&self) -> Option<u16> {
        self.reversion_curve_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn set_reversion_curve(&mut self, value: u16) {
        if let Some(callback) = self.set_reversion_curve_callback {
        (callback)(value, self.context);
        };
    }

    /// Active Power Scale Factor
    ///
    /// Scale factor for curve active power points.
    fn active_power_scale_factor(&self) -> u16 {
        (self.active_power_scale_factor_callback)(self.context)
    }

    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    fn var_scale_factor(&self) -> u16 {
        (self.var_scale_factor_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model712StatefulAdapter {
    der_watt_var_module_enable: Ena,
    active_curve_request: u16,
    set_active_curve_result: AdptCrvRslt,
    number_of_points: u16,
    stored_curve_count: u16,
    reversion_timeout: u32,
    reversion_time_left: u32,
    reversion_curve: u16,
    active_power_scale_factor: u16,
    var_scale_factor: u16,
}

impl ModelAdapter for Model712StatefulAdapter {
    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    fn der_watt_var_module_enable(&self) -> Ena {
        self.der_watt_var_module_enable
    }

    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    fn set_der_watt_var_module_enable(&mut self, value: Ena) {
        self.der_watt_var_module_enable = value;
    }

    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    fn active_curve_request(&self) -> u16 {
        self.active_curve_request
    }

    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    fn set_active_curve_request(&mut self, value: u16) {
        self.active_curve_request = value;
    }

    /// Set Active Curve Result
    ///
    /// Result of last set active curve operation.
    fn set_active_curve_result(&self) -> AdptCrvRslt {
        self.set_active_curve_result
    }

    /// Number Of Points
    ///
    /// Number of curve points supported.
    fn number_of_points(&self) -> u16 {
        self.number_of_points
    }

    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    fn stored_curve_count(&self) -> u16 {
        self.stored_curve_count
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn reversion_timeout(&self) -> Option<u32> {
        Some(
        self.reversion_timeout
        )
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {
        self.reversion_timeout = value;
    }

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
        Some(
        self.reversion_time_left
        )
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn reversion_curve(&self) -> Option<u16> {
        Some(
        self.reversion_curve
        )
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn set_reversion_curve(&mut self, value: u16) {
        self.reversion_curve = value;
    }

    /// Active Power Scale Factor
    ///
    /// Scale factor for curve active power points.
    fn active_power_scale_factor(&self) -> u16 {
        self.active_power_scale_factor
    }

    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    fn var_scale_factor(&self) -> u16 {
        self.var_scale_factor
    }
}