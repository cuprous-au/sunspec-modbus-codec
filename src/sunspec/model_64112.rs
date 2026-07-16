pub struct Model64112 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Port Number
    port: u16,
    v_sf: u16,
    c_sf: u16,
    h_sf: u16,
    p_sf: u16,
    ah_sf: u16,
    kwh_sf: u16,
    /// Faults
    cc_config_fault: u16,
    /// Absorb
    cc_config_absorb_v: u16,
    /// Absorb Time
    cc_config_absorb_hr: u16,
    /// Absorb End
    cc_config_absorb_end_a: u16,
    /// Rebulk
    cc_config_rebulk_v: u16,
    /// Float
    cc_config_float_v: u16,
    /// Maximum Charge
    cc_config_max_chg_a: u16,
    /// Equalize
    cc_config_equalize_v: u16,
    /// Equalize Time
    cc_config_equalize_hr: u16,
    /// Auto Equalize Interval
    cc_config_auto_equalize: u16,
    /// MPPT mode
    cc_config_mppt_mode: CcConfigMpptMode,
    /// Sweep Width
    cc_config_sweep_width: CcConfigSweepWidth,
    /// Sweep Maximum
    cc_config_sweep_max: CcConfigSweepMax,
    /// U-Pick PWM Duty Cycle
    cc_config_u_pick_duty_cyc: u16,
    /// Grid Tie Mode
    cc_config_grid_tie: CcConfigGridTie,
    /// Temp Comp Mode
    cc_config_temp_comp: CcConfigTempComp,
    /// Temp Comp Lower Limit
    cc_config_temp_comp_llimt: u16,
    /// Temp Comp Upper Limit
    cc_config_temp_comp_hlimt: u16,
    /// Auto Restart Mode
    cc_config_auto_restart: CcConfigAutoRestart,
    /// Wakeup VOC Change
    cc_config_wakeup_voc: u16,
    /// Snooze Mode
    cc_config_snooze_mode_a: u16,
    /// Wakeup Interval
    cc_config_wakeup_interval: u16,
    /// AUX Output Mode
    cc_config_aux_mode: CcConfigAuxMode,
    /// AUX Output Control
    cc_config_aux_control: CcConfigAuxControl,
    /// AUX Output State
    cc_config_aux_state: CcConfigAuxState,
    /// AUX Output Polarity
    cc_config_aux_polarity: CcConfigAuxPolarity,
    /// AUX Low Battery Disconnect
    cc_config_aux_l_batt_disc: u16,
    /// AUX Low Battery Reconnect
    cc_config_aux_l_batt_rcon: u16,
    /// AUX Low Battery Disconnect Delay
    cc_config_aux_l_batt_dly: u16,
    /// AUX Vent Fan
    cc_config_aux_vent_fan_v: u16,
    /// AUX PV Trigger
    cc_config_aux_pv_trigger_v: u16,
    /// AUX PV Trigger Hold Time
    cc_config_aux_pv_trg_h_tm: u16,
    /// AUX Night Light Threshold
    cc_config_aux_nlite_thrs_v: u16,
    /// AUX Night Light On Time
    cc_config_aux_nlite_on_tm: u16,
    /// AUX Night Light On Hysteresis
    cc_config_aux_nlite_on_hist: u16,
    /// AUX Night Light Off Hysteresis
    cc_config_aux_nlite_off_hist: u16,
    /// AUX Error Output Low Battery
    cc_config_aux_error_batt_v: u16,
    /// AUX Divert Hold Time
    cc_config_aux_divert_h_time: u16,
    /// AUX Divert Delay Time
    cc_config_aux_divert_dly_time: u16,
    /// AUX Divert Relative
    cc_config_aux_divert_rel_v: u16,
    /// AUX Divert Hysteresis
    cc_config_aux_divert_hyst_v: u16,
    /// FM CC Major Firmware Number
    cc_config_major_fw_rev: u16,
    /// FM CC Mid Firmware Number
    cc_config_mid_fw_rev: u16,
    /// FM CC Minor Firmware Number
    cc_config_minor_fw_rev: u16,
    /// Set Data Log Day Offset
    cc_config_data_log_day_offset: u16,
    /// Current Data Log Day Offset
    cc_config_data_log_cur_day_off: u16,
    /// Data Log Daily (Ah)
    cc_config_data_log_daily_ah: u16,
    /// Data Log Daily (kWh)
    cc_config_data_log_daily_kwh: u16,
    /// Data Log Daily Maximum Output (A)
    cc_config_data_log_max_out_a: u16,
    /// Data Log Daily Maximum Output (W)
    cc_config_data_log_max_out_w: u16,
    /// Data Log Daily Absorb Time
    cc_config_data_log_absorb_t: u16,
    /// Data Log Daily Float Time
    cc_config_data_log_float_t: u16,
    /// Data Log Daily Minimum Battery
    cc_config_data_log_min_batt_v: u16,
    /// Data Log Daily Maximum Battery
    cc_config_data_log_max_batt_v: u16,
    /// Data Log Daily Maximum Input
    cc_config_data_log_max_input_v: u16,
    /// Data Log Clear
    cc_config_data_log_clear: u16,
    /// Data Log Clear Complement
    cc_config_data_log_clr_comp: u16,
}

