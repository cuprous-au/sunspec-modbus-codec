use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 91;

pub static POINTS: [ReadablePoint; 90] = [
    ReadablePoint {
        reference: PointReference::Static { value: 5 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::X },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Offset1,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Value1,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off2 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val2 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off3 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val3 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off4 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val4 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off5 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val5 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off6 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val6 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off7 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val7 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off9 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val9 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off10,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val10,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off11,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val11,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off12,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val12,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off13,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val13,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off14,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val14,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off15,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val15,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off16,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val16,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off17,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val17,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off18,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val18,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off19,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val19,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off20,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val20,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off21,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val21,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off22,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val22,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off23,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val23,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off24,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val24,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off25,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val25,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off26,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val26,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off27,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val27,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off28,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val28,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off29,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val29,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off30,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val30,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off31,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val31,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off32,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val32,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off33,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val33,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off34,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val34,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off35,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val35,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off36,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val36,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off37,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val37,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off38,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val38,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off39,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val39,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Off40,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Val40,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Timestamp,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Milliseconds,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Sequence,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Role },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::Algorithm,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::N },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 {
            point: Point::RepeatingDs,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    X,
    Offset1,
    Value1,
    Off2,
    Val2,
    Off3,
    Val3,
    Off4,
    Val4,
    Off5,
    Val5,
    Off6,
    Val6,
    Off7,
    Val7,
    Off8,
    Val8,
    Off9,
    Val9,
    Off10,
    Val10,
    Off11,
    Val11,
    Off12,
    Val12,
    Off13,
    Val13,
    Off14,
    Val14,
    Off15,
    Val15,
    Off16,
    Val16,
    Off17,
    Val17,
    Off18,
    Val18,
    Off19,
    Val19,
    Off20,
    Val20,
    Off21,
    Val21,
    Off22,
    Val22,
    Off23,
    Val23,
    Off24,
    Val24,
    Off25,
    Val25,
    Off26,
    Val26,
    Off27,
    Val27,
    Off28,
    Val28,
    Off29,
    Val29,
    Off30,
    Val30,
    Off31,
    Val31,
    Off32,
    Val32,
    Off33,
    Val33,
    Off34,
    Val34,
    Off35,
    Val35,
    Off36,
    Val36,
    Off37,
    Val37,
    Off38,
    Val38,
    Off39,
    Val39,
    Off40,
    Val40,
    Timestamp,
    Milliseconds,
    Sequence,
    Role,
    Algorithm,
    N,
    RepeatingDs,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    91
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
        Point::X => {
            buffer::write_u16(model.x(), buffer);
        }
        Point::Offset1 => {
            buffer::write_u16(model.offset1(), buffer);
        }
        Point::Value1 => {
            buffer::write_u16(model.value1(), buffer);
        }
        Point::Off2 => {
            buffer::write_u16(model.off2(), buffer);
        }
        Point::Val2 => {
            buffer::write_u16(model.val2(), buffer);
        }
        Point::Off3 => {
            buffer::write_u16(model.off3(), buffer);
        }
        Point::Val3 => {
            buffer::write_u16(model.val3(), buffer);
        }
        Point::Off4 => {
            buffer::write_u16(model.off4(), buffer);
        }
        Point::Val4 => {
            buffer::write_u16(model.val4(), buffer);
        }
        Point::Off5 => {
            buffer::write_u16(model.off5(), buffer);
        }
        Point::Val5 => {
            buffer::write_u16(model.val5(), buffer);
        }
        Point::Off6 => {
            buffer::write_u16(model.off6(), buffer);
        }
        Point::Val6 => {
            buffer::write_u16(model.val6(), buffer);
        }
        Point::Off7 => {
            buffer::write_u16(model.off7(), buffer);
        }
        Point::Val7 => {
            buffer::write_u16(model.val7(), buffer);
        }
        Point::Off8 => {
            buffer::write_u16(model.off8(), buffer);
        }
        Point::Val8 => {
            buffer::write_u16(model.val8(), buffer);
        }
        Point::Off9 => {
            buffer::write_u16(model.off9(), buffer);
        }
        Point::Val9 => {
            buffer::write_u16(model.val9(), buffer);
        }
        Point::Off10 => {
            buffer::write_u16(model.off10(), buffer);
        }
        Point::Val10 => {
            buffer::write_u16(model.val10(), buffer);
        }
        Point::Off11 => {
            buffer::write_u16(model.off11(), buffer);
        }
        Point::Val11 => {
            buffer::write_u16(model.val11(), buffer);
        }
        Point::Off12 => {
            buffer::write_u16(model.off12(), buffer);
        }
        Point::Val12 => {
            buffer::write_u16(model.val12(), buffer);
        }
        Point::Off13 => {
            buffer::write_u16(model.off13(), buffer);
        }
        Point::Val13 => {
            buffer::write_u16(model.val13(), buffer);
        }
        Point::Off14 => {
            buffer::write_u16(model.off14(), buffer);
        }
        Point::Val14 => {
            buffer::write_u16(model.val14(), buffer);
        }
        Point::Off15 => {
            buffer::write_u16(model.off15(), buffer);
        }
        Point::Val15 => {
            buffer::write_u16(model.val15(), buffer);
        }
        Point::Off16 => {
            buffer::write_u16(model.off16(), buffer);
        }
        Point::Val16 => {
            buffer::write_u16(model.val16(), buffer);
        }
        Point::Off17 => {
            buffer::write_u16(model.off17(), buffer);
        }
        Point::Val17 => {
            buffer::write_u16(model.val17(), buffer);
        }
        Point::Off18 => {
            buffer::write_u16(model.off18(), buffer);
        }
        Point::Val18 => {
            buffer::write_u16(model.val18(), buffer);
        }
        Point::Off19 => {
            buffer::write_u16(model.off19(), buffer);
        }
        Point::Val19 => {
            buffer::write_u16(model.val19(), buffer);
        }
        Point::Off20 => {
            buffer::write_u16(model.off20(), buffer);
        }
        Point::Val20 => {
            buffer::write_u16(model.val20(), buffer);
        }
        Point::Off21 => {
            buffer::write_u16(model.off21(), buffer);
        }
        Point::Val21 => {
            buffer::write_u16(model.val21(), buffer);
        }
        Point::Off22 => {
            buffer::write_u16(model.off22(), buffer);
        }
        Point::Val22 => {
            buffer::write_u16(model.val22(), buffer);
        }
        Point::Off23 => {
            buffer::write_u16(model.off23(), buffer);
        }
        Point::Val23 => {
            buffer::write_u16(model.val23(), buffer);
        }
        Point::Off24 => {
            buffer::write_u16(model.off24(), buffer);
        }
        Point::Val24 => {
            buffer::write_u16(model.val24(), buffer);
        }
        Point::Off25 => {
            buffer::write_u16(model.off25(), buffer);
        }
        Point::Val25 => {
            buffer::write_u16(model.val25(), buffer);
        }
        Point::Off26 => {
            buffer::write_u16(model.off26(), buffer);
        }
        Point::Val26 => {
            buffer::write_u16(model.val26(), buffer);
        }
        Point::Off27 => {
            buffer::write_u16(model.off27(), buffer);
        }
        Point::Val27 => {
            buffer::write_u16(model.val27(), buffer);
        }
        Point::Off28 => {
            buffer::write_u16(model.off28(), buffer);
        }
        Point::Val28 => {
            buffer::write_u16(model.val28(), buffer);
        }
        Point::Off29 => {
            buffer::write_u16(model.off29(), buffer);
        }
        Point::Val29 => {
            buffer::write_u16(model.val29(), buffer);
        }
        Point::Off30 => {
            buffer::write_u16(model.off30(), buffer);
        }
        Point::Val30 => {
            buffer::write_u16(model.val30(), buffer);
        }
        Point::Off31 => {
            buffer::write_u16(model.off31(), buffer);
        }
        Point::Val31 => {
            buffer::write_u16(model.val31(), buffer);
        }
        Point::Off32 => {
            buffer::write_u16(model.off32(), buffer);
        }
        Point::Val32 => {
            buffer::write_u16(model.val32(), buffer);
        }
        Point::Off33 => {
            buffer::write_u16(model.off33(), buffer);
        }
        Point::Val33 => {
            buffer::write_u16(model.val33(), buffer);
        }
        Point::Off34 => {
            buffer::write_u16(model.off34(), buffer);
        }
        Point::Val34 => {
            buffer::write_u16(model.val34(), buffer);
        }
        Point::Off35 => {
            buffer::write_u16(model.off35(), buffer);
        }
        Point::Val35 => {
            buffer::write_u16(model.val35(), buffer);
        }
        Point::Off36 => {
            buffer::write_u16(model.off36(), buffer);
        }
        Point::Val36 => {
            buffer::write_u16(model.val36(), buffer);
        }
        Point::Off37 => {
            buffer::write_u16(model.off37(), buffer);
        }
        Point::Val37 => {
            buffer::write_u16(model.val37(), buffer);
        }
        Point::Off38 => {
            buffer::write_u16(model.off38(), buffer);
        }
        Point::Val38 => {
            buffer::write_u16(model.val38(), buffer);
        }
        Point::Off39 => {
            buffer::write_u16(model.off39(), buffer);
        }
        Point::Val39 => {
            buffer::write_u16(model.val39(), buffer);
        }
        Point::Off40 => {
            buffer::write_u16(model.off40(), buffer);
        }
        Point::Val40 => {
            buffer::write_u16(model.val40(), buffer);
        }
        Point::Timestamp => {
            buffer::write_u32(model.timestamp(), buffer, offset, limit);
        }
        Point::Milliseconds => {
            buffer::write_u16(model.milliseconds(), buffer);
        }
        Point::Sequence => {
            buffer::write_u16(model.sequence(), buffer);
        }
        Point::Role => {
            buffer::write_u16(model.role(), buffer);
        }
        Point::Algorithm => {
            buffer::write_u16(model.algorithm() as u16, buffer);
        }
        Point::N => {
            buffer::write_u16(model.n(), buffer);
        }
        Point::RepeatingDs => {
            buffer::write_u16(model.repeating_ds(), buffer);
        }
    }
}

