use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 32;

pub static POINTS: [ReadablePoint; 32] = [
    ReadablePoint {
        reference: PointReference::Static { value: 121 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 30 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 { point: Point::WMax },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 { point: Point::VRef },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VRefOfs,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 { point: Point::VMax },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 { point: Point::VMin },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VaMax,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VArMaxQ1,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VArMaxQ2,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VArMaxQ3,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VArMaxQ4,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 { point: Point::WGra },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::PfMinQ1,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::PfMinQ2,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::PfMinQ3,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::PfMinQ4,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VArAct,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::ClcTotVa,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::MaxRmpRte,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::EcpNomHz,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::ConnPh,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::WMaxSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VRefSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VRefOfsSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VMinMaxSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VaMaxSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::VArMaxSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::WGraSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::PfMinSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::MaxRmpRteSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model121 {
            point: Point::EcpNomHzSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    WMax,
    VRef,
    VRefOfs,
    VMax,
    VMin,
    VaMax,
    VArMaxQ1,
    VArMaxQ2,
    VArMaxQ3,
    VArMaxQ4,
    WGra,
    PfMinQ1,
    PfMinQ2,
    PfMinQ3,
    PfMinQ4,
    VArAct,
    ClcTotVa,
    MaxRmpRte,
    EcpNomHz,
    ConnPh,
    WMaxSf,
    VRefSf,
    VRefOfsSf,
    VMinMaxSf,
    VaMaxSf,
    VArMaxSf,
    WGraSf,
    PfMinSf,
    MaxRmpRteSf,
    EcpNomHzSf,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::WMax => serialisation::write_u16(model.w_max(), buffer),
        Point::VRef => serialisation::write_u16(model.v_ref(), buffer),
        Point::VRefOfs => serialisation::write_i16(model.v_ref_ofs(), buffer),
        Point::VMax => {
            if let Some(value) = model.v_max() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VMin => {
            if let Some(value) = model.v_min() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VaMax => {
            if let Some(value) = model.va_max() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VArMaxQ1 => {
            if let Some(value) = model.v_ar_max_q1() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArMaxQ2 => {
            if let Some(value) = model.v_ar_max_q2() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArMaxQ3 => {
            if let Some(value) = model.v_ar_max_q3() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArMaxQ4 => {
            if let Some(value) = model.v_ar_max_q4() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WGra => {
            if let Some(value) = model.w_gra() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PfMinQ1 => {
            if let Some(value) = model.pf_min_q1() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfMinQ2 => {
            if let Some(value) = model.pf_min_q2() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfMinQ3 => {
            if let Some(value) = model.pf_min_q3() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfMinQ4 => {
            if let Some(value) = model.pf_min_q4() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArAct => {
            if let Some(value) = model.v_ar_act() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::ClcTotVa => {
            if let Some(value) = model.clc_tot_va() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MaxRmpRte => {
            if let Some(value) = model.max_rmp_rte() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::EcpNomHz => {
            if let Some(value) = model.ecp_nom_hz() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ConnPh => {
            if let Some(value) = model.conn_ph() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::WMaxSf => serialisation::write_u16(model.w_max_sf(), buffer),
        Point::VRefSf => serialisation::write_u16(model.v_ref_sf(), buffer),
        Point::VRefOfsSf => serialisation::write_u16(model.v_ref_ofs_sf(), buffer),
        Point::VMinMaxSf => {
            if let Some(value) = model.v_min_max_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VaMaxSf => {
            if let Some(value) = model.va_max_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VArMaxSf => {
            if let Some(value) = model.v_ar_max_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WGraSf => {
            if let Some(value) = model.w_gra_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PfMinSf => {
            if let Some(value) = model.pf_min_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaxRmpRteSf => {
            if let Some(value) = model.max_rmp_rte_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::EcpNomHzSf => {
            if let Some(value) = model.ecp_nom_hz_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
    }
}

pub trait ModelAdapter {
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

#[repr(C)]
pub struct Model121CallbackAdapter {
    w_max_callback: extern "C" fn() -> u16,
    set_w_max_callback: extern "C" fn(u16),
    v_ref_callback: extern "C" fn() -> u16,
    set_v_ref_callback: extern "C" fn(u16),
    v_ref_ofs_callback: extern "C" fn() -> i16,
    set_v_ref_ofs_callback: extern "C" fn(i16),
    v_max_callback: Option<extern "C" fn() -> u16>,
    set_v_max_callback: Option<extern "C" fn(u16)>,
    v_min_callback: Option<extern "C" fn() -> u16>,
    set_v_min_callback: Option<extern "C" fn(u16)>,
    va_max_callback: Option<extern "C" fn() -> u16>,
    set_va_max_callback: Option<extern "C" fn(u16)>,
    v_ar_max_q1_callback: Option<extern "C" fn() -> i16>,
    set_v_ar_max_q1_callback: Option<extern "C" fn(i16)>,
    v_ar_max_q2_callback: Option<extern "C" fn() -> i16>,
    set_v_ar_max_q2_callback: Option<extern "C" fn(i16)>,
    v_ar_max_q3_callback: Option<extern "C" fn() -> i16>,
    set_v_ar_max_q3_callback: Option<extern "C" fn(i16)>,
    v_ar_max_q4_callback: Option<extern "C" fn() -> i16>,
    set_v_ar_max_q4_callback: Option<extern "C" fn(i16)>,
    w_gra_callback: Option<extern "C" fn() -> u16>,
    set_w_gra_callback: Option<extern "C" fn(u16)>,
    pf_min_q1_callback: Option<extern "C" fn() -> i16>,
    set_pf_min_q1_callback: Option<extern "C" fn(i16)>,
    pf_min_q2_callback: Option<extern "C" fn() -> i16>,
    set_pf_min_q2_callback: Option<extern "C" fn(i16)>,
    pf_min_q3_callback: Option<extern "C" fn() -> i16>,
    set_pf_min_q3_callback: Option<extern "C" fn(i16)>,
    pf_min_q4_callback: Option<extern "C" fn() -> i16>,
    set_pf_min_q4_callback: Option<extern "C" fn(i16)>,
    v_ar_act_callback: Option<extern "C" fn() -> VArAct>,
    set_v_ar_act_callback: Option<extern "C" fn(VArAct)>,
    clc_tot_va_callback: Option<extern "C" fn() -> ClcTotVa>,
    set_clc_tot_va_callback: Option<extern "C" fn(ClcTotVa)>,
    max_rmp_rte_callback: Option<extern "C" fn() -> u16>,
    set_max_rmp_rte_callback: Option<extern "C" fn(u16)>,
    ecp_nom_hz_callback: Option<extern "C" fn() -> u16>,
    set_ecp_nom_hz_callback: Option<extern "C" fn(u16)>,
    conn_ph_callback: Option<extern "C" fn() -> ConnPh>,
    set_conn_ph_callback: Option<extern "C" fn(ConnPh)>,
    w_max_sf_callback: extern "C" fn() -> u16,
    v_ref_sf_callback: extern "C" fn() -> u16,
    v_ref_ofs_sf_callback: extern "C" fn() -> u16,
    v_min_max_sf_callback: Option<extern "C" fn() -> u16>,
    va_max_sf_callback: Option<extern "C" fn() -> u16>,
    v_ar_max_sf_callback: Option<extern "C" fn() -> u16>,
    w_gra_sf_callback: Option<extern "C" fn() -> u16>,
    pf_min_sf_callback: Option<extern "C" fn() -> u16>,
    max_rmp_rte_sf_callback: Option<extern "C" fn() -> u16>,
    ecp_nom_hz_sf_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model121CallbackAdapter {
    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    fn w_max(&self) -> u16 {
        (self.w_max_callback)()
    }

    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    fn set_w_max(&mut self, value: u16) {
        (self.set_w_max_callback)(value);
    }

    /// VRef
    ///
    /// Voltage at the PCC.
    fn v_ref(&self) -> u16 {
        (self.v_ref_callback)()
    }

    /// VRef
    ///
    /// Voltage at the PCC.
    fn set_v_ref(&mut self, value: u16) {
        (self.set_v_ref_callback)(value);
    }

    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    fn v_ref_ofs(&self) -> i16 {
        (self.v_ref_ofs_callback)()
    }

    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    fn set_v_ref_ofs(&mut self, value: i16) {
        (self.set_v_ref_ofs_callback)(value);
    }

    /// VMax
    ///
    /// Setpoint for maximum voltage.
    fn v_max(&self) -> Option<u16> {
        self.v_max_callback.map(|callback| (callback)())
    }

    /// VMax
    ///
    /// Setpoint for maximum voltage.
    fn set_v_max(&mut self, value: u16) {
        if let Some(callback) = self.set_v_max_callback {
            (callback)(value);
        };
    }

    /// VMin
    ///
    /// Setpoint for minimum voltage.
    fn v_min(&self) -> Option<u16> {
        self.v_min_callback.map(|callback| (callback)())
    }

    /// VMin
    ///
    /// Setpoint for minimum voltage.
    fn set_v_min(&mut self, value: u16) {
        if let Some(callback) = self.set_v_min_callback {
            (callback)(value);
        };
    }

    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    fn va_max(&self) -> Option<u16> {
        self.va_max_callback.map(|callback| (callback)())
    }

    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    fn set_va_max(&mut self, value: u16) {
        if let Some(callback) = self.set_va_max_callback {
            (callback)(value);
        };
    }

    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    fn v_ar_max_q1(&self) -> Option<i16> {
        self.v_ar_max_q1_callback.map(|callback| (callback)())
    }

    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    fn set_v_ar_max_q1(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_q1_callback {
            (callback)(value);
        };
    }

    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    fn v_ar_max_q2(&self) -> Option<i16> {
        self.v_ar_max_q2_callback.map(|callback| (callback)())
    }

    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    fn set_v_ar_max_q2(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_q2_callback {
            (callback)(value);
        };
    }

    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    fn v_ar_max_q3(&self) -> Option<i16> {
        self.v_ar_max_q3_callback.map(|callback| (callback)())
    }

    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    fn set_v_ar_max_q3(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_q3_callback {
            (callback)(value);
        };
    }

    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    fn v_ar_max_q4(&self) -> Option<i16> {
        self.v_ar_max_q4_callback.map(|callback| (callback)())
    }

    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    fn set_v_ar_max_q4(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_q4_callback {
            (callback)(value);
        };
    }

    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    fn w_gra(&self) -> Option<u16> {
        self.w_gra_callback.map(|callback| (callback)())
    }

    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    fn set_w_gra(&mut self, value: u16) {
        if let Some(callback) = self.set_w_gra_callback {
            (callback)(value);
        };
    }

    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    fn pf_min_q1(&self) -> Option<i16> {
        self.pf_min_q1_callback.map(|callback| (callback)())
    }

    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    fn set_pf_min_q1(&mut self, value: i16) {
        if let Some(callback) = self.set_pf_min_q1_callback {
            (callback)(value);
        };
    }

    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    fn pf_min_q2(&self) -> Option<i16> {
        self.pf_min_q2_callback.map(|callback| (callback)())
    }

    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    fn set_pf_min_q2(&mut self, value: i16) {
        if let Some(callback) = self.set_pf_min_q2_callback {
            (callback)(value);
        };
    }

    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    fn pf_min_q3(&self) -> Option<i16> {
        self.pf_min_q3_callback.map(|callback| (callback)())
    }

    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    fn set_pf_min_q3(&mut self, value: i16) {
        if let Some(callback) = self.set_pf_min_q3_callback {
            (callback)(value);
        };
    }

    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    fn pf_min_q4(&self) -> Option<i16> {
        self.pf_min_q4_callback.map(|callback| (callback)())
    }

    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    fn set_pf_min_q4(&mut self, value: i16) {
        if let Some(callback) = self.set_pf_min_q4_callback {
            (callback)(value);
        };
    }

    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    fn v_ar_act(&self) -> Option<VArAct> {
        self.v_ar_act_callback.map(|callback| (callback)())
    }

    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    fn set_v_ar_act(&mut self, value: VArAct) {
        if let Some(callback) = self.set_v_ar_act_callback {
            (callback)(value);
        };
    }

    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    fn clc_tot_va(&self) -> Option<ClcTotVa> {
        self.clc_tot_va_callback.map(|callback| (callback)())
    }

    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    fn set_clc_tot_va(&mut self, value: ClcTotVa) {
        if let Some(callback) = self.set_clc_tot_va_callback {
            (callback)(value);
        };
    }

    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    fn max_rmp_rte(&self) -> Option<u16> {
        self.max_rmp_rte_callback.map(|callback| (callback)())
    }

    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    fn set_max_rmp_rte(&mut self, value: u16) {
        if let Some(callback) = self.set_max_rmp_rte_callback {
            (callback)(value);
        };
    }

    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    fn ecp_nom_hz(&self) -> Option<u16> {
        self.ecp_nom_hz_callback.map(|callback| (callback)())
    }

    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    fn set_ecp_nom_hz(&mut self, value: u16) {
        if let Some(callback) = self.set_ecp_nom_hz_callback {
            (callback)(value);
        };
    }

    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    fn conn_ph(&self) -> Option<ConnPh> {
        self.conn_ph_callback.map(|callback| (callback)())
    }

    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    fn set_conn_ph(&mut self, value: ConnPh) {
        if let Some(callback) = self.set_conn_ph_callback {
            (callback)(value);
        };
    }

    /// WMax_SF
    ///
    /// Scale factor for real power.
    fn w_max_sf(&self) -> u16 {
        (self.w_max_sf_callback)()
    }

    /// VRef_SF
    ///
    /// Scale factor for voltage at the PCC.
    fn v_ref_sf(&self) -> u16 {
        (self.v_ref_sf_callback)()
    }

    /// VRefOfs_SF
    ///
    /// Scale factor for offset voltage.
    fn v_ref_ofs_sf(&self) -> u16 {
        (self.v_ref_ofs_sf_callback)()
    }

    /// VMinMax_SF
    ///
    /// Scale factor for min/max voltages.
    fn v_min_max_sf(&self) -> Option<u16> {
        self.v_min_max_sf_callback.map(|callback| (callback)())
    }

    /// VAMax_SF
    ///
    /// Scale factor for apparent power.
    fn va_max_sf(&self) -> Option<u16> {
        self.va_max_sf_callback.map(|callback| (callback)())
    }

    /// VArMax_SF
    ///
    /// Scale factor for reactive power.
    fn v_ar_max_sf(&self) -> Option<u16> {
        self.v_ar_max_sf_callback.map(|callback| (callback)())
    }

    /// WGra_SF
    ///
    /// Scale factor for default ramp rate.
    fn w_gra_sf(&self) -> Option<u16> {
        self.w_gra_sf_callback.map(|callback| (callback)())
    }

    /// PFMin_SF
    ///
    /// Scale factor for minimum power factor.
    fn pf_min_sf(&self) -> Option<u16> {
        self.pf_min_sf_callback.map(|callback| (callback)())
    }

    /// MaxRmpRte_SF
    ///
    /// Scale factor for maximum ramp percentage.
    fn max_rmp_rte_sf(&self) -> Option<u16> {
        self.max_rmp_rte_sf_callback.map(|callback| (callback)())
    }

    /// ECPNomHz_SF
    ///
    /// Scale factor for nominal frequency.
    fn ecp_nom_hz_sf(&self) -> Option<u16> {
        self.ecp_nom_hz_sf_callback.map(|callback| (callback)())
    }
}
