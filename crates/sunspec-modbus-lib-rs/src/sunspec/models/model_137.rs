use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 62;

static POINTS: [PointDetails<()>; 55] = [
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
        point: |()| Point::TmsSf,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::VSf,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::Pad,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::CurveActPt,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::CurveTms1,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::CurveV1,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::CurveTms2,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::CurveV2,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::CurveTms3,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::CurveV3,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::CurveTms4,
        size: 1,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::CurveV4,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::CurveTms5,
        size: 1,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::CurveV5,
        size: 1,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::CurveTms6,
        size: 1,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::CurveV6,
        size: 1,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::CurveTms7,
        size: 1,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::CurveV7,
        size: 1,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::CurveTms8,
        size: 1,
        start_address: 27,
    },
    PointDetails {
        point: |()| Point::CurveV8,
        size: 1,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::CurveTms9,
        size: 1,
        start_address: 29,
    },
    PointDetails {
        point: |()| Point::CurveV9,
        size: 1,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::CurveTms10,
        size: 1,
        start_address: 31,
    },
    PointDetails {
        point: |()| Point::CurveV10,
        size: 1,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::CurveTms11,
        size: 1,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::CurveV11,
        size: 1,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::CurveTms12,
        size: 1,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::CurveV12,
        size: 1,
        start_address: 36,
    },
    PointDetails {
        point: |()| Point::CurveTms13,
        size: 1,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::CurveV13,
        size: 1,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::CurveTms14,
        size: 1,
        start_address: 39,
    },
    PointDetails {
        point: |()| Point::CurveV14,
        size: 1,
        start_address: 40,
    },
    PointDetails {
        point: |()| Point::CurveTms15,
        size: 1,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::CurveV15,
        size: 1,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::CurveTms16,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::CurveV16,
        size: 1,
        start_address: 44,
    },
    PointDetails {
        point: |()| Point::CurveTms17,
        size: 1,
        start_address: 45,
    },
    PointDetails {
        point: |()| Point::CurveV17,
        size: 1,
        start_address: 46,
    },
    PointDetails {
        point: |()| Point::CurveTms18,
        size: 1,
        start_address: 47,
    },
    PointDetails {
        point: |()| Point::CurveV18,
        size: 1,
        start_address: 48,
    },
    PointDetails {
        point: |()| Point::CurveTms19,
        size: 1,
        start_address: 49,
    },
    PointDetails {
        point: |()| Point::CurveV19,
        size: 1,
        start_address: 50,
    },
    PointDetails {
        point: |()| Point::CurveTms20,
        size: 1,
        start_address: 51,
    },
    PointDetails {
        point: |()| Point::CurveV20,
        size: 1,
        start_address: 52,
    },
    PointDetails {
        point: |()| Point::CurveCrvNam,
        size: 8,
        start_address: 53,
    },
    PointDetails {
        point: |()| Point::CurveReadOnly,
        size: 1,
        start_address: 61,
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
    TmsSf,
    VSf,
    Pad,
    CurveActPt,
    CurveTms1,
    CurveV1,
    CurveTms2,
    CurveV2,
    CurveTms3,
    CurveV3,
    CurveTms4,
    CurveV4,
    CurveTms5,
    CurveV5,
    CurveTms6,
    CurveV6,
    CurveTms7,
    CurveV7,
    CurveTms8,
    CurveV8,
    CurveTms9,
    CurveV9,
    CurveTms10,
    CurveV10,
    CurveTms11,
    CurveV11,
    CurveTms12,
    CurveV12,
    CurveTms13,
    CurveV13,
    CurveTms14,
    CurveV14,
    CurveTms15,
    CurveV15,
    CurveTms16,
    CurveV16,
    CurveTms17,
    CurveV17,
    CurveTms18,
    CurveV18,
    CurveTms19,
    CurveV19,
    CurveTms20,
    CurveV20,
    CurveCrvNam,
    CurveReadOnly,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    62
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
            buffer::write_u16(137, buffer);
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
        Point::TmsSf => {
            buffer::write_u16(model.tms_sf(), buffer);
        }
        Point::VSf => {
            buffer::write_u16(model.v_sf(), buffer);
        }
        Point::Pad => {
            buffer::write_u16(0, buffer);
        }
        Point::CurveActPt => {
            buffer::write_u16(model.curve_act_pt(), buffer);
        }
        Point::CurveTms1 => {
            buffer::write_u16(model.curve_tms1(), buffer);
        }
        Point::CurveV1 => {
            buffer::write_u16(model.curve_v1(), buffer);
        }
        Point::CurveTms2 => {
            if let Some(value) = model.curve_tms2() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveV2 => {
            if let Some(value) = model.curve_v2() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveTms3 => {
            if let Some(value) = model.curve_tms3() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms4 => {
            if let Some(value) = model.curve_tms4() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms5 => {
            if let Some(value) = model.curve_tms5() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms6 => {
            if let Some(value) = model.curve_tms6() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms7 => {
            if let Some(value) = model.curve_tms7() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms8 => {
            if let Some(value) = model.curve_tms8() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms9 => {
            if let Some(value) = model.curve_tms9() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms10 => {
            if let Some(value) = model.curve_tms10() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms11 => {
            if let Some(value) = model.curve_tms11() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms12 => {
            if let Some(value) = model.curve_tms12() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms13 => {
            if let Some(value) = model.curve_tms13() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms14 => {
            if let Some(value) = model.curve_tms14() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms15 => {
            if let Some(value) = model.curve_tms15() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms16 => {
            if let Some(value) = model.curve_tms16() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms17 => {
            if let Some(value) = model.curve_tms17() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms18 => {
            if let Some(value) = model.curve_tms18() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms19 => {
            if let Some(value) = model.curve_tms19() {
                buffer::write_u16(value, buffer);
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
        Point::CurveTms20 => {
            if let Some(value) = model.curve_tms20() {
                buffer::write_u16(value, buffer);
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
        Point::CurveCrvNam => {
            if let Some(value) = model.curve_crv_nam() {
                buffer::write_string(value, buffer, offset, limit);
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
    /// LVRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
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

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16;

    /// ActPt
    ///
    /// Number of active points in array.
    fn curve_act_pt(&self) -> u16;

    /// ActPt
    ///
    /// Number of active points in array.
    fn set_curve_act_pt(&mut self, value: u16);

    /// Tms1
    ///
    /// Point 1 must remain connected duration.
    fn curve_tms1(&self) -> u16;

    /// Tms1
    ///
    /// Point 1 must remain connected duration.
    fn set_curve_tms1(&mut self, value: u16);

    /// V1
    ///
    /// Point 1 must remain connected voltage.
    fn curve_v1(&self) -> u16;

    /// V1
    ///
    /// Point 1 must remain connected voltage.
    fn set_curve_v1(&mut self, value: u16);

    /// Tms2
    ///
    /// Point 2 must remain connected duration.
    fn curve_tms2(&self) -> Option<u16> {
        None
    }

    /// Tms2
    ///
    /// Point 2 must remain connected duration.
    fn set_curve_tms2(&mut self, value: u16) {}

    /// V2
    ///
    /// Point 2 must remain connected voltage.
    fn curve_v2(&self) -> Option<u16> {
        None
    }

    /// V2
    ///
    /// Point 2 must remain connected voltage.
    fn set_curve_v2(&mut self, value: u16) {}

    /// Tms3
    ///
    /// Point 3 must remain connected duration.
    fn curve_tms3(&self) -> Option<u16> {
        None
    }

    /// Tms3
    ///
    /// Point 3 must remain connected duration.
    fn set_curve_tms3(&mut self, value: u16) {}

    /// V3
    ///
    /// Point 3 must remain connected voltage.
    fn curve_v3(&self) -> Option<u16> {
        None
    }

    /// V3
    ///
    /// Point 3 must remain connected voltage.
    fn set_curve_v3(&mut self, value: u16) {}

    /// Tms4
    ///
    /// Point 4 must remain connected duration.
    fn curve_tms4(&self) -> Option<u16> {
        None
    }

    /// Tms4
    ///
    /// Point 4 must remain connected duration.
    fn set_curve_tms4(&mut self, value: u16) {}

    /// V4
    ///
    /// Point 4 must remain connected voltage.
    fn curve_v4(&self) -> Option<u16> {
        None
    }

    /// V4
    ///
    /// Point 4 must remain connected voltage.
    fn set_curve_v4(&mut self, value: u16) {}

    /// Tms5
    ///
    /// Point 5 must remain connected duration.
    fn curve_tms5(&self) -> Option<u16> {
        None
    }

    /// Tms5
    ///
    /// Point 5 must remain connected duration.
    fn set_curve_tms5(&mut self, value: u16) {}

    /// V5
    ///
    /// Point 5 must remain connected voltage.
    fn curve_v5(&self) -> Option<u16> {
        None
    }

    /// V5
    ///
    /// Point 5 must remain connected voltage.
    fn set_curve_v5(&mut self, value: u16) {}

    /// Tms6
    ///
    /// Point 6 must remain connected duration.
    fn curve_tms6(&self) -> Option<u16> {
        None
    }

    /// Tms6
    ///
    /// Point 6 must remain connected duration.
    fn set_curve_tms6(&mut self, value: u16) {}

    /// V6
    ///
    /// Point 6 must remain connected voltage.
    fn curve_v6(&self) -> Option<u16> {
        None
    }

    /// V6
    ///
    /// Point 6 must remain connected voltage.
    fn set_curve_v6(&mut self, value: u16) {}

    /// Tms7
    ///
    /// Point 7 must remain connected duration.
    fn curve_tms7(&self) -> Option<u16> {
        None
    }

    /// Tms7
    ///
    /// Point 7 must remain connected duration.
    fn set_curve_tms7(&mut self, value: u16) {}

    /// V7
    ///
    /// Point 7 must remain connected voltage.
    fn curve_v7(&self) -> Option<u16> {
        None
    }

    /// V7
    ///
    /// Point 7 must remain connected voltage.
    fn set_curve_v7(&mut self, value: u16) {}

    /// Tms8
    ///
    /// Point 8 must remain connected duration.
    fn curve_tms8(&self) -> Option<u16> {
        None
    }

    /// Tms8
    ///
    /// Point 8 must remain connected duration.
    fn set_curve_tms8(&mut self, value: u16) {}

    /// V8
    ///
    /// Point 8 must remain connected voltage.
    fn curve_v8(&self) -> Option<u16> {
        None
    }

    /// V8
    ///
    /// Point 8 must remain connected voltage.
    fn set_curve_v8(&mut self, value: u16) {}

    /// Tms9
    ///
    /// Point 9 must remain connected duration.
    fn curve_tms9(&self) -> Option<u16> {
        None
    }

    /// Tms9
    ///
    /// Point 9 must remain connected duration.
    fn set_curve_tms9(&mut self, value: u16) {}

    /// V9
    ///
    /// Point 9 must remain connected voltage.
    fn curve_v9(&self) -> Option<u16> {
        None
    }

    /// V9
    ///
    /// Point 9 must remain connected voltage.
    fn set_curve_v9(&mut self, value: u16) {}

    /// Tms10
    ///
    /// Point 10 must remain connected duration.
    fn curve_tms10(&self) -> Option<u16> {
        None
    }

    /// Tms10
    ///
    /// Point 10 must remain connected duration.
    fn set_curve_tms10(&mut self, value: u16) {}

    /// V10
    ///
    /// Point 10 must remain connected voltage.
    fn curve_v10(&self) -> Option<u16> {
        None
    }

    /// V10
    ///
    /// Point 10 must remain connected voltage.
    fn set_curve_v10(&mut self, value: u16) {}

    /// Tms11
    ///
    /// Point 11 must remain connected duration.
    fn curve_tms11(&self) -> Option<u16> {
        None
    }

    /// Tms11
    ///
    /// Point 11 must remain connected duration.
    fn set_curve_tms11(&mut self, value: u16) {}

    /// V11
    ///
    /// Point 11 must remain connected voltage.
    fn curve_v11(&self) -> Option<u16> {
        None
    }

    /// V11
    ///
    /// Point 11 must remain connected voltage.
    fn set_curve_v11(&mut self, value: u16) {}

    /// Tms12
    ///
    /// Point 12 must remain connected duration.
    fn curve_tms12(&self) -> Option<u16> {
        None
    }

    /// Tms12
    ///
    /// Point 12 must remain connected duration.
    fn set_curve_tms12(&mut self, value: u16) {}

    /// V12
    ///
    /// Point 12 must remain connected voltage.
    fn curve_v12(&self) -> Option<u16> {
        None
    }

    /// V12
    ///
    /// Point 12 must remain connected voltage.
    fn set_curve_v12(&mut self, value: u16) {}

    /// Tms13
    ///
    /// Point 13 must remain connected duration.
    fn curve_tms13(&self) -> Option<u16> {
        None
    }

    /// Tms13
    ///
    /// Point 13 must remain connected duration.
    fn set_curve_tms13(&mut self, value: u16) {}

    /// V13
    ///
    /// Point 13 must remain connected voltage.
    fn curve_v13(&self) -> Option<u16> {
        None
    }

    /// V13
    ///
    /// Point 13 must remain connected voltage.
    fn set_curve_v13(&mut self, value: u16) {}

    /// Tms14
    ///
    /// Point 14 must remain connected duration.
    fn curve_tms14(&self) -> Option<u16> {
        None
    }

    /// Tms14
    ///
    /// Point 14 must remain connected duration.
    fn set_curve_tms14(&mut self, value: u16) {}

    /// V14
    ///
    /// Point 14 must remain connected voltage.
    fn curve_v14(&self) -> Option<u16> {
        None
    }

    /// V14
    ///
    /// Point 14 must remain connected voltage.
    fn set_curve_v14(&mut self, value: u16) {}

    /// Tms15
    ///
    /// Point 15 must remain connected duration.
    fn curve_tms15(&self) -> Option<u16> {
        None
    }

    /// Tms15
    ///
    /// Point 15 must remain connected duration.
    fn set_curve_tms15(&mut self, value: u16) {}

    /// V15
    ///
    /// Point 15 must remain connected voltage.
    fn curve_v15(&self) -> Option<u16> {
        None
    }

    /// V15
    ///
    /// Point 15 must remain connected voltage.
    fn set_curve_v15(&mut self, value: u16) {}

    /// Tms16
    ///
    /// Point 16 must remain connected duration.
    fn curve_tms16(&self) -> Option<u16> {
        None
    }

    /// Tms16
    ///
    /// Point 16 must remain connected duration.
    fn set_curve_tms16(&mut self, value: u16) {}

    /// V16
    ///
    /// Point 16 must remain connected voltage.
    fn curve_v16(&self) -> Option<u16> {
        None
    }

    /// V16
    ///
    /// Point 16 must remain connected voltage.
    fn set_curve_v16(&mut self, value: u16) {}

    /// Tms17
    ///
    /// Point 17 must remain connected duration.
    fn curve_tms17(&self) -> Option<u16> {
        None
    }

    /// Tms17
    ///
    /// Point 17 must remain connected duration.
    fn set_curve_tms17(&mut self, value: u16) {}

    /// V17
    ///
    /// Point 17 must remain connected voltage.
    fn curve_v17(&self) -> Option<u16> {
        None
    }

    /// V17
    ///
    /// Point 17 must remain connected voltage.
    fn set_curve_v17(&mut self, value: u16) {}

    /// Tms18
    ///
    /// Point 18 must remain connected duration.
    fn curve_tms18(&self) -> Option<u16> {
        None
    }

    /// Tms18
    ///
    /// Point 18 must remain connected duration.
    fn set_curve_tms18(&mut self, value: u16) {}

    /// V18
    ///
    /// Point 18 must remain connected voltage.
    fn curve_v18(&self) -> Option<u16> {
        None
    }

    /// V18
    ///
    /// Point 18 must remain connected voltage.
    fn set_curve_v18(&mut self, value: u16) {}

    /// Tms19
    ///
    /// Point 19 must remain connected duration.
    fn curve_tms19(&self) -> Option<u16> {
        None
    }

    /// Tms19
    ///
    /// Point 19 must remain connected duration.
    fn set_curve_tms19(&mut self, value: u16) {}

    /// V19
    ///
    /// Point 19 must remain connected voltage.
    fn curve_v19(&self) -> Option<u16> {
        None
    }

    /// V19
    ///
    /// Point 19 must remain connected voltage.
    fn set_curve_v19(&mut self, value: u16) {}

    /// Tms20
    ///
    /// Point 20 must remain connected duration.
    fn curve_tms20(&self) -> Option<u16> {
        None
    }

    /// Tms20
    ///
    /// Point 20 must remain connected duration.
    fn set_curve_tms20(&mut self, value: u16) {}

    /// V20
    ///
    /// Point 20 must remain connected voltage.
    fn curve_v20(&self) -> Option<u16> {
        None
    }

    /// V20
    ///
    /// Point 20 must remain connected voltage.
    fn set_curve_v20(&mut self, value: u16) {}

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
pub struct Model137CallbackAdapter {
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
    v_sf_callback: extern "C" fn(*const c_void) -> u16,
    curve_act_pt_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_act_pt_callback: extern "C" fn(u16, *mut c_void),
    curve_tms1_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_tms1_callback: extern "C" fn(u16, *mut c_void),
    curve_v1_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_v1_callback: extern "C" fn(u16, *mut c_void),
    curve_tms2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms2_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v2_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms3_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v3_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms4_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v4_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms5_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v5_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms6_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms6_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v6_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v6_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms7_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms7_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v7_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v7_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms8_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms8_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v8_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v8_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms9_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms9_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v9_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v9_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms10_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms10_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v10_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v10_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms11_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms11_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v11_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v11_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms12_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms12_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v12_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v12_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms13_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms13_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v13_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v13_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms14_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms14_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v14_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v14_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms15_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms15_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v15_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v15_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms16_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v16_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms17_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms17_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v17_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v17_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms18_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms18_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v18_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v18_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms19_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms19_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v19_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v19_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms20_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms20_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_v20_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_v20_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_crv_nam_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_curve_crv_nam_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    curve_read_only_callback: extern "C" fn(*const c_void) -> ReadOnly,
}

impl ModelAdapter for Model137CallbackAdapter {
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
    /// LVRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        self.rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        self.rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
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

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)(self.context)
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

    /// Tms1
    ///
    /// Point 1 must remain connected duration.
    fn curve_tms1(&self) -> u16 {
        (self.curve_tms1_callback)(self.context)
    }

    /// Tms1
    ///
    /// Point 1 must remain connected duration.
    fn set_curve_tms1(&mut self, value: u16) {
        (self.set_curve_tms1_callback)(value, self.context);
    }

    /// V1
    ///
    /// Point 1 must remain connected voltage.
    fn curve_v1(&self) -> u16 {
        (self.curve_v1_callback)(self.context)
    }

    /// V1
    ///
    /// Point 1 must remain connected voltage.
    fn set_curve_v1(&mut self, value: u16) {
        (self.set_curve_v1_callback)(value, self.context);
    }

    /// Tms2
    ///
    /// Point 2 must remain connected duration.
    fn curve_tms2(&self) -> Option<u16> {
        self.curve_tms2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms2
    ///
    /// Point 2 must remain connected duration.
    fn set_curve_tms2(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms2_callback {
            (callback)(value, self.context);
        };
    }

    /// V2
    ///
    /// Point 2 must remain connected voltage.
    fn curve_v2(&self) -> Option<u16> {
        self.curve_v2_callback
            .map(|callback| (callback)(self.context))
    }

    /// V2
    ///
    /// Point 2 must remain connected voltage.
    fn set_curve_v2(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v2_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms3
    ///
    /// Point 3 must remain connected duration.
    fn curve_tms3(&self) -> Option<u16> {
        self.curve_tms3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms3
    ///
    /// Point 3 must remain connected duration.
    fn set_curve_tms3(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms3_callback {
            (callback)(value, self.context);
        };
    }

    /// V3
    ///
    /// Point 3 must remain connected voltage.
    fn curve_v3(&self) -> Option<u16> {
        self.curve_v3_callback
            .map(|callback| (callback)(self.context))
    }

    /// V3
    ///
    /// Point 3 must remain connected voltage.
    fn set_curve_v3(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v3_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms4
    ///
    /// Point 4 must remain connected duration.
    fn curve_tms4(&self) -> Option<u16> {
        self.curve_tms4_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms4
    ///
    /// Point 4 must remain connected duration.
    fn set_curve_tms4(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms4_callback {
            (callback)(value, self.context);
        };
    }

    /// V4
    ///
    /// Point 4 must remain connected voltage.
    fn curve_v4(&self) -> Option<u16> {
        self.curve_v4_callback
            .map(|callback| (callback)(self.context))
    }

    /// V4
    ///
    /// Point 4 must remain connected voltage.
    fn set_curve_v4(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v4_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms5
    ///
    /// Point 5 must remain connected duration.
    fn curve_tms5(&self) -> Option<u16> {
        self.curve_tms5_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms5
    ///
    /// Point 5 must remain connected duration.
    fn set_curve_tms5(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms5_callback {
            (callback)(value, self.context);
        };
    }

    /// V5
    ///
    /// Point 5 must remain connected voltage.
    fn curve_v5(&self) -> Option<u16> {
        self.curve_v5_callback
            .map(|callback| (callback)(self.context))
    }

    /// V5
    ///
    /// Point 5 must remain connected voltage.
    fn set_curve_v5(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v5_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms6
    ///
    /// Point 6 must remain connected duration.
    fn curve_tms6(&self) -> Option<u16> {
        self.curve_tms6_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms6
    ///
    /// Point 6 must remain connected duration.
    fn set_curve_tms6(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms6_callback {
            (callback)(value, self.context);
        };
    }

    /// V6
    ///
    /// Point 6 must remain connected voltage.
    fn curve_v6(&self) -> Option<u16> {
        self.curve_v6_callback
            .map(|callback| (callback)(self.context))
    }

    /// V6
    ///
    /// Point 6 must remain connected voltage.
    fn set_curve_v6(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v6_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms7
    ///
    /// Point 7 must remain connected duration.
    fn curve_tms7(&self) -> Option<u16> {
        self.curve_tms7_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms7
    ///
    /// Point 7 must remain connected duration.
    fn set_curve_tms7(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms7_callback {
            (callback)(value, self.context);
        };
    }

    /// V7
    ///
    /// Point 7 must remain connected voltage.
    fn curve_v7(&self) -> Option<u16> {
        self.curve_v7_callback
            .map(|callback| (callback)(self.context))
    }

    /// V7
    ///
    /// Point 7 must remain connected voltage.
    fn set_curve_v7(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v7_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms8
    ///
    /// Point 8 must remain connected duration.
    fn curve_tms8(&self) -> Option<u16> {
        self.curve_tms8_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms8
    ///
    /// Point 8 must remain connected duration.
    fn set_curve_tms8(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms8_callback {
            (callback)(value, self.context);
        };
    }

    /// V8
    ///
    /// Point 8 must remain connected voltage.
    fn curve_v8(&self) -> Option<u16> {
        self.curve_v8_callback
            .map(|callback| (callback)(self.context))
    }

    /// V8
    ///
    /// Point 8 must remain connected voltage.
    fn set_curve_v8(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v8_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms9
    ///
    /// Point 9 must remain connected duration.
    fn curve_tms9(&self) -> Option<u16> {
        self.curve_tms9_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms9
    ///
    /// Point 9 must remain connected duration.
    fn set_curve_tms9(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms9_callback {
            (callback)(value, self.context);
        };
    }

    /// V9
    ///
    /// Point 9 must remain connected voltage.
    fn curve_v9(&self) -> Option<u16> {
        self.curve_v9_callback
            .map(|callback| (callback)(self.context))
    }

    /// V9
    ///
    /// Point 9 must remain connected voltage.
    fn set_curve_v9(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v9_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms10
    ///
    /// Point 10 must remain connected duration.
    fn curve_tms10(&self) -> Option<u16> {
        self.curve_tms10_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms10
    ///
    /// Point 10 must remain connected duration.
    fn set_curve_tms10(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms10_callback {
            (callback)(value, self.context);
        };
    }

    /// V10
    ///
    /// Point 10 must remain connected voltage.
    fn curve_v10(&self) -> Option<u16> {
        self.curve_v10_callback
            .map(|callback| (callback)(self.context))
    }

    /// V10
    ///
    /// Point 10 must remain connected voltage.
    fn set_curve_v10(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v10_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms11
    ///
    /// Point 11 must remain connected duration.
    fn curve_tms11(&self) -> Option<u16> {
        self.curve_tms11_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms11
    ///
    /// Point 11 must remain connected duration.
    fn set_curve_tms11(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms11_callback {
            (callback)(value, self.context);
        };
    }

    /// V11
    ///
    /// Point 11 must remain connected voltage.
    fn curve_v11(&self) -> Option<u16> {
        self.curve_v11_callback
            .map(|callback| (callback)(self.context))
    }

    /// V11
    ///
    /// Point 11 must remain connected voltage.
    fn set_curve_v11(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v11_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms12
    ///
    /// Point 12 must remain connected duration.
    fn curve_tms12(&self) -> Option<u16> {
        self.curve_tms12_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms12
    ///
    /// Point 12 must remain connected duration.
    fn set_curve_tms12(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms12_callback {
            (callback)(value, self.context);
        };
    }

    /// V12
    ///
    /// Point 12 must remain connected voltage.
    fn curve_v12(&self) -> Option<u16> {
        self.curve_v12_callback
            .map(|callback| (callback)(self.context))
    }

    /// V12
    ///
    /// Point 12 must remain connected voltage.
    fn set_curve_v12(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v12_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms13
    ///
    /// Point 13 must remain connected duration.
    fn curve_tms13(&self) -> Option<u16> {
        self.curve_tms13_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms13
    ///
    /// Point 13 must remain connected duration.
    fn set_curve_tms13(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms13_callback {
            (callback)(value, self.context);
        };
    }

    /// V13
    ///
    /// Point 13 must remain connected voltage.
    fn curve_v13(&self) -> Option<u16> {
        self.curve_v13_callback
            .map(|callback| (callback)(self.context))
    }

    /// V13
    ///
    /// Point 13 must remain connected voltage.
    fn set_curve_v13(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v13_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms14
    ///
    /// Point 14 must remain connected duration.
    fn curve_tms14(&self) -> Option<u16> {
        self.curve_tms14_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms14
    ///
    /// Point 14 must remain connected duration.
    fn set_curve_tms14(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms14_callback {
            (callback)(value, self.context);
        };
    }

    /// V14
    ///
    /// Point 14 must remain connected voltage.
    fn curve_v14(&self) -> Option<u16> {
        self.curve_v14_callback
            .map(|callback| (callback)(self.context))
    }

    /// V14
    ///
    /// Point 14 must remain connected voltage.
    fn set_curve_v14(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v14_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms15
    ///
    /// Point 15 must remain connected duration.
    fn curve_tms15(&self) -> Option<u16> {
        self.curve_tms15_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms15
    ///
    /// Point 15 must remain connected duration.
    fn set_curve_tms15(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms15_callback {
            (callback)(value, self.context);
        };
    }

    /// V15
    ///
    /// Point 15 must remain connected voltage.
    fn curve_v15(&self) -> Option<u16> {
        self.curve_v15_callback
            .map(|callback| (callback)(self.context))
    }

    /// V15
    ///
    /// Point 15 must remain connected voltage.
    fn set_curve_v15(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v15_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms16
    ///
    /// Point 16 must remain connected duration.
    fn curve_tms16(&self) -> Option<u16> {
        self.curve_tms16_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms16
    ///
    /// Point 16 must remain connected duration.
    fn set_curve_tms16(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms16_callback {
            (callback)(value, self.context);
        };
    }

    /// V16
    ///
    /// Point 16 must remain connected voltage.
    fn curve_v16(&self) -> Option<u16> {
        self.curve_v16_callback
            .map(|callback| (callback)(self.context))
    }

    /// V16
    ///
    /// Point 16 must remain connected voltage.
    fn set_curve_v16(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v16_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms17
    ///
    /// Point 17 must remain connected duration.
    fn curve_tms17(&self) -> Option<u16> {
        self.curve_tms17_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms17
    ///
    /// Point 17 must remain connected duration.
    fn set_curve_tms17(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms17_callback {
            (callback)(value, self.context);
        };
    }

    /// V17
    ///
    /// Point 17 must remain connected voltage.
    fn curve_v17(&self) -> Option<u16> {
        self.curve_v17_callback
            .map(|callback| (callback)(self.context))
    }

    /// V17
    ///
    /// Point 17 must remain connected voltage.
    fn set_curve_v17(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v17_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms18
    ///
    /// Point 18 must remain connected duration.
    fn curve_tms18(&self) -> Option<u16> {
        self.curve_tms18_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms18
    ///
    /// Point 18 must remain connected duration.
    fn set_curve_tms18(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms18_callback {
            (callback)(value, self.context);
        };
    }

    /// V18
    ///
    /// Point 18 must remain connected voltage.
    fn curve_v18(&self) -> Option<u16> {
        self.curve_v18_callback
            .map(|callback| (callback)(self.context))
    }

    /// V18
    ///
    /// Point 18 must remain connected voltage.
    fn set_curve_v18(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v18_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms19
    ///
    /// Point 19 must remain connected duration.
    fn curve_tms19(&self) -> Option<u16> {
        self.curve_tms19_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms19
    ///
    /// Point 19 must remain connected duration.
    fn set_curve_tms19(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms19_callback {
            (callback)(value, self.context);
        };
    }

    /// V19
    ///
    /// Point 19 must remain connected voltage.
    fn curve_v19(&self) -> Option<u16> {
        self.curve_v19_callback
            .map(|callback| (callback)(self.context))
    }

    /// V19
    ///
    /// Point 19 must remain connected voltage.
    fn set_curve_v19(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v19_callback {
            (callback)(value, self.context);
        };
    }

    /// Tms20
    ///
    /// Point 20 must remain connected duration.
    fn curve_tms20(&self) -> Option<u16> {
        self.curve_tms20_callback
            .map(|callback| (callback)(self.context))
    }

    /// Tms20
    ///
    /// Point 20 must remain connected duration.
    fn set_curve_tms20(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_tms20_callback {
            (callback)(value, self.context);
        };
    }

    /// V20
    ///
    /// Point 20 must remain connected voltage.
    fn curve_v20(&self) -> Option<u16> {
        self.curve_v20_callback
            .map(|callback| (callback)(self.context))
    }

    /// V20
    ///
    /// Point 20 must remain connected voltage.
    fn set_curve_v20(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_v20_callback {
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

    /// ReadOnly
    ///
    /// Curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly {
        (self.curve_read_only_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model137StatefulAdapter {
    act_crv: u16,
    mod_ena: u16,
    win_tms: u16,
    rvrt_tms: u16,
    rmp_tms: u16,
    n_crv: u16,
    n_pt: u16,
    tms_sf: u16,
    v_sf: u16,
    curve_act_pt: u16,
    curve_tms1: u16,
    curve_v1: u16,
    curve_tms2: u16,
    curve_v2: u16,
    curve_tms3: u16,
    curve_v3: u16,
    curve_tms4: u16,
    curve_v4: u16,
    curve_tms5: u16,
    curve_v5: u16,
    curve_tms6: u16,
    curve_v6: u16,
    curve_tms7: u16,
    curve_v7: u16,
    curve_tms8: u16,
    curve_v8: u16,
    curve_tms9: u16,
    curve_v9: u16,
    curve_tms10: u16,
    curve_v10: u16,
    curve_tms11: u16,
    curve_v11: u16,
    curve_tms12: u16,
    curve_v12: u16,
    curve_tms13: u16,
    curve_v13: u16,
    curve_tms14: u16,
    curve_v14: u16,
    curve_tms15: u16,
    curve_v15: u16,
    curve_tms16: u16,
    curve_v16: u16,
    curve_tms17: u16,
    curve_v17: u16,
    curve_tms18: u16,
    curve_v18: u16,
    curve_tms19: u16,
    curve_v19: u16,
    curve_tms20: u16,
    curve_v20: u16,
    curve_crv_nam: [c_char; 16],
    curve_read_only: ReadOnly,
}

impl ModelAdapter for Model137StatefulAdapter {
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
    /// LVRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    fn win_tms(&self) -> Option<u16> {
        Some(self.win_tms)
    }

    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_win_tms(&mut self, value: u16) {
        self.win_tms = value;
    }

    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        Some(self.rvrt_tms)
    }

    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {
        self.rvrt_tms = value;
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        Some(self.rmp_tms)
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
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

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16 {
        self.v_sf
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

    /// Tms1
    ///
    /// Point 1 must remain connected duration.
    fn curve_tms1(&self) -> u16 {
        self.curve_tms1
    }

    /// Tms1
    ///
    /// Point 1 must remain connected duration.
    fn set_curve_tms1(&mut self, value: u16) {
        self.curve_tms1 = value;
    }

    /// V1
    ///
    /// Point 1 must remain connected voltage.
    fn curve_v1(&self) -> u16 {
        self.curve_v1
    }

    /// V1
    ///
    /// Point 1 must remain connected voltage.
    fn set_curve_v1(&mut self, value: u16) {
        self.curve_v1 = value;
    }

    /// Tms2
    ///
    /// Point 2 must remain connected duration.
    fn curve_tms2(&self) -> Option<u16> {
        Some(self.curve_tms2)
    }

    /// Tms2
    ///
    /// Point 2 must remain connected duration.
    fn set_curve_tms2(&mut self, value: u16) {
        self.curve_tms2 = value;
    }

    /// V2
    ///
    /// Point 2 must remain connected voltage.
    fn curve_v2(&self) -> Option<u16> {
        Some(self.curve_v2)
    }

    /// V2
    ///
    /// Point 2 must remain connected voltage.
    fn set_curve_v2(&mut self, value: u16) {
        self.curve_v2 = value;
    }

    /// Tms3
    ///
    /// Point 3 must remain connected duration.
    fn curve_tms3(&self) -> Option<u16> {
        Some(self.curve_tms3)
    }

    /// Tms3
    ///
    /// Point 3 must remain connected duration.
    fn set_curve_tms3(&mut self, value: u16) {
        self.curve_tms3 = value;
    }

    /// V3
    ///
    /// Point 3 must remain connected voltage.
    fn curve_v3(&self) -> Option<u16> {
        Some(self.curve_v3)
    }

    /// V3
    ///
    /// Point 3 must remain connected voltage.
    fn set_curve_v3(&mut self, value: u16) {
        self.curve_v3 = value;
    }

    /// Tms4
    ///
    /// Point 4 must remain connected duration.
    fn curve_tms4(&self) -> Option<u16> {
        Some(self.curve_tms4)
    }

    /// Tms4
    ///
    /// Point 4 must remain connected duration.
    fn set_curve_tms4(&mut self, value: u16) {
        self.curve_tms4 = value;
    }

    /// V4
    ///
    /// Point 4 must remain connected voltage.
    fn curve_v4(&self) -> Option<u16> {
        Some(self.curve_v4)
    }

    /// V4
    ///
    /// Point 4 must remain connected voltage.
    fn set_curve_v4(&mut self, value: u16) {
        self.curve_v4 = value;
    }

    /// Tms5
    ///
    /// Point 5 must remain connected duration.
    fn curve_tms5(&self) -> Option<u16> {
        Some(self.curve_tms5)
    }

    /// Tms5
    ///
    /// Point 5 must remain connected duration.
    fn set_curve_tms5(&mut self, value: u16) {
        self.curve_tms5 = value;
    }

    /// V5
    ///
    /// Point 5 must remain connected voltage.
    fn curve_v5(&self) -> Option<u16> {
        Some(self.curve_v5)
    }

    /// V5
    ///
    /// Point 5 must remain connected voltage.
    fn set_curve_v5(&mut self, value: u16) {
        self.curve_v5 = value;
    }

    /// Tms6
    ///
    /// Point 6 must remain connected duration.
    fn curve_tms6(&self) -> Option<u16> {
        Some(self.curve_tms6)
    }

    /// Tms6
    ///
    /// Point 6 must remain connected duration.
    fn set_curve_tms6(&mut self, value: u16) {
        self.curve_tms6 = value;
    }

    /// V6
    ///
    /// Point 6 must remain connected voltage.
    fn curve_v6(&self) -> Option<u16> {
        Some(self.curve_v6)
    }

    /// V6
    ///
    /// Point 6 must remain connected voltage.
    fn set_curve_v6(&mut self, value: u16) {
        self.curve_v6 = value;
    }

    /// Tms7
    ///
    /// Point 7 must remain connected duration.
    fn curve_tms7(&self) -> Option<u16> {
        Some(self.curve_tms7)
    }

    /// Tms7
    ///
    /// Point 7 must remain connected duration.
    fn set_curve_tms7(&mut self, value: u16) {
        self.curve_tms7 = value;
    }

    /// V7
    ///
    /// Point 7 must remain connected voltage.
    fn curve_v7(&self) -> Option<u16> {
        Some(self.curve_v7)
    }

    /// V7
    ///
    /// Point 7 must remain connected voltage.
    fn set_curve_v7(&mut self, value: u16) {
        self.curve_v7 = value;
    }

    /// Tms8
    ///
    /// Point 8 must remain connected duration.
    fn curve_tms8(&self) -> Option<u16> {
        Some(self.curve_tms8)
    }

    /// Tms8
    ///
    /// Point 8 must remain connected duration.
    fn set_curve_tms8(&mut self, value: u16) {
        self.curve_tms8 = value;
    }

    /// V8
    ///
    /// Point 8 must remain connected voltage.
    fn curve_v8(&self) -> Option<u16> {
        Some(self.curve_v8)
    }

    /// V8
    ///
    /// Point 8 must remain connected voltage.
    fn set_curve_v8(&mut self, value: u16) {
        self.curve_v8 = value;
    }

    /// Tms9
    ///
    /// Point 9 must remain connected duration.
    fn curve_tms9(&self) -> Option<u16> {
        Some(self.curve_tms9)
    }

    /// Tms9
    ///
    /// Point 9 must remain connected duration.
    fn set_curve_tms9(&mut self, value: u16) {
        self.curve_tms9 = value;
    }

    /// V9
    ///
    /// Point 9 must remain connected voltage.
    fn curve_v9(&self) -> Option<u16> {
        Some(self.curve_v9)
    }

    /// V9
    ///
    /// Point 9 must remain connected voltage.
    fn set_curve_v9(&mut self, value: u16) {
        self.curve_v9 = value;
    }

    /// Tms10
    ///
    /// Point 10 must remain connected duration.
    fn curve_tms10(&self) -> Option<u16> {
        Some(self.curve_tms10)
    }

    /// Tms10
    ///
    /// Point 10 must remain connected duration.
    fn set_curve_tms10(&mut self, value: u16) {
        self.curve_tms10 = value;
    }

    /// V10
    ///
    /// Point 10 must remain connected voltage.
    fn curve_v10(&self) -> Option<u16> {
        Some(self.curve_v10)
    }

    /// V10
    ///
    /// Point 10 must remain connected voltage.
    fn set_curve_v10(&mut self, value: u16) {
        self.curve_v10 = value;
    }

    /// Tms11
    ///
    /// Point 11 must remain connected duration.
    fn curve_tms11(&self) -> Option<u16> {
        Some(self.curve_tms11)
    }

    /// Tms11
    ///
    /// Point 11 must remain connected duration.
    fn set_curve_tms11(&mut self, value: u16) {
        self.curve_tms11 = value;
    }

    /// V11
    ///
    /// Point 11 must remain connected voltage.
    fn curve_v11(&self) -> Option<u16> {
        Some(self.curve_v11)
    }

    /// V11
    ///
    /// Point 11 must remain connected voltage.
    fn set_curve_v11(&mut self, value: u16) {
        self.curve_v11 = value;
    }

    /// Tms12
    ///
    /// Point 12 must remain connected duration.
    fn curve_tms12(&self) -> Option<u16> {
        Some(self.curve_tms12)
    }

    /// Tms12
    ///
    /// Point 12 must remain connected duration.
    fn set_curve_tms12(&mut self, value: u16) {
        self.curve_tms12 = value;
    }

    /// V12
    ///
    /// Point 12 must remain connected voltage.
    fn curve_v12(&self) -> Option<u16> {
        Some(self.curve_v12)
    }

    /// V12
    ///
    /// Point 12 must remain connected voltage.
    fn set_curve_v12(&mut self, value: u16) {
        self.curve_v12 = value;
    }

    /// Tms13
    ///
    /// Point 13 must remain connected duration.
    fn curve_tms13(&self) -> Option<u16> {
        Some(self.curve_tms13)
    }

    /// Tms13
    ///
    /// Point 13 must remain connected duration.
    fn set_curve_tms13(&mut self, value: u16) {
        self.curve_tms13 = value;
    }

    /// V13
    ///
    /// Point 13 must remain connected voltage.
    fn curve_v13(&self) -> Option<u16> {
        Some(self.curve_v13)
    }

    /// V13
    ///
    /// Point 13 must remain connected voltage.
    fn set_curve_v13(&mut self, value: u16) {
        self.curve_v13 = value;
    }

    /// Tms14
    ///
    /// Point 14 must remain connected duration.
    fn curve_tms14(&self) -> Option<u16> {
        Some(self.curve_tms14)
    }

    /// Tms14
    ///
    /// Point 14 must remain connected duration.
    fn set_curve_tms14(&mut self, value: u16) {
        self.curve_tms14 = value;
    }

    /// V14
    ///
    /// Point 14 must remain connected voltage.
    fn curve_v14(&self) -> Option<u16> {
        Some(self.curve_v14)
    }

    /// V14
    ///
    /// Point 14 must remain connected voltage.
    fn set_curve_v14(&mut self, value: u16) {
        self.curve_v14 = value;
    }

    /// Tms15
    ///
    /// Point 15 must remain connected duration.
    fn curve_tms15(&self) -> Option<u16> {
        Some(self.curve_tms15)
    }

    /// Tms15
    ///
    /// Point 15 must remain connected duration.
    fn set_curve_tms15(&mut self, value: u16) {
        self.curve_tms15 = value;
    }

    /// V15
    ///
    /// Point 15 must remain connected voltage.
    fn curve_v15(&self) -> Option<u16> {
        Some(self.curve_v15)
    }

    /// V15
    ///
    /// Point 15 must remain connected voltage.
    fn set_curve_v15(&mut self, value: u16) {
        self.curve_v15 = value;
    }

    /// Tms16
    ///
    /// Point 16 must remain connected duration.
    fn curve_tms16(&self) -> Option<u16> {
        Some(self.curve_tms16)
    }

    /// Tms16
    ///
    /// Point 16 must remain connected duration.
    fn set_curve_tms16(&mut self, value: u16) {
        self.curve_tms16 = value;
    }

    /// V16
    ///
    /// Point 16 must remain connected voltage.
    fn curve_v16(&self) -> Option<u16> {
        Some(self.curve_v16)
    }

    /// V16
    ///
    /// Point 16 must remain connected voltage.
    fn set_curve_v16(&mut self, value: u16) {
        self.curve_v16 = value;
    }

    /// Tms17
    ///
    /// Point 17 must remain connected duration.
    fn curve_tms17(&self) -> Option<u16> {
        Some(self.curve_tms17)
    }

    /// Tms17
    ///
    /// Point 17 must remain connected duration.
    fn set_curve_tms17(&mut self, value: u16) {
        self.curve_tms17 = value;
    }

    /// V17
    ///
    /// Point 17 must remain connected voltage.
    fn curve_v17(&self) -> Option<u16> {
        Some(self.curve_v17)
    }

    /// V17
    ///
    /// Point 17 must remain connected voltage.
    fn set_curve_v17(&mut self, value: u16) {
        self.curve_v17 = value;
    }

    /// Tms18
    ///
    /// Point 18 must remain connected duration.
    fn curve_tms18(&self) -> Option<u16> {
        Some(self.curve_tms18)
    }

    /// Tms18
    ///
    /// Point 18 must remain connected duration.
    fn set_curve_tms18(&mut self, value: u16) {
        self.curve_tms18 = value;
    }

    /// V18
    ///
    /// Point 18 must remain connected voltage.
    fn curve_v18(&self) -> Option<u16> {
        Some(self.curve_v18)
    }

    /// V18
    ///
    /// Point 18 must remain connected voltage.
    fn set_curve_v18(&mut self, value: u16) {
        self.curve_v18 = value;
    }

    /// Tms19
    ///
    /// Point 19 must remain connected duration.
    fn curve_tms19(&self) -> Option<u16> {
        Some(self.curve_tms19)
    }

    /// Tms19
    ///
    /// Point 19 must remain connected duration.
    fn set_curve_tms19(&mut self, value: u16) {
        self.curve_tms19 = value;
    }

    /// V19
    ///
    /// Point 19 must remain connected voltage.
    fn curve_v19(&self) -> Option<u16> {
        Some(self.curve_v19)
    }

    /// V19
    ///
    /// Point 19 must remain connected voltage.
    fn set_curve_v19(&mut self, value: u16) {
        self.curve_v19 = value;
    }

    /// Tms20
    ///
    /// Point 20 must remain connected duration.
    fn curve_tms20(&self) -> Option<u16> {
        Some(self.curve_tms20)
    }

    /// Tms20
    ///
    /// Point 20 must remain connected duration.
    fn set_curve_tms20(&mut self, value: u16) {
        self.curve_tms20 = value;
    }

    /// V20
    ///
    /// Point 20 must remain connected voltage.
    fn curve_v20(&self) -> Option<u16> {
        Some(self.curve_v20)
    }

    /// V20
    ///
    /// Point 20 must remain connected voltage.
    fn set_curve_v20(&mut self, value: u16) {
        self.curve_v20 = value;
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

    /// ReadOnly
    ///
    /// Curve is read-only or can be modified.
    fn curve_read_only(&self) -> ReadOnly {
        self.curve_read_only
    }
}
