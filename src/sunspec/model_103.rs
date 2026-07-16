use crate::util::{RegisterReader, write_i16, write_string, write_u16, write_u32};
use heapless::Vec;

pub type Model103 = InverterThreePhase;

/// Include this model for three phase inverter monitoring
pub struct InverterThreePhase {
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
    ///
    /// Sum of active phases
    a: u16,
    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    aph_a: u16,
    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Connected Phase
    aph_b: u16,
    /// Amps PhaseC
    ///
    /// Phase C Current
    ///
    /// Connected Phase
    aph_c: u16,
    a_sf: u16,
    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    pp_vph_ab: Option<u16>,
    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    pp_vph_bc: Option<u16>,
    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    pp_vph_ca: Option<u16>,
    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ph_vph_a: u16,
    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    ph_vph_b: u16,
    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    ph_vph_c: u16,
    v_sf: u16,
    /// Watts
    ///
    /// AC Power
    w: i16,
    w_sf: u16,
    /// Hz
    ///
    /// Line Frequency
    hz: u16,
    hz_sf: u16,
    /// VA
    ///
    /// AC Apparent Power
    va: Option<i16>,
    va_sf: Option<u16>,
    /// VAr
    ///
    /// AC Reactive Power
    v_ar: Option<i16>,
    v_ar_sf: Option<u16>,
    /// PF
    ///
    /// AC Power Factor
    pf: Option<i16>,
    pf_sf: Option<u16>,
    /// WattHours
    ///
    /// AC Energy
    wh: u32,
    wh_sf: u16,
    /// DC Amps
    ///
    /// DC Current
    dca: Option<u16>,
    dca_sf: Option<u16>,
    /// DC Voltage
    ///
    /// DC Voltage
    dcv: Option<u16>,
    dcv_sf: Option<u16>,
    /// DC Watts
    ///
    /// DC Power
    dcw: Option<i16>,
    dcw_sf: Option<u16>,
    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    tmp_cab: i16,
    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    tmp_snk: Option<i16>,
    /// Transformer Temperature
    ///
    /// Transformer Temperature
    tmp_trns: Option<i16>,
    /// Other Temperature
    ///
    /// Other Temperature
    tmp_ot: Option<i16>,
    tmp_sf: u16,
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

// BELOW HAS BEEN MANUALLY EDITED TO ESTABLISH THE PATTERN
// CODEGEN YET TO BE UPDATED
pub fn register_readers() -> Vec<(usize, RegisterReader<dyn ModelAdapter>), 45> {
    let mut vec: Vec<(usize, RegisterReader<dyn ModelAdapter>), 45> = Vec::new();
    vec.extend_from_slice(&[
        // Model ID
        (1, |_, buf, _, _| write_u16(103_u16, buf)),
        // Model Length
        (1, |_, buf, _, _| write_u16(55_u16, buf)),
        // Amps
        (1, |model, buf, _, _| write_u16(model.a(), buf)),
        // Amps PhaseA
        (1, |model, buf, _, _| write_u16(model.aph_a(), buf)),
        // Amps PhaseB
        (1, |model, buf, _, _| write_u16(model.aph_b(), buf)),
        // Amps PhaseC
        (1, |model, buf, _, _| write_u16(model.aph_c(), buf)),
        (1, |model, buf, _, _| write_u16(model.a_sf(), buf)),
        // Phase Voltage AB
        (1, |model, buf, _, _| {
            if let Some(value) = model.pp_vph_ab() {
                write_u16(value, buf);
            };
        }),
        // Phase Voltage BC
        (1, |model, buf, _, _| {
            if let Some(value) = model.pp_vph_bc() {
                write_u16(value, buf);
            };
        }),
        // Phase Voltage CA
        (1, |model, buf, _, _| {
            if let Some(value) = model.pp_vph_ca() {
                write_u16(value, buf);
            };
        }),
        // Phase Voltage AN
        (1, |model, buf, _, _| write_u16(model.ph_vph_a(), buf)),
        // Phase Voltage BN
        (1, |model, buf, _, _| write_u16(model.ph_vph_b(), buf)),
        // Phase Voltage CN
        (1, |model, buf, _, _| write_u16(model.ph_vph_c(), buf)),
        (1, |model, buf, _, _| write_u16(model.v_sf(), buf)),
        // Watts
        (1, |model, buf, _, _| write_i16(model.w(), buf)),
        (1, |model, buf, _, _| write_u16(model.w_sf(), buf)),
        // Hz
        (1, |model, buf, _, _| write_u16(model.hz(), buf)),
        (1, |model, buf, _, _| write_u16(model.hz_sf(), buf)),
        // VA
        (1, |model, buf, _, _| {
            if let Some(value) = model.va() {
                write_i16(value, buf);
            };
        }),
        (1, |model, buf, _, _| {
            if let Some(value) = model.va_sf() {
                write_u16(value, buf);
            };
        }),
        // VAr
        (1, |model, buf, _, _| {
            if let Some(value) = model.v_ar() {
                write_i16(value, buf);
            };
        }),
        (1, |model, buf, _, _| {
            if let Some(value) = model.v_ar_sf() {
                write_u16(value, buf);
            };
        }),
        // PF
        (1, |model, buf, _, _| {
            if let Some(value) = model.pf() {
                write_i16(value, buf);
            };
        }),
        (1, |model, buf, _, _| {
            if let Some(value) = model.pf_sf() {
                write_u16(value, buf);
            };
        }),
        // WattHours
        (2, |model, buf, offset, limit| {
            write_u32(model.wh(), buf, offset, limit)
        }),
        (1, |model, buf, _, _| write_u16(model.wh_sf(), buf)),
        // DC Amps
        (1, |model, buf, _, _| {
            if let Some(value) = model.dca() {
                write_u16(value, buf);
            };
        }),
        (1, |model, buf, _, _| {
            if let Some(value) = model.dca_sf() {
                write_u16(value, buf);
            };
        }),
        // DC Voltage
        (1, |model, buf, _, _| {
            if let Some(value) = model.dcv() {
                write_u16(value, buf);
            };
        }),
        (1, |model, buf, _, _| {
            if let Some(value) = model.dcv_sf() {
                write_u16(value, buf);
            };
        }),
        // DC Watts
        (1, |model, buf, _, _| {
            if let Some(value) = model.dcw() {
                write_i16(value, buf);
            };
        }),
        (1, |model, buf, _, _| {
            if let Some(value) = model.dcw_sf() {
                write_u16(value, buf);
            };
        }),
        // Cabinet Temperature
        (1, |model, buf, _, _| write_i16(model.tmp_cab(), buf)),
        // Heat Sink Temperature
        (1, |model, buf, _, _| {
            if let Some(value) = model.tmp_snk() {
                write_i16(value, buf);
            };
        }),
        // Transformer Temperature
        (1, |model, buf, _, _| {
            if let Some(value) = model.tmp_trns() {
                write_i16(value, buf);
            };
        }),
        // Other Temperature
        (1, |model, buf, _, _| {
            if let Some(value) = model.tmp_ot() {
                write_i16(value, buf);
            };
        }),
        (1, |model, buf, _, _| write_u16(model.tmp_sf(), buf)),
        // Operating State
        (1, |model, buf, _, _| write_u16(model.st() as u16, buf)),
        // Vendor Operating State
        (1, |model, buf, _, _| {
            if let Some(value) = model.st_vnd() {
                write_u16(value as u16, buf);
            };
        }),
        // Event1
        (2, |model, buf, offset, limit| {
            write_u32(model.evt1(), buf, offset, limit)
        }),
        // Event Bitfield 2
        (2, |model, buf, offset, limit| {
            write_u32(model.evt2(), buf, offset, limit)
        }),
        // Vendor Event Bitfield 1
        (2, |model, buf, offset, limit| {
            if let Some(value) = model.evt_vnd1() {
                write_u32(value, buf, offset, limit);
            };
        }),
        // Vendor Event Bitfield 2
        (2, |model, buf, offset, limit| {
            if let Some(value) = model.evt_vnd2() {
                write_u32(value, buf, offset, limit);
            };
        }),
        // Vendor Event Bitfield 3
        (2, |model, buf, offset, limit| {
            if let Some(value) = model.evt_vnd3() {
                write_u32(value, buf, offset, limit);
            };
        }),
        // Vendor Event Bitfield 4
        (2, |model, buf, offset, limit| {
            if let Some(value) = model.evt_vnd4() {
                write_u32(value, buf, offset, limit);
            };
        }),
    ])
    .unwrap();
    vec
}

pub fn size() -> u16 {
    57
}

pub trait ModelAdapter {
    /// Model ID
    ///
    /// Model identifier
    // fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    // fn l(&self) -> u16;

