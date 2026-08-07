use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 9;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 708 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model708 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model708 {
            point: Point::DerTripHvModuleEnable,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model708 {
            point: Point::AdoptCurveRequest,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model708 {
            point: Point::AdoptCurveResult,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model708 {
            point: Point::NumberOfPoints,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model708 {
            point: Point::StoredCurveCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model708 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model708 {
            point: Point::TimePointScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    DerTripHvModuleEnable,
    AdoptCurveRequest,
    AdoptCurveResult,
    NumberOfPoints,
    StoredCurveCount,
    VoltageScaleFactor,
    TimePointScaleFactor,
    CrvCurveAccess { crv_index: u16 },
    MustTripCurveCrvNumberOfActivePoints { crv_index: u16 },
    MayTripCurveCrvNumberOfActivePoints { crv_index: u16 },
    MomentaryCessationCurveCrvNumberOfActivePoints { crv_index: u16 },
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    9 + model.stored_curve_count() * (4)
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
        Point::DerTripHvModuleEnable => {
            buffer::write_u16(model.der_trip_hv_module_enable() as u16, buffer);
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
        Point::VoltageScaleFactor => {
            buffer::write_u16(model.voltage_scale_factor(), buffer);
        }
        Point::TimePointScaleFactor => {
            buffer::write_u16(model.time_point_scale_factor(), buffer);
        }
        Point::CrvCurveAccess { crv_index } => {
            buffer::write_u16(model.crv_curve_access(*crv_index) as u16, buffer);
        }
        Point::MustTripCurveCrvNumberOfActivePoints { crv_index } => {
            if let Some(value) = model.must_trip_curve_crv_number_of_active_points(*crv_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MayTripCurveCrvNumberOfActivePoints { crv_index } => {
            if let Some(value) = model.may_trip_curve_crv_number_of_active_points(*crv_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MomentaryCessationCurveCrvNumberOfActivePoints { crv_index } => {
            if let Some(value) =
                model.momentary_cessation_curve_crv_number_of_active_points(*crv_index)
            {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
    fn der_trip_hv_module_enable(&self) -> Ena;

    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
    fn set_der_trip_hv_module_enable(&mut self, value: Ena);

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

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn voltage_scale_factor(&self) -> u16;

    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    fn time_point_scale_factor(&self) -> u16;

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly;

    /// Number Of Active Points
    ///
    /// Number of active points in must trip curve.
    fn must_trip_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        None
    }

    /// Number Of Active Points
    ///
    /// Number of active points in must trip curve.
    fn set_must_trip_curve_crv_number_of_active_points(&mut self, value: u16, crv_index: u16) {}

    /// Number Of Active Points
    ///
    /// Number of active points in may trip curve.
    fn may_trip_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        None
    }

    /// Number Of Active Points
    ///
    /// Number of active points in may trip curve.
    fn set_may_trip_curve_crv_number_of_active_points(&mut self, value: u16, crv_index: u16) {}

    /// Number Of Active Points
    ///
    /// Number of active points in the momentary cessation curve.
    fn momentary_cessation_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        None
    }

    /// Number Of Active Points
    ///
    /// Number of active points in the momentary cessation curve.
    fn set_momentary_cessation_curve_crv_number_of_active_points(
        &mut self,
        value: u16,
        crv_index: u16,
    ) {
    }
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
pub struct Model708CallbackAdapter {
    context: *mut c_void,
    der_trip_hv_module_enable_callback: extern "C" fn(*const c_void) -> Ena,
    set_der_trip_hv_module_enable_callback: extern "C" fn(Ena, *mut c_void),
    adopt_curve_request_callback: extern "C" fn(*const c_void) -> u16,
    set_adopt_curve_request_callback: extern "C" fn(u16, *mut c_void),
    adopt_curve_result_callback: extern "C" fn(*const c_void) -> AdptCrvRslt,
    number_of_points_callback: extern "C" fn(*const c_void) -> u16,
    stored_curve_count_callback: extern "C" fn(*const c_void) -> u16,
    voltage_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    time_point_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    crv_curve_access_callback: extern "C" fn(*const c_void, u16) -> ReadOnly,
    must_trip_curve_crv_number_of_active_points_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    set_must_trip_curve_crv_number_of_active_points_callback:
        Option<extern "C" fn(u16, *mut c_void, u16)>,
    may_trip_curve_crv_number_of_active_points_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    set_may_trip_curve_crv_number_of_active_points_callback:
        Option<extern "C" fn(u16, *mut c_void, u16)>,
    momentary_cessation_curve_crv_number_of_active_points_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    set_momentary_cessation_curve_crv_number_of_active_points_callback:
        Option<extern "C" fn(u16, *mut c_void, u16)>,
}

impl ModelAdapter for Model708CallbackAdapter {
    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
    fn der_trip_hv_module_enable(&self) -> Ena {
        (self.der_trip_hv_module_enable_callback)(self.context)
    }

    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
    fn set_der_trip_hv_module_enable(&mut self, value: Ena) {
        (self.set_der_trip_hv_module_enable_callback)(value, self.context);
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

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        (self.voltage_scale_factor_callback)(self.context)
    }

    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    fn time_point_scale_factor(&self) -> u16 {
        (self.time_point_scale_factor_callback)(self.context)
    }

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly {
        (self.crv_curve_access_callback)(self.context, crv_index)
    }

    /// Number Of Active Points
    ///
    /// Number of active points in must trip curve.
    fn must_trip_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        self.must_trip_curve_crv_number_of_active_points_callback
            .map(|callback| (callback)(self.context, crv_index))
    }

    /// Number Of Active Points
    ///
    /// Number of active points in must trip curve.
    fn set_must_trip_curve_crv_number_of_active_points(&mut self, value: u16, crv_index: u16) {
        if let Some(callback) = self.set_must_trip_curve_crv_number_of_active_points_callback {
            (callback)(value, self.context, crv_index);
        };
    }

    /// Number Of Active Points
    ///
    /// Number of active points in may trip curve.
    fn may_trip_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        self.may_trip_curve_crv_number_of_active_points_callback
            .map(|callback| (callback)(self.context, crv_index))
    }

    /// Number Of Active Points
    ///
    /// Number of active points in may trip curve.
    fn set_may_trip_curve_crv_number_of_active_points(&mut self, value: u16, crv_index: u16) {
        if let Some(callback) = self.set_may_trip_curve_crv_number_of_active_points_callback {
            (callback)(value, self.context, crv_index);
        };
    }

    /// Number Of Active Points
    ///
    /// Number of active points in the momentary cessation curve.
    fn momentary_cessation_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        self.momentary_cessation_curve_crv_number_of_active_points_callback
            .map(|callback| (callback)(self.context, crv_index))
    }

    /// Number Of Active Points
    ///
    /// Number of active points in the momentary cessation curve.
    fn set_momentary_cessation_curve_crv_number_of_active_points(
        &mut self,
        value: u16,
        crv_index: u16,
    ) {
        if let Some(callback) =
            self.set_momentary_cessation_curve_crv_number_of_active_points_callback
        {
            (callback)(value, self.context, crv_index);
        };
    }
}

#[repr(C)]
pub struct Model708StatefulAdapter<const STORED_CURVE_COUNT: usize> {
    der_trip_hv_module_enable: Ena,
    adopt_curve_request: u16,
    adopt_curve_result: AdptCrvRslt,
    number_of_points: u16,
    stored_curve_count: u16,
    voltage_scale_factor: u16,
    time_point_scale_factor: u16,
    stored_curves: [Model708StoredCurves; STORED_CURVE_COUNT],
}

#[repr(C)]
pub struct Model708StoredCurves {
    crv_curve_access: ReadOnly,
    must_trip_curve_crv_number_of_active_points: u16,
    may_trip_curve_crv_number_of_active_points: u16,
    momentary_cessation_curve_crv_number_of_active_points: u16,
}

impl<const STORED_CURVE_COUNT: usize> ModelAdapter for Model708StatefulAdapter<STORED_CURVE_COUNT> {
    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
    fn der_trip_hv_module_enable(&self) -> Ena {
        self.der_trip_hv_module_enable
    }

    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
    fn set_der_trip_hv_module_enable(&mut self, value: Ena) {
        self.der_trip_hv_module_enable = value;
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

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        self.voltage_scale_factor
    }

    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    fn time_point_scale_factor(&self) -> u16 {
        self.time_point_scale_factor
    }

    /// Curve Access
    ///
    /// Curve read-write access.
    fn crv_curve_access(&self, crv_index: u16) -> ReadOnly {
        self.stored_curves[crv_index as usize].crv_curve_access
    }

    /// Number Of Active Points
    ///
    /// Number of active points in must trip curve.
    fn must_trip_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        Some(self.stored_curves[crv_index as usize].must_trip_curve_crv_number_of_active_points)
    }

    /// Number Of Active Points
    ///
    /// Number of active points in must trip curve.
    fn set_must_trip_curve_crv_number_of_active_points(&mut self, value: u16, crv_index: u16) {
        self.stored_curves[crv_index as usize].must_trip_curve_crv_number_of_active_points = value;
    }

    /// Number Of Active Points
    ///
    /// Number of active points in may trip curve.
    fn may_trip_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        Some(self.stored_curves[crv_index as usize].may_trip_curve_crv_number_of_active_points)
    }

    /// Number Of Active Points
    ///
    /// Number of active points in may trip curve.
    fn set_may_trip_curve_crv_number_of_active_points(&mut self, value: u16, crv_index: u16) {
        self.stored_curves[crv_index as usize].may_trip_curve_crv_number_of_active_points = value;
    }

    /// Number Of Active Points
    ///
    /// Number of active points in the momentary cessation curve.
    fn momentary_cessation_curve_crv_number_of_active_points(&self, crv_index: u16) -> Option<u16> {
        Some(
            self.stored_curves[crv_index as usize]
                .momentary_cessation_curve_crv_number_of_active_points,
        )
    }

    /// Number Of Active Points
    ///
    /// Number of active points in the momentary cessation curve.
    fn set_momentary_cessation_curve_crv_number_of_active_points(
        &mut self,
        value: u16,
        crv_index: u16,
    ) {
        self.stored_curves[crv_index as usize]
            .momentary_cessation_curve_crv_number_of_active_points = value;
    }
}
