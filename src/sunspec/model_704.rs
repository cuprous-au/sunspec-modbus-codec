pub type Model704 = DerCtlAc;

/// DER AC controls model.
pub struct DerCtlAc {
    /// Model ID
    ///
    /// DER AC controls model ID.
    id: u16,
    /// Model Length
    ///
    /// DER AC controls model length.
    l: u16,
    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    pfw_inj_ena: Option<PfwInjEna>,
    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    pfw_inj_ena_rvrt: Option<PfwInjEnaRvrt>,
    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    pfw_inj_rvrt_tms: Option<u32>,
    /// PF Reversion Time Rem (W Inj)
    ///
    /// Power factor reversion time remaining when injecting active power.
    pfw_inj_rvrt_rem: Option<u32>,
    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    pfw_abs_ena: Option<PfwAbsEna>,
    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    pfw_abs_ena_rvrt: Option<PfwAbsEnaRvrt>,
    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    pfw_abs_rvrt_tms: Option<u32>,
    /// PF Reversion Time Rem (W Abs)
    ///
    /// Power factor reversion time remaining when absorbing active power.
    pfw_abs_rvrt_rem: Option<u32>,
    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    w_max_lim_pct_ena: Option<WMaxLimPctEna>,
    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    w_max_lim_pct: Option<u16>,
    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    w_max_lim_pct_rvrt: Option<u16>,
    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    w_max_lim_pct_ena_rvrt: Option<WMaxLimPctEnaRvrt>,
    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    w_max_lim_pct_rvrt_tms: Option<u32>,
    /// Limit Max Power Pct Rev Time Rem
    ///
    /// Limit maximum active power percent reversion time remaining.
    w_max_lim_pct_rvrt_rem: Option<u32>,
    /// Set Active Power Enable
    ///
    /// Set active power enable.
    w_set_ena: Option<WSetEna>,
    /// Set Active Power Mode
    ///
    /// Set active power mode.
    w_set_mod: Option<WSetMod>,
    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    w_set: Option<i32>,
    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    w_set_rvrt: Option<i32>,
    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    w_set_pct: Option<i16>,
    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    w_set_pct_rvrt: Option<i16>,
    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    w_set_ena_rvrt: Option<WSetEnaRvrt>,
    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    w_set_rvrt_tms: Option<u32>,
    /// Active Power Rev Time Rem
    ///
    /// Set active power reversion time remaining.
    w_set_rvrt_rem: Option<u32>,
    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    var_set_ena: Option<VarSetEna>,
    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    var_set_mod: Option<VarSetMod>,
    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    var_set_pri: Option<VarSetPri>,
    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    var_set: Option<i32>,
    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    var_set_rvrt: Option<i32>,
    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    var_set_pct: Option<i16>,
    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    var_set_pct_rvrt: Option<i16>,
    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    var_set_ena_rvrt: Option<VarSetEnaRvrt>,
    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    var_set_rvrt_tms: Option<u32>,
    /// Reactive Power Rev Time Rem
    ///
    /// Set reactive power reversion time remaining.
    var_set_rvrt_rem: Option<u32>,
    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    w_rmp: Option<u16>,
    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    w_rmp_ref: Option<WRmpRef>,
    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    var_rmp: Option<u16>,
    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    anti_isl_ena: Option<AntiIslEna>,
    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    pf_sf: Option<u16>,
    /// Limit Max Power Scale Factor
    ///
    /// Limit maximum power scale factor.
    w_max_lim_pct_sf: Option<u16>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    w_set_sf: Option<u16>,
    /// Active Power Pct Scale Factor
    ///
    /// Active power pct scale factor.
    w_set_pct_sf: Option<u16>,
    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    var_set_sf: Option<u16>,
    /// Reactive Power Pct Scale Factor
    ///
    /// Reactive power pct scale factor.
    var_set_pct_sf: Option<u16>,
}

trait DerCtlAcTrait {
    /// Model ID
    ///
    /// DER AC controls model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER AC controls model length.
    fn l(&self) -> u16;

    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    fn pfw_inj_ena(&self) -> Option<PfwInjEna> {
        None
    }

    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    fn set_pfw_inj_ena(&mut self, value: PfwInjEna) {}

    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    fn pfw_inj_ena_rvrt(&self) -> Option<PfwInjEnaRvrt> {
        None
    }

    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    fn set_pfw_inj_ena_rvrt(&mut self, value: PfwInjEnaRvrt) {}

    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    fn pfw_inj_rvrt_tms(&self) -> Option<u32> {
        None
    }

    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    fn set_pfw_inj_rvrt_tms(&mut self, value: u32) {}

