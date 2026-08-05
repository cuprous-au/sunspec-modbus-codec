use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 26;

pub static POINTS: [ReadablePoint; 26] = [
    ReadablePoint {
        reference: PointReference::Static { value: 123 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::ConnWinTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::ConnRvrtTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 { point: Point::Conn },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::WMaxLimPct,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::WMaxLimPctWinTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::WMaxLimPctRvrtTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::WMaxLimPctRmpTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::WMaxLimEna,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::OutPfSet,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::OutPfSetWinTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::OutPfSetRvrtTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::OutPfSetRmpTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::OutPfSetEna,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArWMaxPct,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArMaxPct,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArAvalPct,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArPctWinTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArPctRvrtTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArPctRmpTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArPctMod,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArPctEna,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::WMaxLimPctSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::OutPfSetSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model123 {
            point: Point::VArPctSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    ConnWinTms,
    ConnRvrtTms,
    Conn,
    WMaxLimPct,
    WMaxLimPctWinTms,
    WMaxLimPctRvrtTms,
    WMaxLimPctRmpTms,
    WMaxLimEna,
    OutPfSet,
    OutPfSetWinTms,
    OutPfSetRvrtTms,
    OutPfSetRmpTms,
    OutPfSetEna,
    VArWMaxPct,
    VArMaxPct,
    VArAvalPct,
    VArPctWinTms,
    VArPctRvrtTms,
    VArPctRmpTms,
    VArPctMod,
    VArPctEna,
    WMaxLimPctSf,
    OutPfSetSf,
    VArPctSf,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    26
}

pub fn write_point<'a, 'b>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::ConnWinTms => {
            if let Some(value) = model.conn_win_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ConnRvrtTms => {
            if let Some(value) = model.conn_rvrt_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Conn => {
            buffer::write_u16(model.conn() as u16, buffer);
        }
        Point::WMaxLimPct => {
            buffer::write_u16(model.w_max_lim_pct(), buffer);
        }
        Point::WMaxLimPctWinTms => {
            if let Some(value) = model.w_max_lim_pct_win_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WMaxLimPctRvrtTms => {
            if let Some(value) = model.w_max_lim_pct_rvrt_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WMaxLimPctRmpTms => {
            if let Some(value) = model.w_max_lim_pct_rmp_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WMaxLimEna => {
            buffer::write_u16(model.w_max_lim_ena() as u16, buffer);
        }
        Point::OutPfSet => {
            buffer::write_i16(model.out_pf_set(), buffer);
        }
        Point::OutPfSetWinTms => {
            if let Some(value) = model.out_pf_set_win_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutPfSetRvrtTms => {
            if let Some(value) = model.out_pf_set_rvrt_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutPfSetRmpTms => {
            if let Some(value) = model.out_pf_set_rmp_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutPfSetEna => {
            buffer::write_u16(model.out_pf_set_ena() as u16, buffer);
        }
        Point::VArWMaxPct => {
            if let Some(value) = model.v_ar_w_max_pct() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArMaxPct => {
            if let Some(value) = model.v_ar_max_pct() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArAvalPct => {
            if let Some(value) = model.v_ar_aval_pct() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArPctWinTms => {
            if let Some(value) = model.v_ar_pct_win_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArPctRvrtTms => {
            if let Some(value) = model.v_ar_pct_rvrt_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArPctRmpTms => {
            if let Some(value) = model.v_ar_pct_rmp_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArPctMod => {
            if let Some(value) = model.v_ar_pct_mod() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArPctEna => {
            buffer::write_u16(model.v_ar_pct_ena() as u16, buffer);
        }
        Point::WMaxLimPctSf => {
            buffer::write_u16(model.w_max_lim_pct_sf(), buffer);
        }
        Point::OutPfSetSf => {
            buffer::write_u16(model.out_pf_set_sf(), buffer);
        }
        Point::VArPctSf => {
            if let Some(value) = model.v_ar_pct_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn conn_win_tms(&self) -> Option<u16> {
        None
    }

    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn set_conn_win_tms(&mut self, value: u16) {}

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn conn_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn set_conn_rvrt_tms(&mut self, value: u16) {}

    /// Conn
    ///
    /// Connection control.
    fn conn(&self) -> Conn;

    /// Conn
    ///
    /// Connection control.
    fn set_conn(&mut self, value: Conn);

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn w_max_lim_pct(&self) -> u16;

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn set_w_max_lim_pct(&mut self, value: u16);

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn w_max_lim_pct_win_tms(&self) -> Option<u16> {
        None
    }

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn set_w_max_lim_pct_win_tms(&mut self, value: u16) {}

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn w_max_lim_pct_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn set_w_max_lim_pct_rvrt_tms(&mut self, value: u16) {}

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn w_max_lim_pct_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_w_max_lim_pct_rmp_tms(&mut self, value: u16) {}

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn w_max_lim_ena(&self) -> WMaxLimEna;

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn set_w_max_lim_ena(&mut self, value: WMaxLimEna);

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn out_pf_set(&self) -> i16;

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn set_out_pf_set(&mut self, value: i16);

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn out_pf_set_win_tms(&self) -> Option<u16> {
        None
    }

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn set_out_pf_set_win_tms(&mut self, value: u16) {}

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn out_pf_set_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn set_out_pf_set_rvrt_tms(&mut self, value: u16) {}

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn out_pf_set_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_out_pf_set_rmp_tms(&mut self, value: u16) {}

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn out_pf_set_ena(&self) -> OutPfSetEna;

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn set_out_pf_set_ena(&mut self, value: OutPfSetEna);

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn v_ar_w_max_pct(&self) -> Option<i16> {
        None
    }

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn set_v_ar_w_max_pct(&mut self, value: i16) {}

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn v_ar_max_pct(&self) -> Option<i16> {
        None
    }

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn set_v_ar_max_pct(&mut self, value: i16) {}

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn v_ar_aval_pct(&self) -> Option<i16> {
        None
    }

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn set_v_ar_aval_pct(&mut self, value: i16) {}

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn v_ar_pct_win_tms(&self) -> Option<u16> {
        None
    }

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn set_v_ar_pct_win_tms(&mut self, value: u16) {}

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn v_ar_pct_rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn set_v_ar_pct_rvrt_tms(&mut self, value: u16) {}

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn v_ar_pct_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_v_ar_pct_rmp_tms(&mut self, value: u16) {}

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn v_ar_pct_mod(&self) -> Option<VArPctMod> {
        None
    }

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn set_v_ar_pct_mod(&mut self, value: VArPctMod) {}

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn v_ar_pct_ena(&self) -> VArPctEna;

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn set_v_ar_pct_ena(&mut self, value: VArPctEna);

    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    fn w_max_lim_pct_sf(&self) -> u16;

    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    fn out_pf_set_sf(&self) -> u16;

    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    fn v_ar_pct_sf(&self) -> Option<u16> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Conn {
    Disconnect = 0,
    Connect = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum OutPfSetEna {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum VArPctEna {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum VArPctMod {
    None = 0,
    WMax = 1,
    VArMax = 2,
    VArAval = 3,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum WMaxLimEna {
    Disabled = 0,
    Enabled = 1,
}

#[repr(C)]
pub struct Model123CallbackAdapter {
    context: *mut c_void,
    conn_win_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_conn_win_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    conn_rvrt_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_conn_rvrt_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    conn_callback: extern "C" fn(*const c_void) -> Conn,
    set_conn_callback: extern "C" fn(Conn, *mut c_void),
    w_max_lim_pct_callback: extern "C" fn(*const c_void) -> u16,
    set_w_max_lim_pct_callback: extern "C" fn(u16, *mut c_void),
    w_max_lim_pct_win_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_w_max_lim_pct_win_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    w_max_lim_pct_rvrt_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_w_max_lim_pct_rvrt_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    w_max_lim_pct_rmp_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_w_max_lim_pct_rmp_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    w_max_lim_ena_callback: extern "C" fn(*const c_void) -> WMaxLimEna,
    set_w_max_lim_ena_callback: extern "C" fn(WMaxLimEna, *mut c_void),
    out_pf_set_callback: extern "C" fn(*const c_void) -> i16,
    set_out_pf_set_callback: extern "C" fn(i16, *mut c_void),
    out_pf_set_win_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_out_pf_set_win_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    out_pf_set_rvrt_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_out_pf_set_rvrt_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    out_pf_set_rmp_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_out_pf_set_rmp_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    out_pf_set_ena_callback: extern "C" fn(*const c_void) -> OutPfSetEna,
    set_out_pf_set_ena_callback: extern "C" fn(OutPfSetEna, *mut c_void),
    v_ar_w_max_pct_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_v_ar_w_max_pct_callback: Option<extern "C" fn(i16, *mut c_void)>,
    v_ar_max_pct_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_v_ar_max_pct_callback: Option<extern "C" fn(i16, *mut c_void)>,
    v_ar_aval_pct_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_v_ar_aval_pct_callback: Option<extern "C" fn(i16, *mut c_void)>,
    v_ar_pct_win_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_v_ar_pct_win_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    v_ar_pct_rvrt_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_v_ar_pct_rvrt_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    v_ar_pct_rmp_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_v_ar_pct_rmp_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    v_ar_pct_mod_callback: Option<extern "C" fn(*const c_void) -> VArPctMod>,
    set_v_ar_pct_mod_callback: Option<extern "C" fn(VArPctMod, *mut c_void)>,
    v_ar_pct_ena_callback: extern "C" fn(*const c_void) -> VArPctEna,
    set_v_ar_pct_ena_callback: extern "C" fn(VArPctEna, *mut c_void),
    w_max_lim_pct_sf_callback: extern "C" fn(*const c_void) -> u16,
    out_pf_set_sf_callback: extern "C" fn(*const c_void) -> u16,
    v_ar_pct_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model123CallbackAdapter {
    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn conn_win_tms(&self) -> Option<u16> {
        self.conn_win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn set_conn_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_conn_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn conn_rvrt_tms(&self) -> Option<u16> {
        self.conn_rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn set_conn_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_conn_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// Conn
    ///
    /// Connection control.
    fn conn(&self) -> Conn {
        (self.conn_callback)(self.context)
    }

    /// Conn
    ///
    /// Connection control.
    fn set_conn(&mut self, value: Conn) {
        (self.set_conn_callback)(value, self.context);
    }

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn w_max_lim_pct(&self) -> u16 {
        (self.w_max_lim_pct_callback)(self.context)
    }

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn set_w_max_lim_pct(&mut self, value: u16) {
        (self.set_w_max_lim_pct_callback)(value, self.context);
    }

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn w_max_lim_pct_win_tms(&self) -> Option<u16> {
        self.w_max_lim_pct_win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn set_w_max_lim_pct_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_w_max_lim_pct_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn w_max_lim_pct_rvrt_tms(&self) -> Option<u16> {
        self.w_max_lim_pct_rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn set_w_max_lim_pct_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_w_max_lim_pct_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn w_max_lim_pct_rmp_tms(&self) -> Option<u16> {
        self.w_max_lim_pct_rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_w_max_lim_pct_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_w_max_lim_pct_rmp_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn w_max_lim_ena(&self) -> WMaxLimEna {
        (self.w_max_lim_ena_callback)(self.context)
    }

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn set_w_max_lim_ena(&mut self, value: WMaxLimEna) {
        (self.set_w_max_lim_ena_callback)(value, self.context);
    }

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn out_pf_set(&self) -> i16 {
        (self.out_pf_set_callback)(self.context)
    }

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn set_out_pf_set(&mut self, value: i16) {
        (self.set_out_pf_set_callback)(value, self.context);
    }

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn out_pf_set_win_tms(&self) -> Option<u16> {
        self.out_pf_set_win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn set_out_pf_set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_out_pf_set_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn out_pf_set_rvrt_tms(&self) -> Option<u16> {
        self.out_pf_set_rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn set_out_pf_set_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_out_pf_set_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn out_pf_set_rmp_tms(&self) -> Option<u16> {
        self.out_pf_set_rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_out_pf_set_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_out_pf_set_rmp_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn out_pf_set_ena(&self) -> OutPfSetEna {
        (self.out_pf_set_ena_callback)(self.context)
    }

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn set_out_pf_set_ena(&mut self, value: OutPfSetEna) {
        (self.set_out_pf_set_ena_callback)(value, self.context);
    }

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn v_ar_w_max_pct(&self) -> Option<i16> {
        self.v_ar_w_max_pct_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn set_v_ar_w_max_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_w_max_pct_callback {
            (callback)(value, self.context);
        };
    }

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn v_ar_max_pct(&self) -> Option<i16> {
        self.v_ar_max_pct_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn set_v_ar_max_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_pct_callback {
            (callback)(value, self.context);
        };
    }

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn v_ar_aval_pct(&self) -> Option<i16> {
        self.v_ar_aval_pct_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn set_v_ar_aval_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_aval_pct_callback {
            (callback)(value, self.context);
        };
    }

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn v_ar_pct_win_tms(&self) -> Option<u16> {
        self.v_ar_pct_win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn set_v_ar_pct_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_v_ar_pct_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn v_ar_pct_rvrt_tms(&self) -> Option<u16> {
        self.v_ar_pct_rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn set_v_ar_pct_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_v_ar_pct_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn v_ar_pct_rmp_tms(&self) -> Option<u16> {
        self.v_ar_pct_rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_v_ar_pct_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_v_ar_pct_rmp_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn v_ar_pct_mod(&self) -> Option<VArPctMod> {
        self.v_ar_pct_mod_callback
            .map(|callback| (callback)(self.context))
    }

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn set_v_ar_pct_mod(&mut self, value: VArPctMod) {
        if let Some(callback) = self.set_v_ar_pct_mod_callback {
            (callback)(value, self.context);
        };
    }

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn v_ar_pct_ena(&self) -> VArPctEna {
        (self.v_ar_pct_ena_callback)(self.context)
    }

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn set_v_ar_pct_ena(&mut self, value: VArPctEna) {
        (self.set_v_ar_pct_ena_callback)(value, self.context);
    }

    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    fn w_max_lim_pct_sf(&self) -> u16 {
        (self.w_max_lim_pct_sf_callback)(self.context)
    }

    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    fn out_pf_set_sf(&self) -> u16 {
        (self.out_pf_set_sf_callback)(self.context)
    }

    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    fn v_ar_pct_sf(&self) -> Option<u16> {
        self.v_ar_pct_sf_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model123StatefulAdapter {
    conn_win_tms: u16,
    conn_rvrt_tms: u16,
    conn: Conn,
    w_max_lim_pct: u16,
    w_max_lim_pct_win_tms: u16,
    w_max_lim_pct_rvrt_tms: u16,
    w_max_lim_pct_rmp_tms: u16,
    w_max_lim_ena: WMaxLimEna,
    out_pf_set: i16,
    out_pf_set_win_tms: u16,
    out_pf_set_rvrt_tms: u16,
    out_pf_set_rmp_tms: u16,
    out_pf_set_ena: OutPfSetEna,
    v_ar_w_max_pct: i16,
    v_ar_max_pct: i16,
    v_ar_aval_pct: i16,
    v_ar_pct_win_tms: u16,
    v_ar_pct_rvrt_tms: u16,
    v_ar_pct_rmp_tms: u16,
    v_ar_pct_mod: VArPctMod,
    v_ar_pct_ena: VArPctEna,
    w_max_lim_pct_sf: u16,
    out_pf_set_sf: u16,
    v_ar_pct_sf: u16,
}

impl ModelAdapter for Model123StatefulAdapter {
    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn conn_win_tms(&self) -> Option<u16> {
        Some(self.conn_win_tms)
    }

    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn set_conn_win_tms(&mut self, value: u16) {
        self.conn_win_tms = value;
    }

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn conn_rvrt_tms(&self) -> Option<u16> {
        Some(self.conn_rvrt_tms)
    }

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn set_conn_rvrt_tms(&mut self, value: u16) {
        self.conn_rvrt_tms = value;
    }

    /// Conn
    ///
    /// Connection control.
    fn conn(&self) -> Conn {
        self.conn
    }

    /// Conn
    ///
    /// Connection control.
    fn set_conn(&mut self, value: Conn) {
        self.conn = value;
    }

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn w_max_lim_pct(&self) -> u16 {
        self.w_max_lim_pct
    }

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn set_w_max_lim_pct(&mut self, value: u16) {
        self.w_max_lim_pct = value;
    }

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn w_max_lim_pct_win_tms(&self) -> Option<u16> {
        Some(self.w_max_lim_pct_win_tms)
    }

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn set_w_max_lim_pct_win_tms(&mut self, value: u16) {
        self.w_max_lim_pct_win_tms = value;
    }

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn w_max_lim_pct_rvrt_tms(&self) -> Option<u16> {
        Some(self.w_max_lim_pct_rvrt_tms)
    }

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn set_w_max_lim_pct_rvrt_tms(&mut self, value: u16) {
        self.w_max_lim_pct_rvrt_tms = value;
    }

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn w_max_lim_pct_rmp_tms(&self) -> Option<u16> {
        Some(self.w_max_lim_pct_rmp_tms)
    }

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_w_max_lim_pct_rmp_tms(&mut self, value: u16) {
        self.w_max_lim_pct_rmp_tms = value;
    }

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn w_max_lim_ena(&self) -> WMaxLimEna {
        self.w_max_lim_ena
    }

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn set_w_max_lim_ena(&mut self, value: WMaxLimEna) {
        self.w_max_lim_ena = value;
    }

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn out_pf_set(&self) -> i16 {
        self.out_pf_set
    }

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn set_out_pf_set(&mut self, value: i16) {
        self.out_pf_set = value;
    }

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn out_pf_set_win_tms(&self) -> Option<u16> {
        Some(self.out_pf_set_win_tms)
    }

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn set_out_pf_set_win_tms(&mut self, value: u16) {
        self.out_pf_set_win_tms = value;
    }

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn out_pf_set_rvrt_tms(&self) -> Option<u16> {
        Some(self.out_pf_set_rvrt_tms)
    }

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn set_out_pf_set_rvrt_tms(&mut self, value: u16) {
        self.out_pf_set_rvrt_tms = value;
    }

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn out_pf_set_rmp_tms(&self) -> Option<u16> {
        Some(self.out_pf_set_rmp_tms)
    }

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_out_pf_set_rmp_tms(&mut self, value: u16) {
        self.out_pf_set_rmp_tms = value;
    }

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn out_pf_set_ena(&self) -> OutPfSetEna {
        self.out_pf_set_ena
    }

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn set_out_pf_set_ena(&mut self, value: OutPfSetEna) {
        self.out_pf_set_ena = value;
    }

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn v_ar_w_max_pct(&self) -> Option<i16> {
        Some(self.v_ar_w_max_pct)
    }

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn set_v_ar_w_max_pct(&mut self, value: i16) {
        self.v_ar_w_max_pct = value;
    }

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn v_ar_max_pct(&self) -> Option<i16> {
        Some(self.v_ar_max_pct)
    }

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn set_v_ar_max_pct(&mut self, value: i16) {
        self.v_ar_max_pct = value;
    }

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn v_ar_aval_pct(&self) -> Option<i16> {
        Some(self.v_ar_aval_pct)
    }

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn set_v_ar_aval_pct(&mut self, value: i16) {
        self.v_ar_aval_pct = value;
    }

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn v_ar_pct_win_tms(&self) -> Option<u16> {
        Some(self.v_ar_pct_win_tms)
    }

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn set_v_ar_pct_win_tms(&mut self, value: u16) {
        self.v_ar_pct_win_tms = value;
    }

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn v_ar_pct_rvrt_tms(&self) -> Option<u16> {
        Some(self.v_ar_pct_rvrt_tms)
    }

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn set_v_ar_pct_rvrt_tms(&mut self, value: u16) {
        self.v_ar_pct_rvrt_tms = value;
    }

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn v_ar_pct_rmp_tms(&self) -> Option<u16> {
        Some(self.v_ar_pct_rmp_tms)
    }

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_v_ar_pct_rmp_tms(&mut self, value: u16) {
        self.v_ar_pct_rmp_tms = value;
    }

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn v_ar_pct_mod(&self) -> Option<VArPctMod> {
        Some(self.v_ar_pct_mod)
    }

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn set_v_ar_pct_mod(&mut self, value: VArPctMod) {
        self.v_ar_pct_mod = value;
    }

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn v_ar_pct_ena(&self) -> VArPctEna {
        self.v_ar_pct_ena
    }

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn set_v_ar_pct_ena(&mut self, value: VArPctEna) {
        self.v_ar_pct_ena = value;
    }

    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    fn w_max_lim_pct_sf(&self) -> u16 {
        self.w_max_lim_pct_sf
    }

    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    fn out_pf_set_sf(&self) -> u16 {
        self.out_pf_set_sf
    }

    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    fn v_ar_pct_sf(&self) -> Option<u16> {
        Some(self.v_ar_pct_sf)
    }
}
