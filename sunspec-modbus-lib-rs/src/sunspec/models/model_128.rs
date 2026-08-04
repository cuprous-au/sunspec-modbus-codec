use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 16;

pub static POINTS: [ReadablePoint; 16] = [
    ReadablePoint {
        reference: PointReference::Static { value: 128 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 14 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ArGraMod },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ArGraSag },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ArGraSwell },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ModEna },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::FilTms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::DbVMin },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::DbVMax },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::BlkZnV },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::HysBlkZnV },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::BlkZnTmms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::HoldTmms },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::ArGraSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model128 { point: Point::VRefPctSf },
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
    ArGraMod,
    ArGraSag,
    ArGraSwell,
    ModEna,
    FilTms,
    DbVMin,
    DbVMax,
    BlkZnV,
    HysBlkZnV,
    BlkZnTmms,
    HoldTmms,
    ArGraSf,
    VRefPctSf,
}

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::ArGraMod => {
            buffer::write_u16(model.ar_gra_mod() as u16, buffer);
        },
        Point::ArGraSag => {
            buffer::write_u16(model.ar_gra_sag(), buffer);
        },
        Point::ArGraSwell => {
            buffer::write_u16(model.ar_gra_swell(), buffer);
        },
        Point::ModEna => {
            buffer::write_u16(model.mod_ena(), buffer);
        },
        Point::FilTms => {
            if let Some(value) = model.fil_tms() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DbVMin => {
            if let Some(value) = model.db_v_min() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DbVMax => {
            if let Some(value) = model.db_v_max() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::BlkZnV => {
            if let Some(value) = model.blk_zn_v() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::HysBlkZnV => {
            if let Some(value) = model.hys_blk_zn_v() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::BlkZnTmms => {
            if let Some(value) = model.blk_zn_tmms() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::HoldTmms => {
            if let Some(value) = model.hold_tmms() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ArGraSf => {
            buffer::write_u16(model.ar_gra_sf(), buffer);
        },
        Point::VRefPctSf => {
            if let Some(value) = model.v_ref_pct_sf() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn ar_gra_mod(&self) -> ArGraMod;

    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn set_ar_gra_mod(&mut self, value: ArGraMod);

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_sag(&self) -> u16;

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_sag(&mut self, value: u16);

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_swell(&self) -> u16;

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_swell(&mut self, value: u16);

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn set_mod_ena(&mut self, value: u16);

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn fil_tms(&self) -> Option<u16> {
        None
    }

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn set_fil_tms(&mut self, value: u16) {
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn db_v_min(&self) -> Option<u16> {
        None
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn set_db_v_min(&mut self, value: u16) {
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn db_v_max(&self) -> Option<u16> {
        None
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn set_db_v_max(&mut self, value: u16) {
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn blk_zn_v(&self) -> Option<u16> {
        None
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn set_blk_zn_v(&mut self, value: u16) {
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn hys_blk_zn_v(&self) -> Option<u16> {
        None
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn set_hys_blk_zn_v(&mut self, value: u16) {
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn blk_zn_tmms(&self) -> Option<u16> {
        None
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn set_blk_zn_tmms(&mut self, value: u16) {
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn hold_tmms(&self) -> Option<u16> {
        None
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn set_hold_tmms(&mut self, value: u16) {
    }

    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    fn ar_gra_sf(&self) -> u16;

    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    fn v_ref_pct_sf(&self) -> Option<u16> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ArGraMod {
    Edge = 0,
    Center = 1,
}

#[repr(C)]
pub struct Model128CallbackAdapter {
    context: *mut c_void,
    ar_gra_mod_callback: extern "C" fn(*const c_void) -> ArGraMod,
    set_ar_gra_mod_callback: extern "C" fn(ArGraMod, *mut c_void),
    ar_gra_sag_callback: extern "C" fn(*const c_void) -> u16,
    set_ar_gra_sag_callback: extern "C" fn(u16, *mut c_void),
    ar_gra_swell_callback: extern "C" fn(*const c_void) -> u16,
    set_ar_gra_swell_callback: extern "C" fn(u16, *mut c_void),
    mod_ena_callback: extern "C" fn(*const c_void) -> u16,
    set_mod_ena_callback: extern "C" fn(u16, *mut c_void),
    fil_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_fil_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    db_v_min_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_db_v_min_callback: Option<extern "C" fn(u16, *mut c_void)>,
    db_v_max_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_db_v_max_callback: Option<extern "C" fn(u16, *mut c_void)>,
    blk_zn_v_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_blk_zn_v_callback: Option<extern "C" fn(u16, *mut c_void)>,
    hys_blk_zn_v_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_hys_blk_zn_v_callback: Option<extern "C" fn(u16, *mut c_void)>,
    blk_zn_tmms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_blk_zn_tmms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    hold_tmms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_hold_tmms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    ar_gra_sf_callback: extern "C" fn(*const c_void) -> u16,
    v_ref_pct_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model128CallbackAdapter {
    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn ar_gra_mod(&self) -> ArGraMod {
        (self.ar_gra_mod_callback)(self.context)
    }

    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn set_ar_gra_mod(&mut self, value: ArGraMod) {
        (self.set_ar_gra_mod_callback)(value, self.context);
    }

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_sag(&self) -> u16 {
        (self.ar_gra_sag_callback)(self.context)
    }

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_sag(&mut self, value: u16) {
        (self.set_ar_gra_sag_callback)(value, self.context);
    }

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_swell(&self) -> u16 {
        (self.ar_gra_swell_callback)(self.context)
    }

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_swell(&mut self, value: u16) {
        (self.set_ar_gra_swell_callback)(value, self.context);
    }

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn fil_tms(&self) -> Option<u16> {
        self.fil_tms_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn set_fil_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_fil_tms_callback {
        (callback)(value, self.context);
        };
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn db_v_min(&self) -> Option<u16> {
        self.db_v_min_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn set_db_v_min(&mut self, value: u16) {
        if let Some(callback) = self.set_db_v_min_callback {
        (callback)(value, self.context);
        };
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn db_v_max(&self) -> Option<u16> {
        self.db_v_max_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn set_db_v_max(&mut self, value: u16) {
        if let Some(callback) = self.set_db_v_max_callback {
        (callback)(value, self.context);
        };
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn blk_zn_v(&self) -> Option<u16> {
        self.blk_zn_v_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn set_blk_zn_v(&mut self, value: u16) {
        if let Some(callback) = self.set_blk_zn_v_callback {
        (callback)(value, self.context);
        };
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn hys_blk_zn_v(&self) -> Option<u16> {
        self.hys_blk_zn_v_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn set_hys_blk_zn_v(&mut self, value: u16) {
        if let Some(callback) = self.set_hys_blk_zn_v_callback {
        (callback)(value, self.context);
        };
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn blk_zn_tmms(&self) -> Option<u16> {
        self.blk_zn_tmms_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn set_blk_zn_tmms(&mut self, value: u16) {
        if let Some(callback) = self.set_blk_zn_tmms_callback {
        (callback)(value, self.context);
        };
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn hold_tmms(&self) -> Option<u16> {
        self.hold_tmms_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn set_hold_tmms(&mut self, value: u16) {
        if let Some(callback) = self.set_hold_tmms_callback {
        (callback)(value, self.context);
        };
    }

    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    fn ar_gra_sf(&self) -> u16 {
        (self.ar_gra_sf_callback)(self.context)
    }

    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    fn v_ref_pct_sf(&self) -> Option<u16> {
        self.v_ref_pct_sf_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model128StatefulAdapter {
    ar_gra_mod: ArGraMod,
    ar_gra_sag: u16,
    ar_gra_swell: u16,
    mod_ena: u16,
    fil_tms: u16,
    db_v_min: u16,
    db_v_max: u16,
    blk_zn_v: u16,
    hys_blk_zn_v: u16,
    blk_zn_tmms: u16,
    hold_tmms: u16,
    ar_gra_sf: u16,
    v_ref_pct_sf: u16,
}

impl ModelAdapter for Model128StatefulAdapter {
    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn ar_gra_mod(&self) -> ArGraMod {
        self.ar_gra_mod
    }

    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn set_ar_gra_mod(&mut self, value: ArGraMod) {
        self.ar_gra_mod = value;
    }

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_sag(&self) -> u16 {
        self.ar_gra_sag
    }

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_sag(&mut self, value: u16) {
        self.ar_gra_sag = value;
    }

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_swell(&self) -> u16 {
        self.ar_gra_swell
    }

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_swell(&mut self, value: u16) {
        self.ar_gra_swell = value;
    }

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn fil_tms(&self) -> Option<u16> {
        Some(
        self.fil_tms
        )
    }

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn set_fil_tms(&mut self, value: u16) {
        self.fil_tms = value;
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn db_v_min(&self) -> Option<u16> {
        Some(
        self.db_v_min
        )
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn set_db_v_min(&mut self, value: u16) {
        self.db_v_min = value;
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn db_v_max(&self) -> Option<u16> {
        Some(
        self.db_v_max
        )
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn set_db_v_max(&mut self, value: u16) {
        self.db_v_max = value;
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn blk_zn_v(&self) -> Option<u16> {
        Some(
        self.blk_zn_v
        )
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn set_blk_zn_v(&mut self, value: u16) {
        self.blk_zn_v = value;
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn hys_blk_zn_v(&self) -> Option<u16> {
        Some(
        self.hys_blk_zn_v
        )
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn set_hys_blk_zn_v(&mut self, value: u16) {
        self.hys_blk_zn_v = value;
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn blk_zn_tmms(&self) -> Option<u16> {
        Some(
        self.blk_zn_tmms
        )
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn set_blk_zn_tmms(&mut self, value: u16) {
        self.blk_zn_tmms = value;
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn hold_tmms(&self) -> Option<u16> {
        Some(
        self.hold_tmms
        )
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn set_hold_tmms(&mut self, value: u16) {
        self.hold_tmms = value;
    }

    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    fn ar_gra_sf(&self) -> u16 {
        self.ar_gra_sf
    }

    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    fn v_ref_pct_sf(&self) -> Option<u16> {
        Some(
        self.v_ref_pct_sf
        )
    }
}