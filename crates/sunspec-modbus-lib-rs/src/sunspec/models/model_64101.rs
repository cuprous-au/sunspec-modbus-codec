use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 9;

static POINTS: [PointDetails<()>; 9] = [
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
        point: |()| Point::EltekCountryCode,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::EltekFeedingPhase,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::EltekApdMethod,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::EltekApdPowerRef,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::EltekRpsMethod,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::EltekRpsQRef,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::EltekRpsCosPhiRef,
        size: 1,
        start_address: 8,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    EltekCountryCode,
    EltekFeedingPhase,
    EltekApdMethod,
    EltekApdPowerRef,
    EltekRpsMethod,
    EltekRpsQRef,
    EltekRpsCosPhiRef,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    9
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
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
            buffer::write_u16(64101, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::EltekCountryCode => {
            if let Some(value) = model.eltek_country_code() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::EltekFeedingPhase => {
            if let Some(value) = model.eltek_feeding_phase() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::EltekApdMethod => {
            if let Some(value) = model.eltek_apd_method() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::EltekApdPowerRef => {
            if let Some(value) = model.eltek_apd_power_ref() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::EltekRpsMethod => {
            if let Some(value) = model.eltek_rps_method() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::EltekRpsQRef => {
            if let Some(value) = model.eltek_rps_q_ref() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::EltekRpsCosPhiRef => {
            if let Some(value) = model.eltek_rps_cos_phi_ref() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
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
        self.eltek_country_code_callback
            .map(|callback| (callback)(self.context))
    }

    fn eltek_feeding_phase(&self) -> Option<u16> {
        self.eltek_feeding_phase_callback
            .map(|callback| (callback)(self.context))
    }

    fn eltek_apd_method(&self) -> Option<u16> {
        self.eltek_apd_method_callback
            .map(|callback| (callback)(self.context))
    }

    fn eltek_apd_power_ref(&self) -> Option<u16> {
        self.eltek_apd_power_ref_callback
            .map(|callback| (callback)(self.context))
    }

    fn eltek_rps_method(&self) -> Option<u16> {
        self.eltek_rps_method_callback
            .map(|callback| (callback)(self.context))
    }

    fn eltek_rps_q_ref(&self) -> Option<u16> {
        self.eltek_rps_q_ref_callback
            .map(|callback| (callback)(self.context))
    }

    fn eltek_rps_cos_phi_ref(&self) -> Option<i16> {
        self.eltek_rps_cos_phi_ref_callback
            .map(|callback| (callback)(self.context))
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
        Some(self.eltek_country_code)
    }

    fn eltek_feeding_phase(&self) -> Option<u16> {
        Some(self.eltek_feeding_phase)
    }

    fn eltek_apd_method(&self) -> Option<u16> {
        Some(self.eltek_apd_method)
    }

    fn eltek_apd_power_ref(&self) -> Option<u16> {
        Some(self.eltek_apd_power_ref)
    }

    fn eltek_rps_method(&self) -> Option<u16> {
        Some(self.eltek_rps_method)
    }

    fn eltek_rps_q_ref(&self) -> Option<u16> {
        Some(self.eltek_rps_q_ref)
    }

    fn eltek_rps_cos_phi_ref(&self) -> Option<i16> {
        Some(self.eltek_rps_cos_phi_ref)
    }
}
