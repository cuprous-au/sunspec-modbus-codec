use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 32;

static POINTS: [PointDetails<()>; 32] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::WMax,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::VRef,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::VRefOfs,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::VMax,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::VMin,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::VaMax,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::VArMaxQ1,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::VArMaxQ2,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::VArMaxQ3,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::VArMaxQ4,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::WGra,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::PfMinQ1,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::PfMinQ2,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::PfMinQ3,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::PfMinQ4,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::VArAct,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::ClcTotVa,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::MaxRmpRte,
        size: 1,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::EcpNomHz,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::ConnPh,
        size: 1,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::WMaxSf,
        size: 1,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::VRefSf,
        size: 1,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::VRefOfsSf,
        size: 1,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::VMinMaxSf,
        size: 1,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::VaMaxSf,
        size: 1,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::VArMaxSf,
        size: 1,
        start_address: 27,
    },
    PointDetails {
        point: |()| Point::WGraSf,
        size: 1,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::PfMinSf,
        size: 1,
        start_address: 29,
    },
    PointDetails {
        point: |()| Point::MaxRmpRteSf,
        size: 1,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::EcpNomHzSf,
        size: 1,
        start_address: 31,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
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

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    32
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(121, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::WMax => {
            buffer::write_u16(model.w_max(), buffer);
        }
        Point::VRef => {
            buffer::write_u16(model.v_ref(), buffer);
        }
        Point::VRefOfs => {
            buffer::write_i16(model.v_ref_ofs(), buffer);
        }
        Point::VMax => {
            if let Some(value) = model.v_max() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VMin => {
            if let Some(value) = model.v_min() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VaMax => {
            if let Some(value) = model.va_max() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VArMaxQ1 => {
            if let Some(value) = model.v_ar_max_q1() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VArMaxQ2 => {
            if let Some(value) = model.v_ar_max_q2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VArMaxQ3 => {
            if let Some(value) = model.v_ar_max_q3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VArMaxQ4 => {
            if let Some(value) = model.v_ar_max_q4() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::WGra => {
            if let Some(value) = model.w_gra() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::PfMinQ1 => {
            if let Some(value) = model.pf_min_q1() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::PfMinQ2 => {
            if let Some(value) = model.pf_min_q2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::PfMinQ3 => {
            if let Some(value) = model.pf_min_q3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::PfMinQ4 => {
            if let Some(value) = model.pf_min_q4() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VArAct => {
            if let Some(value) = model.v_ar_act() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ClcTotVa => {
            if let Some(value) = model.clc_tot_va() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MaxRmpRte => {
            if let Some(value) = model.max_rmp_rte() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::EcpNomHz => {
            if let Some(value) = model.ecp_nom_hz() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ConnPh => {
            if let Some(value) = model.conn_ph() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::WMaxSf => {
            buffer::write_u16(model.w_max_sf(), buffer);
        }
        Point::VRefSf => {
            buffer::write_u16(model.v_ref_sf(), buffer);
        }
        Point::VRefOfsSf => {
            buffer::write_u16(model.v_ref_ofs_sf(), buffer);
        }
        Point::VMinMaxSf => {
            if let Some(value) = model.v_min_max_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VaMaxSf => {
            if let Some(value) = model.va_max_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VArMaxSf => {
            if let Some(value) = model.v_ar_max_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::WGraSf => {
            if let Some(value) = model.w_gra_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::PfMinSf => {
            if let Some(value) = model.pf_min_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MaxRmpRteSf => {
            if let Some(value) = model.max_rmp_rte_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::EcpNomHzSf => {
            if let Some(value) = model.ecp_nom_hz_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
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

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ClcTotVa {
    Vector = 1,
    Arithmetic = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ConnPh {
    A = 1,
    B = 2,
    C = 3,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum VArAct {
    Switch = 1,
    Maintain = 2,
}

#[repr(C)]
pub struct Model121CallbackAdapter {
    context: *mut c_void,
    w_max_callback: extern "C" fn(*const c_void) -> u16,
    set_w_max_callback: extern "C" fn(u16, *mut c_void),
    v_ref_callback: extern "C" fn(*const c_void) -> u16,
    set_v_ref_callback: extern "C" fn(u16, *mut c_void),
    v_ref_ofs_callback: extern "C" fn(*const c_void) -> i16,
    set_v_ref_ofs_callback: extern "C" fn(i16, *mut c_void),
    v_max_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_v_max_callback: Option<extern "C" fn(u16, *mut c_void)>,
    v_min_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_v_min_callback: Option<extern "C" fn(u16, *mut c_void)>,
    va_max_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_va_max_callback: Option<extern "C" fn(u16, *mut c_void)>,
    v_ar_max_q1_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_v_ar_max_q1_callback: Option<extern "C" fn(i16, *mut c_void)>,
    v_ar_max_q2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_v_ar_max_q2_callback: Option<extern "C" fn(i16, *mut c_void)>,
    v_ar_max_q3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_v_ar_max_q3_callback: Option<extern "C" fn(i16, *mut c_void)>,
    v_ar_max_q4_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_v_ar_max_q4_callback: Option<extern "C" fn(i16, *mut c_void)>,
    w_gra_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_w_gra_callback: Option<extern "C" fn(u16, *mut c_void)>,
    pf_min_q1_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_pf_min_q1_callback: Option<extern "C" fn(i16, *mut c_void)>,
    pf_min_q2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_pf_min_q2_callback: Option<extern "C" fn(i16, *mut c_void)>,
    pf_min_q3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_pf_min_q3_callback: Option<extern "C" fn(i16, *mut c_void)>,
    pf_min_q4_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_pf_min_q4_callback: Option<extern "C" fn(i16, *mut c_void)>,
    v_ar_act_callback: Option<extern "C" fn(*const c_void) -> VArAct>,
    set_v_ar_act_callback: Option<extern "C" fn(VArAct, *mut c_void)>,
    clc_tot_va_callback: Option<extern "C" fn(*const c_void) -> ClcTotVa>,
    set_clc_tot_va_callback: Option<extern "C" fn(ClcTotVa, *mut c_void)>,
    max_rmp_rte_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_max_rmp_rte_callback: Option<extern "C" fn(u16, *mut c_void)>,
    ecp_nom_hz_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_ecp_nom_hz_callback: Option<extern "C" fn(u16, *mut c_void)>,
    conn_ph_callback: Option<extern "C" fn(*const c_void) -> ConnPh>,
    set_conn_ph_callback: Option<extern "C" fn(ConnPh, *mut c_void)>,
    w_max_sf_callback: extern "C" fn(*const c_void) -> u16,
    v_ref_sf_callback: extern "C" fn(*const c_void) -> u16,
    v_ref_ofs_sf_callback: extern "C" fn(*const c_void) -> u16,
    v_min_max_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    va_max_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    v_ar_max_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    w_gra_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    pf_min_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_rmp_rte_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    ecp_nom_hz_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model121CallbackAdapter {
    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    fn w_max(&self) -> u16 {
        (self.w_max_callback)(self.context)
    }

    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    fn set_w_max(&mut self, value: u16) {
        (self.set_w_max_callback)(value, self.context);
    }

    /// VRef
    ///
    /// Voltage at the PCC.
    fn v_ref(&self) -> u16 {
        (self.v_ref_callback)(self.context)
    }

    /// VRef
    ///
    /// Voltage at the PCC.
    fn set_v_ref(&mut self, value: u16) {
        (self.set_v_ref_callback)(value, self.context);
    }

    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    fn v_ref_ofs(&self) -> i16 {
        (self.v_ref_ofs_callback)(self.context)
    }

    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    fn set_v_ref_ofs(&mut self, value: i16) {
        (self.set_v_ref_ofs_callback)(value, self.context);
    }

    /// VMax
    ///
    /// Setpoint for maximum voltage.
    fn v_max(&self) -> Option<u16> {
        self.v_max_callback.map(|callback| (callback)(self.context))
    }

    /// VMax
    ///
    /// Setpoint for maximum voltage.
    fn set_v_max(&mut self, value: u16) {
        if let Some(callback) = self.set_v_max_callback {
            (callback)(value, self.context);
        };
    }

    /// VMin
    ///
    /// Setpoint for minimum voltage.
    fn v_min(&self) -> Option<u16> {
        self.v_min_callback.map(|callback| (callback)(self.context))
    }

    /// VMin
    ///
    /// Setpoint for minimum voltage.
    fn set_v_min(&mut self, value: u16) {
        if let Some(callback) = self.set_v_min_callback {
            (callback)(value, self.context);
        };
    }

    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    fn va_max(&self) -> Option<u16> {
        self.va_max_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    fn set_va_max(&mut self, value: u16) {
        if let Some(callback) = self.set_va_max_callback {
            (callback)(value, self.context);
        };
    }

    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    fn v_ar_max_q1(&self) -> Option<i16> {
        self.v_ar_max_q1_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    fn set_v_ar_max_q1(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_q1_callback {
            (callback)(value, self.context);
        };
    }

    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    fn v_ar_max_q2(&self) -> Option<i16> {
        self.v_ar_max_q2_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    fn set_v_ar_max_q2(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_q2_callback {
            (callback)(value, self.context);
        };
    }

    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    fn v_ar_max_q3(&self) -> Option<i16> {
        self.v_ar_max_q3_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    fn set_v_ar_max_q3(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_q3_callback {
            (callback)(value, self.context);
        };
    }

    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    fn v_ar_max_q4(&self) -> Option<i16> {
        self.v_ar_max_q4_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    fn set_v_ar_max_q4(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_q4_callback {
            (callback)(value, self.context);
        };
    }

    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    fn w_gra(&self) -> Option<u16> {
        self.w_gra_callback.map(|callback| (callback)(self.context))
    }

    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    fn set_w_gra(&mut self, value: u16) {
        if let Some(callback) = self.set_w_gra_callback {
            (callback)(value, self.context);
        };
    }

    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    fn pf_min_q1(&self) -> Option<i16> {
        self.pf_min_q1_callback
            .map(|callback| (callback)(self.context))
    }

    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    fn set_pf_min_q1(&mut self, value: i16) {
        if let Some(callback) = self.set_pf_min_q1_callback {
            (callback)(value, self.context);
        };
    }

    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    fn pf_min_q2(&self) -> Option<i16> {
        self.pf_min_q2_callback
            .map(|callback| (callback)(self.context))
    }

    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    fn set_pf_min_q2(&mut self, value: i16) {
        if let Some(callback) = self.set_pf_min_q2_callback {
            (callback)(value, self.context);
        };
    }

    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    fn pf_min_q3(&self) -> Option<i16> {
        self.pf_min_q3_callback
            .map(|callback| (callback)(self.context))
    }

    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    fn set_pf_min_q3(&mut self, value: i16) {
        if let Some(callback) = self.set_pf_min_q3_callback {
            (callback)(value, self.context);
        };
    }

    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    fn pf_min_q4(&self) -> Option<i16> {
        self.pf_min_q4_callback
            .map(|callback| (callback)(self.context))
    }

    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    fn set_pf_min_q4(&mut self, value: i16) {
        if let Some(callback) = self.set_pf_min_q4_callback {
            (callback)(value, self.context);
        };
    }

    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    fn v_ar_act(&self) -> Option<VArAct> {
        self.v_ar_act_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    fn set_v_ar_act(&mut self, value: VArAct) {
        if let Some(callback) = self.set_v_ar_act_callback {
            (callback)(value, self.context);
        };
    }

    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    fn clc_tot_va(&self) -> Option<ClcTotVa> {
        self.clc_tot_va_callback
            .map(|callback| (callback)(self.context))
    }

    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    fn set_clc_tot_va(&mut self, value: ClcTotVa) {
        if let Some(callback) = self.set_clc_tot_va_callback {
            (callback)(value, self.context);
        };
    }

    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    fn max_rmp_rte(&self) -> Option<u16> {
        self.max_rmp_rte_callback
            .map(|callback| (callback)(self.context))
    }

    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    fn set_max_rmp_rte(&mut self, value: u16) {
        if let Some(callback) = self.set_max_rmp_rte_callback {
            (callback)(value, self.context);
        };
    }

    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    fn ecp_nom_hz(&self) -> Option<u16> {
        self.ecp_nom_hz_callback
            .map(|callback| (callback)(self.context))
    }

    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    fn set_ecp_nom_hz(&mut self, value: u16) {
        if let Some(callback) = self.set_ecp_nom_hz_callback {
            (callback)(value, self.context);
        };
    }

    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    fn conn_ph(&self) -> Option<ConnPh> {
        self.conn_ph_callback
            .map(|callback| (callback)(self.context))
    }

    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    fn set_conn_ph(&mut self, value: ConnPh) {
        if let Some(callback) = self.set_conn_ph_callback {
            (callback)(value, self.context);
        };
    }

    /// WMax_SF
    ///
    /// Scale factor for real power.
    fn w_max_sf(&self) -> u16 {
        (self.w_max_sf_callback)(self.context)
    }

    /// VRef_SF
    ///
    /// Scale factor for voltage at the PCC.
    fn v_ref_sf(&self) -> u16 {
        (self.v_ref_sf_callback)(self.context)
    }

    /// VRefOfs_SF
    ///
    /// Scale factor for offset voltage.
    fn v_ref_ofs_sf(&self) -> u16 {
        (self.v_ref_ofs_sf_callback)(self.context)
    }

    /// VMinMax_SF
    ///
    /// Scale factor for min/max voltages.
    fn v_min_max_sf(&self) -> Option<u16> {
        self.v_min_max_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAMax_SF
    ///
    /// Scale factor for apparent power.
    fn va_max_sf(&self) -> Option<u16> {
        self.va_max_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArMax_SF
    ///
    /// Scale factor for reactive power.
    fn v_ar_max_sf(&self) -> Option<u16> {
        self.v_ar_max_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// WGra_SF
    ///
    /// Scale factor for default ramp rate.
    fn w_gra_sf(&self) -> Option<u16> {
        self.w_gra_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// PFMin_SF
    ///
    /// Scale factor for minimum power factor.
    fn pf_min_sf(&self) -> Option<u16> {
        self.pf_min_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// MaxRmpRte_SF
    ///
    /// Scale factor for maximum ramp percentage.
    fn max_rmp_rte_sf(&self) -> Option<u16> {
        self.max_rmp_rte_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// ECPNomHz_SF
    ///
    /// Scale factor for nominal frequency.
    fn ecp_nom_hz_sf(&self) -> Option<u16> {
        self.ecp_nom_hz_sf_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model121StatefulAdapter {
    w_max: u16,
    v_ref: u16,
    v_ref_ofs: i16,
    v_max: u16,
    v_min: u16,
    va_max: u16,
    v_ar_max_q1: i16,
    v_ar_max_q2: i16,
    v_ar_max_q3: i16,
    v_ar_max_q4: i16,
    w_gra: u16,
    pf_min_q1: i16,
    pf_min_q2: i16,
    pf_min_q3: i16,
    pf_min_q4: i16,
    v_ar_act: VArAct,
    clc_tot_va: ClcTotVa,
    max_rmp_rte: u16,
    ecp_nom_hz: u16,
    conn_ph: ConnPh,
    w_max_sf: u16,
    v_ref_sf: u16,
    v_ref_ofs_sf: u16,
    v_min_max_sf: u16,
    va_max_sf: u16,
    v_ar_max_sf: u16,
    w_gra_sf: u16,
    pf_min_sf: u16,
    max_rmp_rte_sf: u16,
    ecp_nom_hz_sf: u16,
}

impl ModelAdapter for Model121StatefulAdapter {
    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    fn w_max(&self) -> u16 {
        self.w_max
    }

    /// WMax
    ///
    /// Setting for maximum power output. Default to WRtg.
    fn set_w_max(&mut self, value: u16) {
        self.w_max = value;
    }

    /// VRef
    ///
    /// Voltage at the PCC.
    fn v_ref(&self) -> u16 {
        self.v_ref
    }

    /// VRef
    ///
    /// Voltage at the PCC.
    fn set_v_ref(&mut self, value: u16) {
        self.v_ref = value;
    }

    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    fn v_ref_ofs(&self) -> i16 {
        self.v_ref_ofs
    }

    /// VRefOfs
    ///
    /// Offset from PCC to inverter.
    fn set_v_ref_ofs(&mut self, value: i16) {
        self.v_ref_ofs = value;
    }

    /// VMax
    ///
    /// Setpoint for maximum voltage.
    fn v_max(&self) -> Option<u16> {
        Some(self.v_max)
    }

    /// VMax
    ///
    /// Setpoint for maximum voltage.
    fn set_v_max(&mut self, value: u16) {
        self.v_max = value;
    }

    /// VMin
    ///
    /// Setpoint for minimum voltage.
    fn v_min(&self) -> Option<u16> {
        Some(self.v_min)
    }

    /// VMin
    ///
    /// Setpoint for minimum voltage.
    fn set_v_min(&mut self, value: u16) {
        self.v_min = value;
    }

    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    fn va_max(&self) -> Option<u16> {
        Some(self.va_max)
    }

    /// VAMax
    ///
    /// Setpoint for maximum apparent power. Default to VARtg.
    fn set_va_max(&mut self, value: u16) {
        self.va_max = value;
    }

    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    fn v_ar_max_q1(&self) -> Option<i16> {
        Some(self.v_ar_max_q1)
    }

    /// VArMaxQ1
    ///
    /// Setting for maximum reactive power in quadrant 1. Default to VArRtgQ1.
    fn set_v_ar_max_q1(&mut self, value: i16) {
        self.v_ar_max_q1 = value;
    }

    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    fn v_ar_max_q2(&self) -> Option<i16> {
        Some(self.v_ar_max_q2)
    }

    /// VArMaxQ2
    ///
    /// Setting for maximum reactive power in quadrant 2. Default to VArRtgQ2.
    fn set_v_ar_max_q2(&mut self, value: i16) {
        self.v_ar_max_q2 = value;
    }

    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    fn v_ar_max_q3(&self) -> Option<i16> {
        Some(self.v_ar_max_q3)
    }

    /// VArMaxQ3
    ///
    /// Setting for maximum reactive power in quadrant 3. Default to VArRtgQ3.
    fn set_v_ar_max_q3(&mut self, value: i16) {
        self.v_ar_max_q3 = value;
    }

    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    fn v_ar_max_q4(&self) -> Option<i16> {
        Some(self.v_ar_max_q4)
    }

    /// VArMaxQ4
    ///
    /// Setting for maximum reactive power in quadrant 4. Default to VArRtgQ4.
    fn set_v_ar_max_q4(&mut self, value: i16) {
        self.v_ar_max_q4 = value;
    }

    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    fn w_gra(&self) -> Option<u16> {
        Some(self.w_gra)
    }

    /// WGra
    ///
    /// Default ramp rate of change of active power due to command or internal action.
    fn set_w_gra(&mut self, value: u16) {
        self.w_gra = value;
    }

    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    fn pf_min_q1(&self) -> Option<i16> {
        Some(self.pf_min_q1)
    }

    /// PFMinQ1
    ///
    /// Setpoint for minimum power factor value in quadrant 1. Default to PFRtgQ1.
    ///
    /// EEI sign convention.
    fn set_pf_min_q1(&mut self, value: i16) {
        self.pf_min_q1 = value;
    }

    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    fn pf_min_q2(&self) -> Option<i16> {
        Some(self.pf_min_q2)
    }

    /// PFMinQ2
    ///
    /// Setpoint for minimum power factor value in quadrant 2. Default to PFRtgQ2.
    ///
    /// EEI sign convention.
    fn set_pf_min_q2(&mut self, value: i16) {
        self.pf_min_q2 = value;
    }

    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    fn pf_min_q3(&self) -> Option<i16> {
        Some(self.pf_min_q3)
    }

    /// PFMinQ3
    ///
    /// Setpoint for minimum power factor value in quadrant 3. Default to PFRtgQ3.
    ///
    /// EEI sign convention.
    fn set_pf_min_q3(&mut self, value: i16) {
        self.pf_min_q3 = value;
    }

    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    fn pf_min_q4(&self) -> Option<i16> {
        Some(self.pf_min_q4)
    }

    /// PFMinQ4
    ///
    /// Setpoint for minimum power factor value in quadrant 4. Default to PFRtgQ4.
    ///
    /// EEI sign convention.
    fn set_pf_min_q4(&mut self, value: i16) {
        self.pf_min_q4 = value;
    }

    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    fn v_ar_act(&self) -> Option<VArAct> {
        Some(self.v_ar_act)
    }

    /// VArAct
    ///
    /// VAR action on change between charging and discharging: 1=switch 2=maintain VAR characterization.
    fn set_v_ar_act(&mut self, value: VArAct) {
        self.v_ar_act = value;
    }

    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    fn clc_tot_va(&self) -> Option<ClcTotVa> {
        Some(self.clc_tot_va)
    }

    /// ClcTotVA
    ///
    /// Calculation method for total apparent power. 1=vector 2=arithmetic.
    fn set_clc_tot_va(&mut self, value: ClcTotVa) {
        self.clc_tot_va = value;
    }

    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    fn max_rmp_rte(&self) -> Option<u16> {
        Some(self.max_rmp_rte)
    }

    /// MaxRmpRte
    ///
    /// Setpoint for maximum ramp rate as percentage of nominal maximum ramp rate. This setting will limit the rate that watts delivery to the grid can increase or decrease in response to intermittent PV generation.
    fn set_max_rmp_rte(&mut self, value: u16) {
        self.max_rmp_rte = value;
    }

    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    fn ecp_nom_hz(&self) -> Option<u16> {
        Some(self.ecp_nom_hz)
    }

    /// ECPNomHz
    ///
    /// Setpoint for nominal frequency at the ECP.
    fn set_ecp_nom_hz(&mut self, value: u16) {
        self.ecp_nom_hz = value;
    }

    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    fn conn_ph(&self) -> Option<ConnPh> {
        Some(self.conn_ph)
    }

    /// ConnPh
    ///
    /// Identity of connected phase for single phase inverters. A=1 B=2 C=3.
    fn set_conn_ph(&mut self, value: ConnPh) {
        self.conn_ph = value;
    }

    /// WMax_SF
    ///
    /// Scale factor for real power.
    fn w_max_sf(&self) -> u16 {
        self.w_max_sf
    }

    /// VRef_SF
    ///
    /// Scale factor for voltage at the PCC.
    fn v_ref_sf(&self) -> u16 {
        self.v_ref_sf
    }

    /// VRefOfs_SF
    ///
    /// Scale factor for offset voltage.
    fn v_ref_ofs_sf(&self) -> u16 {
        self.v_ref_ofs_sf
    }

    /// VMinMax_SF
    ///
    /// Scale factor for min/max voltages.
    fn v_min_max_sf(&self) -> Option<u16> {
        Some(self.v_min_max_sf)
    }

    /// VAMax_SF
    ///
    /// Scale factor for apparent power.
    fn va_max_sf(&self) -> Option<u16> {
        Some(self.va_max_sf)
    }

    /// VArMax_SF
    ///
    /// Scale factor for reactive power.
    fn v_ar_max_sf(&self) -> Option<u16> {
        Some(self.v_ar_max_sf)
    }

    /// WGra_SF
    ///
    /// Scale factor for default ramp rate.
    fn w_gra_sf(&self) -> Option<u16> {
        Some(self.w_gra_sf)
    }

    /// PFMin_SF
    ///
    /// Scale factor for minimum power factor.
    fn pf_min_sf(&self) -> Option<u16> {
        Some(self.pf_min_sf)
    }

    /// MaxRmpRte_SF
    ///
    /// Scale factor for maximum ramp percentage.
    fn max_rmp_rte_sf(&self) -> Option<u16> {
        Some(self.max_rmp_rte_sf)
    }

    /// ECPNomHz_SF
    ///
    /// Scale factor for nominal frequency.
    fn ecp_nom_hz_sf(&self) -> Option<u16> {
        Some(self.ecp_nom_hz_sf)
    }
}
