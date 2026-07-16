use heapless::String;

pub type Model701 = DerMeasureAc;

/// DER AC measurement model.
pub struct DerMeasureAc {
    /// Model ID
    ///
    /// DER AC measurement model ID.
    id: u16,
    /// Model Length
    ///
    /// DER AC measurement model length.
    l: u16,
    /// AC Wiring Type
    ///
    /// AC wiring type.
    ac_type: AcType,
    /// Operating State
    ///
    /// Operating state of the DER.
    st: Option<St>,
    /// Inverter State
    ///
    /// Inverter state.
    inv_st: Option<InvSt>,
    /// Grid Connection State
    ///
    /// Grid connection state of the DER.
    conn_st: Option<ConnSt>,
    /// Alarm Bitfield
    ///
    /// Active alarms for the DER.
    alrm: Option<u32>,
    /// DER Operational Characteristics
    ///
    /// Current operational characteristics of the DER.
    der_mode: Option<u32>,
    /// Active Power
    ///
    /// Total active power. Active power is positive for DER generation and negative for absorption.
    w: Option<i16>,
    /// Apparent Power
    ///
    /// Total apparent power.
    va: Option<i16>,
    /// Reactive Power
    ///
    /// Total reactive power.
    var: Option<i16>,
    /// Power Factor
    ///
    /// Power factor. The sign of power factor should be the sign of active power.
    pf: Option<i16>,
    /// Total AC Current
    ///
    /// Total AC current.
    a: Option<i16>,
    /// Voltage LL
    ///
    /// Line to line AC voltage as an average of active phases.
    llv: Option<u16>,
    /// Voltage LN
    ///
    /// Line to neutral AC voltage as an average of active phases.
    lnv: Option<u16>,
    /// Frequency
    ///
    /// AC frequency.
    hz: Option<u32>,
    /// Total Energy Injected
    ///
    /// Total active energy injected (Quadrants 1 & 4).
    tot_wh_inj: Option<u64>,
    /// Total Energy Absorbed
    ///
    /// Total active energy absorbed (Quadrants 2 & 3).
    tot_wh_abs: Option<u64>,
    /// Total Reactive Energy Inj
    ///
    /// Total reactive energy injected (Quadrants 1 & 2).
    tot_varh_inj: Option<u64>,
    /// Total Reactive Energy Abs
    ///
    /// Total reactive energy absorbed (Quadrants 3 & 4).
    tot_varh_abs: Option<u64>,
    /// Ambient Temperature
    ///
    /// Ambient temperature.
    tmp_amb: Option<i16>,
    /// Cabinet Temperature
    ///
    /// Cabinet temperature.
    tmp_cab: Option<i16>,
    /// Heat Sink Temperature
    ///
    /// Heat sink temperature.
    tmp_snk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer temperature.
    tmp_trns: Option<i16>,
    /// IGBT/MOSFET Temperature
    ///
    /// IGBT/MOSFET temperature.
    tmp_sw: Option<i16>,
    /// Other Temperature
    ///
    /// Other temperature.
    tmp_ot: Option<i16>,
    /// Watts L1
    ///
    /// Active power L1.
    wl1: Option<i16>,
    /// VA L1
    ///
    /// Apparent power L1.
    val1: Option<i16>,
    /// Var L1
    ///
    /// Reactive power L1.
    var_l1: Option<i16>,
    /// PF L1
    ///
    /// Power factor phase L1.
    pfl1: Option<i16>,
    /// Amps L1
    ///
    /// Current phase L1.
    al1: Option<i16>,
    /// Phase Voltage L1-L2
    ///
    /// Phase voltage L1-L2.
    vl1l2: Option<u16>,
    /// Phase Voltage L1-N
    ///
    /// Phase voltage L1-N.
    vl1: Option<u16>,
    /// Total Watt-Hours Inj L1
    ///
    /// Total active energy injected L1.
    tot_wh_inj_l1: Option<u64>,
    /// Total Watt-Hours Abs L1
    ///
    /// Total active energy absorbed L1.
    tot_wh_abs_l1: Option<u64>,
    /// Total Var-Hours Inj L1
    ///
    /// Total reactive energy injected L1.
    tot_varh_inj_l1: Option<u64>,
    /// Total Var-Hours Abs L1
    ///
    /// Total reactive energy absorbed L1.
    tot_varh_abs_l1: Option<u64>,
    /// Watts L2
    ///
    /// Active power L2.
    wl2: Option<i16>,
    /// VA L2
    ///
    /// Apparent power L2.
    val2: Option<i16>,
    /// Var L2
    ///
    /// Reactive power L2.
    var_l2: Option<i16>,
    /// PF L2
    ///
    /// Power factor L2.
    pfl2: Option<i16>,
    /// Amps L2
    ///
    /// Current L2.
    al2: Option<i16>,
    /// Phase Voltage L2-L3
    ///
    /// Phase voltage L2-L3.
    vl2l3: Option<u16>,
    /// Phase Voltage L2-N
    ///
    /// Phase voltage L2-N.
    vl2: Option<u16>,
    /// Total Watt-Hours Inj L2
    ///
    /// Total active energy injected L2.
    tot_wh_inj_l2: Option<u64>,
    /// Total Watt-Hours Abs L2
    ///
    /// Total active energy absorbed L2.
    tot_wh_abs_l2: Option<u64>,
    /// Total Var-Hours Inj L2
    ///
    /// Total reactive energy injected L2.
    tot_varh_inj_l2: Option<u64>,
    /// Total Var-Hours Abs L2
    ///
    /// Total reactive energy absorbed L2.
    tot_varh_abs_l2: Option<u64>,
    /// Watts L3
    ///
    /// Active power L3.
    wl3: Option<i16>,
    /// VA L3
    ///
    /// Apparent power L3.
    val3: Option<i16>,
    /// Var L3
    ///
    /// Reactive power L3.
    var_l3: Option<i16>,
    /// PF L3
    ///
    /// Power factor L3.
    pfl3: Option<i16>,
    /// Amps L3
    ///
    /// Current L3.
    al3: Option<i16>,
    /// Phase Voltage L3-L1
    ///
    /// Phase voltage L3-L1.
    vl3l1: Option<u16>,
    /// Phase Voltage L3-N
    ///
    /// Phase voltage L3-N.
    vl3: Option<u16>,
    /// Total Watt-Hours Inj L3
    ///
    /// Total active energy injected L3.
    tot_wh_inj_l3: Option<u64>,
    /// Total Watt-Hours Abs L3
    ///
    /// Total active energy absorbed L3.
    tot_wh_abs_l3: Option<u64>,
    /// Total Var-Hours Inj L3
    ///
    /// Total reactive energy injected L3.
    tot_varh_inj_l3: Option<u64>,
    /// Total Var-Hours Abs L3
    ///
    /// Total reactive energy absorbed L3.
    tot_varh_abs_l3: Option<u64>,
    /// Throttling In Pct
    ///
    /// Throttling in pct of maximum active power.
    throt_pct: Option<u16>,
    /// Throttle Source Information
    ///
    /// Active throttling source.
    throt_src: Option<u32>,
    /// Current Scale Factor
    ///
    /// Current scale factor.
    a_sf: Option<u16>,
    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    v_sf: Option<u16>,
    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    hz_sf: Option<u16>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    w_sf: Option<u16>,
    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    pf_sf: Option<u16>,
    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    va_sf: Option<u16>,
    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    var_sf: Option<u16>,
    /// Active Energy Scale Factor
    ///
    /// Active energy scale factor.
    tot_wh_sf: Option<u16>,
    /// Reactive Energy Scale Factor
    ///
    /// Reactive energy scale factor.
    tot_varh_sf: Option<u16>,
    /// Temperature Scale Factor
    ///
    /// Temperature scale factor.
    tmp_sf: Option<u16>,
    /// Manufacturer Alarm Info
    ///
    /// Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.
    mn_alrm_info: Option<String<64>>,
}

