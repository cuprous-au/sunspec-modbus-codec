pub type Model121 = Settings;

/// Inverter Controls Basic Settings
pub struct Settings {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    w_max: u16,
    /// VRef
    ///
    /// Voltage at the PCC.
    v_ref: u16,
    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    v_ref_ofs: i16,
    /// VMax
    ///
    /// Setpoint for maximum voltage.
    v_max: Option<u16>,
    /// VMin
    ///
    /// Setpoint for minimum voltage.
    v_min: Option<u16>,
    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    va_max: Option<u16>,
    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    v_ar_max_q1: Option<i16>,
    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    v_ar_max_q2: Option<i16>,
    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    v_ar_max_q3: Option<i16>,
    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    v_ar_max_q4: Option<i16>,
    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    w_gra: Option<u16>,
    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    pf_min_q1: Option<i16>,
    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    pf_min_q2: Option<i16>,
    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    pf_min_q3: Option<i16>,
    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    pf_min_q4: Option<i16>,
    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    v_ar_act: Option<VArAct>,
    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    clc_tot_va: Option<ClcTotVa>,
    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    max_rmp_rte: Option<u16>,
    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    ecp_nom_hz: Option<u16>,
    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    conn_ph: Option<ConnPh>,
    /// WMax_SF
    ///
    /// Scale factor for real power.
    w_max_sf: u16,
    /// VRef_SF
    ///
    /// Scale factor for voltage at the PCC.
    v_ref_sf: u16,
    /// VRefOfs_SF
    ///
    /// Scale factor for offset voltage.
    v_ref_ofs_sf: u16,
    /// VMinMax_SF
    ///
    /// Scale factor for min/max voltages.
    v_min_max_sf: Option<u16>,
    /// VAMax_SF
    ///
    /// Scale factor for apparent power.
    va_max_sf: Option<u16>,
    /// VArMax_SF
    ///
    /// Scale factor for reactive power.
    v_ar_max_sf: Option<u16>,
    /// WGra_SF
    ///
    /// Scale factor for default ramp rate.
    w_gra_sf: Option<u16>,
    /// PFMin_SF
    ///
    /// Scale factor for minimum power factor.
    pf_min_sf: Option<u16>,
    /// MaxRmpRte_SF
    ///
    /// Scale factor for maximum ramp percentage.
    max_rmp_rte_sf: Option<u16>,
    /// ECPNomHz_SF
    ///
    /// Scale factor for nominal frequency.
    ecp_nom_hz_sf: Option<u16>,
}

trait SettingsTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    fn w_max(&self) -> u16;

    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    fn set_w_max(&mut self, value: u16);

    /// VRef
    ///
    /// Voltage at the PCC.
    fn v_ref(&self) -> u16;

    /// VRef
    ///
    /// Voltage at the PCC.
    fn set_v_ref(&mut self, value: u16);

    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    fn v_ref_ofs(&self) -> i16;

    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    fn set_v_ref_ofs(&mut self, value: i16);

    /// VMax
    ///
    /// Setpoint for maximum voltage.
    fn v_max(&self) -> Option<u16> {
        None
    }

    /// VMax
    ///
    /// Setpoint for maximum voltage.
    fn set_v_max(&mut self, value: u16) {}

    /// VMin
    ///
    /// Setpoint for minimum voltage.
    fn v_min(&self) -> Option<u16> {
        None
    }

    /// VMin
    ///
    /// Setpoint for minimum voltage.
    fn set_v_min(&mut self, value: u16) {}

    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    fn va_max(&self) -> Option<u16> {
        None
    }

    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    fn set_va_max(&mut self, value: u16) {}

    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    fn v_ar_max_q1(&self) -> Option<i16> {
        None
    }

    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    fn set_v_ar_max_q1(&mut self, value: i16) {}

    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    fn v_ar_max_q2(&self) -> Option<i16> {
        None
    }

    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    fn set_v_ar_max_q2(&mut self, value: i16) {}

    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    fn v_ar_max_q3(&self) -> Option<i16> {
        None
    }

    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    fn set_v_ar_max_q3(&mut self, value: i16) {}

    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    fn v_ar_max_q4(&self) -> Option<i16> {
        None
    }

    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    fn set_v_ar_max_q4(&mut self, value: i16) {}

    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    fn w_gra(&self) -> Option<u16> {
        None
    }

    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    fn set_w_gra(&mut self, value: u16) {}

    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    fn pf_min_q1(&self) -> Option<i16> {
        None
    }

    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    fn set_pf_min_q1(&mut self, value: i16) {}

    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    fn pf_min_q2(&self) -> Option<i16> {
        None
    }

    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    fn set_pf_min_q2(&mut self, value: i16) {}

    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    fn pf_min_q3(&self) -> Option<i16> {
        None
    }

    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    fn set_pf_min_q3(&mut self, value: i16) {}

    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    fn pf_min_q4(&self) -> Option<i16> {
        None
    }

    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    fn set_pf_min_q4(&mut self, value: i16) {}

    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    fn v_ar_act(&self) -> Option<VArAct> {
        None
    }

    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    fn set_v_ar_act(&mut self, value: VArAct) {}

    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    fn clc_tot_va(&self) -> Option<ClcTotVa> {
        None
    }

    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    fn set_clc_tot_va(&mut self, value: ClcTotVa) {}

    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    fn max_rmp_rte(&self) -> Option<u16> {
        None
    }

    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    fn set_max_rmp_rte(&mut self, value: u16) {}

    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    fn ecp_nom_hz(&self) -> Option<u16> {
        None
    }

    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    fn set_ecp_nom_hz(&mut self, value: u16) {}

    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    fn conn_ph(&self) -> Option<ConnPh> {
        None
    }

    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    fn set_conn_ph(&mut self, value: ConnPh) {}

    /// WMax_SF
    ///
    /// Scale factor for real power.
    fn w_max_sf(&self) -> u16;

    /// VRef_SF
    ///
    /// Scale factor for voltage at the PCC.
    fn v_ref_sf(&self) -> u16;

    /// VRefOfs_SF
    ///
    /// Scale factor for offset voltage.
    fn v_ref_ofs_sf(&self) -> u16;

    /// VMinMax_SF
    ///
    /// Scale factor for min/max voltages.
    fn v_min_max_sf(&self) -> Option<u16> {
        None
    }

    /// VAMax_SF
    ///
    /// Scale factor for apparent power.
    fn va_max_sf(&self) -> Option<u16> {
        None
    }

    /// VArMax_SF
    ///
    /// Scale factor for reactive power.
    fn v_ar_max_sf(&self) -> Option<u16> {
        None
    }

    /// WGra_SF
    ///
    /// Scale factor for default ramp rate.
    fn w_gra_sf(&self) -> Option<u16> {
        None
    }

    /// PFMin_SF
    ///
    /// Scale factor for minimum power factor.
    fn pf_min_sf(&self) -> Option<u16> {
        None
    }

    /// MaxRmpRte_SF
    ///
    /// Scale factor for maximum ramp percentage.
    fn max_rmp_rte_sf(&self) -> Option<u16> {
        None
    }

    /// ECPNomHz_SF
    ///
    /// Scale factor for nominal frequency.
    fn ecp_nom_hz_sf(&self) -> Option<u16> {
        None
    }
}

pub enum VArAct {
    Switch = 1,
    Maintain = 2,
}

pub enum ClcTotVa {
    Vector = 1,
    Arithmetic = 2,
}

pub enum ConnPh {
    A = 1,
    B = 2,
    C = 3,
}