trait Model64112Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Port Number
    fn port(&self) -> u16;

    fn v_sf(&self) -> u16;

    fn c_sf(&self) -> u16;

    fn h_sf(&self) -> u16;

    fn p_sf(&self) -> u16;

    fn ah_sf(&self) -> u16;

    fn kwh_sf(&self) -> u16;

    /// Faults
    fn cc_config_fault(&self) -> u16;

    /// Absorb
    fn cc_config_absorb_v(&self) -> u16;

    /// Absorb Time
    fn cc_config_absorb_hr(&self) -> u16;

    /// Absorb End
    fn cc_config_absorb_end_a(&self) -> u16;

    /// Rebulk
    fn cc_config_rebulk_v(&self) -> u16;

    /// Float
    fn cc_config_float_v(&self) -> u16;

    /// Maximum Charge
    fn cc_config_max_chg_a(&self) -> u16;

    /// Equalize
    fn cc_config_equalize_v(&self) -> u16;

    /// Equalize Time
    fn cc_config_equalize_hr(&self) -> u16;

    /// Auto Equalize Interval
    fn cc_config_auto_equalize(&self) -> u16;

    /// MPPT mode
    fn cc_config_mppt_mode(&self) -> CcConfigMpptMode;

    /// Sweep Width
    fn cc_config_sweep_width(&self) -> CcConfigSweepWidth;

    /// Sweep Maximum
    fn cc_config_sweep_max(&self) -> CcConfigSweepMax;

    /// U-Pick PWM Duty Cycle
    fn cc_config_u_pick_duty_cyc(&self) -> u16;

    /// Grid Tie Mode
    fn cc_config_grid_tie(&self) -> CcConfigGridTie;

    /// Temp Comp Mode
    fn cc_config_temp_comp(&self) -> CcConfigTempComp;

    /// Temp Comp Lower Limit
    fn cc_config_temp_comp_llimt(&self) -> u16;

    /// Temp Comp Upper Limit
    fn cc_config_temp_comp_hlimt(&self) -> u16;

    /// Auto Restart Mode
    fn cc_config_auto_restart(&self) -> CcConfigAutoRestart;

    /// Wakeup VOC Change
    fn cc_config_wakeup_voc(&self) -> u16;

    /// Snooze Mode
    fn cc_config_snooze_mode_a(&self) -> u16;

    /// Wakeup Interval
    fn cc_config_wakeup_interval(&self) -> u16;

    /// AUX Output Mode
    fn cc_config_aux_mode(&self) -> CcConfigAuxMode;

    /// AUX Output Control
    fn cc_config_aux_control(&self) -> CcConfigAuxControl;

    /// AUX Output State
    fn cc_config_aux_state(&self) -> CcConfigAuxState;

    /// AUX Output Polarity
    fn cc_config_aux_polarity(&self) -> CcConfigAuxPolarity;

    /// AUX Low Battery Disconnect
    fn cc_config_aux_l_batt_disc(&self) -> u16;

    /// AUX Low Battery Reconnect
    fn cc_config_aux_l_batt_rcon(&self) -> u16;

    /// AUX Low Battery Disconnect Delay
    fn cc_config_aux_l_batt_dly(&self) -> u16;

    /// AUX Vent Fan
    fn cc_config_aux_vent_fan_v(&self) -> u16;

    /// AUX PV Trigger
    fn cc_config_aux_pv_trigger_v(&self) -> u16;

    /// AUX PV Trigger Hold Time
    fn cc_config_aux_pv_trg_h_tm(&self) -> u16;

    /// AUX Night Light Threshold
    fn cc_config_aux_nlite_thrs_v(&self) -> u16;

    /// AUX Night Light On Time
    fn cc_config_aux_nlite_on_tm(&self) -> u16;

    /// AUX Night Light On Hysteresis
    fn cc_config_aux_nlite_on_hist(&self) -> u16;

    /// AUX Night Light Off Hysteresis
    fn cc_config_aux_nlite_off_hist(&self) -> u16;

    /// AUX Error Output Low Battery
    fn cc_config_aux_error_batt_v(&self) -> u16;

    /// AUX Divert Hold Time
    fn cc_config_aux_divert_h_time(&self) -> u16;

    /// AUX Divert Delay Time
    fn cc_config_aux_divert_dly_time(&self) -> u16;

    /// AUX Divert Relative
    fn cc_config_aux_divert_rel_v(&self) -> u16;

    /// AUX Divert Hysteresis
    fn cc_config_aux_divert_hyst_v(&self) -> u16;

    /// FM CC Major Firmware Number
    fn cc_config_major_fw_rev(&self) -> u16;

    /// FM CC Mid Firmware Number
    fn cc_config_mid_fw_rev(&self) -> u16;

    /// FM CC Minor Firmware Number
    fn cc_config_minor_fw_rev(&self) -> u16;

    /// Set Data Log Day Offset
    fn cc_config_data_log_day_offset(&self) -> u16;

    /// Current Data Log Day Offset
    fn cc_config_data_log_cur_day_off(&self) -> u16;

    /// Data Log Daily (Ah)
    fn cc_config_data_log_daily_ah(&self) -> u16;

    /// Data Log Daily (kWh)
    fn cc_config_data_log_daily_kwh(&self) -> u16;

    /// Data Log Daily Maximum Output (A)
    fn cc_config_data_log_max_out_a(&self) -> u16;

    /// Data Log Daily Maximum Output (W)
    fn cc_config_data_log_max_out_w(&self) -> u16;

    /// Data Log Daily Absorb Time
    fn cc_config_data_log_absorb_t(&self) -> u16;

    /// Data Log Daily Float Time
    fn cc_config_data_log_float_t(&self) -> u16;

    /// Data Log Daily Minimum Battery
    fn cc_config_data_log_min_batt_v(&self) -> u16;

    /// Data Log Daily Maximum Battery
    fn cc_config_data_log_max_batt_v(&self) -> u16;

    /// Data Log Daily Maximum Input
    fn cc_config_data_log_max_input_v(&self) -> u16;

    /// Data Log Clear
    fn cc_config_data_log_clear(&self) -> u16;

    /// Data Log Clear Complement
    fn cc_config_data_log_clr_comp(&self) -> u16;
}

