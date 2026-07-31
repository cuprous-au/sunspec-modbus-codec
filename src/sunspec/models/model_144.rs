use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 12;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 144 },
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
        reference: PointReference::Model144 {
            point: Point::ActCrv,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model144 {
            point: Point::ModEna,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model144 {
            point: Point::WinTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model144 {
            point: Point::RvrtTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model144 {
            point: Point::RmpTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model144 { point: Point::NCrv },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model144 { point: Point::NPt },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model144 {
            point: Point::TmsSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model144 { point: Point::HzSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model144 {
            point: Point::CrvType,
        },
        size: 1,
        data_type: PointType::Enum16,
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
    CrvType,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ActCrv => serialisation::write_u16(model.act_crv(), buffer),
        Point::ModEna => serialisation::write_u16(model.mod_ena(), buffer),
        Point::WinTms => {
            if let Some(value) = model.win_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::RvrtTms => {
            if let Some(value) = model.rvrt_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::RmpTms => {
            if let Some(value) = model.rmp_tms() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::NCrv => serialisation::write_u16(model.n_crv(), buffer),
        Point::NPt => serialisation::write_u16(model.n_pt(), buffer),
        Point::TmsSf => serialisation::write_u16(model.tms_sf(), buffer),
        Point::HzSf => serialisation::write_u16(model.hz_sf(), buffer),
        Point::CrvType => serialisation::write_u16(model.crv_type() as u16, buffer),
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
    /// LHzRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Setting is ignored for LFRT controls.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Setting is ignored for LFRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LFRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_rmp_tms(&mut self, value: u16) {}

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

    fn crv_type(&self) -> CrvType;
}

pub enum CrvType {
    CeaseToEnergize = 1,
}

#[repr(C)]
pub struct Model144CallbackAdapter {
    act_crv_callback: extern "C" fn() -> u16,
    set_act_crv_callback: extern "C" fn(u16),
    mod_ena_callback: extern "C" fn() -> u16,
    set_mod_ena_callback: extern "C" fn(u16),
    win_tms_callback: Option<extern "C" fn() -> u16>,
    set_win_tms_callback: Option<extern "C" fn(u16)>,
    rvrt_tms_callback: Option<extern "C" fn() -> u16>,
    set_rvrt_tms_callback: Option<extern "C" fn(u16)>,
    rmp_tms_callback: Option<extern "C" fn() -> u16>,
    set_rmp_tms_callback: Option<extern "C" fn(u16)>,
    n_crv_callback: extern "C" fn() -> u16,
    n_pt_callback: extern "C" fn() -> u16,
    tms_sf_callback: extern "C" fn() -> u16,
    hz_sf_callback: extern "C" fn() -> u16,
    crv_type_callback: extern "C" fn() -> CrvType,
}

impl ModelAdapter for Model144CallbackAdapter {
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn act_crv(&self) -> u16 {
        (self.act_crv_callback)()
    }

    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn set_act_crv(&mut self, value: u16) {
        (self.set_act_crv_callback)(value);
    }

    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)()
    }

    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value);
    }

    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Setting is ignored for LFRT controls.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback.map(|callback| (callback)())
    }

    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
            (callback)(value);
        };
    }

    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Setting is ignored for LFRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        self.rvrt_tms_callback.map(|callback| (callback)())
    }

    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rvrt_tms_callback {
            (callback)(value);
        };
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LFRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        self.rmp_tms_callback.map(|callback| (callback)())
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rmp_tms_callback {
            (callback)(value);
        };
    }

    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16 {
        (self.n_crv_callback)()
    }

    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16 {
        (self.n_pt_callback)()
    }

    /// Tms_SF
    ///
    /// Scale factor for duration.
    fn tms_sf(&self) -> u16 {
        (self.tms_sf_callback)()
    }

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16 {
        (self.hz_sf_callback)()
    }

    fn crv_type(&self) -> CrvType {
        (self.crv_type_callback)()
    }
}