pub trait ModelAdapter {
    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn x(&self) -> u16;

    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn set_x(&mut self, value: u16);

    /// Offset1
    ///
    /// Offset of control register to write value to
    fn offset1(&self) -> u16;

    /// Offset1
    ///
    /// Offset of control register to write value to
    fn set_offset1(&mut self, value: u16);

    /// Value1
    ///
    /// Value to write to control register at offset
    fn value1(&self) -> u16;

    /// Value1
    ///
    /// Value to write to control register at offset
    fn set_value1(&mut self, value: u16);

    fn off2(&self) -> u16;

    fn set_off2(&mut self, value: u16);

    fn val2(&self) -> u16;

    fn set_val2(&mut self, value: u16);

    fn off3(&self) -> u16;

    fn set_off3(&mut self, value: u16);

    fn val3(&self) -> u16;

    fn set_val3(&mut self, value: u16);

    fn off4(&self) -> u16;

    fn set_off4(&mut self, value: u16);

    fn val4(&self) -> u16;

    fn set_val4(&mut self, value: u16);

    fn off5(&self) -> u16;

    fn set_off5(&mut self, value: u16);

    fn val5(&self) -> u16;

    fn set_val5(&mut self, value: u16);

    fn off6(&self) -> u16;

    fn set_off6(&mut self, value: u16);

    fn val6(&self) -> u16;

    fn set_val6(&mut self, value: u16);

    fn off7(&self) -> u16;

    fn set_off7(&mut self, value: u16);

    fn val7(&self) -> u16;

    fn set_val7(&mut self, value: u16);

    fn off8(&self) -> u16;

    fn set_off8(&mut self, value: u16);

    fn val8(&self) -> u16;

    fn set_val8(&mut self, value: u16);

    fn off9(&self) -> u16;

    fn set_off9(&mut self, value: u16);

    fn val9(&self) -> u16;

    fn set_val9(&mut self, value: u16);

    fn off10(&self) -> u16;

    fn set_off10(&mut self, value: u16);

    fn val10(&self) -> u16;

    fn set_val10(&mut self, value: u16);

    fn off11(&self) -> u16;

    fn set_off11(&mut self, value: u16);

    fn val11(&self) -> u16;

    fn set_val11(&mut self, value: u16);

    fn off12(&self) -> u16;

    fn set_off12(&mut self, value: u16);

    fn val12(&self) -> u16;

    fn set_val12(&mut self, value: u16);

    fn off13(&self) -> u16;

    fn set_off13(&mut self, value: u16);

    fn val13(&self) -> u16;

    fn set_val13(&mut self, value: u16);

    fn off14(&self) -> u16;

    fn set_off14(&mut self, value: u16);

    fn val14(&self) -> u16;

    fn set_val14(&mut self, value: u16);

    fn off15(&self) -> u16;

    fn set_off15(&mut self, value: u16);

    fn val15(&self) -> u16;

    fn set_val15(&mut self, value: u16);

    fn off16(&self) -> u16;

    fn set_off16(&mut self, value: u16);

    fn val16(&self) -> u16;

    fn set_val16(&mut self, value: u16);

    fn off17(&self) -> u16;

    fn set_off17(&mut self, value: u16);

    fn val17(&self) -> u16;

    fn set_val17(&mut self, value: u16);

    fn off18(&self) -> u16;

    fn set_off18(&mut self, value: u16);

    fn val18(&self) -> u16;

    fn set_val18(&mut self, value: u16);

    fn off19(&self) -> u16;

    fn set_off19(&mut self, value: u16);

    fn val19(&self) -> u16;

    fn set_val19(&mut self, value: u16);

    fn off20(&self) -> u16;

    fn set_off20(&mut self, value: u16);

    fn val20(&self) -> u16;

    fn set_val20(&mut self, value: u16);

    fn off21(&self) -> u16;

    fn set_off21(&mut self, value: u16);

    fn val21(&self) -> u16;

    fn set_val21(&mut self, value: u16);

    fn off22(&self) -> u16;

    fn set_off22(&mut self, value: u16);

    fn val22(&self) -> u16;

    fn set_val22(&mut self, value: u16);

    fn off23(&self) -> u16;

    fn set_off23(&mut self, value: u16);

    fn val23(&self) -> u16;

    fn set_val23(&mut self, value: u16);

    fn off24(&self) -> u16;

    fn set_off24(&mut self, value: u16);

    fn val24(&self) -> u16;

    fn set_val24(&mut self, value: u16);

    fn off25(&self) -> u16;

    fn set_off25(&mut self, value: u16);

    fn val25(&self) -> u16;

    fn set_val25(&mut self, value: u16);

    fn off26(&self) -> u16;

    fn set_off26(&mut self, value: u16);

    fn val26(&self) -> u16;

    fn set_val26(&mut self, value: u16);

    fn off27(&self) -> u16;

    fn set_off27(&mut self, value: u16);

    fn val27(&self) -> u16;

    fn set_val27(&mut self, value: u16);

    fn off28(&self) -> u16;

    fn set_off28(&mut self, value: u16);

    fn val28(&self) -> u16;

    fn set_val28(&mut self, value: u16);

    fn off29(&self) -> u16;

    fn set_off29(&mut self, value: u16);

    fn val29(&self) -> u16;

    fn set_val29(&mut self, value: u16);

    fn off30(&self) -> u16;

    fn set_off30(&mut self, value: u16);

