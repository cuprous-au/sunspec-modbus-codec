use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 26;

pub static POINTS: [ReadablePoint; 26] = [
    ReadablePoint {
        reference: PointReference::Static { value: 123 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 24 },
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ConnWinTms => {
            if let Some(value) = model.conn_win_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ConnRvrtTms => {
            if let Some(value) = model.conn_rvrt_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Conn => serialisation::write_u16(model.conn() as u16, buffer),
        Point::WMaxLimPct => serialisation::write_u16(model.w_max_lim_pct(), buffer),
        Point::WMaxLimPctWinTms => {
            if let Some(value) = model.w_max_lim_pct_win_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WMaxLimPctRvrtTms => {
            if let Some(value) = model.w_max_lim_pct_rvrt_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WMaxLimPctRmpTms => {
            if let Some(value) = model.w_max_lim_pct_rmp_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WMaxLimEna => serialisation::write_u16(model.w_max_lim_ena() as u16, buffer),
        Point::OutPfSet => serialisation::write_i16(model.out_pf_set(), buffer),
        Point::OutPfSetWinTms => {
            if let Some(value) = model.out_pf_set_win_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::OutPfSetRvrtTms => {
            if let Some(value) = model.out_pf_set_rvrt_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::OutPfSetRmpTms => {
            if let Some(value) = model.out_pf_set_rmp_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::OutPfSetEna => serialisation::write_u16(model.out_pf_set_ena() as u16, buffer),
        Point::VArWMaxPct => {
            if let Some(value) = model.v_ar_w_max_pct() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArMaxPct => {
            if let Some(value) = model.v_ar_max_pct() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArAvalPct => {
            if let Some(value) = model.v_ar_aval_pct() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArPctWinTms => {
            if let Some(value) = model.v_ar_pct_win_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VArPctRvrtTms => {
            if let Some(value) = model.v_ar_pct_rvrt_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VArPctRmpTms => {
            if let Some(value) = model.v_ar_pct_rmp_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VArPctMod => {
            if let Some(value) = model.v_ar_pct_mod() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::VArPctEna => serialisation::write_u16(model.v_ar_pct_ena() as u16, buffer),
        Point::WMaxLimPctSf => serialisation::write_u16(model.w_max_lim_pct_sf(), buffer),
        Point::OutPfSetSf => serialisation::write_u16(model.out_pf_set_sf(), buffer),
        Point::VArPctSf => {
            if let Some(value) = model.v_ar_pct_sf() {
                serialisation::write_u16(value, buffer);
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

pub enum Conn {
    Disconnect = 0,
    Connect = 1,
}

pub enum WMaxLimEna {
    Disabled = 0,
    Enabled = 1,
}

pub enum OutPfSetEna {
    Disabled = 0,
    Enabled = 1,
}

pub enum VArPctMod {
    None = 0,
    WMax = 1,
    VArMax = 2,
    VArAval = 3,
}

pub enum VArPctEna {
    Disabled = 0,
    Enabled = 1,
}

#[repr(C)]
pub struct Model123CallbackAdapter {
    conn_win_tms_callback: Option<extern "C" fn() -> u16>,
    set_conn_win_tms_callback: Option<extern "C" fn(u16)>,
    conn_rvrt_tms_callback: Option<extern "C" fn() -> u16>,
    set_conn_rvrt_tms_callback: Option<extern "C" fn(u16)>,
    conn_callback: extern "C" fn() -> Conn,
    set_conn_callback: extern "C" fn(Conn),
    w_max_lim_pct_callback: extern "C" fn() -> u16,
    set_w_max_lim_pct_callback: extern "C" fn(u16),
    w_max_lim_pct_win_tms_callback: Option<extern "C" fn() -> u16>,
    set_w_max_lim_pct_win_tms_callback: Option<extern "C" fn(u16)>,
    w_max_lim_pct_rvrt_tms_callback: Option<extern "C" fn() -> u16>,
    set_w_max_lim_pct_rvrt_tms_callback: Option<extern "C" fn(u16)>,
    w_max_lim_pct_rmp_tms_callback: Option<extern "C" fn() -> u16>,
    set_w_max_lim_pct_rmp_tms_callback: Option<extern "C" fn(u16)>,
    w_max_lim_ena_callback: extern "C" fn() -> WMaxLimEna,
    set_w_max_lim_ena_callback: extern "C" fn(WMaxLimEna),
    out_pf_set_callback: extern "C" fn() -> i16,
    set_out_pf_set_callback: extern "C" fn(i16),
    out_pf_set_win_tms_callback: Option<extern "C" fn() -> u16>,
    set_out_pf_set_win_tms_callback: Option<extern "C" fn(u16)>,
    out_pf_set_rvrt_tms_callback: Option<extern "C" fn() -> u16>,
    set_out_pf_set_rvrt_tms_callback: Option<extern "C" fn(u16)>,
    out_pf_set_rmp_tms_callback: Option<extern "C" fn() -> u16>,
    set_out_pf_set_rmp_tms_callback: Option<extern "C" fn(u16)>,
    out_pf_set_ena_callback: extern "C" fn() -> OutPfSetEna,
    set_out_pf_set_ena_callback: extern "C" fn(OutPfSetEna),
    v_ar_w_max_pct_callback: Option<extern "C" fn() -> i16>,
    set_v_ar_w_max_pct_callback: Option<extern "C" fn(i16)>,
    v_ar_max_pct_callback: Option<extern "C" fn() -> i16>,
    set_v_ar_max_pct_callback: Option<extern "C" fn(i16)>,
    v_ar_aval_pct_callback: Option<extern "C" fn() -> i16>,
    set_v_ar_aval_pct_callback: Option<extern "C" fn(i16)>,
    v_ar_pct_win_tms_callback: Option<extern "C" fn() -> u16>,
    set_v_ar_pct_win_tms_callback: Option<extern "C" fn(u16)>,
    v_ar_pct_rvrt_tms_callback: Option<extern "C" fn() -> u16>,
    set_v_ar_pct_rvrt_tms_callback: Option<extern "C" fn(u16)>,
    v_ar_pct_rmp_tms_callback: Option<extern "C" fn() -> u16>,
    set_v_ar_pct_rmp_tms_callback: Option<extern "C" fn(u16)>,
    v_ar_pct_mod_callback: Option<extern "C" fn() -> VArPctMod>,
    set_v_ar_pct_mod_callback: Option<extern "C" fn(VArPctMod)>,
    v_ar_pct_ena_callback: extern "C" fn() -> VArPctEna,
    set_v_ar_pct_ena_callback: extern "C" fn(VArPctEna),
    w_max_lim_pct_sf_callback: extern "C" fn() -> u16,
    out_pf_set_sf_callback: extern "C" fn() -> u16,
    v_ar_pct_sf_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model123CallbackAdapter {
    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn conn_win_tms(&self) -> Option<u16> {
        self.conn_win_tms_callback.map(|callback| (callback)())
    }

    /// Conn_WinTms
    ///
    /// Time window for connect/disconnect.
    fn set_conn_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_conn_win_tms_callback {
            (callback)(value);
        };
    }

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn conn_rvrt_tms(&self) -> Option<u16> {
        self.conn_rvrt_tms_callback.map(|callback| (callback)())
    }

    /// Conn_RvrtTms
    ///
    /// Timeout period for connect/disconnect.
    fn set_conn_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_conn_rvrt_tms_callback {
            (callback)(value);
        };
    }

    /// Conn
    ///
    /// Connection control.
    fn conn(&self) -> Conn {
        (self.conn_callback)()
    }

    /// Conn
    ///
    /// Connection control.
    fn set_conn(&mut self, value: Conn) {
        (self.set_conn_callback)(value);
    }

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn w_max_lim_pct(&self) -> u16 {
        (self.w_max_lim_pct_callback)()
    }

    /// WMaxLimPct
    ///
    /// Set power output to specified level.
    fn set_w_max_lim_pct(&mut self, value: u16) {
        (self.set_w_max_lim_pct_callback)(value);
    }

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn w_max_lim_pct_win_tms(&self) -> Option<u16> {
        self.w_max_lim_pct_win_tms_callback
            .map(|callback| (callback)())
    }

    /// WMaxLimPct_WinTms
    ///
    /// Time window for power limit change.
    fn set_w_max_lim_pct_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_w_max_lim_pct_win_tms_callback {
            (callback)(value);
        };
    }

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn w_max_lim_pct_rvrt_tms(&self) -> Option<u16> {
        self.w_max_lim_pct_rvrt_tms_callback
            .map(|callback| (callback)())
    }

    /// WMaxLimPct_RvrtTms
    ///
    /// Timeout period for power limit.
    fn set_w_max_lim_pct_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_w_max_lim_pct_rvrt_tms_callback {
            (callback)(value);
        };
    }

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn w_max_lim_pct_rmp_tms(&self) -> Option<u16> {
        self.w_max_lim_pct_rmp_tms_callback
            .map(|callback| (callback)())
    }

    /// WMaxLimPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_w_max_lim_pct_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_w_max_lim_pct_rmp_tms_callback {
            (callback)(value);
        };
    }

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn w_max_lim_ena(&self) -> WMaxLimEna {
        (self.w_max_lim_ena_callback)()
    }

    /// WMaxLim_Ena
    ///
    /// Throttle enable/disable control.
    fn set_w_max_lim_ena(&mut self, value: WMaxLimEna) {
        (self.set_w_max_lim_ena_callback)(value);
    }

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn out_pf_set(&self) -> i16 {
        (self.out_pf_set_callback)()
    }

    /// OutPFSet
    ///
    /// Set power factor to specific value - cosine of angle.
    fn set_out_pf_set(&mut self, value: i16) {
        (self.set_out_pf_set_callback)(value);
    }

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn out_pf_set_win_tms(&self) -> Option<u16> {
        self.out_pf_set_win_tms_callback
            .map(|callback| (callback)())
    }

    /// OutPFSet_WinTms
    ///
    /// Time window for power factor change.
    fn set_out_pf_set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_out_pf_set_win_tms_callback {
            (callback)(value);
        };
    }

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn out_pf_set_rvrt_tms(&self) -> Option<u16> {
        self.out_pf_set_rvrt_tms_callback
            .map(|callback| (callback)())
    }

    /// OutPFSet_RvrtTms
    ///
    /// Timeout period for power factor.
    fn set_out_pf_set_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_out_pf_set_rvrt_tms_callback {
            (callback)(value);
        };
    }

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn out_pf_set_rmp_tms(&self) -> Option<u16> {
        self.out_pf_set_rmp_tms_callback
            .map(|callback| (callback)())
    }

    /// OutPFSet_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_out_pf_set_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_out_pf_set_rmp_tms_callback {
            (callback)(value);
        };
    }

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn out_pf_set_ena(&self) -> OutPfSetEna {
        (self.out_pf_set_ena_callback)()
    }

    /// OutPFSet_Ena
    ///
    /// Fixed power factor enable/disable control.
    fn set_out_pf_set_ena(&mut self, value: OutPfSetEna) {
        (self.set_out_pf_set_ena_callback)(value);
    }

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn v_ar_w_max_pct(&self) -> Option<i16> {
        self.v_ar_w_max_pct_callback.map(|callback| (callback)())
    }

    /// VArWMaxPct
    ///
    /// Reactive power in percent of WMax.
    fn set_v_ar_w_max_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_w_max_pct_callback {
            (callback)(value);
        };
    }

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn v_ar_max_pct(&self) -> Option<i16> {
        self.v_ar_max_pct_callback.map(|callback| (callback)())
    }

    /// VArMaxPct
    ///
    /// Reactive power in percent of VArMax.
    fn set_v_ar_max_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_max_pct_callback {
            (callback)(value);
        };
    }

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn v_ar_aval_pct(&self) -> Option<i16> {
        self.v_ar_aval_pct_callback.map(|callback| (callback)())
    }

    /// VArAvalPct
    ///
    /// Reactive power in percent of VArAval.
    fn set_v_ar_aval_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_v_ar_aval_pct_callback {
            (callback)(value);
        };
    }

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn v_ar_pct_win_tms(&self) -> Option<u16> {
        self.v_ar_pct_win_tms_callback.map(|callback| (callback)())
    }

    /// VArPct_WinTms
    ///
    /// Time window for VAR limit change.
    fn set_v_ar_pct_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_v_ar_pct_win_tms_callback {
            (callback)(value);
        };
    }

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn v_ar_pct_rvrt_tms(&self) -> Option<u16> {
        self.v_ar_pct_rvrt_tms_callback.map(|callback| (callback)())
    }

    /// VArPct_RvrtTms
    ///
    /// Timeout period for VAR limit.
    fn set_v_ar_pct_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_v_ar_pct_rvrt_tms_callback {
            (callback)(value);
        };
    }

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn v_ar_pct_rmp_tms(&self) -> Option<u16> {
        self.v_ar_pct_rmp_tms_callback.map(|callback| (callback)())
    }

    /// VArPct_RmpTms
    ///
    /// Ramp time for moving from current setpoint to new setpoint.
    fn set_v_ar_pct_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_v_ar_pct_rmp_tms_callback {
            (callback)(value);
        };
    }

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn v_ar_pct_mod(&self) -> Option<VArPctMod> {
        self.v_ar_pct_mod_callback.map(|callback| (callback)())
    }

    /// VArPct_Mod
    ///
    /// VAR percent limit mode.
    fn set_v_ar_pct_mod(&mut self, value: VArPctMod) {
        if let Some(callback) = self.set_v_ar_pct_mod_callback {
            (callback)(value);
        };
    }

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn v_ar_pct_ena(&self) -> VArPctEna {
        (self.v_ar_pct_ena_callback)()
    }

    /// VArPct_Ena
    ///
    /// Percent limit VAr enable/disable control.
    fn set_v_ar_pct_ena(&mut self, value: VArPctEna) {
        (self.set_v_ar_pct_ena_callback)(value);
    }

    /// WMaxLimPct_SF
    ///
    /// Scale factor for power output percent.
    fn w_max_lim_pct_sf(&self) -> u16 {
        (self.w_max_lim_pct_sf_callback)()
    }

    /// OutPFSet_SF
    ///
    /// Scale factor for power factor.
    fn out_pf_set_sf(&self) -> u16 {
        (self.out_pf_set_sf_callback)()
    }

    /// VArPct_SF
    ///
    /// Scale factor for reactive power percent.
    fn v_ar_pct_sf(&self) -> Option<u16> {
        self.v_ar_pct_sf_callback.map(|callback| (callback)())
    }
}
