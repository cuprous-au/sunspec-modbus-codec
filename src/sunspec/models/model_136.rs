use core::ffi::c_void;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 12;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 136 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 10 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::ActCrv },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::ModEna },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::WinTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::RvrtTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::RmpTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::NCrv },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::NPt },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::TmsSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model136 { point: Point::HzSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ActCrv,
    ModEna,
    WinTms,
    RvrtTms,
    RmpTms,
    NCrv,
    NPt,
    TmsSf,
    HzSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::ActCrv => {
            serialisation::write_u16(model.act_crv(), buffer);
        },
        Point::ModEna => {
            serialisation::write_u16(model.mod_ena(), buffer);
        },
        Point::WinTms => {
            if let Some(value) = model.win_tms() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::RvrtTms => {
            if let Some(value) = model.rvrt_tms() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::RmpTms => {
            if let Some(value) = model.rmp_tms() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::NCrv => {
            serialisation::write_u16(model.n_crv(), buffer);
        },
        Point::NPt => {
            serialisation::write_u16(model.n_pt(), buffer);
        },
        Point::TmsSf => {
            serialisation::write_u16(model.tms_sf(), buffer);
        },
        Point::HzSf => {
            serialisation::write_u16(model.hz_sf(), buffer);
        },
    }
}

pub trait ModelAdapter {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn act_crv(&self) -> u16;

    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn set_act_crv(&mut self, value: u16);

    /// ModEna
    ///
    /// HFRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// HFRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for HFRT change.
    ///
    /// Setting is ignored for HFRT controls.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for HFRT change.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_win_tms(&mut self, value: u16) {
    }

    /// RvrtTms
    ///
    /// Timeout period for HFRT curve selection.
    ///
    /// Setting is ignored for HFRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for HFRT curve selection.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for HFRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_rmp_tms(&mut self, value: u16) {
    }

    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16;

    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16;

    /// Tms_SF
    ///
    /// Scale factor for duration.
    fn tms_sf(&self) -> u16;

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16;
}

#[repr(C)]
pub struct Model136CallbackAdapter {
    context: *mut c_void,
    act_crv_callback: extern "C" fn(*const c_void) -> u16,
    set_act_crv_callback: extern "C" fn(u16, *mut c_void),
    mod_ena_callback: extern "C" fn(*const c_void) -> u16,
    set_mod_ena_callback: extern "C" fn(u16, *mut c_void),
    win_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_win_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    rvrt_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_rvrt_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    rmp_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_rmp_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    n_crv_callback: extern "C" fn(*const c_void) -> u16,
    n_pt_callback: extern "C" fn(*const c_void) -> u16,
    tms_sf_callback: extern "C" fn(*const c_void) -> u16,
    hz_sf_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model136CallbackAdapter {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn act_crv(&self) -> u16 {
        (self.act_crv_callback)(self.context)
    }

    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn set_act_crv(&mut self, value: u16) {
        (self.set_act_crv_callback)(value, self.context);
    }

    /// ModEna
    ///
    /// HFRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// HFRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// WinTms
    ///
    /// Time window for HFRT change.
    ///
    /// Setting is ignored for HFRT controls.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// WinTms
    ///
    /// Time window for HFRT change.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
        (callback)(value, self.context);
        };
    }

    /// RvrtTms
    ///
    /// Timeout period for HFRT curve selection.
    ///
    /// Setting is ignored for HFRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        self.rvrt_tms_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// RvrtTms
    ///
    /// Timeout period for HFRT curve selection.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rvrt_tms_callback {
        (callback)(value, self.context);
        };
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for HFRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        self.rmp_tms_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rmp_tms_callback {
        (callback)(value, self.context);
        };
    }

    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16 {
        (self.n_crv_callback)(self.context)
    }

    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16 {
        (self.n_pt_callback)(self.context)
    }

    /// Tms_SF
    ///
    /// Scale factor for duration.
    fn tms_sf(&self) -> u16 {
        (self.tms_sf_callback)(self.context)
    }

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16 {
        (self.hz_sf_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model136StatefulAdapter {
    act_crv: u16,
    mod_ena: u16,
    win_tms: u16,
    rvrt_tms: u16,
    rmp_tms: u16,
    n_crv: u16,
    n_pt: u16,
    tms_sf: u16,
    hz_sf: u16,
}

impl ModelAdapter for Model136StatefulAdapter {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn act_crv(&self) -> u16 {
        self.act_crv
    }

    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn set_act_crv(&mut self, value: u16) {
        self.act_crv = value;
    }

    /// ModEna
    ///
    /// HFRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// HFRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// WinTms
    ///
    /// Time window for HFRT change.
    ///
    /// Setting is ignored for HFRT controls.
    fn win_tms(&self) -> Option<u16> {
        Some(
        self.win_tms
        )
    }

    /// WinTms
    ///
    /// Time window for HFRT change.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_win_tms(&mut self, value: u16) {
        self.win_tms = value;
    }

    /// RvrtTms
    ///
    /// Timeout period for HFRT curve selection.
    ///
    /// Setting is ignored for HFRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        Some(
        self.rvrt_tms
        )
    }

    /// RvrtTms
    ///
    /// Timeout period for HFRT curve selection.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {
        self.rvrt_tms = value;
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for HFRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        Some(
        self.rmp_tms
        )
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for HFRT controls.
    fn set_rmp_tms(&mut self, value: u16) {
        self.rmp_tms = value;
    }

    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16 {
        self.n_crv
    }

    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16 {
        self.n_pt
    }

    /// Tms_SF
    ///
    /// Scale factor for duration.
    fn tms_sf(&self) -> u16 {
        self.tms_sf
    }

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16 {
        self.hz_sf
    }
}