    fn val30(&self) -> u16;

    fn set_val30(&mut self, value: u16);

    fn off31(&self) -> u16;

    fn set_off31(&mut self, value: u16);

    fn val31(&self) -> u16;

    fn set_val31(&mut self, value: u16);

    fn off32(&self) -> u16;

    fn set_off32(&mut self, value: u16);

    fn val32(&self) -> u16;

    fn set_val32(&mut self, value: u16);

    fn off33(&self) -> u16;

    fn set_off33(&mut self, value: u16);

    fn val33(&self) -> u16;

    fn set_val33(&mut self, value: u16);

    fn off34(&self) -> u16;

    fn set_off34(&mut self, value: u16);

    fn val34(&self) -> u16;

    fn set_val34(&mut self, value: u16);

    fn off35(&self) -> u16;

    fn set_off35(&mut self, value: u16);

    fn val35(&self) -> u16;

    fn set_val35(&mut self, value: u16);

    fn off36(&self) -> u16;

    fn set_off36(&mut self, value: u16);

    fn val36(&self) -> u16;

    fn set_val36(&mut self, value: u16);

    fn off37(&self) -> u16;

    fn set_off37(&mut self, value: u16);

    fn val37(&self) -> u16;

    fn set_val37(&mut self, value: u16);

    fn off38(&self) -> u16;

    fn set_off38(&mut self, value: u16);

    fn val38(&self) -> u16;

    fn set_val38(&mut self, value: u16);

    fn off39(&self) -> u16;

    fn set_off39(&mut self, value: u16);

    fn val39(&self) -> u16;

    fn set_val39(&mut self, value: u16);

    fn off40(&self) -> u16;

    fn set_off40(&mut self, value: u16);

    fn val40(&self) -> u16;

    fn set_val40(&mut self, value: u16);

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32;

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn set_timestamp(&mut self, value: u32);

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16;

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn set_milliseconds(&mut self, value: u16);

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn sequence(&self) -> u16;

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn set_sequence(&mut self, value: u16);

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn role(&self) -> u16;

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn set_role(&mut self, value: u16);

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg;

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn set_algorithm(&mut self, value: Alg);

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16;

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn set_n(&mut self, value: u16);

    /// DS
    ///
    /// Digital Signature
    fn repeating_ds(&self) -> u16;

    /// DS
    ///
    /// Digital Signature
    fn set_repeating_ds(&mut self, value: u16);
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}

