pub type Model211 = AcMeterAnOrAbFloat;

pub struct AcMeterAnOrAbFloat {
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
    a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    aph_a: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    aph_b: Option<f32>,
    /// Amps PhaseC
    ///
    /// Phase C Current
    aph_c: Option<f32>,
    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    ph_v: Option<f32>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ph_vph_a: Option<f32>,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    ph_vph_b: Option<f32>,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    ph_vph_c: Option<f32>,
    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    ppv: Option<f32>,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pp_vph_ab: Option<f32>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pp_vph_bc: Option<f32>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pp_vph_ca: Option<f32>,
    /// Hz
    ///
    /// Frequency
    hz: f32,
    /// Watts
    ///
    /// Total Real Power
    w: f32,
    /// Watts phase A
    wph_a: Option<f32>,
    /// Watts phase B
    wph_b: Option<f32>,
    /// Watts phase C
    wph_c: Option<f32>,
    /// VA
    ///
    /// AC Apparent Power
    va: Option<f32>,
    /// VA phase A
    v_aph_a: Option<f32>,
    /// VA phase B
    v_aph_b: Option<f32>,
    /// VA phase C
    v_aph_c: Option<f32>,
    /// VAR
    ///
    /// Reactive Power
    var: Option<f32>,
    /// VAR phase A
    va_rph_a: Option<f32>,
    /// VAR phase B
    va_rph_b: Option<f32>,
    /// VAR phase C
    va_rph_c: Option<f32>,
    /// PF
    ///
    /// Power Factor
    pf: Option<f32>,
    /// PF phase A
    p_fph_a: Option<f32>,
    /// PF phase B
    p_fph_b: Option<f32>,
    /// PF phase C
    p_fph_c: Option<f32>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    tot_wh_exp: f32,
    /// Total Watt-hours Exported phase A
    tot_wh_exp_ph_a: Option<f32>,
    /// Total Watt-hours Exported phase B
    tot_wh_exp_ph_b: Option<f32>,
    /// Total Watt-hours Exported phase C
    tot_wh_exp_ph_c: Option<f32>,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    tot_wh_imp: f32,
    /// Total Watt-hours Imported phase A
    tot_wh_imp_ph_a: Option<f32>,
    /// Total Watt-hours Imported phase B
    tot_wh_imp_ph_b: Option<f32>,
    /// Total Watt-hours Imported phase C
    tot_wh_imp_ph_c: Option<f32>,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    tot_v_ah_exp: Option<f32>,
    /// Total VA-hours Exported phase A
    tot_v_ah_exp_ph_a: Option<f32>,
    /// Total VA-hours Exported phase B
    tot_v_ah_exp_ph_b: Option<f32>,
    /// Total VA-hours Exported phase C
    tot_v_ah_exp_ph_c: Option<f32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    tot_v_ah_imp: Option<f32>,
    /// Total VA-hours Imported phase A
    tot_v_ah_imp_ph_a: Option<f32>,
    /// Total VA-hours Imported phase B
    tot_v_ah_imp_ph_b: Option<f32>,
    /// Total VA-hours Imported phase C
    tot_v_ah_imp_ph_c: Option<f32>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    tot_v_arh_imp_q1: Option<f32>,
    /// Total VAr-hours Imported Q1 phase A
    tot_v_arh_imp_q1ph_a: Option<f32>,
    /// Total VAr-hours Imported Q1 phase B
    tot_v_arh_imp_q1ph_b: Option<f32>,
    /// Total VAr-hours Imported Q1 phase C
    tot_v_arh_imp_q1ph_c: Option<f32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    tot_v_arh_imp_q2: Option<f32>,
    /// Total VAr-hours Imported Q2 phase A
    tot_v_arh_imp_q2ph_a: Option<f32>,
    /// Total VAr-hours Imported Q2 phase B
    tot_v_arh_imp_q2ph_b: Option<f32>,
    /// Total VAr-hours Imported Q2 phase C
    tot_v_arh_imp_q2ph_c: Option<f32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    tot_v_arh_exp_q3: Option<f32>,
    /// Total VAr-hours Exported Q3 phase A
    tot_v_arh_exp_q3ph_a: Option<f32>,
    /// Total VAr-hours Exported Q3 phase B
    tot_v_arh_exp_q3ph_b: Option<f32>,
    /// Total VAr-hours Exported Q3 phase C
    tot_v_arh_exp_q3ph_c: Option<f32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    tot_v_arh_exp_q4: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase A
    tot_v_arh_exp_q4ph_a: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase B
    tot_v_arh_exp_q4ph_b: Option<f32>,
    /// Total VAr-hours Exported Q4 Imported phase C
    tot_v_arh_exp_q4ph_c: Option<f32>,
    /// Events
    ///
    /// Meter Event Flags
    evt: u32,
}

