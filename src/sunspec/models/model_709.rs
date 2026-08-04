use core::ffi::c_void;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 9;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 709 },
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
        reference: PointReference::Model709 { point: Point::DerTripLfModuleEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model709 { point: Point::AdoptCurveRequest },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model709 { point: Point::AdoptCurveResult },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model709 { point: Point::NumberOfPoints },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model709 { point: Point::StoredCurveCount },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model709 { point: Point::FrequencyScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model709 { point: Point::TimePointScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    DerTripLfModuleEnable,
    AdoptCurveRequest,
    AdoptCurveResult,
    NumberOfPoints,
    StoredCurveCount,
    FrequencyScaleFactor,
    TimePointScaleFactor,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::DerTripLfModuleEnable => {
            serialisation::write_u16(model.der_trip_lf_module_enable() as u16, buffer);
        },
        Point::AdoptCurveRequest => {
            serialisation::write_u16(model.adopt_curve_request(), buffer);
        },
        Point::AdoptCurveResult => {
            serialisation::write_u16(model.adopt_curve_result() as u16, buffer);
        },
        Point::NumberOfPoints => {
            serialisation::write_u16(model.number_of_points(), buffer);
        },
        Point::StoredCurveCount => {
            serialisation::write_u16(model.stored_curve_count(), buffer);
        },
        Point::FrequencyScaleFactor => {
            serialisation::write_u16(model.frequency_scale_factor(), buffer);
        },
        Point::TimePointScaleFactor => {
            serialisation::write_u16(model.time_point_scale_factor(), buffer);
        },
    }
}

pub trait ModelAdapter {
    /// DER Trip LF Module Enable
    ///
    /// DER low frequency trip control enable.
    fn der_trip_lf_module_enable(&self) -> Ena;

    /// DER Trip LF Module Enable
    ///
    /// DER low frequency trip control enable.
    fn set_der_trip_lf_module_enable(&mut self, value: Ena);

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

    /// Frequency Scale Factor
    ///
    /// Scale factor for curve frequency points.
    fn frequency_scale_factor(&self) -> u16;

    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    fn time_point_scale_factor(&self) -> u16;
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
pub struct Model709CallbackAdapter {
    context: *mut c_void,
    der_trip_lf_module_enable_callback: extern "C" fn(*const c_void) -> Ena,
    set_der_trip_lf_module_enable_callback: extern "C" fn(Ena, *mut c_void),
    adopt_curve_request_callback: extern "C" fn(*const c_void) -> u16,
    set_adopt_curve_request_callback: extern "C" fn(u16, *mut c_void),
    adopt_curve_result_callback: extern "C" fn(*const c_void) -> AdptCrvRslt,
    number_of_points_callback: extern "C" fn(*const c_void) -> u16,
    stored_curve_count_callback: extern "C" fn(*const c_void) -> u16,
    frequency_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    time_point_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model709CallbackAdapter {
    /// DER Trip LF Module Enable
    ///
    /// DER low frequency trip control enable.
    fn der_trip_lf_module_enable(&self) -> Ena {
        (self.der_trip_lf_module_enable_callback)(self.context)
    }

    /// DER Trip LF Module Enable
    ///
    /// DER low frequency trip control enable.
    fn set_der_trip_lf_module_enable(&mut self, value: Ena) {
        (self.set_der_trip_lf_module_enable_callback)(value, self.context);
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

    /// Frequency Scale Factor
    ///
    /// Scale factor for curve frequency points.
    fn frequency_scale_factor(&self) -> u16 {
        (self.frequency_scale_factor_callback)(self.context)
    }

    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    fn time_point_scale_factor(&self) -> u16 {
        (self.time_point_scale_factor_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model709StatefulAdapter {
    der_trip_lf_module_enable: Ena,
    adopt_curve_request: u16,
    adopt_curve_result: AdptCrvRslt,
    number_of_points: u16,
    stored_curve_count: u16,
    frequency_scale_factor: u16,
    time_point_scale_factor: u16,
}

impl ModelAdapter for Model709StatefulAdapter {
    /// DER Trip LF Module Enable
    ///
    /// DER low frequency trip control enable.
    fn der_trip_lf_module_enable(&self) -> Ena {
        self.der_trip_lf_module_enable
    }

    /// DER Trip LF Module Enable
    ///
    /// DER low frequency trip control enable.
    fn set_der_trip_lf_module_enable(&mut self, value: Ena) {
        self.der_trip_lf_module_enable = value;
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

    /// Frequency Scale Factor
    ///
    /// Scale factor for curve frequency points.
    fn frequency_scale_factor(&self) -> u16 {
        self.frequency_scale_factor
    }

    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    fn time_point_scale_factor(&self) -> u16 {
        self.time_point_scale_factor
    }
}