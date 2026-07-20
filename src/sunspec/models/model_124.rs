use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 26;

pub static POINTS: [ReadablePoint; 26] = [
    ReadablePoint {
        reference: PointReference::Static { value: 124 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 24 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::WChaMax },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::WChaGra },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::WDisChaGra },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::StorCtlMod },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::VaChaMax },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::MinRsvPct },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::ChaState },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::StorAval },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::InBatV },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::ChaSt },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::OutWRte },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::InWRte },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::InOutWRteWinTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::InOutWRteRvrtTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::InOutWRteRmpTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::ChaGriSet },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::WChaMaxSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::WChaDisChaGraSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::VaChaMaxSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::MinRsvPctSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::ChaStateSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::StorAvalSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::InBatVSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model124 { point: Point::InOutWRteSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    WChaMax,
    WChaGra,
    WDisChaGra,
    StorCtlMod,
    VaChaMax,
    MinRsvPct,
    ChaState,
    StorAval,
    InBatV,
    ChaSt,
    OutWRte,
    InWRte,
    InOutWRteWinTms,
    InOutWRteRvrtTms,
    InOutWRteRmpTms,
    ChaGriSet,
    WChaMaxSf,
    WChaDisChaGraSf,
    VaChaMaxSf,
    MinRsvPctSf,
    ChaStateSf,
    StorAvalSf,
    InBatVSf,
    InOutWRteSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::WChaMax => serialisation::write_u16(model.w_cha_max(), buffer),
        Point::WChaGra => serialisation::write_u16(model.w_cha_gra(), buffer),
        Point::WDisChaGra => serialisation::write_u16(model.w_dis_cha_gra(), buffer),
        Point::StorCtlMod => serialisation::write_u16(model.stor_ctl_mod(), buffer),
        Point::VaChaMax => if let Some(value) = model.va_cha_max() { serialisation::write_u16(value, buffer); },
        Point::MinRsvPct => if let Some(value) = model.min_rsv_pct() { serialisation::write_u16(value, buffer); },
        Point::ChaState => if let Some(value) = model.cha_state() { serialisation::write_u16(value, buffer); },
        Point::StorAval => if let Some(value) = model.stor_aval() { serialisation::write_u16(value, buffer); },
        Point::InBatV => if let Some(value) = model.in_bat_v() { serialisation::write_u16(value, buffer); },
        Point::ChaSt => if let Some(value) = model.cha_st() { serialisation::write_u16(value as u16, buffer); },
        Point::OutWRte => if let Some(value) = model.out_w_rte() { serialisation::write_i16(value, buffer); },
        Point::InWRte => if let Some(value) = model.in_w_rte() { serialisation::write_i16(value, buffer); },
        Point::InOutWRteWinTms => if let Some(value) = model.in_out_w_rte_win_tms() { serialisation::write_u16(value, buffer); },
        Point::InOutWRteRvrtTms => if let Some(value) = model.in_out_w_rte_rvrt_tms() { serialisation::write_u16(value, buffer); },
        Point::InOutWRteRmpTms => if let Some(value) = model.in_out_w_rte_rmp_tms() { serialisation::write_u16(value, buffer); },
        Point::ChaGriSet => if let Some(value) = model.cha_gri_set() { serialisation::write_u16(value as u16, buffer); },
        Point::WChaMaxSf => serialisation::write_u16(model.w_cha_max_sf(), buffer),
        Point::WChaDisChaGraSf => serialisation::write_u16(model.w_cha_dis_cha_gra_sf(), buffer),
        Point::VaChaMaxSf => if let Some(value) = model.va_cha_max_sf() { serialisation::write_u16(value, buffer); },
        Point::MinRsvPctSf => if let Some(value) = model.min_rsv_pct_sf() { serialisation::write_u16(value, buffer); },
        Point::ChaStateSf => if let Some(value) = model.cha_state_sf() { serialisation::write_u16(value, buffer); },
        Point::StorAvalSf => if let Some(value) = model.stor_aval_sf() { serialisation::write_u16(value, buffer); },
        Point::InBatVSf => if let Some(value) = model.in_bat_v_sf() { serialisation::write_u16(value, buffer); },
        Point::InOutWRteSf => if let Some(value) = model.in_out_w_rte_sf() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    fn w_cha_max(&self) -> u16;

    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    fn set_w_cha_max(&mut self, value: u16);

    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    fn w_cha_gra(&self) -> u16;

    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    fn set_w_cha_gra(&mut self, value: u16);

    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    fn w_dis_cha_gra(&self) -> u16;

    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    fn set_w_dis_cha_gra(&mut self, value: u16);

    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode.
    fn stor_ctl_mod(&self) -> u16;

    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode.
    fn set_stor_ctl_mod(&mut self, value: u16);

    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    fn va_cha_max(&self) -> Option<u16> {
        None
    }

    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    fn set_va_cha_max(&mut self, value: u16) {
    }

    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn min_rsv_pct(&self) -> Option<u16> {
        None
    }

    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_min_rsv_pct(&mut self, value: u16) {
    }

    /// ChaState
    ///
    /// Currently available energy as a percent of the capacity rating.
    fn cha_state(&self) -> Option<u16> {
        None
    }

    /// StorAval
    ///
    /// State of charge (ChaState) minus storage reserve (MinRsvPct) times capacity rating (AhrRtg).
    fn stor_aval(&self) -> Option<u16> {
        None
    }

    /// InBatV
    ///
    /// Internal battery voltage.
    fn in_bat_v(&self) -> Option<u16> {
        None
    }

    /// ChaSt
    ///
    /// Charge status of storage device.
    fn cha_st(&self) -> Option<ChaSt> {
        None
    }

    /// OutWRte
    ///
    /// Percent of max discharge rate.
    fn out_w_rte(&self) -> Option<i16> {
        None
    }

    /// OutWRte
    ///
    /// Percent of max discharge rate.
    fn set_out_w_rte(&mut self, value: i16) {
    }

    /// InWRte
    ///
    /// Percent of max charging rate.
    fn in_w_rte(&self) -> Option<i16> {
        None
    }

    /// InWRte
    ///
    /// Percent of max charging rate.
    fn set_in_w_rte(&mut self, value: i16) {
    }

    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    fn in_out_w_rte_win_tms(&self) -> Option<u16> {
        None
    }

    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    fn set_in_out_w_rte_win_tms(&mut self, value: u16) {
    }

    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    fn in_out_w_rte_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    fn set_in_out_w_rte_rvrt_tms(&mut self, value: u16) {
    }

    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn in_out_w_rte_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_in_out_w_rte_rmp_tms(&mut self, value: u16) {
    }

    fn cha_gri_set(&self) -> Option<ChaGriSet> {
        None
    }

    fn set_cha_gri_set(&mut self, value: ChaGriSet) {
    }

    /// WChaMax_SF
    ///
    /// Scale factor for maximum charge.
    fn w_cha_max_sf(&self) -> u16;

    /// WChaDisChaGra_SF
    ///
    /// Scale factor for maximum charge and discharge rate.
    fn w_cha_dis_cha_gra_sf(&self) -> u16;

    /// VAChaMax_SF
    ///
    /// Scale factor for maximum charging VA.
    fn va_cha_max_sf(&self) -> Option<u16> {
        None
    }

    /// MinRsvPct_SF
    ///
    /// Scale factor for minimum reserve percentage.
    fn min_rsv_pct_sf(&self) -> Option<u16> {
        None
    }

    /// ChaState_SF
    ///
    /// Scale factor for available energy percent.
    fn cha_state_sf(&self) -> Option<u16> {
        None
    }

    /// StorAval_SF
    ///
    /// Scale factor for state of charge.
    fn stor_aval_sf(&self) -> Option<u16> {
        None
    }

    /// InBatV_SF
    ///
    /// Scale factor for battery voltage.
    fn in_bat_v_sf(&self) -> Option<u16> {
        None
    }

    /// InOutWRte_SF
    ///
    /// Scale factor for percent charge/discharge rate.
    fn in_out_w_rte_sf(&self) -> Option<u16> {
        None
    }
}

pub enum ChaSt {
    Off = 1,
    Empty = 2,
    Discharging = 3,
    Charging = 4,
    Full = 5,
    Holding = 6,
    Testing = 7,
}

pub enum ChaGriSet {
    Pv = 0,
    Grid = 1,
}