trait AcMeterAnOrAbFloatTrait {
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
    fn a(&self) -> f32;

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn aph_a(&self) -> f32;

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn aph_b(&self) -> Option<f32> {
        None
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn aph_c(&self) -> Option<f32> {
        None
    }

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    fn ph_v(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn ph_vph_a(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn ph_vph_b(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn ph_vph_c(&self) -> Option<f32> {
        None
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn ppv(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn pp_vph_ab(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn pp_vph_bc(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn pp_vph_ca(&self) -> Option<f32> {
        None
    }

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> f32;

    /// Watts
    ///
    /// Total Real Power
    fn w(&self) -> f32;

    /// Watts phase A
    fn wph_a(&self) -> Option<f32> {
        None
    }

    /// Watts phase B
    fn wph_b(&self) -> Option<f32> {
        None
    }

    /// Watts phase C
    fn wph_c(&self) -> Option<f32> {
        None
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        None
    }

    /// VA phase A
    fn v_aph_a(&self) -> Option<f32> {
        None
    }

    /// VA phase B
    fn v_aph_b(&self) -> Option<f32> {
        None
    }

    /// VA phase C
    fn v_aph_c(&self) -> Option<f32> {
        None
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<f32> {
        None
    }

    /// VAR phase A
    fn va_rph_a(&self) -> Option<f32> {
        None
    }

    /// VAR phase B
    fn va_rph_b(&self) -> Option<f32> {
        None
    }

    /// VAR phase C
    fn va_rph_c(&self) -> Option<f32> {
        None
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<f32> {
        None
    }

    /// PF phase A
    fn p_fph_a(&self) -> Option<f32> {
        None
    }

    /// PF phase B
    fn p_fph_b(&self) -> Option<f32> {
        None
    }

    /// PF phase C
    fn p_fph_c(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn tot_wh_exp(&self) -> f32;

    /// Total Watt-hours Exported phase A
    fn tot_wh_exp_ph_a(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Exported phase B
    fn tot_wh_exp_ph_b(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Exported phase C
    fn tot_wh_exp_ph_c(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn tot_wh_imp(&self) -> f32;

    /// Total Watt-hours Imported phase A
    fn tot_wh_imp_ph_a(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Imported phase B
    fn tot_wh_imp_ph_b(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Imported phase C
    fn tot_wh_imp_ph_c(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn tot_v_ah_exp(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Exported phase A
    fn tot_v_ah_exp_ph_a(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Exported phase B
    fn tot_v_ah_exp_ph_b(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Exported phase C
    fn tot_v_ah_exp_ph_c(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn tot_v_ah_imp(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Imported phase A
    fn tot_v_ah_imp_ph_a(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Imported phase B
    fn tot_v_ah_imp_ph_b(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Imported phase C
    fn tot_v_ah_imp_ph_c(&self) -> Option<f32> {
        None
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn tot_v_arh_imp_q1(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase A
    fn tot_v_arh_imp_q1ph_a(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase B
    fn tot_v_arh_imp_q1ph_b(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase C
    fn tot_v_arh_imp_q1ph_c(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn tot_v_arh_imp_q2(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase A
    fn tot_v_arh_imp_q2ph_a(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase B
    fn tot_v_arh_imp_q2ph_b(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase C
    fn tot_v_arh_imp_q2ph_c(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn tot_v_arh_exp_q3(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase A
    fn tot_v_arh_exp_q3ph_a(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase B
    fn tot_v_arh_exp_q3ph_b(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase C
    fn tot_v_arh_exp_q3ph_c(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn tot_v_arh_exp_q4(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn tot_v_arh_exp_q4ph_a(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn tot_v_arh_exp_q4ph_b(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn tot_v_arh_exp_q4ph_c(&self) -> Option<f32> {
        None
    }

    /// Events
    ///
    /// Meter Event Flags
    fn evt(&self) -> u32;
}
