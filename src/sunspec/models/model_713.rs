use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

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
        reference: PointReference::Model713 { point: Point::EnergyRating },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 { point: Point::EnergyAvailable },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 { point: Point::StateOfCharge },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 { point: Point::StateOfHealth },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 { point: Point::Status },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 { point: Point::EnergyScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model713 { point: Point::PercentScaleFactor },
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::EnergyRating => if let Some(value) = model.energy_rating() { serialisation::write_u16(value, buffer); },
        Point::EnergyAvailable => if let Some(value) = model.energy_available() { serialisation::write_u16(value, buffer); },
        Point::StateOfCharge => if let Some(value) = model.state_of_charge() { serialisation::write_u16(value, buffer); },
        Point::StateOfHealth => if let Some(value) = model.state_of_health() { serialisation::write_u16(value, buffer); },
        Point::Status => if let Some(value) = model.status() { serialisation::write_u16(value as u16, buffer); },
        Point::EnergyScaleFactor => if let Some(value) = model.energy_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::PercentScaleFactor => if let Some(value) = model.percent_scale_factor() { serialisation::write_u16(value, buffer); },
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