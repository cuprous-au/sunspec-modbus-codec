use heapless::String;

pub type Model64411 = AcSimInterface;

/// A generic AC simulator/power supply control interface for DER electrical testing.
pub struct AcSimInterface {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length.
    l: u16,
    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    phases: Option<u16>,
    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    phase_angle: Option<u16>,
    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    v_nom: Option<u16>,
    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    v_max: Option<u16>,
    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    i_max: Option<u16>,
    /// Frequency
    ///
    /// Frequency Setpoint
    freq: Option<u16>,
    /// Output State
    ///
    /// AC Output State
    output: Option<Output>,
    /// Relay State
    ///
    /// AC Relay State
    relay: Option<Relay>,
    /// Regeneration State
    ///
    /// Regeneration State
    regen: Option<Regen>,
    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    v_set: Option<u16>,
    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    v_set_a: Option<u16>,
    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    v_set_b: Option<u16>,
    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    v_set_c: Option<u16>,
    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    freq_slew: Option<u16>,
    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    v_slew: Option<u16>,
    /// Measured Voltage Phase A
    ///
    /// Measured Voltage Phase A
    va: Option<i32>,
    /// Measured Voltage Phase B
    ///
    /// Measured Voltage Phase B
    vb: Option<i32>,
    /// Measured Voltage Phase C
    ///
    /// Measured Voltage Phase C
    vc: Option<i32>,
    /// Measured Frequency
    ///
    /// Measured Frequency
    hz: Option<i32>,
    /// Measured Current Phase A
    ///
    /// Measured Current Phase A
    ia: Option<i32>,
    /// Measured Current Phase B
    ///
    /// Measured Current Phase B
    ib: Option<i32>,
    /// Measured Current Phase C
    ///
    /// Measured Current Phase C
    ic: Option<i32>,
    /// Voltage Harmonics Phase A
    ///
    /// Voltage Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    v_har_a: Option<String<300>>,
    /// Voltage Harmonics Phase B
    ///
    /// Voltage Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    v_har_b: Option<String<300>>,
    /// Voltage Harmonics Phase C
    ///
    /// Voltage Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    v_har_c: Option<String<300>>,
    /// Current Harmonics Phase A
    ///
    /// Current Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    i_har_a: Option<String<300>>,
    /// Current Harmonics Phase B
    ///
    /// Current Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    i_har_b: Option<String<300>>,
    /// Current Harmonics Phase C
    ///
    /// Current Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    i_har_c: Option<String<300>>,
    /// Current Interharmonics Phase A
    ///
    /// Current Interharmonics Pct, Phase A (comma seperated string for interharmonics 1-50)
    i_int_har_a: Option<String<300>>,
    /// Current Interharmonics Phase B
    ///
    /// Current Interharmonics Pct, Phase B (comma seperated string for interharmonics 1-50)
    i_int_har_b: Option<String<300>>,
    /// Current Interharmonics Phase C
    ///
    /// Current Interharmonics Pct, Phase C (comma seperated string for interharmonics 1-50)
    i_int_har_c: Option<String<300>>,
    /// Voltage THD Phase A
    ///
    /// Voltage THD Phase A
    v_thd_a: Option<u16>,
    /// Voltage THD Phase B
    ///
    /// Voltage THD Phase B
    v_thd_b: Option<u16>,
    /// Voltage THD Phase C
    ///
    /// Voltage THD Phase C
    v_thd_c: Option<u16>,
    /// Current THD Phase A
    ///
    /// Current THD Phase A
    i_thd_a: Option<u16>,
    /// Current THD Phase B
    ///
    /// Current THD Phase B
    i_thd_b: Option<u16>,
    /// Current THD Phase C
    ///
    /// Current THD Phase C
    i_thd_c: Option<u16>,
    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    ena_prof: Option<EnaProf>,
    /// Profile Result
    ///
    /// Result of last profile operation.
    prof_rslt: ProfRslt,
    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    n_prof: u16,
    /// Max Profile Point Count
    ///
    /// Max profile points in the profiles.
    n_pt: u16,
    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    v_sf: u16,
    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    a_sf: u16,
    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    tms_sf: u16,
    /// Frequency Scale Factor
    ///
    /// Scale factor for frequency points.
    hz_sf: u16,
    /// Frequency Slew Rate Scale Factor
    ///
    /// Scale factor for frequency slew rate.
    hz_slew_sf: u16,
    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    v_slew_sf: u16,
    /// THD Scale Factor
    ///
    /// Scale factor for THD values.
    thd_sf: u16,
}