    /// PF Reversion Time Rem (W Inj)
    ///
    /// Power factor reversion time remaining when injecting active power.
    fn pfw_inj_rvrt_rem(&self) -> Option<u32> {
        None
    }

    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    fn pfw_abs_ena(&self) -> Option<PfwAbsEna> {
        None
    }

    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    fn set_pfw_abs_ena(&mut self, value: PfwAbsEna) {}

    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    fn pfw_abs_ena_rvrt(&self) -> Option<PfwAbsEnaRvrt> {
        None
    }

    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    fn set_pfw_abs_ena_rvrt(&mut self, value: PfwAbsEnaRvrt) {}

    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    fn pfw_abs_rvrt_tms(&self) -> Option<u32> {
        None
    }

    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    fn set_pfw_abs_rvrt_tms(&mut self, value: u32) {}

    /// PF Reversion Time Rem (W Abs)
    ///
    /// Power factor reversion time remaining when absorbing active power.
    fn pfw_abs_rvrt_rem(&self) -> Option<u32> {
        None
    }

    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    fn w_max_lim_pct_ena(&self) -> Option<WMaxLimPctEna> {
        None
    }

    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    fn set_w_max_lim_pct_ena(&mut self, value: WMaxLimPctEna) {}

    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    fn w_max_lim_pct(&self) -> Option<u16> {
        None
    }

    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    fn set_w_max_lim_pct(&mut self, value: u16) {}

    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    fn w_max_lim_pct_rvrt(&self) -> Option<u16> {
        None
    }

    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    fn set_w_max_lim_pct_rvrt(&mut self, value: u16) {}

    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    fn w_max_lim_pct_ena_rvrt(&self) -> Option<WMaxLimPctEnaRvrt> {
        None
    }

    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    fn set_w_max_lim_pct_ena_rvrt(&mut self, value: WMaxLimPctEnaRvrt) {}

    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    fn w_max_lim_pct_rvrt_tms(&self) -> Option<u32> {
        None
    }

    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    fn set_w_max_lim_pct_rvrt_tms(&mut self, value: u32) {}