pub enum CcConfigMpptMode {
    Auto = 0,
    UPick = 1,
    Wind = 2,
}

pub enum CcConfigSweepWidth {
    Half = 0,
    Full = 1,
}

pub enum CcConfigSweepMax {
    EightyPercent = 0,
    EightyFivePercent = 1,
    NintyPercent = 2,
    NintyNinePercent = 3,
}

pub enum CcConfigGridTie {
    Disabled = 0,
    Enabled = 1,
}

pub enum CcConfigTempComp {
    Wide = 0,
    Limited = 1,
}

pub enum CcConfigAutoRestart {
    Off = 0,
    Every90Minutes = 1,
    Every90MinutesIfAbsorbOrFloat = 2,
}

pub enum CcConfigAuxMode {
    Float = 0,
    DiversionRelay = 1,
    DiversionSolidSt = 2,
    LowBattDisconnect = 3,
    Remote = 4,
    VentFan = 5,
    PvTrigger = 6,
    ErrorOutput = 7,
    NightLight = 8,
}

pub enum CcConfigAuxControl {
    Off = 0,
    Auto = 1,
    On = 2,
}

pub enum CcConfigAuxState {
    Disabled = 0,
    Enabled = 1,
}

pub enum CcConfigAuxPolarity {
    Low = 0,
    High = 1,
}