trait AcSimInterfaceTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length.
    fn l(&self) -> u16;

    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn phases(&self) -> Option<u16> {
        None
    }

    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn set_phases(&mut self, value: u16) {}

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn phase_angle(&self) -> Option<u16> {
        None
    }

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn set_phase_angle(&mut self, value: u16) {}

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn v_nom(&self) -> Option<u16> {
        None
    }

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn set_v_nom(&mut self, value: u16) {}

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn v_max(&self) -> Option<u16> {
        None
    }

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn set_v_max(&mut self, value: u16) {}

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn i_max(&self) -> Option<u16> {
        None
    }

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn set_i_max(&mut self, value: u16) {}

    /// Frequency
    ///
    /// Frequency Setpoint
    fn freq(&self) -> Option<u16> {
        None
    }

    /// Frequency
    ///
    /// Frequency Setpoint
    fn set_freq(&mut self, value: u16) {}

    /// Output State
    ///
    /// AC Output State
    fn output(&self) -> Option<Output> {
        None
    }

    /// Output State
    ///
    /// AC Output State
    fn set_output(&mut self, value: Output) {}

    /// Relay State
    ///
    /// AC Relay State
    fn relay(&self) -> Option<Relay> {
        None
    }

    /// Relay State
    ///
    /// AC Relay State
    fn set_relay(&mut self, value: Relay) {}

    /// Regeneration State
    ///
    /// Regeneration State
    fn regen(&self) -> Option<Regen> {
        None
    }

    /// Regeneration State
    ///
    /// Regeneration State
    fn set_regen(&mut self, value: Regen) {}

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn v_set(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn set_v_set(&mut self, value: u16) {}

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn v_set_a(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn set_v_set_a(&mut self, value: u16) {}

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn v_set_b(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn set_v_set_b(&mut self, value: u16) {}

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn v_set_c(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn set_v_set_c(&mut self, value: u16) {}

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn freq_slew(&self) -> Option<u16> {
        None
    }

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn set_freq_slew(&mut self, value: u16) {}

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn v_slew(&self) -> Option<u16> {
        None
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn set_v_slew(&mut self, value: u16) {}

    /// Measured Voltage Phase A
    ///
    /// Measured Voltage Phase A
    fn va(&self) -> Option<i32> {
        None
    }

    /// Measured Voltage Phase B
    ///
    /// Measured Voltage Phase B
    fn vb(&self) -> Option<i32> {
        None
    }

    /// Measured Voltage Phase C
    ///
    /// Measured Voltage Phase C
    fn vc(&self) -> Option<i32> {
        None
    }

    /// Measured Frequency
    ///
    /// Measured Frequency
    fn hz(&self) -> Option<i32> {
        None
    }

    /// Measured Current Phase A
    ///
    /// Measured Current Phase A
    fn ia(&self) -> Option<i32> {
        None
    }

    /// Measured Current Phase B
    ///
    /// Measured Current Phase B
    fn ib(&self) -> Option<i32> {
        None
    }

    /// Measured Current Phase C
    ///
    /// Measured Current Phase C
    fn ic(&self) -> Option<i32> {
        None
    }

    /// Voltage Harmonics Phase A
    ///
    /// Voltage Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn v_har_a(&self) -> Option<String<300>> {
        None
    }

    /// Voltage Harmonics Phase B
    ///
    /// Voltage Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn v_har_b(&self) -> Option<String<300>> {
        None
    }

    /// Voltage Harmonics Phase C
    ///
    /// Voltage Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn v_har_c(&self) -> Option<String<300>> {
        None
    }

    /// Current Harmonics Phase A
    ///
    /// Current Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn i_har_a(&self) -> Option<String<300>> {
        None
    }

    /// Current Harmonics Phase B
    ///
    /// Current Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn i_har_b(&self) -> Option<String<300>> {
        None
    }

    /// Current Harmonics Phase C
    ///
    /// Current Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn i_har_c(&self) -> Option<String<300>> {
        None
    }

    /// Current Interharmonics Phase A
    ///
    /// Current Interharmonics Pct, Phase A (comma seperated string for interharmonics 1-50)
    fn i_int_har_a(&self) -> Option<String<300>> {
        None
    }

    /// Current Interharmonics Phase B
    ///
    /// Current Interharmonics Pct, Phase B (comma seperated string for interharmonics 1-50)
    fn i_int_har_b(&self) -> Option<String<300>> {
        None
    }

    /// Current Interharmonics Phase C
    ///
    /// Current Interharmonics Pct, Phase C (comma seperated string for interharmonics 1-50)
    fn i_int_har_c(&self) -> Option<String<300>> {
        None
    }

    /// Voltage THD Phase A
    ///
    /// Voltage THD Phase A
    fn v_thd_a(&self) -> Option<u16> {
        None
    }

    /// Voltage THD Phase B
    ///
    /// Voltage THD Phase B
    fn v_thd_b(&self) -> Option<u16> {
        None
    }

    /// Voltage THD Phase C
    ///
    /// Voltage THD Phase C
    fn v_thd_c(&self) -> Option<u16> {
        None
    }

    /// Current THD Phase A
    ///
    /// Current THD Phase A
    fn i_thd_a(&self) -> Option<u16> {
        None
    }

    /// Current THD Phase B
    ///
    /// Current THD Phase B
    fn i_thd_b(&self) -> Option<u16> {
        None
    }

    /// Current THD Phase C
    ///
    /// Current THD Phase C
    fn i_thd_c(&self) -> Option<u16> {
        None
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn ena_prof(&self) -> Option<EnaProf> {
        None
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn set_ena_prof(&mut self, value: EnaProf) {}

    /// Profile Result
    ///
    /// Result of last profile operation.
    fn prof_rslt(&self) -> ProfRslt;

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn n_prof(&self) -> u16;

    /// Max Profile Point Count
    ///
    /// Max profile points in the profiles.
    fn n_pt(&self) -> u16;

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn v_sf(&self) -> u16;

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn a_sf(&self) -> u16;

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn tms_sf(&self) -> u16;

    /// Frequency Scale Factor
    ///
    /// Scale factor for frequency points.
    fn hz_sf(&self) -> u16;

    /// Frequency Slew Rate Scale Factor
    ///
    /// Scale factor for frequency slew rate.
    fn hz_slew_sf(&self) -> u16;

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn v_slew_sf(&self) -> u16;

    /// THD Scale Factor
    ///
    /// Scale factor for THD values.
    fn thd_sf(&self) -> u16;
}

pub enum Output {
    /// Output Off
    Off = 0,
    /// Output On
    On = 1,
}

pub enum Relay {
    /// Relay Open
    Open = 0,
    /// Relay Closed
    Closed = 1,
}

pub enum Regen {
    /// Regen Off
    Off = 0,
    /// Regen On
    On = 1,
}

pub enum EnaProf {
    /// Stop Profile
    Stop = 0,
    /// Start Profile Immediately
    Start = 1,
    /// Start Profile via External Trigger Signal
    Trigger = 2,
}

pub enum ProfRslt {
    /// Profile update in progress.
    InProgress = 0,
    /// Profile update completed successfully.
    Completed = 1,
    /// Profile update failed.
    Failed = 2,
}
