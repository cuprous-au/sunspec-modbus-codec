pub type Model112 = InverterSplitPhaseFloat;

/// Include this model for split phase inverter monitoring using float values
pub struct InverterSplitPhaseFloat {
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
    /// AC Current
    a: f32,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    aph_a: f32,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Connected Phase
    aph_b: f32,
    /// Amps PhaseC
    ///
    /// Phase C Current
    aph_c: Option<f32>,
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
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ph_vph_a: f32,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    ph_vph_b: f32,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    ph_vph_c: Option<f32>,
    /// Watts
    ///
    /// AC Power
    w: f32,
    /// Hz
    ///
    /// Line Frequency
    hz: f32,
    /// VA
    ///
    /// AC Apparent Power
    va: Option<f32>,
    /// VAr
    ///
    /// AC Reactive Power
    v_ar: Option<f32>,
    /// PF
    ///
    /// AC Power Factor
    pf: Option<f32>,
    /// WattHours
    ///
    /// AC Energy
    wh: f32,
    /// DC Amps
    ///
    /// DC Current
    dca: Option<f32>,
    /// DC Voltage
    ///
    /// DC Voltage
    dcv: Option<f32>,
    /// DC Watts
    ///
    /// DC Power
    dcw: Option<f32>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    tmp_cab: f32,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    tmp_snk: Option<f32>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    tmp_trns: Option<f32>,
    /// Other Temperature
    ///
    /// Other Temperature
    tmp_ot: Option<f32>,
    /// Operating State
    ///
    /// Operating state
    st: St,
    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    st_vnd: Option<StVnd>,
    /// Event1
    ///
    /// Event fields
    evt1: u32,
    /// Event Bitfield 2
    ///
    /// Reserved for future use
    evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    evt_vnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    evt_vnd2: Option<u32>,
    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    evt_vnd3: Option<u32>,
    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    evt_vnd4: Option<u32>,
}

trait InverterSplitPhaseFloatTrait {
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
    /// AC Current
    fn a(&self) -> f32;

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn aph_a(&self) -> f32;

    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Connected Phase
    fn aph_b(&self) -> f32;

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn aph_c(&self) -> Option<f32> {
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

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn ph_vph_a(&self) -> f32;

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn ph_vph_b(&self) -> f32;

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn ph_vph_c(&self) -> Option<f32> {
        None
    }

    /// Watts
    ///
    /// AC Power
    fn w(&self) -> f32;

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> f32;

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        None
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<f32> {
        None
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<f32> {
        None
    }

    /// WattHours
    ///
    /// AC Energy
    fn wh(&self) -> f32;

    /// DC Amps
    ///
    /// DC Current
    fn dca(&self) -> Option<f32> {
        None
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dcv(&self) -> Option<f32> {
        None
    }

    /// DC Watts
    ///
    /// DC Power
    fn dcw(&self) -> Option<f32> {
        None
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn tmp_cab(&self) -> f32;

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn tmp_snk(&self) -> Option<f32> {
        None
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn tmp_trns(&self) -> Option<f32> {
        None
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn tmp_ot(&self) -> Option<f32> {
        None
    }

    /// Operating State
    ///
    /// Operating state
    fn st(&self) -> St;

    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    fn st_vnd(&self) -> Option<StVnd> {
        None
    }

    /// Event1
    ///
    /// Event fields
    fn evt1(&self) -> u32;

    /// Event Bitfield 2
    ///
    /// Reserved for future use
    fn evt2(&self) -> u32;

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    fn evt_vnd1(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    fn evt_vnd2(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    fn evt_vnd3(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    fn evt_vnd4(&self) -> Option<u32> {
        None
    }
}

pub enum St {
    Off = 1,
    Sleeping = 2,
    Starting = 3,
    Mppt = 4,
    Throttled = 5,
    ShuttingDown = 6,
    Fault = 7,
    Standby = 8,
}

pub enum StVnd {}
