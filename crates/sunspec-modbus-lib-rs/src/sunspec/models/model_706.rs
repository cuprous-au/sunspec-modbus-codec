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
    CrvActivePoints { crv_index: u16 },
    CrvDependentReference { crv_index: u16 },
    CrvOpenLoopResponseTime { crv_index: u16 },
    CrvCurveAccess { crv_index: u16 },
    PtVoltagePoint { crv_index: u16, pt_index: u16 },
    PtDependentReference { crv_index: u16, pt_index: u16 },
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    15 + model.stored_curve_count() * (5 + model.number_of_points() * (2))
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
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
        Point::CrvActivePoints { crv_index } => {
            buffer::write_u16(model.crv_active_points(*crv_index), buffer);
        }
        Point::CrvDependentReference { crv_index } => {
            buffer::write_u16(model.crv_dependent_reference(*crv_index) as u16, buffer);
        }
        Point::CrvOpenLoopResponseTime { crv_index } => {
            if let Some(value) = model.crv_open_loop_response_time(*crv_index) {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CrvCurveAccess { crv_index } => {
            buffer::write_u16(model.crv_curve_access(*crv_index) as u16, buffer);
        }
        Point::PtVoltagePoint {
            crv_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_voltage_point(*crv_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtDependentReference {
            crv_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_dependent_reference(*crv_index, *pt_index) {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
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

    /// Active Points
    ///
    /// Number of active points.
    fn crv_active_points(&self, crv_index: u16) -> u16;

    /// Active Points
    ///
    /// Number of active points.
    fn set_crv_active_points(&mut self, value: u16, crv_index: u16);

    /// Dependent Reference
    ///
    /// Curve dependent reference.
    fn crv_dependent_reference(&self, crv_index: u16) -> DeptRef;

    /// Dependent Reference
    ///
    /// Curve dependent reference.
    fn set_crv_dependent_reference(&mut self, value: DeptRef, crv_index: u16);

    /// Open Loop Response Time
    ///
    /// Open loop response time.
    fn crv_open_loop_response_time(&self, crv_index: u16) -> Option<u32> {
        None
    }

    /// Open Loop Response Time
    ///
    /// Open loop response time.
    fn set_crv_open_loop_response_time(&mut self, value: u32, crv_index: u16) {}

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly;

    /// Voltage Point
    ///
    /// Curve voltage point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn pt_voltage_point(&self, crv_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Voltage Point
    ///
    /// Curve voltage point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn set_pt_voltage_point(&mut self, value: u16, crv_index: u16, pt_index: u16) {}

    /// Dependent Reference
    ///
    /// Active power in percent of rated active power.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn pt_dependent_reference(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        None
    }

    /// Dependent Reference
    ///
    /// Active power in percent of rated active power.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn set_pt_dependent_reference(&mut self, value: i16, crv_index: u16, pt_index: u16) {}
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
pub enum DeptRef {
    WMaxPct = 0,
    WAvalPct = 1,
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
pub enum ReadOnly {
    /// Read-Write Access
    ///
    /// Curve has read-write access.
    Rw = 0,
    /// Read-Only Access
    ///
    /// Curve has read-only access.
    R = 1,
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
    crv_active_points_callback: extern "C" fn(*const c_void, u16) -> u16,
    set_crv_active_points_callback: extern "C" fn(u16, *mut c_void, u16),
    crv_dependent_reference_callback: extern "C" fn(*const c_void, u16) -> DeptRef,
    set_crv_dependent_reference_callback: extern "C" fn(DeptRef, *mut c_void, u16),
    crv_open_loop_response_time_callback: Option<extern "C" fn(*const c_void, u16) -> u32>,
    set_crv_open_loop_response_time_callback: Option<extern "C" fn(u32, *mut c_void, u16)>,
    crv_curve_access_callback: extern "C" fn(*const c_void, u16) -> ReadOnly,
    pt_voltage_point_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_voltage_point_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
    pt_dependent_reference_callback: Option<extern "C" fn(*const c_void, u16, u16) -> i16>,
    set_pt_dependent_reference_callback: Option<extern "C" fn(i16, *mut c_void, u16, u16)>,
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

    /// Active Points
    ///
    /// Number of active points.
    fn crv_active_points(&self, crv_index: u16) -> u16 {
        (self.crv_active_points_callback)(self.context, crv_index)
    }

    /// Active Points
    ///
    /// Number of active points.
    fn set_crv_active_points(&mut self, value: u16, crv_index: u16) {
        (self.set_crv_active_points_callback)(value, self.context, crv_index);
    }

    /// Dependent Reference
    ///
    /// Curve dependent reference.
    fn crv_dependent_reference(&self, crv_index: u16) -> DeptRef {
        (self.crv_dependent_reference_callback)(self.context, crv_index)
    }

    /// Dependent Reference
    ///
    /// Curve dependent reference.
    fn set_crv_dependent_reference(&mut self, value: DeptRef, crv_index: u16) {
        (self.set_crv_dependent_reference_callback)(value, self.context, crv_index);
    }

    /// Open Loop Response Time
    ///
    /// Open loop response time.
    fn crv_open_loop_response_time(&self, crv_index: u16) -> Option<u32> {
        self.crv_open_loop_response_time_callback
            .map(|callback| (callback)(self.context, crv_index))
    }

    /// Open Loop Response Time
    ///
    /// Open loop response time.
    fn set_crv_open_loop_response_time(&mut self, value: u32, crv_index: u16) {
        if let Some(callback) = self.set_crv_open_loop_response_time_callback {
            (callback)(value, self.context, crv_index);
        };
    }

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly {
        (self.crv_curve_access_callback)(self.context, crv_index)
    }

    /// Voltage Point
    ///
    /// Curve voltage point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn pt_voltage_point(&self, crv_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_voltage_point_callback
            .map(|callback| (callback)(self.context, crv_index, pt_index))
    }

    /// Voltage Point
    ///
    /// Curve voltage point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn set_pt_voltage_point(&mut self, value: u16, crv_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_voltage_point_callback {
            (callback)(value, self.context, crv_index, pt_index);
        };
    }

    /// Dependent Reference
    ///
    /// Active power in percent of rated active power.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn pt_dependent_reference(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        self.pt_dependent_reference_callback
            .map(|callback| (callback)(self.context, crv_index, pt_index))
    }

    /// Dependent Reference
    ///
    /// Active power in percent of rated active power.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn set_pt_dependent_reference(&mut self, value: i16, crv_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_dependent_reference_callback {
            (callback)(value, self.context, crv_index, pt_index);
        };
    }
}