pub trait DerMeasureAcTrait {
    /// Model ID
    ///
    /// DER AC measurement model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER AC measurement model length.
    fn l(&self) -> u16;

    /// AC Wiring Type
    ///
    /// AC wiring type.
    fn ac_type(&self) -> AcType;

    /// Operating State
    ///
    /// Operating state of the DER.
    fn st(&self) -> Option<St> {
        None
    }

    /// Inverter State
    ///
    /// Inverter state.
    fn inv_st(&self) -> Option<InvSt> {
        None
    }

    /// Grid Connection State
    ///
    /// Grid connection state of the DER.
    fn conn_st(&self) -> Option<ConnSt> {
        None
    }

    /// Alarm Bitfield
    ///
    /// Active alarms for the DER.
    fn alrm(&self) -> Option<u32> {
        None
    }

    /// DER Operational Characteristics
    ///
    /// Current operational characteristics of the DER.
    fn der_mode(&self) -> Option<u32> {
        None
    }

    /// Active Power
    ///
    /// Total active power. Active power is positive for DER generation and negative for absorption.
    fn w(&self) -> Option<i16> {
        None
    }

    /// Apparent Power
    ///
    /// Total apparent power.
    fn va(&self) -> Option<i16> {
        None
    }

    /// Reactive Power
    ///
    /// Total reactive power.
    fn var(&self) -> Option<i16> {
        None
    }

