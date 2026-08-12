use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 70;

static POINTS: [PointDetails<()>; 63] = [
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
        point: |()| Point::HzSf,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::WSf,
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
        point: |()| Point::CurveHz1,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::CurveW1,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::CurveHz2,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::CurveW2,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::CurveHz3,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::CurveW3,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::CurveHz4,
        size: 1,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::CurveW4,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::CurveHz5,
        size: 1,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::CurveW5,
        size: 1,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::CurveHz6,
        size: 1,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::CurveW6,
        size: 1,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::CurveHz7,
        size: 1,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::CurveW7,
        size: 1,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::CurveHz8,
        size: 1,
        start_address: 27,
    },
    PointDetails {
        point: |()| Point::CurveW8,
        size: 1,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::CurveHz9,
        size: 1,
        start_address: 29,
    },
    PointDetails {
        point: |()| Point::CurveW9,
        size: 1,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::CurveHz10,
        size: 1,
        start_address: 31,
    },
    PointDetails {
        point: |()| Point::CurveW10,
        size: 1,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::CurveHz11,
        size: 1,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::CurveW11,
        size: 1,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::CurveHz12,
        size: 1,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::CurveW12,
        size: 1,
        start_address: 36,
    },
    PointDetails {
        point: |()| Point::CurveHz13,
        size: 1,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::CurveW13,
        size: 1,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::CurveHz14,
        size: 1,
        start_address: 39,
    },
    PointDetails {
        point: |()| Point::CurveW14,
        size: 1,
        start_address: 40,
    },
    PointDetails {
        point: |()| Point::CurveHz15,
        size: 1,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::CurveW15,
        size: 1,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::CurveHz16,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::CurveW16,
        size: 1,
        start_address: 44,
    },
    PointDetails {
        point: |()| Point::CurveHz17,
        size: 1,
        start_address: 45,
    },
    PointDetails {
        point: |()| Point::CurveW17,
        size: 1,
        start_address: 46,
    },
    PointDetails {
        point: |()| Point::CurveHz18,
        size: 1,
        start_address: 47,
    },
    PointDetails {
        point: |()| Point::CurveW18,
        size: 1,
        start_address: 48,
    },
    PointDetails {
        point: |()| Point::CurveHz19,
        size: 1,
        start_address: 49,
    },
    PointDetails {
        point: |()| Point::CurveW19,
        size: 1,
        start_address: 50,
    },
    PointDetails {
        point: |()| Point::CurveHz20,
        size: 1,
        start_address: 51,
    },
    PointDetails {
        point: |()| Point::CurveW20,
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
        point: |()| Point::CurveRmpRsUp,
        size: 1,
        start_address: 64,
    },
    PointDetails {
        point: |()| Point::CurveSnptW,
        size: 1,
        start_address: 65,
    },
    PointDetails {
        point: |()| Point::CurveWRef,
        size: 1,
        start_address: 66,
    },
    PointDetails {
        point: |()| Point::CurveWRefStrHz,
        size: 1,
        start_address: 67,
    },
    PointDetails {
        point: |()| Point::CurveWRefStopHz,
        size: 1,
        start_address: 68,
    },
    PointDetails {
        point: |()| Point::CurveReadOnly,
        size: 1,
        start_address: 69,
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
    HzSf,
    WSf,
    RmpIncDecSf,
    CurveActPt,
    CurveHz1,
    CurveW1,
    CurveHz2,
    CurveW2,
    CurveHz3,
    CurveW3,
    CurveHz4,
    CurveW4,
    CurveHz5,
    CurveW5,
    CurveHz6,
    CurveW6,
    CurveHz7,
    CurveW7,
    CurveHz8,
    CurveW8,
    CurveHz9,
    CurveW9,
    CurveHz10,
    CurveW10,
    CurveHz11,
    CurveW11,
    CurveHz12,
    CurveW12,
    CurveHz13,
    CurveW13,
    CurveHz14,
    CurveW14,
    CurveHz15,
    CurveW15,
    CurveHz16,
    CurveW16,
    CurveHz17,
    CurveW17,
    CurveHz18,
    CurveW18,
    CurveHz19,
    CurveW19,
    CurveHz20,
    CurveW20,
    CurveCrvNam,
    CurveRmpPt1Tms,
    CurveRmpDecTmm,
    CurveRmpIncTmm,
    CurveRmpRsUp,
    CurveSnptW,
    CurveWRef,
    CurveWRefStrHz,
    CurveWRefStopHz,
    CurveReadOnly,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    70
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
            buffer::write_u16(134, buffer);
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
        Point::HzSf => {
            buffer::write_u16(model.hz_sf(), buffer);
        }
        Point::WSf => {
            buffer::write_u16(model.w_sf(), buffer);
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
        Point::CurveHz1 => {
            buffer::write_u16(model.curve_hz1(), buffer);
        }
        Point::CurveW1 => {
            buffer::write_i16(model.curve_w1(), buffer);
        }
        Point::CurveHz2 => {
            if let Some(value) = model.curve_hz2() {
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
        Point::CurveHz3 => {
            if let Some(value) = model.curve_hz3() {
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
        Point::CurveHz4 => {
            if let Some(value) = model.curve_hz4() {
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
        Point::CurveHz5 => {
            if let Some(value) = model.curve_hz5() {
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
        Point::CurveHz6 => {
            if let Some(value) = model.curve_hz6() {
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
        Point::CurveHz7 => {
            if let Some(value) = model.curve_hz7() {
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
        Point::CurveHz8 => {
            if let Some(value) = model.curve_hz8() {
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
        Point::CurveHz9 => {
            if let Some(value) = model.curve_hz9() {
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
        Point::CurveHz10 => {
            if let Some(value) = model.curve_hz10() {
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
        Point::CurveHz11 => {
            if let Some(value) = model.curve_hz11() {
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
        Point::CurveHz12 => {
            if let Some(value) = model.curve_hz12() {
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
        Point::CurveHz13 => {
            if let Some(value) = model.curve_hz13() {
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
        Point::CurveHz14 => {
            if let Some(value) = model.curve_hz14() {
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
        Point::CurveHz15 => {
            if let Some(value) = model.curve_hz15() {
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
        Point::CurveHz16 => {
            if let Some(value) = model.curve_hz16() {
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
        Point::CurveHz17 => {
            if let Some(value) = model.curve_hz17() {
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
        Point::CurveHz18 => {
            if let Some(value) = model.curve_hz18() {
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
        Point::CurveHz19 => {
            if let Some(value) = model.curve_hz19() {
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
        Point::CurveHz20 => {
            if let Some(value) = model.curve_hz20() {
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
        Point::CurveRmpRsUp => {
            if let Some(value) = model.curve_rmp_rs_up() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveSnptW => {
            buffer::write_u16(model.curve_snpt_w(), buffer);
        }
        Point::CurveWRef => {
            if let Some(value) = model.curve_w_ref() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveWRefStrHz => {
            if let Some(value) = model.curve_w_ref_str_hz() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::CurveWRefStopHz => {
            if let Some(value) = model.curve_w_ref_stop_hz() {
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
    /// Is curve-based Frequency-Watt control active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for freq-watt change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for freq-watt change.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
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
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16;

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16;

    /// W_SF
    ///
    /// Scale factor for percent WRef.
    fn w_sf(&self) -> u16;

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

    /// Hz1
    ///
    /// Point 1 Hertz.
    fn curve_hz1(&self) -> u16;

    /// Hz1
    ///
    /// Point 1 Hertz.
    fn set_curve_hz1(&mut self, value: u16);

    /// W1
    ///
    /// Point 1 Watts.
    fn curve_w1(&self) -> i16;

    /// W1
    ///
    /// Point 1 Watts.
    fn set_curve_w1(&mut self, value: i16);

    /// Hz2
    ///
    /// Point 2 Hertz.
    fn curve_hz2(&self) -> Option<u16> {
        None
    }

    /// Hz2
    ///
    /// Point 2 Hertz.
    fn set_curve_hz2(&mut self, value: u16) {}

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

    /// Hz3
    ///
    /// Point 3 Hertz.
    fn curve_hz3(&self) -> Option<u16> {
        None
    }

    /// Hz3
    ///
    /// Point 3 Hertz.
    fn set_curve_hz3(&mut self, value: u16) {}

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

    /// Hz4
    ///
    /// Point 4 Hertz.
    fn curve_hz4(&self) -> Option<u16> {
        None
    }

    /// Hz4
    ///
    /// Point 4 Hertz.
    fn set_curve_hz4(&mut self, value: u16) {}

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

    /// Hz5
    ///
    /// Point 5 Hertz.
    fn curve_hz5(&self) -> Option<u16> {
        None
    }

    /// Hz5
    ///
    /// Point 5 Hertz.
    fn set_curve_hz5(&mut self, value: u16) {}

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

    /// Hz6
    ///
    /// Point 6 Hertz.
    fn curve_hz6(&self) -> Option<u16> {
        None
    }

    /// Hz6
    ///
    /// Point 6 Hertz.
    fn set_curve_hz6(&mut self, value: u16) {}

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

    /// Hz7
    ///
    /// Point 7 Hertz.
    fn curve_hz7(&self) -> Option<u16> {
        None
    }

    /// Hz7
    ///
    /// Point 7 Hertz.
    fn set_curve_hz7(&mut self, value: u16) {}

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

    /// Hz8
    ///
    /// Point 8 Hertz.
    fn curve_hz8(&self) -> Option<u16> {
        None
    }

    /// Hz8
    ///
    /// Point 8 Hertz.
    fn set_curve_hz8(&mut self, value: u16) {}

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

    /// Hz9
    ///
    /// Point 9 Hertz.
    fn curve_hz9(&self) -> Option<u16> {
        None
    }

    /// Hz9
    ///
    /// Point 9 Hertz.
    fn set_curve_hz9(&mut self, value: u16) {}

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

    /// Hz10
    ///
    /// Point 10 Hertz.
    fn curve_hz10(&self) -> Option<u16> {
        None
    }

    /// Hz10
    ///
    /// Point 10 Hertz.
    fn set_curve_hz10(&mut self, value: u16) {}

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

    /// Hz11
    ///
    /// Point 11 Hertz.
    fn curve_hz11(&self) -> Option<u16> {
        None
    }

    /// Hz11
    ///
    /// Point 11 Hertz.
    fn set_curve_hz11(&mut self, value: u16) {}

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

    /// Hz12
    ///
    /// Point 12 Hertz.
    fn curve_hz12(&self) -> Option<u16> {
        None
    }

    /// Hz12
    ///
    /// Point 12 Hertz.
    fn set_curve_hz12(&mut self, value: u16) {}

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

    /// Hz13
    ///
    /// Point 13 Hertz.
    fn curve_hz13(&self) -> Option<u16> {
        None
    }

    /// Hz13
    ///
    /// Point 13 Hertz.
    fn set_curve_hz13(&mut self, value: u16) {}

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

    /// Hz14
    ///
    /// Point 14 Hertz.
    fn curve_hz14(&self) -> Option<u16> {
        None
    }

    /// Hz14
    ///
    /// Point 14 Hertz.
    fn set_curve_hz14(&mut self, value: u16) {}

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

    /// Hz15
    ///
    /// Point 15 Hertz.
    fn curve_hz15(&self) -> Option<u16> {
        None
    }

    /// Hz15
    ///
    /// Point 15 Hertz.
    fn set_curve_hz15(&mut self, value: u16) {}

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

    /// Hz16
    ///
    /// Point 16 Hertz.
    fn curve_hz16(&self) -> Option<u16> {
        None
    }

    /// Hz16
    ///
    /// Point 16 Hertz.
    fn set_curve_hz16(&mut self, value: u16) {}

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

    /// Hz17
    ///
    /// Point 17 Hertz.
    fn curve_hz17(&self) -> Option<u16> {
        None
    }

    /// Hz17
    ///
    /// Point 17 Hertz.
    fn set_curve_hz17(&mut self, value: u16) {}

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

    /// Hz18
    ///
    /// Point 18 Hertz.
    fn curve_hz18(&self) -> Option<u16> {
        None
    }

    /// Hz18
    ///
    /// Point 18 Hertz.
    fn set_curve_hz18(&mut self, value: u16) {}

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

    /// Hz19
    ///
    /// Point 19 Hertz.
    fn curve_hz19(&self) -> Option<u16> {
        None
    }

    /// Hz19
    ///
    /// Point 19 Hertz.
    fn set_curve_hz19(&mut self, value: u16) {}

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

    /// Hz20
    ///
    /// Point 20 Hertz.
    fn curve_hz20(&self) -> Option<u16> {
        None
    }

    /// Hz20
    ///
    /// Point 20 Hertz.
    fn set_curve_hz20(&mut self, value: u16) {}

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
    /// Optional description for curve. (Max 16 chars)
    fn curve_crv_nam(&self) -> Option<&CStr> {
        None
    }

    /// CrvNam
    ///
    /// Optional description for curve. (Max 16 chars)
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
    /// The maximum rate at which the power value may be reduced in response to changes in the frequency value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        None
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power value may be reduced in response to changes in the frequency value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {}

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power value may be increased in response to changes in the frequency value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        None
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power value may be increased in response to changes in the frequency value.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {}

    /// RmpRsUp
    ///
    /// The maximum rate at which the power may be increased after releasing the frozen value of snap shot function.
    fn curve_rmp_rs_up(&self) -> Option<u16> {
        None
    }

    /// RmpRsUp
    ///
    /// The maximum rate at which the power may be increased after releasing the frozen value of snap shot function.
    fn set_curve_rmp_rs_up(&mut self, value: u16) {}

    /// SnptW
    ///
    /// 1=enable snapshot/capture mode
    fn curve_snpt_w(&self) -> u16;

    /// SnptW
    ///
    /// 1=enable snapshot/capture mode
    fn set_curve_snpt_w(&mut self, value: u16);

    /// WRef
    ///
    /// Reference active power (default = WMax).
    fn curve_w_ref(&self) -> Option<u16> {
        None
    }

    /// WRef
    ///
    /// Reference active power (default = WMax).
    fn set_curve_w_ref(&mut self, value: u16) {}

    /// WRefStrHz
    ///
    /// Frequency deviation from nominal frequency at the time of the snapshot to start constraining power output.
    fn curve_w_ref_str_hz(&self) -> Option<u16> {
        None
    }

    /// WRefStrHz
    ///
    /// Frequency deviation from nominal frequency at the time of the snapshot to start constraining power output.
    fn set_curve_w_ref_str_hz(&mut self, value: u16) {}

    /// WRefStopHz
    ///
    /// Frequency deviation from nominal frequency at which to release the power output.
    fn curve_w_ref_stop_hz(&self) -> Option<u16> {
        None
    }

    /// WRefStopHz
    ///
    /// Frequency deviation from nominal frequency at which to release the power output.
    fn set_curve_w_ref_stop_hz(&mut self, value: u16) {}

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
pub struct Model134CallbackAdapter {
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
    hz_sf_callback: extern "C" fn(*const c_void) -> u16,
    w_sf_callback: extern "C" fn(*const c_void) -> u16,
    rmp_inc_dec_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    curve_act_pt_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_act_pt_callback: extern "C" fn(u16, *mut c_void),
    curve_hz1_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_hz1_callback: extern "C" fn(u16, *mut c_void),
    curve_w1_callback: extern "C" fn(*const c_void) -> i16,
    set_curve_w1_callback: extern "C" fn(i16, *mut c_void),
    curve_hz2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz2_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w2_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz3_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w3_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz4_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w4_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w4_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz5_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w5_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w5_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz6_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz6_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w6_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w6_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz7_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz7_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w7_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w7_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz8_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz8_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w8_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w8_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz9_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz9_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w9_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w9_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz10_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz10_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w10_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w10_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz11_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz11_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w11_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w11_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz12_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz12_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w12_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w12_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz13_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz13_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w13_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w13_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz14_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz14_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w14_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w14_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz15_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz15_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w15_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w15_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz16_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w16_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w16_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz17_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz17_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w17_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w17_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz18_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz18_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w18_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w18_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz19_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz19_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w19_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_curve_w19_callback: Option<extern "C" fn(i16, *mut c_void)>,
    curve_hz20_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz20_callback: Option<extern "C" fn(u16, *mut c_void)>,
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
    curve_rmp_rs_up_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_rmp_rs_up_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_snpt_w_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_snpt_w_callback: extern "C" fn(u16, *mut c_void),
    curve_w_ref_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_w_ref_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w_ref_str_hz_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_w_ref_str_hz_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_w_ref_stop_hz_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_w_ref_stop_hz_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_read_only_callback: extern "C" fn(*const c_void) -> ReadOnly,
}

impl ModelAdapter for Model134CallbackAdapter {
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
    /// Is curve-based Frequency-Watt control active.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// WinTms
    ///
    /// Time window for freq-watt change.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WinTms
    ///
    /// Time window for freq-watt change.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        self.rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
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
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16 {
        (self.n_pt_callback)(self.context)
    }

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16 {
        (self.hz_sf_callback)(self.context)
    }

    /// W_SF
    ///
    /// Scale factor for percent WRef.
    fn w_sf(&self) -> u16 {
        (self.w_sf_callback)(self.context)
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

    /// Hz1
    ///
    /// Point 1 Hertz.
    fn curve_hz1(&self) -> u16 {
        (self.curve_hz1_callback)(self.context)
    }

    /// Hz1
    ///
    /// Point 1 Hertz.
    fn set_curve_hz1(&mut self, value: u16) {
        (self.set_curve_hz1_callback)(value, self.context);
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

    /// Hz2
    ///
    /// Point 2 Hertz.
    fn curve_hz2(&self) -> Option<u16> {
        self.curve_hz2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz2
    ///
    /// Point 2 Hertz.
    fn set_curve_hz2(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz2_callback {
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

    /// Hz3
    ///
    /// Point 3 Hertz.
    fn curve_hz3(&self) -> Option<u16> {
        self.curve_hz3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz3
    ///
    /// Point 3 Hertz.
    fn set_curve_hz3(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz3_callback {
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

    /// Hz4
    ///
    /// Point 4 Hertz.
    fn curve_hz4(&self) -> Option<u16> {
        self.curve_hz4_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz4
    ///
    /// Point 4 Hertz.
    fn set_curve_hz4(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz4_callback {
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

    /// Hz5
    ///
    /// Point 5 Hertz.
    fn curve_hz5(&self) -> Option<u16> {
        self.curve_hz5_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz5
    ///
    /// Point 5 Hertz.
    fn set_curve_hz5(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz5_callback {
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

    /// Hz6
    ///
    /// Point 6 Hertz.
    fn curve_hz6(&self) -> Option<u16> {
        self.curve_hz6_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz6
    ///
    /// Point 6 Hertz.
    fn set_curve_hz6(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz6_callback {
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

    /// Hz7
    ///
    /// Point 7 Hertz.
    fn curve_hz7(&self) -> Option<u16> {
        self.curve_hz7_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz7
    ///
    /// Point 7 Hertz.
    fn set_curve_hz7(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz7_callback {
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

    /// Hz8
    ///
    /// Point 8 Hertz.
    fn curve_hz8(&self) -> Option<u16> {
        self.curve_hz8_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz8
    ///
    /// Point 8 Hertz.
    fn set_curve_hz8(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz8_callback {
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

    /// Hz9
    ///
    /// Point 9 Hertz.
    fn curve_hz9(&self) -> Option<u16> {
        self.curve_hz9_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz9
    ///
    /// Point 9 Hertz.
    fn set_curve_hz9(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz9_callback {
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

    /// Hz10
    ///
    /// Point 10 Hertz.
    fn curve_hz10(&self) -> Option<u16> {
        self.curve_hz10_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz10
    ///
    /// Point 10 Hertz.
    fn set_curve_hz10(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz10_callback {
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

    /// Hz11
    ///
    /// Point 11 Hertz.
    fn curve_hz11(&self) -> Option<u16> {
        self.curve_hz11_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz11
    ///
    /// Point 11 Hertz.
    fn set_curve_hz11(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz11_callback {
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

    /// Hz12
    ///
    /// Point 12 Hertz.
    fn curve_hz12(&self) -> Option<u16> {
        self.curve_hz12_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz12
    ///
    /// Point 12 Hertz.
    fn set_curve_hz12(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz12_callback {
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

    /// Hz13
    ///
    /// Point 13 Hertz.
    fn curve_hz13(&self) -> Option<u16> {
        self.curve_hz13_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz13
    ///
    /// Point 13 Hertz.
    fn set_curve_hz13(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz13_callback {
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

    /// Hz14
    ///
    /// Point 14 Hertz.
    fn curve_hz14(&self) -> Option<u16> {
        self.curve_hz14_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz14
    ///
    /// Point 14 Hertz.
    fn set_curve_hz14(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz14_callback {
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

    /// Hz15
    ///
    /// Point 15 Hertz.
    fn curve_hz15(&self) -> Option<u16> {
        self.curve_hz15_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz15
    ///
    /// Point 15 Hertz.
    fn set_curve_hz15(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz15_callback {
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

    /// Hz16
    ///
    /// Point 16 Hertz.
    fn curve_hz16(&self) -> Option<u16> {
        self.curve_hz16_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz16
    ///
    /// Point 16 Hertz.
    fn set_curve_hz16(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz16_callback {
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

    /// Hz17
    ///
    /// Point 17 Hertz.
    fn curve_hz17(&self) -> Option<u16> {
        self.curve_hz17_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz17
    ///
    /// Point 17 Hertz.
    fn set_curve_hz17(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz17_callback {
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

    /// Hz18
    ///
    /// Point 18 Hertz.
    fn curve_hz18(&self) -> Option<u16> {
        self.curve_hz18_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz18
    ///
    /// Point 18 Hertz.
    fn set_curve_hz18(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz18_callback {
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

    /// Hz19
    ///
    /// Point 19 Hertz.
    fn curve_hz19(&self) -> Option<u16> {
        self.curve_hz19_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz19
    ///
    /// Point 19 Hertz.
    fn set_curve_hz19(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz19_callback {
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

    /// Hz20
    ///
    /// Point 20 Hertz.
    fn curve_hz20(&self) -> Option<u16> {
        self.curve_hz20_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz20
    ///
    /// Point 20 Hertz.
    fn set_curve_hz20(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz20_callback {
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
    /// The maximum rate at which the power value may be reduced in response to changes in the frequency value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        self.curve_rmp_dec_tmm_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power value may be reduced in response to changes in the frequency value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_dec_tmm_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power value may be increased in response to changes in the frequency value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        self.curve_rmp_inc_tmm_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power value may be increased in response to changes in the frequency value.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_inc_tmm_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpRsUp
    ///
    /// The maximum rate at which the power may be increased after releasing the frozen value of snap shot function.
    fn curve_rmp_rs_up(&self) -> Option<u16> {
        self.curve_rmp_rs_up_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpRsUp
    ///
    /// The maximum rate at which the power may be increased after releasing the frozen value of snap shot function.
    fn set_curve_rmp_rs_up(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_rmp_rs_up_callback {
            (callback)(value, self.context);
        };
    }

    /// SnptW
    ///
    /// 1=enable snapshot/capture mode
    fn curve_snpt_w(&self) -> u16 {
        (self.curve_snpt_w_callback)(self.context)
    }

    /// SnptW
    ///
    /// 1=enable snapshot/capture mode
    fn set_curve_snpt_w(&mut self, value: u16) {
        (self.set_curve_snpt_w_callback)(value, self.context);
    }

    /// WRef
    ///
    /// Reference active power (default = WMax).
    fn curve_w_ref(&self) -> Option<u16> {
        self.curve_w_ref_callback
            .map(|callback| (callback)(self.context))
    }

    /// WRef
    ///
    /// Reference active power (default = WMax).
    fn set_curve_w_ref(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_w_ref_callback {
            (callback)(value, self.context);
        };
    }

    /// WRefStrHz
    ///
    /// Frequency deviation from nominal frequency at the time of the snapshot to start constraining power output.
    fn curve_w_ref_str_hz(&self) -> Option<u16> {
        self.curve_w_ref_str_hz_callback
            .map(|callback| (callback)(self.context))
    }

    /// WRefStrHz
    ///
    /// Frequency deviation from nominal frequency at the time of the snapshot to start constraining power output.
    fn set_curve_w_ref_str_hz(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_w_ref_str_hz_callback {
            (callback)(value, self.context);
        };
    }

    /// WRefStopHz
    ///
    /// Frequency deviation from nominal frequency at which to release the power output.
    fn curve_w_ref_stop_hz(&self) -> Option<u16> {
        self.curve_w_ref_stop_hz_callback
            .map(|callback| (callback)(self.context))
    }

    /// WRefStopHz
    ///
    /// Frequency deviation from nominal frequency at which to release the power output.
    fn set_curve_w_ref_stop_hz(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_w_ref_stop_hz_callback {
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
pub struct Model134StatefulAdapter {
    act_crv: u16,
    mod_ena: u16,
    win_tms: u16,
    rvrt_tms: u16,
    rmp_tms: u16,
    n_crv: u16,
    n_pt: u16,
    hz_sf: u16,
    w_sf: u16,
    rmp_inc_dec_sf: u16,
    curve_act_pt: u16,
    curve_hz1: u16,
    curve_w1: i16,
    curve_hz2: u16,
    curve_w2: i16,
    curve_hz3: u16,
    curve_w3: i16,
    curve_hz4: u16,
    curve_w4: i16,
    curve_hz5: u16,
    curve_w5: i16,
    curve_hz6: u16,
    curve_w6: i16,
    curve_hz7: u16,
    curve_w7: i16,
    curve_hz8: u16,
    curve_w8: i16,
    curve_hz9: u16,
    curve_w9: i16,
    curve_hz10: u16,
    curve_w10: i16,
    curve_hz11: u16,
    curve_w11: i16,
    curve_hz12: u16,
    curve_w12: i16,
    curve_hz13: u16,
    curve_w13: i16,
    curve_hz14: u16,
    curve_w14: i16,
    curve_hz15: u16,
    curve_w15: i16,
    curve_hz16: u16,
    curve_w16: i16,
    curve_hz17: u16,
    curve_w17: i16,
    curve_hz18: u16,
    curve_w18: i16,
    curve_hz19: u16,
    curve_w19: i16,
    curve_hz20: u16,
    curve_w20: i16,
    curve_crv_nam: [c_char; 16],
    curve_rmp_pt1_tms: u16,
    curve_rmp_dec_tmm: u16,
    curve_rmp_inc_tmm: u16,
    curve_rmp_rs_up: u16,
    curve_snpt_w: u16,
    curve_w_ref: u16,
    curve_w_ref_str_hz: u16,
    curve_w_ref_stop_hz: u16,
    curve_read_only: ReadOnly,
}

impl ModelAdapter for Model134StatefulAdapter {
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
    /// Is curve-based Frequency-Watt control active.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// WinTms
    ///
    /// Time window for freq-watt change.
    fn win_tms(&self) -> Option<u16> {
        Some(self.win_tms)
    }

    /// WinTms
    ///
    /// Time window for freq-watt change.
    fn set_win_tms(&mut self, value: u16) {
        self.win_tms = value;
    }

    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        Some(self.rvrt_tms)
    }

    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
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
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16 {
        self.n_pt
    }

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16 {
        self.hz_sf
    }

    /// W_SF
    ///
    /// Scale factor for percent WRef.
    fn w_sf(&self) -> u16 {
        self.w_sf
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

    /// Hz1
    ///
    /// Point 1 Hertz.
    fn curve_hz1(&self) -> u16 {
        self.curve_hz1
    }

    /// Hz1
    ///
    /// Point 1 Hertz.
    fn set_curve_hz1(&mut self, value: u16) {
        self.curve_hz1 = value;
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

    /// Hz2
    ///
    /// Point 2 Hertz.
    fn curve_hz2(&self) -> Option<u16> {
        Some(self.curve_hz2)
    }

    /// Hz2
    ///
    /// Point 2 Hertz.
    fn set_curve_hz2(&mut self, value: u16) {
        self.curve_hz2 = value;
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

    /// Hz3
    ///
    /// Point 3 Hertz.
    fn curve_hz3(&self) -> Option<u16> {
        Some(self.curve_hz3)
    }

    /// Hz3
    ///
    /// Point 3 Hertz.
    fn set_curve_hz3(&mut self, value: u16) {
        self.curve_hz3 = value;
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

    /// Hz4
    ///
    /// Point 4 Hertz.
    fn curve_hz4(&self) -> Option<u16> {
        Some(self.curve_hz4)
    }

    /// Hz4
    ///
    /// Point 4 Hertz.
    fn set_curve_hz4(&mut self, value: u16) {
        self.curve_hz4 = value;
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

    /// Hz5
    ///
    /// Point 5 Hertz.
    fn curve_hz5(&self) -> Option<u16> {
        Some(self.curve_hz5)
    }

    /// Hz5
    ///
    /// Point 5 Hertz.
    fn set_curve_hz5(&mut self, value: u16) {
        self.curve_hz5 = value;
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

    /// Hz6
    ///
    /// Point 6 Hertz.
    fn curve_hz6(&self) -> Option<u16> {
        Some(self.curve_hz6)
    }

    /// Hz6
    ///
    /// Point 6 Hertz.
    fn set_curve_hz6(&mut self, value: u16) {
        self.curve_hz6 = value;
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

    /// Hz7
    ///
    /// Point 7 Hertz.
    fn curve_hz7(&self) -> Option<u16> {
        Some(self.curve_hz7)
    }

    /// Hz7
    ///
    /// Point 7 Hertz.
    fn set_curve_hz7(&mut self, value: u16) {
        self.curve_hz7 = value;
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

    /// Hz8
    ///
    /// Point 8 Hertz.
    fn curve_hz8(&self) -> Option<u16> {
        Some(self.curve_hz8)
    }

    /// Hz8
    ///
    /// Point 8 Hertz.
    fn set_curve_hz8(&mut self, value: u16) {
        self.curve_hz8 = value;
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

    /// Hz9
    ///
    /// Point 9 Hertz.
    fn curve_hz9(&self) -> Option<u16> {
        Some(self.curve_hz9)
    }

    /// Hz9
    ///
    /// Point 9 Hertz.
    fn set_curve_hz9(&mut self, value: u16) {
        self.curve_hz9 = value;
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

    /// Hz10
    ///
    /// Point 10 Hertz.
    fn curve_hz10(&self) -> Option<u16> {
        Some(self.curve_hz10)
    }

    /// Hz10
    ///
    /// Point 10 Hertz.
    fn set_curve_hz10(&mut self, value: u16) {
        self.curve_hz10 = value;
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

    /// Hz11
    ///
    /// Point 11 Hertz.
    fn curve_hz11(&self) -> Option<u16> {
        Some(self.curve_hz11)
    }

    /// Hz11
    ///
    /// Point 11 Hertz.
    fn set_curve_hz11(&mut self, value: u16) {
        self.curve_hz11 = value;
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

    /// Hz12
    ///
    /// Point 12 Hertz.
    fn curve_hz12(&self) -> Option<u16> {
        Some(self.curve_hz12)
    }

    /// Hz12
    ///
    /// Point 12 Hertz.
    fn set_curve_hz12(&mut self, value: u16) {
        self.curve_hz12 = value;
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

    /// Hz13
    ///
    /// Point 13 Hertz.
    fn curve_hz13(&self) -> Option<u16> {
        Some(self.curve_hz13)
    }

    /// Hz13
    ///
    /// Point 13 Hertz.
    fn set_curve_hz13(&mut self, value: u16) {
        self.curve_hz13 = value;
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

    /// Hz14
    ///
    /// Point 14 Hertz.
    fn curve_hz14(&self) -> Option<u16> {
        Some(self.curve_hz14)
    }

    /// Hz14
    ///
    /// Point 14 Hertz.
    fn set_curve_hz14(&mut self, value: u16) {
        self.curve_hz14 = value;
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

    /// Hz15
    ///
    /// Point 15 Hertz.
    fn curve_hz15(&self) -> Option<u16> {
        Some(self.curve_hz15)
    }

    /// Hz15
    ///
    /// Point 15 Hertz.
    fn set_curve_hz15(&mut self, value: u16) {
        self.curve_hz15 = value;
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

    /// Hz16
    ///
    /// Point 16 Hertz.
    fn curve_hz16(&self) -> Option<u16> {
        Some(self.curve_hz16)
    }

    /// Hz16
    ///
    /// Point 16 Hertz.
    fn set_curve_hz16(&mut self, value: u16) {
        self.curve_hz16 = value;
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

    /// Hz17
    ///
    /// Point 17 Hertz.
    fn curve_hz17(&self) -> Option<u16> {
        Some(self.curve_hz17)
    }

    /// Hz17
    ///
    /// Point 17 Hertz.
    fn set_curve_hz17(&mut self, value: u16) {
        self.curve_hz17 = value;
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

    /// Hz18
    ///
    /// Point 18 Hertz.
    fn curve_hz18(&self) -> Option<u16> {
        Some(self.curve_hz18)
    }

    /// Hz18
    ///
    /// Point 18 Hertz.
    fn set_curve_hz18(&mut self, value: u16) {
        self.curve_hz18 = value;
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

    /// Hz19
    ///
    /// Point 19 Hertz.
    fn curve_hz19(&self) -> Option<u16> {
        Some(self.curve_hz19)
    }

    /// Hz19
    ///
    /// Point 19 Hertz.
    fn set_curve_hz19(&mut self, value: u16) {
        self.curve_hz19 = value;
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

    /// Hz20
    ///
    /// Point 20 Hertz.
    fn curve_hz20(&self) -> Option<u16> {
        Some(self.curve_hz20)
    }

    /// Hz20
    ///
    /// Point 20 Hertz.
    fn set_curve_hz20(&mut self, value: u16) {
        self.curve_hz20 = value;
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
    /// The maximum rate at which the power value may be reduced in response to changes in the frequency value.
    fn curve_rmp_dec_tmm(&self) -> Option<u16> {
        Some(self.curve_rmp_dec_tmm)
    }

    /// RmpDecTmm
    ///
    /// The maximum rate at which the power value may be reduced in response to changes in the frequency value.
    fn set_curve_rmp_dec_tmm(&mut self, value: u16) {
        self.curve_rmp_dec_tmm = value;
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power value may be increased in response to changes in the frequency value.
    fn curve_rmp_inc_tmm(&self) -> Option<u16> {
        Some(self.curve_rmp_inc_tmm)
    }

    /// RmpIncTmm
    ///
    /// The maximum rate at which the power value may be increased in response to changes in the frequency value.
    fn set_curve_rmp_inc_tmm(&mut self, value: u16) {
        self.curve_rmp_inc_tmm = value;
    }

    /// RmpRsUp
    ///
    /// The maximum rate at which the power may be increased after releasing the frozen value of snap shot function.
    fn curve_rmp_rs_up(&self) -> Option<u16> {
        Some(self.curve_rmp_rs_up)
    }

    /// RmpRsUp
    ///
    /// The maximum rate at which the power may be increased after releasing the frozen value of snap shot function.
    fn set_curve_rmp_rs_up(&mut self, value: u16) {
        self.curve_rmp_rs_up = value;
    }

    /// SnptW
    ///
    /// 1=enable snapshot/capture mode
    fn curve_snpt_w(&self) -> u16 {
        self.curve_snpt_w
    }

    /// SnptW
    ///
    /// 1=enable snapshot/capture mode
    fn set_curve_snpt_w(&mut self, value: u16) {
        self.curve_snpt_w = value;
    }

    /// WRef
    ///
    /// Reference active power (default = WMax).
    fn curve_w_ref(&self) -> Option<u16> {
        Some(self.curve_w_ref)
    }

    /// WRef
    ///
    /// Reference active power (default = WMax).
    fn set_curve_w_ref(&mut self, value: u16) {
        self.curve_w_ref = value;
    }

    /// WRefStrHz
    ///
    /// Frequency deviation from nominal frequency at the time of the snapshot to start constraining power output.
    fn curve_w_ref_str_hz(&self) -> Option<u16> {
        Some(self.curve_w_ref_str_hz)
    }

    /// WRefStrHz
    ///
    /// Frequency deviation from nominal frequency at the time of the snapshot to start constraining power output.
    fn set_curve_w_ref_str_hz(&mut self, value: u16) {
        self.curve_w_ref_str_hz = value;
    }

    /// WRefStopHz
    ///
    /// Frequency deviation from nominal frequency at which to release the power output.
    fn curve_w_ref_stop_hz(&self) -> Option<u16> {
        Some(self.curve_w_ref_stop_hz)
    }

    /// WRefStopHz
    ///
    /// Frequency deviation from nominal frequency at which to release the power output.
    fn set_curve_w_ref_stop_hz(&mut self, value: u16) {
        self.curve_w_ref_stop_hz = value;
    }

    /// ReadOnly
    ///
    /// Curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly {
        self.curve_read_only
    }
}
