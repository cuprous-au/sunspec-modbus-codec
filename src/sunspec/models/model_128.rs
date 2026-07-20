use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 16;

pub static POINTS: [ReadablePoint; 16] = [
    ReadablePoint {
        reference: PointReference::Static { value: 128 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 14 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ArGraMod },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ArGraSag },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ArGraSwell },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ModEna },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::FilTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::DbVMin },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::DbVMax },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::BlkZnV },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::HysBlkZnV },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::BlkZnTmms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::HoldTmms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ArGraSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::VRefPctSf },
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
    ArGraMod,
    ArGraSag,
    ArGraSwell,
    ModEna,
    FilTms,
    DbVMin,
    DbVMax,
    BlkZnV,
    HysBlkZnV,
    BlkZnTmms,
    HoldTmms,
    ArGraSf,
    VRefPctSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::ArGraMod => serialisation::write_u16(model.ar_gra_mod() as u16, buffer),
        Point::ArGraSag => serialisation::write_u16(model.ar_gra_sag(), buffer),
        Point::ArGraSwell => serialisation::write_u16(model.ar_gra_swell(), buffer),
        Point::ModEna => serialisation::write_u16(model.mod_ena(), buffer),
        Point::FilTms => if let Some(value) = model.fil_tms() { serialisation::write_u16(value, buffer); },
        Point::DbVMin => if let Some(value) = model.db_v_min() { serialisation::write_u16(value, buffer); },
        Point::DbVMax => if let Some(value) = model.db_v_max() { serialisation::write_u16(value, buffer); },
        Point::BlkZnV => if let Some(value) = model.blk_zn_v() { serialisation::write_u16(value, buffer); },
        Point::HysBlkZnV => if let Some(value) = model.hys_blk_zn_v() { serialisation::write_u16(value, buffer); },
        Point::BlkZnTmms => if let Some(value) = model.blk_zn_tmms() { serialisation::write_u16(value, buffer); },
        Point::HoldTmms => if let Some(value) = model.hold_tmms() { serialisation::write_u16(value, buffer); },
        Point::ArGraSf => serialisation::write_u16(model.ar_gra_sf(), buffer),
        Point::VRefPctSf => if let Some(value) = model.v_ref_pct_sf() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn ar_gra_mod(&self) -> ArGraMod;

    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn set_ar_gra_mod(&mut self, value: ArGraMod);

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_sag(&self) -> u16;

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_sag(&mut self, value: u16);

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_swell(&self) -> u16;

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_swell(&mut self, value: u16);

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn set_mod_ena(&mut self, value: u16);

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn fil_tms(&self) -> Option<u16> {
        None
    }

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn set_fil_tms(&mut self, value: u16) {
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn db_v_min(&self) -> Option<u16> {
        None
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn set_db_v_min(&mut self, value: u16) {
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn db_v_max(&self) -> Option<u16> {
        None
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn set_db_v_max(&mut self, value: u16) {
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn blk_zn_v(&self) -> Option<u16> {
        None
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn set_blk_zn_v(&mut self, value: u16) {
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn hys_blk_zn_v(&self) -> Option<u16> {
        None
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn set_hys_blk_zn_v(&mut self, value: u16) {
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn blk_zn_tmms(&self) -> Option<u16> {
        None
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn set_blk_zn_tmms(&mut self, value: u16) {
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn hold_tmms(&self) -> Option<u16> {
        None
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn set_hold_tmms(&mut self, value: u16) {
    }

    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    fn ar_gra_sf(&self) -> u16;

    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    fn v_ref_pct_sf(&self) -> Option<u16> {
        None
    }
}

pub enum ArGraMod {
    Edge = 0,
    Center = 1,
}