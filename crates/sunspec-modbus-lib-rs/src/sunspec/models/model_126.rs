use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 66;

pub static POINTS: [ReadablePoint; 59] = [
    ReadablePoint {
        reference: PointReference::Static { value: 126 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::ActCrv,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::ModEna,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::WinTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::RvrtTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::RmpTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::NCrv },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::NPt },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::DeptRefSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::RmpIncDecSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveActPt,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveDeptRef,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV1,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr1,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV2,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr2,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV3,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr3,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV4,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr4,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV5,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr5,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV6,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr6,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV7,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr7,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV8,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr8,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV9,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr9,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV10,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr10,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV11,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr11,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV12,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr12,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV13,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr13,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV14,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr14,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV15,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr15,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV16,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr16,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV17,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr17,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV18,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr18,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV19,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr19,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveV20,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveVAr20,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveCrvNam,
        },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveRmpTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveRmpDecTmm,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveRmpIncTmm,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model126 {
            point: Point::CurveReadOnly,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    ActCrv,
    ModEna,
    WinTms,
    RvrtTms,
    RmpTms,
    NCrv,
    NPt,
    VSf,
    DeptRefSf,
    RmpIncDecSf,
    CurveActPt,
    CurveDeptRef,
    CurveV1,
    CurveVAr1,
    CurveV2,
    CurveVAr2,
    CurveV3,
    CurveVAr3,
    CurveV4,
    CurveVAr4,
    CurveV5,
    CurveVAr5,
    CurveV6,
    CurveVAr6,
    CurveV7,
    CurveVAr7,
    CurveV8,
    CurveVAr8,
    CurveV9,
    CurveVAr9,
    CurveV10,
    CurveVAr10,
    CurveV11,
    CurveVAr11,
    CurveV12,
    CurveVAr12,
    CurveV13,
    CurveVAr13,
    CurveV14,
    CurveVAr14,
    CurveV15,
    CurveVAr15,
    CurveV16,
    CurveVAr16,
    CurveV17,
    CurveVAr17,
    CurveV18,
    CurveVAr18,
    CurveV19,
    CurveVAr19,
    CurveV20,
    CurveVAr20,
    CurveCrvNam,
    CurveRmpTms,
    CurveRmpDecTmm,
    CurveRmpIncTmm,
    CurveReadOnly,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    66
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
        Point::ActCrv => {
            buffer::write_u16(model.act_crv(), buffer);
        }
        Point::ModEna => {
            buffer::write_u16(model.mod_ena(), buffer);
        }
        Point::WinTms => {
            if let Some(value) = model.win_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RvrtTms => {
            if let Some(value) = model.rvrt_tms() {
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
        Point::NCrv => {
            buffer::write_u16(model.n_crv(), buffer);
        }
        Point::NPt => {
            buffer::write_u16(model.n_pt(), buffer);
        }
        Point::VSf => {
            buffer::write_u16(model.v_sf(), buffer);
        }
        Point::DeptRefSf => {
            buffer::write_u16(model.dept_ref_sf(), buffer);
        }
        Point::RmpIncDecSf => {
            if let Some(value) = model.rmp_inc_dec_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveActPt => {
            buffer::write_u16(model.curve_act_pt(), buffer);
        }
        Point::CurveDeptRef => {
            buffer::write_u16(model.curve_dept_ref() as u16, buffer);
        }
        Point::CurveV1 => {
            buffer::write_u16(model.curve_v1(), buffer);
        }
        Point::CurveVAr1 => {
            buffer::write_i16(model.curve_v_ar1(), buffer);
        }
        Point::CurveV2 => {
            if let Some(value) = model.curve_v2() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr2 => {
            if let Some(value) = model.curve_v_ar2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV3 => {
            if let Some(value) = model.curve_v3() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr3 => {
            if let Some(value) = model.curve_v_ar3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV4 => {
            if let Some(value) = model.curve_v4() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr4 => {
            if let Some(value) = model.curve_v_ar4() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV5 => {
            if let Some(value) = model.curve_v5() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr5 => {
            if let Some(value) = model.curve_v_ar5() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV6 => {
            if let Some(value) = model.curve_v6() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr6 => {
            if let Some(value) = model.curve_v_ar6() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV7 => {
            if let Some(value) = model.curve_v7() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr7 => {
            if let Some(value) = model.curve_v_ar7() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV8 => {
            if let Some(value) = model.curve_v8() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr8 => {
            if let Some(value) = model.curve_v_ar8() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV9 => {
            if let Some(value) = model.curve_v9() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr9 => {
            if let Some(value) = model.curve_v_ar9() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV10 => {
            if let Some(value) = model.curve_v10() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr10 => {
            if let Some(value) = model.curve_v_ar10() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV11 => {
            if let Some(value) = model.curve_v11() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr11 => {
            if let Some(value) = model.curve_v_ar11() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV12 => {
            if let Some(value) = model.curve_v12() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr12 => {
            if let Some(value) = model.curve_v_ar12() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV13 => {
            if let Some(value) = model.curve_v13() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr13 => {
            if let Some(value) = model.curve_v_ar13() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV14 => {
            if let Some(value) = model.curve_v14() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr14 => {
            if let Some(value) = model.curve_v_ar14() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV15 => {
            if let Some(value) = model.curve_v15() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr15 => {
            if let Some(value) = model.curve_v_ar15() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV16 => {
            if let Some(value) = model.curve_v16() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr16 => {
            if let Some(value) = model.curve_v_ar16() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV17 => {
            if let Some(value) = model.curve_v17() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr17 => {
            if let Some(value) = model.curve_v_ar17() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV18 => {
            if let Some(value) = model.curve_v18() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr18 => {
            if let Some(value) = model.curve_v_ar18() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV19 => {
            if let Some(value) = model.curve_v19() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr19 => {
            if let Some(value) = model.curve_v_ar19() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV20 => {
            if let Some(value) = model.curve_v20() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveVAr20 => {
            if let Some(value) = model.curve_v_ar20() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveCrvNam => {
            if let Some(value) = model.curve_crv_nam() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveRmpTms => {
            if let Some(value) = model.curve_rmp_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveRmpDecTmm => {
            if let Some(value) = model.curve_rmp_dec_tmm() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveRmpIncTmm => {
            if let Some(value) = model.curve_rmp_inc_tmm() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveReadOnly => {
            buffer::write_u16(model.curve_read_only() as u16, buffer);
        }
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
    /// Is Volt-VAR control active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is Volt-VAR control active.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for volt-VAR change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for volt-VAR change.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    fn set_rvrt_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_rmp_tms(&mut self, value: u16) {}

    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16;

    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16;

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16;

    /// DeptRef_SF
    ///
    /// Scale factor for dependent variable.
    fn dept_ref_sf(&self) -> u16;

    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        None
    }

    /// ActPt
    ///
    /// Number of active points in array.
    fn curve_act_pt(&self) -> u16;

    /// ActPt
    ///
    /// Number of active points in array.
    fn set_curve_act_pt(&mut self, value: u16);

    /// DeptRef
    ///
    /// Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.
    fn curve_dept_ref(&self) -> DeptRef;

    /// DeptRef
    ///
    /// Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.
    fn set_curve_dept_ref(&mut self, value: DeptRef);

    /// V1
    ///
    /// Point 1 Volts.
    fn curve_v1(&self) -> u16;

    /// V1
    ///
    /// Point 1 Volts.
    fn set_curve_v1(&mut self, value: u16);

    /// VAr1
    ///
    /// Point 1 VARs.
    fn curve_v_ar1(&self) -> i16;

    /// VAr1
    ///
    /// Point 1 VARs.
    fn set_curve_v_ar1(&mut self, value: i16);

    /// V2
    ///
    /// Point 2 Volts.
    fn curve_v2(&self) -> Option<u16> {
        None
    }

    /// V2
    ///
    /// Point 2 Volts.
    fn set_curve_v2(&mut self, value: u16) {}

    /// VAr2
    ///
    /// Point 2 VARs.
    fn curve_v_ar2(&self) -> Option<i16> {
        None
    }

    /// VAr2
    ///
    /// Point 2 VARs.
    fn set_curve_v_ar2(&mut self, value: i16) {}

    /// V3
    ///
    /// Point 3 Volts.
    fn curve_v3(&self) -> Option<u16> {
        None
    }

    /// V3
    ///
    /// Point 3 Volts.
    fn set_curve_v3(&mut self, value: u16) {}

    /// VAr3
    ///
    /// Point 3 VARs.
    fn curve_v_ar3(&self) -> Option<i16> {
        None
    }

    /// VAr3
    ///
    /// Point 3 VARs.
    fn set_curve_v_ar3(&mut self, value: i16) {}

    /// V4
    ///
    /// Point 4 Volts.
    fn curve_v4(&self) -> Option<u16> {
        None
    }

    /// V4
    ///
    /// Point 4 Volts.
    fn set_curve_v4(&mut self, value: u16) {}

    /// VAr4
    ///
    /// Point 4 VARs.
    fn curve_v_ar4(&self) -> Option<i16> {
        None
    }

    /// VAr4
    ///
    /// Point 4 VARs.
    fn set_curve_v_ar4(&mut self, value: i16) {}

    /// V5
    ///
    /// Point 5 Volts.
    fn curve_v5(&self) -> Option<u16> {
        None
    }

    /// V5
    ///
    /// Point 5 Volts.
    fn set_curve_v5(&mut self, value: u16) {}

    /// VAr5
    ///
    /// Point 5 VARs.
    fn curve_v_ar5(&self) -> Option<i16> {
        None
    }

    /// VAr5
    ///
    /// Point 5 VARs.
    fn set_curve_v_ar5(&mut self, value: i16) {}

    /// V6
    ///
    /// Point 6 Volts.
    fn curve_v6(&self) -> Option<u16> {
        None
    }

    /// V6
    ///
    /// Point 6 Volts.
    fn set_curve_v6(&mut self, value: u16) {}

    /// VAr6
    ///
    /// Point 6 VARs.
    fn curve_v_ar6(&self) -> Option<i16> {
        None
    }

    /// VAr6
    ///
    /// Point 6 VARs.
    fn set_curve_v_ar6(&mut self, value: i16) {}

    /// V7
    ///
    /// Point 7 Volts.
    fn curve_v7(&self) -> Option<u16> {
        None
    }

    /// V7
    ///
    /// Point 7 Volts.
    fn set_curve_v7(&mut self, value: u16) {}

    /// VAr7
    ///
    /// Point 7 VARs.
    fn curve_v_ar7(&self) -> Option<i16> {
        None
    }

    /// VAr7
    ///
    /// Point 7 VARs.
    fn set_curve_v_ar7(&mut self, value: i16) {}

    /// V8
    ///
    /// Point 8 Volts.
    fn curve_v8(&self) -> Option<u16> {
        None
    }

    /// V8
    ///
    /// Point 8 Volts.
    fn set_curve_v8(&mut self, value: u16) {}

    /// VAr8
    ///
    /// Point 8 VARs.
    fn curve_v_ar8(&self) -> Option<i16> {
        None
    }

    /// VAr8
    ///
    /// Point 8 VARs.
    fn set_curve_v_ar8(&mut self, value: i16) {}

    /// V9
    ///
    /// Point 9 Volts.
    fn curve_v9(&self) -> Option<u16> {
        None
    }

    /// V9
    ///
    /// Point 9 Volts.
    fn set_curve_v9(&mut self, value: u16) {}

    /// VAr9
    ///
    /// Point 9 VARs.
    fn curve_v_ar9(&self) -> Option<i16> {
        None
    }

    /// VAr9
    ///
    /// Point 9 VARs.
    fn set_curve_v_ar9(&mut self, value: i16) {}

    /// V10
    ///
    /// Point 10 Volts.
    fn curve_v10(&self) -> Option<u16> {
        None
    }

    /// V10
    ///
    /// Point 10 Volts.
    fn set_curve_v10(&mut self, value: u16) {}

    /// VAr10
    ///
    /// Point 10 VARs.
    fn curve_v_ar10(&self) -> Option<i16> {
        None
    }

    /// VAr10
    ///
    /// Point 10 VARs.
    fn set_curve_v_ar10(&mut self, value: i16) {}

    /// V11
    ///
    /// Point 11 Volts.
    fn curve_v11(&self) -> Option<u16> {
        None
    }

    /// V11
    ///
    /// Point 11 Volts.
    fn set_curve_v11(&mut self, value: u16) {}

    /// VAr11
    ///
    /// Point 11 VARs.
    fn curve_v_ar11(&self) -> Option<i16> {
        None
    }

    /// VAr11
    ///
    /// Point 11 VARs.
    fn set_curve_v_ar11(&mut self, value: i16) {}

    /// V12
    ///
    /// Point 12 Volts.
    fn curve_v12(&self) -> Option<u16> {
        None
    }

    /// V12
    ///
    /// Point 12 Volts.
    fn set_curve_v12(&mut self, value: u16) {}

    /// VAr12
    ///
    /// Point 12 VARs.
    fn curve_v_ar12(&self) -> Option<i16> {
        None
    }

    /// VAr12
    ///
    /// Point 12 VARs.
    fn set_curve_v_ar12(&mut self, value: i16) {}

    /// V13
    ///
    /// Point 13 Volts.
    fn curve_v13(&self) -> Option<u16> {
        None
    }

    /// V13
    ///
    /// Point 13 Volts.
    fn set_curve_v13(&mut self, value: u16) {}

    /// VAr13
    ///
    /// Point 13 VARs.
    fn curve_v_ar13(&self) -> Option<i16> {
        None
    }

    /// VAr13
    ///
    /// Point 13 VARs.
    fn set_curve_v_ar13(&mut self, value: i16) {}

    /// V14
    ///
    /// Point 14 Volts.
    fn curve_v14(&self) -> Option<u16> {
        None
    }

    /// V14
    ///
    /// Point 14 Volts.
    fn set_curve_v14(&mut self, value: u16) {}

    /// VAr14
    ///
    /// Point 14 VARs.
    fn curve_v_ar14(&self) -> Option<i16> {
        None
    }

    /// VAr14
    ///
    /// Point 14 VARs.
    fn set_curve_v_ar14(&mut self, value: i16) {}

    /// V15
    ///
    /// Point 15 Volts.
    fn curve_v15(&self) -> Option<u16> {
        None
    }

    /// V15
    ///
    /// Point 15 Volts.
    fn set_curve_v15(&mut self, value: u16) {}

    /// VAr15
    ///
    /// Point 15 VARs.
    fn curve_v_ar15(&self) -> Option<i16> {
        None
    }

    /// VAr15
    ///
    /// Point 15 VARs.
    fn set_curve_v_ar15(&mut self, value: i16) {}

    /// V16
    ///
    /// Point 16 Volts.
    fn curve_v16(&self) -> Option<u16> {
        None
    }

    /// V16
    ///
    /// Point 16 Volts.
    fn set_curve_v16(&mut self, value: u16) {}

    /// VAr16
    ///
    /// Point 16 VARs.
    fn curve_v_ar16(&self) -> Option<i16> {
        None
    }

    /// VAr16
    ///
    /// Point 16 VARs.
    fn set_curve_v_ar16(&mut self, value: i16) {}

    /// V17
    ///
    /// Point 17 Volts.
    fn curve_v17(&self) -> Option<u16> {
        None
    }

    /// V17
    ///
    /// Point 17 Volts.
    fn set_curve_v17(&mut self, value: u16) {}

    /// VAr17
    ///
    /// Point 17 VARs.
    fn curve_v_ar17(&self) -> Option<i16> {
        None
    }

    /// VAr17
    ///
    /// Point 17 VARs.
    fn set_curve_v_ar17(&mut self, value: i16) {}

    /// V18
    ///
    /// Point 18 Volts.
    fn curve_v18(&self) -> Option<u16> {
        None
    }

    /// V18
    ///
    /// Point 18 Volts.
    fn set_curve_v18(&mut self, value: u16) {}

    /// VAr18
    ///
    /// Point 18 VARs.
    fn curve_v_ar18(&self) -> Option<i16> {
        None
    }

    /// VAr18
    ///
    /// Point 18 VARs.
    fn set_curve_v_ar18(&mut self, value: i16) {}

    /// V19
    ///
    /// Point 19 Volts.
    fn curve_v19(&self) -> Option<u16> {
        None
    }

    /// V19
    ///
    /// Point 19 Volts.
    fn set_curve_v19(&mut self, value: u16) {}

    /// VAr19
    ///
    /// Point 19 VARs.
    fn curve_v_ar19(&self) -> Option<i16> {
        None
    }

    /// VAr19
    ///
    /// Point 19 VARs.
    fn set_curve_v_ar19(&mut self, value: i16) {}

    /// V20
    ///
    /// Point 20 Volts.
    fn curve_v20(&self) -> Option<u16> {
        None
    }

    /// V20
    ///
    /// Point 20 Volts.
    fn set_curve_v20(&mut self, value: u16) {}

    /// VAr20
    ///
    /// Point 20 VARs.
    fn curve_v_ar20(&self) -> Option<i16> {
        None
    }

    /// VAr20
    ///
    /// Point 20 VARs.
    fn set_curve_v_ar20(&mut self, value: i16) {}

    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
    fn curve_crv_nam(&self) -> Option<&CStr> {
        None
    }

    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
    fn set_curve_crv_nam(&mut self, value: &CStr) {}

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_tms(&mut self, value: u16) {}

    /// RmpDecTmm
    ///
    /// The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        None
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {}

    /// RmpIncTmm
    ///
    /// The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        None
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {}

    /// ReadOnly
    ///
    /// Boolean flag indicates if curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum DeptRef {
    WMax = 1,
    VArMax = 2,
    VArAval = 3,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ReadOnly {
    Readwrite = 0,
    Readonly = 1,
}

#[repr(C)]
pub struct Model126CallbackAdapter {
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
    v_sf_callback: extern "C" fn(*const c_void) -> u16,
    dept_ref_sf_callback: extern "C" fn(*const c_void) -> u16,
    rmp_inc_dec_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    curve_act_pt_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_act_pt_callback: extern "C" fn(u16, *mut c_void),
    curve_dept_ref_callback: extern "C" fn(*const c_void) -> DeptRef,
    set_curve_dept_ref_callback: extern "C" fn(DeptRef, *mut c_void),
    curve_v1_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_v1_callback: extern "C" fn(u16, *mut c_void),
    curve_v_ar1_callback: extern "C" fn(*const c_void) -> i16,
    set_curve_v_ar1_callback: extern "C" fn(i16, *mut c_void),
    curve_v2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v2_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar2_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v3_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar3_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v4_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar4_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar4_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v5_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar5_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar5_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v6_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v6_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar6_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar6_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v7_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v7_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar7_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar7_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v8_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v8_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar8_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar8_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v9_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v9_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar9_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar9_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v10_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v10_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar10_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar10_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v11_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v11_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar11_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar11_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v12_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v12_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar12_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar12_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v13_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v13_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar13_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar13_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v14_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v14_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar14_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar14_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v15_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v15_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar15_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar15_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v16_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar16_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar16_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v17_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v17_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar17_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar17_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v18_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v18_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar18_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar18_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v19_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v19_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar19_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar19_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v20_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v20_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v_ar20_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_v_ar20_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_crv_nam_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_curve_crv_nam_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    curve_rmp_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_rmp_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_rmp_dec_tmm_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_rmp_dec_tmm_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_rmp_inc_tmm_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_rmp_inc_tmm_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_read_only_callback: extern "C" fn(*const c_void) -> ReadOnly,
}

impl ModelAdapter for Model126CallbackAdapter {
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
    /// Is Volt-VAR control active.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Is Volt-VAR control active.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// WinTms
    ///
    /// Time window for volt-VAR change.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WinTms
    ///
    /// Time window for volt-VAR change.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        self.rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    fn set_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn rmp_tms(&self) -> Option<u16> {
        self.rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
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

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)(self.context)
    }

    /// DeptRef_SF
    ///
    /// Scale factor for dependent variable.
    fn dept_ref_sf(&self) -> u16 {
        (self.dept_ref_sf_callback)(self.context)
    }

    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        self.rmp_inc_dec_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// ActPt
    ///
    /// Number of active points in array.
    fn curve_act_pt(&self) -> u16 {
        (self.curve_act_pt_callback)(self.context)
    }

    /// ActPt
    ///
    /// Number of active points in array.
    fn set_curve_act_pt(&mut self, value: u16) {
        (self.set_curve_act_pt_callback)(value, self.context);
    }

    /// DeptRef
    ///
    /// Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.
    fn curve_dept_ref(&self) -> DeptRef {
        (self.curve_dept_ref_callback)(self.context)
    }

    /// DeptRef
    ///
    /// Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.
    fn set_curve_dept_ref(&mut self, value: DeptRef) {
        (self.set_curve_dept_ref_callback)(value, self.context);
    }

    /// V1
    ///
    /// Point 1 Volts.
    fn curve_v1(&self) -> u16 {
        (self.curve_v1_callback)(self.context)
    }

    /// V1
    ///
    /// Point 1 Volts.
    fn set_curve_v1(&mut self, value: u16) {
        (self.set_curve_v1_callback)(value, self.context);
    }

    /// VAr1
    ///
    /// Point 1 VARs.
    fn curve_v_ar1(&self) -> i16 {
        (self.curve_v_ar1_callback)(self.context)
    }

    /// VAr1
    ///
    /// Point 1 VARs.
    fn set_curve_v_ar1(&mut self, value: i16) {
        (self.set_curve_v_ar1_callback)(value, self.context);
    }

    /// V2
    ///
    /// Point 2 Volts.
    fn curve_v2(&self) -> Option<u16> {
        self.curve_v2_callback
            .map(|callback| (callback)(self.context))
    }

    /// V2
    ///
    /// Point 2 Volts.
    fn set_curve_v2(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v2_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr2
    ///
    /// Point 2 VARs.
    fn curve_v_ar2(&self) -> Option<i16> {
        self.curve_v_ar2_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr2
    ///
    /// Point 2 VARs.
    fn set_curve_v_ar2(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar2_callback {
            (callback)(value, self.context);
        };
    }

    /// V3
    ///
    /// Point 3 Volts.
    fn curve_v3(&self) -> Option<u16> {
        self.curve_v3_callback
            .map(|callback| (callback)(self.context))
    }

    /// V3
    ///
    /// Point 3 Volts.
    fn set_curve_v3(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v3_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr3
    ///
    /// Point 3 VARs.
    fn curve_v_ar3(&self) -> Option<i16> {
        self.curve_v_ar3_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr3
    ///
    /// Point 3 VARs.
    fn set_curve_v_ar3(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar3_callback {
            (callback)(value, self.context);
        };
    }

    /// V4
    ///
    /// Point 4 Volts.
    fn curve_v4(&self) -> Option<u16> {
        self.curve_v4_callback
            .map(|callback| (callback)(self.context))
    }

    /// V4
    ///
    /// Point 4 Volts.
    fn set_curve_v4(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v4_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr4
    ///
    /// Point 4 VARs.
    fn curve_v_ar4(&self) -> Option<i16> {
        self.curve_v_ar4_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr4
    ///
    /// Point 4 VARs.
    fn set_curve_v_ar4(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar4_callback {
            (callback)(value, self.context);
        };
    }

    /// V5
    ///
    /// Point 5 Volts.
    fn curve_v5(&self) -> Option<u16> {
        self.curve_v5_callback
            .map(|callback| (callback)(self.context))
    }

    /// V5
    ///
    /// Point 5 Volts.
    fn set_curve_v5(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v5_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr5
    ///
    /// Point 5 VARs.
    fn curve_v_ar5(&self) -> Option<i16> {
        self.curve_v_ar5_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr5
    ///
    /// Point 5 VARs.
    fn set_curve_v_ar5(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar5_callback {
            (callback)(value, self.context);
        };
    }

    /// V6
    ///
    /// Point 6 Volts.
    fn curve_v6(&self) -> Option<u16> {
        self.curve_v6_callback
            .map(|callback| (callback)(self.context))
    }

    /// V6
    ///
    /// Point 6 Volts.
    fn set_curve_v6(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v6_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr6
    ///
    /// Point 6 VARs.
    fn curve_v_ar6(&self) -> Option<i16> {
        self.curve_v_ar6_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr6
    ///
    /// Point 6 VARs.
    fn set_curve_v_ar6(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar6_callback {
            (callback)(value, self.context);
        };
    }

    /// V7
    ///
    /// Point 7 Volts.
    fn curve_v7(&self) -> Option<u16> {
        self.curve_v7_callback
            .map(|callback| (callback)(self.context))
    }

    /// V7
    ///
    /// Point 7 Volts.
    fn set_curve_v7(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v7_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr7
    ///
    /// Point 7 VARs.
    fn curve_v_ar7(&self) -> Option<i16> {
        self.curve_v_ar7_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr7
    ///
    /// Point 7 VARs.
    fn set_curve_v_ar7(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar7_callback {
            (callback)(value, self.context);
        };
    }

    /// V8
    ///
    /// Point 8 Volts.
    fn curve_v8(&self) -> Option<u16> {
        self.curve_v8_callback
            .map(|callback| (callback)(self.context))
    }

    /// V8
    ///
    /// Point 8 Volts.
    fn set_curve_v8(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v8_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr8
    ///
    /// Point 8 VARs.
    fn curve_v_ar8(&self) -> Option<i16> {
        self.curve_v_ar8_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr8
    ///
    /// Point 8 VARs.
    fn set_curve_v_ar8(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar8_callback {
            (callback)(value, self.context);
        };
    }

    /// V9
    ///
    /// Point 9 Volts.
    fn curve_v9(&self) -> Option<u16> {
        self.curve_v9_callback
            .map(|callback| (callback)(self.context))
    }

    /// V9
    ///
    /// Point 9 Volts.
    fn set_curve_v9(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v9_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr9
    ///
    /// Point 9 VARs.
    fn curve_v_ar9(&self) -> Option<i16> {
        self.curve_v_ar9_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr9
    ///
    /// Point 9 VARs.
    fn set_curve_v_ar9(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar9_callback {
            (callback)(value, self.context);
        };
    }

    /// V10
    ///
    /// Point 10 Volts.
    fn curve_v10(&self) -> Option<u16> {
        self.curve_v10_callback
            .map(|callback| (callback)(self.context))
    }

    /// V10
    ///
    /// Point 10 Volts.
    fn set_curve_v10(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v10_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr10
    ///
    /// Point 10 VARs.
    fn curve_v_ar10(&self) -> Option<i16> {
        self.curve_v_ar10_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr10
    ///
    /// Point 10 VARs.
    fn set_curve_v_ar10(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar10_callback {
            (callback)(value, self.context);
        };
    }

    /// V11
    ///
    /// Point 11 Volts.
    fn curve_v11(&self) -> Option<u16> {
        self.curve_v11_callback
            .map(|callback| (callback)(self.context))
    }

    /// V11
    ///
    /// Point 11 Volts.
    fn set_curve_v11(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v11_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr11
    ///
    /// Point 11 VARs.
    fn curve_v_ar11(&self) -> Option<i16> {
        self.curve_v_ar11_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr11
    ///
    /// Point 11 VARs.
    fn set_curve_v_ar11(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar11_callback {
            (callback)(value, self.context);
        };
    }

    /// V12
    ///
    /// Point 12 Volts.
    fn curve_v12(&self) -> Option<u16> {
        self.curve_v12_callback
            .map(|callback| (callback)(self.context))
    }

    /// V12
    ///
    /// Point 12 Volts.
    fn set_curve_v12(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v12_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr12
    ///
    /// Point 12 VARs.
    fn curve_v_ar12(&self) -> Option<i16> {
        self.curve_v_ar12_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr12
    ///
    /// Point 12 VARs.
    fn set_curve_v_ar12(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar12_callback {
            (callback)(value, self.context);
        };
    }

    /// V13
    ///
    /// Point 13 Volts.
    fn curve_v13(&self) -> Option<u16> {
        self.curve_v13_callback
            .map(|callback| (callback)(self.context))
    }

    /// V13
    ///
    /// Point 13 Volts.
    fn set_curve_v13(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v13_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr13
    ///
    /// Point 13 VARs.
    fn curve_v_ar13(&self) -> Option<i16> {
        self.curve_v_ar13_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr13
    ///
    /// Point 13 VARs.
    fn set_curve_v_ar13(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar13_callback {
            (callback)(value, self.context);
        };
    }

    /// V14
    ///
    /// Point 14 Volts.
    fn curve_v14(&self) -> Option<u16> {
        self.curve_v14_callback
            .map(|callback| (callback)(self.context))
    }

    /// V14
    ///
    /// Point 14 Volts.
    fn set_curve_v14(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v14_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr14
    ///
    /// Point 14 VARs.
    fn curve_v_ar14(&self) -> Option<i16> {
        self.curve_v_ar14_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr14
    ///
    /// Point 14 VARs.
    fn set_curve_v_ar14(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar14_callback {
            (callback)(value, self.context);
        };
    }

    /// V15
    ///
    /// Point 15 Volts.
    fn curve_v15(&self) -> Option<u16> {
        self.curve_v15_callback
            .map(|callback| (callback)(self.context))
    }

    /// V15
    ///
    /// Point 15 Volts.
    fn set_curve_v15(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v15_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr15
    ///
    /// Point 15 VARs.
    fn curve_v_ar15(&self) -> Option<i16> {
        self.curve_v_ar15_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr15
    ///
    /// Point 15 VARs.
    fn set_curve_v_ar15(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar15_callback {
            (callback)(value, self.context);
        };
    }

    /// V16
    ///
    /// Point 16 Volts.
    fn curve_v16(&self) -> Option<u16> {
        self.curve_v16_callback
            .map(|callback| (callback)(self.context))
    }

    /// V16
    ///
    /// Point 16 Volts.
    fn set_curve_v16(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v16_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr16
    ///
    /// Point 16 VARs.
    fn curve_v_ar16(&self) -> Option<i16> {
        self.curve_v_ar16_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr16
    ///
    /// Point 16 VARs.
    fn set_curve_v_ar16(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar16_callback {
            (callback)(value, self.context);
        };
    }

    /// V17
    ///
    /// Point 17 Volts.
    fn curve_v17(&self) -> Option<u16> {
        self.curve_v17_callback
            .map(|callback| (callback)(self.context))
    }

    /// V17
    ///
    /// Point 17 Volts.
    fn set_curve_v17(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v17_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr17
    ///
    /// Point 17 VARs.
    fn curve_v_ar17(&self) -> Option<i16> {
        self.curve_v_ar17_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr17
    ///
    /// Point 17 VARs.
    fn set_curve_v_ar17(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar17_callback {
            (callback)(value, self.context);
        };
    }

    /// V18
    ///
    /// Point 18 Volts.
    fn curve_v18(&self) -> Option<u16> {
        self.curve_v18_callback
            .map(|callback| (callback)(self.context))
    }

    /// V18
    ///
    /// Point 18 Volts.
    fn set_curve_v18(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v18_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr18
    ///
    /// Point 18 VARs.
    fn curve_v_ar18(&self) -> Option<i16> {
        self.curve_v_ar18_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr18
    ///
    /// Point 18 VARs.
    fn set_curve_v_ar18(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar18_callback {
            (callback)(value, self.context);
        };
    }

    /// V19
    ///
    /// Point 19 Volts.
    fn curve_v19(&self) -> Option<u16> {
        self.curve_v19_callback
            .map(|callback| (callback)(self.context))
    }

    /// V19
    ///
    /// Point 19 Volts.
    fn set_curve_v19(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v19_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr19
    ///
    /// Point 19 VARs.
    fn curve_v_ar19(&self) -> Option<i16> {
        self.curve_v_ar19_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr19
    ///
    /// Point 19 VARs.
    fn set_curve_v_ar19(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar19_callback {
            (callback)(value, self.context);
        };
    }

    /// V20
    ///
    /// Point 20 Volts.
    fn curve_v20(&self) -> Option<u16> {
        self.curve_v20_callback
            .map(|callback| (callback)(self.context))
    }

    /// V20
    ///
    /// Point 20 Volts.
    fn set_curve_v20(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v20_callback {
            (callback)(value, self.context);
        };
    }

    /// VAr20
    ///
    /// Point 20 VARs.
    fn curve_v_ar20(&self) -> Option<i16> {
        self.curve_v_ar20_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAr20
    ///
    /// Point 20 VARs.
    fn set_curve_v_ar20(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_v_ar20_callback {
            (callback)(value, self.context);
        };
    }

    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
    fn curve_crv_nam(&self) -> Option<&CStr> {
        self.curve_crv_nam_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
    fn set_curve_crv_nam(&mut self, value: &CStr) {
        if let Some(callback) = self.set_curve_crv_nam_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_tms(&self) -> Option<u16> {
        self.curve_rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        self.curve_rmp_dec_tmm_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_dec_tmm_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        self.curve_rmp_inc_tmm_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_inc_tmm_callback {
            (callback)(value, self.context);
        };
    }

    /// ReadOnly
    ///
    /// Boolean flag indicates if curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly {
        (self.curve_read_only_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model126StatefulAdapter {
    act_crv: u16,
    mod_ena: u16,
    win_tms: u16,
    rvrt_tms: u16,
    rmp_tms: u16,
    n_crv: u16,
    n_pt: u16,
    v_sf: u16,
    dept_ref_sf: u16,
    rmp_inc_dec_sf: u16,
    curve_act_pt: u16,
    curve_dept_ref: DeptRef,
    curve_v1: u16,
    curve_v_ar1: i16,
    curve_v2: u16,
    curve_v_ar2: i16,
    curve_v3: u16,
    curve_v_ar3: i16,
    curve_v4: u16,
    curve_v_ar4: i16,
    curve_v5: u16,
    curve_v_ar5: i16,
    curve_v6: u16,
    curve_v_ar6: i16,
    curve_v7: u16,
    curve_v_ar7: i16,
    curve_v8: u16,
    curve_v_ar8: i16,
    curve_v9: u16,
    curve_v_ar9: i16,
    curve_v10: u16,
    curve_v_ar10: i16,
    curve_v11: u16,
    curve_v_ar11: i16,
    curve_v12: u16,
    curve_v_ar12: i16,
    curve_v13: u16,
    curve_v_ar13: i16,
    curve_v14: u16,
    curve_v_ar14: i16,
    curve_v15: u16,
    curve_v_ar15: i16,
    curve_v16: u16,
    curve_v_ar16: i16,
    curve_v17: u16,
    curve_v_ar17: i16,
    curve_v18: u16,
    curve_v_ar18: i16,
    curve_v19: u16,
    curve_v_ar19: i16,
    curve_v20: u16,
    curve_v_ar20: i16,
    curve_crv_nam: [c_char; 16],
    curve_rmp_tms: u16,
    curve_rmp_dec_tmm: u16,
    curve_rmp_inc_tmm: u16,
    curve_read_only: ReadOnly,
}

impl ModelAdapter for Model126StatefulAdapter {
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
    /// Is Volt-VAR control active.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Is Volt-VAR control active.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// WinTms
    ///
    /// Time window for volt-VAR change.
    fn win_tms(&self) -> Option<u16> {
        Some(self.win_tms)
    }

    /// WinTms
    ///
    /// Time window for volt-VAR change.
    fn set_win_tms(&mut self, value: u16) {
        self.win_tms = value;
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        Some(self.rvrt_tms)
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-VAR curve selection.
    fn set_rvrt_tms(&mut self, value: u16) {
        self.rvrt_tms = value;
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn rmp_tms(&self) -> Option<u16> {
        Some(self.rmp_tms)
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
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

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16 {
        self.v_sf
    }

    /// DeptRef_SF
    ///
    /// Scale factor for dependent variable.
    fn dept_ref_sf(&self) -> u16 {
        self.dept_ref_sf
    }

    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        Some(self.rmp_inc_dec_sf)
    }

    /// ActPt
    ///
    /// Number of active points in array.
    fn curve_act_pt(&self) -> u16 {
        self.curve_act_pt
    }

    /// ActPt
    ///
    /// Number of active points in array.
    fn set_curve_act_pt(&mut self, value: u16) {
        self.curve_act_pt = value;
    }

    /// DeptRef
    ///
    /// Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.
    fn curve_dept_ref(&self) -> DeptRef {
        self.curve_dept_ref
    }

    /// DeptRef
    ///
    /// Meaning of dependent variable: 1=%WMax 2=%VArMax 3=%VArAval.
    fn set_curve_dept_ref(&mut self, value: DeptRef) {
        self.curve_dept_ref = value;
    }

    /// V1
    ///
    /// Point 1 Volts.
    fn curve_v1(&self) -> u16 {
        self.curve_v1
    }

    /// V1
    ///
    /// Point 1 Volts.
    fn set_curve_v1(&mut self, value: u16) {
        self.curve_v1 = value;
    }

    /// VAr1
    ///
    /// Point 1 VARs.
    fn curve_v_ar1(&self) -> i16 {
        self.curve_v_ar1
    }

    /// VAr1
    ///
    /// Point 1 VARs.
    fn set_curve_v_ar1(&mut self, value: i16) {
        self.curve_v_ar1 = value;
    }

    /// V2
    ///
    /// Point 2 Volts.
    fn curve_v2(&self) -> Option<u16> {
        Some(self.curve_v2)
    }

    /// V2
    ///
    /// Point 2 Volts.
    fn set_curve_v2(&mut self, value: u16) {
        self.curve_v2 = value;
    }

    /// VAr2
    ///
    /// Point 2 VARs.
    fn curve_v_ar2(&self) -> Option<i16> {
        Some(self.curve_v_ar2)
    }

    /// VAr2
    ///
    /// Point 2 VARs.
    fn set_curve_v_ar2(&mut self, value: i16) {
        self.curve_v_ar2 = value;
    }

    /// V3
    ///
    /// Point 3 Volts.
    fn curve_v3(&self) -> Option<u16> {
        Some(self.curve_v3)
    }

    /// V3
    ///
    /// Point 3 Volts.
    fn set_curve_v3(&mut self, value: u16) {
        self.curve_v3 = value;
    }

    /// VAr3
    ///
    /// Point 3 VARs.
    fn curve_v_ar3(&self) -> Option<i16> {
        Some(self.curve_v_ar3)
    }

    /// VAr3
    ///
    /// Point 3 VARs.
    fn set_curve_v_ar3(&mut self, value: i16) {
        self.curve_v_ar3 = value;
    }

    /// V4
    ///
    /// Point 4 Volts.
    fn curve_v4(&self) -> Option<u16> {
        Some(self.curve_v4)
    }

    /// V4
    ///
    /// Point 4 Volts.
    fn set_curve_v4(&mut self, value: u16) {
        self.curve_v4 = value;
    }

    /// VAr4
    ///
    /// Point 4 VARs.
    fn curve_v_ar4(&self) -> Option<i16> {
        Some(self.curve_v_ar4)
    }

    /// VAr4
    ///
    /// Point 4 VARs.
    fn set_curve_v_ar4(&mut self, value: i16) {
        self.curve_v_ar4 = value;
    }

    /// V5
    ///
    /// Point 5 Volts.
    fn curve_v5(&self) -> Option<u16> {
        Some(self.curve_v5)
    }

    /// V5
    ///
    /// Point 5 Volts.
    fn set_curve_v5(&mut self, value: u16) {
        self.curve_v5 = value;
    }

    /// VAr5
    ///
    /// Point 5 VARs.
    fn curve_v_ar5(&self) -> Option<i16> {
        Some(self.curve_v_ar5)
    }

    /// VAr5
    ///
    /// Point 5 VARs.
    fn set_curve_v_ar5(&mut self, value: i16) {
        self.curve_v_ar5 = value;
    }

    /// V6
    ///
    /// Point 6 Volts.
    fn curve_v6(&self) -> Option<u16> {
        Some(self.curve_v6)
    }

    /// V6
    ///
    /// Point 6 Volts.
    fn set_curve_v6(&mut self, value: u16) {
        self.curve_v6 = value;
    }

    /// VAr6
    ///
    /// Point 6 VARs.
    fn curve_v_ar6(&self) -> Option<i16> {
        Some(self.curve_v_ar6)
    }

    /// VAr6
    ///
    /// Point 6 VARs.
    fn set_curve_v_ar6(&mut self, value: i16) {
        self.curve_v_ar6 = value;
    }

    /// V7
    ///
    /// Point 7 Volts.
    fn curve_v7(&self) -> Option<u16> {
        Some(self.curve_v7)
    }

    /// V7
    ///
    /// Point 7 Volts.
    fn set_curve_v7(&mut self, value: u16) {
        self.curve_v7 = value;
    }

    /// VAr7
    ///
    /// Point 7 VARs.
    fn curve_v_ar7(&self) -> Option<i16> {
        Some(self.curve_v_ar7)
    }

    /// VAr7
    ///
    /// Point 7 VARs.
    fn set_curve_v_ar7(&mut self, value: i16) {
        self.curve_v_ar7 = value;
    }

    /// V8
    ///
    /// Point 8 Volts.
    fn curve_v8(&self) -> Option<u16> {
        Some(self.curve_v8)
    }

    /// V8
    ///
    /// Point 8 Volts.
    fn set_curve_v8(&mut self, value: u16) {
        self.curve_v8 = value;
    }

    /// VAr8
    ///
    /// Point 8 VARs.
    fn curve_v_ar8(&self) -> Option<i16> {
        Some(self.curve_v_ar8)
    }

    /// VAr8
    ///
    /// Point 8 VARs.
    fn set_curve_v_ar8(&mut self, value: i16) {
        self.curve_v_ar8 = value;
    }

    /// V9
    ///
    /// Point 9 Volts.
    fn curve_v9(&self) -> Option<u16> {
        Some(self.curve_v9)
    }

    /// V9
    ///
    /// Point 9 Volts.
    fn set_curve_v9(&mut self, value: u16) {
        self.curve_v9 = value;
    }

    /// VAr9
    ///
    /// Point 9 VARs.
    fn curve_v_ar9(&self) -> Option<i16> {
        Some(self.curve_v_ar9)
    }

    /// VAr9
    ///
    /// Point 9 VARs.
    fn set_curve_v_ar9(&mut self, value: i16) {
        self.curve_v_ar9 = value;
    }

    /// V10
    ///
    /// Point 10 Volts.
    fn curve_v10(&self) -> Option<u16> {
        Some(self.curve_v10)
    }

    /// V10
    ///
    /// Point 10 Volts.
    fn set_curve_v10(&mut self, value: u16) {
        self.curve_v10 = value;
    }

    /// VAr10
    ///
    /// Point 10 VARs.
    fn curve_v_ar10(&self) -> Option<i16> {
        Some(self.curve_v_ar10)
    }

    /// VAr10
    ///
    /// Point 10 VARs.
    fn set_curve_v_ar10(&mut self, value: i16) {
        self.curve_v_ar10 = value;
    }

    /// V11
    ///
    /// Point 11 Volts.
    fn curve_v11(&self) -> Option<u16> {
        Some(self.curve_v11)
    }

    /// V11
    ///
    /// Point 11 Volts.
    fn set_curve_v11(&mut self, value: u16) {
        self.curve_v11 = value;
    }

    /// VAr11
    ///
    /// Point 11 VARs.
    fn curve_v_ar11(&self) -> Option<i16> {
        Some(self.curve_v_ar11)
    }

    /// VAr11
    ///
    /// Point 11 VARs.
    fn set_curve_v_ar11(&mut self, value: i16) {
        self.curve_v_ar11 = value;
    }

    /// V12
    ///
    /// Point 12 Volts.
    fn curve_v12(&self) -> Option<u16> {
        Some(self.curve_v12)
    }

    /// V12
    ///
    /// Point 12 Volts.
    fn set_curve_v12(&mut self, value: u16) {
        self.curve_v12 = value;
    }

    /// VAr12
    ///
    /// Point 12 VARs.
    fn curve_v_ar12(&self) -> Option<i16> {
        Some(self.curve_v_ar12)
    }

    /// VAr12
    ///
    /// Point 12 VARs.
    fn set_curve_v_ar12(&mut self, value: i16) {
        self.curve_v_ar12 = value;
    }

    /// V13
    ///
    /// Point 13 Volts.
    fn curve_v13(&self) -> Option<u16> {
        Some(self.curve_v13)
    }

    /// V13
    ///
    /// Point 13 Volts.
    fn set_curve_v13(&mut self, value: u16) {
        self.curve_v13 = value;
    }

    /// VAr13
    ///
    /// Point 13 VARs.
    fn curve_v_ar13(&self) -> Option<i16> {
        Some(self.curve_v_ar13)
    }

    /// VAr13
    ///
    /// Point 13 VARs.
    fn set_curve_v_ar13(&mut self, value: i16) {
        self.curve_v_ar13 = value;
    }

    /// V14
    ///
    /// Point 14 Volts.
    fn curve_v14(&self) -> Option<u16> {
        Some(self.curve_v14)
    }

    /// V14
    ///
    /// Point 14 Volts.
    fn set_curve_v14(&mut self, value: u16) {
        self.curve_v14 = value;
    }

    /// VAr14
    ///
    /// Point 14 VARs.
    fn curve_v_ar14(&self) -> Option<i16> {
        Some(self.curve_v_ar14)
    }

    /// VAr14
    ///
    /// Point 14 VARs.
    fn set_curve_v_ar14(&mut self, value: i16) {
        self.curve_v_ar14 = value;
    }

    /// V15
    ///
    /// Point 15 Volts.
    fn curve_v15(&self) -> Option<u16> {
        Some(self.curve_v15)
    }

    /// V15
    ///
    /// Point 15 Volts.
    fn set_curve_v15(&mut self, value: u16) {
        self.curve_v15 = value;
    }

    /// VAr15
    ///
    /// Point 15 VARs.
    fn curve_v_ar15(&self) -> Option<i16> {
        Some(self.curve_v_ar15)
    }

    /// VAr15
    ///
    /// Point 15 VARs.
    fn set_curve_v_ar15(&mut self, value: i16) {
        self.curve_v_ar15 = value;
    }

    /// V16
    ///
    /// Point 16 Volts.
    fn curve_v16(&self) -> Option<u16> {
        Some(self.curve_v16)
    }

    /// V16
    ///
    /// Point 16 Volts.
    fn set_curve_v16(&mut self, value: u16) {
        self.curve_v16 = value;
    }

    /// VAr16
    ///
    /// Point 16 VARs.
    fn curve_v_ar16(&self) -> Option<i16> {
        Some(self.curve_v_ar16)
    }

    /// VAr16
    ///
    /// Point 16 VARs.
    fn set_curve_v_ar16(&mut self, value: i16) {
        self.curve_v_ar16 = value;
    }

    /// V17
    ///
    /// Point 17 Volts.
    fn curve_v17(&self) -> Option<u16> {
        Some(self.curve_v17)
    }

    /// V17
    ///
    /// Point 17 Volts.
    fn set_curve_v17(&mut self, value: u16) {
        self.curve_v17 = value;
    }

    /// VAr17
    ///
    /// Point 17 VARs.
    fn curve_v_ar17(&self) -> Option<i16> {
        Some(self.curve_v_ar17)
    }

    /// VAr17
    ///
    /// Point 17 VARs.
    fn set_curve_v_ar17(&mut self, value: i16) {
        self.curve_v_ar17 = value;
    }

    /// V18
    ///
    /// Point 18 Volts.
    fn curve_v18(&self) -> Option<u16> {
        Some(self.curve_v18)
    }

    /// V18
    ///
    /// Point 18 Volts.
    fn set_curve_v18(&mut self, value: u16) {
        self.curve_v18 = value;
    }

    /// VAr18
    ///
    /// Point 18 VARs.
    fn curve_v_ar18(&self) -> Option<i16> {
        Some(self.curve_v_ar18)
    }

    /// VAr18
    ///
    /// Point 18 VARs.
    fn set_curve_v_ar18(&mut self, value: i16) {
        self.curve_v_ar18 = value;
    }

    /// V19
    ///
    /// Point 19 Volts.
    fn curve_v19(&self) -> Option<u16> {
        Some(self.curve_v19)
    }

    /// V19
    ///
    /// Point 19 Volts.
    fn set_curve_v19(&mut self, value: u16) {
        self.curve_v19 = value;
    }

    /// VAr19
    ///
    /// Point 19 VARs.
    fn curve_v_ar19(&self) -> Option<i16> {
        Some(self.curve_v_ar19)
    }

    /// VAr19
    ///
    /// Point 19 VARs.
    fn set_curve_v_ar19(&mut self, value: i16) {
        self.curve_v_ar19 = value;
    }

    /// V20
    ///
    /// Point 20 Volts.
    fn curve_v20(&self) -> Option<u16> {
        Some(self.curve_v20)
    }

    /// V20
    ///
    /// Point 20 Volts.
    fn set_curve_v20(&mut self, value: u16) {
        self.curve_v20 = value;
    }

    /// VAr20
    ///
    /// Point 20 VARs.
    fn curve_v_ar20(&self) -> Option<i16> {
        Some(self.curve_v_ar20)
    }

    /// VAr20
    ///
    /// Point 20 VARs.
    fn set_curve_v_ar20(&mut self, value: i16) {
        self.curve_v_ar20 = value;
    }

    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
    fn curve_crv_nam(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.curve_crv_nam.as_ptr()) })
    }

    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
    fn set_curve_crv_nam(&mut self, value: &CStr) {
        for (dest, src) in self
            .curve_crv_nam
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_tms(&self) -> Option<u16> {
        Some(self.curve_rmp_tms)
    }

    /// RmpTms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_tms(&mut self, value: u16) {
        self.curve_rmp_tms = value;
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        Some(self.curve_rmp_dec_tmm)
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the VAR value may be reduced in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {
        self.curve_rmp_dec_tmm = value;
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        Some(self.curve_rmp_inc_tmm)
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the VAR value may be increased in response to changes in the voltage value. %refVal is %WMax %VArMax or %VArAval depending on value of DeptRef.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {
        self.curve_rmp_inc_tmm = value;
    }

    /// ReadOnly
    ///
    /// Boolean flag indicates if curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly {
        self.curve_read_only
    }
}
