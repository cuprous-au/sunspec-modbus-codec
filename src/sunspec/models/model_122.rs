use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 46;

pub static POINTS: [ReadablePoint; 22] = [
    ReadablePoint {
        reference: PointReference::Static { value: 122 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 44 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::PvConn,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::StorConn,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::EcpConn,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::ActWh,
        },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::ActVAh,
        },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::ActVArhQ1,
        },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::ActVArhQ2,
        },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::ActVArhQ3,
        },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::ActVArhQ4,
        },
        size: 4,
        data_type: PointType::Acc64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::VArAval,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::VArAvalSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::WAval,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::WAvalSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::StSetLimMsk,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::StActCtl,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::TmSrc,
        },
        size: 4,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 { point: Point::Tms },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 { point: Point::RtSt },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 { point: Point::Ris },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model122 {
            point: Point::RisSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::PvConn => serialisation::write_u16(model.pv_conn(), buffer),
        Point::StorConn => serialisation::write_u16(model.stor_conn(), buffer),
        Point::EcpConn => serialisation::write_u16(model.ecp_conn(), buffer),
        Point::ActWh => {
            if let Some(value) = model.act_wh() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::ActVAh => {
            if let Some(value) = model.act_v_ah() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::ActVArhQ1 => {
            if let Some(value) = model.act_v_arh_q1() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::ActVArhQ2 => {
            if let Some(value) = model.act_v_arh_q2() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::ActVArhQ3 => {
            if let Some(value) = model.act_v_arh_q3() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::ActVArhQ4 => {
            if let Some(value) = model.act_v_arh_q4() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::VArAval => {
            if let Some(value) = model.v_ar_aval() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArAvalSf => {
            if let Some(value) = model.v_ar_aval_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WAval => {
            if let Some(value) = model.w_aval() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WAvalSf => {
            if let Some(value) = model.w_aval_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::StSetLimMsk => {
            if let Some(value) = model.st_set_lim_msk() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::StActCtl => {
            if let Some(value) = model.st_act_ctl() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TmSrc => {
            if let Some(value) = model.tm_src() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Tms => {
            if let Some(value) = model.tms() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::RtSt => {
            if let Some(value) = model.rt_st() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Ris => {
            if let Some(value) = model.ris() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::RisSf => {
            if let Some(value) = model.ris_sf() {
                serialisation::write_u16(value, buffer);
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
    pv_conn_callback: extern "C" fn() -> u16,
    stor_conn_callback: extern "C" fn() -> u16,
    ecp_conn_callback: extern "C" fn() -> u16,
    act_wh_callback: Option<extern "C" fn() -> u64>,
    act_v_ah_callback: Option<extern "C" fn() -> u64>,
    act_v_arh_q1_callback: Option<extern "C" fn() -> u64>,
    act_v_arh_q2_callback: Option<extern "C" fn() -> u64>,
    act_v_arh_q3_callback: Option<extern "C" fn() -> u64>,
    act_v_arh_q4_callback: Option<extern "C" fn() -> u64>,
    v_ar_aval_callback: Option<extern "C" fn() -> i16>,
    v_ar_aval_sf_callback: Option<extern "C" fn() -> u16>,
    w_aval_callback: Option<extern "C" fn() -> u16>,
    w_aval_sf_callback: Option<extern "C" fn() -> u16>,
    st_set_lim_msk_callback: Option<extern "C" fn() -> u32>,
    st_act_ctl_callback: Option<extern "C" fn() -> u32>,
    tm_src_callback: Option<extern "C" fn() -> *const c_char>,
    tms_callback: Option<extern "C" fn() -> u32>,
    rt_st_callback: Option<extern "C" fn() -> u16>,
    ris_callback: Option<extern "C" fn() -> u16>,
    ris_sf_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model122CallbackAdapter {
    /// PVConn
    ///
    /// PV inverter present/available status.
    fn pv_conn(&self) -> u16 {
        (self.pv_conn_callback)()
    }

    /// StorConn
    ///
    /// Storage inverter present/available status.
    fn stor_conn(&self) -> u16 {
        (self.stor_conn_callback)()
    }

    /// ECPConn
    ///
    /// ECP connection status: disconnected=0 connected=1.
    fn ecp_conn(&self) -> u16 {
        (self.ecp_conn_callback)()
    }

    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    fn act_wh(&self) -> Option<u64> {
        self.act_wh_callback.map(|callback| (callback)())
    }

    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    fn act_v_ah(&self) -> Option<u64> {
        self.act_v_ah_callback.map(|callback| (callback)())
    }

    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    fn act_v_arh_q1(&self) -> Option<u64> {
        self.act_v_arh_q1_callback.map(|callback| (callback)())
    }

    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    fn act_v_arh_q2(&self) -> Option<u64> {
        self.act_v_arh_q2_callback.map(|callback| (callback)())
    }

    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output in quadrant 3.
    fn act_v_arh_q3(&self) -> Option<u64> {
        self.act_v_arh_q3_callback.map(|callback| (callback)())
    }

    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output in quadrant 4.
    fn act_v_arh_q4(&self) -> Option<u64> {
        self.act_v_arh_q4_callback.map(|callback| (callback)())
    }

    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    fn v_ar_aval(&self) -> Option<i16> {
        self.v_ar_aval_callback.map(|callback| (callback)())
    }

    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    fn v_ar_aval_sf(&self) -> Option<u16> {
        self.v_ar_aval_sf_callback.map(|callback| (callback)())
    }

    /// WAval
    ///
    /// Amount of Watts available.
    fn w_aval(&self) -> Option<u16> {
        self.w_aval_callback.map(|callback| (callback)())
    }

    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    fn w_aval_sf(&self) -> Option<u16> {
        self.w_aval_sf_callback.map(|callback| (callback)())
    }

    /// StSetLimMsk
    ///
    /// Setpoint limit(s) reached.
    ///
    /// Bits shall be automatically cleared on read.
    fn st_set_lim_msk(&self) -> Option<u32> {
        self.st_set_lim_msk_callback.map(|callback| (callback)())
    }

    /// StActCtl
    ///
    /// Which inverter controls are currently active.
    fn st_act_ctl(&self) -> Option<u32> {
        self.st_act_ctl_callback.map(|callback| (callback)())
    }

    /// TmSrc
    ///
    /// Source of time synchronization.
    fn tm_src(&self) -> Option<&CStr> {
        self.tm_src_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    fn tms(&self) -> Option<u32> {
        self.tms_callback.map(|callback| (callback)())
    }

    /// RtSt
    ///
    /// Active ride-through status.
    fn rt_st(&self) -> Option<u16> {
        self.rt_st_callback.map(|callback| (callback)())
    }

    /// Ris
    ///
    /// Isolation resistance.
    fn ris(&self) -> Option<u16> {
        self.ris_callback.map(|callback| (callback)())
    }

    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    fn ris_sf(&self) -> Option<u16> {
        self.ris_sf_callback.map(|callback| (callback)())
    }
}