    /// Power Factor
    ///
    /// Power factor. The sign of power factor should be the sign of active power.
    fn pf(&self) -> Option<i16> {
        None
    }

    /// Total AC Current
    ///
    /// Total AC current.
    fn a(&self) -> Option<i16> {
        None
    }

    /// Voltage LL
    ///
    /// Line to line AC voltage as an average of active phases.
    fn llv(&self) -> Option<u16> {
        None
    }

    /// Voltage LN
    ///
    /// Line to neutral AC voltage as an average of active phases.
    fn lnv(&self) -> Option<u16> {
        None
    }

    /// Frequency
    ///
    /// AC frequency.
    fn hz(&self) -> Option<u32> {
        None
    }

    /// Total Energy Injected
    ///
    /// Total active energy injected (Quadrants 1 & 4).
    fn tot_wh_inj(&self) -> Option<u64> {
        None
    }

    /// Total Energy Absorbed
    ///
    /// Total active energy absorbed (Quadrants 2 & 3).
    fn tot_wh_abs(&self) -> Option<u64> {
        None
    }

    /// Total Reactive Energy Inj
    ///
    /// Total reactive energy injected (Quadrants 1 & 2).
    fn tot_varh_inj(&self) -> Option<u64> {
        None
    }

    /// Total Reactive Energy Abs
    ///
    /// Total reactive energy absorbed (Quadrants 3 & 4).
    fn tot_varh_abs(&self) -> Option<u64> {
        None
    }

    /// Ambient Temperature
    ///
    /// Ambient temperature.
    fn tmp_amb(&self) -> Option<i16> {
        None
    }

    /// Cabinet Temperature
    ///
    /// Cabinet temperature.
    fn tmp_cab(&self) -> Option<i16> {
        None
    }

    /// Heat Sink Temperature
    ///
    /// Heat sink temperature.
    fn tmp_snk(&self) -> Option<i16> {
        None
    }

    /// Transformer Temperature
    ///
    /// Transformer temperature.
    fn tmp_trns(&self) -> Option<i16> {
        None
    }

    /// IGBT/MOSFET Temperature
    ///
    /// IGBT/MOSFET temperature.
    fn tmp_sw(&self) -> Option<i16> {
        None
    }

    /// Other Temperature
    ///
    /// Other temperature.
    fn tmp_ot(&self) -> Option<i16> {
        None
    }

    /// Watts L1
    ///
    /// Active power L1.
    fn wl1(&self) -> Option<i16> {
        None
    }

    /// VA L1
    ///
    /// Apparent power L1.
    fn val1(&self) -> Option<i16> {
        None
    }

    /// Var L1
    ///
    /// Reactive power L1.
    fn var_l1(&self) -> Option<i16> {
        None
    }

    /// PF L1
    ///
    /// Power factor phase L1.
    fn pfl1(&self) -> Option<i16> {
        None
    }

