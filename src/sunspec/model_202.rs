pub type Model202 = AcMeterAbn;

pub struct AcMeterAbn {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Amps
    ///
    /// Total AC Current
    a: i16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    aph_a: Option<i16>,
    /// Amps PhaseB
    ///
    /// Phase B Current
    aph_b: i16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    aph_c: i16,
    /// Current scale factor
    a_sf: u16,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    ph_v: i16,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ph_vph_a: i16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    ph_vph_b: i16,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    ph_vph_c: Option<i16>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    ppv: i16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    ph_vph_ab: i16,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    ph_vph_bc: Option<i16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    ph_vph_ca: Option<i16>,
    /// Voltage scale factor
    v_sf: u16,
    /// Hz
    ///
    /// Frequency
    hz: i16,
    /// Frequency scale factor
    hz_sf: Option<u16>,
    /// Watts
    ///
    /// Total Real Power
    w: i16,
    /// Watts phase A
    wph_a: Option<i16>,
    /// Watts phase B
    wph_b: Option<i16>,
    /// Watts phase C
    wph_c: Option<i16>,
    /// Real Power scale factor
    w_sf: u16,
    /// VA
    ///
    /// AC Apparent Power
    va: Option<i16>,
    /// VA phase A
    v_aph_a: Option<i16>,
    /// VA phase B
    v_aph_b: Option<i16>,
    /// VA phase C
    v_aph_c: Option<i16>,
    /// Apparent Power scale factor
    va_sf: Option<u16>,
    /// VAR
    ///
    /// Reactive Power
    var: Option<i16>,
    /// VAR phase A
    va_rph_a: Option<i16>,
    /// VAR phase B
    va_rph_b: Option<i16>,
    /// VAR phase C
    va_rph_c: Option<i16>,
    /// Reactive Power scale factor
    var_sf: Option<u16>,
    /// PF
    ///
    /// Power Factor
    pf: Option<i16>,
    /// PF phase A
    p_fph_a: Option<i16>,
    /// PF phase B
    p_fph_b: Option<i16>,
    /// PF phase C
    p_fph_c: Option<i16>,
    /// Power Factor scale factor
    pf_sf: Option<u16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    tot_wh_exp: u32,
    /// Total Watt-hours Exported phase A
    tot_wh_exp_ph_a: Option<u32>,
    /// Total Watt-hours Exported phase B
    tot_wh_exp_ph_b: Option<u32>,
    /// Total Watt-hours Exported phase C
    tot_wh_exp_ph_c: Option<u32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    tot_wh_imp: u32,
    /// Total Watt-hours Imported phase A
    tot_wh_imp_ph_a: Option<u32>,
    /// Total Watt-hours Imported phase B
    tot_wh_imp_ph_b: Option<u32>,
    /// Total Watt-hours Imported phase C
    tot_wh_imp_ph_c: Option<u32>,
    /// Real Energy scale factor
    tot_wh_sf: u16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    tot_v_ah_exp: Option<u32>,
    /// Total VA-hours Exported phase A
    tot_v_ah_exp_ph_a: Option<u32>,
    /// Total VA-hours Exported phase B
    tot_v_ah_exp_ph_b: Option<u32>,
    /// Total VA-hours Exported phase C
    tot_v_ah_exp_ph_c: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    tot_v_ah_imp: Option<u32>,
    /// Total VA-hours Imported phase A
    tot_v_ah_imp_ph_a: Option<u32>,
    /// Total VA-hours Imported phase B
    tot_v_ah_imp_ph_b: Option<u32>,
    /// Total VA-hours Imported phase C
    tot_v_ah_imp_ph_c: Option<u32>,
    /// Apparent Energy scale factor
    tot_v_ah_sf: Option<u16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    tot_v_arh_imp_q1: Option<u32>,
    /// Total VAr-hours Imported Q1 phase A
    tot_v_arh_imp_q1_ph_a: Option<u32>,
    /// Total VAr-hours Imported Q1 phase B
    tot_v_arh_imp_q1_ph_b: Option<u32>,
    /// Total VAr-hours Imported Q1 phase C
    tot_v_arh_imp_q1_ph_c: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    tot_v_arh_imp_q2: Option<u32>,
    /// Total VAr-hours Imported Q2 phase A
    tot_v_arh_imp_q2_ph_a: Option<u32>,
    /// Total VAr-hours Imported Q2 phase B
    tot_v_arh_imp_q2_ph_b: Option<u32>,
    /// Total VAr-hours Imported Q2 phase C
    tot_v_arh_imp_q2_ph_c: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    tot_v_arh_exp_q3: Option<u32>,
    /// Total VAr-hours Exported Q3 phase A
    tot_v_arh_exp_q3_ph_a: Option<u32>,
    /// Total VAr-hours Exported Q3 phase B
    tot_v_arh_exp_q3_ph_b: Option<u32>,
    /// Total VAr-hours Exported Q3 phase C
    tot_v_arh_exp_q3_ph_c: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    tot_v_arh_exp_q4: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    tot_v_arh_exp_q4_ph_a: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    tot_v_arh_exp_q4_ph_b: Option<u32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    tot_v_arh_exp_q4_ph_c: Option<u32>,
    /// Reactive Energy scale factor
    tot_v_arh_sf: Option<u16>,
    /// Events
    ///
    /// Meter Event Flags
    evt: u32,
}

