pub type Model123 = Controls;

/// Immediate Inverter Controls
pub struct Controls {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    conn_win_tms: Option<u16>,
    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    conn_rvrt_tms: Option<u16>,
    /// Conn
    ///
    /// Connection control.
    conn: Conn,
    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    w_max_lim_pct: u16,
    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    w_max_lim_pct_win_tms: Option<u16>,
    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    w_max_lim_pct_rvrt_tms: Option<u16>,
    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    w_max_lim_pct_rmp_tms: Option<u16>,
    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    w_max_lim_ena: WMaxLimEna,
    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    out_pf_set: i16,
    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    out_pf_set_win_tms: Option<u16>,
    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    out_pf_set_rvrt_tms: Option<u16>,
    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    out_pf_set_rmp_tms: Option<u16>,
    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    out_pf_set_ena: OutPfSetEna,
    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    v_ar_w_max_pct: Option<i16>,
    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    v_ar_max_pct: Option<i16>,
    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    v_ar_aval_pct: Option<i16>,
    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    v_ar_pct_win_tms: Option<u16>,
    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    v_ar_pct_rvrt_tms: Option<u16>,
    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    v_ar_pct_rmp_tms: Option<u16>,
    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    v_ar_pct_mod: Option<VArPctMod>,
    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    v_ar_pct_ena: VArPctEna,
    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    w_max_lim_pct_sf: u16,
    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    out_pf_set_sf: u16,
    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    v_ar_pct_sf: Option<u16>,
}

trait ControlsTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn conn_win_tms(&self) -> Option<u16> {
        None
    }

    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn set_conn_win_tms(&mut self, value: u16) {}

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn conn_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn set_conn_rvrt_tms(&mut self, value: u16) {}

    /// Conn
    ///
    /// Connection control.
    fn conn(&self) -> Conn;

    /// Conn
    ///
    /// Connection control.
    fn set_conn(&mut self, value: Conn);

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn w_max_lim_pct(&self) -> u16;

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn set_w_max_lim_pct(&mut self, value: u16);

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn w_max_lim_pct_win_tms(&self) -> Option<u16> {
        None
    }

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn set_w_max_lim_pct_win_tms(&mut self, value: u16) {}

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn w_max_lim_pct_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn set_w_max_lim_pct_rvrt_tms(&mut self, value: u16) {}

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn w_max_lim_pct_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_w_max_lim_pct_rmp_tms(&mut self, value: u16) {}

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn w_max_lim_ena(&self) -> WMaxLimEna;

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn set_w_max_lim_ena(&mut self, value: WMaxLimEna);

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn out_pf_set(&self) -> i16;

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn set_out_pf_set(&mut self, value: i16);

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn out_pf_set_win_tms(&self) -> Option<u16> {
        None
    }

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn set_out_pf_set_win_tms(&mut self, value: u16) {}

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn out_pf_set_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn set_out_pf_set_rvrt_tms(&mut self, value: u16) {}

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn out_pf_set_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_out_pf_set_rmp_tms(&mut self, value: u16) {}

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn out_pf_set_ena(&self) -> OutPfSetEna;

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn set_out_pf_set_ena(&mut self, value: OutPfSetEna);

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn v_ar_w_max_pct(&self) -> Option<i16> {
        None
    }

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn set_v_ar_w_max_pct(&mut self, value: i16) {}

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn v_ar_max_pct(&self) -> Option<i16> {
        None
    }

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn set_v_ar_max_pct(&mut self, value: i16) {}

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn v_ar_aval_pct(&self) -> Option<i16> {
        None
    }

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn set_v_ar_aval_pct(&mut self, value: i16) {}

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn v_ar_pct_win_tms(&self) -> Option<u16> {
        None
    }

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn set_v_ar_pct_win_tms(&mut self, value: u16) {}

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn v_ar_pct_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn set_v_ar_pct_rvrt_tms(&mut self, value: u16) {}

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn v_ar_pct_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_v_ar_pct_rmp_tms(&mut self, value: u16) {}

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn v_ar_pct_mod(&self) -> Option<VArPctMod> {
        None
    }

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn set_v_ar_pct_mod(&mut self, value: VArPctMod) {}

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn v_ar_pct_ena(&self) -> VArPctEna;

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn set_v_ar_pct_ena(&mut self, value: VArPctEna);

    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    fn w_max_lim_pct_sf(&self) -> u16;

    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    fn out_pf_set_sf(&self) -> u16;

    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    fn v_ar_pct_sf(&self) -> Option<u16> {
        None
    }
}

pub enum Conn {
    Disconnect = 0,
    Connect = 1,
}

pub enum WMaxLimEna {
    Disabled = 0,
    Enabled = 1,
}

pub enum OutPfSetEna {
    Disabled = 0,
    Enabled = 1,
}

pub enum VArPctMod {
    None = 0,
    WMax = 1,
    VArMax = 2,
    VArAval = 3,
}

pub enum VArPctEna {
    Disabled = 0,
    Enabled = 1,
}
