use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 12;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 126 },
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
        reference: PointReference::Model126 { point: Point::ActCrv },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::ModEna },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::WinTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::RvrtTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::RmpTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::NCrv },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::NPt },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::DeptRefSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::RmpIncDecSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ActCrv,
    ModEna,
    WinTms,
    RvrtTms,
    RmpTms,
    NCrv,
    NPt,
    VSf,
    DeptRefSf,
    RmpIncDecSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::ActCrv => serialisation::write_u16(model.act_crv(), buffer),
        Point::ModEna => serialisation::write_u16(model.mod_ena(), buffer),
        Point::WinTms => if let Some(value) = model.win_tms() { serialisation::write_u16(value, buffer); },
        Point::RvrtTms => if let Some(value) = model.rvrt_tms() { serialisation::write_u16(value, buffer); },
        Point::RmpTms => if let Some(value) = model.rmp_tms() { serialisation::write_u16(value, buffer); },
        Point::NCrv => serialisation::write_u16(model.n_crv(), buffer),
        Point::NPt => serialisation::write_u16(model.n_pt(), buffer),
        Point::VSf => serialisation::write_u16(model.v_sf(), buffer),
        Point::DeptRefSf => serialisation::write_u16(model.dept_ref_sf(), buffer),
        Point::RmpIncDecSf => if let Some(value) = model.rmp_inc_dec_sf() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn act_crv(&self) -> u16;

    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn set_act_crv(&mut self, value: u16);

    /// ModEna
    ///
    /// Is Volt-VAR control active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is Volt-VAR control active.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for volt-VAR change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for volt-VAR change.
    fn set_win_tms(&mut self, value: u16) {
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    fn set_rvrt_tms(&mut self, value: u16) {
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_rmp_tms(&mut self, value: u16) {
    }

    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16;

    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16;

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16;

    /// DeptRef_SF
    ///
    /// Scale factor for dependent variable.
    fn dept_ref_sf(&self) -> u16;

    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        None
    }
}