#[repr(C)]
pub struct Model5CallbackAdapter {
    context: *mut c_void,
    x_callback: extern "C" fn(*const c_void) -> u16,
    set_x_callback: extern "C" fn(u16, *mut c_void),
    offset1_callback: extern "C" fn(*const c_void) -> u16,
    set_offset1_callback: extern "C" fn(u16, *mut c_void),
    value1_callback: extern "C" fn(*const c_void) -> u16,
    set_value1_callback: extern "C" fn(u16, *mut c_void),
    off2_callback: extern "C" fn(*const c_void) -> u16,
    set_off2_callback: extern "C" fn(u16, *mut c_void),
    val2_callback: extern "C" fn(*const c_void) -> u16,
    set_val2_callback: extern "C" fn(u16, *mut c_void),
    off3_callback: extern "C" fn(*const c_void) -> u16,
    set_off3_callback: extern "C" fn(u16, *mut c_void),
    val3_callback: extern "C" fn(*const c_void) -> u16,
    set_val3_callback: extern "C" fn(u16, *mut c_void),
    off4_callback: extern "C" fn(*const c_void) -> u16,
    set_off4_callback: extern "C" fn(u16, *mut c_void),
    val4_callback: extern "C" fn(*const c_void) -> u16,
    set_val4_callback: extern "C" fn(u16, *mut c_void),
    off5_callback: extern "C" fn(*const c_void) -> u16,
    set_off5_callback: extern "C" fn(u16, *mut c_void),
    val5_callback: extern "C" fn(*const c_void) -> u16,
    set_val5_callback: extern "C" fn(u16, *mut c_void),
    off6_callback: extern "C" fn(*const c_void) -> u16,
    set_off6_callback: extern "C" fn(u16, *mut c_void),
    val6_callback: extern "C" fn(*const c_void) -> u16,
    set_val6_callback: extern "C" fn(u16, *mut c_void),
    off7_callback: extern "C" fn(*const c_void) -> u16,
    set_off7_callback: extern "C" fn(u16, *mut c_void),
    val7_callback: extern "C" fn(*const c_void) -> u16,
    set_val7_callback: extern "C" fn(u16, *mut c_void),
    off8_callback: extern "C" fn(*const c_void) -> u16,
    set_off8_callback: extern "C" fn(u16, *mut c_void),
    val8_callback: extern "C" fn(*const c_void) -> u16,
    set_val8_callback: extern "C" fn(u16, *mut c_void),
    off9_callback: extern "C" fn(*const c_void) -> u16,
    set_off9_callback: extern "C" fn(u16, *mut c_void),
    val9_callback: extern "C" fn(*const c_void) -> u16,
    set_val9_callback: extern "C" fn(u16, *mut c_void),
    off10_callback: extern "C" fn(*const c_void) -> u16,
    set_off10_callback: extern "C" fn(u16, *mut c_void),
    val10_callback: extern "C" fn(*const c_void) -> u16,
    set_val10_callback: extern "C" fn(u16, *mut c_void),
    off11_callback: extern "C" fn(*const c_void) -> u16,
    set_off11_callback: extern "C" fn(u16, *mut c_void),
    val11_callback: extern "C" fn(*const c_void) -> u16,
    set_val11_callback: extern "C" fn(u16, *mut c_void),
    off12_callback: extern "C" fn(*const c_void) -> u16,
    set_off12_callback: extern "C" fn(u16, *mut c_void),
    val12_callback: extern "C" fn(*const c_void) -> u16,
    set_val12_callback: extern "C" fn(u16, *mut c_void),
    off13_callback: extern "C" fn(*const c_void) -> u16,
    set_off13_callback: extern "C" fn(u16, *mut c_void),
    val13_callback: extern "C" fn(*const c_void) -> u16,
    set_val13_callback: extern "C" fn(u16, *mut c_void),
    off14_callback: extern "C" fn(*const c_void) -> u16,
    set_off14_callback: extern "C" fn(u16, *mut c_void),
    val14_callback: extern "C" fn(*const c_void) -> u16,
    set_val14_callback: extern "C" fn(u16, *mut c_void),
    off15_callback: extern "C" fn(*const c_void) -> u16,
    set_off15_callback: extern "C" fn(u16, *mut c_void),
    val15_callback: extern "C" fn(*const c_void) -> u16,
    set_val15_callback: extern "C" fn(u16, *mut c_void),
    off16_callback: extern "C" fn(*const c_void) -> u16,
    set_off16_callback: extern "C" fn(u16, *mut c_void),
    val16_callback: extern "C" fn(*const c_void) -> u16,
    set_val16_callback: extern "C" fn(u16, *mut c_void),
    off17_callback: extern "C" fn(*const c_void) -> u16,
    set_off17_callback: extern "C" fn(u16, *mut c_void),
    val17_callback: extern "C" fn(*const c_void) -> u16,
    set_val17_callback: extern "C" fn(u16, *mut c_void),
    off18_callback: extern "C" fn(*const c_void) -> u16,
    set_off18_callback: extern "C" fn(u16, *mut c_void),
    val18_callback: extern "C" fn(*const c_void) -> u16,
    set_val18_callback: extern "C" fn(u16, *mut c_void),
    off19_callback: extern "C" fn(*const c_void) -> u16,
    set_off19_callback: extern "C" fn(u16, *mut c_void),
    val19_callback: extern "C" fn(*const c_void) -> u16,
    set_val19_callback: extern "C" fn(u16, *mut c_void),
    off20_callback: extern "C" fn(*const c_void) -> u16,
    set_off20_callback: extern "C" fn(u16, *mut c_void),
    val20_callback: extern "C" fn(*const c_void) -> u16,
    set_val20_callback: extern "C" fn(u16, *mut c_void),
    off21_callback: extern "C" fn(*const c_void) -> u16,
    set_off21_callback: extern "C" fn(u16, *mut c_void),
    val21_callback: extern "C" fn(*const c_void) -> u16,
    set_val21_callback: extern "C" fn(u16, *mut c_void),
    off22_callback: extern "C" fn(*const c_void) -> u16,
    set_off22_callback: extern "C" fn(u16, *mut c_void),
    val22_callback: extern "C" fn(*const c_void) -> u16,
    set_val22_callback: extern "C" fn(u16, *mut c_void),
    off23_callback: extern "C" fn(*const c_void) -> u16,
    set_off23_callback: extern "C" fn(u16, *mut c_void),
    val23_callback: extern "C" fn(*const c_void) -> u16,
    set_val23_callback: extern "C" fn(u16, *mut c_void),
    off24_callback: extern "C" fn(*const c_void) -> u16,
    set_off24_callback: extern "C" fn(u16, *mut c_void),
    val24_callback: extern "C" fn(*const c_void) -> u16,
    set_val24_callback: extern "C" fn(u16, *mut c_void),
    off25_callback: extern "C" fn(*const c_void) -> u16,
    set_off25_callback: extern "C" fn(u16, *mut c_void),
    val25_callback: extern "C" fn(*const c_void) -> u16,
    set_val25_callback: extern "C" fn(u16, *mut c_void),
    off26_callback: extern "C" fn(*const c_void) -> u16,
    set_off26_callback: extern "C" fn(u16, *mut c_void),
    val26_callback: extern "C" fn(*const c_void) -> u16,
    set_val26_callback: extern "C" fn(u16, *mut c_void),
    off27_callback: extern "C" fn(*const c_void) -> u16,
    set_off27_callback: extern "C" fn(u16, *mut c_void),
    val27_callback: extern "C" fn(*const c_void) -> u16,
    set_val27_callback: extern "C" fn(u16, *mut c_void),
    off28_callback: extern "C" fn(*const c_void) -> u16,
    set_off28_callback: extern "C" fn(u16, *mut c_void),
    val28_callback: extern "C" fn(*const c_void) -> u16,
    set_val28_callback: extern "C" fn(u16, *mut c_void),
    off29_callback: extern "C" fn(*const c_void) -> u16,
    set_off29_callback: extern "C" fn(u16, *mut c_void),
    val29_callback: extern "C" fn(*const c_void) -> u16,
    set_val29_callback: extern "C" fn(u16, *mut c_void),
    off30_callback: extern "C" fn(*const c_void) -> u16,
    set_off30_callback: extern "C" fn(u16, *mut c_void),
    val30_callback: extern "C" fn(*const c_void) -> u16,
    set_val30_callback: extern "C" fn(u16, *mut c_void),
    off31_callback: extern "C" fn(*const c_void) -> u16,
    set_off31_callback: extern "C" fn(u16, *mut c_void),
    val31_callback: extern "C" fn(*const c_void) -> u16,
    set_val31_callback: extern "C" fn(u16, *mut c_void),
    off32_callback: extern "C" fn(*const c_void) -> u16,
    set_off32_callback: extern "C" fn(u16, *mut c_void),
    val32_callback: extern "C" fn(*const c_void) -> u16,
    set_val32_callback: extern "C" fn(u16, *mut c_void),
    off33_callback: extern "C" fn(*const c_void) -> u16,
    set_off33_callback: extern "C" fn(u16, *mut c_void),
    val33_callback: extern "C" fn(*const c_void) -> u16,
    set_val33_callback: extern "C" fn(u16, *mut c_void),
    off34_callback: extern "C" fn(*const c_void) -> u16,
    set_off34_callback: extern "C" fn(u16, *mut c_void),
    val34_callback: extern "C" fn(*const c_void) -> u16,
    set_val34_callback: extern "C" fn(u16, *mut c_void),
    off35_callback: extern "C" fn(*const c_void) -> u16,
    set_off35_callback: extern "C" fn(u16, *mut c_void),
    val35_callback: extern "C" fn(*const c_void) -> u16,
    set_val35_callback: extern "C" fn(u16, *mut c_void),
    off36_callback: extern "C" fn(*const c_void) -> u16,
    set_off36_callback: extern "C" fn(u16, *mut c_void),
    val36_callback: extern "C" fn(*const c_void) -> u16,
    set_val36_callback: extern "C" fn(u16, *mut c_void),
    off37_callback: extern "C" fn(*const c_void) -> u16,
    set_off37_callback: extern "C" fn(u16, *mut c_void),
    val37_callback: extern "C" fn(*const c_void) -> u16,
    set_val37_callback: extern "C" fn(u16, *mut c_void),
    off38_callback: extern "C" fn(*const c_void) -> u16,
    set_off38_callback: extern "C" fn(u16, *mut c_void),
    val38_callback: extern "C" fn(*const c_void) -> u16,
    set_val38_callback: extern "C" fn(u16, *mut c_void),
    off39_callback: extern "C" fn(*const c_void) -> u16,
    set_off39_callback: extern "C" fn(u16, *mut c_void),
    val39_callback: extern "C" fn(*const c_void) -> u16,
    set_val39_callback: extern "C" fn(u16, *mut c_void),
    off40_callback: extern "C" fn(*const c_void) -> u16,
    set_off40_callback: extern "C" fn(u16, *mut c_void),
    val40_callback: extern "C" fn(*const c_void) -> u16,
    set_val40_callback: extern "C" fn(u16, *mut c_void),
    timestamp_callback: extern "C" fn(*const c_void) -> u32,
    set_timestamp_callback: extern "C" fn(u32, *mut c_void),
    milliseconds_callback: extern "C" fn(*const c_void) -> u16,
    set_milliseconds_callback: extern "C" fn(u16, *mut c_void),
    sequence_callback: extern "C" fn(*const c_void) -> u16,
    set_sequence_callback: extern "C" fn(u16, *mut c_void),
    role_callback: extern "C" fn(*const c_void) -> u16,
    set_role_callback: extern "C" fn(u16, *mut c_void),
    algorithm_callback: extern "C" fn(*const c_void) -> Alg,
    set_algorithm_callback: extern "C" fn(Alg, *mut c_void),
    n_callback: extern "C" fn(*const c_void) -> u16,
    set_n_callback: extern "C" fn(u16, *mut c_void),
    repeating_ds_callback: extern "C" fn(*const c_void) -> u16,
    set_repeating_ds_callback: extern "C" fn(u16, *mut c_void),
}

impl ModelAdapter for Model5CallbackAdapter {
    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn x(&self) -> u16 {
        (self.x_callback)(self.context)
    }

    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn set_x(&mut self, value: u16) {
        (self.set_x_callback)(value, self.context);
    }

    /// Offset1
    ///
    /// Offset of control register to write value to
    fn offset1(&self) -> u16 {
        (self.offset1_callback)(self.context)
    }

    /// Offset1
    ///
    /// Offset of control register to write value to
    fn set_offset1(&mut self, value: u16) {
        (self.set_offset1_callback)(value, self.context);
    }

    /// Value1
    ///
    /// Value to write to control register at offset
    fn value1(&self) -> u16 {
        (self.value1_callback)(self.context)
    }

    /// Value1
    ///
    /// Value to write to control register at offset
    fn set_value1(&mut self, value: u16) {
        (self.set_value1_callback)(value, self.context);
    }

    fn off2(&self) -> u16 {
        (self.off2_callback)(self.context)
    }

