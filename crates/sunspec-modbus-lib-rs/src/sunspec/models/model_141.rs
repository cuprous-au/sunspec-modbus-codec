use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 62;

pub static POINTS: [ReadablePoint; 55] = [
    ReadablePoint {
        reference: PointReference::Static { value: 141 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::ActCrv,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::ModEna,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::WinTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::RvrtTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::RmpTms,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 { point: Point::NCrv },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model141 { point: Point::NPt },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::TmsSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model141 { point: Point::HzSf },
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
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveActPt,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms1,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz1,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms2,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz2,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms3,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz3,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms4,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz4,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms5,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz5,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms6,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz6,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms7,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz7,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms8,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz8,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms9,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz9,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms10,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz10,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms11,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz11,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms12,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz12,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms13,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz13,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms14,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz14,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms15,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz15,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms16,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz16,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms17,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz17,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms18,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz18,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms19,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz19,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveTms20,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveHz20,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
            point: Point::CurveCrvNam,
        },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model141 {
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
    TmsSf,
    HzSf,
    CurveActPt,
    CurveTms1,
    CurveHz1,
    CurveTms2,
    CurveHz2,
    CurveTms3,
    CurveHz3,
    CurveTms4,
    CurveHz4,
    CurveTms5,
    CurveHz5,
    CurveTms6,
    CurveHz6,
    CurveTms7,
    CurveHz7,
    CurveTms8,
    CurveHz8,
    CurveTms9,
    CurveHz9,
    CurveTms10,
    CurveHz10,
    CurveTms11,
    CurveHz11,
    CurveTms12,
    CurveHz12,
    CurveTms13,
    CurveHz13,
    CurveTms14,
    CurveHz14,
    CurveTms15,
    CurveHz15,
    CurveTms16,
    CurveHz16,
    CurveTms17,
    CurveHz17,
    CurveTms18,
    CurveHz18,
    CurveTms19,
    CurveHz19,
    CurveTms20,
    CurveHz20,
    CurveCrvNam,
    CurveReadOnly,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    62
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
        Point::HzSf => {
            buffer::write_u16(model.hz_sf(), buffer);
        }
        Point::CurveActPt => {
            buffer::write_u16(model.curve_act_pt(), buffer);
        }
        Point::CurveTms1 => {
            buffer::write_u16(model.curve_tms1(), buffer);
        }
        Point::CurveHz1 => {
            buffer::write_u16(model.curve_hz1(), buffer);
        }
        Point::CurveTms2 => {
            if let Some(value) = model.curve_tms2() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurveHz2 => {
            if let Some(value) = model.curve_hz2() {
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
        Point::CurveHz3 => {
            if let Some(value) = model.curve_hz3() {
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
        Point::CurveHz4 => {
            if let Some(value) = model.curve_hz4() {
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
        Point::CurveHz5 => {
            if let Some(value) = model.curve_hz5() {
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
        Point::CurveHz6 => {
            if let Some(value) = model.curve_hz6() {
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
        Point::CurveHz7 => {
            if let Some(value) = model.curve_hz7() {
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
        Point::CurveHz8 => {
            if let Some(value) = model.curve_hz8() {
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
        Point::CurveHz9 => {
            if let Some(value) = model.curve_hz9() {
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
        Point::CurveHz10 => {
            if let Some(value) = model.curve_hz10() {
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
        Point::CurveHz11 => {
            if let Some(value) = model.curve_hz11() {
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
        Point::CurveHz12 => {
            if let Some(value) = model.curve_hz12() {
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
        Point::CurveHz13 => {
            if let Some(value) = model.curve_hz13() {
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
        Point::CurveHz14 => {
            if let Some(value) = model.curve_hz14() {
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
        Point::CurveHz15 => {
            if let Some(value) = model.curve_hz15() {
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
        Point::CurveHz16 => {
            if let Some(value) = model.curve_hz16() {
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
        Point::CurveHz17 => {
            if let Some(value) = model.curve_hz17() {
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
        Point::CurveHz18 => {
            if let Some(value) = model.curve_hz18() {
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
        Point::CurveHz19 => {
            if let Some(value) = model.curve_hz19() {
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
        Point::CurveHz20 => {
            if let Some(value) = model.curve_hz20() {
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

    /// Hz1
    ///
    /// Point 1 must remain connected frequency.
    fn curve_hz1(&self) -> u16;

    /// Hz1
    ///
    /// Point 1 must remain connected frequency.
    fn set_curve_hz1(&mut self, value: u16);

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

    /// Hz2
    ///
    /// Point 2 must remain connected frequency.
    fn curve_hz2(&self) -> Option<u16> {
        None
    }

    /// Hz2
    ///
    /// Point 2 must remain connected frequency.
    fn set_curve_hz2(&mut self, value: u16) {}

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

    /// Hz3
    ///
    /// Point 3 must remain connected frequency.
    fn curve_hz3(&self) -> Option<u16> {
        None
    }

    /// Hz3
    ///
    /// Point 3 must remain connected frequency.
    fn set_curve_hz3(&mut self, value: u16) {}

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

    /// Hz4
    ///
    /// Point 4 must remain connected frequency.
    fn curve_hz4(&self) -> Option<u16> {
        None
    }

    /// Hz4
    ///
    /// Point 4 must remain connected frequency.
    fn set_curve_hz4(&mut self, value: u16) {}

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

    /// Hz5
    ///
    /// Point 5 must remain connected frequency.
    fn curve_hz5(&self) -> Option<u16> {
        None
    }

    /// Hz5
    ///
    /// Point 5 must remain connected frequency.
    fn set_curve_hz5(&mut self, value: u16) {}

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

    /// Hz6
    ///
    /// Point 6 must remain connected frequency.
    fn curve_hz6(&self) -> Option<u16> {
        None
    }

    /// Hz6
    ///
    /// Point 6 must remain connected frequency.
    fn set_curve_hz6(&mut self, value: u16) {}

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

    /// Hz7
    ///
    /// Point 7 must remain connected frequency.
    fn curve_hz7(&self) -> Option<u16> {
        None
    }

    /// Hz7
    ///
    /// Point 7 must remain connected frequency.
    fn set_curve_hz7(&mut self, value: u16) {}

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

    /// Hz8
    ///
    /// Point 8 must remain connected frequency.
    fn curve_hz8(&self) -> Option<u16> {
        None
    }

    /// Hz8
    ///
    /// Point 8 must remain connected frequency.
    fn set_curve_hz8(&mut self, value: u16) {}

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

    /// Hz9
    ///
    /// Point 9 must remain connected frequency.
    fn curve_hz9(&self) -> Option<u16> {
        None
    }

    /// Hz9
    ///
    /// Point 9 must remain connected frequency.
    fn set_curve_hz9(&mut self, value: u16) {}

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

    /// Hz10
    ///
    /// Point 10 must remain connected frequency.
    fn curve_hz10(&self) -> Option<u16> {
        None
    }

    /// Hz10
    ///
    /// Point 10 must remain connected frequency.
    fn set_curve_hz10(&mut self, value: u16) {}

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

    /// Hz11
    ///
    /// Point 11 must remain connected frequency.
    fn curve_hz11(&self) -> Option<u16> {
        None
    }

    /// Hz11
    ///
    /// Point 11 must remain connected frequency.
    fn set_curve_hz11(&mut self, value: u16) {}

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

    /// Hz12
    ///
    /// Point 12 must remain connected frequency.
    fn curve_hz12(&self) -> Option<u16> {
        None
    }

    /// Hz12
    ///
    /// Point 12 must remain connected frequency.
    fn set_curve_hz12(&mut self, value: u16) {}

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

    /// Hz13
    ///
    /// Point 13 must remain connected frequency.
    fn curve_hz13(&self) -> Option<u16> {
        None
    }

    /// Hz13
    ///
    /// Point 13 must remain connected frequency.
    fn set_curve_hz13(&mut self, value: u16) {}

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

    /// Hz14
    ///
    /// Point 14 must remain connected frequency.
    fn curve_hz14(&self) -> Option<u16> {
        None
    }

    /// Hz14
    ///
    /// Point 14 must remain connected frequency.
    fn set_curve_hz14(&mut self, value: u16) {}

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

    /// Hz15
    ///
    /// Point 15 must remain connected frequency.
    fn curve_hz15(&self) -> Option<u16> {
        None
    }

    /// Hz15
    ///
    /// Point 15 must remain connected frequency.
    fn set_curve_hz15(&mut self, value: u16) {}

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

    /// Hz16
    ///
    /// Point 16 must remain connected frequency.
    fn curve_hz16(&self) -> Option<u16> {
        None
    }

    /// Hz16
    ///
    /// Point 16 must remain connected frequency.
    fn set_curve_hz16(&mut self, value: u16) {}

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

    /// Hz17
    ///
    /// Point 17 must remain connected frequency.
    fn curve_hz17(&self) -> Option<u16> {
        None
    }

    /// Hz17
    ///
    /// Point 17 must remain connected frequency.
    fn set_curve_hz17(&mut self, value: u16) {}

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

    /// Hz18
    ///
    /// Point 18 must remain connected frequency.
    fn curve_hz18(&self) -> Option<u16> {
        None
    }

    /// Hz18
    ///
    /// Point 18 must remain connected frequency.
    fn set_curve_hz18(&mut self, value: u16) {}

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

    /// Hz19
    ///
    /// Point 19 must remain connected frequency.
    fn curve_hz19(&self) -> Option<u16> {
        None
    }

    /// Hz19
    ///
    /// Point 19 must remain connected frequency.
    fn set_curve_hz19(&mut self, value: u16) {}

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

    /// Hz20
    ///
    /// Point 20 must remain connected frequency.
    fn curve_hz20(&self) -> Option<u16> {
        None
    }

    /// Hz20
    ///
    /// Point 20 must remain connected frequency.
    fn set_curve_hz20(&mut self, value: u16) {}

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
pub struct Model141CallbackAdapter {
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
    hz_sf_callback: extern "C" fn(*const c_void) -> u16,
    curve_act_pt_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_act_pt_callback: extern "C" fn(u16, *mut c_void),
    curve_tms1_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_tms1_callback: extern "C" fn(u16, *mut c_void),
    curve_hz1_callback: extern "C" fn(*const c_void) -> u16,
    set_curve_hz1_callback: extern "C" fn(u16, *mut c_void),
    curve_tms2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms2_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz2_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms3_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz3_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms4_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz4_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz4_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms5_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz5_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz5_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms6_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms6_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz6_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz6_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms7_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms7_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz7_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz7_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms8_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms8_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz8_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz8_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms9_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms9_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz9_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz9_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms10_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms10_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz10_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz10_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms11_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms11_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz11_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz11_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms12_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms12_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz12_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz12_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms13_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms13_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz13_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz13_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms14_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms14_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz14_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz14_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms15_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms15_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz15_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz15_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms16_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz16_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz16_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms17_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms17_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz17_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz17_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms18_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms18_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz18_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz18_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms19_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms19_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz19_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz19_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_tms20_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_tms20_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_hz20_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_curve_hz20_callback: Option<extern "C" fn(u16, *mut c_void)>,
    curve_crv_nam_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_curve_crv_nam_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    curve_read_only_callback: extern "C" fn(*const c_void) -> ReadOnly,
}

impl ModelAdapter for Model141CallbackAdapter {
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
    /// LHzRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Setting is ignored for LFRT controls.
    fn win_tms(&self) -> Option<u16> {
        self.win_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_win_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_win_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Setting is ignored for LFRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        self.rvrt_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {
        if let Some(callback) = self.set_rvrt_tms_callback {
            (callback)(value, self.context);
        };
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LFRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        self.rmp_tms_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LFRT controls.
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

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16 {
        (self.hz_sf_callback)(self.context)
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

    /// Hz1
    ///
    /// Point 1 must remain connected frequency.
    fn curve_hz1(&self) -> u16 {
        (self.curve_hz1_callback)(self.context)
    }

    /// Hz1
    ///
    /// Point 1 must remain connected frequency.
    fn set_curve_hz1(&mut self, value: u16) {
        (self.set_curve_hz1_callback)(value, self.context);
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

    /// Hz2
    ///
    /// Point 2 must remain connected frequency.
    fn curve_hz2(&self) -> Option<u16> {
        self.curve_hz2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz2
    ///
    /// Point 2 must remain connected frequency.
    fn set_curve_hz2(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz2_callback {
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

    /// Hz3
    ///
    /// Point 3 must remain connected frequency.
    fn curve_hz3(&self) -> Option<u16> {
        self.curve_hz3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz3
    ///
    /// Point 3 must remain connected frequency.
    fn set_curve_hz3(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz3_callback {
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

    /// Hz4
    ///
    /// Point 4 must remain connected frequency.
    fn curve_hz4(&self) -> Option<u16> {
        self.curve_hz4_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz4
    ///
    /// Point 4 must remain connected frequency.
    fn set_curve_hz4(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz4_callback {
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

    /// Hz5
    ///
    /// Point 5 must remain connected frequency.
    fn curve_hz5(&self) -> Option<u16> {
        self.curve_hz5_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz5
    ///
    /// Point 5 must remain connected frequency.
    fn set_curve_hz5(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz5_callback {
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

    /// Hz6
    ///
    /// Point 6 must remain connected frequency.
    fn curve_hz6(&self) -> Option<u16> {
        self.curve_hz6_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz6
    ///
    /// Point 6 must remain connected frequency.
    fn set_curve_hz6(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz6_callback {
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

    /// Hz7
    ///
    /// Point 7 must remain connected frequency.
    fn curve_hz7(&self) -> Option<u16> {
        self.curve_hz7_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz7
    ///
    /// Point 7 must remain connected frequency.
    fn set_curve_hz7(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz7_callback {
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

    /// Hz8
    ///
    /// Point 8 must remain connected frequency.
    fn curve_hz8(&self) -> Option<u16> {
        self.curve_hz8_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz8
    ///
    /// Point 8 must remain connected frequency.
    fn set_curve_hz8(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz8_callback {
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

    /// Hz9
    ///
    /// Point 9 must remain connected frequency.
    fn curve_hz9(&self) -> Option<u16> {
        self.curve_hz9_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz9
    ///
    /// Point 9 must remain connected frequency.
    fn set_curve_hz9(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz9_callback {
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

    /// Hz10
    ///
    /// Point 10 must remain connected frequency.
    fn curve_hz10(&self) -> Option<u16> {
        self.curve_hz10_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz10
    ///
    /// Point 10 must remain connected frequency.
    fn set_curve_hz10(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz10_callback {
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

    /// Hz11
    ///
    /// Point 11 must remain connected frequency.
    fn curve_hz11(&self) -> Option<u16> {
        self.curve_hz11_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz11
    ///
    /// Point 11 must remain connected frequency.
    fn set_curve_hz11(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz11_callback {
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

    /// Hz12
    ///
    /// Point 12 must remain connected frequency.
    fn curve_hz12(&self) -> Option<u16> {
        self.curve_hz12_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz12
    ///
    /// Point 12 must remain connected frequency.
    fn set_curve_hz12(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz12_callback {
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

    /// Hz13
    ///
    /// Point 13 must remain connected frequency.
    fn curve_hz13(&self) -> Option<u16> {
        self.curve_hz13_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz13
    ///
    /// Point 13 must remain connected frequency.
    fn set_curve_hz13(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz13_callback {
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

    /// Hz14
    ///
    /// Point 14 must remain connected frequency.
    fn curve_hz14(&self) -> Option<u16> {
        self.curve_hz14_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz14
    ///
    /// Point 14 must remain connected frequency.
    fn set_curve_hz14(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz14_callback {
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

    /// Hz15
    ///
    /// Point 15 must remain connected frequency.
    fn curve_hz15(&self) -> Option<u16> {
        self.curve_hz15_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz15
    ///
    /// Point 15 must remain connected frequency.
    fn set_curve_hz15(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz15_callback {
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

    /// Hz16
    ///
    /// Point 16 must remain connected frequency.
    fn curve_hz16(&self) -> Option<u16> {
        self.curve_hz16_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz16
    ///
    /// Point 16 must remain connected frequency.
    fn set_curve_hz16(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz16_callback {
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

    /// Hz17
    ///
    /// Point 17 must remain connected frequency.
    fn curve_hz17(&self) -> Option<u16> {
        self.curve_hz17_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz17
    ///
    /// Point 17 must remain connected frequency.
    fn set_curve_hz17(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz17_callback {
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

    /// Hz18
    ///
    /// Point 18 must remain connected frequency.
    fn curve_hz18(&self) -> Option<u16> {
        self.curve_hz18_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz18
    ///
    /// Point 18 must remain connected frequency.
    fn set_curve_hz18(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz18_callback {
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

    /// Hz19
    ///
    /// Point 19 must remain connected frequency.
    fn curve_hz19(&self) -> Option<u16> {
        self.curve_hz19_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz19
    ///
    /// Point 19 must remain connected frequency.
    fn set_curve_hz19(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz19_callback {
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

    /// Hz20
    ///
    /// Point 20 must remain connected frequency.
    fn curve_hz20(&self) -> Option<u16> {
        self.curve_hz20_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz20
    ///
    /// Point 20 must remain connected frequency.
    fn set_curve_hz20(&mut self, value: u16) {
        if let Some(callback) = self.set_curve_hz20_callback {
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
pub struct Model141StatefulAdapter {
    act_crv: u16,
    mod_ena: u16,
    win_tms: u16,
    rvrt_tms: u16,
    rmp_tms: u16,
    n_crv: u16,
    n_pt: u16,
    tms_sf: u16,
    hz_sf: u16,
    curve_act_pt: u16,
    curve_tms1: u16,
    curve_hz1: u16,
    curve_tms2: u16,
    curve_hz2: u16,
    curve_tms3: u16,
    curve_hz3: u16,
    curve_tms4: u16,
    curve_hz4: u16,
    curve_tms5: u16,
    curve_hz5: u16,
    curve_tms6: u16,
    curve_hz6: u16,
    curve_tms7: u16,
    curve_hz7: u16,
    curve_tms8: u16,
    curve_hz8: u16,
    curve_tms9: u16,
    curve_hz9: u16,
    curve_tms10: u16,
    curve_hz10: u16,
    curve_tms11: u16,
    curve_hz11: u16,
    curve_tms12: u16,
    curve_hz12: u16,
    curve_tms13: u16,
    curve_hz13: u16,
    curve_tms14: u16,
    curve_hz14: u16,
    curve_tms15: u16,
    curve_hz15: u16,
    curve_tms16: u16,
    curve_hz16: u16,
    curve_tms17: u16,
    curve_hz17: u16,
    curve_tms18: u16,
    curve_hz18: u16,
    curve_tms19: u16,
    curve_hz19: u16,
    curve_tms20: u16,
    curve_hz20: u16,
    curve_crv_nam: [c_char; 16],
    curve_read_only: ReadOnly,
}

impl ModelAdapter for Model141StatefulAdapter {
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
    /// LHzRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// LHzRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Setting is ignored for LFRT controls.
    fn win_tms(&self) -> Option<u16> {
        Some(self.win_tms)
    }

    /// WinTms
    ///
    /// Time window for LFRT change.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_win_tms(&mut self, value: u16) {
        self.win_tms = value;
    }

    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Setting is ignored for LFRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        Some(self.rvrt_tms)
    }

    /// RvrtTms
    ///
    /// Timeout period for LFRT curve selection.
    ///
    /// Setting is ignored for LFRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {
        self.rvrt_tms = value;
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LFRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        Some(self.rmp_tms)
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LFRT controls.
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

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16 {
        self.hz_sf
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

    /// Hz1
    ///
    /// Point 1 must remain connected frequency.
    fn curve_hz1(&self) -> u16 {
        self.curve_hz1
    }

    /// Hz1
    ///
    /// Point 1 must remain connected frequency.
    fn set_curve_hz1(&mut self, value: u16) {
        self.curve_hz1 = value;
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

    /// Hz2
    ///
    /// Point 2 must remain connected frequency.
    fn curve_hz2(&self) -> Option<u16> {
        Some(self.curve_hz2)
    }

    /// Hz2
    ///
    /// Point 2 must remain connected frequency.
    fn set_curve_hz2(&mut self, value: u16) {
        self.curve_hz2 = value;
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

    /// Hz3
    ///
    /// Point 3 must remain connected frequency.
    fn curve_hz3(&self) -> Option<u16> {
        Some(self.curve_hz3)
    }

    /// Hz3
    ///
    /// Point 3 must remain connected frequency.
    fn set_curve_hz3(&mut self, value: u16) {
        self.curve_hz3 = value;
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

    /// Hz4
    ///
    /// Point 4 must remain connected frequency.
    fn curve_hz4(&self) -> Option<u16> {
        Some(self.curve_hz4)
    }

    /// Hz4
    ///
    /// Point 4 must remain connected frequency.
    fn set_curve_hz4(&mut self, value: u16) {
        self.curve_hz4 = value;
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

    /// Hz5
    ///
    /// Point 5 must remain connected frequency.
    fn curve_hz5(&self) -> Option<u16> {
        Some(self.curve_hz5)
    }

    /// Hz5
    ///
    /// Point 5 must remain connected frequency.
    fn set_curve_hz5(&mut self, value: u16) {
        self.curve_hz5 = value;
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

    /// Hz6
    ///
    /// Point 6 must remain connected frequency.
    fn curve_hz6(&self) -> Option<u16> {
        Some(self.curve_hz6)
    }

    /// Hz6
    ///
    /// Point 6 must remain connected frequency.
    fn set_curve_hz6(&mut self, value: u16) {
        self.curve_hz6 = value;
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

    /// Hz7
    ///
    /// Point 7 must remain connected frequency.
    fn curve_hz7(&self) -> Option<u16> {
        Some(self.curve_hz7)
    }

    /// Hz7
    ///
    /// Point 7 must remain connected frequency.
    fn set_curve_hz7(&mut self, value: u16) {
        self.curve_hz7 = value;
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

    /// Hz8
    ///
    /// Point 8 must remain connected frequency.
    fn curve_hz8(&self) -> Option<u16> {
        Some(self.curve_hz8)
    }

    /// Hz8
    ///
    /// Point 8 must remain connected frequency.
    fn set_curve_hz8(&mut self, value: u16) {
        self.curve_hz8 = value;
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

    /// Hz9
    ///
    /// Point 9 must remain connected frequency.
    fn curve_hz9(&self) -> Option<u16> {
        Some(self.curve_hz9)
    }

    /// Hz9
    ///
    /// Point 9 must remain connected frequency.
    fn set_curve_hz9(&mut self, value: u16) {
        self.curve_hz9 = value;
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

    /// Hz10
    ///
    /// Point 10 must remain connected frequency.
    fn curve_hz10(&self) -> Option<u16> {
        Some(self.curve_hz10)
    }

    /// Hz10
    ///
    /// Point 10 must remain connected frequency.
    fn set_curve_hz10(&mut self, value: u16) {
        self.curve_hz10 = value;
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

    /// Hz11
    ///
    /// Point 11 must remain connected frequency.
    fn curve_hz11(&self) -> Option<u16> {
        Some(self.curve_hz11)
    }

    /// Hz11
    ///
    /// Point 11 must remain connected frequency.
    fn set_curve_hz11(&mut self, value: u16) {
        self.curve_hz11 = value;
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

    /// Hz12
    ///
    /// Point 12 must remain connected frequency.
    fn curve_hz12(&self) -> Option<u16> {
        Some(self.curve_hz12)
    }

    /// Hz12
    ///
    /// Point 12 must remain connected frequency.
    fn set_curve_hz12(&mut self, value: u16) {
        self.curve_hz12 = value;
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

    /// Hz13
    ///
    /// Point 13 must remain connected frequency.
    fn curve_hz13(&self) -> Option<u16> {
        Some(self.curve_hz13)
    }

    /// Hz13
    ///
    /// Point 13 must remain connected frequency.
    fn set_curve_hz13(&mut self, value: u16) {
        self.curve_hz13 = value;
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

    /// Hz14
    ///
    /// Point 14 must remain connected frequency.
    fn curve_hz14(&self) -> Option<u16> {
        Some(self.curve_hz14)
    }

    /// Hz14
    ///
    /// Point 14 must remain connected frequency.
    fn set_curve_hz14(&mut self, value: u16) {
        self.curve_hz14 = value;
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

    /// Hz15
    ///
    /// Point 15 must remain connected frequency.
    fn curve_hz15(&self) -> Option<u16> {
        Some(self.curve_hz15)
    }

    /// Hz15
    ///
    /// Point 15 must remain connected frequency.
    fn set_curve_hz15(&mut self, value: u16) {
        self.curve_hz15 = value;
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

    /// Hz16
    ///
    /// Point 16 must remain connected frequency.
    fn curve_hz16(&self) -> Option<u16> {
        Some(self.curve_hz16)
    }

    /// Hz16
    ///
    /// Point 16 must remain connected frequency.
    fn set_curve_hz16(&mut self, value: u16) {
        self.curve_hz16 = value;
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

    /// Hz17
    ///
    /// Point 17 must remain connected frequency.
    fn curve_hz17(&self) -> Option<u16> {
        Some(self.curve_hz17)
    }

    /// Hz17
    ///
    /// Point 17 must remain connected frequency.
    fn set_curve_hz17(&mut self, value: u16) {
        self.curve_hz17 = value;
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

    /// Hz18
    ///
    /// Point 18 must remain connected frequency.
    fn curve_hz18(&self) -> Option<u16> {
        Some(self.curve_hz18)
    }

    /// Hz18
    ///
    /// Point 18 must remain connected frequency.
    fn set_curve_hz18(&mut self, value: u16) {
        self.curve_hz18 = value;
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

    /// Hz19
    ///
    /// Point 19 must remain connected frequency.
    fn curve_hz19(&self) -> Option<u16> {
        Some(self.curve_hz19)
    }

    /// Hz19
    ///
    /// Point 19 must remain connected frequency.
    fn set_curve_hz19(&mut self, value: u16) {
        self.curve_hz19 = value;
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

    /// Hz20
    ///
    /// Point 20 must remain connected frequency.
    fn curve_hz20(&self) -> Option<u16> {
        Some(self.curve_hz20)
    }

    /// Hz20
    ///
    /// Point 20 must remain connected frequency.
    fn set_curve_hz20(&mut self, value: u16) {
        self.curve_hz20 = value;
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
