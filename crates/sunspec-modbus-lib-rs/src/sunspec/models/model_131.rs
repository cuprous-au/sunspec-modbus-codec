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
        point: |()| Point::WSf,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::PfSf,
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
        point: |()| Point::CurveW1,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::CurvePf1,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::CurveW2,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::CurvePf2,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::CurveW3,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::CurvePf3,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::CurveW4,
        size: 1,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::CurvePf4,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::CurveW5,
        size: 1,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::CurvePf5,
        size: 1,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::CurveW6,
        size: 1,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::CurvePf6,
        size: 1,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::CurveW7,
        size: 1,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::CurvePf7,
        size: 1,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::CurveW8,
        size: 1,
        start_address: 27,
    },
    PointDetails {
        point: |()| Point::CurvePf8,
        size: 1,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::CurveW9,
        size: 1,
        start_address: 29,
    },
    PointDetails {
        point: |()| Point::CurvePf9,
        size: 1,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::CurveW10,
        size: 1,
        start_address: 31,
    },
    PointDetails {
        point: |()| Point::CurvePf10,
        size: 1,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::CurveW11,
        size: 1,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::CurvePf11,
        size: 1,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::CurveW12,
        size: 1,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::CurvePf12,
        size: 1,
        start_address: 36,
    },
    PointDetails {
        point: |()| Point::CurveW13,
        size: 1,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::CurvePf13,
        size: 1,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::CurveW14,
        size: 1,
        start_address: 39,
    },
    PointDetails {
        point: |()| Point::CurvePf14,
        size: 1,
        start_address: 40,
    },
    PointDetails {
        point: |()| Point::CurveW15,
        size: 1,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::CurvePf15,
        size: 1,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::CurveW16,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::CurvePf16,
        size: 1,
        start_address: 44,
    },
    PointDetails {
        point: |()| Point::CurveW17,
        size: 1,
        start_address: 45,
    },
    PointDetails {
        point: |()| Point::CurvePf17,
        size: 1,
        start_address: 46,
    },
    PointDetails {
        point: |()| Point::CurveW18,
        size: 1,
        start_address: 47,
    },
    PointDetails {
        point: |()| Point::CurvePf18,
        size: 1,
        start_address: 48,
    },
    PointDetails {
        point: |()| Point::CurveW19,
        size: 1,
        start_address: 49,
    },
    PointDetails {
        point: |()| Point::CurvePf19,
        size: 1,
        start_address: 50,
    },
    PointDetails {
        point: |()| Point::CurveW20,
        size: 1,
        start_address: 51,
    },
    PointDetails {
        point: |()| Point::CurvePf20,
        size: 1,
        start_address: 52,
    },
    PointDetails {
        point: |()| Point::CurveCrvNam,
        size: 8,
        start_address: 53,
    },
    PointDetails {
        point: |()| Point::CurveRmpPt1Tms,
        size: 1,
        start_address: 61,
    },
    PointDetails {
        point: |()| Point::CurveRmpDecTmm,
        size: 1,
        start_address: 62,
    },
    PointDetails {
        point: |()| Point::CurveRmpIncTmm,
        size: 1,
        start_address: 63,
    },
    PointDetails {
        point: |()| Point::CurveReadOnly,
        size: 1,
        start_address: 64,
    },
    PointDetails {
        point: |()| Point::CurvePad,
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
    WSf,
    PfSf,
    RmpIncDecSf,
    CurveActPt,
    CurveW1,
    CurvePf1,
    CurveW2,
    CurvePf2,
    CurveW3,
    CurvePf3,
    CurveW4,
    CurvePf4,
    CurveW5,
    CurvePf5,
    CurveW6,
    CurvePf6,
    CurveW7,
    CurvePf7,
    CurveW8,
    CurvePf8,
    CurveW9,
    CurvePf9,
    CurveW10,
    CurvePf10,
    CurveW11,
    CurvePf11,
    CurveW12,
    CurvePf12,
    CurveW13,
    CurvePf13,
    CurveW14,
    CurvePf14,
    CurveW15,
    CurvePf15,
    CurveW16,
    CurvePf16,
    CurveW17,
    CurvePf17,
    CurveW18,
    CurvePf18,
    CurveW19,
    CurvePf19,
    CurveW20,
    CurvePf20,
    CurveCrvNam,
    CurveRmpPt1Tms,
    CurveRmpDecTmm,
    CurveRmpIncTmm,
    CurveReadOnly,
    CurvePad,
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
            buffer::write_u16(131, buffer);
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
        Point::WSf => {
            buffer::write_u16(model.w_sf(), buffer);
        }
        Point::PfSf => {
            buffer::write_u16(model.pf_sf(), buffer);
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
        Point::CurveW1 => {
            buffer::write_i16(model.curve_w1(), buffer);
        }
        Point::CurvePf1 => {
            buffer::write_i16(model.curve_pf1(), buffer);
        }
        Point::CurveW2 => {
            if let Some(value) = model.curve_w2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf2 => {
            if let Some(value) = model.curve_pf2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW3 => {
            if let Some(value) = model.curve_w3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf3 => {
            if let Some(value) = model.curve_pf3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW4 => {
            if let Some(value) = model.curve_w4() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf4 => {
            if let Some(value) = model.curve_pf4() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW5 => {
            if let Some(value) = model.curve_w5() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf5 => {
            if let Some(value) = model.curve_pf5() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW6 => {
            if let Some(value) = model.curve_w6() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf6 => {
            if let Some(value) = model.curve_pf6() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW7 => {
            if let Some(value) = model.curve_w7() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf7 => {
            if let Some(value) = model.curve_pf7() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW8 => {
            if let Some(value) = model.curve_w8() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf8 => {
            if let Some(value) = model.curve_pf8() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW9 => {
            if let Some(value) = model.curve_w9() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf9 => {
            if let Some(value) = model.curve_pf9() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW10 => {
            if let Some(value) = model.curve_w10() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf10 => {
            if let Some(value) = model.curve_pf10() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW11 => {
            if let Some(value) = model.curve_w11() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf11 => {
            if let Some(value) = model.curve_pf11() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW12 => {
            if let Some(value) = model.curve_w12() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf12 => {
            if let Some(value) = model.curve_pf12() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW13 => {
            if let Some(value) = model.curve_w13() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf13 => {
            if let Some(value) = model.curve_pf13() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW14 => {
            if let Some(value) = model.curve_w14() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf14 => {
            if let Some(value) = model.curve_pf14() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW15 => {
            if let Some(value) = model.curve_w15() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf15 => {
            if let Some(value) = model.curve_pf15() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW16 => {
            if let Some(value) = model.curve_w16() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf16 => {
            if let Some(value) = model.curve_pf16() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW17 => {
            if let Some(value) = model.curve_w17() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf17 => {
            if let Some(value) = model.curve_pf17() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW18 => {
            if let Some(value) = model.curve_w18() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf18 => {
            if let Some(value) = model.curve_pf18() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW19 => {
            if let Some(value) = model.curve_w19() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf19 => {
            if let Some(value) = model.curve_pf19() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveW20 => {
            if let Some(value) = model.curve_w20() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurvePf20 => {
            if let Some(value) = model.curve_pf20() {
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
        Point::CurveRmpPt1Tms => {
            if let Some(value) = model.curve_rmp_pt1_tms() {
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
        Point::CurvePad => {
            buffer::write_u16(0, buffer);
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
    /// Is watt-PF mode active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is watt-PF mode active.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for watt-PF change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for watt-PF change.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
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
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16;

    /// NPt
    ///
    /// Max number of points in array.
    fn n_pt(&self) -> u16;

    /// W_SF
    ///
    /// Scale factor for percent WMax.
    fn w_sf(&self) -> u16;

    /// PF_SF
    ///
    /// Scale factor for PF.
    fn pf_sf(&self) -> u16;

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

    /// W1
    ///
    /// Point 1 Watts.
    fn curve_w1(&self) -> i16;

    /// W1
    ///
    /// Point 1 Watts.
    fn set_curve_w1(&mut self, value: i16);

    /// PF1
    ///
    /// Point 1 PF in EEI notation.
    fn curve_pf1(&self) -> i16;

    /// PF1
    ///
    /// Point 1 PF in EEI notation.
    fn set_curve_pf1(&mut self, value: i16);

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

    /// PF2
    ///
    /// Point 2 PF in EEI notation.
    fn curve_pf2(&self) -> Option<i16> {
        None
    }

    /// PF2
    ///
    /// Point 2 PF in EEI notation.
    fn set_curve_pf2(&mut self, value: i16) {}

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

    /// PF3
    ///
    /// Point 3 PF in EEI notation.
    fn curve_pf3(&self) -> Option<i16> {
        None
    }

    /// PF3
    ///
    /// Point 3 PF in EEI notation.
    fn set_curve_pf3(&mut self, value: i16) {}

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

    /// PF4
    ///
    /// Point 4 PF in EEI notation.
    fn curve_pf4(&self) -> Option<i16> {
        None
    }

    /// PF4
    ///
    /// Point 4 PF in EEI notation.
    fn set_curve_pf4(&mut self, value: i16) {}

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

    /// PF5
    ///
    /// Point 5 PF in EEI notation.
    fn curve_pf5(&self) -> Option<i16> {
        None
    }

    /// PF5
    ///
    /// Point 5 PF in EEI notation.
    fn set_curve_pf5(&mut self, value: i16) {}

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

    /// PF6
    ///
    /// Point 6 PF in EEI notation.
    fn curve_pf6(&self) -> Option<i16> {
        None
    }

    /// PF6
    ///
    /// Point 6 PF in EEI notation.
    fn set_curve_pf6(&mut self, value: i16) {}

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

    /// PF7
    ///
    /// Point 7 PF in EEI notation.
    fn curve_pf7(&self) -> Option<i16> {
        None
    }

    /// PF7
    ///
    /// Point 7 PF in EEI notation.
    fn set_curve_pf7(&mut self, value: i16) {}

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

    /// PF8
    ///
    /// Point 8 PF in EEI notation.
    fn curve_pf8(&self) -> Option<i16> {
        None
    }

    /// PF8
    ///
    /// Point 8 PF in EEI notation.
    fn set_curve_pf8(&mut self, value: i16) {}

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

    /// PF9
    ///
    /// Point 9 PF in EEI notation.
    fn curve_pf9(&self) -> Option<i16> {
        None
    }

    /// PF9
    ///
    /// Point 9 PF in EEI notation.
    fn set_curve_pf9(&mut self, value: i16) {}

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

    /// PF10
    ///
    /// Point 10 PF in EEI notation.
    fn curve_pf10(&self) -> Option<i16> {
        None
    }

    /// PF10
    ///
    /// Point 10 PF in EEI notation.
    fn set_curve_pf10(&mut self, value: i16) {}

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

    /// PF11
    ///
    /// Point 11 PF in EEI notation.
    fn curve_pf11(&self) -> Option<i16> {
        None
    }

    /// PF11
    ///
    /// Point 11 PF in EEI notation.
    fn set_curve_pf11(&mut self, value: i16) {}

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

    /// PF12
    ///
    /// Point 12 PF in EEI notation.
    fn curve_pf12(&self) -> Option<i16> {
        None
    }

    /// PF12
    ///
    /// Point 12 PF in EEI notation.
    fn set_curve_pf12(&mut self, value: i16) {}

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

    /// PF13
    ///
    /// Point 13 PF in EEI notation.
    fn curve_pf13(&self) -> Option<i16> {
        None
    }

    /// PF13
    ///
    /// Point 13 PF in EEI notation.
    fn set_curve_pf13(&mut self, value: i16) {}

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

    /// PF14
    ///
    /// Point 14 PF in EEI notation.
    fn curve_pf14(&self) -> Option<i16> {
        None
    }

    /// PF14
    ///
    /// Point 14 PF in EEI notation.
    fn set_curve_pf14(&mut self, value: i16) {}

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

    /// PF15
    ///
    /// Point 15 PF in EEI notation.
    fn curve_pf15(&self) -> Option<i16> {
        None
    }

    /// PF15
    ///
    /// Point 15 PF in EEI notation.
    fn set_curve_pf15(&mut self, value: i16) {}

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

    /// PF16
    ///
    /// Point 16 PF in EEI notation.
    fn curve_pf16(&self) -> Option<i16> {
        None
    }

    /// PF16
    ///
    /// Point 16 PF in EEI notation.
    fn set_curve_pf16(&mut self, value: i16) {}

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

    /// PF17
    ///
    /// Point 17 PF in EEI notation.
    fn curve_pf17(&self) -> Option<i16> {
        None
    }

    /// PF17
    ///
    /// Point 17 PF in EEI notation.
    fn set_curve_pf17(&mut self, value: i16) {}

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

    /// PF18
    ///
    /// Point 18 PF in EEI notation.
    fn curve_pf18(&self) -> Option<i16> {
        None
    }

    /// PF18
    ///
    /// Point 18 PF in EEI notation.
    fn set_curve_pf18(&mut self, value: i16) {}

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

    /// PF19
    ///
    /// Point 19 PF in EEI notation.
    fn curve_pf19(&self) -> Option<i16> {
        None
    }

    /// PF19
    ///
    /// Point 19 PF in EEI notation.
    fn set_curve_pf19(&mut self, value: i16) {}

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

    /// PF20
    ///
    /// Point 20 PF in EEI notation.
    fn curve_pf20(&self) -> Option<i16> {
        None
    }

    /// PF20
    ///
    /// Point 20 PF in EEI notation.
    fn set_curve_pf20(&mut self, value: i16) {}

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

    /// RmpPT1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_pt1_tms(&self) -> Option<u16> {
        None
    }

    /// RmpPT1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_pt1_tms(&mut self, value: u16) {}

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power factor may be reduced in response to changes in the power value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        None
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power factor may be reduced in response to changes in the power value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {}

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power factor may be increased in response to changes in the power value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        None
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power factor may be increased in response to changes in the power value.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {}

    /// ReadOnly
    ///
    /// Curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ReadOnly {
    Readwrite = 0,
    Readonly = 1,
}

#[repr(C)]
pub struct Model131CallbackAdapter {
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
    w_sf_callback: extern "C" fn(*const c_void) -> u16,
    pf_sf_callback: extern "C" fn(*const c_void) -> u16,
    rmp_inc_dec_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    curve_act_pt_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_act_pt_callback: extern "C" fn(u16, *mut c_void),
    curve_w1_callback: extern "C" fn(*const c_void) -> i16,
    set_curve_w1_callback: extern "C" fn(i16, *mut c_void),
    curve_pf1_callback: extern "C" fn(*const c_void) -> i16,
    set_curve_pf1_callback: extern "C" fn(i16, *mut c_void),
    curve_w2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w2_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf2_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w3_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf3_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w4_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w4_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf4_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf4_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w5_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w5_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf5_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf5_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w6_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w6_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf6_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf6_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w7_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w7_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf7_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf7_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w8_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w8_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf8_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf8_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w9_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w9_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf9_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf9_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w10_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w10_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf10_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf10_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w11_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w11_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf11_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf11_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w12_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w12_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf12_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf12_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w13_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w13_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf13_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf13_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w14_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w14_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf14_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf14_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w15_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w15_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf15_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf15_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w16_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w16_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf16_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf16_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w17_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w17_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf17_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf17_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w18_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w18_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf18_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf18_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w19_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w19_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf19_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf19_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_w20_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w20_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_pf20_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_pf20_callback: Option<extern "C" fn(i16, *mut c_void)>,
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

impl ModelAdapter for Model131CallbackAdapter {
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
    /// Is watt-PF mode active.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Is watt-PF mode active.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// WinTms
    ///
    /// Time window for watt-PF change.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WinTms
    ///
    /// Time window for watt-PF change.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        self.rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
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
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16 {
        (self.n_crv_callback)(self.context)
    }

    /// NPt
    ///
    /// Max number of points in array.
    fn n_pt(&self) -> u16 {
        (self.n_pt_callback)(self.context)
    }

    /// W_SF
    ///
    /// Scale factor for percent WMax.
    fn w_sf(&self) -> u16 {
        (self.w_sf_callback)(self.context)
    }

    /// PF_SF
    ///
    /// Scale factor for PF.
    fn pf_sf(&self) -> u16 {
        (self.pf_sf_callback)(self.context)
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

    /// PF1
    ///
    /// Point 1 PF in EEI notation.
    fn curve_pf1(&self) -> i16 {
        (self.curve_pf1_callback)(self.context)
    }

    /// PF1
    ///
    /// Point 1 PF in EEI notation.
    fn set_curve_pf1(&mut self, value: i16) {
        (self.set_curve_pf1_callback)(value, self.context);
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

    /// PF2
    ///
    /// Point 2 PF in EEI notation.
    fn curve_pf2(&self) -> Option<i16> {
        self.curve_pf2_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF2
    ///
    /// Point 2 PF in EEI notation.
    fn set_curve_pf2(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf2_callback {
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

    /// PF3
    ///
    /// Point 3 PF in EEI notation.
    fn curve_pf3(&self) -> Option<i16> {
        self.curve_pf3_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF3
    ///
    /// Point 3 PF in EEI notation.
    fn set_curve_pf3(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf3_callback {
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

    /// PF4
    ///
    /// Point 4 PF in EEI notation.
    fn curve_pf4(&self) -> Option<i16> {
        self.curve_pf4_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF4
    ///
    /// Point 4 PF in EEI notation.
    fn set_curve_pf4(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf4_callback {
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

    /// PF5
    ///
    /// Point 5 PF in EEI notation.
    fn curve_pf5(&self) -> Option<i16> {
        self.curve_pf5_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF5
    ///
    /// Point 5 PF in EEI notation.
    fn set_curve_pf5(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf5_callback {
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

    /// PF6
    ///
    /// Point 6 PF in EEI notation.
    fn curve_pf6(&self) -> Option<i16> {
        self.curve_pf6_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF6
    ///
    /// Point 6 PF in EEI notation.
    fn set_curve_pf6(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf6_callback {
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

    /// PF7
    ///
    /// Point 7 PF in EEI notation.
    fn curve_pf7(&self) -> Option<i16> {
        self.curve_pf7_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF7
    ///
    /// Point 7 PF in EEI notation.
    fn set_curve_pf7(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf7_callback {
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

    /// PF8
    ///
    /// Point 8 PF in EEI notation.
    fn curve_pf8(&self) -> Option<i16> {
        self.curve_pf8_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF8
    ///
    /// Point 8 PF in EEI notation.
    fn set_curve_pf8(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf8_callback {
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

    /// PF9
    ///
    /// Point 9 PF in EEI notation.
    fn curve_pf9(&self) -> Option<i16> {
        self.curve_pf9_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF9
    ///
    /// Point 9 PF in EEI notation.
    fn set_curve_pf9(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf9_callback {
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

    /// PF10
    ///
    /// Point 10 PF in EEI notation.
    fn curve_pf10(&self) -> Option<i16> {
        self.curve_pf10_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF10
    ///
    /// Point 10 PF in EEI notation.
    fn set_curve_pf10(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf10_callback {
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

    /// PF11
    ///
    /// Point 11 PF in EEI notation.
    fn curve_pf11(&self) -> Option<i16> {
        self.curve_pf11_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF11
    ///
    /// Point 11 PF in EEI notation.
    fn set_curve_pf11(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf11_callback {
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

    /// PF12
    ///
    /// Point 12 PF in EEI notation.
    fn curve_pf12(&self) -> Option<i16> {
        self.curve_pf12_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF12
    ///
    /// Point 12 PF in EEI notation.
    fn set_curve_pf12(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf12_callback {
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

    /// PF13
    ///
    /// Point 13 PF in EEI notation.
    fn curve_pf13(&self) -> Option<i16> {
        self.curve_pf13_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF13
    ///
    /// Point 13 PF in EEI notation.
    fn set_curve_pf13(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf13_callback {
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

    /// PF14
    ///
    /// Point 14 PF in EEI notation.
    fn curve_pf14(&self) -> Option<i16> {
        self.curve_pf14_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF14
    ///
    /// Point 14 PF in EEI notation.
    fn set_curve_pf14(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf14_callback {
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

    /// PF15
    ///
    /// Point 15 PF in EEI notation.
    fn curve_pf15(&self) -> Option<i16> {
        self.curve_pf15_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF15
    ///
    /// Point 15 PF in EEI notation.
    fn set_curve_pf15(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf15_callback {
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

    /// PF16
    ///
    /// Point 16 PF in EEI notation.
    fn curve_pf16(&self) -> Option<i16> {
        self.curve_pf16_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF16
    ///
    /// Point 16 PF in EEI notation.
    fn set_curve_pf16(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf16_callback {
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

    /// PF17
    ///
    /// Point 17 PF in EEI notation.
    fn curve_pf17(&self) -> Option<i16> {
        self.curve_pf17_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF17
    ///
    /// Point 17 PF in EEI notation.
    fn set_curve_pf17(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf17_callback {
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

    /// PF18
    ///
    /// Point 18 PF in EEI notation.
    fn curve_pf18(&self) -> Option<i16> {
        self.curve_pf18_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF18
    ///
    /// Point 18 PF in EEI notation.
    fn set_curve_pf18(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf18_callback {
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

    /// PF19
    ///
    /// Point 19 PF in EEI notation.
    fn curve_pf19(&self) -> Option<i16> {
        self.curve_pf19_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF19
    ///
    /// Point 19 PF in EEI notation.
    fn set_curve_pf19(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf19_callback {
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

    /// PF20
    ///
    /// Point 20 PF in EEI notation.
    fn curve_pf20(&self) -> Option<i16> {
        self.curve_pf20_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF20
    ///
    /// Point 20 PF in EEI notation.
    fn set_curve_pf20(&mut self, value: i16) {
        if let Some(callback) = self.set_curve_pf20_callback {
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

    /// RmpPT1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_pt1_tms(&self) -> Option<u16> {
        self.curve_rmp_pt1_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpPT1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_pt1_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_pt1_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power factor may be reduced in response to changes in the power value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        self.curve_rmp_dec_tmm_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power factor may be reduced in response to changes in the power value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_dec_tmm_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power factor may be increased in response to changes in the power value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        self.curve_rmp_inc_tmm_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power factor may be increased in response to changes in the power value.
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
pub struct Model131StatefulAdapter {
    act_crv: u16,
    mod_ena: u16,
    win_tms: u16,
    rvrt_tms: u16,
    rmp_tms: u16,
    n_crv: u16,
    n_pt: u16,
    w_sf: u16,
    pf_sf: u16,
    rmp_inc_dec_sf: u16,
    curve_act_pt: u16,
    curve_w1: i16,
    curve_pf1: i16,
    curve_w2: i16,
    curve_pf2: i16,
    curve_w3: i16,
    curve_pf3: i16,
    curve_w4: i16,
    curve_pf4: i16,
    curve_w5: i16,
    curve_pf5: i16,
    curve_w6: i16,
    curve_pf6: i16,
    curve_w7: i16,
    curve_pf7: i16,
    curve_w8: i16,
    curve_pf8: i16,
    curve_w9: i16,
    curve_pf9: i16,
    curve_w10: i16,
    curve_pf10: i16,
    curve_w11: i16,
    curve_pf11: i16,
    curve_w12: i16,
    curve_pf12: i16,
    curve_w13: i16,
    curve_pf13: i16,
    curve_w14: i16,
    curve_pf14: i16,
    curve_w15: i16,
    curve_pf15: i16,
    curve_w16: i16,
    curve_pf16: i16,
    curve_w17: i16,
    curve_pf17: i16,
    curve_w18: i16,
    curve_pf18: i16,
    curve_w19: i16,
    curve_pf19: i16,
    curve_w20: i16,
    curve_pf20: i16,
    curve_crv_nam: [c_char; 16],
    curve_rmp_pt1_tms: u16,
    curve_rmp_dec_tmm: u16,
    curve_rmp_inc_tmm: u16,
    curve_read_only: ReadOnly,
}

impl ModelAdapter for Model131StatefulAdapter {
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
    /// Is watt-PF mode active.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Is watt-PF mode active.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// WinTms
    ///
    /// Time window for watt-PF change.
    fn win_tms(&self) -> Option<u16> {
        Some(self.win_tms)
    }

    /// WinTms
    ///
    /// Time window for watt-PF change.
    fn set_win_tms(&mut self, value: u16) {
        self.win_tms = value;
    }

    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        Some(self.rvrt_tms)
    }

    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
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
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16 {
        self.n_crv
    }

    /// NPt
    ///
    /// Max number of points in array.
    fn n_pt(&self) -> u16 {
        self.n_pt
    }

    /// W_SF
    ///
    /// Scale factor for percent WMax.
    fn w_sf(&self) -> u16 {
        self.w_sf
    }

    /// PF_SF
    ///
    /// Scale factor for PF.
    fn pf_sf(&self) -> u16 {
        self.pf_sf
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

    /// PF1
    ///
    /// Point 1 PF in EEI notation.
    fn curve_pf1(&self) -> i16 {
        self.curve_pf1
    }

    /// PF1
    ///
    /// Point 1 PF in EEI notation.
    fn set_curve_pf1(&mut self, value: i16) {
        self.curve_pf1 = value;
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

    /// PF2
    ///
    /// Point 2 PF in EEI notation.
    fn curve_pf2(&self) -> Option<i16> {
        Some(self.curve_pf2)
    }

    /// PF2
    ///
    /// Point 2 PF in EEI notation.
    fn set_curve_pf2(&mut self, value: i16) {
        self.curve_pf2 = value;
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

    /// PF3
    ///
    /// Point 3 PF in EEI notation.
    fn curve_pf3(&self) -> Option<i16> {
        Some(self.curve_pf3)
    }

    /// PF3
    ///
    /// Point 3 PF in EEI notation.
    fn set_curve_pf3(&mut self, value: i16) {
        self.curve_pf3 = value;
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

    /// PF4
    ///
    /// Point 4 PF in EEI notation.
    fn curve_pf4(&self) -> Option<i16> {
        Some(self.curve_pf4)
    }

    /// PF4
    ///
    /// Point 4 PF in EEI notation.
    fn set_curve_pf4(&mut self, value: i16) {
        self.curve_pf4 = value;
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

    /// PF5
    ///
    /// Point 5 PF in EEI notation.
    fn curve_pf5(&self) -> Option<i16> {
        Some(self.curve_pf5)
    }

    /// PF5
    ///
    /// Point 5 PF in EEI notation.
    fn set_curve_pf5(&mut self, value: i16) {
        self.curve_pf5 = value;
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

    /// PF6
    ///
    /// Point 6 PF in EEI notation.
    fn curve_pf6(&self) -> Option<i16> {
        Some(self.curve_pf6)
    }

    /// PF6
    ///
    /// Point 6 PF in EEI notation.
    fn set_curve_pf6(&mut self, value: i16) {
        self.curve_pf6 = value;
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

    /// PF7
    ///
    /// Point 7 PF in EEI notation.
    fn curve_pf7(&self) -> Option<i16> {
        Some(self.curve_pf7)
    }

    /// PF7
    ///
    /// Point 7 PF in EEI notation.
    fn set_curve_pf7(&mut self, value: i16) {
        self.curve_pf7 = value;
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

    /// PF8
    ///
    /// Point 8 PF in EEI notation.
    fn curve_pf8(&self) -> Option<i16> {
        Some(self.curve_pf8)
    }

    /// PF8
    ///
    /// Point 8 PF in EEI notation.
    fn set_curve_pf8(&mut self, value: i16) {
        self.curve_pf8 = value;
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

    /// PF9
    ///
    /// Point 9 PF in EEI notation.
    fn curve_pf9(&self) -> Option<i16> {
        Some(self.curve_pf9)
    }

    /// PF9
    ///
    /// Point 9 PF in EEI notation.
    fn set_curve_pf9(&mut self, value: i16) {
        self.curve_pf9 = value;
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

    /// PF10
    ///
    /// Point 10 PF in EEI notation.
    fn curve_pf10(&self) -> Option<i16> {
        Some(self.curve_pf10)
    }

    /// PF10
    ///
    /// Point 10 PF in EEI notation.
    fn set_curve_pf10(&mut self, value: i16) {
        self.curve_pf10 = value;
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

    /// PF11
    ///
    /// Point 11 PF in EEI notation.
    fn curve_pf11(&self) -> Option<i16> {
        Some(self.curve_pf11)
    }

    /// PF11
    ///
    /// Point 11 PF in EEI notation.
    fn set_curve_pf11(&mut self, value: i16) {
        self.curve_pf11 = value;
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

    /// PF12
    ///
    /// Point 12 PF in EEI notation.
    fn curve_pf12(&self) -> Option<i16> {
        Some(self.curve_pf12)
    }

    /// PF12
    ///
    /// Point 12 PF in EEI notation.
    fn set_curve_pf12(&mut self, value: i16) {
        self.curve_pf12 = value;
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

    /// PF13
    ///
    /// Point 13 PF in EEI notation.
    fn curve_pf13(&self) -> Option<i16> {
        Some(self.curve_pf13)
    }

    /// PF13
    ///
    /// Point 13 PF in EEI notation.
    fn set_curve_pf13(&mut self, value: i16) {
        self.curve_pf13 = value;
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

    /// PF14
    ///
    /// Point 14 PF in EEI notation.
    fn curve_pf14(&self) -> Option<i16> {
        Some(self.curve_pf14)
    }

    /// PF14
    ///
    /// Point 14 PF in EEI notation.
    fn set_curve_pf14(&mut self, value: i16) {
        self.curve_pf14 = value;
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

    /// PF15
    ///
    /// Point 15 PF in EEI notation.
    fn curve_pf15(&self) -> Option<i16> {
        Some(self.curve_pf15)
    }

    /// PF15
    ///
    /// Point 15 PF in EEI notation.
    fn set_curve_pf15(&mut self, value: i16) {
        self.curve_pf15 = value;
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

    /// PF16
    ///
    /// Point 16 PF in EEI notation.
    fn curve_pf16(&self) -> Option<i16> {
        Some(self.curve_pf16)
    }

    /// PF16
    ///
    /// Point 16 PF in EEI notation.
    fn set_curve_pf16(&mut self, value: i16) {
        self.curve_pf16 = value;
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

    /// PF17
    ///
    /// Point 17 PF in EEI notation.
    fn curve_pf17(&self) -> Option<i16> {
        Some(self.curve_pf17)
    }

    /// PF17
    ///
    /// Point 17 PF in EEI notation.
    fn set_curve_pf17(&mut self, value: i16) {
        self.curve_pf17 = value;
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

    /// PF18
    ///
    /// Point 18 PF in EEI notation.
    fn curve_pf18(&self) -> Option<i16> {
        Some(self.curve_pf18)
    }

    /// PF18
    ///
    /// Point 18 PF in EEI notation.
    fn set_curve_pf18(&mut self, value: i16) {
        self.curve_pf18 = value;
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

    /// PF19
    ///
    /// Point 19 PF in EEI notation.
    fn curve_pf19(&self) -> Option<i16> {
        Some(self.curve_pf19)
    }

    /// PF19
    ///
    /// Point 19 PF in EEI notation.
    fn set_curve_pf19(&mut self, value: i16) {
        self.curve_pf19 = value;
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

    /// PF20
    ///
    /// Point 20 PF in EEI notation.
    fn curve_pf20(&self) -> Option<i16> {
        Some(self.curve_pf20)
    }

    /// PF20
    ///
    /// Point 20 PF in EEI notation.
    fn set_curve_pf20(&mut self, value: i16) {
        self.curve_pf20 = value;
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

    /// RmpPT1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn curve_rmp_pt1_tms(&self) -> Option<u16> {
        Some(self.curve_rmp_pt1_tms)
    }

    /// RmpPT1Tms
    ///
    /// The time of the PT1 in seconds (time to accomplish a change of 95%).
    fn set_curve_rmp_pt1_tms(&mut self, value: u16) {
        self.curve_rmp_pt1_tms = value;
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power factor may be reduced in response to changes in the power value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        Some(self.curve_rmp_dec_tmm)
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power factor may be reduced in response to changes in the power value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {
        self.curve_rmp_dec_tmm = value;
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power factor may be increased in response to changes in the power value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        Some(self.curve_rmp_inc_tmm)
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power factor may be increased in response to changes in the power value.
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