    fn set_off2(&mut self, value: u16) {
        (self.set_off2_callback)(value, self.context);
    }

    fn val2(&self) -> u16 {
        (self.val2_callback)(self.context)
    }

    fn set_val2(&mut self, value: u16) {
        (self.set_val2_callback)(value, self.context);
    }

    fn off3(&self) -> u16 {
        (self.off3_callback)(self.context)
    }

    fn set_off3(&mut self, value: u16) {
        (self.set_off3_callback)(value, self.context);
    }

    fn val3(&self) -> u16 {
        (self.val3_callback)(self.context)
    }

    fn set_val3(&mut self, value: u16) {
        (self.set_val3_callback)(value, self.context);
    }

    fn off4(&self) -> u16 {
        (self.off4_callback)(self.context)
    }

    fn set_off4(&mut self, value: u16) {
        (self.set_off4_callback)(value, self.context);
    }

    fn val4(&self) -> u16 {
        (self.val4_callback)(self.context)
    }

    fn set_val4(&mut self, value: u16) {
        (self.set_val4_callback)(value, self.context);
    }

    fn off5(&self) -> u16 {
        (self.off5_callback)(self.context)
    }

    fn set_off5(&mut self, value: u16) {
        (self.set_off5_callback)(value, self.context);
    }

    fn val5(&self) -> u16 {
        (self.val5_callback)(self.context)
    }

    fn set_val5(&mut self, value: u16) {
        (self.set_val5_callback)(value, self.context);
    }

    fn off6(&self) -> u16 {
        (self.off6_callback)(self.context)
    }

    fn set_off6(&mut self, value: u16) {
        (self.set_off6_callback)(value, self.context);
    }

    fn val6(&self) -> u16 {
        (self.val6_callback)(self.context)
    }

    fn set_val6(&mut self, value: u16) {
        (self.set_val6_callback)(value, self.context);
    }

    fn off7(&self) -> u16 {
        (self.off7_callback)(self.context)
    }

    fn set_off7(&mut self, value: u16) {
        (self.set_off7_callback)(value, self.context);
    }

    fn val7(&self) -> u16 {
        (self.val7_callback)(self.context)
    }

    fn set_val7(&mut self, value: u16) {
        (self.set_val7_callback)(value, self.context);
    }

    fn off8(&self) -> u16 {
        (self.off8_callback)(self.context)
    }

    fn set_off8(&mut self, value: u16) {
        (self.set_off8_callback)(value, self.context);
    }

    fn val8(&self) -> u16 {
        (self.val8_callback)(self.context)
    }

    fn set_val8(&mut self, value: u16) {
        (self.set_val8_callback)(value, self.context);
    }

    fn off9(&self) -> u16 {
        (self.off9_callback)(self.context)
    }

    fn set_off9(&mut self, value: u16) {
        (self.set_off9_callback)(value, self.context);
    }

    fn val9(&self) -> u16 {
        (self.val9_callback)(self.context)
    }

    fn set_val9(&mut self, value: u16) {
        (self.set_val9_callback)(value, self.context);
    }

    fn off10(&self) -> u16 {
        (self.off10_callback)(self.context)
    }

    fn set_off10(&mut self, value: u16) {
        (self.set_off10_callback)(value, self.context);
    }

    fn val10(&self) -> u16 {
        (self.val10_callback)(self.context)
    }

    fn set_val10(&mut self, value: u16) {
        (self.set_val10_callback)(value, self.context);
    }

    fn off11(&self) -> u16 {
        (self.off11_callback)(self.context)
    }

    fn set_off11(&mut self, value: u16) {
        (self.set_off11_callback)(value, self.context);
    }

    fn val11(&self) -> u16 {
        (self.val11_callback)(self.context)
    }

    fn set_val11(&mut self, value: u16) {
        (self.set_val11_callback)(value, self.context);
    }

    fn off12(&self) -> u16 {
        (self.off12_callback)(self.context)
    }

    fn set_off12(&mut self, value: u16) {
        (self.set_off12_callback)(value, self.context);
    }

    fn val12(&self) -> u16 {
        (self.val12_callback)(self.context)
    }

    fn set_val12(&mut self, value: u16) {
        (self.set_val12_callback)(value, self.context);
    }

    fn off13(&self) -> u16 {
        (self.off13_callback)(self.context)
    }

    fn set_off13(&mut self, value: u16) {
        (self.set_off13_callback)(value, self.context);
    }

    fn val13(&self) -> u16 {
        (self.val13_callback)(self.context)
    }

    fn set_val13(&mut self, value: u16) {
        (self.set_val13_callback)(value, self.context);
    }

    fn off14(&self) -> u16 {
        (self.off14_callback)(self.context)
    }

    fn set_off14(&mut self, value: u16) {
        (self.set_off14_callback)(value, self.context);
    }

    fn val14(&self) -> u16 {
        (self.val14_callback)(self.context)
    }

    fn set_val14(&mut self, value: u16) {
        (self.set_val14_callback)(value, self.context);
    }

    fn off15(&self) -> u16 {
        (self.off15_callback)(self.context)
    }

    fn set_off15(&mut self, value: u16) {
        (self.set_off15_callback)(value, self.context);
    }

    fn val15(&self) -> u16 {
        (self.val15_callback)(self.context)
    }

    fn set_val15(&mut self, value: u16) {
        (self.set_val15_callback)(value, self.context);
    }

    fn off16(&self) -> u16 {
        (self.off16_callback)(self.context)
    }

    fn set_off16(&mut self, value: u16) {
        (self.set_off16_callback)(value, self.context);
    }

    fn val16(&self) -> u16 {
        (self.val16_callback)(self.context)
    }

    fn set_val16(&mut self, value: u16) {
        (self.set_val16_callback)(value, self.context);
    }

    fn off17(&self) -> u16 {
        (self.off17_callback)(self.context)
    }

    fn set_off17(&mut self, value: u16) {
        (self.set_off17_callback)(value, self.context);
    }

    fn val17(&self) -> u16 {
        (self.val17_callback)(self.context)
    }

    fn set_val17(&mut self, value: u16) {
        (self.set_val17_callback)(value, self.context);
    }

    fn off18(&self) -> u16 {
        (self.off18_callback)(self.context)
    }

    fn set_off18(&mut self, value: u16) {
        (self.set_off18_callback)(value, self.context);
    }

    fn val18(&self) -> u16 {
        (self.val18_callback)(self.context)
    }

    fn set_val18(&mut self, value: u16) {
        (self.set_val18_callback)(value, self.context);
    }

    fn off19(&self) -> u16 {
        (self.off19_callback)(self.context)
    }

    fn set_off19(&mut self, value: u16) {
        (self.set_off19_callback)(value, self.context);
    }

    fn val19(&self) -> u16 {
        (self.val19_callback)(self.context)
    }

    fn set_val19(&mut self, value: u16) {
        (self.set_val19_callback)(value, self.context);
    }

    fn off20(&self) -> u16 {
        (self.off20_callback)(self.context)
    }

    fn set_off20(&mut self, value: u16) {
        (self.set_off20_callback)(value, self.context);
    }

    fn val20(&self) -> u16 {
        (self.val20_callback)(self.context)
    }

    fn set_val20(&mut self, value: u16) {
        (self.set_val20_callback)(value, self.context);
    }

    fn off21(&self) -> u16 {
        (self.off21_callback)(self.context)
    }

    fn set_off21(&mut self, value: u16) {
        (self.set_off21_callback)(value, self.context);
    }

    fn val21(&self) -> u16 {
        (self.val21_callback)(self.context)
    }

    fn set_val21(&mut self, value: u16) {
        (self.set_val21_callback)(value, self.context);
    }

    fn off22(&self) -> u16 {
        (self.off22_callback)(self.context)
    }

