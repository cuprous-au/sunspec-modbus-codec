use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 9;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 713 },
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
        reference: PointReference::Model713 {
            point: Point::EnergyRating,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 {
            point: Point::EnergyAvailable,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 {
            point: Point::StateOfCharge,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 {
            point: Point::StateOfHealth,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 {
            point: Point::Status,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 {
            point: Point::EnergyScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 {
            point: Point::PercentScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    EnergyRating,
    EnergyAvailable,
    StateOfCharge,
    StateOfHealth,
    Status,
    EnergyScaleFactor,
    PercentScaleFactor,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    9
}

pub fn write_point<'a, 'b>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) {
    match point {
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
