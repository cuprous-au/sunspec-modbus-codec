use heapless::String;

pub type Model64410 = DcSimInterface;

/// A generic DC simulator/power supply control interface for DER electrical testing.
pub struct DcSimInterface {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length.
    l: u16,
    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    v_max_lim: Option<u16>,
    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    p_max_lim: Option<u16>,
    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    i_max_lim: Option<u16>,
    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    mode: Option<Mode>,
    /// Power On/Off
    ///
    /// Power On/Off
    ena: Option<Ena>,
    /// Reset Device
    ///
    /// Reset Device
    reset: Option<Reset>,
    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    v_set: Option<u16>,
    /// Power Setpoint
    ///
    /// Power Setpoint
    p_set: Option<u16>,
    /// Current Setpoint
    ///
    /// Current Setpoint
    i_set: Option<u16>,
    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    en50530: Option<En50530>,
    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    vmpp: Option<u16>,
    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    pmpp: Option<u16>,
    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    g_set: Option<u16>,
    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    v_slew_rate: Option<u16>,
    /// Power Slew Rate
    ///
    /// Power Slew Rate
    p_slew_rate: Option<u16>,
    /// Current Slew Rate
    ///
    /// Current Slew Rate
    i_slew_rate: Option<u16>,
    /// Enable Profile
    ///
    /// Start/Stop the Profile
    ena_prof: Option<EnaProf>,
    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    adpt_prof_req: Option<u16>,
    /// Adopt Profile Result
    ///
    /// Result of last adopt profile operation.
    adpt_prof_rslt: AdptProfRslt,
    /// Measured Voltage
    ///
    /// Measured Voltage
    v: Option<i32>,
    /// Measured Power
    ///
    /// Measured Power
    p: Option<i32>,
    /// Measured Current
    ///
    /// Measured Current
    i: Option<i32>,
    /// Errors
    ///
    /// Error States
    errors: Option<String<64>>,
    /// Number Of Points
    ///
    /// Number of profile points supported.
    n_pt: u16,
    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    n_prof: u16,
    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    w_sf: u16,
    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    v_sf: u16,
    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    a_sf: u16,
    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    g_sf: u16,
    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    tms_sf: u16,
    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    v_slew_sf: u16,
    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    p_slew_sf: u16,
    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    i_slew_sf: u16,
    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    pct_sf: u16,
}

trait DcSimInterfaceTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length.
    fn l(&self) -> u16;

    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    fn v_max_lim(&self) -> Option<u16> {
        None
    }

    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    fn set_v_max_lim(&mut self, value: u16) {}

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn p_max_lim(&self) -> Option<u16> {
        None
    }

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn set_p_max_lim(&mut self, value: u16) {}

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn i_max_lim(&self) -> Option<u16> {
        None
    }

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn set_i_max_lim(&mut self, value: u16) {}

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn mode(&self) -> Option<Mode> {
        None
    }

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn set_mode(&mut self, value: Mode) {}

    /// Power On/Off
    ///
    /// Power On/Off
    fn ena(&self) -> Option<Ena> {
        None
    }

    /// Power On/Off
    ///
    /// Power On/Off
    fn set_ena(&mut self, value: Ena) {}

    /// Reset Device
    ///
    /// Reset Device
    fn reset(&self) -> Option<Reset> {
        None
    }

    /// Reset Device
    ///
    /// Reset Device
    fn set_reset(&mut self, value: Reset) {}

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn v_set(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn set_v_set(&mut self, value: u16) {}

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn p_set(&self) -> Option<u16> {
        None
    }

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn set_p_set(&mut self, value: u16) {}

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn i_set(&self) -> Option<u16> {
        None
    }

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn set_i_set(&mut self, value: u16) {}

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn en50530(&self) -> Option<En50530> {
        None
    }

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn set_en50530(&mut self, value: En50530) {}

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn vmpp(&self) -> Option<u16> {
        None
    }

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn set_vmpp(&mut self, value: u16) {}

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn pmpp(&self) -> Option<u16> {
        None
    }

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn set_pmpp(&mut self, value: u16) {}

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn g_set(&self) -> Option<u16> {
        None
    }

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn set_g_set(&mut self, value: u16) {}

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn v_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn set_v_slew_rate(&mut self, value: u16) {}

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn p_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn set_p_slew_rate(&mut self, value: u16) {}

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn i_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn set_i_slew_rate(&mut self, value: u16) {}

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn ena_prof(&self) -> Option<EnaProf> {
        None
    }

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn set_ena_prof(&mut self, value: EnaProf) {}

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn adpt_prof_req(&self) -> Option<u16> {
        None
    }

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn set_adpt_prof_req(&mut self, value: u16) {}

    /// Adopt Profile Result
    ///
    /// Result of last adopt profile operation.
    fn adpt_prof_rslt(&self) -> AdptProfRslt;

    /// Measured Voltage
    ///
    /// Measured Voltage
    fn v(&self) -> Option<i32> {
        None
    }

    /// Measured Power
    ///
    /// Measured Power
    fn p(&self) -> Option<i32> {
        None
    }

    /// Measured Current
    ///
    /// Measured Current
    fn i(&self) -> Option<i32> {
        None
    }

    /// Errors
    ///
    /// Error States
    fn errors(&self) -> Option<String<64>> {
        None
    }

    /// Number Of Points
    ///
    /// Number of profile points supported.
    fn n_pt(&self) -> u16;

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn n_prof(&self) -> u16;

    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    fn w_sf(&self) -> u16;

    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    fn set_w_sf(&mut self, value: u16);

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn v_sf(&self) -> u16;

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn set_v_sf(&mut self, value: u16);

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn a_sf(&self) -> u16;

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn set_a_sf(&mut self, value: u16);

    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    fn g_sf(&self) -> u16;

    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    fn set_g_sf(&mut self, value: u16);

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn tms_sf(&self) -> u16;

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn set_tms_sf(&mut self, value: u16);

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn v_slew_sf(&self) -> u16;

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn set_v_slew_sf(&mut self, value: u16);

    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    fn p_slew_sf(&self) -> u16;

    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    fn set_p_slew_sf(&mut self, value: u16);

    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    fn i_slew_sf(&self) -> u16;

    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    fn set_i_slew_sf(&mut self, value: u16);

    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    fn pct_sf(&self) -> u16;

    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    fn set_pct_sf(&mut self, value: u16);
}

pub enum Mode {
    /// CV Mode
    ///
    /// Constant Voltage (CV) Mode
    Cv = 0,
    /// CC Mode
    ///
    /// Constant Current (CC) Mode.
    Cc = 1,
}

pub enum Ena {
    /// Power On
    ///
    /// Power On
    On = 1,
    /// Power Off
    ///
    /// Power Off
    Off = 0,
}

pub enum Reset {
    /// Reset Device
    ///
    /// Reset Device
    Reset = 1,
    /// Do Not Reset Device
    ///
    /// Do Not Reset Device
    DoNotReset = 0,
}

pub enum En50530 {
    /// EN50530 Mode
    ///
    /// EN50530 Mode
    En50530 = 1,
    /// Do Not Use EN50530 Mode
    ///
    /// Do Not Use EN50530 Mode
    DoNotEn50530 = 0,
}

pub enum EnaProf {
    /// Start Profile
    ///
    /// Start the Profile
    Start = 1,
    /// Stop Profile
    ///
    /// Stop the Profile
    Stop = 0,
}

pub enum AdptProfRslt {
    /// Update In Progress
    ///
    /// Profile update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Profile update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Profile update failed.
    Failed = 2,
}
