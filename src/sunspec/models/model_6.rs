use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 92;

pub static POINTS: [ReadablePoint; 91] = [
    ReadablePoint {
        reference: PointReference::Static { value: 6 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 90 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::X },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Offset,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Value1,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Val2 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Val3 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Val4 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Val5 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Val6 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Val7 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Val8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Val9 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val10,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val11,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val12,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val13,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val14,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val15,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val16,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val17,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val18,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val19,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val20,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val21,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val22,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val23,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val24,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val25,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val26,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val27,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val28,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val29,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val30,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val31,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val32,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val33,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val34,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val35,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val36,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val37,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val38,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val39,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val40,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val41,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val42,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val43,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val44,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val45,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val46,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val47,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val48,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val49,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val50,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val51,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val52,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val53,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val54,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val55,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val56,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val57,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val58,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val59,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val60,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val61,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val62,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val63,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val64,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val65,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val66,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val67,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val68,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val69,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val70,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val71,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val72,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val73,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val74,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val75,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val76,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val77,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val78,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val79,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Val80,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Timestamp,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Milliseconds,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Sequence,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::Role },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 {
            point: Point::Algorithm,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model6 { point: Point::N },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    X,
    Offset,
    Value1,
    Val2,
    Val3,
    Val4,
    Val5,
    Val6,
    Val7,
    Val8,
    Val9,
    Val10,
    Val11,
    Val12,
    Val13,
    Val14,
    Val15,
    Val16,
    Val17,
    Val18,
    Val19,
    Val20,
    Val21,
    Val22,
    Val23,
    Val24,
    Val25,
    Val26,
    Val27,
    Val28,
    Val29,
    Val30,
    Val31,
    Val32,
    Val33,
    Val34,
    Val35,
    Val36,
    Val37,
    Val38,
    Val39,
    Val40,
    Val41,
    Val42,
    Val43,
    Val44,
    Val45,
    Val46,
    Val47,
    Val48,
    Val49,
    Val50,
    Val51,
    Val52,
    Val53,
    Val54,
    Val55,
    Val56,
    Val57,
    Val58,
    Val59,
    Val60,
    Val61,
    Val62,
    Val63,
    Val64,
    Val65,
    Val66,
    Val67,
    Val68,
    Val69,
    Val70,
    Val71,
    Val72,
    Val73,
    Val74,
    Val75,
    Val76,
    Val77,
    Val78,
    Val79,
    Val80,
    Timestamp,
    Milliseconds,
    Sequence,
    Role,
    Algorithm,
    N,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::X => serialisation::write_u16(model.x(), buffer),
        Point::Offset => serialisation::write_u16(model.offset(), buffer),
        Point::Value1 => serialisation::write_u16(model.value1(), buffer),
        Point::Val2 => serialisation::write_u16(model.val2(), buffer),
        Point::Val3 => serialisation::write_u16(model.val3(), buffer),
        Point::Val4 => serialisation::write_u16(model.val4(), buffer),
        Point::Val5 => serialisation::write_u16(model.val5(), buffer),
        Point::Val6 => serialisation::write_u16(model.val6(), buffer),
        Point::Val7 => serialisation::write_u16(model.val7(), buffer),
        Point::Val8 => serialisation::write_u16(model.val8(), buffer),
        Point::Val9 => serialisation::write_u16(model.val9(), buffer),
        Point::Val10 => serialisation::write_u16(model.val10(), buffer),
        Point::Val11 => serialisation::write_u16(model.val11(), buffer),
        Point::Val12 => serialisation::write_u16(model.val12(), buffer),
        Point::Val13 => serialisation::write_u16(model.val13(), buffer),
        Point::Val14 => serialisation::write_u16(model.val14(), buffer),
        Point::Val15 => serialisation::write_u16(model.val15(), buffer),
        Point::Val16 => serialisation::write_u16(model.val16(), buffer),
        Point::Val17 => serialisation::write_u16(model.val17(), buffer),
        Point::Val18 => serialisation::write_u16(model.val18(), buffer),
        Point::Val19 => serialisation::write_u16(model.val19(), buffer),
        Point::Val20 => serialisation::write_u16(model.val20(), buffer),
        Point::Val21 => serialisation::write_u16(model.val21(), buffer),
        Point::Val22 => serialisation::write_u16(model.val22(), buffer),
        Point::Val23 => serialisation::write_u16(model.val23(), buffer),
        Point::Val24 => serialisation::write_u16(model.val24(), buffer),
        Point::Val25 => serialisation::write_u16(model.val25(), buffer),
        Point::Val26 => serialisation::write_u16(model.val26(), buffer),
        Point::Val27 => serialisation::write_u16(model.val27(), buffer),
        Point::Val28 => serialisation::write_u16(model.val28(), buffer),
        Point::Val29 => serialisation::write_u16(model.val29(), buffer),
        Point::Val30 => serialisation::write_u16(model.val30(), buffer),
        Point::Val31 => serialisation::write_u16(model.val31(), buffer),
        Point::Val32 => serialisation::write_u16(model.val32(), buffer),
        Point::Val33 => serialisation::write_u16(model.val33(), buffer),
        Point::Val34 => serialisation::write_u16(model.val34(), buffer),
        Point::Val35 => serialisation::write_u16(model.val35(), buffer),
        Point::Val36 => serialisation::write_u16(model.val36(), buffer),
        Point::Val37 => serialisation::write_u16(model.val37(), buffer),
        Point::Val38 => serialisation::write_u16(model.val38(), buffer),
        Point::Val39 => serialisation::write_u16(model.val39(), buffer),
        Point::Val40 => serialisation::write_u16(model.val40(), buffer),
        Point::Val41 => serialisation::write_u16(model.val41(), buffer),
        Point::Val42 => serialisation::write_u16(model.val42(), buffer),
        Point::Val43 => serialisation::write_u16(model.val43(), buffer),
        Point::Val44 => serialisation::write_u16(model.val44(), buffer),
        Point::Val45 => serialisation::write_u16(model.val45(), buffer),
        Point::Val46 => serialisation::write_u16(model.val46(), buffer),
        Point::Val47 => serialisation::write_u16(model.val47(), buffer),
        Point::Val48 => serialisation::write_u16(model.val48(), buffer),
        Point::Val49 => serialisation::write_u16(model.val49(), buffer),
        Point::Val50 => serialisation::write_u16(model.val50(), buffer),
        Point::Val51 => serialisation::write_u16(model.val51(), buffer),
        Point::Val52 => serialisation::write_u16(model.val52(), buffer),
        Point::Val53 => serialisation::write_u16(model.val53(), buffer),
        Point::Val54 => serialisation::write_u16(model.val54(), buffer),
        Point::Val55 => serialisation::write_u16(model.val55(), buffer),
        Point::Val56 => serialisation::write_u16(model.val56(), buffer),
        Point::Val57 => serialisation::write_u16(model.val57(), buffer),
        Point::Val58 => serialisation::write_u16(model.val58(), buffer),
        Point::Val59 => serialisation::write_u16(model.val59(), buffer),
        Point::Val60 => serialisation::write_u16(model.val60(), buffer),
        Point::Val61 => serialisation::write_u16(model.val61(), buffer),
        Point::Val62 => serialisation::write_u16(model.val62(), buffer),
        Point::Val63 => serialisation::write_u16(model.val63(), buffer),
        Point::Val64 => serialisation::write_u16(model.val64(), buffer),
        Point::Val65 => serialisation::write_u16(model.val65(), buffer),
        Point::Val66 => serialisation::write_u16(model.val66(), buffer),
        Point::Val67 => serialisation::write_u16(model.val67(), buffer),
        Point::Val68 => serialisation::write_u16(model.val68(), buffer),
        Point::Val69 => serialisation::write_u16(model.val69(), buffer),
        Point::Val70 => serialisation::write_u16(model.val70(), buffer),
        Point::Val71 => serialisation::write_u16(model.val71(), buffer),
        Point::Val72 => serialisation::write_u16(model.val72(), buffer),
        Point::Val73 => serialisation::write_u16(model.val73(), buffer),
        Point::Val74 => serialisation::write_u16(model.val74(), buffer),
        Point::Val75 => serialisation::write_u16(model.val75(), buffer),
        Point::Val76 => serialisation::write_u16(model.val76(), buffer),
        Point::Val77 => serialisation::write_u16(model.val77(), buffer),
        Point::Val78 => serialisation::write_u16(model.val78(), buffer),
        Point::Val79 => serialisation::write_u16(model.val79(), buffer),
        Point::Val80 => serialisation::write_u16(model.val80(), buffer),
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

    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// X values to follow
    fn offset(&self) -> u16;

    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// X values to follow
    fn set_offset(&mut self, value: u16);

    /// Value1
    ///
    /// Value to write to control register at offset
    fn value1(&self) -> u16;

    /// Value1
    ///
    /// Value to write to control register at offset
    fn set_value1(&mut self, value: u16);

    fn val2(&self) -> u16;

    fn set_val2(&mut self, value: u16);

    fn val3(&self) -> u16;

    fn set_val3(&mut self, value: u16);

    fn val4(&self) -> u16;

    fn set_val4(&mut self, value: u16);

    fn val5(&self) -> u16;

    fn set_val5(&mut self, value: u16);

    fn val6(&self) -> u16;

    fn set_val6(&mut self, value: u16);

    fn val7(&self) -> u16;

    fn set_val7(&mut self, value: u16);

    fn val8(&self) -> u16;

    fn set_val8(&mut self, value: u16);

    fn val9(&self) -> u16;

    fn set_val9(&mut self, value: u16);

    fn val10(&self) -> u16;

    fn set_val10(&mut self, value: u16);

    fn val11(&self) -> u16;

    fn set_val11(&mut self, value: u16);

    fn val12(&self) -> u16;

    fn set_val12(&mut self, value: u16);

    fn val13(&self) -> u16;

    fn set_val13(&mut self, value: u16);

    fn val14(&self) -> u16;

    fn set_val14(&mut self, value: u16);

    fn val15(&self) -> u16;

    fn set_val15(&mut self, value: u16);

    fn val16(&self) -> u16;

    fn set_val16(&mut self, value: u16);

    fn val17(&self) -> u16;

    fn set_val17(&mut self, value: u16);

    fn val18(&self) -> u16;

    fn set_val18(&mut self, value: u16);

    fn val19(&self) -> u16;

    fn set_val19(&mut self, value: u16);

    fn val20(&self) -> u16;

    fn set_val20(&mut self, value: u16);

    fn val21(&self) -> u16;

    fn set_val21(&mut self, value: u16);

    fn val22(&self) -> u16;

    fn set_val22(&mut self, value: u16);

    fn val23(&self) -> u16;

    fn set_val23(&mut self, value: u16);

    fn val24(&self) -> u16;

    fn set_val24(&mut self, value: u16);

    fn val25(&self) -> u16;

    fn set_val25(&mut self, value: u16);

    fn val26(&self) -> u16;

    fn set_val26(&mut self, value: u16);

    fn val27(&self) -> u16;

    fn set_val27(&mut self, value: u16);

    fn val28(&self) -> u16;

    fn set_val28(&mut self, value: u16);

    fn val29(&self) -> u16;

    fn set_val29(&mut self, value: u16);

    fn val30(&self) -> u16;

    fn set_val30(&mut self, value: u16);

    fn val31(&self) -> u16;

    fn set_val31(&mut self, value: u16);

    fn val32(&self) -> u16;

    fn set_val32(&mut self, value: u16);

    fn val33(&self) -> u16;

    fn set_val33(&mut self, value: u16);

    fn val34(&self) -> u16;

    fn set_val34(&mut self, value: u16);

    fn val35(&self) -> u16;

    fn set_val35(&mut self, value: u16);

    fn val36(&self) -> u16;

    fn set_val36(&mut self, value: u16);

    fn val37(&self) -> u16;

    fn set_val37(&mut self, value: u16);

    fn val38(&self) -> u16;

    fn set_val38(&mut self, value: u16);

    fn val39(&self) -> u16;

    fn set_val39(&mut self, value: u16);

    fn val40(&self) -> u16;

    fn set_val40(&mut self, value: u16);

    fn val41(&self) -> u16;

    fn set_val41(&mut self, value: u16);

    fn val42(&self) -> u16;

    fn set_val42(&mut self, value: u16);

    fn val43(&self) -> u16;

    fn set_val43(&mut self, value: u16);

    fn val44(&self) -> u16;

    fn set_val44(&mut self, value: u16);

    fn val45(&self) -> u16;

    fn set_val45(&mut self, value: u16);

    fn val46(&self) -> u16;

    fn set_val46(&mut self, value: u16);

    fn val47(&self) -> u16;

    fn set_val47(&mut self, value: u16);

    fn val48(&self) -> u16;

    fn set_val48(&mut self, value: u16);

    fn val49(&self) -> u16;

    fn set_val49(&mut self, value: u16);

    fn val50(&self) -> u16;

    fn set_val50(&mut self, value: u16);

    fn val51(&self) -> u16;

    fn set_val51(&mut self, value: u16);

    fn val52(&self) -> u16;

    fn set_val52(&mut self, value: u16);

    fn val53(&self) -> u16;

    fn set_val53(&mut self, value: u16);

    fn val54(&self) -> u16;

    fn set_val54(&mut self, value: u16);

    fn val55(&self) -> u16;

    fn set_val55(&mut self, value: u16);

    fn val56(&self) -> u16;

    fn set_val56(&mut self, value: u16);

    fn val57(&self) -> u16;

    fn set_val57(&mut self, value: u16);

    fn val58(&self) -> u16;

    fn set_val58(&mut self, value: u16);

    fn val59(&self) -> u16;

    fn set_val59(&mut self, value: u16);

    fn val60(&self) -> u16;

    fn set_val60(&mut self, value: u16);

    fn val61(&self) -> u16;

    fn set_val61(&mut self, value: u16);

    fn val62(&self) -> u16;

    fn set_val62(&mut self, value: u16);

    fn val63(&self) -> u16;

    fn set_val63(&mut self, value: u16);

    fn val64(&self) -> u16;

    fn set_val64(&mut self, value: u16);

    fn val65(&self) -> u16;

    fn set_val65(&mut self, value: u16);

    fn val66(&self) -> u16;

    fn set_val66(&mut self, value: u16);

    fn val67(&self) -> u16;

    fn set_val67(&mut self, value: u16);

    fn val68(&self) -> u16;

    fn set_val68(&mut self, value: u16);

    fn val69(&self) -> u16;

    fn set_val69(&mut self, value: u16);

    fn val70(&self) -> u16;

    fn set_val70(&mut self, value: u16);

    fn val71(&self) -> u16;

    fn set_val71(&mut self, value: u16);

    fn val72(&self) -> u16;

    fn set_val72(&mut self, value: u16);

    fn val73(&self) -> u16;

    fn set_val73(&mut self, value: u16);

    fn val74(&self) -> u16;

    fn set_val74(&mut self, value: u16);

    fn val75(&self) -> u16;

    fn set_val75(&mut self, value: u16);

    fn val76(&self) -> u16;

    fn set_val76(&mut self, value: u16);

    fn val77(&self) -> u16;

    fn set_val77(&mut self, value: u16);

    fn val78(&self) -> u16;

    fn set_val78(&mut self, value: u16);

    fn val79(&self) -> u16;

    fn set_val79(&mut self, value: u16);

    fn val80(&self) -> u16;

    fn set_val80(&mut self, value: u16);

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

#[repr(u16)]
pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}

#[repr(C)]
pub struct Model6CallbackAdapter {
    x_callback: extern "C" fn() -> u16,
    set_x_callback: extern "C" fn(u16),
    offset_callback: extern "C" fn() -> u16,
    set_offset_callback: extern "C" fn(u16),
    value1_callback: extern "C" fn() -> u16,
    set_value1_callback: extern "C" fn(u16),
    val2_callback: extern "C" fn() -> u16,
    set_val2_callback: extern "C" fn(u16),
    val3_callback: extern "C" fn() -> u16,
    set_val3_callback: extern "C" fn(u16),
    val4_callback: extern "C" fn() -> u16,
    set_val4_callback: extern "C" fn(u16),
    val5_callback: extern "C" fn() -> u16,
    set_val5_callback: extern "C" fn(u16),
    val6_callback: extern "C" fn() -> u16,
    set_val6_callback: extern "C" fn(u16),
    val7_callback: extern "C" fn() -> u16,
    set_val7_callback: extern "C" fn(u16),
    val8_callback: extern "C" fn() -> u16,
    set_val8_callback: extern "C" fn(u16),
    val9_callback: extern "C" fn() -> u16,
    set_val9_callback: extern "C" fn(u16),
    val10_callback: extern "C" fn() -> u16,
    set_val10_callback: extern "C" fn(u16),
    val11_callback: extern "C" fn() -> u16,
    set_val11_callback: extern "C" fn(u16),
    val12_callback: extern "C" fn() -> u16,
    set_val12_callback: extern "C" fn(u16),
    val13_callback: extern "C" fn() -> u16,
    set_val13_callback: extern "C" fn(u16),
    val14_callback: extern "C" fn() -> u16,
    set_val14_callback: extern "C" fn(u16),
    val15_callback: extern "C" fn() -> u16,
    set_val15_callback: extern "C" fn(u16),
    val16_callback: extern "C" fn() -> u16,
    set_val16_callback: extern "C" fn(u16),
    val17_callback: extern "C" fn() -> u16,
    set_val17_callback: extern "C" fn(u16),
    val18_callback: extern "C" fn() -> u16,
    set_val18_callback: extern "C" fn(u16),
    val19_callback: extern "C" fn() -> u16,
    set_val19_callback: extern "C" fn(u16),
    val20_callback: extern "C" fn() -> u16,
    set_val20_callback: extern "C" fn(u16),
    val21_callback: extern "C" fn() -> u16,
    set_val21_callback: extern "C" fn(u16),
    val22_callback: extern "C" fn() -> u16,
    set_val22_callback: extern "C" fn(u16),
    val23_callback: extern "C" fn() -> u16,
    set_val23_callback: extern "C" fn(u16),
    val24_callback: extern "C" fn() -> u16,
    set_val24_callback: extern "C" fn(u16),
    val25_callback: extern "C" fn() -> u16,
    set_val25_callback: extern "C" fn(u16),
    val26_callback: extern "C" fn() -> u16,
    set_val26_callback: extern "C" fn(u16),
    val27_callback: extern "C" fn() -> u16,
    set_val27_callback: extern "C" fn(u16),
    val28_callback: extern "C" fn() -> u16,
    set_val28_callback: extern "C" fn(u16),
    val29_callback: extern "C" fn() -> u16,
    set_val29_callback: extern "C" fn(u16),
    val30_callback: extern "C" fn() -> u16,
    set_val30_callback: extern "C" fn(u16),
    val31_callback: extern "C" fn() -> u16,
    set_val31_callback: extern "C" fn(u16),
    val32_callback: extern "C" fn() -> u16,
    set_val32_callback: extern "C" fn(u16),
    val33_callback: extern "C" fn() -> u16,
    set_val33_callback: extern "C" fn(u16),
    val34_callback: extern "C" fn() -> u16,
    set_val34_callback: extern "C" fn(u16),
    val35_callback: extern "C" fn() -> u16,
    set_val35_callback: extern "C" fn(u16),
    val36_callback: extern "C" fn() -> u16,
    set_val36_callback: extern "C" fn(u16),
    val37_callback: extern "C" fn() -> u16,
    set_val37_callback: extern "C" fn(u16),
    val38_callback: extern "C" fn() -> u16,
    set_val38_callback: extern "C" fn(u16),
    val39_callback: extern "C" fn() -> u16,
    set_val39_callback: extern "C" fn(u16),
    val40_callback: extern "C" fn() -> u16,
    set_val40_callback: extern "C" fn(u16),
    val41_callback: extern "C" fn() -> u16,
    set_val41_callback: extern "C" fn(u16),
    val42_callback: extern "C" fn() -> u16,
    set_val42_callback: extern "C" fn(u16),
    val43_callback: extern "C" fn() -> u16,
    set_val43_callback: extern "C" fn(u16),
    val44_callback: extern "C" fn() -> u16,
    set_val44_callback: extern "C" fn(u16),
    val45_callback: extern "C" fn() -> u16,
    set_val45_callback: extern "C" fn(u16),
    val46_callback: extern "C" fn() -> u16,
    set_val46_callback: extern "C" fn(u16),
    val47_callback: extern "C" fn() -> u16,
    set_val47_callback: extern "C" fn(u16),
    val48_callback: extern "C" fn() -> u16,
    set_val48_callback: extern "C" fn(u16),
    val49_callback: extern "C" fn() -> u16,
    set_val49_callback: extern "C" fn(u16),
    val50_callback: extern "C" fn() -> u16,
    set_val50_callback: extern "C" fn(u16),
    val51_callback: extern "C" fn() -> u16,
    set_val51_callback: extern "C" fn(u16),
    val52_callback: extern "C" fn() -> u16,
    set_val52_callback: extern "C" fn(u16),
    val53_callback: extern "C" fn() -> u16,
    set_val53_callback: extern "C" fn(u16),
    val54_callback: extern "C" fn() -> u16,
    set_val54_callback: extern "C" fn(u16),
    val55_callback: extern "C" fn() -> u16,
    set_val55_callback: extern "C" fn(u16),
    val56_callback: extern "C" fn() -> u16,
    set_val56_callback: extern "C" fn(u16),
    val57_callback: extern "C" fn() -> u16,
    set_val57_callback: extern "C" fn(u16),
    val58_callback: extern "C" fn() -> u16,
    set_val58_callback: extern "C" fn(u16),
    val59_callback: extern "C" fn() -> u16,
    set_val59_callback: extern "C" fn(u16),
    val60_callback: extern "C" fn() -> u16,
    set_val60_callback: extern "C" fn(u16),
    val61_callback: extern "C" fn() -> u16,
    set_val61_callback: extern "C" fn(u16),
    val62_callback: extern "C" fn() -> u16,
    set_val62_callback: extern "C" fn(u16),
    val63_callback: extern "C" fn() -> u16,
    set_val63_callback: extern "C" fn(u16),
    val64_callback: extern "C" fn() -> u16,
    set_val64_callback: extern "C" fn(u16),
    val65_callback: extern "C" fn() -> u16,
    set_val65_callback: extern "C" fn(u16),
    val66_callback: extern "C" fn() -> u16,
    set_val66_callback: extern "C" fn(u16),
    val67_callback: extern "C" fn() -> u16,
    set_val67_callback: extern "C" fn(u16),
    val68_callback: extern "C" fn() -> u16,
    set_val68_callback: extern "C" fn(u16),
    val69_callback: extern "C" fn() -> u16,
    set_val69_callback: extern "C" fn(u16),
    val70_callback: extern "C" fn() -> u16,
    set_val70_callback: extern "C" fn(u16),
    val71_callback: extern "C" fn() -> u16,
    set_val71_callback: extern "C" fn(u16),
    val72_callback: extern "C" fn() -> u16,
    set_val72_callback: extern "C" fn(u16),
    val73_callback: extern "C" fn() -> u16,
    set_val73_callback: extern "C" fn(u16),
    val74_callback: extern "C" fn() -> u16,
    set_val74_callback: extern "C" fn(u16),
    val75_callback: extern "C" fn() -> u16,
    set_val75_callback: extern "C" fn(u16),
    val76_callback: extern "C" fn() -> u16,
    set_val76_callback: extern "C" fn(u16),
    val77_callback: extern "C" fn() -> u16,
    set_val77_callback: extern "C" fn(u16),
    val78_callback: extern "C" fn() -> u16,
    set_val78_callback: extern "C" fn(u16),
    val79_callback: extern "C" fn() -> u16,
    set_val79_callback: extern "C" fn(u16),
    val80_callback: extern "C" fn() -> u16,
    set_val80_callback: extern "C" fn(u16),
    timestamp_callback: extern "C" fn() -> u32,
    set_timestamp_callback: extern "C" fn(u32),
    milliseconds_callback: extern "C" fn() -> u16,
    set_milliseconds_callback: extern "C" fn(u16),
    sequence_callback: extern "C" fn() -> u16,
    set_sequence_callback: extern "C" fn(u16),
    role_callback: extern "C" fn() -> u16,
    set_role_callback: extern "C" fn(u16),
    algorithm_callback: extern "C" fn() -> Alg,
    set_algorithm_callback: extern "C" fn(Alg),
    n_callback: extern "C" fn() -> u16,
    set_n_callback: extern "C" fn(u16),
}

impl ModelAdapter for Model6CallbackAdapter {
    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn x(&self) -> u16 {
        (self.x_callback)()
    }

    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn set_x(&mut self, value: u16) {
        (self.set_x_callback)(value);
    }

    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// X values to follow
    fn offset(&self) -> u16 {
        (self.offset_callback)()
    }

    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// X values to follow
    fn set_offset(&mut self, value: u16) {
        (self.set_offset_callback)(value);
    }

    /// Value1
    ///
    /// Value to write to control register at offset
    fn value1(&self) -> u16 {
        (self.value1_callback)()
    }

    /// Value1
    ///
    /// Value to write to control register at offset
    fn set_value1(&mut self, value: u16) {
        (self.set_value1_callback)(value);
    }

    fn val2(&self) -> u16 {
        (self.val2_callback)()
    }

    fn set_val2(&mut self, value: u16) {
        (self.set_val2_callback)(value);
    }

    fn val3(&self) -> u16 {
        (self.val3_callback)()
    }

    fn set_val3(&mut self, value: u16) {
        (self.set_val3_callback)(value);
    }

    fn val4(&self) -> u16 {
        (self.val4_callback)()
    }

    fn set_val4(&mut self, value: u16) {
        (self.set_val4_callback)(value);
    }

    fn val5(&self) -> u16 {
        (self.val5_callback)()
    }

    fn set_val5(&mut self, value: u16) {
        (self.set_val5_callback)(value);
    }

    fn val6(&self) -> u16 {
        (self.val6_callback)()
    }

    fn set_val6(&mut self, value: u16) {
        (self.set_val6_callback)(value);
    }

    fn val7(&self) -> u16 {
        (self.val7_callback)()
    }

    fn set_val7(&mut self, value: u16) {
        (self.set_val7_callback)(value);
    }

    fn val8(&self) -> u16 {
        (self.val8_callback)()
    }

    fn set_val8(&mut self, value: u16) {
        (self.set_val8_callback)(value);
    }

    fn val9(&self) -> u16 {
        (self.val9_callback)()
    }

    fn set_val9(&mut self, value: u16) {
        (self.set_val9_callback)(value);
    }

    fn val10(&self) -> u16 {
        (self.val10_callback)()
    }

    fn set_val10(&mut self, value: u16) {
        (self.set_val10_callback)(value);
    }

    fn val11(&self) -> u16 {
        (self.val11_callback)()
    }

    fn set_val11(&mut self, value: u16) {
        (self.set_val11_callback)(value);
    }

    fn val12(&self) -> u16 {
        (self.val12_callback)()
    }

    fn set_val12(&mut self, value: u16) {
        (self.set_val12_callback)(value);
    }

    fn val13(&self) -> u16 {
        (self.val13_callback)()
    }

    fn set_val13(&mut self, value: u16) {
        (self.set_val13_callback)(value);
    }

    fn val14(&self) -> u16 {
        (self.val14_callback)()
    }

    fn set_val14(&mut self, value: u16) {
        (self.set_val14_callback)(value);
    }

    fn val15(&self) -> u16 {
        (self.val15_callback)()
    }

    fn set_val15(&mut self, value: u16) {
        (self.set_val15_callback)(value);
    }

    fn val16(&self) -> u16 {
        (self.val16_callback)()
    }

    fn set_val16(&mut self, value: u16) {
        (self.set_val16_callback)(value);
    }

    fn val17(&self) -> u16 {
        (self.val17_callback)()
    }

    fn set_val17(&mut self, value: u16) {
        (self.set_val17_callback)(value);
    }

    fn val18(&self) -> u16 {
        (self.val18_callback)()
    }

    fn set_val18(&mut self, value: u16) {
        (self.set_val18_callback)(value);
    }

    fn val19(&self) -> u16 {
        (self.val19_callback)()
    }

    fn set_val19(&mut self, value: u16) {
        (self.set_val19_callback)(value);
    }

    fn val20(&self) -> u16 {
        (self.val20_callback)()
    }

    fn set_val20(&mut self, value: u16) {
        (self.set_val20_callback)(value);
    }

    fn val21(&self) -> u16 {
        (self.val21_callback)()
    }

    fn set_val21(&mut self, value: u16) {
        (self.set_val21_callback)(value);
    }

    fn val22(&self) -> u16 {
        (self.val22_callback)()
    }

    fn set_val22(&mut self, value: u16) {
        (self.set_val22_callback)(value);
    }

    fn val23(&self) -> u16 {
        (self.val23_callback)()
    }

    fn set_val23(&mut self, value: u16) {
        (self.set_val23_callback)(value);
    }

    fn val24(&self) -> u16 {
        (self.val24_callback)()
    }

    fn set_val24(&mut self, value: u16) {
        (self.set_val24_callback)(value);
    }

    fn val25(&self) -> u16 {
        (self.val25_callback)()
    }

    fn set_val25(&mut self, value: u16) {
        (self.set_val25_callback)(value);
    }

    fn val26(&self) -> u16 {
        (self.val26_callback)()
    }

    fn set_val26(&mut self, value: u16) {
        (self.set_val26_callback)(value);
    }

    fn val27(&self) -> u16 {
        (self.val27_callback)()
    }

    fn set_val27(&mut self, value: u16) {
        (self.set_val27_callback)(value);
    }

    fn val28(&self) -> u16 {
        (self.val28_callback)()
    }

    fn set_val28(&mut self, value: u16) {
        (self.set_val28_callback)(value);
    }

    fn val29(&self) -> u16 {
        (self.val29_callback)()
    }

    fn set_val29(&mut self, value: u16) {
        (self.set_val29_callback)(value);
    }

    fn val30(&self) -> u16 {
        (self.val30_callback)()
    }

    fn set_val30(&mut self, value: u16) {
        (self.set_val30_callback)(value);
    }

    fn val31(&self) -> u16 {
        (self.val31_callback)()
    }

    fn set_val31(&mut self, value: u16) {
        (self.set_val31_callback)(value);
    }

    fn val32(&self) -> u16 {
        (self.val32_callback)()
    }

    fn set_val32(&mut self, value: u16) {
        (self.set_val32_callback)(value);
    }

    fn val33(&self) -> u16 {
        (self.val33_callback)()
    }

    fn set_val33(&mut self, value: u16) {
        (self.set_val33_callback)(value);
    }

    fn val34(&self) -> u16 {
        (self.val34_callback)()
    }

    fn set_val34(&mut self, value: u16) {
        (self.set_val34_callback)(value);
    }

    fn val35(&self) -> u16 {
        (self.val35_callback)()
    }

    fn set_val35(&mut self, value: u16) {
        (self.set_val35_callback)(value);
    }

    fn val36(&self) -> u16 {
        (self.val36_callback)()
    }

    fn set_val36(&mut self, value: u16) {
        (self.set_val36_callback)(value);
    }

    fn val37(&self) -> u16 {
        (self.val37_callback)()
    }

    fn set_val37(&mut self, value: u16) {
        (self.set_val37_callback)(value);
    }

    fn val38(&self) -> u16 {
        (self.val38_callback)()
    }

    fn set_val38(&mut self, value: u16) {
        (self.set_val38_callback)(value);
    }

    fn val39(&self) -> u16 {
        (self.val39_callback)()
    }

    fn set_val39(&mut self, value: u16) {
        (self.set_val39_callback)(value);
    }

    fn val40(&self) -> u16 {
        (self.val40_callback)()
    }

    fn set_val40(&mut self, value: u16) {
        (self.set_val40_callback)(value);
    }

    fn val41(&self) -> u16 {
        (self.val41_callback)()
    }

    fn set_val41(&mut self, value: u16) {
        (self.set_val41_callback)(value);
    }

    fn val42(&self) -> u16 {
        (self.val42_callback)()
    }

    fn set_val42(&mut self, value: u16) {
        (self.set_val42_callback)(value);
    }

    fn val43(&self) -> u16 {
        (self.val43_callback)()
    }

    fn set_val43(&mut self, value: u16) {
        (self.set_val43_callback)(value);
    }

    fn val44(&self) -> u16 {
        (self.val44_callback)()
    }

    fn set_val44(&mut self, value: u16) {
        (self.set_val44_callback)(value);
    }

    fn val45(&self) -> u16 {
        (self.val45_callback)()
    }

    fn set_val45(&mut self, value: u16) {
        (self.set_val45_callback)(value);
    }

    fn val46(&self) -> u16 {
        (self.val46_callback)()
    }

    fn set_val46(&mut self, value: u16) {
        (self.set_val46_callback)(value);
    }

    fn val47(&self) -> u16 {
        (self.val47_callback)()
    }

    fn set_val47(&mut self, value: u16) {
        (self.set_val47_callback)(value);
    }

    fn val48(&self) -> u16 {
        (self.val48_callback)()
    }

    fn set_val48(&mut self, value: u16) {
        (self.set_val48_callback)(value);
    }

    fn val49(&self) -> u16 {
        (self.val49_callback)()
    }

    fn set_val49(&mut self, value: u16) {
        (self.set_val49_callback)(value);
    }

    fn val50(&self) -> u16 {
        (self.val50_callback)()
    }

    fn set_val50(&mut self, value: u16) {
        (self.set_val50_callback)(value);
    }

    fn val51(&self) -> u16 {
        (self.val51_callback)()
    }

    fn set_val51(&mut self, value: u16) {
        (self.set_val51_callback)(value);
    }

    fn val52(&self) -> u16 {
        (self.val52_callback)()
    }

    fn set_val52(&mut self, value: u16) {
        (self.set_val52_callback)(value);
    }

    fn val53(&self) -> u16 {
        (self.val53_callback)()
    }

    fn set_val53(&mut self, value: u16) {
        (self.set_val53_callback)(value);
    }

    fn val54(&self) -> u16 {
        (self.val54_callback)()
    }

    fn set_val54(&mut self, value: u16) {
        (self.set_val54_callback)(value);
    }

    fn val55(&self) -> u16 {
        (self.val55_callback)()
    }

    fn set_val55(&mut self, value: u16) {
        (self.set_val55_callback)(value);
    }

    fn val56(&self) -> u16 {
        (self.val56_callback)()
    }

    fn set_val56(&mut self, value: u16) {
        (self.set_val56_callback)(value);
    }

    fn val57(&self) -> u16 {
        (self.val57_callback)()
    }

    fn set_val57(&mut self, value: u16) {
        (self.set_val57_callback)(value);
    }

    fn val58(&self) -> u16 {
        (self.val58_callback)()
    }

    fn set_val58(&mut self, value: u16) {
        (self.set_val58_callback)(value);
    }

    fn val59(&self) -> u16 {
        (self.val59_callback)()
    }

    fn set_val59(&mut self, value: u16) {
        (self.set_val59_callback)(value);
    }

    fn val60(&self) -> u16 {
        (self.val60_callback)()
    }

    fn set_val60(&mut self, value: u16) {
        (self.set_val60_callback)(value);
    }

    fn val61(&self) -> u16 {
        (self.val61_callback)()
    }

    fn set_val61(&mut self, value: u16) {
        (self.set_val61_callback)(value);
    }

    fn val62(&self) -> u16 {
        (self.val62_callback)()
    }

    fn set_val62(&mut self, value: u16) {
        (self.set_val62_callback)(value);
    }

    fn val63(&self) -> u16 {
        (self.val63_callback)()
    }

    fn set_val63(&mut self, value: u16) {
        (self.set_val63_callback)(value);
    }

    fn val64(&self) -> u16 {
        (self.val64_callback)()
    }

    fn set_val64(&mut self, value: u16) {
        (self.set_val64_callback)(value);
    }

    fn val65(&self) -> u16 {
        (self.val65_callback)()
    }

    fn set_val65(&mut self, value: u16) {
        (self.set_val65_callback)(value);
    }

    fn val66(&self) -> u16 {
        (self.val66_callback)()
    }

    fn set_val66(&mut self, value: u16) {
        (self.set_val66_callback)(value);
    }

    fn val67(&self) -> u16 {
        (self.val67_callback)()
    }

    fn set_val67(&mut self, value: u16) {
        (self.set_val67_callback)(value);
    }

    fn val68(&self) -> u16 {
        (self.val68_callback)()
    }

    fn set_val68(&mut self, value: u16) {
        (self.set_val68_callback)(value);
    }

    fn val69(&self) -> u16 {
        (self.val69_callback)()
    }

    fn set_val69(&mut self, value: u16) {
        (self.set_val69_callback)(value);
    }

    fn val70(&self) -> u16 {
        (self.val70_callback)()
    }

    fn set_val70(&mut self, value: u16) {
        (self.set_val70_callback)(value);
    }

    fn val71(&self) -> u16 {
        (self.val71_callback)()
    }

    fn set_val71(&mut self, value: u16) {
        (self.set_val71_callback)(value);
    }

    fn val72(&self) -> u16 {
        (self.val72_callback)()
    }

    fn set_val72(&mut self, value: u16) {
        (self.set_val72_callback)(value);
    }

    fn val73(&self) -> u16 {
        (self.val73_callback)()
    }

    fn set_val73(&mut self, value: u16) {
        (self.set_val73_callback)(value);
    }

    fn val74(&self) -> u16 {
        (self.val74_callback)()
    }

    fn set_val74(&mut self, value: u16) {
        (self.set_val74_callback)(value);
    }

    fn val75(&self) -> u16 {
        (self.val75_callback)()
    }

    fn set_val75(&mut self, value: u16) {
        (self.set_val75_callback)(value);
    }

    fn val76(&self) -> u16 {
        (self.val76_callback)()
    }

    fn set_val76(&mut self, value: u16) {
        (self.set_val76_callback)(value);
    }

    fn val77(&self) -> u16 {
        (self.val77_callback)()
    }

    fn set_val77(&mut self, value: u16) {
        (self.set_val77_callback)(value);
    }

    fn val78(&self) -> u16 {
        (self.val78_callback)()
    }

    fn set_val78(&mut self, value: u16) {
        (self.set_val78_callback)(value);
    }

    fn val79(&self) -> u16 {
        (self.val79_callback)()
    }

    fn set_val79(&mut self, value: u16) {
        (self.set_val79_callback)(value);
    }

    fn val80(&self) -> u16 {
        (self.val80_callback)()
    }

    fn set_val80(&mut self, value: u16) {
        (self.set_val80_callback)(value);
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        (self.timestamp_callback)()
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn set_timestamp(&mut self, value: u32) {
        (self.set_timestamp_callback)(value);
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        (self.milliseconds_callback)()
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn set_milliseconds(&mut self, value: u16) {
        (self.set_milliseconds_callback)(value);
    }

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn sequence(&self) -> u16 {
        (self.sequence_callback)()
    }

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn set_sequence(&mut self, value: u16) {
        (self.set_sequence_callback)(value);
    }

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn role(&self) -> u16 {
        (self.role_callback)()
    }

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn set_role(&mut self, value: u16) {
        (self.set_role_callback)(value);
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg {
        (self.algorithm_callback)()
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn set_algorithm(&mut self, value: Alg) {
        (self.set_algorithm_callback)(value);
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        (self.n_callback)()
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn set_n(&mut self, value: u16) {
        (self.set_n_callback)(value);
    }
}
