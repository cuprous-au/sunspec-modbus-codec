use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 28;

pub static POINTS: [ReadablePoint; 28] = [
    ReadablePoint {
        reference: PointReference::Static { value: 120 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 26 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::DerTyp },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::WRtg },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::WRtgSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::VaRtg },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::VaRtgSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::VArRtgQ1 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::VArRtgQ2 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::VArRtgQ3 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::VArRtgQ4 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::VArRtgSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::ARtg },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::ARtgSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::PfRtgQ1 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::PfRtgQ2 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::PfRtgQ3 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::PfRtgQ4 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::PfRtgSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::WhRtg },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::WhRtgSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::AhrRtg },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::AhrRtgSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::MaxChaRte },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::MaxChaRteSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::MaxDisChaRte },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::MaxDisChaRteSf },
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
    DerTyp,
    WRtg,
    WRtgSf,
    VaRtg,
    VaRtgSf,
    VArRtgQ1,
    VArRtgQ2,
    VArRtgQ3,
    VArRtgQ4,
    VArRtgSf,
    ARtg,
    ARtgSf,
    PfRtgQ1,
    PfRtgQ2,
    PfRtgQ3,
    PfRtgQ4,
    PfRtgSf,
    WhRtg,
    WhRtgSf,
    AhrRtg,
    AhrRtgSf,
    MaxChaRte,
    MaxChaRteSf,
    MaxDisChaRte,
    MaxDisChaRteSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::DerTyp => serialisation::write_u16(model.der_typ() as u16, buffer),
        Point::WRtg => serialisation::write_u16(model.w_rtg(), buffer),
        Point::WRtgSf => serialisation::write_u16(model.w_rtg_sf(), buffer),
        Point::VaRtg => serialisation::write_u16(model.va_rtg(), buffer),
        Point::VaRtgSf => serialisation::write_u16(model.va_rtg_sf(), buffer),
        Point::VArRtgQ1 => serialisation::write_i16(model.v_ar_rtg_q1(), buffer),
        Point::VArRtgQ2 => serialisation::write_i16(model.v_ar_rtg_q2(), buffer),
        Point::VArRtgQ3 => serialisation::write_i16(model.v_ar_rtg_q3(), buffer),
        Point::VArRtgQ4 => serialisation::write_i16(model.v_ar_rtg_q4(), buffer),
        Point::VArRtgSf => serialisation::write_u16(model.v_ar_rtg_sf(), buffer),
        Point::ARtg => serialisation::write_u16(model.a_rtg(), buffer),
        Point::ARtgSf => serialisation::write_u16(model.a_rtg_sf(), buffer),
        Point::PfRtgQ1 => serialisation::write_i16(model.pf_rtg_q1(), buffer),
        Point::PfRtgQ2 => serialisation::write_i16(model.pf_rtg_q2(), buffer),
        Point::PfRtgQ3 => serialisation::write_i16(model.pf_rtg_q3(), buffer),
        Point::PfRtgQ4 => serialisation::write_i16(model.pf_rtg_q4(), buffer),
        Point::PfRtgSf => serialisation::write_u16(model.pf_rtg_sf(), buffer),
        Point::WhRtg => if let Some(value) = model.wh_rtg() { serialisation::write_u16(value, buffer); },
        Point::WhRtgSf => if let Some(value) = model.wh_rtg_sf() { serialisation::write_u16(value, buffer); },
        Point::AhrRtg => if let Some(value) = model.ahr_rtg() { serialisation::write_u16(value, buffer); },
        Point::AhrRtgSf => if let Some(value) = model.ahr_rtg_sf() { serialisation::write_u16(value, buffer); },
        Point::MaxChaRte => if let Some(value) = model.max_cha_rte() { serialisation::write_u16(value, buffer); },
        Point::MaxChaRteSf => if let Some(value) = model.max_cha_rte_sf() { serialisation::write_u16(value, buffer); },
        Point::MaxDisChaRte => if let Some(value) = model.max_dis_cha_rte() { serialisation::write_u16(value, buffer); },
        Point::MaxDisChaRteSf => if let Some(value) = model.max_dis_cha_rte_sf() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    fn der_typ(&self) -> DerTyp;

    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    fn w_rtg(&self) -> u16;

    /// WRtg_SF
    ///
    /// Scale factor
    fn w_rtg_sf(&self) -> u16;

    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    fn va_rtg(&self) -> u16;

    /// VARtg_SF
    ///
    /// Scale factor
    fn va_rtg_sf(&self) -> u16;

    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    fn v_ar_rtg_q1(&self) -> i16;

    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    fn v_ar_rtg_q2(&self) -> i16;

    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    fn v_ar_rtg_q3(&self) -> i16;

    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    fn v_ar_rtg_q4(&self) -> i16;

    /// VArRtg_SF
    ///
    /// Scale factor
    fn v_ar_rtg_sf(&self) -> u16;

    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    fn a_rtg(&self) -> u16;

    /// ARtg_SF
    ///
    /// Scale factor
    fn a_rtg_sf(&self) -> u16;

    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// EEI sign convention.
    fn pf_rtg_q1(&self) -> i16;

    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// EEI sign convention.
    fn pf_rtg_q2(&self) -> i16;

    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// EEI sign convention.
    fn pf_rtg_q3(&self) -> i16;

    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// EEI sign convention.
    fn pf_rtg_q4(&self) -> i16;

    /// PFRtg_SF
    ///
    /// Scale factor
    fn pf_rtg_sf(&self) -> u16;

    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    fn wh_rtg(&self) -> Option<u16> {
        None
    }

    /// WHRtg_SF
    ///
    /// Scale factor
    fn wh_rtg_sf(&self) -> Option<u16> {
        None
    }

    /// AhrRtg
    ///
    /// The usable capacity of the battery. Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    fn ahr_rtg(&self) -> Option<u16> {
        None
    }

    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    fn ahr_rtg_sf(&self) -> Option<u16> {
        None
    }

    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    fn max_cha_rte(&self) -> Option<u16> {
        None
    }

    /// MaxChaRte_SF
    ///
    /// Scale factor
    fn max_cha_rte_sf(&self) -> Option<u16> {
        None
    }

    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    fn max_dis_cha_rte(&self) -> Option<u16> {
        None
    }

    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    fn max_dis_cha_rte_sf(&self) -> Option<u16> {
        None
    }
}

pub enum DerTyp {
    Pv = 4,
    PvStor = 82,
}