    /// Amps
    ///
    /// AC Current
    ///
    /// Sum of active phases
    fn a(&self) -> u16;

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn aph_a(&self) -> u16;

    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Connected Phase
    fn aph_b(&self) -> u16;

    /// Amps PhaseC
    ///
    /// Phase C Current
    ///
    /// Connected Phase
    fn aph_c(&self) -> u16;

    fn a_sf(&self) -> u16;

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn pp_vph_ab(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn pp_vph_bc(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn pp_vph_ca(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn ph_vph_a(&self) -> u16;

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn ph_vph_b(&self) -> u16;

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn ph_vph_c(&self) -> u16;

    fn v_sf(&self) -> u16;

    /// Watts
    ///
    /// AC Power
    fn w(&self) -> i16;

    fn w_sf(&self) -> u16;

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> u16;

    fn hz_sf(&self) -> u16;

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        None
    }

    fn va_sf(&self) -> Option<u16> {
        None
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<i16> {
        None
    }

    fn v_ar_sf(&self) -> Option<u16> {
        None
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<i16> {
        None
    }

    fn pf_sf(&self) -> Option<u16> {
        None
    }

    /// WattHours
    ///
    /// AC Energy
    fn wh(&self) -> u32;

    fn wh_sf(&self) -> u16;

    /// DC Amps
    ///
    /// DC Current
    fn dca(&self) -> Option<u16> {
        None
    }

    fn dca_sf(&self) -> Option<u16> {
        None
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dcv(&self) -> Option<u16> {
        None
    }

    fn dcv_sf(&self) -> Option<u16> {
        None
    }

    /// DC Watts
    ///
    /// DC Power
    fn dcw(&self) -> Option<i16> {
        None
    }

    fn dcw_sf(&self) -> Option<u16> {
        None
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn tmp_cab(&self) -> i16;

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn tmp_snk(&self) -> Option<i16> {
        None
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn tmp_trns(&self) -> Option<i16> {
        None
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn tmp_ot(&self) -> Option<i16> {
        None
    }

    fn tmp_sf(&self) -> u16;

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