    fn set_off22(&mut self, value: u16) {
        (self.set_off22_callback)(value, self.context);
    }

    fn val22(&self) -> u16 {
        (self.val22_callback)(self.context)
    }

    fn set_val22(&mut self, value: u16) {
        (self.set_val22_callback)(value, self.context);
    }

    fn off23(&self) -> u16 {
        (self.off23_callback)(self.context)
    }

    fn set_off23(&mut self, value: u16) {
        (self.set_off23_callback)(value, self.context);
    }

    fn val23(&self) -> u16 {
        (self.val23_callback)(self.context)
    }

    fn set_val23(&mut self, value: u16) {
        (self.set_val23_callback)(value, self.context);
    }

    fn off24(&self) -> u16 {
        (self.off24_callback)(self.context)
    }

    fn set_off24(&mut self, value: u16) {
        (self.set_off24_callback)(value, self.context);
    }

    fn val24(&self) -> u16 {
        (self.val24_callback)(self.context)
    }

    fn set_val24(&mut self, value: u16) {
        (self.set_val24_callback)(value, self.context);
    }

    fn off25(&self) -> u16 {
        (self.off25_callback)(self.context)
    }

    fn set_off25(&mut self, value: u16) {
        (self.set_off25_callback)(value, self.context);
    }

    fn val25(&self) -> u16 {
        (self.val25_callback)(self.context)
    }

    fn set_val25(&mut self, value: u16) {
        (self.set_val25_callback)(value, self.context);
    }

    fn off26(&self) -> u16 {
        (self.off26_callback)(self.context)
    }

    fn set_off26(&mut self, value: u16) {
        (self.set_off26_callback)(value, self.context);
    }

    fn val26(&self) -> u16 {
        (self.val26_callback)(self.context)
    }

    fn set_val26(&mut self, value: u16) {
        (self.set_val26_callback)(value, self.context);
    }

    fn off27(&self) -> u16 {
        (self.off27_callback)(self.context)
    }

    fn set_off27(&mut self, value: u16) {
        (self.set_off27_callback)(value, self.context);
    }

    fn val27(&self) -> u16 {
        (self.val27_callback)(self.context)
    }

    fn set_val27(&mut self, value: u16) {
        (self.set_val27_callback)(value, self.context);
    }

    fn off28(&self) -> u16 {
        (self.off28_callback)(self.context)
    }

    fn set_off28(&mut self, value: u16) {
        (self.set_off28_callback)(value, self.context);
    }

    fn val28(&self) -> u16 {
        (self.val28_callback)(self.context)
    }

    fn set_val28(&mut self, value: u16) {
        (self.set_val28_callback)(value, self.context);
    }

    fn off29(&self) -> u16 {
        (self.off29_callback)(self.context)
    }

    fn set_off29(&mut self, value: u16) {
        (self.set_off29_callback)(value, self.context);
    }

    fn val29(&self) -> u16 {
        (self.val29_callback)(self.context)
    }

    fn set_val29(&mut self, value: u16) {
        (self.set_val29_callback)(value, self.context);
    }

    fn off30(&self) -> u16 {
        (self.off30_callback)(self.context)
    }

    fn set_off30(&mut self, value: u16) {
        (self.set_off30_callback)(value, self.context);
    }

    fn val30(&self) -> u16 {
        (self.val30_callback)(self.context)
    }

    fn set_val30(&mut self, value: u16) {
        (self.set_val30_callback)(value, self.context);
    }

    fn off31(&self) -> u16 {
        (self.off31_callback)(self.context)
    }

    fn set_off31(&mut self, value: u16) {
        (self.set_off31_callback)(value, self.context);
    }

    fn val31(&self) -> u16 {
        (self.val31_callback)(self.context)
    }

    fn set_val31(&mut self, value: u16) {
        (self.set_val31_callback)(value, self.context);
    }

    fn off32(&self) -> u16 {
        (self.off32_callback)(self.context)
    }

    fn set_off32(&mut self, value: u16) {
        (self.set_off32_callback)(value, self.context);
    }

    fn val32(&self) -> u16 {
        (self.val32_callback)(self.context)
    }

    fn set_val32(&mut self, value: u16) {
        (self.set_val32_callback)(value, self.context);
    }

    fn off33(&self) -> u16 {
        (self.off33_callback)(self.context)
    }

    fn set_off33(&mut self, value: u16) {
        (self.set_off33_callback)(value, self.context);
    }

    fn val33(&self) -> u16 {
        (self.val33_callback)(self.context)
    }

    fn set_val33(&mut self, value: u16) {
        (self.set_val33_callback)(value, self.context);
    }

    fn off34(&self) -> u16 {
        (self.off34_callback)(self.context)
    }

    fn set_off34(&mut self, value: u16) {
        (self.set_off34_callback)(value, self.context);
    }

    fn val34(&self) -> u16 {
        (self.val34_callback)(self.context)
    }

    fn set_val34(&mut self, value: u16) {
        (self.set_val34_callback)(value, self.context);
    }

    fn off35(&self) -> u16 {
        (self.off35_callback)(self.context)
    }

    fn set_off35(&mut self, value: u16) {
        (self.set_off35_callback)(value, self.context);
    }

    fn val35(&self) -> u16 {
        (self.val35_callback)(self.context)
    }

    fn set_val35(&mut self, value: u16) {
        (self.set_val35_callback)(value, self.context);
    }

    fn off36(&self) -> u16 {
        (self.off36_callback)(self.context)
    }

    fn set_off36(&mut self, value: u16) {
        (self.set_off36_callback)(value, self.context);
    }

    fn val36(&self) -> u16 {
        (self.val36_callback)(self.context)
    }

    fn set_val36(&mut self, value: u16) {
        (self.set_val36_callback)(value, self.context);
    }

    fn off37(&self) -> u16 {
        (self.off37_callback)(self.context)
    }

    fn set_off37(&mut self, value: u16) {
        (self.set_off37_callback)(value, self.context);
    }

    fn val37(&self) -> u16 {
        (self.val37_callback)(self.context)
    }

    fn set_val37(&mut self, value: u16) {
        (self.set_val37_callback)(value, self.context);
    }

    fn off38(&self) -> u16 {
        (self.off38_callback)(self.context)
    }

    fn set_off38(&mut self, value: u16) {
        (self.set_off38_callback)(value, self.context);
    }

    fn val38(&self) -> u16 {
        (self.val38_callback)(self.context)
    }

    fn set_val38(&mut self, value: u16) {
        (self.set_val38_callback)(value, self.context);
    }

    fn off39(&self) -> u16 {
        (self.off39_callback)(self.context)
    }

    fn set_off39(&mut self, value: u16) {
        (self.set_off39_callback)(value, self.context);
    }

    fn val39(&self) -> u16 {
        (self.val39_callback)(self.context)
    }

    fn set_val39(&mut self, value: u16) {
        (self.set_val39_callback)(value, self.context);
    }

    fn off40(&self) -> u16 {
        (self.off40_callback)(self.context)
    }

    fn set_off40(&mut self, value: u16) {
        (self.set_off40_callback)(value, self.context);
    }

    fn val40(&self) -> u16 {
        (self.val40_callback)(self.context)
    }

