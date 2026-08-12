use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 66;

static POINTS: [PointDetails<()>; 59] = [
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
        point: |()| Point::ActCrv,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::ModEna,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::WinTms,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::RvrtTms,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::RmpTms,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::NCrv,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::NPt,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::VSf,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::DeptRefSf,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::RmpIncDecSf,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::CurveActPt,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::CurveDeptRef,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::CurveV1,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::CurveW1,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::CurveV2,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::CurveW2,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::CurveV3,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::CurveW3,
        size: 1,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::CurveV4,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::CurveW4,
        size: 1,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::CurveV5,
        size: 1,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::CurveW5,
        size: 1,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::CurveV6,
        size: 1,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::CurveW6,
        size: 1,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::CurveV7,
        size: 1,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::CurveW7,
        size: 1,
        start_address: 27,
    },
    PointDetails {
        point: |()| Point::CurveV8,
        size: 1,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::CurveW8,
        size: 1,
        start_address: 29,
    },
    PointDetails {
        point: |()| Point::CurveV9,
        size: 1,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::CurveW9,
        size: 1,
        start_address: 31,
    },
    PointDetails {
        point: |()| Point::CurveV10,
        size: 1,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::CurveW10,
        size: 1,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::CurveV11,
        size: 1,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::CurveW11,
        size: 1,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::CurveV12,
        size: 1,
        start_address: 36,
    },
    PointDetails {
        point: |()| Point::CurveW12,
        size: 1,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::CurveV13,
        size: 1,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::CurveW13,
        size: 1,
        start_address: 39,
    },
    PointDetails {
        point: |()| Point::CurveV14,
        size: 1,
        start_address: 40,
    },
    PointDetails {
        point: |()| Point::CurveW14,
        size: 1,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::CurveV15,
        size: 1,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::CurveW15,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::CurveV16,
        size: 1,
        start_address: 44,
    },
    PointDetails {
        point: |()| Point::CurveW16,
        size: 1,
        start_address: 45,
    },
    PointDetails {
        point: |()| Point::CurveV17,
        size: 1,
        start_address: 46,
    },
    PointDetails {
        point: |()| Point::CurveW17,
        size: 1,
        start_address: 47,
    },
    PointDetails {
        point: |()| Point::CurveV18,
        size: 1,
        start_address: 48,
    },
    PointDetails {
        point: |()| Point::CurveW18,
        size: 1,
        start_address: 49,
    },
    PointDetails {
        point: |()| Point::CurveV19,
        size: 1,
        start_address: 50,
    },
    PointDetails {
        point: |()| Point::CurveW19,
        size: 1,
        start_address: 51,
    },
    PointDetails {
        point: |()| Point::CurveV20,
        size: 1,
        start_address: 52,
    },
    PointDetails {
        point: |()| Point::CurveW20,
        size: 1,
        start_address: 53,
    },
    PointDetails {
        point: |()| Point::CurveCrvNam,
        size: 8,
        start_address: 54,
    },
    PointDetails {
        point: |()| Point::CurveRmpPt1Tms,
        size: 1,
        start_address: 62,
    },
    PointDetails {
        point: |()| Point::CurveRmpDecTmm,
        size: 1,
        start_address: 63,
    },
    PointDetails {
        point: |()| Point::CurveRmpIncTmm,
        size: 1,
        start_address: 64,
    },
    PointDetails {
        point: |()| Point::CurveReadOnly,
        size: 1,
        start_address: 65,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
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
    CurveW1,
    CurveV2,
    CurveW2,
    CurveV3,
    CurveW3,
    CurveV4,
    CurveW4,
    CurveV5,
    CurveW5,
    CurveV6,
    CurveW6,
    CurveV7,
    CurveW7,
    CurveV8,
    CurveW8,
    CurveV9,
    CurveW9,
    CurveV10,
    CurveW10,
    CurveV11,
    CurveW11,
    CurveV12,
    CurveW12,
    CurveV13,
    CurveW13,
    CurveV14,
    CurveW14,
    CurveV15,
    CurveW15,
    CurveV16,
    CurveW16,
    CurveV17,
    CurveW17,
    CurveV18,
    CurveW18,
    CurveV19,
    CurveW19,
    CurveV20,
    CurveW20,
    CurveCrvNam,
    CurveRmpPt1Tms,
    CurveRmpDecTmm,
    CurveRmpIncTmm,
    CurveReadOnly,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    66
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
            buffer::write_u16(132, buffer);
        }
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
                buffer::zero(buffer, 1);
            }
        }
        Point::RvrtTms => {
            if let Some(value) = model.rvrt_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::RmpTms => {
            if let Some(value) = model.rmp_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
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
                buffer::zero(buffer, 1);
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
        Point::CurveW1 => {
            buffer::write_i16(model.curve_w1(), buffer);
        }
        Point::CurveV2 => {
            if let Some(value) = model.curve_v2() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW2 => {
            if let Some(value) = model.curve_w2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV3 => {
            if let Some(value) = model.curve_v3() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW3 => {
            if let Some(value) = model.curve_w3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV4 => {
            if let Some(value) = model.curve_v4() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW4 => {
            if let Some(value) = model.curve_w4() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV5 => {
            if let Some(value) = model.curve_v5() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW5 => {
            if let Some(value) = model.curve_w5() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV6 => {
            if let Some(value) = model.curve_v6() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW6 => {
            if let Some(value) = model.curve_w6() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV7 => {
            if let Some(value) = model.curve_v7() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW7 => {
            if let Some(value) = model.curve_w7() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV8 => {
            if let Some(value) = model.curve_v8() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW8 => {
            if let Some(value) = model.curve_w8() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV9 => {
            if let Some(value) = model.curve_v9() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW9 => {
            if let Some(value) = model.curve_w9() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV10 => {
            if let Some(value) = model.curve_v10() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW10 => {
            if let Some(value) = model.curve_w10() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV11 => {
            if let Some(value) = model.curve_v11() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW11 => {
            if let Some(value) = model.curve_w11() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV12 => {
            if let Some(value) = model.curve_v12() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW12 => {
            if let Some(value) = model.curve_w12() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV13 => {
            if let Some(value) = model.curve_v13() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW13 => {
            if let Some(value) = model.curve_w13() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV14 => {
            if let Some(value) = model.curve_v14() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW14 => {
            if let Some(value) = model.curve_w14() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV15 => {
            if let Some(value) = model.curve_v15() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW15 => {
            if let Some(value) = model.curve_w15() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV16 => {
            if let Some(value) = model.curve_v16() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW16 => {
            if let Some(value) = model.curve_w16() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV17 => {
            if let Some(value) = model.curve_v17() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW17 => {
            if let Some(value) = model.curve_w17() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV18 => {
            if let Some(value) = model.curve_v18() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW18 => {
            if let Some(value) = model.curve_w18() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV19 => {
            if let Some(value) = model.curve_v19() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW19 => {
            if let Some(value) = model.curve_w19() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveV20 => {
            if let Some(value) = model.curve_v20() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveW20 => {
            if let Some(value) = model.curve_w20() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveCrvNam => {
            if let Some(value) = model.curve_crv_nam() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 8);
            }
        }
        Point::CurveRmpPt1Tms => {
            if let Some(value) = model.curve_rmp_pt1_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveRmpDecTmm => {
            if let Some(value) = model.curve_rmp_dec_tmm() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveRmpIncTmm => {
            if let Some(value) = model.curve_rmp_inc_tmm() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
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
    /// Is Volt-Watt control active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is Volt-Watt control active.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for volt-watt change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for volt-watt change.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for volt-watt curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-watt curve selection.
    fn set_rvrt_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    fn set_rmp_tms(&mut self, value: u16) {}

    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    fn n_crv(&self) -> u16;

    /// NPt
    ///
    /// Number of points in array (maximum 20).
    fn n_pt(&self) -> u16;

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16;

    /// DeptRef_SF
    ///
    /// Scale Factor for % DeptRef
    fn dept_ref_sf(&self) -> u16;

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
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
    /// Defines the meaning of the Watts DeptRef. 1=% WMax 2=% WAvail
    fn curve_dept_ref(&self) -> DeptRef;

    /// DeptRef
    ///
    /// Defines the meaning of the Watts DeptRef. 1=% WMax 2=% WAvail
    fn set_curve_dept_ref(&mut self, value: DeptRef);

    /// V1
    ///
    /// Point 1 Volts.
    fn curve_v1(&self) -> u16;

    /// V1
    ///
    /// Point 1 Volts.
    fn set_curve_v1(&mut self, value: u16);

    /// W1
    ///
    /// Point 1 Watts.
    fn curve_w1(&self) -> i16;

    /// W1
    ///
    /// Point 1 Watts.
    fn set_curve_w1(&mut self, value: i16);

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

    /// W2
    ///
    /// Point 2 Watts.
    fn curve_w2(&self) -> Option<i16> {
        None
    }

    /// W2
    ///
    /// Point 2 Watts.
    fn set_curve_w2(&mut self, value: i16) {}

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

    /// W3
    ///
    /// Point 3 Watts.
    fn curve_w3(&self) -> Option<i16> {
        None
    }

    /// W3
    ///
    /// Point 3 Watts.
    fn set_curve_w3(&mut self, value: i16) {}

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

    /// W4
    ///
    /// Point 4 Watts.
    fn curve_w4(&self) -> Option<i16> {
        None
    }

    /// W4
    ///
    /// Point 4 Watts.
    fn set_curve_w4(&mut self, value: i16) {}

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

    /// W5
    ///
    /// Point 5 Watts.
    fn curve_w5(&self) -> Option<i16> {
        None
    }

    /// W5
    ///
    /// Point 5 Watts.
    fn set_curve_w5(&mut self, value: i16) {}

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

    /// W6
    ///
    /// Point 6 Watts.
    fn curve_w6(&self) -> Option<i16> {
        None
    }

    /// W6
    ///
    /// Point 6 Watts.
    fn set_curve_w6(&mut self, value: i16) {}

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

    /// W7
    ///
    /// Point 7 Watts.
    fn curve_w7(&self) -> Option<i16> {
        None
    }

    /// W7
    ///
    /// Point 7 Watts.
    fn set_curve_w7(&mut self, value: i16) {}

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

    /// W8
    ///
    /// Point 8 Watts.
    fn curve_w8(&self) -> Option<i16> {
        None
    }

    /// W8
    ///
    /// Point 8 Watts.
    fn set_curve_w8(&mut self, value: i16) {}

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

    /// W9
    ///
    /// Point 9 Watts.
    fn curve_w9(&self) -> Option<i16> {
        None
    }

    /// W9
    ///
    /// Point 9 Watts.
    fn set_curve_w9(&mut self, value: i16) {}

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

    /// W10
    ///
    /// Point 10 Watts.
    fn curve_w10(&self) -> Option<i16> {
        None
    }

    /// W10
    ///
    /// Point 10 Watts.
    fn set_curve_w10(&mut self, value: i16) {}

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

    /// W11
    ///
    /// Point 11 Watts.
    fn curve_w11(&self) -> Option<i16> {
        None
    }

    /// W11
    ///
    /// Point 11 Watts.
    fn set_curve_w11(&mut self, value: i16) {}

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

    /// W12
    ///
    /// Point 12 Watts.
    fn curve_w12(&self) -> Option<i16> {
        None
    }

    /// W12
    ///
    /// Point 12 Watts.
    fn set_curve_w12(&mut self, value: i16) {}

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

    /// W13
    ///
    /// Point 13 Watts.
    fn curve_w13(&self) -> Option<i16> {
        None
    }

    /// W13
    ///
    /// Point 13 Watts.
    fn set_curve_w13(&mut self, value: i16) {}

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

    /// W14
    ///
    /// Point 14 Watts.
    fn curve_w14(&self) -> Option<i16> {
        None
    }

    /// W14
    ///
    /// Point 14 Watts.
    fn set_curve_w14(&mut self, value: i16) {}

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

    /// W15
    ///
    /// Point 15 Watts.
    fn curve_w15(&self) -> Option<i16> {
        None
    }

    /// W15
    ///
    /// Point 15 Watts.
    fn set_curve_w15(&mut self, value: i16) {}

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

    /// W16
    ///
    /// Point 16 Watts.
    fn curve_w16(&self) -> Option<i16> {
        None
    }

    /// W16
    ///
    /// Point 16 Watts.
    fn set_curve_w16(&mut self, value: i16) {}

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

    /// W17
    ///
    /// Point 17 Watts.
    fn curve_w17(&self) -> Option<i16> {
        None
    }

    /// W17
    ///
    /// Point 17 Watts.
    fn set_curve_w17(&mut self, value: i16) {}

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

    /// W18
    ///
    /// Point 18 Watts.
    fn curve_w18(&self) -> Option<i16> {
        None
    }

    /// W18
    ///
    /// Point 18 Watts.
    fn set_curve_w18(&mut self, value: i16) {}

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

    /// W19
    ///
    /// Point 19 Watts.
    fn curve_w19(&self) -> Option<i16> {
        None
    }

    /// W19
    ///
    /// Point 19 Watts.
    fn set_curve_w19(&mut self, value: i16) {}

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

    /// W20
    ///
    /// Point 20 Watts.
    fn curve_w20(&self) -> Option<i16> {
        None
    }

    /// W20
    ///
    /// Point 20 Watts.
    fn set_curve_w20(&mut self, value: i16) {}

    /// CrvNam
    ///
    /// Optional description for curve.
    fn curve_crv_nam(&self) -> Option<&CStr> {
        None
    }

    /// CrvNam
    ///
    /// Optional description for curve.
    fn set_curve_crv_nam(&mut self, value: &CStr) {}

    /// RmpPt1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_pt1_tms(&self) -> Option<u16> {
        None
    }

    /// RmpPt1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_pt1_tms(&mut self, value: u16) {}

    /// RmpDecTmm
    ///
    /// The maximum rate at which the watt value may be reduced in response to changes in the voltage value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        None
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the watt value may be reduced in response to changes in the voltage value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {}

    /// RmpIncTmm
    ///
    /// The maximum rate at which the watt value may be increased in response to changes in the voltage value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        None
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the watt value may be increased in response to changes in the voltage value.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {}

    /// ReadOnly
    ///
    /// Curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum DeptRef {
    WMax = 1,
    WAval = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ReadOnly {
    Readwrite = 0,
    Readonly = 1,
}

#[repr(C)]
pub struct Model132CallbackAdapter {
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
    curve_w1_callback: extern "C" fn(*const c_void) -> i16,
    set_curve_w1_callback: extern "C" fn(i16, *mut c_void),
    curve_v2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v2_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w2_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v3_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w3_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v4_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w4_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w4_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v5_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w5_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w5_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v6_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v6_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w6_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w6_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v7_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v7_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w7_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w7_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v8_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v8_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w8_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w8_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v9_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v9_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w9_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w9_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v10_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v10_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w10_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w10_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v11_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v11_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w11_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w11_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v12_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v12_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w12_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w12_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v13_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v13_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w13_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w13_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v14_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v14_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w14_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w14_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v15_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v15_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w15_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w15_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v16_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w16_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w16_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v17_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v17_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w17_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w17_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v18_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v18_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w18_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w18_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v19_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v19_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w19_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w19_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_v20_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v20_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w20_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w20_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_crv_nam_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_curve_crv_nam_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    curve_rmp_pt1_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_rmp_pt1_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_rmp_dec_tmm_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_rmp_dec_tmm_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_rmp_inc_tmm_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_rmp_inc_tmm_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_read_only_callback: extern "C" fn(*const c_void) -> ReadOnly,
}

impl ModelAdapter for Model132CallbackAdapter {
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
    /// Is Volt-Watt control active.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Is Volt-Watt control active.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// WinTms
    ///
    /// Time window for volt-watt change.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WinTms
    ///
    /// Time window for volt-watt change.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-watt curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        self.rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-watt curve selection.
    fn set_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    fn rmp_tms(&self) -> Option<u16> {
        self.rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    fn set_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rmp_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    fn n_crv(&self) -> u16 {
        (self.n_crv_callback)(self.context)
    }

    /// NPt
    ///
    /// Number of points in array (maximum 20).
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
    /// Scale Factor for % DeptRef
    fn dept_ref_sf(&self) -> u16 {
        (self.dept_ref_sf_callback)(self.context)
    }

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
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
    /// Defines the meaning of the Watts DeptRef. 1=% WMax 2=% WAvail
    fn curve_dept_ref(&self) -> DeptRef {
        (self.curve_dept_ref_callback)(self.context)
    }

    /// DeptRef
    ///
    /// Defines the meaning of the Watts DeptRef. 1=% WMax 2=% WAvail
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

    /// W1
    ///
    /// Point 1 Watts.
    fn curve_w1(&self) -> i16 {
        (self.curve_w1_callback)(self.context)
    }

    /// W1
    ///
    /// Point 1 Watts.
    fn set_curve_w1(&mut self, value: i16) {
        (self.set_curve_w1_callback)(value, self.context);
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

    /// W2
    ///
    /// Point 2 Watts.
    fn curve_w2(&self) -> Option<i16> {
        self.curve_w2_callback
            .map(|callback| (callback)(self.context))
    }

    /// W2
    ///
    /// Point 2 Watts.
    fn set_curve_w2(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w2_callback {
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

    /// W3
    ///
    /// Point 3 Watts.
    fn curve_w3(&self) -> Option<i16> {
        self.curve_w3_callback
            .map(|callback| (callback)(self.context))
    }

    /// W3
    ///
    /// Point 3 Watts.
    fn set_curve_w3(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w3_callback {
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

    /// W4
    ///
    /// Point 4 Watts.
    fn curve_w4(&self) -> Option<i16> {
        self.curve_w4_callback
            .map(|callback| (callback)(self.context))
    }

    /// W4
    ///
    /// Point 4 Watts.
    fn set_curve_w4(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w4_callback {
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

    /// W5
    ///
    /// Point 5 Watts.
    fn curve_w5(&self) -> Option<i16> {
        self.curve_w5_callback
            .map(|callback| (callback)(self.context))
    }

    /// W5
    ///
    /// Point 5 Watts.
    fn set_curve_w5(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w5_callback {
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

    /// W6
    ///
    /// Point 6 Watts.
    fn curve_w6(&self) -> Option<i16> {
        self.curve_w6_callback
            .map(|callback| (callback)(self.context))
    }

    /// W6
    ///
    /// Point 6 Watts.
    fn set_curve_w6(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w6_callback {
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

    /// W7
    ///
    /// Point 7 Watts.
    fn curve_w7(&self) -> Option<i16> {
        self.curve_w7_callback
            .map(|callback| (callback)(self.context))
    }

    /// W7
    ///
    /// Point 7 Watts.
    fn set_curve_w7(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w7_callback {
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

    /// W8
    ///
    /// Point 8 Watts.
    fn curve_w8(&self) -> Option<i16> {
        self.curve_w8_callback
            .map(|callback| (callback)(self.context))
    }

    /// W8
    ///
    /// Point 8 Watts.
    fn set_curve_w8(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w8_callback {
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

    /// W9
    ///
    /// Point 9 Watts.
    fn curve_w9(&self) -> Option<i16> {
        self.curve_w9_callback
            .map(|callback| (callback)(self.context))
    }

    /// W9
    ///
    /// Point 9 Watts.
    fn set_curve_w9(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w9_callback {
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

    /// W10
    ///
    /// Point 10 Watts.
    fn curve_w10(&self) -> Option<i16> {
        self.curve_w10_callback
            .map(|callback| (callback)(self.context))
    }

    /// W10
    ///
    /// Point 10 Watts.
    fn set_curve_w10(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w10_callback {
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

    /// W11
    ///
    /// Point 11 Watts.
    fn curve_w11(&self) -> Option<i16> {
        self.curve_w11_callback
            .map(|callback| (callback)(self.context))
    }

    /// W11
    ///
    /// Point 11 Watts.
    fn set_curve_w11(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w11_callback {
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

    /// W12
    ///
    /// Point 12 Watts.
    fn curve_w12(&self) -> Option<i16> {
        self.curve_w12_callback
            .map(|callback| (callback)(self.context))
    }

    /// W12
    ///
    /// Point 12 Watts.
    fn set_curve_w12(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w12_callback {
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

    /// W13
    ///
    /// Point 13 Watts.
    fn curve_w13(&self) -> Option<i16> {
        self.curve_w13_callback
            .map(|callback| (callback)(self.context))
    }

    /// W13
    ///
    /// Point 13 Watts.
    fn set_curve_w13(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w13_callback {
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

    /// W14
    ///
    /// Point 14 Watts.
    fn curve_w14(&self) -> Option<i16> {
        self.curve_w14_callback
            .map(|callback| (callback)(self.context))
    }

    /// W14
    ///
    /// Point 14 Watts.
    fn set_curve_w14(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w14_callback {
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

    /// W15
    ///
    /// Point 15 Watts.
    fn curve_w15(&self) -> Option<i16> {
        self.curve_w15_callback
            .map(|callback| (callback)(self.context))
    }

    /// W15
    ///
    /// Point 15 Watts.
    fn set_curve_w15(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w15_callback {
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

    /// W16
    ///
    /// Point 16 Watts.
    fn curve_w16(&self) -> Option<i16> {
        self.curve_w16_callback
            .map(|callback| (callback)(self.context))
    }

    /// W16
    ///
    /// Point 16 Watts.
    fn set_curve_w16(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w16_callback {
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

    /// W17
    ///
    /// Point 17 Watts.
    fn curve_w17(&self) -> Option<i16> {
        self.curve_w17_callback
            .map(|callback| (callback)(self.context))
    }

    /// W17
    ///
    /// Point 17 Watts.
    fn set_curve_w17(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w17_callback {
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

    /// W18
    ///
    /// Point 18 Watts.
    fn curve_w18(&self) -> Option<i16> {
        self.curve_w18_callback
            .map(|callback| (callback)(self.context))
    }

    /// W18
    ///
    /// Point 18 Watts.
    fn set_curve_w18(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w18_callback {
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

    /// W19
    ///
    /// Point 19 Watts.
    fn curve_w19(&self) -> Option<i16> {
        self.curve_w19_callback
            .map(|callback| (callback)(self.context))
    }

    /// W19
    ///
    /// Point 19 Watts.
    fn set_curve_w19(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w19_callback {
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

    /// W20
    ///
    /// Point 20 Watts.
    fn curve_w20(&self) -> Option<i16> {
        self.curve_w20_callback
            .map(|callback| (callback)(self.context))
    }

    /// W20
    ///
    /// Point 20 Watts.
    fn set_curve_w20(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_w20_callback {
            (callback)(value, self.context);
        };
    }

    /// CrvNam
    ///
    /// Optional description for curve.
    fn curve_crv_nam(&self) -> Option<&CStr> {
        self.curve_crv_nam_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// CrvNam
    ///
    /// Optional description for curve.
    fn set_curve_crv_nam(&mut self, value: &CStr) {
        if let Some(callback) = self.set_curve_crv_nam_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// RmpPt1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_pt1_tms(&self) -> Option<u16> {
        self.curve_rmp_pt1_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpPt1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_pt1_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_pt1_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the watt value may be reduced in response to changes in the voltage value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        self.curve_rmp_dec_tmm_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the watt value may be reduced in response to changes in the voltage value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_dec_tmm_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the watt value may be increased in response to changes in the voltage value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        self.curve_rmp_inc_tmm_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the watt value may be increased in response to changes in the voltage value.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_inc_tmm_callback {
            (callback)(value, self.context);
        };
    }

    /// ReadOnly
    ///
    /// Curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly {
        (self.curve_read_only_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model132StatefulAdapter {
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
    curve_w1: i16,
    curve_v2: u16,
    curve_w2: i16,
    curve_v3: u16,
    curve_w3: i16,
    curve_v4: u16,
    curve_w4: i16,
    curve_v5: u16,
    curve_w5: i16,
    curve_v6: u16,
    curve_w6: i16,
    curve_v7: u16,
    curve_w7: i16,
    curve_v8: u16,
    curve_w8: i16,
    curve_v9: u16,
    curve_w9: i16,
    curve_v10: u16,
    curve_w10: i16,
    curve_v11: u16,
    curve_w11: i16,
    curve_v12: u16,
    curve_w12: i16,
    curve_v13: u16,
    curve_w13: i16,
    curve_v14: u16,
    curve_w14: i16,
    curve_v15: u16,
    curve_w15: i16,
    curve_v16: u16,
    curve_w16: i16,
    curve_v17: u16,
    curve_w17: i16,
    curve_v18: u16,
    curve_w18: i16,
    curve_v19: u16,
    curve_w19: i16,
    curve_v20: u16,
    curve_w20: i16,
    curve_crv_nam: [c_char; 16],
    curve_rmp_pt1_tms: u16,
    curve_rmp_dec_tmm: u16,
    curve_rmp_inc_tmm: u16,
    curve_read_only: ReadOnly,
}

impl ModelAdapter for Model132StatefulAdapter {
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
    /// Is Volt-Watt control active.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Is Volt-Watt control active.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// WinTms
    ///
    /// Time window for volt-watt change.
    fn win_tms(&self) -> Option<u16> {
        Some(self.win_tms)
    }

    /// WinTms
    ///
    /// Time window for volt-watt change.
    fn set_win_tms(&mut self, value: u16) {
        self.win_tms = value;
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-watt curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        Some(self.rvrt_tms)
    }

    /// RvrtTms
    ///
    /// Timeout period for volt-watt curve selection.
    fn set_rvrt_tms(&mut self, value: u16) {
        self.rvrt_tms = value;
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    fn rmp_tms(&self) -> Option<u16> {
        Some(self.rmp_tms)
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    fn set_rmp_tms(&mut self, value: u16) {
        self.rmp_tms = value;
    }

    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    fn n_crv(&self) -> u16 {
        self.n_crv
    }

    /// NPt
    ///
    /// Number of points in array (maximum 20).
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
    /// Scale Factor for % DeptRef
    fn dept_ref_sf(&self) -> u16 {
        self.dept_ref_sf
    }

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
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
    /// Defines the meaning of the Watts DeptRef. 1=% WMax 2=% WAvail
    fn curve_dept_ref(&self) -> DeptRef {
        self.curve_dept_ref
    }

    /// DeptRef
    ///
    /// Defines the meaning of the Watts DeptRef. 1=% WMax 2=% WAvail
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

    /// W1
    ///
    /// Point 1 Watts.
    fn curve_w1(&self) -> i16 {
        self.curve_w1
    }

    /// W1
    ///
    /// Point 1 Watts.
    fn set_curve_w1(&mut self, value: i16) {
        self.curve_w1 = value;
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

    /// W2
    ///
    /// Point 2 Watts.
    fn curve_w2(&self) -> Option<i16> {
        Some(self.curve_w2)
    }

    /// W2
    ///
    /// Point 2 Watts.
    fn set_curve_w2(&mut self, value: i16) {
        self.curve_w2 = value;
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

    /// W3
    ///
    /// Point 3 Watts.
    fn curve_w3(&self) -> Option<i16> {
        Some(self.curve_w3)
    }

    /// W3
    ///
    /// Point 3 Watts.
    fn set_curve_w3(&mut self, value: i16) {
        self.curve_w3 = value;
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

    /// W4
    ///
    /// Point 4 Watts.
    fn curve_w4(&self) -> Option<i16> {
        Some(self.curve_w4)
    }

    /// W4
    ///
    /// Point 4 Watts.
    fn set_curve_w4(&mut self, value: i16) {
        self.curve_w4 = value;
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

    /// W5
    ///
    /// Point 5 Watts.
    fn curve_w5(&self) -> Option<i16> {
        Some(self.curve_w5)
    }

    /// W5
    ///
    /// Point 5 Watts.
    fn set_curve_w5(&mut self, value: i16) {
        self.curve_w5 = value;
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

    /// W6
    ///
    /// Point 6 Watts.
    fn curve_w6(&self) -> Option<i16> {
        Some(self.curve_w6)
    }

    /// W6
    ///
    /// Point 6 Watts.
    fn set_curve_w6(&mut self, value: i16) {
        self.curve_w6 = value;
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

    /// W7
    ///
    /// Point 7 Watts.
    fn curve_w7(&self) -> Option<i16> {
        Some(self.curve_w7)
    }

    /// W7
    ///
    /// Point 7 Watts.
    fn set_curve_w7(&mut self, value: i16) {
        self.curve_w7 = value;
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

    /// W8
    ///
    /// Point 8 Watts.
    fn curve_w8(&self) -> Option<i16> {
        Some(self.curve_w8)
    }

    /// W8
    ///
    /// Point 8 Watts.
    fn set_curve_w8(&mut self, value: i16) {
        self.curve_w8 = value;
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

    /// W9
    ///
    /// Point 9 Watts.
    fn curve_w9(&self) -> Option<i16> {
        Some(self.curve_w9)
    }

    /// W9
    ///
    /// Point 9 Watts.
    fn set_curve_w9(&mut self, value: i16) {
        self.curve_w9 = value;
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

    /// W10
    ///
    /// Point 10 Watts.
    fn curve_w10(&self) -> Option<i16> {
        Some(self.curve_w10)
    }

    /// W10
    ///
    /// Point 10 Watts.
    fn set_curve_w10(&mut self, value: i16) {
        self.curve_w10 = value;
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

    /// W11
    ///
    /// Point 11 Watts.
    fn curve_w11(&self) -> Option<i16> {
        Some(self.curve_w11)
    }

    /// W11
    ///
    /// Point 11 Watts.
    fn set_curve_w11(&mut self, value: i16) {
        self.curve_w11 = value;
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

    /// W12
    ///
    /// Point 12 Watts.
    fn curve_w12(&self) -> Option<i16> {
        Some(self.curve_w12)
    }

    /// W12
    ///
    /// Point 12 Watts.
    fn set_curve_w12(&mut self, value: i16) {
        self.curve_w12 = value;
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

    /// W13
    ///
    /// Point 13 Watts.
    fn curve_w13(&self) -> Option<i16> {
        Some(self.curve_w13)
    }

    /// W13
    ///
    /// Point 13 Watts.
    fn set_curve_w13(&mut self, value: i16) {
        self.curve_w13 = value;
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

    /// W14
    ///
    /// Point 14 Watts.
    fn curve_w14(&self) -> Option<i16> {
        Some(self.curve_w14)
    }

    /// W14
    ///
    /// Point 14 Watts.
    fn set_curve_w14(&mut self, value: i16) {
        self.curve_w14 = value;
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

    /// W15
    ///
    /// Point 15 Watts.
    fn curve_w15(&self) -> Option<i16> {
        Some(self.curve_w15)
    }

    /// W15
    ///
    /// Point 15 Watts.
    fn set_curve_w15(&mut self, value: i16) {
        self.curve_w15 = value;
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

    /// W16
    ///
    /// Point 16 Watts.
    fn curve_w16(&self) -> Option<i16> {
        Some(self.curve_w16)
    }

    /// W16
    ///
    /// Point 16 Watts.
    fn set_curve_w16(&mut self, value: i16) {
        self.curve_w16 = value;
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

    /// W17
    ///
    /// Point 17 Watts.
    fn curve_w17(&self) -> Option<i16> {
        Some(self.curve_w17)
    }

    /// W17
    ///
    /// Point 17 Watts.
    fn set_curve_w17(&mut self, value: i16) {
        self.curve_w17 = value;
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

    /// W18
    ///
    /// Point 18 Watts.
    fn curve_w18(&self) -> Option<i16> {
        Some(self.curve_w18)
    }

    /// W18
    ///
    /// Point 18 Watts.
    fn set_curve_w18(&mut self, value: i16) {
        self.curve_w18 = value;
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

    /// W19
    ///
    /// Point 19 Watts.
    fn curve_w19(&self) -> Option<i16> {
        Some(self.curve_w19)
    }

    /// W19
    ///
    /// Point 19 Watts.
    fn set_curve_w19(&mut self, value: i16) {
        self.curve_w19 = value;
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

    /// W20
    ///
    /// Point 20 Watts.
    fn curve_w20(&self) -> Option<i16> {
        Some(self.curve_w20)
    }

    /// W20
    ///
    /// Point 20 Watts.
    fn set_curve_w20(&mut self, value: i16) {
        self.curve_w20 = value;
    }

    /// CrvNam
    ///
    /// Optional description for curve.
    fn curve_crv_nam(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.curve_crv_nam.as_ptr()) })
    }

    /// CrvNam
    ///
    /// Optional description for curve.
    fn set_curve_crv_nam(&mut self, value: &CStr) {
        for (dest, src) in self
            .curve_crv_nam
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// RmpPt1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_pt1_tms(&self) -> Option<u16> {
        Some(self.curve_rmp_pt1_tms)
    }

    /// RmpPt1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_pt1_tms(&mut self, value: u16) {
        self.curve_rmp_pt1_tms = value;
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the watt value may be reduced in response to changes in the voltage value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        Some(self.curve_rmp_dec_tmm)
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the watt value may be reduced in response to changes in the voltage value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {
        self.curve_rmp_dec_tmm = value;
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the watt value may be increased in response to changes in the voltage value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        Some(self.curve_rmp_inc_tmm)
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the watt value may be increased in response to changes in the voltage value.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {
        self.curve_rmp_inc_tmm = value;
    }

    /// ReadOnly
    ///
    /// Curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly {
        self.curve_read_only
    }
}
