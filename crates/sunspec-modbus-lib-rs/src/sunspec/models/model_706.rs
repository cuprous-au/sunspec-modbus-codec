use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 15;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 706 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::DerVoltWattModuleEnable,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::AdoptCurveRequest,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::AdoptCurveResult,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::NumberOfPoints,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::StoredCurveCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::ReversionTimeout,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::ReversionTimeRemaining,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::ReversionCurve,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::WattScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model706 {
            point: Point::OpenLoopScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    DerVoltWattModuleEnable,
    AdoptCurveRequest,
    AdoptCurveResult,
    NumberOfPoints,
    StoredCurveCount,
    ReversionTimeout,
    ReversionTimeRemaining,
    ReversionCurve,
    VoltageScaleFactor,
    WattScaleFactor,
    OpenLoopScaleFactor,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    15 + model.stored_curve_count() * (5 + model.number_of_points() * (2))
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
        Point::DerVoltWattModuleEnable => {
            buffer::write_u16(model.der_volt_watt_module_enable() as u16, buffer);
        }
        Point::AdoptCurveRequest => {
            buffer::write_u16(model.adopt_curve_request(), buffer);
        }
        Point::AdoptCurveResult => {
            buffer::write_u16(model.adopt_curve_result() as u16, buffer);
        }
        Point::NumberOfPoints => {
            buffer::write_u16(model.number_of_points(), buffer);
        }
        Point::StoredCurveCount => {
            buffer::write_u16(model.stored_curve_count(), buffer);
        }
        Point::ReversionTimeout => {
            if let Some(value) = model.reversion_timeout() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionTimeRemaining => {
            if let Some(value) = model.reversion_time_remaining() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionCurve => {
            if let Some(value) = model.reversion_curve() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageScaleFactor => {
            buffer::write_u16(model.voltage_scale_factor(), buffer);
        }
        Point::WattScaleFactor => {
            buffer::write_u16(model.watt_scale_factor(), buffer);
        }
        Point::OpenLoopScaleFactor => {
            buffer::write_u16(model.open_loop_scale_factor(), buffer);
        }
    }
}

pub trait ModelAdapter {
    /// DER Volt-Watt Module Enable
    ///
    /// Volt-Watt control enable.
    fn der_volt_watt_module_enable(&self) -> Ena;

    /// DER Volt-Watt Module Enable
    ///
    /// Volt-Watt control enable.
    fn set_der_volt_watt_module_enable(&mut self, value: Ena);

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

    /// Watt Scale Factor
    ///
    /// Scale factor for curve watt points.
    fn watt_scale_factor(&self) -> u16;

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16;
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

#[repr(C)]
pub struct Model706CallbackAdapter {
    context: *mut c_void,
    der_volt_watt_module_enable_callback: extern "C" fn(*const c_void) -> Ena,
    set_der_volt_watt_module_enable_callback: extern "C" fn(Ena, *mut c_void),
    adopt_curve_request_callback: extern "C" fn(*const c_void) -> u16,
    set_adopt_curve_request_callback: extern "C" fn(u16, *mut c_void),
    adopt_curve_result_callback: extern "C" fn(*const c_void) -> AdptCrvRslt,
    number_of_points_callback: extern "C" fn(*const c_void) -> u16,
    stored_curve_count_callback: extern "C" fn(*const c_void) -> u16,
    reversion_timeout_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_reversion_timeout_callback: Option<extern "C" fn(u32, *mut c_void)>,
    reversion_time_remaining_callback: Option<extern "C" fn(*const c_void) -> u32>,
    reversion_curve_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_reversion_curve_callback: Option<extern "C" fn(u16, *mut c_void)>,
    voltage_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    watt_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    open_loop_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model706CallbackAdapter {
    /// DER Volt-Watt Module Enable
    ///
    /// Volt-Watt control enable.
    fn der_volt_watt_module_enable(&self) -> Ena {
        (self.der_volt_watt_module_enable_callback)(self.context)
    }

    /// DER Volt-Watt Module Enable
    ///
    /// Volt-Watt control enable.
    fn set_der_volt_watt_module_enable(&mut self, value: Ena) {
        (self.set_der_volt_watt_module_enable_callback)(value, self.context);
    }

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn adopt_curve_request(&self) -> u16 {
        (self.adopt_curve_request_callback)(self.context)
    }

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn set_adopt_curve_request(&mut self, value: u16) {
        (self.set_adopt_curve_request_callback)(value, self.context);
    }

    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
    fn adopt_curve_result(&self) -> AdptCrvRslt {
        (self.adopt_curve_result_callback)(self.context)
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
        self.reversion_timeout_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {
        if let Some(callback) = self.set_reversion_timeout_callback {
            (callback)(value, self.context);
        };
    }

    /// Reversion Time Remaining
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_remaining(&self) -> Option<u32> {
        self.reversion_time_remaining_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn reversion_curve(&self) -> Option<u16> {
        self.reversion_curve_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn set_reversion_curve(&mut self, value: u16) {
        if let Some(callback) = self.set_reversion_curve_callback {
            (callback)(value, self.context);
        };
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        (self.voltage_scale_factor_callback)(self.context)
    }

    /// Watt Scale Factor
    ///
    /// Scale factor for curve watt points.
    fn watt_scale_factor(&self) -> u16 {
        (self.watt_scale_factor_callback)(self.context)
    }

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16 {
        (self.open_loop_scale_factor_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model706StatefulAdapter {
    der_volt_watt_module_enable: Ena,
    adopt_curve_request: u16,
    adopt_curve_result: AdptCrvRslt,
    number_of_points: u16,
    stored_curve_count: u16,
    reversion_timeout: u32,
    reversion_time_remaining: u32,
    reversion_curve: u16,
    voltage_scale_factor: u16,
    watt_scale_factor: u16,
    open_loop_scale_factor: u16,
}

impl ModelAdapter for Model706StatefulAdapter {
    /// DER Volt-Watt Module Enable
    ///
    /// Volt-Watt control enable.
    fn der_volt_watt_module_enable(&self) -> Ena {
        self.der_volt_watt_module_enable
    }

    /// DER Volt-Watt Module Enable
    ///
    /// Volt-Watt control enable.
    fn set_der_volt_watt_module_enable(&mut self, value: Ena) {
        self.der_volt_watt_module_enable = value;
    }

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn adopt_curve_request(&self) -> u16 {
        self.adopt_curve_request
    }

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn set_adopt_curve_request(&mut self, value: u16) {
        self.adopt_curve_request = value;
    }

    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
    fn adopt_curve_result(&self) -> AdptCrvRslt {
        self.adopt_curve_result
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
        Some(self.reversion_timeout)
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {
        self.reversion_timeout = value;
    }

    /// Reversion Time Remaining
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_remaining(&self) -> Option<u32> {
        Some(self.reversion_time_remaining)
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn reversion_curve(&self) -> Option<u16> {
        Some(self.reversion_curve)
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn set_reversion_curve(&mut self, value: u16) {
        self.reversion_curve = value;
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        self.voltage_scale_factor
    }

    /// Watt Scale Factor
    ///
    /// Scale factor for curve watt points.
    fn watt_scale_factor(&self) -> u16 {
        self.watt_scale_factor
    }

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16 {
        self.open_loop_scale_factor
    }
}
