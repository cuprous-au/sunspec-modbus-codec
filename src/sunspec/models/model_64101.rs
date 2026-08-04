use core::ffi::c_void;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 9;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64101 },
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
        reference: PointReference::Model64101 { point: Point::EltekCountryCode },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64101 { point: Point::EltekFeedingPhase },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64101 { point: Point::EltekApdMethod },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64101 { point: Point::EltekApdPowerRef },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64101 { point: Point::EltekRpsMethod },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64101 { point: Point::EltekRpsQRef },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64101 { point: Point::EltekRpsCosPhiRef },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    EltekCountryCode,
    EltekFeedingPhase,
    EltekApdMethod,
    EltekApdPowerRef,
    EltekRpsMethod,
    EltekRpsQRef,
    EltekRpsCosPhiRef,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::EltekCountryCode => {
            if let Some(value) = model.eltek_country_code() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::EltekFeedingPhase => {
            if let Some(value) = model.eltek_feeding_phase() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::EltekApdMethod => {
            if let Some(value) = model.eltek_apd_method() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::EltekApdPowerRef => {
            if let Some(value) = model.eltek_apd_power_ref() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::EltekRpsMethod => {
            if let Some(value) = model.eltek_rps_method() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::EltekRpsQRef => {
            if let Some(value) = model.eltek_rps_q_ref() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::EltekRpsCosPhiRef => {
            if let Some(value) = model.eltek_rps_cos_phi_ref() {
                serialisation::write_i16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
    }
}

pub trait ModelAdapter {
    fn eltek_country_code(&self) -> Option<u16> {
        None
    }

    fn eltek_feeding_phase(&self) -> Option<u16> {
        None
    }

    fn eltek_apd_method(&self) -> Option<u16> {
        None
    }

    fn eltek_apd_power_ref(&self) -> Option<u16> {
        None
    }

    fn eltek_rps_method(&self) -> Option<u16> {
        None
    }

    fn eltek_rps_q_ref(&self) -> Option<u16> {
        None
    }

    fn eltek_rps_cos_phi_ref(&self) -> Option<i16> {
        None
    }
}

#[repr(C)]
pub struct Model64101CallbackAdapter {
    context: *mut c_void,
    eltek_country_code_callback: Option<extern "C" fn(*const c_void) -> u16>,
    eltek_feeding_phase_callback: Option<extern "C" fn(*const c_void) -> u16>,
    eltek_apd_method_callback: Option<extern "C" fn(*const c_void) -> u16>,
    eltek_apd_power_ref_callback: Option<extern "C" fn(*const c_void) -> u16>,
    eltek_rps_method_callback: Option<extern "C" fn(*const c_void) -> u16>,
    eltek_rps_q_ref_callback: Option<extern "C" fn(*const c_void) -> u16>,
    eltek_rps_cos_phi_ref_callback: Option<extern "C" fn(*const c_void) -> i16>,
}

impl ModelAdapter for Model64101CallbackAdapter {
    fn eltek_country_code(&self) -> Option<u16> {
        self.eltek_country_code_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    fn eltek_feeding_phase(&self) -> Option<u16> {
        self.eltek_feeding_phase_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    fn eltek_apd_method(&self) -> Option<u16> {
        self.eltek_apd_method_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    fn eltek_apd_power_ref(&self) -> Option<u16> {
        self.eltek_apd_power_ref_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    fn eltek_rps_method(&self) -> Option<u16> {
        self.eltek_rps_method_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    fn eltek_rps_q_ref(&self) -> Option<u16> {
        self.eltek_rps_q_ref_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    fn eltek_rps_cos_phi_ref(&self) -> Option<i16> {
        self.eltek_rps_cos_phi_ref_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model64101StatefulAdapter {
    eltek_country_code: u16,
    eltek_feeding_phase: u16,
    eltek_apd_method: u16,
    eltek_apd_power_ref: u16,
    eltek_rps_method: u16,
    eltek_rps_q_ref: u16,
    eltek_rps_cos_phi_ref: i16,
}

impl ModelAdapter for Model64101StatefulAdapter {
    fn eltek_country_code(&self) -> Option<u16> {
        Some(
        self.eltek_country_code
        )
    }

    fn eltek_feeding_phase(&self) -> Option<u16> {
        Some(
        self.eltek_feeding_phase
        )
    }

    fn eltek_apd_method(&self) -> Option<u16> {
        Some(
        self.eltek_apd_method
        )
    }

    fn eltek_apd_power_ref(&self) -> Option<u16> {
        Some(
        self.eltek_apd_power_ref
        )
    }

    fn eltek_rps_method(&self) -> Option<u16> {
        Some(
        self.eltek_rps_method
        )
    }

    fn eltek_rps_q_ref(&self) -> Option<u16> {
        Some(
        self.eltek_rps_q_ref
        )
    }

    fn eltek_rps_cos_phi_ref(&self) -> Option<i16> {
        Some(
        self.eltek_rps_cos_phi_ref
        )
    }
}