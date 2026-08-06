use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 28;

pub static POINTS: [ReadablePoint; 28] = [
    ReadablePoint {
        reference: PointReference::Static { value: 120 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::DerTyp,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::WRtg },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::WRtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::VaRtg,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::VaRtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::VArRtgQ1,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::VArRtgQ2,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::VArRtgQ3,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::VArRtgQ4,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::VArRtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 { point: Point::ARtg },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::ARtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::PfRtgQ1,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::PfRtgQ2,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::PfRtgQ3,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::PfRtgQ4,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::PfRtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::WhRtg,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::WhRtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::AhrRtg,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::AhrRtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::MaxChaRte,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::MaxChaRteSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::MaxDisChaRte,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model120 {
            point: Point::MaxDisChaRteSf,
        },
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
    ModelLength,
    DerTyp,
    WRtg,
    WRtgSf,
    VaRtg,
    VaRtgSf,
    VArRtgQ1,
    VArRtgQ2,
    VArRtgQ3,
    VArRtgQ4,
    VArRtgSf,
    ARtg,
    ARtgSf,
    PfRtgQ1,
    PfRtgQ2,
    PfRtgQ3,
    PfRtgQ4,
    PfRtgSf,
    WhRtg,
    WhRtgSf,
    AhrRtg,
    AhrRtgSf,
    MaxChaRte,
    MaxChaRteSf,
    MaxDisChaRte,
    MaxDisChaRteSf,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    28
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::DerTyp => {
            buffer::write_u16(model.der_typ() as u16, buffer);
        }
        Point::WRtg => {
            buffer::write_u16(model.w_rtg(), buffer);
        }
        Point::WRtgSf => {
            buffer::write_u16(model.w_rtg_sf(), buffer);
        }
        Point::VaRtg => {
            buffer::write_u16(model.va_rtg(), buffer);
        }
        Point::VaRtgSf => {
            buffer::write_u16(model.va_rtg_sf(), buffer);
        }
        Point::VArRtgQ1 => {
            buffer::write_i16(model.v_ar_rtg_q1(), buffer);
        }
        Point::VArRtgQ2 => {
            buffer::write_i16(model.v_ar_rtg_q2(), buffer);
        }
        Point::VArRtgQ3 => {
            buffer::write_i16(model.v_ar_rtg_q3(), buffer);
        }
        Point::VArRtgQ4 => {
            buffer::write_i16(model.v_ar_rtg_q4(), buffer);
        }
        Point::VArRtgSf => {
            buffer::write_u16(model.v_ar_rtg_sf(), buffer);
        }
        Point::ARtg => {
            buffer::write_u16(model.a_rtg(), buffer);
        }
        Point::ARtgSf => {
            buffer::write_u16(model.a_rtg_sf(), buffer);
        }
        Point::PfRtgQ1 => {
            buffer::write_i16(model.pf_rtg_q1(), buffer);
        }
        Point::PfRtgQ2 => {
            buffer::write_i16(model.pf_rtg_q2(), buffer);
        }
        Point::PfRtgQ3 => {
            buffer::write_i16(model.pf_rtg_q3(), buffer);
        }
        Point::PfRtgQ4 => {
            buffer::write_i16(model.pf_rtg_q4(), buffer);
        }
        Point::PfRtgSf => {
            buffer::write_u16(model.pf_rtg_sf(), buffer);
        }
        Point::WhRtg => {
            if let Some(value) = model.wh_rtg() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WhRtgSf => {
            if let Some(value) = model.wh_rtg_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AhrRtg => {
            if let Some(value) = model.ahr_rtg() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AhrRtgSf => {
            if let Some(value) = model.ahr_rtg_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxChaRte => {
            if let Some(value) = model.max_cha_rte() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxChaRteSf => {
            if let Some(value) = model.max_cha_rte_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxDisChaRte => {
            if let Some(value) = model.max_dis_cha_rte() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxDisChaRteSf => {
            if let Some(value) = model.max_dis_cha_rte_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    fn der_typ(&self) -> DerTyp;

    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    fn w_rtg(&self) -> u16;

    /// WRtg_SF
    ///
    /// Scale factor
    fn w_rtg_sf(&self) -> u16;

    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    fn va_rtg(&self) -> u16;

    /// VARtg_SF
    ///
    /// Scale factor
    fn va_rtg_sf(&self) -> u16;

    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    fn v_ar_rtg_q1(&self) -> i16;

    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    fn v_ar_rtg_q2(&self) -> i16;

    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    fn v_ar_rtg_q3(&self) -> i16;

    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    fn v_ar_rtg_q4(&self) -> i16;

    /// VArRtg_SF
    ///
    /// Scale factor
    fn v_ar_rtg_sf(&self) -> u16;

    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    fn a_rtg(&self) -> u16;

    /// ARtg_SF
    ///
    /// Scale factor
    fn a_rtg_sf(&self) -> u16;

    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// EEI sign convention.
    fn pf_rtg_q1(&self) -> i16;

    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// EEI sign convention.
    fn pf_rtg_q2(&self) -> i16;

    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// EEI sign convention.
    fn pf_rtg_q3(&self) -> i16;

    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// EEI sign convention.
    fn pf_rtg_q4(&self) -> i16;

    /// PFRtg_SF
    ///
    /// Scale factor
    fn pf_rtg_sf(&self) -> u16;

    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    fn wh_rtg(&self) -> Option<u16> {
        None
    }

    /// WHRtg_SF
    ///
    /// Scale factor
    fn wh_rtg_sf(&self) -> Option<u16> {
        None
    }

    /// AhrRtg
    ///
    /// The usable capacity of the battery. Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    fn ahr_rtg(&self) -> Option<u16> {
        None
    }

    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    fn ahr_rtg_sf(&self) -> Option<u16> {
        None
    }

    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    fn max_cha_rte(&self) -> Option<u16> {
        None
    }

    /// MaxChaRte_SF
    ///
    /// Scale factor
    fn max_cha_rte_sf(&self) -> Option<u16> {
        None
    }

    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    fn max_dis_cha_rte(&self) -> Option<u16> {
        None
    }

    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    fn max_dis_cha_rte_sf(&self) -> Option<u16> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum DerTyp {
    Pv = 4,
    PvStor = 82,
}

#[repr(C)]
pub struct Model120CallbackAdapter {
    context: *mut c_void,
    der_typ_callback: extern "C" fn(*const c_void) -> DerTyp,
    w_rtg_callback: extern "C" fn(*const c_void) -> u16,
    w_rtg_sf_callback: extern "C" fn(*const c_void) -> u16,
    va_rtg_callback: extern "C" fn(*const c_void) -> u16,
    va_rtg_sf_callback: extern "C" fn(*const c_void) -> u16,
    v_ar_rtg_q1_callback: extern "C" fn(*const c_void) -> i16,
    v_ar_rtg_q2_callback: extern "C" fn(*const c_void) -> i16,
    v_ar_rtg_q3_callback: extern "C" fn(*const c_void) -> i16,
    v_ar_rtg_q4_callback: extern "C" fn(*const c_void) -> i16,
    v_ar_rtg_sf_callback: extern "C" fn(*const c_void) -> u16,
    a_rtg_callback: extern "C" fn(*const c_void) -> u16,
    a_rtg_sf_callback: extern "C" fn(*const c_void) -> u16,
    pf_rtg_q1_callback: extern "C" fn(*const c_void) -> i16,
    pf_rtg_q2_callback: extern "C" fn(*const c_void) -> i16,
    pf_rtg_q3_callback: extern "C" fn(*const c_void) -> i16,
    pf_rtg_q4_callback: extern "C" fn(*const c_void) -> i16,
    pf_rtg_sf_callback: extern "C" fn(*const c_void) -> u16,
    wh_rtg_callback: Option<extern "C" fn(*const c_void) -> u16>,
    wh_rtg_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    ahr_rtg_callback: Option<extern "C" fn(*const c_void) -> u16>,
    ahr_rtg_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_cha_rte_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_cha_rte_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_dis_cha_rte_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_dis_cha_rte_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model120CallbackAdapter {
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    fn der_typ(&self) -> DerTyp {
        (self.der_typ_callback)(self.context)
    }

    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    fn w_rtg(&self) -> u16 {
        (self.w_rtg_callback)(self.context)
    }

    /// WRtg_SF
    ///
    /// Scale factor
    fn w_rtg_sf(&self) -> u16 {
        (self.w_rtg_sf_callback)(self.context)
    }

    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    fn va_rtg(&self) -> u16 {
        (self.va_rtg_callback)(self.context)
    }

    /// VARtg_SF
    ///
    /// Scale factor
    fn va_rtg_sf(&self) -> u16 {
        (self.va_rtg_sf_callback)(self.context)
    }

    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    fn v_ar_rtg_q1(&self) -> i16 {
        (self.v_ar_rtg_q1_callback)(self.context)
    }

    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    fn v_ar_rtg_q2(&self) -> i16 {
        (self.v_ar_rtg_q2_callback)(self.context)
    }

    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    fn v_ar_rtg_q3(&self) -> i16 {
        (self.v_ar_rtg_q3_callback)(self.context)
    }

    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    fn v_ar_rtg_q4(&self) -> i16 {
        (self.v_ar_rtg_q4_callback)(self.context)
    }

    /// VArRtg_SF
    ///
    /// Scale factor
    fn v_ar_rtg_sf(&self) -> u16 {
        (self.v_ar_rtg_sf_callback)(self.context)
    }

    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    fn a_rtg(&self) -> u16 {
        (self.a_rtg_callback)(self.context)
    }

    /// ARtg_SF
    ///
    /// Scale factor
    fn a_rtg_sf(&self) -> u16 {
        (self.a_rtg_sf_callback)(self.context)
    }

    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// EEI sign convention.
    fn pf_rtg_q1(&self) -> i16 {
        (self.pf_rtg_q1_callback)(self.context)
    }

    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// EEI sign convention.
    fn pf_rtg_q2(&self) -> i16 {
        (self.pf_rtg_q2_callback)(self.context)
    }

    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// EEI sign convention.
    fn pf_rtg_q3(&self) -> i16 {
        (self.pf_rtg_q3_callback)(self.context)
    }

    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// EEI sign convention.
    fn pf_rtg_q4(&self) -> i16 {
        (self.pf_rtg_q4_callback)(self.context)
    }

    /// PFRtg_SF
    ///
    /// Scale factor
    fn pf_rtg_sf(&self) -> u16 {
        (self.pf_rtg_sf_callback)(self.context)
    }

    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    fn wh_rtg(&self) -> Option<u16> {
        self.wh_rtg_callback
            .map(|callback| (callback)(self.context))
    }

    /// WHRtg_SF
    ///
    /// Scale factor
    fn wh_rtg_sf(&self) -> Option<u16> {
        self.wh_rtg_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// AhrRtg
    ///
    /// The usable capacity of the battery. Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    fn ahr_rtg(&self) -> Option<u16> {
        self.ahr_rtg_callback
            .map(|callback| (callback)(self.context))
    }

    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    fn ahr_rtg_sf(&self) -> Option<u16> {
        self.ahr_rtg_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    fn max_cha_rte(&self) -> Option<u16> {
        self.max_cha_rte_callback
            .map(|callback| (callback)(self.context))
    }

    /// MaxChaRte_SF
    ///
    /// Scale factor
    fn max_cha_rte_sf(&self) -> Option<u16> {
        self.max_cha_rte_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    fn max_dis_cha_rte(&self) -> Option<u16> {
        self.max_dis_cha_rte_callback
            .map(|callback| (callback)(self.context))
    }

    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    fn max_dis_cha_rte_sf(&self) -> Option<u16> {
        self.max_dis_cha_rte_sf_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model120StatefulAdapter {
    der_typ: DerTyp,
    w_rtg: u16,
    w_rtg_sf: u16,
    va_rtg: u16,
    va_rtg_sf: u16,
    v_ar_rtg_q1: i16,
    v_ar_rtg_q2: i16,
    v_ar_rtg_q3: i16,
    v_ar_rtg_q4: i16,
    v_ar_rtg_sf: u16,
    a_rtg: u16,
    a_rtg_sf: u16,
    pf_rtg_q1: i16,
    pf_rtg_q2: i16,
    pf_rtg_q3: i16,
    pf_rtg_q4: i16,
    pf_rtg_sf: u16,
    wh_rtg: u16,
    wh_rtg_sf: u16,
    ahr_rtg: u16,
    ahr_rtg_sf: u16,
    max_cha_rte: u16,
    max_cha_rte_sf: u16,
    max_dis_cha_rte: u16,
    max_dis_cha_rte_sf: u16,
}

impl ModelAdapter for Model120StatefulAdapter {
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    fn der_typ(&self) -> DerTyp {
        self.der_typ
    }

    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    fn w_rtg(&self) -> u16 {
        self.w_rtg
    }

    /// WRtg_SF
    ///
    /// Scale factor
    fn w_rtg_sf(&self) -> u16 {
        self.w_rtg_sf
    }

    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    fn va_rtg(&self) -> u16 {
        self.va_rtg
    }

    /// VARtg_SF
    ///
    /// Scale factor
    fn va_rtg_sf(&self) -> u16 {
        self.va_rtg_sf
    }

    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    fn v_ar_rtg_q1(&self) -> i16 {
        self.v_ar_rtg_q1
    }

    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    fn v_ar_rtg_q2(&self) -> i16 {
        self.v_ar_rtg_q2
    }

    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    fn v_ar_rtg_q3(&self) -> i16 {
        self.v_ar_rtg_q3
    }

    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    fn v_ar_rtg_q4(&self) -> i16 {
        self.v_ar_rtg_q4
    }

    /// VArRtg_SF
    ///
    /// Scale factor
    fn v_ar_rtg_sf(&self) -> u16 {
        self.v_ar_rtg_sf
    }

    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    fn a_rtg(&self) -> u16 {
        self.a_rtg
    }

    /// ARtg_SF
    ///
    /// Scale factor
    fn a_rtg_sf(&self) -> u16 {
        self.a_rtg_sf
    }

    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// EEI sign convention.
    fn pf_rtg_q1(&self) -> i16 {
        self.pf_rtg_q1
    }

    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// EEI sign convention.
    fn pf_rtg_q2(&self) -> i16 {
        self.pf_rtg_q2
    }

    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// EEI sign convention.
    fn pf_rtg_q3(&self) -> i16 {
        self.pf_rtg_q3
    }

    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// EEI sign convention.
    fn pf_rtg_q4(&self) -> i16 {
        self.pf_rtg_q4
    }

    /// PFRtg_SF
    ///
    /// Scale factor
    fn pf_rtg_sf(&self) -> u16 {
        self.pf_rtg_sf
    }

    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    fn wh_rtg(&self) -> Option<u16> {
        Some(self.wh_rtg)
    }

    /// WHRtg_SF
    ///
    /// Scale factor
    fn wh_rtg_sf(&self) -> Option<u16> {
        Some(self.wh_rtg_sf)
    }

    /// AhrRtg
    ///
    /// The usable capacity of the battery. Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    fn ahr_rtg(&self) -> Option<u16> {
        Some(self.ahr_rtg)
    }

    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    fn ahr_rtg_sf(&self) -> Option<u16> {
        Some(self.ahr_rtg_sf)
    }

    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    fn max_cha_rte(&self) -> Option<u16> {
        Some(self.max_cha_rte)
    }

    /// MaxChaRte_SF
    ///
    /// Scale factor
    fn max_cha_rte_sf(&self) -> Option<u16> {
        Some(self.max_cha_rte_sf)
    }

    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    fn max_dis_cha_rte(&self) -> Option<u16> {
        Some(self.max_dis_cha_rte)
    }

    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    fn max_dis_cha_rte_sf(&self) -> Option<u16> {
        Some(self.max_dis_cha_rte_sf)
    }
}
