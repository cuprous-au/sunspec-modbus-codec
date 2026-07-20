use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 10;

pub static POINTS: [ReadablePoint; 10] = [
    ReadablePoint {
        reference: PointReference::Static { value: 125 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model125 { point: Point::ModEna },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model125 { point: Point::SigType },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model125 { point: Point::Sig },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model125 { point: Point::WinTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model125 { point: Point::RvtTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model125 { point: Point::RmpTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model125 { point: Point::SigSf },
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
    ModEna,
    SigType,
    Sig,
    WinTms,
    RvtTms,
    RmpTms,
    SigSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::ModEna => serialisation::write_u16(model.mod_ena(), buffer),
        Point::SigType => if let Some(value) = model.sig_type() { serialisation::write_u16(value as u16, buffer); },
        Point::Sig => serialisation::write_i16(model.sig(), buffer),
        Point::WinTms => if let Some(value) = model.win_tms() { serialisation::write_u16(value, buffer); },
        Point::RvtTms => if let Some(value) = model.rvt_tms() { serialisation::write_u16(value, buffer); },
        Point::RmpTms => if let Some(value) = model.rmp_tms() { serialisation::write_u16(value, buffer); },
        Point::SigSf => serialisation::write_u16(model.sig_sf(), buffer),
    }
}

pub trait ModelAdapter {
    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn set_mod_ena(&mut self, value: u16);

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn sig_type(&self) -> Option<SigType> {
        None
    }

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn set_sig_type(&mut self, value: SigType) {
    }

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn sig(&self) -> i16;

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn set_sig(&mut self, value: i16);

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn set_win_tms(&mut self, value: u16) {
    }

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn rvt_tms(&self) -> Option<u16> {
        None
    }

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn set_rvt_tms(&mut self, value: u16) {
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn set_rmp_tms(&mut self, value: u16) {
    }

    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    fn sig_sf(&self) -> u16;
}

pub enum SigType {
    Unknown = 0,
    Absolute = 1,
    Relative = 2,
    Multiplier = 3,
    Level = 4,
}