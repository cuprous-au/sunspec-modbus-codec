use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 90;

pub static POINTS: [ReadablePoint; 89] = [
    ReadablePoint {
        reference: PointReference::Static { value: 5 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 88 },
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
        reference: PointReference::Model5 { point: Point::Offset1 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Value1 },
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
        reference: PointReference::Model5 { point: Point::Off10 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val10 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off11 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val11 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off12 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val12 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off13 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val13 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off14 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val14 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off15 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val15 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off16 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val16 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off17 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val17 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off18 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val18 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off19 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val19 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off20 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val20 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off21 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val21 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off22 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val22 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off23 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val23 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off24 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val24 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off25 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val25 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off26 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val26 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off27 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val27 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off28 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val28 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off29 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val29 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off30 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val30 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off31 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val31 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off32 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val32 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off33 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val33 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off34 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val34 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off35 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val35 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off36 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val36 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off37 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val37 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off38 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val38 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off39 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val39 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Off40 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Val40 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Timestamp },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Milliseconds },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model5 { point: Point::Sequence },
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
        reference: PointReference::Model5 { point: Point::Algorithm },
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
];

#[derive(Debug)]
pub enum Point {
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
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::X => serialisation::write_u16(model.x(), buffer),
        Point::Offset1 => serialisation::write_u16(model.offset1(), buffer),
        Point::Value1 => serialisation::write_u16(model.value1(), buffer),
        Point::Off2 => serialisation::write_u16(model.off2(), buffer),
        Point::Val2 => serialisation::write_u16(model.val2(), buffer),
        Point::Off3 => serialisation::write_u16(model.off3(), buffer),
        Point::Val3 => serialisation::write_u16(model.val3(), buffer),
        Point::Off4 => serialisation::write_u16(model.off4(), buffer),
        Point::Val4 => serialisation::write_u16(model.val4(), buffer),
        Point::Off5 => serialisation::write_u16(model.off5(), buffer),
        Point::Val5 => serialisation::write_u16(model.val5(), buffer),
        Point::Off6 => serialisation::write_u16(model.off6(), buffer),
        Point::Val6 => serialisation::write_u16(model.val6(), buffer),
        Point::Off7 => serialisation::write_u16(model.off7(), buffer),
        Point::Val7 => serialisation::write_u16(model.val7(), buffer),
        Point::Off8 => serialisation::write_u16(model.off8(), buffer),
        Point::Val8 => serialisation::write_u16(model.val8(), buffer),
        Point::Off9 => serialisation::write_u16(model.off9(), buffer),
        Point::Val9 => serialisation::write_u16(model.val9(), buffer),
        Point::Off10 => serialisation::write_u16(model.off10(), buffer),
        Point::Val10 => serialisation::write_u16(model.val10(), buffer),
        Point::Off11 => serialisation::write_u16(model.off11(), buffer),
        Point::Val11 => serialisation::write_u16(model.val11(), buffer),
        Point::Off12 => serialisation::write_u16(model.off12(), buffer),
        Point::Val12 => serialisation::write_u16(model.val12(), buffer),
        Point::Off13 => serialisation::write_u16(model.off13(), buffer),
        Point::Val13 => serialisation::write_u16(model.val13(), buffer),
        Point::Off14 => serialisation::write_u16(model.off14(), buffer),
        Point::Val14 => serialisation::write_u16(model.val14(), buffer),
        Point::Off15 => serialisation::write_u16(model.off15(), buffer),
        Point::Val15 => serialisation::write_u16(model.val15(), buffer),
        Point::Off16 => serialisation::write_u16(model.off16(), buffer),
        Point::Val16 => serialisation::write_u16(model.val16(), buffer),
        Point::Off17 => serialisation::write_u16(model.off17(), buffer),
        Point::Val17 => serialisation::write_u16(model.val17(), buffer),
        Point::Off18 => serialisation::write_u16(model.off18(), buffer),
        Point::Val18 => serialisation::write_u16(model.val18(), buffer),
        Point::Off19 => serialisation::write_u16(model.off19(), buffer),
        Point::Val19 => serialisation::write_u16(model.val19(), buffer),
        Point::Off20 => serialisation::write_u16(model.off20(), buffer),
        Point::Val20 => serialisation::write_u16(model.val20(), buffer),
        Point::Off21 => serialisation::write_u16(model.off21(), buffer),
        Point::Val21 => serialisation::write_u16(model.val21(), buffer),
        Point::Off22 => serialisation::write_u16(model.off22(), buffer),
        Point::Val22 => serialisation::write_u16(model.val22(), buffer),
        Point::Off23 => serialisation::write_u16(model.off23(), buffer),
        Point::Val23 => serialisation::write_u16(model.val23(), buffer),
        Point::Off24 => serialisation::write_u16(model.off24(), buffer),
        Point::Val24 => serialisation::write_u16(model.val24(), buffer),
        Point::Off25 => serialisation::write_u16(model.off25(), buffer),
        Point::Val25 => serialisation::write_u16(model.val25(), buffer),
        Point::Off26 => serialisation::write_u16(model.off26(), buffer),
        Point::Val26 => serialisation::write_u16(model.val26(), buffer),
        Point::Off27 => serialisation::write_u16(model.off27(), buffer),
        Point::Val27 => serialisation::write_u16(model.val27(), buffer),
        Point::Off28 => serialisation::write_u16(model.off28(), buffer),
        Point::Val28 => serialisation::write_u16(model.val28(), buffer),
        Point::Off29 => serialisation::write_u16(model.off29(), buffer),
        Point::Val29 => serialisation::write_u16(model.val29(), buffer),
        Point::Off30 => serialisation::write_u16(model.off30(), buffer),
        Point::Val30 => serialisation::write_u16(model.val30(), buffer),
        Point::Off31 => serialisation::write_u16(model.off31(), buffer),
        Point::Val31 => serialisation::write_u16(model.val31(), buffer),
        Point::Off32 => serialisation::write_u16(model.off32(), buffer),
        Point::Val32 => serialisation::write_u16(model.val32(), buffer),
        Point::Off33 => serialisation::write_u16(model.off33(), buffer),
        Point::Val33 => serialisation::write_u16(model.val33(), buffer),
        Point::Off34 => serialisation::write_u16(model.off34(), buffer),
        Point::Val34 => serialisation::write_u16(model.val34(), buffer),
        Point::Off35 => serialisation::write_u16(model.off35(), buffer),
        Point::Val35 => serialisation::write_u16(model.val35(), buffer),
        Point::Off36 => serialisation::write_u16(model.off36(), buffer),
        Point::Val36 => serialisation::write_u16(model.val36(), buffer),
        Point::Off37 => serialisation::write_u16(model.off37(), buffer),
        Point::Val37 => serialisation::write_u16(model.val37(), buffer),
        Point::Off38 => serialisation::write_u16(model.off38(), buffer),
        Point::Val38 => serialisation::write_u16(model.val38(), buffer),
        Point::Off39 => serialisation::write_u16(model.off39(), buffer),
        Point::Val39 => serialisation::write_u16(model.val39(), buffer),
        Point::Off40 => serialisation::write_u16(model.off40(), buffer),
        Point::Val40 => serialisation::write_u16(model.val40(), buffer),
        Point::Timestamp => serialisation::write_u32(model.timestamp(), buffer, offset, limit),
        Point::Milliseconds => serialisation::write_u16(model.milliseconds(), buffer),
        Point::Sequence => serialisation::write_u16(model.sequence(), buffer),
        Point::Role => serialisation::write_u16(model.role(), buffer),
        Point::Algorithm => serialisation::write_u16(model.algorithm() as u16, buffer),
        Point::N => serialisation::write_u16(model.n(), buffer),
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
}

pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}