trait AcMeterAbnTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Amps
    ///
    /// Total AC Current
    fn a(&self) -> i16;

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn aph_a(&self) -> Option<i16> {
        None
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn aph_b(&self) -> i16;

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn aph_c(&self) -> i16;

    /// Current scale factor
    fn a_sf(&self) -> u16;

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    fn ph_v(&self) -> i16;

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn ph_vph_a(&self) -> i16;

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn ph_vph_b(&self) -> i16;

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn ph_vph_c(&self) -> Option<i16> {
        None
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn ppv(&self) -> i16;

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn ph_vph_ab(&self) -> i16;

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn ph_vph_bc(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn ph_vph_ca(&self) -> Option<i16> {
        None
    }

    /// Voltage scale factor
    fn v_sf(&self) -> u16;

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> i16;

    /// Frequency scale factor
    fn hz_sf(&self) -> Option<u16> {
        None
    }

    /// Watts
    ///
    /// Total Real Power
    fn w(&self) -> i16;

    /// Watts phase A
    fn wph_a(&self) -> Option<i16> {
        None
    }

    /// Watts phase B
    fn wph_b(&self) -> Option<i16> {
        None
    }

    /// Watts phase C
    fn wph_c(&self) -> Option<i16> {
        None
    }

    /// Real Power scale factor
    fn w_sf(&self) -> u16;

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        None
    }

    /// VA phase A
    fn v_aph_a(&self) -> Option<i16> {
        None
    }

    /// VA phase B
    fn v_aph_b(&self) -> Option<i16> {
        None
    }

    /// VA phase C
    fn v_aph_c(&self) -> Option<i16> {
        None
    }

    /// Apparent Power scale factor
    fn va_sf(&self) -> Option<u16> {
        None
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<i16> {
        None
    }

    /// VAR phase A
    fn va_rph_a(&self) -> Option<i16> {
        None
    }

    /// VAR phase B
    fn va_rph_b(&self) -> Option<i16> {
        None
    }

    /// VAR phase C
    fn va_rph_c(&self) -> Option<i16> {
        None
    }

    /// Reactive Power scale factor
    fn var_sf(&self) -> Option<u16> {
        None
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<i16> {
        None
    }

    /// PF phase A
    fn p_fph_a(&self) -> Option<i16> {
        None
    }

    /// PF phase B
    fn p_fph_b(&self) -> Option<i16> {
        None
    }

    /// PF phase C
    fn p_fph_c(&self) -> Option<i16> {
        None
    }

    /// Power Factor scale factor
    fn pf_sf(&self) -> Option<u16> {
        None
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn tot_wh_exp(&self) -> u32;

    /// Total Watt-hours Exported phase A
    fn tot_wh_exp_ph_a(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Exported phase B
    fn tot_wh_exp_ph_b(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Exported phase C
    fn tot_wh_exp_ph_c(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn tot_wh_imp(&self) -> u32;

    /// Total Watt-hours Imported phase A
    fn tot_wh_imp_ph_a(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Imported phase B
    fn tot_wh_imp_ph_b(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Imported phase C
    fn tot_wh_imp_ph_c(&self) -> Option<u32> {
        None
    }

    /// Real Energy scale factor
    fn tot_wh_sf(&self) -> u16;

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn tot_v_ah_exp(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Exported phase A
    fn tot_v_ah_exp_ph_a(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Exported phase B
    fn tot_v_ah_exp_ph_b(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Exported phase C
    fn tot_v_ah_exp_ph_c(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn tot_v_ah_imp(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported phase A
    fn tot_v_ah_imp_ph_a(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported phase B
    fn tot_v_ah_imp_ph_b(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported phase C
    fn tot_v_ah_imp_ph_c(&self) -> Option<u32> {
        None
    }

    /// Apparent Energy scale factor
    fn tot_v_ah_sf(&self) -> Option<u16> {
        None
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn tot_v_arh_imp_q1(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase A
    fn tot_v_arh_imp_q1_ph_a(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase B
    fn tot_v_arh_imp_q1_ph_b(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase C
    fn tot_v_arh_imp_q1_ph_c(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn tot_v_arh_imp_q2(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase A
    fn tot_v_arh_imp_q2_ph_a(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase B
    fn tot_v_arh_imp_q2_ph_b(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase C
    fn tot_v_arh_imp_q2_ph_c(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn tot_v_arh_exp_q3(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase A
    fn tot_v_arh_exp_q3_ph_a(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase B
    fn tot_v_arh_exp_q3_ph_b(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase C
    fn tot_v_arh_exp_q3_ph_c(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn tot_v_arh_exp_q4(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn tot_v_arh_exp_q4_ph_a(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn tot_v_arh_exp_q4_ph_b(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn tot_v_arh_exp_q4_ph_c(&self) -> Option<u32> {
        None
    }

    /// Reactive Energy scale factor
    fn tot_v_arh_sf(&self) -> Option<u16> {
        None
    }

    /// Events
    ///
    /// Meter Event Flags
    fn evt(&self) -> u32;
}
