pub type Model220 = AcMeterSecure;

/// Include this model for secure metering
pub struct AcMeterSecure {
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
    /// Current scale factor
    a_sf: u16,
    /// Voltage
    ///
    /// Average phase or line voltage
    ph_v: Option<i16>,
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
    /// Real Power scale factor
    w_sf: u16,
    /// VA
    ///
    /// AC Apparent Power
    va: Option<i16>,
    /// Apparent Power scale factor
    va_sf: Option<u16>,
    /// VAR
    ///
    /// Reactive Power
    var: Option<i16>,
    /// Reactive Power scale factor
    var_sf: Option<u16>,
    /// PF
    ///
    /// Power Factor
    pf: Option<i16>,
    /// Power Factor scale factor
    pf_sf: Option<u16>,
    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    tot_wh_exp: u32,
    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    tot_wh_imp: u32,
    /// Real Energy scale factor
    tot_wh_sf: u16,
    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    tot_v_ah_exp: Option<u32>,
    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    tot_v_ah_imp: Option<u32>,
    /// Apparent Energy scale factor
    tot_v_ah_sf: Option<u16>,
    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    tot_v_arh_imp_q1: Option<u32>,
    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    tot_v_arh_imp_q2: Option<u32>,
    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    tot_v_arh_exp_q3: Option<u32>,
    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    tot_v_arh_exp_q4: Option<u32>,
    /// Reactive Energy scale factor
    tot_v_arh_sf: Option<u16>,
    /// Events
    ///
    /// Meter Event Flags
    evt: u32,
    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    ts: u32,
    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    ms: u16,
    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    seq: u16,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    alg: Alg,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    n: u16,
}

trait AcMeterSecureTrait {
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

    /// Current scale factor
    fn a_sf(&self) -> u16;

    /// Voltage
    ///
    /// Average phase or line voltage
    fn ph_v(&self) -> Option<i16> {
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

    /// Real Power scale factor
    fn w_sf(&self) -> u16;

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
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

    /// Power Factor scale factor
    fn pf_sf(&self) -> Option<u16> {
        None
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn tot_wh_exp(&self) -> u32;

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn tot_wh_imp(&self) -> u32;

    /// Real Energy scale factor
    fn tot_wh_sf(&self) -> u16;

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn tot_v_ah_exp(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn tot_v_ah_imp(&self) -> Option<u32> {
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

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn tot_v_arh_imp_q2(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn tot_v_arh_exp_q3(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn tot_v_arh_exp_q4(&self) -> Option<u32> {
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

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn ts(&self) -> u32;

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn ms(&self) -> u16;

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn seq(&self) -> u16;

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn alg(&self) -> Alg;

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16;
}

pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}
