use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 14;

static POINTS: [PointDetails<()>; 12] = [
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
        point: |()| Point::DerWattVarModuleEnable,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::ActiveCurveRequest,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::SetActiveCurveResult,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::NumberOfPoints,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::StoredCurveCount,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::ReversionTimeout,
        size: 2,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::ReversionTimeLeft,
        size: 2,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::ReversionCurve,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::ActivePowerScaleFactor,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::VarScaleFactor,
        size: 1,
        start_address: 13,
    },
];

static CRV_POINTS: [PointDetails<u16>; 4] = [
    PointDetails {
        point: |crv_index| Point::CrvActivePoints { crv_index },
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |crv_index| Point::CrvDependentReference { crv_index },
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |crv_index| Point::CrvPowerPriority { crv_index },
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |crv_index| Point::CrvCurveAccess { crv_index },
        size: 1,
        start_address: 3,
    },
];

static PT_POINTS: [PointDetails<(u16, u16)>; 2] = [
    PointDetails {
        point: |(crv_index, pt_index)| Point::PtActivePowerPoint {
            crv_index,
            pt_index,
        },
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |(crv_index, pt_index)| Point::PtReactivePowerPoint {
            crv_index,
            pt_index,
        },
        size: 1,
        start_address: 1,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
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
    CrvActivePoints { crv_index: u16 },
    CrvDependentReference { crv_index: u16 },
    CrvPowerPriority { crv_index: u16 },
    CrvCurveAccess { crv_index: u16 },
    PtActivePowerPoint { crv_index: u16, pt_index: u16 },
    PtReactivePowerPoint { crv_index: u16, pt_index: u16 },
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    14 + model.stored_curve_count() * (4 + model.number_of_points() * (2))
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    let pt_count = model.number_of_points();
    let pt_size = 2;

    let crv_count = model.stored_curve_count();
    let crv_size = 4 + pt_count * pt_size;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .chain((0..crv_count).flat_map(move |crv_index| {
            let crv_address = 14 + crv_index * crv_size;
            CRV_POINTS
                .iter()
                .map(move |p| (crv_address + p.start_address, p.size, (p.point)(crv_index)))
                .chain((0..pt_count).flat_map(move |pt_index| {
                    let pt_address = crv_address + pt_index * pt_size;
                    PT_POINTS.iter().map(move |p| {
                        (
                            pt_address + p.start_address,
                            p.size,
                            (p.point)((crv_index, pt_index)),
                        )
                    })
                }))
        }))
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
            buffer::write_u16(712, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::DerWattVarModuleEnable => {
            buffer::write_u16(model.der_watt_var_module_enable() as u16, buffer);
        }
        Point::ActiveCurveRequest => {
            buffer::write_u16(model.active_curve_request(), buffer);
        }
        Point::SetActiveCurveResult => {
            buffer::write_u16(model.set_active_curve_result() as u16, buffer);
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
        Point::ReversionTimeLeft => {
            if let Some(value) = model.reversion_time_left() {
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
        Point::ActivePowerScaleFactor => {
            buffer::write_u16(model.active_power_scale_factor(), buffer);
        }
        Point::VarScaleFactor => {
            buffer::write_u16(model.var_scale_factor(), buffer);
        }
        Point::CrvActivePoints { crv_index } => {
            buffer::write_u16(model.crv_active_points(*crv_index), buffer);
        }
        Point::CrvDependentReference { crv_index } => {
            buffer::write_u16(model.crv_dependent_reference(*crv_index) as u16, buffer);
        }
        Point::CrvPowerPriority { crv_index } => {
            if let Some(value) = model.crv_power_priority(*crv_index) {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CrvCurveAccess { crv_index } => {
            buffer::write_u16(model.crv_curve_access(*crv_index) as u16, buffer);
        }
        Point::PtActivePowerPoint {
            crv_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_active_power_point(*crv_index, *pt_index) {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtReactivePowerPoint {
            crv_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_reactive_power_point(*crv_index, *pt_index) {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
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
    fn set_reversion_timeout(&mut self, value: u32) {}

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
    fn set_reversion_curve(&mut self, value: u16) {}

    /// Active Power Scale Factor
    ///
    /// Scale factor for curve active power points.
    fn active_power_scale_factor(&self) -> u16;

    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    fn var_scale_factor(&self) -> u16;

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

    /// Power Priority
    ///
    /// Power priority.
    fn crv_power_priority(&self, crv_index: u16) -> Option<Pri> {
        None
    }

    /// Power Priority
    ///
    /// Power priority.
    fn set_crv_power_priority(&mut self, value: Pri, crv_index: u16) {}

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly;

    /// Active Power Point
    ///
    /// Curve active power point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points.
    fn pt_active_power_point(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        None
    }

    /// Active Power Point
    ///
    /// Curve active power point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points.
    fn set_pt_active_power_point(&mut self, value: i16, crv_index: u16, pt_index: u16) {}

    /// Reactive Power Point
    ///
    /// Curve reactive power point as set in DeptRef point.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points
    fn pt_reactive_power_point(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        None
    }

    /// Reactive Power Point
    ///
    /// Curve reactive power point as set in DeptRef point.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points
    fn set_pt_reactive_power_point(&mut self, value: i16, crv_index: u16, pt_index: u16) {}
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
    /// Percent Max Watts
    WMaxPct = 0,
    /// Percent Max Vars
    VarMaxPct = 1,
    /// Percent Available Vars
    VarAvalPct = 2,
    /// Percent Max Apparent Power
    VaMaxPct = 3,
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
pub enum Pri {
    /// Active Power Priority
    ///
    /// Active power priority.
    Active = 0,
    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    Reactive = 1,
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
    crv_active_points_callback: extern "C" fn(*const c_void, u16) -> u16,
    set_crv_active_points_callback: extern "C" fn(u16, *mut c_void, u16),
    crv_dependent_reference_callback: extern "C" fn(*const c_void, u16) -> DeptRef,
    set_crv_dependent_reference_callback: extern "C" fn(DeptRef, *mut c_void, u16),
    crv_power_priority_callback: Option<extern "C" fn(*const c_void, u16) -> Pri>,
    set_crv_power_priority_callback: Option<extern "C" fn(Pri, *mut c_void, u16)>,
    crv_curve_access_callback: extern "C" fn(*const c_void, u16) -> ReadOnly,
    pt_active_power_point_callback: Option<extern "C" fn(*const c_void, u16, u16) -> i16>,
    set_pt_active_power_point_callback: Option<extern "C" fn(i16, *mut c_void, u16, u16)>,
    pt_reactive_power_point_callback: Option<extern "C" fn(*const c_void, u16, u16) -> i16>,
    set_pt_reactive_power_point_callback: Option<extern "C" fn(i16, *mut c_void, u16, u16)>,
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

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
        self.reversion_time_left_callback
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

    /// Power Priority
    ///
    /// Power priority.
    fn crv_power_priority(&self, crv_index: u16) -> Option<Pri> {
        self.crv_power_priority_callback
            .map(|callback| (callback)(self.context, crv_index))
    }

    /// Power Priority
    ///
    /// Power priority.
    fn set_crv_power_priority(&mut self, value: Pri, crv_index: u16) {
        if let Some(callback) = self.set_crv_power_priority_callback {
            (callback)(value, self.context, crv_index);
        };
    }

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly {
        (self.crv_curve_access_callback)(self.context, crv_index)
    }

    /// Active Power Point
    ///
    /// Curve active power point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points.
    fn pt_active_power_point(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        self.pt_active_power_point_callback
            .map(|callback| (callback)(self.context, crv_index, pt_index))
    }

    /// Active Power Point
    ///
    /// Curve active power point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points.
    fn set_pt_active_power_point(&mut self, value: i16, crv_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_active_power_point_callback {
            (callback)(value, self.context, crv_index, pt_index);
        };
    }

    /// Reactive Power Point
    ///
    /// Curve reactive power point as set in DeptRef point.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points
    fn pt_reactive_power_point(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        self.pt_reactive_power_point_callback
            .map(|callback| (callback)(self.context, crv_index, pt_index))
    }

    /// Reactive Power Point
    ///
    /// Curve reactive power point as set in DeptRef point.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points
    fn set_pt_reactive_power_point(&mut self, value: i16, crv_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_reactive_power_point_callback {
            (callback)(value, self.context, crv_index, pt_index);
        };
    }
}

#[repr(C)]
pub struct Model712StatefulAdapter<const STORED_CURVE_COUNT: usize, const NUMBER_OF_POINTS: usize> {
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
    stored_curves: [Model712StoredCurves<NUMBER_OF_POINTS>; STORED_CURVE_COUNT],
}

#[repr(C)]
pub struct Model712StoredCurves<const NUMBER_OF_POINTS: usize> {
    crv_active_points: u16,
    crv_dependent_reference: DeptRef,
    crv_power_priority: Pri,
    crv_curve_access: ReadOnly,
    stored_curve_points: [Model712StoredCurvePoints; NUMBER_OF_POINTS],
}

#[repr(C)]
pub struct Model712StoredCurvePoints {
    pt_active_power_point: i16,
    pt_reactive_power_point: i16,
}

impl<const STORED_CURVE_COUNT: usize, const NUMBER_OF_POINTS: usize> ModelAdapter
    for Model712StatefulAdapter<STORED_CURVE_COUNT, NUMBER_OF_POINTS>
{
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
        Some(self.reversion_timeout)
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
        Some(self.reversion_time_left)
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

    /// Power Priority
    ///
    /// Power priority.
    fn crv_power_priority(&self, crv_index: u16) -> Option<Pri> {
        Some(self.stored_curves[crv_index as usize].crv_power_priority)
    }

    /// Power Priority
    ///
    /// Power priority.
    fn set_crv_power_priority(&mut self, value: Pri, crv_index: u16) {
        self.stored_curves[crv_index as usize].crv_power_priority = value;
    }

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly {
        self.stored_curves[crv_index as usize].crv_curve_access
    }

    /// Active Power Point
    ///
    /// Curve active power point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points.
    fn pt_active_power_point(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        Some(
            self.stored_curves[crv_index as usize].stored_curve_points[pt_index as usize]
                .pt_active_power_point,
        )
    }

    /// Active Power Point
    ///
    /// Curve active power point as percentage.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points.
    fn set_pt_active_power_point(&mut self, value: i16, crv_index: u16, pt_index: u16) {
        self.stored_curves[crv_index as usize].stored_curve_points[pt_index as usize]
            .pt_active_power_point = value;
    }

    /// Reactive Power Point
    ///
    /// Curve reactive power point as set in DeptRef point.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points
    fn pt_reactive_power_point(&self, crv_index: u16, pt_index: u16) -> Option<i16> {
        Some(
            self.stored_curves[crv_index as usize].stored_curve_points[pt_index as usize]
                .pt_reactive_power_point,
        )
    }

    /// Reactive Power Point
    ///
    /// Curve reactive power point as set in DeptRef point.
    ///
    /// Internal curve conformance checks should be conducted when AdptCrvReq is set to 1, not on point writes. IEEE 1547 implementations must allow 0 values for all three load points
    fn set_pt_reactive_power_point(&mut self, value: i16, crv_index: u16, pt_index: u16) {
        self.stored_curves[crv_index as usize].stored_curve_points[pt_index as usize]
            .pt_reactive_power_point = value;
    }
}
