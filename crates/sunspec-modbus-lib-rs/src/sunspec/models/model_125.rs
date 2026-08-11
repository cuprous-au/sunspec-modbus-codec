use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 10;

static POINTS: [PointDetails<()>; 10] = [
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
        point: |()| Point::ModEna,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::SigType,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::Sig,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::WinTms,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::RvtTms,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::RmpTms,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::SigSf,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::Pad,
        size: 1,
        start_address: 9,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    ModEna,
    SigType,
    Sig,
    WinTms,
    RvtTms,
    RmpTms,
    SigSf,
    Pad,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    10
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
            buffer::write_u16(125, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::ModEna => {
            buffer::write_u16(model.mod_ena(), buffer);
        }
        Point::SigType => {
            if let Some(value) = model.sig_type() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Sig => {
            buffer::write_i16(model.sig(), buffer);
        }
        Point::WinTms => {
            if let Some(value) = model.win_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RvtTms => {
            if let Some(value) = model.rvt_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RmpTms => {
            if let Some(value) = model.rmp_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SigSf => {
            buffer::write_u16(model.sig_sf(), buffer);
        }
        Point::Pad => {
            buffer::write_u16(0, buffer);
        }
    }
}

pub trait ModelAdapter {
    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn set_mod_ena(&mut self, value: u16);

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn sig_type(&self) -> Option<SigType> {
        None
    }

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn set_sig_type(&mut self, value: SigType) {}

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn sig(&self) -> i16;

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn set_sig(&mut self, value: i16);

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn rvt_tms(&self) -> Option<u16> {
        None
    }

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn set_rvt_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn set_rmp_tms(&mut self, value: u16) {}

    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    fn sig_sf(&self) -> u16;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum SigType {
    Unknown = 0,
    Absolute = 1,
    Relative = 2,
    Multiplier = 3,
    Level = 4,
}

#[repr(C)]
pub struct Model125CallbackAdapter {
    context: *mut c_void,
    mod_ena_callback: extern "C" fn(*const c_void) -> u16,
    set_mod_ena_callback: extern "C" fn(u16, *mut c_void),
    sig_type_callback: Option<extern "C" fn(*const c_void) -> SigType>,
    set_sig_type_callback: Option<extern "C" fn(SigType, *mut c_void)>,
    sig_callback: extern "C" fn(*const c_void) -> i16,
    set_sig_callback: extern "C" fn(i16, *mut c_void),
    win_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_win_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    rvt_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_rvt_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    rmp_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_rmp_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    sig_sf_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model125CallbackAdapter {
    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn sig_type(&self) -> Option<SigType> {
        self.sig_type_callback
            .map(|callback| (callback)(self.context))
    }

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn set_sig_type(&mut self, value: SigType) {
        if let Some(callback) = self.set_sig_type_callback {
            (callback)(value, self.context);
        };
    }

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn sig(&self) -> i16 {
        (self.sig_callback)(self.context)
    }

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn set_sig(&mut self, value: i16) {
        (self.set_sig_callback)(value, self.context);
    }

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn rvt_tms(&self) -> Option<u16> {
        self.rvt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn set_rvt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rvt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn rmp_tms(&self) -> Option<u16> {
        self.rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn set_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rmp_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    fn sig_sf(&self) -> u16 {
        (self.sig_sf_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model125StatefulAdapter {
    mod_ena: u16,
    sig_type: SigType,
    sig: i16,
    win_tms: u16,
    rvt_tms: u16,
    rmp_tms: u16,
    sig_sf: u16,
}

impl ModelAdapter for Model125StatefulAdapter {
    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn sig_type(&self) -> Option<SigType> {
        Some(self.sig_type)
    }

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn set_sig_type(&mut self, value: SigType) {
        self.sig_type = value;
    }

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn sig(&self) -> i16 {
        self.sig
    }

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn set_sig(&mut self, value: i16) {
        self.sig = value;
    }

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn win_tms(&self) -> Option<u16> {
        Some(self.win_tms)
    }

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn set_win_tms(&mut self, value: u16) {
        self.win_tms = value;
    }

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn rvt_tms(&self) -> Option<u16> {
        Some(self.rvt_tms)
    }

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn set_rvt_tms(&mut self, value: u16) {
        self.rvt_tms = value;
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn rmp_tms(&self) -> Option<u16> {
        Some(self.rmp_tms)
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn set_rmp_tms(&mut self, value: u16) {
        self.rmp_tms = value;
    }

    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    fn sig_sf(&self) -> u16 {
        self.sig_sf
    }
}
