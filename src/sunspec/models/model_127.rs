use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 12;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 127 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 10 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::WGra },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::HzStr },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::HzStop },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::HysEna },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::ModEna },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::HzStopWGra },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::WGraSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::HzStrStopSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model127 { point: Point::RmpIncDecSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    WGra,
    HzStr,
    HzStop,
    HysEna,
    ModEna,
    HzStopWGra,
    WGraSf,
    HzStrStopSf,
    RmpIncDecSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::WGra => serialisation::write_u16(model.w_gra(), buffer),
        Point::HzStr => serialisation::write_i16(model.hz_str(), buffer),
        Point::HzStop => serialisation::write_i16(model.hz_stop(), buffer),
        Point::HysEna => serialisation::write_u16(model.hys_ena(), buffer),
        Point::ModEna => serialisation::write_u16(model.mod_ena(), buffer),
        Point::HzStopWGra => if let Some(value) = model.hz_stop_w_gra() { serialisation::write_u16(value, buffer); },
        Point::WGraSf => if let Some(value) = model.w_gra_sf() { serialisation::write_u16(value, buffer); },
        Point::HzStrStopSf => if let Some(value) = model.hz_str_stop_sf() { serialisation::write_u16(value, buffer); },
        Point::RmpIncDecSf => if let Some(value) = model.rmp_inc_dec_sf() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn w_gra(&self) -> u16;

    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn set_w_gra(&mut self, value: u16);

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn hz_str(&self) -> i16;

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn set_hz_str(&mut self, value: i16);

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn hz_stop(&self) -> i16;

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn set_hz_stop(&mut self, value: i16);

    /// HysEna
    ///
    /// Enable hysteresis
    fn hys_ena(&self) -> u16;

    /// HysEna
    ///
    /// Enable hysteresis
    fn set_hys_ena(&mut self, value: u16);

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16);

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn hz_stop_w_gra(&self) -> Option<u16> {
        None
    }

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn set_hz_stop_w_gra(&mut self, value: u16) {
    }

    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    fn w_gra_sf(&self) -> Option<u16> {
        None
    }

    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    fn hz_str_stop_sf(&self) -> Option<u16> {
        None
    }

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        None
    }
}