    /// Amps L1
    ///
    /// Current phase L1.
    fn al1(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage L1-L2
    ///
    /// Phase voltage L1-L2.
    fn vl1l2(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage L1-N
    ///
    /// Phase voltage L1-N.
    fn vl1(&self) -> Option<u16> {
        None
    }

    /// Total Watt-Hours Inj L1
    ///
    /// Total active energy injected L1.
    fn tot_wh_inj_l1(&self) -> Option<u64> {
        None
    }

    /// Total Watt-Hours Abs L1
    ///
    /// Total active energy absorbed L1.
    fn tot_wh_abs_l1(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Inj L1
    ///
    /// Total reactive energy injected L1.
    fn tot_varh_inj_l1(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Abs L1
    ///
    /// Total reactive energy absorbed L1.
    fn tot_varh_abs_l1(&self) -> Option<u64> {
        None
    }

    /// Watts L2
    ///
    /// Active power L2.
    fn wl2(&self) -> Option<i16> {
        None
    }

    /// VA L2
    ///
    /// Apparent power L2.
    fn val2(&self) -> Option<i16> {
        None
    }

    /// Var L2
    ///
    /// Reactive power L2.
    fn var_l2(&self) -> Option<i16> {
        None
    }

    /// PF L2
    ///
    /// Power factor L2.
    fn pfl2(&self) -> Option<i16> {
        None
    }

    /// Amps L2
    ///
    /// Current L2.
    fn al2(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage L2-L3
    ///
    /// Phase voltage L2-L3.
    fn vl2l3(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage L2-N
    ///
    /// Phase voltage L2-N.
    fn vl2(&self) -> Option<u16> {
        None
    }

    /// Total Watt-Hours Inj L2
    ///
    /// Total active energy injected L2.
    fn tot_wh_inj_l2(&self) -> Option<u64> {
        None
    }

    /// Total Watt-Hours Abs L2
    ///
    /// Total active energy absorbed L2.
    fn tot_wh_abs_l2(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Inj L2
    ///
    /// Total reactive energy injected L2.
    fn tot_varh_inj_l2(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Abs L2
    ///
    /// Total reactive energy absorbed L2.
    fn tot_varh_abs_l2(&self) -> Option<u64> {
        None
    }

    /// Watts L3
    ///
    /// Active power L3.
    fn wl3(&self) -> Option<i16> {
        None
    }

    /// VA L3
    ///
    /// Apparent power L3.
    fn val3(&self) -> Option<i16> {
        None
    }

    /// Var L3
    ///
    /// Reactive power L3.
    fn var_l3(&self) -> Option<i16> {
        None
    }

    /// PF L3
    ///
    /// Power factor L3.
    fn pfl3(&self) -> Option<i16> {
        None
    }

    /// Amps L3
    ///
    /// Current L3.
    fn al3(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage L3-L1
    ///
    /// Phase voltage L3-L1.
    fn vl3l1(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage L3-N
    ///
    /// Phase voltage L3-N.
    fn vl3(&self) -> Option<u16> {
        None
    }

    /// Total Watt-Hours Inj L3
    ///
    /// Total active energy injected L3.
    fn tot_wh_inj_l3(&self) -> Option<u64> {
        None
    }

    /// Total Watt-Hours Abs L3
    ///
    /// Total active energy absorbed L3.
    fn tot_wh_abs_l3(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Inj L3
    ///
    /// Total reactive energy injected L3.
    fn tot_varh_inj_l3(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Abs L3
    ///
    /// Total reactive energy absorbed L3.
    fn tot_varh_abs_l3(&self) -> Option<u64> {
        None
    }

    /// Throttling In Pct
    ///
    /// Throttling in pct of maximum active power.
    fn throt_pct(&self) -> Option<u16> {
        None
    }

    /// Throttle Source Information
    ///
    /// Active throttling source.
    fn throt_src(&self) -> Option<u32> {
        None
    }

    /// Current Scale Factor
    ///
    /// Current scale factor.
    fn a_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    fn v_sf(&self) -> Option<u16> {
        None
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn hz_sf(&self) -> Option<u16> {
        None
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn w_sf(&self) -> Option<u16> {
        None
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn pf_sf(&self) -> Option<u16> {
        None
    }

    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    fn va_sf(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn var_sf(&self) -> Option<u16> {
        None
    }

    /// Active Energy Scale Factor
    ///
    /// Active energy scale factor.
    fn tot_wh_sf(&self) -> Option<u16> {
        None
    }

    /// Reactive Energy Scale Factor
    ///
    /// Reactive energy scale factor.
    fn tot_varh_sf(&self) -> Option<u16> {
        None
    }

    /// Temperature Scale Factor
    ///
    /// Temperature scale factor.
    fn tmp_sf(&self) -> Option<u16> {
        None
    }

    /// Manufacturer Alarm Info
    ///
    /// Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.
    fn mn_alrm_info(&self) -> Option<String<64>> {
        None
    }
}

pub enum AcType {
    /// Single Phase
    SinglePhase = 0,
    /// Split Phase
    SplitPhase = 1,
    /// Three Phase
    ThreePhase = 2,
}

pub enum St {
    /// Off
    Off = 0,
    /// On
    On = 1,
}

pub enum InvSt {
    Off = 0,
    Sleeping = 1,
    Starting = 2,
    Running = 3,
    Throttled = 4,
    ShuttingDown = 5,
    Fault = 6,
    Standby = 7,
}

pub enum ConnSt {
    /// Disconnected
    ///
    /// Disconnected from the grid.
    Disconnected = 0,
    /// Connected
    ///
    /// Connected to the grid.
    Connected = 1,
}
