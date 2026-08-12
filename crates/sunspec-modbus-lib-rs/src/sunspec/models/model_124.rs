use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 26;

static POINTS: [PointDetails<()>; 26] = [
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
        point: |()| Point::WChaMax,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::WChaGra,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::WDisChaGra,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::StorCtlMod,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::VaChaMax,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::MinRsvPct,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::ChaState,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::StorAval,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::InBatV,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::ChaSt,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::OutWRte,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::InWRte,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::InOutWRteWinTms,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::InOutWRteRvrtTms,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::InOutWRteRmpTms,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::ChaGriSet,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::WChaMaxSf,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::WChaDisChaGraSf,
        size: 1,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::VaChaMaxSf,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::MinRsvPctSf,
        size: 1,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::ChaStateSf,
        size: 1,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::StorAvalSf,
        size: 1,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::InBatVSf,
        size: 1,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::InOutWRteSf,
        size: 1,
        start_address: 25,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
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

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    26
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
            buffer::write_u16(124, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::WChaMax => {
            buffer::write_u16(model.w_cha_max(), buffer);
        }
        Point::WChaGra => {
            buffer::write_u16(model.w_cha_gra(), buffer);
        }
        Point::WDisChaGra => {
            buffer::write_u16(model.w_dis_cha_gra(), buffer);
        }
        Point::StorCtlMod => {
            buffer::write_u16(model.stor_ctl_mod(), buffer);
        }
        Point::VaChaMax => {
            if let Some(value) = model.va_cha_max() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MinRsvPct => {
            if let Some(value) = model.min_rsv_pct() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ChaState => {
            if let Some(value) = model.cha_state() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::StorAval => {
            if let Some(value) = model.stor_aval() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InBatV => {
            if let Some(value) = model.in_bat_v() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ChaSt => {
            if let Some(value) = model.cha_st() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::OutWRte => {
            if let Some(value) = model.out_w_rte() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InWRte => {
            if let Some(value) = model.in_w_rte() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InOutWRteWinTms => {
            if let Some(value) = model.in_out_w_rte_win_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InOutWRteRvrtTms => {
            if let Some(value) = model.in_out_w_rte_rvrt_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InOutWRteRmpTms => {
            if let Some(value) = model.in_out_w_rte_rmp_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ChaGriSet => {
            if let Some(value) = model.cha_gri_set() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::WChaMaxSf => {
            buffer::write_u16(model.w_cha_max_sf(), buffer);
        }
        Point::WChaDisChaGraSf => {
            buffer::write_u16(model.w_cha_dis_cha_gra_sf(), buffer);
        }
        Point::VaChaMaxSf => {
            if let Some(value) = model.va_cha_max_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MinRsvPctSf => {
            if let Some(value) = model.min_rsv_pct_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ChaStateSf => {
            if let Some(value) = model.cha_state_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::StorAvalSf => {
            if let Some(value) = model.stor_aval_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InBatVSf => {
            if let Some(value) = model.in_bat_v_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InOutWRteSf => {
            if let Some(value) = model.in_out_w_rte_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
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
    fn set_va_cha_max(&mut self, value: u16) {}

    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn min_rsv_pct(&self) -> Option<u16> {
        None
    }

    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_min_rsv_pct(&mut self, value: u16) {}

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
    fn set_out_w_rte(&mut self, value: i16) {}

    /// InWRte
    ///
    /// Percent of max charging rate.
    fn in_w_rte(&self) -> Option<i16> {
        None
    }

    /// InWRte
    ///
    /// Percent of max charging rate.
    fn set_in_w_rte(&mut self, value: i16) {}

    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    fn in_out_w_rte_win_tms(&self) -> Option<u16> {
        None
    }

    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    fn set_in_out_w_rte_win_tms(&mut self, value: u16) {}

    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    fn in_out_w_rte_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    fn set_in_out_w_rte_rvrt_tms(&mut self, value: u16) {}

    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn in_out_w_rte_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_in_out_w_rte_rmp_tms(&mut self, value: u16) {}

    fn cha_gri_set(&self) -> Option<ChaGriSet> {
        None
    }

    fn set_cha_gri_set(&mut self, value: ChaGriSet) {}

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

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ChaGriSet {
    Pv = 0,
    Grid = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ChaSt {
    Off = 1,
    Empty = 2,
    Discharging = 3,
    Charging = 4,
    Full = 5,
    Holding = 6,
    Testing = 7,
}

#[repr(C)]
pub struct Model124CallbackAdapter {
    context: *mut c_void,
    w_cha_max_callback: extern "C" fn(*const c_void) -> u16,
    set_w_cha_max_callback: extern "C" fn(u16, *mut c_void),
    w_cha_gra_callback: extern "C" fn(*const c_void) -> u16,
    set_w_cha_gra_callback: extern "C" fn(u16, *mut c_void),
    w_dis_cha_gra_callback: extern "C" fn(*const c_void) -> u16,
    set_w_dis_cha_gra_callback: extern "C" fn(u16, *mut c_void),
    stor_ctl_mod_callback: extern "C" fn(*const c_void) -> u16,
    set_stor_ctl_mod_callback: extern "C" fn(u16, *mut c_void),
    va_cha_max_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_va_cha_max_callback: Option<extern "C" fn(u16, *mut c_void)>,
    min_rsv_pct_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_min_rsv_pct_callback: Option<extern "C" fn(u16, *mut c_void)>,
    cha_state_callback: Option<extern "C" fn(*const c_void) -> u16>,
    stor_aval_callback: Option<extern "C" fn(*const c_void) -> u16>,
    in_bat_v_callback: Option<extern "C" fn(*const c_void) -> u16>,
    cha_st_callback: Option<extern "C" fn(*const c_void) -> ChaSt>,
    out_w_rte_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_out_w_rte_callback: Option<extern "C" fn(i16, *mut c_void)>,
    in_w_rte_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_in_w_rte_callback: Option<extern "C" fn(i16, *mut c_void)>,
    in_out_w_rte_win_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_in_out_w_rte_win_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    in_out_w_rte_rvrt_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_in_out_w_rte_rvrt_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    in_out_w_rte_rmp_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_in_out_w_rte_rmp_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    cha_gri_set_callback: Option<extern "C" fn(*const c_void) -> ChaGriSet>,
    set_cha_gri_set_callback: Option<extern "C" fn(ChaGriSet, *mut c_void)>,
    w_cha_max_sf_callback: extern "C" fn(*const c_void) -> u16,
    w_cha_dis_cha_gra_sf_callback: extern "C" fn(*const c_void) -> u16,
    va_cha_max_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_rsv_pct_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    cha_state_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    stor_aval_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    in_bat_v_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    in_out_w_rte_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model124CallbackAdapter {
    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    fn w_cha_max(&self) -> u16 {
        (self.w_cha_max_callback)(self.context)
    }

    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    fn set_w_cha_max(&mut self, value: u16) {
        (self.set_w_cha_max_callback)(value, self.context);
    }

    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    fn w_cha_gra(&self) -> u16 {
        (self.w_cha_gra_callback)(self.context)
    }

    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    fn set_w_cha_gra(&mut self, value: u16) {
        (self.set_w_cha_gra_callback)(value, self.context);
    }

    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    fn w_dis_cha_gra(&self) -> u16 {
        (self.w_dis_cha_gra_callback)(self.context)
    }

    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    fn set_w_dis_cha_gra(&mut self, value: u16) {
        (self.set_w_dis_cha_gra_callback)(value, self.context);
    }

    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode.
    fn stor_ctl_mod(&self) -> u16 {
        (self.stor_ctl_mod_callback)(self.context)
    }

    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode.
    fn set_stor_ctl_mod(&mut self, value: u16) {
        (self.set_stor_ctl_mod_callback)(value, self.context);
    }

    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    fn va_cha_max(&self) -> Option<u16> {
        self.va_cha_max_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    fn set_va_cha_max(&mut self, value: u16) {
        if let Some(callback) = self.set_va_cha_max_callback {
            (callback)(value, self.context);
        };
    }

    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn min_rsv_pct(&self) -> Option<u16> {
        self.min_rsv_pct_callback
            .map(|callback| (callback)(self.context))
    }

    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_min_rsv_pct(&mut self, value: u16) {
        if let Some(callback) = self.set_min_rsv_pct_callback {
            (callback)(value, self.context);
        };
    }

    /// ChaState
    ///
    /// Currently available energy as a percent of the capacity rating.
    fn cha_state(&self) -> Option<u16> {
        self.cha_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// StorAval
    ///
    /// State of charge (ChaState) minus storage reserve (MinRsvPct) times capacity rating (AhrRtg).
    fn stor_aval(&self) -> Option<u16> {
        self.stor_aval_callback
            .map(|callback| (callback)(self.context))
    }

    /// InBatV
    ///
    /// Internal battery voltage.
    fn in_bat_v(&self) -> Option<u16> {
        self.in_bat_v_callback
            .map(|callback| (callback)(self.context))
    }

    /// ChaSt
    ///
    /// Charge status of storage device.
    fn cha_st(&self) -> Option<ChaSt> {
        self.cha_st_callback
            .map(|callback| (callback)(self.context))
    }

    /// OutWRte
    ///
    /// Percent of max discharge rate.
    fn out_w_rte(&self) -> Option<i16> {
        self.out_w_rte_callback
            .map(|callback| (callback)(self.context))
    }

    /// OutWRte
    ///
    /// Percent of max discharge rate.
    fn set_out_w_rte(&mut self, value: i16) {
        if let Some(callback) = self.set_out_w_rte_callback {
            (callback)(value, self.context);
        };
    }

    /// InWRte
    ///
    /// Percent of max charging rate.
    fn in_w_rte(&self) -> Option<i16> {
        self.in_w_rte_callback
            .map(|callback| (callback)(self.context))
    }

    /// InWRte
    ///
    /// Percent of max charging rate.
    fn set_in_w_rte(&mut self, value: i16) {
        if let Some(callback) = self.set_in_w_rte_callback {
            (callback)(value, self.context);
        };
    }

    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    fn in_out_w_rte_win_tms(&self) -> Option<u16> {
        self.in_out_w_rte_win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    fn set_in_out_w_rte_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_in_out_w_rte_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    fn in_out_w_rte_rvrt_tms(&self) -> Option<u16> {
        self.in_out_w_rte_rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    fn set_in_out_w_rte_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_in_out_w_rte_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn in_out_w_rte_rmp_tms(&self) -> Option<u16> {
        self.in_out_w_rte_rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_in_out_w_rte_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_in_out_w_rte_rmp_tms_callback {
            (callback)(value, self.context);
        };
    }

    fn cha_gri_set(&self) -> Option<ChaGriSet> {
        self.cha_gri_set_callback
            .map(|callback| (callback)(self.context))
    }

    fn set_cha_gri_set(&mut self, value: ChaGriSet) {
        if let Some(callback) = self.set_cha_gri_set_callback {
            (callback)(value, self.context);
        };
    }

    /// WChaMax_SF
    ///
    /// Scale factor for maximum charge.
    fn w_cha_max_sf(&self) -> u16 {
        (self.w_cha_max_sf_callback)(self.context)
    }

    /// WChaDisChaGra_SF
    ///
    /// Scale factor for maximum charge and discharge rate.
    fn w_cha_dis_cha_gra_sf(&self) -> u16 {
        (self.w_cha_dis_cha_gra_sf_callback)(self.context)
    }

    /// VAChaMax_SF
    ///
    /// Scale factor for maximum charging VA.
    fn va_cha_max_sf(&self) -> Option<u16> {
        self.va_cha_max_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// MinRsvPct_SF
    ///
    /// Scale factor for minimum reserve percentage.
    fn min_rsv_pct_sf(&self) -> Option<u16> {
        self.min_rsv_pct_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// ChaState_SF
    ///
    /// Scale factor for available energy percent.
    fn cha_state_sf(&self) -> Option<u16> {
        self.cha_state_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// StorAval_SF
    ///
    /// Scale factor for state of charge.
    fn stor_aval_sf(&self) -> Option<u16> {
        self.stor_aval_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// InBatV_SF
    ///
    /// Scale factor for battery voltage.
    fn in_bat_v_sf(&self) -> Option<u16> {
        self.in_bat_v_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// InOutWRte_SF
    ///
    /// Scale factor for percent charge/discharge rate.
    fn in_out_w_rte_sf(&self) -> Option<u16> {
        self.in_out_w_rte_sf_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model124StatefulAdapter {
    w_cha_max: u16,
    w_cha_gra: u16,
    w_dis_cha_gra: u16,
    stor_ctl_mod: u16,
    va_cha_max: u16,
    min_rsv_pct: u16,
    cha_state: u16,
    stor_aval: u16,
    in_bat_v: u16,
    cha_st: ChaSt,
    out_w_rte: i16,
    in_w_rte: i16,
    in_out_w_rte_win_tms: u16,
    in_out_w_rte_rvrt_tms: u16,
    in_out_w_rte_rmp_tms: u16,
    cha_gri_set: ChaGriSet,
    w_cha_max_sf: u16,
    w_cha_dis_cha_gra_sf: u16,
    va_cha_max_sf: u16,
    min_rsv_pct_sf: u16,
    cha_state_sf: u16,
    stor_aval_sf: u16,
    in_bat_v_sf: u16,
    in_out_w_rte_sf: u16,
}

impl ModelAdapter for Model124StatefulAdapter {
    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    fn w_cha_max(&self) -> u16 {
        self.w_cha_max
    }

    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    fn set_w_cha_max(&mut self, value: u16) {
        self.w_cha_max = value;
    }

    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    fn w_cha_gra(&self) -> u16 {
        self.w_cha_gra
    }

    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    fn set_w_cha_gra(&mut self, value: u16) {
        self.w_cha_gra = value;
    }

    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    fn w_dis_cha_gra(&self) -> u16 {
        self.w_dis_cha_gra
    }

    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    fn set_w_dis_cha_gra(&mut self, value: u16) {
        self.w_dis_cha_gra = value;
    }

    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode.
    fn stor_ctl_mod(&self) -> u16 {
        self.stor_ctl_mod
    }

    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode.
    fn set_stor_ctl_mod(&mut self, value: u16) {
        self.stor_ctl_mod = value;
    }

    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    fn va_cha_max(&self) -> Option<u16> {
        Some(self.va_cha_max)
    }

    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    fn set_va_cha_max(&mut self, value: u16) {
        self.va_cha_max = value;
    }

    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn min_rsv_pct(&self) -> Option<u16> {
        Some(self.min_rsv_pct)
    }

    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_min_rsv_pct(&mut self, value: u16) {
        self.min_rsv_pct = value;
    }

    /// ChaState
    ///
    /// Currently available energy as a percent of the capacity rating.
    fn cha_state(&self) -> Option<u16> {
        Some(self.cha_state)
    }

    /// StorAval
    ///
    /// State of charge (ChaState) minus storage reserve (MinRsvPct) times capacity rating (AhrRtg).
    fn stor_aval(&self) -> Option<u16> {
        Some(self.stor_aval)
    }

    /// InBatV
    ///
    /// Internal battery voltage.
    fn in_bat_v(&self) -> Option<u16> {
        Some(self.in_bat_v)
    }

    /// ChaSt
    ///
    /// Charge status of storage device.
    fn cha_st(&self) -> Option<ChaSt> {
        Some(self.cha_st)
    }

    /// OutWRte
    ///
    /// Percent of max discharge rate.
    fn out_w_rte(&self) -> Option<i16> {
        Some(self.out_w_rte)
    }

    /// OutWRte
    ///
    /// Percent of max discharge rate.
    fn set_out_w_rte(&mut self, value: i16) {
        self.out_w_rte = value;
    }

    /// InWRte
    ///
    /// Percent of max charging rate.
    fn in_w_rte(&self) -> Option<i16> {
        Some(self.in_w_rte)
    }

    /// InWRte
    ///
    /// Percent of max charging rate.
    fn set_in_w_rte(&mut self, value: i16) {
        self.in_w_rte = value;
    }

    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    fn in_out_w_rte_win_tms(&self) -> Option<u16> {
        Some(self.in_out_w_rte_win_tms)
    }

    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    fn set_in_out_w_rte_win_tms(&mut self, value: u16) {
        self.in_out_w_rte_win_tms = value;
    }

    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    fn in_out_w_rte_rvrt_tms(&self) -> Option<u16> {
        Some(self.in_out_w_rte_rvrt_tms)
    }

    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    fn set_in_out_w_rte_rvrt_tms(&mut self, value: u16) {
        self.in_out_w_rte_rvrt_tms = value;
    }

    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn in_out_w_rte_rmp_tms(&self) -> Option<u16> {
        Some(self.in_out_w_rte_rmp_tms)
    }

    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_in_out_w_rte_rmp_tms(&mut self, value: u16) {
        self.in_out_w_rte_rmp_tms = value;
    }

    fn cha_gri_set(&self) -> Option<ChaGriSet> {
        Some(self.cha_gri_set)
    }

    fn set_cha_gri_set(&mut self, value: ChaGriSet) {
        self.cha_gri_set = value;
    }

    /// WChaMax_SF
    ///
    /// Scale factor for maximum charge.
    fn w_cha_max_sf(&self) -> u16 {
        self.w_cha_max_sf
    }

    /// WChaDisChaGra_SF
    ///
    /// Scale factor for maximum charge and discharge rate.
    fn w_cha_dis_cha_gra_sf(&self) -> u16 {
        self.w_cha_dis_cha_gra_sf
    }

    /// VAChaMax_SF
    ///
    /// Scale factor for maximum charging VA.
    fn va_cha_max_sf(&self) -> Option<u16> {
        Some(self.va_cha_max_sf)
    }

    /// MinRsvPct_SF
    ///
    /// Scale factor for minimum reserve percentage.
    fn min_rsv_pct_sf(&self) -> Option<u16> {
        Some(self.min_rsv_pct_sf)
    }

    /// ChaState_SF
    ///
    /// Scale factor for available energy percent.
    fn cha_state_sf(&self) -> Option<u16> {
        Some(self.cha_state_sf)
    }

    /// StorAval_SF
    ///
    /// Scale factor for state of charge.
    fn stor_aval_sf(&self) -> Option<u16> {
        Some(self.stor_aval_sf)
    }

    /// InBatV_SF
    ///
    /// Scale factor for battery voltage.
    fn in_bat_v_sf(&self) -> Option<u16> {
        Some(self.in_bat_v_sf)
    }

    /// InOutWRte_SF
    ///
    /// Scale factor for percent charge/discharge rate.
    fn in_out_w_rte_sf(&self) -> Option<u16> {
        Some(self.in_out_w_rte_sf)
    }
}
