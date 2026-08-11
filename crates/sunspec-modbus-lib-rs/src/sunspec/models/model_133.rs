use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 68;

static POINTS: [PointDetails<()>; 39] = [
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
        point: |()| Point::ActSchd,
        size: 2,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::ModEna,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::NSchd,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::NPts,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::Pad,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::RepeatingActPts,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::RepeatingStrTms,
        size: 2,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::RepeatingRepPer,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::RepeatingSchdTyp,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::RepeatingXTyp,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::RepeatingXSf,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::RepeatingYTyp,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::RepeatingYSf,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::RepeatingX1,
        size: 2,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::RepeatingY1,
        size: 2,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::RepeatingX2,
        size: 2,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::RepeatingY2,
        size: 2,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::RepeatingX3,
        size: 2,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::RepeatingY3,
        size: 2,
        start_address: 27,
    },
    PointDetails {
        point: |()| Point::RepeatingX4,
        size: 2,
        start_address: 29,
    },
    PointDetails {
        point: |()| Point::RepeatingY4,
        size: 2,
        start_address: 31,
    },
    PointDetails {
        point: |()| Point::RepeatingX5,
        size: 2,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::RepeatingY5,
        size: 2,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::RepeatingX6,
        size: 2,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::RepeatingY6,
        size: 2,
        start_address: 39,
    },
    PointDetails {
        point: |()| Point::RepeatingX7,
        size: 2,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::RepeatingY7,
        size: 2,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::RepeatingX8,
        size: 2,
        start_address: 45,
    },
    PointDetails {
        point: |()| Point::RepeatingY8,
        size: 2,
        start_address: 47,
    },
    PointDetails {
        point: |()| Point::RepeatingX9,
        size: 2,
        start_address: 49,
    },
    PointDetails {
        point: |()| Point::RepeatingY9,
        size: 2,
        start_address: 51,
    },
    PointDetails {
        point: |()| Point::RepeatingX10,
        size: 2,
        start_address: 53,
    },
    PointDetails {
        point: |()| Point::RepeatingY10,
        size: 2,
        start_address: 55,
    },
    PointDetails {
        point: |()| Point::RepeatingNam,
        size: 8,
        start_address: 57,
    },
    PointDetails {
        point: |()| Point::RepeatingWinTms,
        size: 1,
        start_address: 65,
    },
    PointDetails {
        point: |()| Point::RepeatingRmpTms,
        size: 1,
        start_address: 66,
    },
    PointDetails {
        point: |()| Point::RepeatingActIndx,
        size: 1,
        start_address: 67,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    ActSchd,
    ModEna,
    NSchd,
    NPts,
    Pad,
    RepeatingActPts,
    RepeatingStrTms,
    RepeatingRepPer,
    RepeatingSchdTyp,
    RepeatingXTyp,
    RepeatingXSf,
    RepeatingYTyp,
    RepeatingYSf,
    RepeatingX1,
    RepeatingY1,
    RepeatingX2,
    RepeatingY2,
    RepeatingX3,
    RepeatingY3,
    RepeatingX4,
    RepeatingY4,
    RepeatingX5,
    RepeatingY5,
    RepeatingX6,
    RepeatingY6,
    RepeatingX7,
    RepeatingY7,
    RepeatingX8,
    RepeatingY8,
    RepeatingX9,
    RepeatingY9,
    RepeatingX10,
    RepeatingY10,
    RepeatingNam,
    RepeatingWinTms,
    RepeatingRmpTms,
    RepeatingActIndx,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    68
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
            buffer::write_u16(133, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::ActSchd => {
            buffer::write_u32(model.act_schd(), buffer, offset, limit);
        }
        Point::ModEna => {
            buffer::write_u16(model.mod_ena(), buffer);
        }
        Point::NSchd => {
            buffer::write_u16(model.n_schd(), buffer);
        }
        Point::NPts => {
            buffer::write_u16(model.n_pts(), buffer);
        }
        Point::Pad => {
            buffer::write_u16(0, buffer);
        }
        Point::RepeatingActPts => {
            buffer::write_u16(model.repeating_act_pts(), buffer);
        }
        Point::RepeatingStrTms => {
            buffer::write_u32(model.repeating_str_tms(), buffer, offset, limit);
        }
        Point::RepeatingRepPer => {
            buffer::write_u16(model.repeating_rep_per(), buffer);
        }
        Point::RepeatingSchdTyp => {
            buffer::write_u16(model.repeating_schd_typ() as u16, buffer);
        }
        Point::RepeatingXTyp => {
            buffer::write_u16(model.repeating_x_typ() as u16, buffer);
        }
        Point::RepeatingXSf => {
            buffer::write_u16(model.repeating_x_sf(), buffer);
        }
        Point::RepeatingYTyp => {
            buffer::write_u16(model.repeating_y_typ() as u16, buffer);
        }
        Point::RepeatingYSf => {
            buffer::write_u16(model.repeating_y_sf(), buffer);
        }
        Point::RepeatingX1 => {
            buffer::write_i32(model.repeating_x1(), buffer, offset, limit);
        }
        Point::RepeatingY1 => {
            buffer::write_i32(model.repeating_y1(), buffer, offset, limit);
        }
        Point::RepeatingX2 => {
            if let Some(value) = model.repeating_x2() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY2 => {
            if let Some(value) = model.repeating_y2() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingX3 => {
            if let Some(value) = model.repeating_x3() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY3 => {
            if let Some(value) = model.repeating_y3() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingX4 => {
            if let Some(value) = model.repeating_x4() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY4 => {
            if let Some(value) = model.repeating_y4() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingX5 => {
            if let Some(value) = model.repeating_x5() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY5 => {
            if let Some(value) = model.repeating_y5() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingX6 => {
            if let Some(value) = model.repeating_x6() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY6 => {
            if let Some(value) = model.repeating_y6() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingX7 => {
            if let Some(value) = model.repeating_x7() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY7 => {
            if let Some(value) = model.repeating_y7() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingX8 => {
            if let Some(value) = model.repeating_x8() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY8 => {
            if let Some(value) = model.repeating_y8() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingX9 => {
            if let Some(value) = model.repeating_x9() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY9 => {
            if let Some(value) = model.repeating_y9() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingX10 => {
            if let Some(value) = model.repeating_x10() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingY10 => {
            if let Some(value) = model.repeating_y10() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingNam => {
            if let Some(value) = model.repeating_nam() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingWinTms => {
            if let Some(value) = model.repeating_win_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingRmpTms => {
            if let Some(value) = model.repeating_rmp_tms() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RepeatingActIndx => {
            buffer::write_u16(model.repeating_act_indx(), buffer);
        }
    }
}

pub trait ModelAdapter {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn act_schd(&self) -> u32;

    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn set_act_schd(&mut self, value: u32);

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn set_mod_ena(&mut self, value: u16);

    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    fn n_schd(&self) -> u16;

    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    fn n_pts(&self) -> u16;

    /// ActPts
    ///
    /// Number of active entries in schedule.
    fn repeating_act_pts(&self) -> u16;

    /// ActPts
    ///
    /// Number of active entries in schedule.
    fn set_repeating_act_pts(&mut self, value: u16);

    /// StrTms
    ///
    /// Schedule start in seconds since 2000 JAN 01 00:00:00 UTC.
    fn repeating_str_tms(&self) -> u32;

    /// StrTms
    ///
    /// Schedule start in seconds since 2000 JAN 01 00:00:00 UTC.
    fn set_repeating_str_tms(&mut self, value: u32);

    /// RepPer
    ///
    /// The repetition count for time-based schedules (0=repeat forever)
    fn repeating_rep_per(&self) -> u16;

    /// RepPer
    ///
    /// The repetition count for time-based schedules (0=repeat forever)
    fn set_repeating_rep_per(&mut self, value: u16);

    /// SchdTyp
    ///
    /// The repetition frequency for time-based schedules: no repeat=0
    fn repeating_schd_typ(&self) -> IntvTyp;

    /// SchdTyp
    ///
    /// The repetition frequency for time-based schedules: no repeat=0
    fn set_repeating_schd_typ(&mut self, value: IntvTyp);

    /// XTyp
    ///
    /// The meaning of the X-values in the array.
    fn repeating_x_typ(&self) -> XTyp;

    /// XTyp
    ///
    /// The meaning of the X-values in the array.
    fn set_repeating_x_typ(&mut self, value: XTyp);

    /// X_SF
    ///
    /// Scale factor for schedule range values.
    fn repeating_x_sf(&self) -> u16;

    /// X_SF
    ///
    /// Scale factor for schedule range values.
    fn set_repeating_x_sf(&mut self, value: u16);

    /// YTyp
    ///
    /// The meaning of the Y-values in the array.
    fn repeating_y_typ(&self) -> YTyp;

    /// YTyp
    ///
    /// The meaning of the Y-values in the array.
    fn set_repeating_y_typ(&mut self, value: YTyp);

    /// Y_SF
    ///
    /// Scale factor for schedule target values.
    fn repeating_y_sf(&self) -> u16;

    /// Y_SF
    ///
    /// Scale factor for schedule target values.
    fn set_repeating_y_sf(&mut self, value: u16);

    /// X1
    ///
    /// Entry 1 range.
    fn repeating_x1(&self) -> i32;

    /// X1
    ///
    /// Entry 1 range.
    fn set_repeating_x1(&mut self, value: i32);

    /// Y1
    ///
    /// Entry 1 target.
    fn repeating_y1(&self) -> i32;

    /// Y1
    ///
    /// Entry 1 target.
    fn set_repeating_y1(&mut self, value: i32);

    /// X2
    ///
    /// Entry 2 range.
    fn repeating_x2(&self) -> Option<i32> {
        None
    }

    /// X2
    ///
    /// Entry 2 range.
    fn set_repeating_x2(&mut self, value: i32) {}

    /// Y2
    ///
    /// Entry 2 target.
    fn repeating_y2(&self) -> Option<i32> {
        None
    }

    /// Y2
    ///
    /// Entry 2 target.
    fn set_repeating_y2(&mut self, value: i32) {}

    /// X3
    ///
    /// Entry 3 range.
    fn repeating_x3(&self) -> Option<i32> {
        None
    }

    /// X3
    ///
    /// Entry 3 range.
    fn set_repeating_x3(&mut self, value: i32) {}

    /// Y3
    ///
    /// Entry 3 target.
    fn repeating_y3(&self) -> Option<i32> {
        None
    }

    /// Y3
    ///
    /// Entry 3 target.
    fn set_repeating_y3(&mut self, value: i32) {}

    /// X4
    ///
    /// Entry 4 range.
    fn repeating_x4(&self) -> Option<i32> {
        None
    }

    /// X4
    ///
    /// Entry 4 range.
    fn set_repeating_x4(&mut self, value: i32) {}

    /// Y4
    ///
    /// Entry 4 target.
    fn repeating_y4(&self) -> Option<i32> {
        None
    }

    /// Y4
    ///
    /// Entry 4 target.
    fn set_repeating_y4(&mut self, value: i32) {}

    /// X5
    ///
    /// Entry 15range.
    fn repeating_x5(&self) -> Option<i32> {
        None
    }

    /// X5
    ///
    /// Entry 15range.
    fn set_repeating_x5(&mut self, value: i32) {}

    /// Y5
    ///
    /// Entry 5 target.
    fn repeating_y5(&self) -> Option<i32> {
        None
    }

    /// Y5
    ///
    /// Entry 5 target.
    fn set_repeating_y5(&mut self, value: i32) {}

    /// X6
    ///
    /// Entry 6 range.
    fn repeating_x6(&self) -> Option<i32> {
        None
    }

    /// X6
    ///
    /// Entry 6 range.
    fn set_repeating_x6(&mut self, value: i32) {}

    /// Y6
    ///
    /// Entry 6 target.
    fn repeating_y6(&self) -> Option<i32> {
        None
    }

    /// Y6
    ///
    /// Entry 6 target.
    fn set_repeating_y6(&mut self, value: i32) {}

    /// X7
    ///
    /// Entry 7 range.
    fn repeating_x7(&self) -> Option<i32> {
        None
    }

    /// X7
    ///
    /// Entry 7 range.
    fn set_repeating_x7(&mut self, value: i32) {}

    /// Y7
    ///
    /// Entry 7 target.
    fn repeating_y7(&self) -> Option<i32> {
        None
    }

    /// Y7
    ///
    /// Entry 7 target.
    fn set_repeating_y7(&mut self, value: i32) {}

    /// X8
    ///
    /// Entry 8 range.
    fn repeating_x8(&self) -> Option<i32> {
        None
    }

    /// X8
    ///
    /// Entry 8 range.
    fn set_repeating_x8(&mut self, value: i32) {}

    /// Y8
    ///
    /// Entry 8 target.
    fn repeating_y8(&self) -> Option<i32> {
        None
    }

    /// Y8
    ///
    /// Entry 8 target.
    fn set_repeating_y8(&mut self, value: i32) {}

    /// X9
    ///
    /// Entry 9 range.
    fn repeating_x9(&self) -> Option<i32> {
        None
    }

    /// X9
    ///
    /// Entry 9 range.
    fn set_repeating_x9(&mut self, value: i32) {}

    /// Y9
    ///
    /// Entry 9 target.
    fn repeating_y9(&self) -> Option<i32> {
        None
    }

    /// Y9
    ///
    /// Entry 9 target.
    fn set_repeating_y9(&mut self, value: i32) {}

    /// X10
    ///
    /// Entry 10 range.
    fn repeating_x10(&self) -> Option<i32> {
        None
    }

    /// X10
    ///
    /// Entry 10 range.
    fn set_repeating_x10(&mut self, value: i32) {}

    /// Y10
    ///
    /// Entry 10 target.
    fn repeating_y10(&self) -> Option<i32> {
        None
    }

    /// Y10
    ///
    /// Entry 10 target.
    fn set_repeating_y10(&mut self, value: i32) {}

    /// Nam
    ///
    /// Optional description for schedule.
    fn repeating_nam(&self) -> Option<&CStr> {
        None
    }

    /// Nam
    ///
    /// Optional description for schedule.
    fn set_repeating_nam(&mut self, value: &CStr) {}

    /// WinTms
    ///
    /// Time window for schedule entry change.
    fn repeating_win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for schedule entry change.
    fn set_repeating_win_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// Ramp time for moving from current target to new target.
    fn repeating_rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current target to new target.
    fn set_repeating_rmp_tms(&mut self, value: u16) {}

    /// ActIndx
    ///
    /// Index of active entry in the active schedule.
    fn repeating_act_indx(&self) -> u16;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum IntvTyp {
    Onetime = 0,
    Daily = 1,
    Weekly = 2,
    Monthly = 3,
    Weekday = 4,
    Holiday = 5,
    Weekend = 6,
    Yearly = 7,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum XTyp {
    Unset = 0,
    Time = 1,
    Temp = 2,
    Price = 3,
    Other = 99,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum YTyp {
    Unset = 0,
    WMax = 1,
    Rsrvd2 = 2,
    Pf = 3,
    Rsrvd4 = 4,
    WattPrice = 5,
    VarPrice = 6,
    Rsrvd7 = 7,
    VoltVarArray = 8,
    WChaGra = 9,
    WDisChaGra = 10,
    VArAval = 11,
    Schedule = 12,
    Other = 99,
}

#[repr(C)]
pub struct Model133CallbackAdapter {
    context: *mut c_void,
    act_schd_callback: extern "C" fn(*const c_void) -> u32,
    set_act_schd_callback: extern "C" fn(u32, *mut c_void),
    mod_ena_callback: extern "C" fn(*const c_void) -> u16,
    set_mod_ena_callback: extern "C" fn(u16, *mut c_void),
    n_schd_callback: extern "C" fn(*const c_void) -> u16,
    n_pts_callback: extern "C" fn(*const c_void) -> u16,
    repeating_act_pts_callback: extern "C" fn(*const c_void) -> u16,
    set_repeating_act_pts_callback: extern "C" fn(u16, *mut c_void),
    repeating_str_tms_callback: extern "C" fn(*const c_void) -> u32,
    set_repeating_str_tms_callback: extern "C" fn(u32, *mut c_void),
    repeating_rep_per_callback: extern "C" fn(*const c_void) -> u16,
    set_repeating_rep_per_callback: extern "C" fn(u16, *mut c_void),
    repeating_schd_typ_callback: extern "C" fn(*const c_void) -> IntvTyp,
    set_repeating_schd_typ_callback: extern "C" fn(IntvTyp, *mut c_void),
    repeating_x_typ_callback: extern "C" fn(*const c_void) -> XTyp,
    set_repeating_x_typ_callback: extern "C" fn(XTyp, *mut c_void),
    repeating_x_sf_callback: extern "C" fn(*const c_void) -> u16,
    set_repeating_x_sf_callback: extern "C" fn(u16, *mut c_void),
    repeating_y_typ_callback: extern "C" fn(*const c_void) -> YTyp,
    set_repeating_y_typ_callback: extern "C" fn(YTyp, *mut c_void),
    repeating_y_sf_callback: extern "C" fn(*const c_void) -> u16,
    set_repeating_y_sf_callback: extern "C" fn(u16, *mut c_void),
    repeating_x1_callback: extern "C" fn(*const c_void) -> i32,
    set_repeating_x1_callback: extern "C" fn(i32, *mut c_void),
    repeating_y1_callback: extern "C" fn(*const c_void) -> i32,
    set_repeating_y1_callback: extern "C" fn(i32, *mut c_void),
    repeating_x2_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x2_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y2_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y2_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_x3_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x3_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y3_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y3_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_x4_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x4_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y4_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y4_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_x5_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x5_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y5_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y5_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_x6_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x6_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y6_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y6_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_x7_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x7_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y7_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y7_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_x8_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x8_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y8_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y8_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_x9_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x9_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y9_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y9_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_x10_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_x10_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_y10_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_repeating_y10_callback: Option<extern "C" fn(i32, *mut c_void)>,
    repeating_nam_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_repeating_nam_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    repeating_win_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_repeating_win_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    repeating_rmp_tms_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_repeating_rmp_tms_callback: Option<extern "C" fn(u16, *mut c_void)>,
    repeating_act_indx_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model133CallbackAdapter {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn act_schd(&self) -> u32 {
        (self.act_schd_callback)(self.context)
    }

    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn set_act_schd(&mut self, value: u32) {
        (self.set_act_schd_callback)(value, self.context);
    }

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    fn n_schd(&self) -> u16 {
        (self.n_schd_callback)(self.context)
    }

    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    fn n_pts(&self) -> u16 {
        (self.n_pts_callback)(self.context)
    }

    /// ActPts
    ///
    /// Number of active entries in schedule.
    fn repeating_act_pts(&self) -> u16 {
        (self.repeating_act_pts_callback)(self.context)
    }

    /// ActPts
    ///
    /// Number of active entries in schedule.
    fn set_repeating_act_pts(&mut self, value: u16) {
        (self.set_repeating_act_pts_callback)(value, self.context);
    }

    /// StrTms
    ///
    /// Schedule start in seconds since 2000 JAN 01 00:00:00 UTC.
    fn repeating_str_tms(&self) -> u32 {
        (self.repeating_str_tms_callback)(self.context)
    }

    /// StrTms
    ///
    /// Schedule start in seconds since 2000 JAN 01 00:00:00 UTC.
    fn set_repeating_str_tms(&mut self, value: u32) {
        (self.set_repeating_str_tms_callback)(value, self.context);
    }

    /// RepPer
    ///
    /// The repetition count for time-based schedules (0=repeat forever)
    fn repeating_rep_per(&self) -> u16 {
        (self.repeating_rep_per_callback)(self.context)
    }

    /// RepPer
    ///
    /// The repetition count for time-based schedules (0=repeat forever)
    fn set_repeating_rep_per(&mut self, value: u16) {
        (self.set_repeating_rep_per_callback)(value, self.context);
    }

    /// SchdTyp
    ///
    /// The repetition frequency for time-based schedules: no repeat=0
    fn repeating_schd_typ(&self) -> IntvTyp {
        (self.repeating_schd_typ_callback)(self.context)
    }

    /// SchdTyp
    ///
    /// The repetition frequency for time-based schedules: no repeat=0
    fn set_repeating_schd_typ(&mut self, value: IntvTyp) {
        (self.set_repeating_schd_typ_callback)(value, self.context);
    }

    /// XTyp
    ///
    /// The meaning of the X-values in the array.
    fn repeating_x_typ(&self) -> XTyp {
        (self.repeating_x_typ_callback)(self.context)
    }

    /// XTyp
    ///
    /// The meaning of the X-values in the array.
    fn set_repeating_x_typ(&mut self, value: XTyp) {
        (self.set_repeating_x_typ_callback)(value, self.context);
    }

    /// X_SF
    ///
    /// Scale factor for schedule range values.
    fn repeating_x_sf(&self) -> u16 {
        (self.repeating_x_sf_callback)(self.context)
    }

    /// X_SF
    ///
    /// Scale factor for schedule range values.
    fn set_repeating_x_sf(&mut self, value: u16) {
        (self.set_repeating_x_sf_callback)(value, self.context);
    }

    /// YTyp
    ///
    /// The meaning of the Y-values in the array.
    fn repeating_y_typ(&self) -> YTyp {
        (self.repeating_y_typ_callback)(self.context)
    }

    /// YTyp
    ///
    /// The meaning of the Y-values in the array.
    fn set_repeating_y_typ(&mut self, value: YTyp) {
        (self.set_repeating_y_typ_callback)(value, self.context);
    }

    /// Y_SF
    ///
    /// Scale factor for schedule target values.
    fn repeating_y_sf(&self) -> u16 {
        (self.repeating_y_sf_callback)(self.context)
    }

    /// Y_SF
    ///
    /// Scale factor for schedule target values.
    fn set_repeating_y_sf(&mut self, value: u16) {
        (self.set_repeating_y_sf_callback)(value, self.context);
    }

    /// X1
    ///
    /// Entry 1 range.
    fn repeating_x1(&self) -> i32 {
        (self.repeating_x1_callback)(self.context)
    }

    /// X1
    ///
    /// Entry 1 range.
    fn set_repeating_x1(&mut self, value: i32) {
        (self.set_repeating_x1_callback)(value, self.context);
    }

    /// Y1
    ///
    /// Entry 1 target.
    fn repeating_y1(&self) -> i32 {
        (self.repeating_y1_callback)(self.context)
    }

    /// Y1
    ///
    /// Entry 1 target.
    fn set_repeating_y1(&mut self, value: i32) {
        (self.set_repeating_y1_callback)(value, self.context);
    }

    /// X2
    ///
    /// Entry 2 range.
    fn repeating_x2(&self) -> Option<i32> {
        self.repeating_x2_callback
            .map(|callback| (callback)(self.context))
    }

    /// X2
    ///
    /// Entry 2 range.
    fn set_repeating_x2(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x2_callback {
            (callback)(value, self.context);
        };
    }

    /// Y2
    ///
    /// Entry 2 target.
    fn repeating_y2(&self) -> Option<i32> {
        self.repeating_y2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y2
    ///
    /// Entry 2 target.
    fn set_repeating_y2(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y2_callback {
            (callback)(value, self.context);
        };
    }

    /// X3
    ///
    /// Entry 3 range.
    fn repeating_x3(&self) -> Option<i32> {
        self.repeating_x3_callback
            .map(|callback| (callback)(self.context))
    }

    /// X3
    ///
    /// Entry 3 range.
    fn set_repeating_x3(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x3_callback {
            (callback)(value, self.context);
        };
    }

    /// Y3
    ///
    /// Entry 3 target.
    fn repeating_y3(&self) -> Option<i32> {
        self.repeating_y3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y3
    ///
    /// Entry 3 target.
    fn set_repeating_y3(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y3_callback {
            (callback)(value, self.context);
        };
    }

    /// X4
    ///
    /// Entry 4 range.
    fn repeating_x4(&self) -> Option<i32> {
        self.repeating_x4_callback
            .map(|callback| (callback)(self.context))
    }

    /// X4
    ///
    /// Entry 4 range.
    fn set_repeating_x4(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x4_callback {
            (callback)(value, self.context);
        };
    }

    /// Y4
    ///
    /// Entry 4 target.
    fn repeating_y4(&self) -> Option<i32> {
        self.repeating_y4_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y4
    ///
    /// Entry 4 target.
    fn set_repeating_y4(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y4_callback {
            (callback)(value, self.context);
        };
    }

    /// X5
    ///
    /// Entry 15range.
    fn repeating_x5(&self) -> Option<i32> {
        self.repeating_x5_callback
            .map(|callback| (callback)(self.context))
    }

    /// X5
    ///
    /// Entry 15range.
    fn set_repeating_x5(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x5_callback {
            (callback)(value, self.context);
        };
    }

    /// Y5
    ///
    /// Entry 5 target.
    fn repeating_y5(&self) -> Option<i32> {
        self.repeating_y5_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y5
    ///
    /// Entry 5 target.
    fn set_repeating_y5(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y5_callback {
            (callback)(value, self.context);
        };
    }

    /// X6
    ///
    /// Entry 6 range.
    fn repeating_x6(&self) -> Option<i32> {
        self.repeating_x6_callback
            .map(|callback| (callback)(self.context))
    }

    /// X6
    ///
    /// Entry 6 range.
    fn set_repeating_x6(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x6_callback {
            (callback)(value, self.context);
        };
    }

    /// Y6
    ///
    /// Entry 6 target.
    fn repeating_y6(&self) -> Option<i32> {
        self.repeating_y6_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y6
    ///
    /// Entry 6 target.
    fn set_repeating_y6(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y6_callback {
            (callback)(value, self.context);
        };
    }

    /// X7
    ///
    /// Entry 7 range.
    fn repeating_x7(&self) -> Option<i32> {
        self.repeating_x7_callback
            .map(|callback| (callback)(self.context))
    }

    /// X7
    ///
    /// Entry 7 range.
    fn set_repeating_x7(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x7_callback {
            (callback)(value, self.context);
        };
    }

    /// Y7
    ///
    /// Entry 7 target.
    fn repeating_y7(&self) -> Option<i32> {
        self.repeating_y7_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y7
    ///
    /// Entry 7 target.
    fn set_repeating_y7(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y7_callback {
            (callback)(value, self.context);
        };
    }

    /// X8
    ///
    /// Entry 8 range.
    fn repeating_x8(&self) -> Option<i32> {
        self.repeating_x8_callback
            .map(|callback| (callback)(self.context))
    }

    /// X8
    ///
    /// Entry 8 range.
    fn set_repeating_x8(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x8_callback {
            (callback)(value, self.context);
        };
    }

    /// Y8
    ///
    /// Entry 8 target.
    fn repeating_y8(&self) -> Option<i32> {
        self.repeating_y8_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y8
    ///
    /// Entry 8 target.
    fn set_repeating_y8(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y8_callback {
            (callback)(value, self.context);
        };
    }

    /// X9
    ///
    /// Entry 9 range.
    fn repeating_x9(&self) -> Option<i32> {
        self.repeating_x9_callback
            .map(|callback| (callback)(self.context))
    }

    /// X9
    ///
    /// Entry 9 range.
    fn set_repeating_x9(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x9_callback {
            (callback)(value, self.context);
        };
    }

    /// Y9
    ///
    /// Entry 9 target.
    fn repeating_y9(&self) -> Option<i32> {
        self.repeating_y9_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y9
    ///
    /// Entry 9 target.
    fn set_repeating_y9(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y9_callback {
            (callback)(value, self.context);
        };
    }

    /// X10
    ///
    /// Entry 10 range.
    fn repeating_x10(&self) -> Option<i32> {
        self.repeating_x10_callback
            .map(|callback| (callback)(self.context))
    }

    /// X10
    ///
    /// Entry 10 range.
    fn set_repeating_x10(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_x10_callback {
            (callback)(value, self.context);
        };
    }

    /// Y10
    ///
    /// Entry 10 target.
    fn repeating_y10(&self) -> Option<i32> {
        self.repeating_y10_callback
            .map(|callback| (callback)(self.context))
    }

    /// Y10
    ///
    /// Entry 10 target.
    fn set_repeating_y10(&mut self, value: i32) {
        if let Some(callback) = self.set_repeating_y10_callback {
            (callback)(value, self.context);
        };
    }

    /// Nam
    ///
    /// Optional description for schedule.
    fn repeating_nam(&self) -> Option<&CStr> {
        self.repeating_nam_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Nam
    ///
    /// Optional description for schedule.
    fn set_repeating_nam(&mut self, value: &CStr) {
        if let Some(callback) = self.set_repeating_nam_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// WinTms
    ///
    /// Time window for schedule entry change.
    fn repeating_win_tms(&self) -> Option<u16> {
        self.repeating_win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WinTms
    ///
    /// Time window for schedule entry change.
    fn set_repeating_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_repeating_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current target to new target.
    fn repeating_rmp_tms(&self) -> Option<u16> {
        self.repeating_rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current target to new target.
    fn set_repeating_rmp_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_repeating_rmp_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// ActIndx
    ///
    /// Index of active entry in the active schedule.
    fn repeating_act_indx(&self) -> u16 {
        (self.repeating_act_indx_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model133StatefulAdapter {
    act_schd: u32,
    mod_ena: u16,
    n_schd: u16,
    n_pts: u16,
    repeating_act_pts: u16,
    repeating_str_tms: u32,
    repeating_rep_per: u16,
    repeating_schd_typ: IntvTyp,
    repeating_x_typ: XTyp,
    repeating_x_sf: u16,
    repeating_y_typ: YTyp,
    repeating_y_sf: u16,
    repeating_x1: i32,
    repeating_y1: i32,
    repeating_x2: i32,
    repeating_y2: i32,
    repeating_x3: i32,
    repeating_y3: i32,
    repeating_x4: i32,
    repeating_y4: i32,
    repeating_x5: i32,
    repeating_y5: i32,
    repeating_x6: i32,
    repeating_y6: i32,
    repeating_x7: i32,
    repeating_y7: i32,
    repeating_x8: i32,
    repeating_y8: i32,
    repeating_x9: i32,
    repeating_y9: i32,
    repeating_x10: i32,
    repeating_y10: i32,
    repeating_nam: [c_char; 16],
    repeating_win_tms: u16,
    repeating_rmp_tms: u16,
    repeating_act_indx: u16,
}

impl ModelAdapter for Model133StatefulAdapter {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn act_schd(&self) -> u32 {
        self.act_schd
    }

    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn set_act_schd(&mut self, value: u32) {
        self.act_schd = value;
    }

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    fn n_schd(&self) -> u16 {
        self.n_schd
    }

    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    fn n_pts(&self) -> u16 {
        self.n_pts
    }

    /// ActPts
    ///
    /// Number of active entries in schedule.
    fn repeating_act_pts(&self) -> u16 {
        self.repeating_act_pts
    }

    /// ActPts
    ///
    /// Number of active entries in schedule.
    fn set_repeating_act_pts(&mut self, value: u16) {
        self.repeating_act_pts = value;
    }

    /// StrTms
    ///
    /// Schedule start in seconds since 2000 JAN 01 00:00:00 UTC.
    fn repeating_str_tms(&self) -> u32 {
        self.repeating_str_tms
    }

    /// StrTms
    ///
    /// Schedule start in seconds since 2000 JAN 01 00:00:00 UTC.
    fn set_repeating_str_tms(&mut self, value: u32) {
        self.repeating_str_tms = value;
    }

    /// RepPer
    ///
    /// The repetition count for time-based schedules (0=repeat forever)
    fn repeating_rep_per(&self) -> u16 {
        self.repeating_rep_per
    }

    /// RepPer
    ///
    /// The repetition count for time-based schedules (0=repeat forever)
    fn set_repeating_rep_per(&mut self, value: u16) {
        self.repeating_rep_per = value;
    }

    /// SchdTyp
    ///
    /// The repetition frequency for time-based schedules: no repeat=0
    fn repeating_schd_typ(&self) -> IntvTyp {
        self.repeating_schd_typ
    }

    /// SchdTyp
    ///
    /// The repetition frequency for time-based schedules: no repeat=0
    fn set_repeating_schd_typ(&mut self, value: IntvTyp) {
        self.repeating_schd_typ = value;
    }

    /// XTyp
    ///
    /// The meaning of the X-values in the array.
    fn repeating_x_typ(&self) -> XTyp {
        self.repeating_x_typ
    }

    /// XTyp
    ///
    /// The meaning of the X-values in the array.
    fn set_repeating_x_typ(&mut self, value: XTyp) {
        self.repeating_x_typ = value;
    }

    /// X_SF
    ///
    /// Scale factor for schedule range values.
    fn repeating_x_sf(&self) -> u16 {
        self.repeating_x_sf
    }

    /// X_SF
    ///
    /// Scale factor for schedule range values.
    fn set_repeating_x_sf(&mut self, value: u16) {
        self.repeating_x_sf = value;
    }

    /// YTyp
    ///
    /// The meaning of the Y-values in the array.
    fn repeating_y_typ(&self) -> YTyp {
        self.repeating_y_typ
    }

    /// YTyp
    ///
    /// The meaning of the Y-values in the array.
    fn set_repeating_y_typ(&mut self, value: YTyp) {
        self.repeating_y_typ = value;
    }

    /// Y_SF
    ///
    /// Scale factor for schedule target values.
    fn repeating_y_sf(&self) -> u16 {
        self.repeating_y_sf
    }

    /// Y_SF
    ///
    /// Scale factor for schedule target values.
    fn set_repeating_y_sf(&mut self, value: u16) {
        self.repeating_y_sf = value;
    }

    /// X1
    ///
    /// Entry 1 range.
    fn repeating_x1(&self) -> i32 {
        self.repeating_x1
    }

    /// X1
    ///
    /// Entry 1 range.
    fn set_repeating_x1(&mut self, value: i32) {
        self.repeating_x1 = value;
    }

    /// Y1
    ///
    /// Entry 1 target.
    fn repeating_y1(&self) -> i32 {
        self.repeating_y1
    }

    /// Y1
    ///
    /// Entry 1 target.
    fn set_repeating_y1(&mut self, value: i32) {
        self.repeating_y1 = value;
    }

    /// X2
    ///
    /// Entry 2 range.
    fn repeating_x2(&self) -> Option<i32> {
        Some(self.repeating_x2)
    }

    /// X2
    ///
    /// Entry 2 range.
    fn set_repeating_x2(&mut self, value: i32) {
        self.repeating_x2 = value;
    }

    /// Y2
    ///
    /// Entry 2 target.
    fn repeating_y2(&self) -> Option<i32> {
        Some(self.repeating_y2)
    }

    /// Y2
    ///
    /// Entry 2 target.
    fn set_repeating_y2(&mut self, value: i32) {
        self.repeating_y2 = value;
    }

    /// X3
    ///
    /// Entry 3 range.
    fn repeating_x3(&self) -> Option<i32> {
        Some(self.repeating_x3)
    }

    /// X3
    ///
    /// Entry 3 range.
    fn set_repeating_x3(&mut self, value: i32) {
        self.repeating_x3 = value;
    }

    /// Y3
    ///
    /// Entry 3 target.
    fn repeating_y3(&self) -> Option<i32> {
        Some(self.repeating_y3)
    }

    /// Y3
    ///
    /// Entry 3 target.
    fn set_repeating_y3(&mut self, value: i32) {
        self.repeating_y3 = value;
    }

    /// X4
    ///
    /// Entry 4 range.
    fn repeating_x4(&self) -> Option<i32> {
        Some(self.repeating_x4)
    }

    /// X4
    ///
    /// Entry 4 range.
    fn set_repeating_x4(&mut self, value: i32) {
        self.repeating_x4 = value;
    }

    /// Y4
    ///
    /// Entry 4 target.
    fn repeating_y4(&self) -> Option<i32> {
        Some(self.repeating_y4)
    }

    /// Y4
    ///
    /// Entry 4 target.
    fn set_repeating_y4(&mut self, value: i32) {
        self.repeating_y4 = value;
    }

    /// X5
    ///
    /// Entry 15range.
    fn repeating_x5(&self) -> Option<i32> {
        Some(self.repeating_x5)
    }

    /// X5
    ///
    /// Entry 15range.
    fn set_repeating_x5(&mut self, value: i32) {
        self.repeating_x5 = value;
    }

    /// Y5
    ///
    /// Entry 5 target.
    fn repeating_y5(&self) -> Option<i32> {
        Some(self.repeating_y5)
    }

    /// Y5
    ///
    /// Entry 5 target.
    fn set_repeating_y5(&mut self, value: i32) {
        self.repeating_y5 = value;
    }

    /// X6
    ///
    /// Entry 6 range.
    fn repeating_x6(&self) -> Option<i32> {
        Some(self.repeating_x6)
    }

    /// X6
    ///
    /// Entry 6 range.
    fn set_repeating_x6(&mut self, value: i32) {
        self.repeating_x6 = value;
    }

    /// Y6
    ///
    /// Entry 6 target.
    fn repeating_y6(&self) -> Option<i32> {
        Some(self.repeating_y6)
    }

    /// Y6
    ///
    /// Entry 6 target.
    fn set_repeating_y6(&mut self, value: i32) {
        self.repeating_y6 = value;
    }

    /// X7
    ///
    /// Entry 7 range.
    fn repeating_x7(&self) -> Option<i32> {
        Some(self.repeating_x7)
    }

    /// X7
    ///
    /// Entry 7 range.
    fn set_repeating_x7(&mut self, value: i32) {
        self.repeating_x7 = value;
    }

    /// Y7
    ///
    /// Entry 7 target.
    fn repeating_y7(&self) -> Option<i32> {
        Some(self.repeating_y7)
    }

    /// Y7
    ///
    /// Entry 7 target.
    fn set_repeating_y7(&mut self, value: i32) {
        self.repeating_y7 = value;
    }

    /// X8
    ///
    /// Entry 8 range.
    fn repeating_x8(&self) -> Option<i32> {
        Some(self.repeating_x8)
    }

    /// X8
    ///
    /// Entry 8 range.
    fn set_repeating_x8(&mut self, value: i32) {
        self.repeating_x8 = value;
    }

    /// Y8
    ///
    /// Entry 8 target.
    fn repeating_y8(&self) -> Option<i32> {
        Some(self.repeating_y8)
    }

    /// Y8
    ///
    /// Entry 8 target.
    fn set_repeating_y8(&mut self, value: i32) {
        self.repeating_y8 = value;
    }

    /// X9
    ///
    /// Entry 9 range.
    fn repeating_x9(&self) -> Option<i32> {
        Some(self.repeating_x9)
    }

    /// X9
    ///
    /// Entry 9 range.
    fn set_repeating_x9(&mut self, value: i32) {
        self.repeating_x9 = value;
    }

    /// Y9
    ///
    /// Entry 9 target.
    fn repeating_y9(&self) -> Option<i32> {
        Some(self.repeating_y9)
    }

    /// Y9
    ///
    /// Entry 9 target.
    fn set_repeating_y9(&mut self, value: i32) {
        self.repeating_y9 = value;
    }

    /// X10
    ///
    /// Entry 10 range.
    fn repeating_x10(&self) -> Option<i32> {
        Some(self.repeating_x10)
    }

    /// X10
    ///
    /// Entry 10 range.
    fn set_repeating_x10(&mut self, value: i32) {
        self.repeating_x10 = value;
    }

    /// Y10
    ///
    /// Entry 10 target.
    fn repeating_y10(&self) -> Option<i32> {
        Some(self.repeating_y10)
    }

    /// Y10
    ///
    /// Entry 10 target.
    fn set_repeating_y10(&mut self, value: i32) {
        self.repeating_y10 = value;
    }

    /// Nam
    ///
    /// Optional description for schedule.
    fn repeating_nam(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.repeating_nam.as_ptr()) })
    }

    /// Nam
    ///
    /// Optional description for schedule.
    fn set_repeating_nam(&mut self, value: &CStr) {
        for (dest, src) in self
            .repeating_nam
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// WinTms
    ///
    /// Time window for schedule entry change.
    fn repeating_win_tms(&self) -> Option<u16> {
        Some(self.repeating_win_tms)
    }

    /// WinTms
    ///
    /// Time window for schedule entry change.
    fn set_repeating_win_tms(&mut self, value: u16) {
        self.repeating_win_tms = value;
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current target to new target.
    fn repeating_rmp_tms(&self) -> Option<u16> {
        Some(self.repeating_rmp_tms)
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current target to new target.
    fn set_repeating_rmp_tms(&mut self, value: u16) {
        self.repeating_rmp_tms = value;
    }

    /// ActIndx
    ///
    /// Index of active entry in the active schedule.
    fn repeating_act_indx(&self) -> u16 {
        self.repeating_act_indx
    }
}