    /// Limit Max Power Pct Rev Time Rem
    ///
    /// Limit maximum active power percent reversion time remaining.
    fn w_max_lim_pct_rvrt_rem(&self) -> Option<u32> {
        None
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn w_set_ena(&self) -> Option<WSetEna> {
        None
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn set_w_set_ena(&mut self, value: WSetEna) {}

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn w_set_mod(&self) -> Option<WSetMod> {
        None
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn set_w_set_mod(&mut self, value: WSetMod) {}

    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    fn w_set(&self) -> Option<i32> {
        None
    }

    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    fn set_w_set(&mut self, value: i32) {}

    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    fn w_set_rvrt(&self) -> Option<i32> {
        None
    }

    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    fn set_w_set_rvrt(&mut self, value: i32) {}

    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    fn w_set_pct(&self) -> Option<i16> {
        None
    }

    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    fn set_w_set_pct(&mut self, value: i16) {}

    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    fn w_set_pct_rvrt(&self) -> Option<i16> {
        None
    }

    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    fn set_w_set_pct_rvrt(&mut self, value: i16) {}

    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    fn w_set_ena_rvrt(&self) -> Option<WSetEnaRvrt> {
        None
    }

    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    fn set_w_set_ena_rvrt(&mut self, value: WSetEnaRvrt) {}

    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    fn w_set_rvrt_tms(&self) -> Option<u32> {
        None
    }

    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    fn set_w_set_rvrt_tms(&mut self, value: u32) {}

    /// Active Power Rev Time Rem
    ///
    /// Set active power reversion time remaining.
    fn w_set_rvrt_rem(&self) -> Option<u32> {
        None
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn var_set_ena(&self) -> Option<VarSetEna> {
        None
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn set_var_set_ena(&mut self, value: VarSetEna) {}

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn var_set_mod(&self) -> Option<VarSetMod> {
        None
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn set_var_set_mod(&mut self, value: VarSetMod) {}

    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    fn var_set_pri(&self) -> Option<VarSetPri> {
        None
    }

    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    fn set_var_set_pri(&mut self, value: VarSetPri) {}

    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    fn var_set(&self) -> Option<i32> {
        None
    }

    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    fn set_var_set(&mut self, value: i32) {}

    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    fn var_set_rvrt(&self) -> Option<i32> {
        None
    }

    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    fn set_var_set_rvrt(&mut self, value: i32) {}

    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    fn var_set_pct(&self) -> Option<i16> {
        None
    }

    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    fn set_var_set_pct(&mut self, value: i16) {}

    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    fn var_set_pct_rvrt(&self) -> Option<i16> {
        None
    }

    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    fn set_var_set_pct_rvrt(&mut self, value: i16) {}

    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    fn var_set_ena_rvrt(&self) -> Option<VarSetEnaRvrt> {
        None
    }

    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    fn set_var_set_ena_rvrt(&mut self, value: VarSetEnaRvrt) {}

    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    fn var_set_rvrt_tms(&self) -> Option<u32> {
        None
    }

    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    fn set_var_set_rvrt_tms(&mut self, value: u32) {}

    /// Reactive Power Rev Time Rem
    ///
    /// Set reactive power reversion time remaining.
    fn var_set_rvrt_rem(&self) -> Option<u32> {
        None
    }

    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    fn w_rmp(&self) -> Option<u16> {
        None
    }

    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    fn set_w_rmp(&mut self, value: u16) {}

    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    fn w_rmp_ref(&self) -> Option<WRmpRef> {
        None
    }

    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    fn set_w_rmp_ref(&mut self, value: WRmpRef) {}

    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    fn var_rmp(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    fn set_var_rmp(&mut self, value: u16) {}

    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    fn anti_isl_ena(&self) -> Option<AntiIslEna> {
        None
    }

    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    fn set_anti_isl_ena(&mut self, value: AntiIslEna) {}

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn pf_sf(&self) -> Option<u16> {
        None
    }

    /// Limit Max Power Scale Factor
    ///
    /// Limit maximum power scale factor.
    fn w_max_lim_pct_sf(&self) -> Option<u16> {
        None
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn w_set_sf(&self) -> Option<u16> {
        None
    }

    /// Active Power Pct Scale Factor
    ///
    /// Active power pct scale factor.
    fn w_set_pct_sf(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn var_set_sf(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Pct Scale Factor
    ///
    /// Reactive power pct scale factor.
    fn var_set_pct_sf(&self) -> Option<u16> {
        None
    }
}

pub enum PfwInjEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum PfwInjEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum PfwAbsEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum PfwAbsEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum WMaxLimPctEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum WMaxLimPctEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum WSetEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum WSetMod {
    /// Active Power As Max Percent
    ///
    /// Active power setting is percentage of maximum active power.
    WMaxPct = 0,
    /// Active Power As Watts
    ///
    /// Active power setting is in watts.
    Watts = 1,
}

pub enum WSetEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum VarSetEna {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum VarSetMod {
    /// Reactive Power As Watt Max Pct
    ///
    /// Reactive power setting is percent of maximum active power.
    WMaxPct = 0,
    /// Reactive Power As Var Max Pct
    ///
    /// Reactive power setting is percent of maximum reactive power.
    VarMaxPct = 1,
    /// Reactive Power As Var Avail Pct
    ///
    /// Reactive power setting is percent of available reactive  power.
    VarAvailPct = 2,
    /// Reactive Power As VA Max Pct
    ///
    /// Reactive power setting is percent of maximum apparent power.
    VaMaxPct = 3,
    /// Reactive Power As Vars
    ///
    /// Reactive power is in vars.
    Vars = 4,
}

pub enum VarSetPri {
    /// Active Power Priority
    ///
    /// Active power priority.
    Active = 0,
    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    Reactive = 1,
    /// Vendor Power Priority
    ///
    /// Power priority is vendor specific mode.
    Vendor = 2,
}

pub enum VarSetEnaRvrt {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum WRmpRef {
    /// Max Current Ramp
    ///
    /// Ramp based on percent of max current per second.
    AMax = 0,
    /// Max Active Power Ramp
    ///
    /// Ramp based on percent of max active power per second.
    WMax = 1,
}

pub enum AntiIslEna {
    /// Disabled
    ///
    /// Anti-islanding is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Anti-islanding is enabled.
    Enabled = 1,
}