#[repr(C)]
pub struct Model706StatefulAdapter<const STORED_CURVE_COUNT: usize, const NUMBER_OF_POINTS: usize> {
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
    stored_curves: [Model706StoredCurves<NUMBER_OF_POINTS>; STORED_CURVE_COUNT],
}

#[repr(C)]
pub struct Model706StoredCurves<const NUMBER_OF_POINTS: usize> {
    crv_active_points: u16,
    crv_dependent_reference: DeptRef,
    crv_open_loop_response_time: u32,
    crv_curve_access: ReadOnly,
    stored_curve_points: [Model706StoredCurvePoints; NUMBER_OF_POINTS],
}

#[repr(C)]
pub struct Model706StoredCurvePoints {
    pt_voltage_point: u16,
    pt_dependent_reference: i16,
}

impl<const STORED_CURVE_COUNT: usize, const NUMBER_OF_POINTS: usize> ModelAdapter
    for Model706StatefulAdapter<STORED_CURVE_COUNT, NUMBER_OF_POINTS>
{
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

    /// Active Points
    ///
    /// Number of active points.
    fn crv_active_points(&self, crv_index: u16) -> u16 {
        self.stored_curves[crv_index as usize].crv_active_points
    }

    /// Active Points
    ///
    /// Number of active points.
    fn set_crv_active_points(&mut self, value: u16, crv_index: u16) {
        self.stored_curves[crv_index as usize].crv_active_points = value;
    }

    /// Dependent Reference
    ///
    /// Curve dependent reference.
    fn crv_dependent_reference(&self, crv_index: u16) -> DeptRef {
        self.stored_curves[crv_index as usize].crv_dependent_reference
    }

    /// Dependent Reference
    ///
    /// Curve dependent reference.
    fn set_crv_dependent_reference(&mut self, value: DeptRef, crv_index: u16) {
        self.stored_curves[crv_index as usize].crv_dependent_reference = value;
    }

    /// Open Loop Response Time
    ///
    /// Open loop response time.
    fn crv_open_loop_response_time(&self, crv_index: u16) -> Option<u32> {
        Some(self.stored_curves[crv_index as usize].crv_open_loop_response_time)
    }

    /// Open Loop Response Time
    ///
    /// Open loop response time.
    fn set_crv_open_loop_response_time(&mut self, value: u32, crv_index: u16) {
        self.stored_curves[crv_index as usize].crv_open_loop_response_time = value;
    }

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly {
        self.stored_curves[crv_index as usize].crv_curve_access
    }

    /// Voltage Point
    ///
    /// Curve voltage point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn pt_voltage_point(&self, crv_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_curves[crv_index as usize].stored_curve_points[pt_index as usize]
                .pt_voltage_point,
        )
    }

    /// Voltage Point
    ///
    /// Curve voltage point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn set_pt_voltage_point(&mut self, value: u16, crv_index: u16, pt_index: u16) {
        self.stored_curves[crv_index as usize].stored_curve_points[pt_index as usize]
            .pt_voltage_point = value;
    }

    /// Dependent Reference
    ///
    /// Active power in percent of rated active power.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn pt_dependent_reference(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        Some(
            self.stored_curves[crv_index as usize].stored_curve_points[pt_index as usize]
                .pt_dependent_reference,
        )
    }

    /// Dependent Reference
    ///
    /// Active power in percent of rated active power.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes.
    fn set_pt_dependent_reference(&mut self, value: i16, crv_index: u16, pt_index: u16) {
        self.stored_curves[crv_index as usize].stored_curve_points[pt_index as usize]
            .pt_dependent_reference = value;
    }
}