    fn set_val40(&mut self, value: u16) {
        (self.set_val40_callback)(value, self.context);
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        (self.timestamp_callback)(self.context)
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn set_timestamp(&mut self, value: u32) {
        (self.set_timestamp_callback)(value, self.context);
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        (self.milliseconds_callback)(self.context)
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn set_milliseconds(&mut self, value: u16) {
        (self.set_milliseconds_callback)(value, self.context);
    }

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn sequence(&self) -> u16 {
        (self.sequence_callback)(self.context)
    }

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn set_sequence(&mut self, value: u16) {
        (self.set_sequence_callback)(value, self.context);
    }

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn role(&self) -> u16 {
        (self.role_callback)(self.context)
    }

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn set_role(&mut self, value: u16) {
        (self.set_role_callback)(value, self.context);
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg {
        (self.algorithm_callback)(self.context)
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn set_algorithm(&mut self, value: Alg) {
        (self.set_algorithm_callback)(value, self.context);
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        (self.n_callback)(self.context)
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn set_n(&mut self, value: u16) {
        (self.set_n_callback)(value, self.context);
    }

    /// DS
    ///
    /// Digital Signature
    fn repeating_ds(&self) -> u16 {
        (self.repeating_ds_callback)(self.context)
    }

    /// DS
    ///
    /// Digital Signature
    fn set_repeating_ds(&mut self, value: u16) {
        (self.set_repeating_ds_callback)(value, self.context);
    }
}

#[repr(C)]
pub struct Model5StatefulAdapter {
    x: u16,
    offset1: u16,
    value1: u16,
    off2: u16,
    val2: u16,
    off3: u16,
    val3: u16,
    off4: u16,
    val4: u16,
    off5: u16,
    val5: u16,
    off6: u16,
    val6: u16,
    off7: u16,
    val7: u16,
    off8: u16,
    val8: u16,
    off9: u16,
    val9: u16,
    off10: u16,
    val10: u16,
    off11: u16,
    val11: u16,
    off12: u16,
    val12: u16,
    off13: u16,
    val13: u16,
    off14: u16,
    val14: u16,
    off15: u16,
    val15: u16,
    off16: u16,
    val16: u16,
    off17: u16,
    val17: u16,
    off18: u16,
    val18: u16,
    off19: u16,
    val19: u16,
    off20: u16,
    val20: u16,
    off21: u16,
    val21: u16,
    off22: u16,
    val22: u16,
    off23: u16,
    val23: u16,
    off24: u16,
    val24: u16,
    off25: u16,
    val25: u16,
    off26: u16,
    val26: u16,
    off27: u16,
    val27: u16,
    off28: u16,
    val28: u16,
    off29: u16,
    val29: u16,
    off30: u16,
    val30: u16,
    off31: u16,
    val31: u16,
    off32: u16,
    val32: u16,
    off33: u16,
    val33: u16,
    off34: u16,
    val34: u16,
    off35: u16,
    val35: u16,
    off36: u16,
    val36: u16,
    off37: u16,
    val37: u16,
    off38: u16,
    val38: u16,
    off39: u16,
    val39: u16,
    off40: u16,
    val40: u16,
    timestamp: u32,
    milliseconds: u16,
    sequence: u16,
    role: u16,
    algorithm: Alg,
    n: u16,
    repeating_ds: u16,
}

impl ModelAdapter for Model5StatefulAdapter {
    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn x(&self) -> u16 {
        self.x
    }

    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn set_x(&mut self, value: u16) {
        self.x = value;
    }

    /// Offset1
    ///
    /// Offset of control register to write value to
    fn offset1(&self) -> u16 {
        self.offset1
    }

    /// Offset1
    ///
    /// Offset of control register to write value to
    fn set_offset1(&mut self, value: u16) {
        self.offset1 = value;
    }

    /// Value1
    ///
    /// Value to write to control register at offset
    fn value1(&self) -> u16 {
        self.value1
    }

    /// Value1
    ///
    /// Value to write to control register at offset
    fn set_value1(&mut self, value: u16) {
        self.value1 = value;
    }

    fn off2(&self) -> u16 {
        self.off2
    }

    fn set_off2(&mut self, value: u16) {
        self.off2 = value;
    }

    fn val2(&self) -> u16 {
        self.val2
    }

    fn set_val2(&mut self, value: u16) {
        self.val2 = value;
    }

    fn off3(&self) -> u16 {
        self.off3
    }

    fn set_off3(&mut self, value: u16) {
        self.off3 = value;
    }

    fn val3(&self) -> u16 {
        self.val3
    }

    fn set_val3(&mut self, value: u16) {
        self.val3 = value;
    }

    fn off4(&self) -> u16 {
        self.off4
    }

    fn set_off4(&mut self, value: u16) {
        self.off4 = value;
    }

    fn val4(&self) -> u16 {
        self.val4
    }

    fn set_val4(&mut self, value: u16) {
        self.val4 = value;
    }

    fn off5(&self) -> u16 {
        self.off5
    }

    fn set_off5(&mut self, value: u16) {
        self.off5 = value;
    }

    fn val5(&self) -> u16 {
        self.val5
    }

    fn set_val5(&mut self, value: u16) {
        self.val5 = value;
    }

    fn off6(&self) -> u16 {
        self.off6
    }

    fn set_off6(&mut self, value: u16) {
        self.off6 = value;
    }

    fn val6(&self) -> u16 {
        self.val6
    }

    fn set_val6(&mut self, value: u16) {
        self.val6 = value;
    }

    fn off7(&self) -> u16 {
        self.off7
    }

    fn set_off7(&mut self, value: u16) {
        self.off7 = value;
    }

    fn val7(&self) -> u16 {
        self.val7
    }

    fn set_val7(&mut self, value: u16) {
        self.val7 = value;
    }

    fn off8(&self) -> u16 {
        self.off8
    }

    fn set_off8(&mut self, value: u16) {
        self.off8 = value;
    }

    fn val8(&self) -> u16 {
        self.val8
    }

    fn set_val8(&mut self, value: u16) {
        self.val8 = value;
    }

    fn off9(&self) -> u16 {
        self.off9
    }

    fn set_off9(&mut self, value: u16) {
        self.off9 = value;
    }

    fn val9(&self) -> u16 {
        self.val9
    }

    fn set_val9(&mut self, value: u16) {
        self.val9 = value;
    }

    fn off10(&self) -> u16 {
        self.off10
    }

    fn set_off10(&mut self, value: u16) {
        self.off10 = value;
    }

    fn val10(&self) -> u16 {
        self.val10
    }

    fn set_val10(&mut self, value: u16) {
        self.val10 = value;
    }

    fn off11(&self) -> u16 {
        self.off11
    }

    fn set_off11(&mut self, value: u16) {
        self.off11 = value;
    }

    fn val11(&self) -> u16 {
        self.val11
    }

    fn set_val11(&mut self, value: u16) {
        self.val11 = value;
    }

    fn off12(&self) -> u16 {
        self.off12
    }

    fn set_off12(&mut self, value: u16) {
        self.off12 = value;
    }

    fn val12(&self) -> u16 {
        self.val12
    }

    fn set_val12(&mut self, value: u16) {
        self.val12 = value;
    }

    fn off13(&self) -> u16 {
        self.off13
    }

    fn set_off13(&mut self, value: u16) {
        self.off13 = value;
    }

    fn val13(&self) -> u16 {
        self.val13
    }

    fn set_val13(&mut self, value: u16) {
        self.val13 = value;
    }

    fn off14(&self) -> u16 {
        self.off14
    }

    fn set_off14(&mut self, value: u16) {
        self.off14 = value;
    }

    fn val14(&self) -> u16 {
        self.val14
    }

    fn set_val14(&mut self, value: u16) {
        self.val14 = value;
    }

    fn off15(&self) -> u16 {
        self.off15
    }

    fn set_off15(&mut self, value: u16) {
        self.off15 = value;
    }

    fn val15(&self) -> u16 {
        self.val15
    }

    fn set_val15(&mut self, value: u16) {
        self.val15 = value;
    }

    fn off16(&self) -> u16 {
        self.off16
    }

    fn set_off16(&mut self, value: u16) {
        self.off16 = value;
    }

    fn val16(&self) -> u16 {
        self.val16
    }

    fn set_val16(&mut self, value: u16) {
        self.val16 = value;
    }

    fn off17(&self) -> u16 {
        self.off17
    }

    fn set_off17(&mut self, value: u16) {
        self.off17 = value;
    }

    fn val17(&self) -> u16 {
        self.val17
    }

    fn set_val17(&mut self, value: u16) {
        self.val17 = value;
    }

    fn off18(&self) -> u16 {
        self.off18
    }

    fn set_off18(&mut self, value: u16) {
        self.off18 = value;
    }

    fn val18(&self) -> u16 {
        self.val18
    }

    fn set_val18(&mut self, value: u16) {
        self.val18 = value;
    }

    fn off19(&self) -> u16 {
        self.off19
    }

    fn set_off19(&mut self, value: u16) {
        self.off19 = value;
    }

    fn val19(&self) -> u16 {
        self.val19
    }

    fn set_val19(&mut self, value: u16) {
        self.val19 = value;
    }

    fn off20(&self) -> u16 {
        self.off20
    }

    fn set_off20(&mut self, value: u16) {
        self.off20 = value;
    }

    fn val20(&self) -> u16 {
        self.val20
    }

    fn set_val20(&mut self, value: u16) {
        self.val20 = value;
    }

    fn off21(&self) -> u16 {
        self.off21
    }

    fn set_off21(&mut self, value: u16) {
        self.off21 = value;
    }

    fn val21(&self) -> u16 {
        self.val21
    }

    fn set_val21(&mut self, value: u16) {
        self.val21 = value;
    }

    fn off22(&self) -> u16 {
        self.off22
    }

    fn set_off22(&mut self, value: u16) {
        self.off22 = value;
    }

    fn val22(&self) -> u16 {
        self.val22
    }

    fn set_val22(&mut self, value: u16) {
        self.val22 = value;
    }

    fn off23(&self) -> u16 {
        self.off23
    }

    fn set_off23(&mut self, value: u16) {
        self.off23 = value;
    }

    fn val23(&self) -> u16 {
        self.val23
    }

    fn set_val23(&mut self, value: u16) {
        self.val23 = value;
    }

    fn off24(&self) -> u16 {
        self.off24
    }

    fn set_off24(&mut self, value: u16) {
        self.off24 = value;
    }

    fn val24(&self) -> u16 {
        self.val24
    }

    fn set_val24(&mut self, value: u16) {
        self.val24 = value;
    }

    fn off25(&self) -> u16 {
        self.off25
    }

    fn set_off25(&mut self, value: u16) {
        self.off25 = value;
    }

    fn val25(&self) -> u16 {
        self.val25
    }

    fn set_val25(&mut self, value: u16) {
        self.val25 = value;
    }

    fn off26(&self) -> u16 {
        self.off26
    }

    fn set_off26(&mut self, value: u16) {
        self.off26 = value;
    }

    fn val26(&self) -> u16 {
        self.val26
    }

    fn set_val26(&mut self, value: u16) {
        self.val26 = value;
    }

    fn off27(&self) -> u16 {
        self.off27
    }

    fn set_off27(&mut self, value: u16) {
        self.off27 = value;
    }

    fn val27(&self) -> u16 {
        self.val27
    }

    fn set_val27(&mut self, value: u16) {
        self.val27 = value;
    }

    fn off28(&self) -> u16 {
        self.off28
    }

    fn set_off28(&mut self, value: u16) {
        self.off28 = value;
    }

    fn val28(&self) -> u16 {
        self.val28
    }

    fn set_val28(&mut self, value: u16) {
        self.val28 = value;
    }

    fn off29(&self) -> u16 {
        self.off29
    }

    fn set_off29(&mut self, value: u16) {
        self.off29 = value;
    }

    fn val29(&self) -> u16 {
        self.val29
    }

    fn set_val29(&mut self, value: u16) {
        self.val29 = value;
    }

    fn off30(&self) -> u16 {
        self.off30
    }

    fn set_off30(&mut self, value: u16) {
        self.off30 = value;
    }

    fn val30(&self) -> u16 {
        self.val30
    }

    fn set_val30(&mut self, value: u16) {
        self.val30 = value;
    }

    fn off31(&self) -> u16 {
        self.off31
    }

    fn set_off31(&mut self, value: u16) {
        self.off31 = value;
    }

    fn val31(&self) -> u16 {
        self.val31
    }

    fn set_val31(&mut self, value: u16) {
        self.val31 = value;
    }

    fn off32(&self) -> u16 {
        self.off32
    }

    fn set_off32(&mut self, value: u16) {
        self.off32 = value;
    }

    fn val32(&self) -> u16 {
        self.val32
    }

    fn set_val32(&mut self, value: u16) {
        self.val32 = value;
    }

    fn off33(&self) -> u16 {
        self.off33
    }

    fn set_off33(&mut self, value: u16) {
        self.off33 = value;
    }

    fn val33(&self) -> u16 {
        self.val33
    }

    fn set_val33(&mut self, value: u16) {
        self.val33 = value;
    }

    fn off34(&self) -> u16 {
        self.off34
    }

    fn set_off34(&mut self, value: u16) {
        self.off34 = value;
    }

    fn val34(&self) -> u16 {
        self.val34
    }

    fn set_val34(&mut self, value: u16) {
        self.val34 = value;
    }

    fn off35(&self) -> u16 {
        self.off35
    }

    fn set_off35(&mut self, value: u16) {
        self.off35 = value;
    }

    fn val35(&self) -> u16 {
        self.val35
    }

    fn set_val35(&mut self, value: u16) {
        self.val35 = value;
    }

    fn off36(&self) -> u16 {
        self.off36
    }

    fn set_off36(&mut self, value: u16) {
        self.off36 = value;
    }

    fn val36(&self) -> u16 {
        self.val36
    }

    fn set_val36(&mut self, value: u16) {
        self.val36 = value;
    }

    fn off37(&self) -> u16 {
        self.off37
    }

    fn set_off37(&mut self, value: u16) {
        self.off37 = value;
    }

    fn val37(&self) -> u16 {
        self.val37
    }

    fn set_val37(&mut self, value: u16) {
        self.val37 = value;
    }

    fn off38(&self) -> u16 {
        self.off38
    }

    fn set_off38(&mut self, value: u16) {
        self.off38 = value;
    }

    fn val38(&self) -> u16 {
        self.val38
    }

    fn set_val38(&mut self, value: u16) {
        self.val38 = value;
    }

    fn off39(&self) -> u16 {
        self.off39
    }

    fn set_off39(&mut self, value: u16) {
        self.off39 = value;
    }

    fn val39(&self) -> u16 {
        self.val39
    }

    fn set_val39(&mut self, value: u16) {
        self.val39 = value;
    }

    fn off40(&self) -> u16 {
        self.off40
    }

    fn set_off40(&mut self, value: u16) {
        self.off40 = value;
    }

    fn val40(&self) -> u16 {
        self.val40
    }

    fn set_val40(&mut self, value: u16) {
        self.val40 = value;
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn set_timestamp(&mut self, value: u32) {
        self.timestamp = value;
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn set_milliseconds(&mut self, value: u16) {
        self.milliseconds = value;
    }

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn sequence(&self) -> u16 {
        self.sequence
    }

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn set_sequence(&mut self, value: u16) {
        self.sequence = value;
    }

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn role(&self) -> u16 {
        self.role
    }

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn set_role(&mut self, value: u16) {
        self.role = value;
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg {
        self.algorithm
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn set_algorithm(&mut self, value: Alg) {
        self.algorithm = value;
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        self.n
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn set_n(&mut self, value: u16) {
        self.n = value;
    }

    /// DS
    ///
    /// Digital Signature
    fn repeating_ds(&self) -> u16 {
        self.repeating_ds
    }

    /// DS
    ///
    /// Digital Signature
    fn set_repeating_ds(&mut self, value: u16) {
        self.repeating_ds = value;
    }
}
