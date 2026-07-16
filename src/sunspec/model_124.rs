pub type Model124 = StorageBasic;

/// Basic Storage Controls
pub struct StorageBasic {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// WChaMax
    ///
    /// Setpoint for maximum charge.
    w_cha_max: u16,
    /// WChaGra
    ///
    /// Setpoint for maximum charging rate. Default is MaxChaRte.
    w_cha_gra: u16,
    /// WDisChaGra
    ///
    /// Setpoint for maximum discharge rate. Default is MaxDisChaRte.
    w_dis_cha_gra: u16,
    /// StorCtl_Mod
    ///
    /// Activate hold/discharge/charge storage control mode.
    stor_ctl_mod: u16,
    /// VAChaMax
    ///
    /// Setpoint for maximum charging VA.
    va_cha_max: Option<u16>,
    /// MinRsvPct
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    min_rsv_pct: Option<u16>,
    /// ChaState
    ///
    /// Currently available energy as a percent of the capacity rating.
    cha_state: Option<u16>,
    /// StorAval
    ///
    /// State of charge (ChaState) minus storage reserve (MinRsvPct) times capacity rating (AhrRtg).
    stor_aval: Option<u16>,
    /// InBatV
    ///
    /// Internal battery voltage.
    in_bat_v: Option<u16>,
    /// ChaSt
    ///
    /// Charge status of storage device.
    cha_st: Option<ChaSt>,
    /// OutWRte
    ///
    /// Percent of max discharge rate.
    out_w_rte: Option<i16>,
    /// InWRte
    ///
    /// Percent of max charging rate.
    in_w_rte: Option<i16>,
    /// InOutWRte_WinTms
    ///
    /// Time window for charge/discharge rate change.
    in_out_w_rte_win_tms: Option<u16>,
    /// InOutWRte_RvrtTms
    ///
    /// Timeout period for charge/discharge rate.
    in_out_w_rte_rvrt_tms: Option<u16>,
    /// InOutWRte_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    in_out_w_rte_rmp_tms: Option<u16>,
    cha_gri_set: Option<ChaGriSet>,
    /// WChaMax_SF
    ///
    /// Scale factor for maximum charge.
    w_cha_max_sf: u16,
    /// WChaDisChaGra_SF
    ///
    /// Scale factor for maximum charge and discharge rate.
    w_cha_dis_cha_gra_sf: u16,
    /// VAChaMax_SF
    ///
    /// Scale factor for maximum charging VA.
    va_cha_max_sf: Option<u16>,
    /// MinRsvPct_SF
    ///
    /// Scale factor for minimum reserve percentage.
    min_rsv_pct_sf: Option<u16>,
    /// ChaState_SF
    ///
    /// Scale factor for available energy percent.
    cha_state_sf: Option<u16>,
    /// StorAval_SF
    ///
    /// Scale factor for state of charge.
    stor_aval_sf: Option<u16>,
    /// InBatV_SF
    ///
    /// Scale factor for battery voltage.
    in_bat_v_sf: Option<u16>,
    /// InOutWRte_SF
    ///
    /// Scale factor for percent charge/discharge rate.
    in_out_w_rte_sf: Option<u16>,
}

trait StorageBasicTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

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
