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
        point: |()| Point::EnergyRating,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::EnergyAvailable,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::StateOfCharge,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::StateOfHealth,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::Status,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::EnergyScaleFactor,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::PercentScaleFactor,
        size: 1,
        start_address: 8,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    EnergyRating,
    EnergyAvailable,
    StateOfCharge,
    StateOfHealth,
    Status,
    EnergyScaleFactor,
    PercentScaleFactor,
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
            buffer::write_u16(713, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(7, buffer);
        }
        Point::EnergyRating => {
            if let Some(value) = model.energy_rating() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnergyAvailable => {
            if let Some(value) = model.energy_available() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StateOfCharge => {
            if let Some(value) = model.state_of_charge() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StateOfHealth => {
            if let Some(value) = model.state_of_health() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Status => {
            if let Some(value) = model.status() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnergyScaleFactor => {
            if let Some(value) = model.energy_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PercentScaleFactor => {
            if let Some(value) = model.percent_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Energy Rating
    ///
    /// Energy rating of the DER storage.
    fn energy_rating(&self) -> Option<u16> {
        None
    }

    /// Energy Available
    ///
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    fn energy_available(&self) -> Option<u16> {
        None
    }

    /// State of Charge
    ///
    /// State of charge of the DER storage.
    ///
    /// SOC shall be fixed to 0% for DER without storage capabilities.
    fn state_of_charge(&self) -> Option<u16> {
        None
    }

    /// State of Health
    ///
    /// State of health of the DER storage.
    fn state_of_health(&self) -> Option<u16> {
        None
    }

    /// Status
    ///
    /// Storage status.
    fn status(&self) -> Option<Sta> {
        None
    }

    /// Energy Scale Factor
    ///
    /// Scale factor for energy capacity.
    fn energy_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Percent Scale Factor
    ///
    /// Scale factor for percentage.
    fn percent_scale_factor(&self) -> Option<u16> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Sta {
    /// OK
    ///
    /// No warnings or errors pending.
    Ok = 0,
    /// Warning
    ///
    /// One or more warnings pending.
    Warning = 1,
    /// Error
    ///
    /// One or more errors pending.
    Error = 2,
}

#[repr(C)]
pub struct Model713CallbackAdapter {
    context: *mut c_void,
    energy_rating_callback: Option<extern "C" fn(*const c_void) -> u16>,
    energy_available_callback: Option<extern "C" fn(*const c_void) -> u16>,
    state_of_charge_callback: Option<extern "C" fn(*const c_void) -> u16>,
    state_of_health_callback: Option<extern "C" fn(*const c_void) -> u16>,
    status_callback: Option<extern "C" fn(*const c_void) -> Sta>,
    energy_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    percent_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model713CallbackAdapter {
    /// Energy Rating
    ///
    /// Energy rating of the DER storage.
    fn energy_rating(&self) -> Option<u16> {
        self.energy_rating_callback
            .map(|callback| (callback)(self.context))
    }

    /// Energy Available
    ///
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    fn energy_available(&self) -> Option<u16> {
        self.energy_available_callback
            .map(|callback| (callback)(self.context))
    }

    /// State of Charge
    ///
    /// State of charge of the DER storage.
    ///
    /// SOC shall be fixed to 0% for DER without storage capabilities.
    fn state_of_charge(&self) -> Option<u16> {
        self.state_of_charge_callback
            .map(|callback| (callback)(self.context))
    }

    /// State of Health
    ///
    /// State of health of the DER storage.
    fn state_of_health(&self) -> Option<u16> {
        self.state_of_health_callback
            .map(|callback| (callback)(self.context))
    }

    /// Status
    ///
    /// Storage status.
    fn status(&self) -> Option<Sta> {
        self.status_callback
            .map(|callback| (callback)(self.context))
    }

    /// Energy Scale Factor
    ///
    /// Scale factor for energy capacity.
    fn energy_scale_factor(&self) -> Option<u16> {
        self.energy_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Percent Scale Factor
    ///
    /// Scale factor for percentage.
    fn percent_scale_factor(&self) -> Option<u16> {
        self.percent_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model713StatefulAdapter {
    energy_rating: u16,
    energy_available: u16,
    state_of_charge: u16,
    state_of_health: u16,
    status: Sta,
    energy_scale_factor: u16,
    percent_scale_factor: u16,
}

impl ModelAdapter for Model713StatefulAdapter {
    /// Energy Rating
    ///
    /// Energy rating of the DER storage.
    fn energy_rating(&self) -> Option<u16> {
        Some(self.energy_rating)
    }

    /// Energy Available
    ///
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    fn energy_available(&self) -> Option<u16> {
        Some(self.energy_available)
    }

    /// State of Charge
    ///
    /// State of charge of the DER storage.
    ///
    /// SOC shall be fixed to 0% for DER without storage capabilities.
    fn state_of_charge(&self) -> Option<u16> {
        Some(self.state_of_charge)
    }

    /// State of Health
    ///
    /// State of health of the DER storage.
    fn state_of_health(&self) -> Option<u16> {
        Some(self.state_of_health)
    }

    /// Status
    ///
    /// Storage status.
    fn status(&self) -> Option<Sta> {
        Some(self.status)
    }

    /// Energy Scale Factor
    ///
    /// Scale factor for energy capacity.
    fn energy_scale_factor(&self) -> Option<u16> {
        Some(self.energy_scale_factor)
    }

    /// Percent Scale Factor
    ///
    /// Scale factor for percentage.
    fn percent_scale_factor(&self) -> Option<u16> {
        Some(self.percent_scale_factor)
    }
}
