use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 46;

static POINTS: [PointDetails<()>; 22] = [
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
        point: |()| Point::PvConn,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::StorConn,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::EcpConn,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::ActWh,
        size: 4,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::ActVAh,
        size: 4,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::ActVArhQ1,
        size: 4,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::ActVArhQ2,
        size: 4,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::ActVArhQ3,
        size: 4,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::ActVArhQ4,
        size: 4,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::VArAval,
        size: 1,
        start_address: 29,
    },
    PointDetails {
        point: |()| Point::VArAvalSf,
        size: 1,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::WAval,
        size: 1,
        start_address: 31,
    },
    PointDetails {
        point: |()| Point::WAvalSf,
        size: 1,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::StSetLimMsk,
        size: 2,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::StActCtl,
        size: 2,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::TmSrc,
        size: 4,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::Tms,
        size: 2,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::RtSt,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::Ris,
        size: 1,
        start_address: 44,
    },
    PointDetails {
        point: |()| Point::RisSf,
        size: 1,
        start_address: 45,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    PvConn,
    StorConn,
    EcpConn,
    ActWh,
    ActVAh,
    ActVArhQ1,
    ActVArhQ2,
    ActVArhQ3,
    ActVArhQ4,
    VArAval,
    VArAvalSf,
    WAval,
    WAvalSf,
    StSetLimMsk,
    StActCtl,
    TmSrc,
    Tms,
    RtSt,
    Ris,
    RisSf,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    46
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
            buffer::write_u16(122, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::PvConn => {
            buffer::write_u16(model.pv_conn(), buffer);
        }
        Point::StorConn => {
            buffer::write_u16(model.stor_conn(), buffer);
        }
        Point::EcpConn => {
            buffer::write_u16(model.ecp_conn(), buffer);
        }
        Point::ActWh => {
            if let Some(value) = model.act_wh() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActVAh => {
            if let Some(value) = model.act_v_ah() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActVArhQ1 => {
            if let Some(value) = model.act_v_arh_q1() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActVArhQ2 => {
            if let Some(value) = model.act_v_arh_q2() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActVArhQ3 => {
            if let Some(value) = model.act_v_arh_q3() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActVArhQ4 => {
            if let Some(value) = model.act_v_arh_q4() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArAval => {
            if let Some(value) = model.v_ar_aval() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArAvalSf => {
            if let Some(value) = model.v_ar_aval_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WAval => {
            if let Some(value) = model.w_aval() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WAvalSf => {
            if let Some(value) = model.w_aval_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StSetLimMsk => {
            if let Some(value) = model.st_set_lim_msk() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StActCtl => {
            if let Some(value) = model.st_act_ctl() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TmSrc => {
            if let Some(value) = model.tm_src() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Tms => {
            if let Some(value) = model.tms() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RtSt => {
            if let Some(value) = model.rt_st() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Ris => {
            if let Some(value) = model.ris() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RisSf => {
            if let Some(value) = model.ris_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// PVConn
    ///
    /// PV inverter present/available status.
    fn pv_conn(&self) -> u16;

    /// StorConn
    ///
    /// Storage inverter present/available status.
    fn stor_conn(&self) -> u16;

    /// ECPConn
    ///
    /// ECP connection status: disconnected=0 connected=1.
    fn ecp_conn(&self) -> u16;

    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    fn act_wh(&self) -> Option<u64> {
        None
    }

    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    fn act_v_ah(&self) -> Option<u64> {
        None
    }

    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    fn act_v_arh_q1(&self) -> Option<u64> {
        None
    }

    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    fn act_v_arh_q2(&self) -> Option<u64> {
        None
    }

    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output in quadrant 3.
    fn act_v_arh_q3(&self) -> Option<u64> {
        None
    }

    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output in quadrant 4.
    fn act_v_arh_q4(&self) -> Option<u64> {
        None
    }

    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    fn v_ar_aval(&self) -> Option<i16> {
        None
    }

    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    fn v_ar_aval_sf(&self) -> Option<u16> {
        None
    }

    /// WAval
    ///
    /// Amount of Watts available.
    fn w_aval(&self) -> Option<u16> {
        None
    }

    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    fn w_aval_sf(&self) -> Option<u16> {
        None
    }

    /// StSetLimMsk
    ///
    /// Setpoint limit(s) reached.
    ///
    /// Bits shall be automatically cleared on read.
    fn st_set_lim_msk(&self) -> Option<u32> {
        None
    }

    /// StActCtl
    ///
    /// Which inverter controls are currently active.
    fn st_act_ctl(&self) -> Option<u32> {
        None
    }

    /// TmSrc
    ///
    /// Source of time synchronization.
    fn tm_src(&self) -> Option<&CStr> {
        None
    }

    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    fn tms(&self) -> Option<u32> {
        None
    }

    /// RtSt
    ///
    /// Active ride-through status.
    fn rt_st(&self) -> Option<u16> {
        None
    }

    /// Ris
    ///
    /// Isolation resistance.
    fn ris(&self) -> Option<u16> {
        None
    }

    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    fn ris_sf(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model122CallbackAdapter {
    context: *mut c_void,
    pv_conn_callback: extern "C" fn(*const c_void) -> u16,
    stor_conn_callback: extern "C" fn(*const c_void) -> u16,
    ecp_conn_callback: extern "C" fn(*const c_void) -> u16,
    act_wh_callback: Option<extern "C" fn(*const c_void) -> u64>,
    act_v_ah_callback: Option<extern "C" fn(*const c_void) -> u64>,
    act_v_arh_q1_callback: Option<extern "C" fn(*const c_void) -> u64>,
    act_v_arh_q2_callback: Option<extern "C" fn(*const c_void) -> u64>,
    act_v_arh_q3_callback: Option<extern "C" fn(*const c_void) -> u64>,
    act_v_arh_q4_callback: Option<extern "C" fn(*const c_void) -> u64>,
    v_ar_aval_callback: Option<extern "C" fn(*const c_void) -> i16>,
    v_ar_aval_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    w_aval_callback: Option<extern "C" fn(*const c_void) -> u16>,
    w_aval_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    st_set_lim_msk_callback: Option<extern "C" fn(*const c_void) -> u32>,
    st_act_ctl_callback: Option<extern "C" fn(*const c_void) -> u32>,
    tm_src_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    tms_callback: Option<extern "C" fn(*const c_void) -> u32>,
    rt_st_callback: Option<extern "C" fn(*const c_void) -> u16>,
    ris_callback: Option<extern "C" fn(*const c_void) -> u16>,
    ris_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model122CallbackAdapter {
    /// PVConn
    ///
    /// PV inverter present/available status.
    fn pv_conn(&self) -> u16 {
        (self.pv_conn_callback)(self.context)
    }

    /// StorConn
    ///
    /// Storage inverter present/available status.
    fn stor_conn(&self) -> u16 {
        (self.stor_conn_callback)(self.context)
    }

    /// ECPConn
    ///
    /// ECP connection status: disconnected=0 connected=1.
    fn ecp_conn(&self) -> u16 {
        (self.ecp_conn_callback)(self.context)
    }

    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    fn act_wh(&self) -> Option<u64> {
        self.act_wh_callback
            .map(|callback| (callback)(self.context))
    }

    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    fn act_v_ah(&self) -> Option<u64> {
        self.act_v_ah_callback
            .map(|callback| (callback)(self.context))
    }

    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    fn act_v_arh_q1(&self) -> Option<u64> {
        self.act_v_arh_q1_callback
            .map(|callback| (callback)(self.context))
    }

    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    fn act_v_arh_q2(&self) -> Option<u64> {
        self.act_v_arh_q2_callback
            .map(|callback| (callback)(self.context))
    }

    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output in quadrant 3.
    fn act_v_arh_q3(&self) -> Option<u64> {
        self.act_v_arh_q3_callback
            .map(|callback| (callback)(self.context))
    }

    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output in quadrant 4.
    fn act_v_arh_q4(&self) -> Option<u64> {
        self.act_v_arh_q4_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    fn v_ar_aval(&self) -> Option<i16> {
        self.v_ar_aval_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    fn v_ar_aval_sf(&self) -> Option<u16> {
        self.v_ar_aval_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// WAval
    ///
    /// Amount of Watts available.
    fn w_aval(&self) -> Option<u16> {
        self.w_aval_callback
            .map(|callback| (callback)(self.context))
    }

    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    fn w_aval_sf(&self) -> Option<u16> {
        self.w_aval_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// StSetLimMsk
    ///
    /// Setpoint limit(s) reached.
    ///
    /// Bits shall be automatically cleared on read.
    fn st_set_lim_msk(&self) -> Option<u32> {
        self.st_set_lim_msk_callback
            .map(|callback| (callback)(self.context))
    }

    /// StActCtl
    ///
    /// Which inverter controls are currently active.
    fn st_act_ctl(&self) -> Option<u32> {
        self.st_act_ctl_callback
            .map(|callback| (callback)(self.context))
    }

    /// TmSrc
    ///
    /// Source of time synchronization.
    fn tm_src(&self) -> Option<&CStr> {
        self.tm_src_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    fn tms(&self) -> Option<u32> {
        self.tms_callback.map(|callback| (callback)(self.context))
    }

    /// RtSt
    ///
    /// Active ride-through status.
    fn rt_st(&self) -> Option<u16> {
        self.rt_st_callback.map(|callback| (callback)(self.context))
    }

    /// Ris
    ///
    /// Isolation resistance.
    fn ris(&self) -> Option<u16> {
        self.ris_callback.map(|callback| (callback)(self.context))
    }

    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    fn ris_sf(&self) -> Option<u16> {
        self.ris_sf_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model122StatefulAdapter {
    pv_conn: u16,
    stor_conn: u16,
    ecp_conn: u16,
    act_wh: u64,
    act_v_ah: u64,
    act_v_arh_q1: u64,
    act_v_arh_q2: u64,
    act_v_arh_q3: u64,
    act_v_arh_q4: u64,
    v_ar_aval: i16,
    v_ar_aval_sf: u16,
    w_aval: u16,
    w_aval_sf: u16,
    st_set_lim_msk: u32,
    st_act_ctl: u32,
    tm_src: [c_char; 8],
    tms: u32,
    rt_st: u16,
    ris: u16,
    ris_sf: u16,
}

impl ModelAdapter for Model122StatefulAdapter {
    /// PVConn
    ///
    /// PV inverter present/available status.
    fn pv_conn(&self) -> u16 {
        self.pv_conn
    }

    /// StorConn
    ///
    /// Storage inverter present/available status.
    fn stor_conn(&self) -> u16 {
        self.stor_conn
    }

    /// ECPConn
    ///
    /// ECP connection status: disconnected=0 connected=1.
    fn ecp_conn(&self) -> u16 {
        self.ecp_conn
    }

    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    fn act_wh(&self) -> Option<u64> {
        Some(self.act_wh)
    }

    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    fn act_v_ah(&self) -> Option<u64> {
        Some(self.act_v_ah)
    }

    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    fn act_v_arh_q1(&self) -> Option<u64> {
        Some(self.act_v_arh_q1)
    }

    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    fn act_v_arh_q2(&self) -> Option<u64> {
        Some(self.act_v_arh_q2)
    }

    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output in quadrant 3.
    fn act_v_arh_q3(&self) -> Option<u64> {
        Some(self.act_v_arh_q3)
    }

    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output in quadrant 4.
    fn act_v_arh_q4(&self) -> Option<u64> {
        Some(self.act_v_arh_q4)
    }

    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    fn v_ar_aval(&self) -> Option<i16> {
        Some(self.v_ar_aval)
    }

    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    fn v_ar_aval_sf(&self) -> Option<u16> {
        Some(self.v_ar_aval_sf)
    }

    /// WAval
    ///
    /// Amount of Watts available.
    fn w_aval(&self) -> Option<u16> {
        Some(self.w_aval)
    }

    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    fn w_aval_sf(&self) -> Option<u16> {
        Some(self.w_aval_sf)
    }

    /// StSetLimMsk
    ///
    /// Setpoint limit(s) reached.
    ///
    /// Bits shall be automatically cleared on read.
    fn st_set_lim_msk(&self) -> Option<u32> {
        Some(self.st_set_lim_msk)
    }

    /// StActCtl
    ///
    /// Which inverter controls are currently active.
    fn st_act_ctl(&self) -> Option<u32> {
        Some(self.st_act_ctl)
    }

    /// TmSrc
    ///
    /// Source of time synchronization.
    fn tm_src(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.tm_src.as_ptr()) })
    }

    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    fn tms(&self) -> Option<u32> {
        Some(self.tms)
    }

    /// RtSt
    ///
    /// Active ride-through status.
    fn rt_st(&self) -> Option<u16> {
        Some(self.rt_st)
    }

    /// Ris
    ///
    /// Isolation resistance.
    fn ris(&self) -> Option<u16> {
        Some(self.ris)
    }

    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    fn ris_sf(&self) -> Option<u16> {
        Some(self.ris_sf)
    }
}
