use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 60;

pub static POINTS: [ReadablePoint; 59] = [
    ReadablePoint {
        reference: PointReference::Static { value: 3 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 58 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::X },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Offset1 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off2 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off3 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off4 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off5 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off6 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off7 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off9 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off10 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off11 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off12 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off13 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off14 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off15 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off16 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off17 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off18 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off19 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off20 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off21 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off22 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off23 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off24 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off25 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off26 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off27 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off28 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off29 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off30 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off31 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off32 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off33 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off34 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off35 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off36 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off37 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off38 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off39 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off40 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off41 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off42 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off43 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off44 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off45 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off46 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off47 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off48 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off49 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Off50 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Timestamp },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Milliseconds },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Sequence },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Role },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::Algorithm },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model3 { point: Point::N },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    X,
    Offset1,
    Off2,
    Off3,
    Off4,
    Off5,
    Off6,
    Off7,
    Off8,
    Off9,
    Off10,
    Off11,
    Off12,
    Off13,
    Off14,
    Off15,
    Off16,
    Off17,
    Off18,
    Off19,
    Off20,
    Off21,
    Off22,
    Off23,
    Off24,
    Off25,
    Off26,
    Off27,
    Off28,
    Off29,
    Off30,
    Off31,
    Off32,
    Off33,
    Off34,
    Off35,
    Off36,
    Off37,
    Off38,
    Off39,
    Off40,
    Off41,
    Off42,
    Off43,
    Off44,
    Off45,
    Off46,
    Off47,
    Off48,
    Off49,
    Off50,
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
        Point::Off2 => serialisation::write_u16(model.off2(), buffer),
        Point::Off3 => serialisation::write_u16(model.off3(), buffer),
        Point::Off4 => serialisation::write_u16(model.off4(), buffer),
        Point::Off5 => serialisation::write_u16(model.off5(), buffer),
        Point::Off6 => serialisation::write_u16(model.off6(), buffer),
        Point::Off7 => serialisation::write_u16(model.off7(), buffer),
        Point::Off8 => serialisation::write_u16(model.off8(), buffer),
        Point::Off9 => serialisation::write_u16(model.off9(), buffer),
        Point::Off10 => serialisation::write_u16(model.off10(), buffer),
        Point::Off11 => serialisation::write_u16(model.off11(), buffer),
        Point::Off12 => serialisation::write_u16(model.off12(), buffer),
        Point::Off13 => serialisation::write_u16(model.off13(), buffer),
        Point::Off14 => serialisation::write_u16(model.off14(), buffer),
        Point::Off15 => serialisation::write_u16(model.off15(), buffer),
        Point::Off16 => serialisation::write_u16(model.off16(), buffer),
        Point::Off17 => serialisation::write_u16(model.off17(), buffer),
        Point::Off18 => serialisation::write_u16(model.off18(), buffer),
        Point::Off19 => serialisation::write_u16(model.off19(), buffer),
        Point::Off20 => serialisation::write_u16(model.off20(), buffer),
        Point::Off21 => serialisation::write_u16(model.off21(), buffer),
        Point::Off22 => serialisation::write_u16(model.off22(), buffer),
        Point::Off23 => serialisation::write_u16(model.off23(), buffer),
        Point::Off24 => serialisation::write_u16(model.off24(), buffer),
        Point::Off25 => serialisation::write_u16(model.off25(), buffer),
        Point::Off26 => serialisation::write_u16(model.off26(), buffer),
        Point::Off27 => serialisation::write_u16(model.off27(), buffer),
        Point::Off28 => serialisation::write_u16(model.off28(), buffer),
        Point::Off29 => serialisation::write_u16(model.off29(), buffer),
        Point::Off30 => serialisation::write_u16(model.off30(), buffer),
        Point::Off31 => serialisation::write_u16(model.off31(), buffer),
        Point::Off32 => serialisation::write_u16(model.off32(), buffer),
        Point::Off33 => serialisation::write_u16(model.off33(), buffer),
        Point::Off34 => serialisation::write_u16(model.off34(), buffer),
        Point::Off35 => serialisation::write_u16(model.off35(), buffer),
        Point::Off36 => serialisation::write_u16(model.off36(), buffer),
        Point::Off37 => serialisation::write_u16(model.off37(), buffer),
        Point::Off38 => serialisation::write_u16(model.off38(), buffer),
        Point::Off39 => serialisation::write_u16(model.off39(), buffer),
        Point::Off40 => serialisation::write_u16(model.off40(), buffer),
        Point::Off41 => serialisation::write_u16(model.off41(), buffer),
        Point::Off42 => serialisation::write_u16(model.off42(), buffer),
        Point::Off43 => serialisation::write_u16(model.off43(), buffer),
        Point::Off44 => serialisation::write_u16(model.off44(), buffer),
        Point::Off45 => serialisation::write_u16(model.off45(), buffer),
        Point::Off46 => serialisation::write_u16(model.off46(), buffer),
        Point::Off47 => serialisation::write_u16(model.off47(), buffer),
        Point::Off48 => serialisation::write_u16(model.off48(), buffer),
        Point::Off49 => serialisation::write_u16(model.off49(), buffer),
        Point::Off50 => serialisation::write_u16(model.off50(), buffer),
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
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    fn x(&self) -> u16;

    /// X
    ///
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    fn set_x(&mut self, value: u16);

    /// Offset1
    ///
    /// Offset of value to read
    fn offset1(&self) -> u16;

    /// Offset1
    ///
    /// Offset of value to read
    fn set_offset1(&mut self, value: u16);

    fn off2(&self) -> u16;

    fn set_off2(&mut self, value: u16);

    fn off3(&self) -> u16;

    fn set_off3(&mut self, value: u16);

    fn off4(&self) -> u16;

    fn set_off4(&mut self, value: u16);

    fn off5(&self) -> u16;

    fn set_off5(&mut self, value: u16);

    fn off6(&self) -> u16;

    fn set_off6(&mut self, value: u16);

    fn off7(&self) -> u16;

    fn set_off7(&mut self, value: u16);

    fn off8(&self) -> u16;

    fn set_off8(&mut self, value: u16);

    fn off9(&self) -> u16;

    fn set_off9(&mut self, value: u16);

    fn off10(&self) -> u16;

    fn set_off10(&mut self, value: u16);

    fn off11(&self) -> u16;

    fn set_off11(&mut self, value: u16);

    fn off12(&self) -> u16;

    fn set_off12(&mut self, value: u16);

    fn off13(&self) -> u16;

    fn set_off13(&mut self, value: u16);

    fn off14(&self) -> u16;

    fn set_off14(&mut self, value: u16);

    fn off15(&self) -> u16;

    fn set_off15(&mut self, value: u16);

    fn off16(&self) -> u16;

    fn set_off16(&mut self, value: u16);

    fn off17(&self) -> u16;

    fn set_off17(&mut self, value: u16);

    fn off18(&self) -> u16;

    fn set_off18(&mut self, value: u16);

    fn off19(&self) -> u16;

    fn set_off19(&mut self, value: u16);

    fn off20(&self) -> u16;

    fn set_off20(&mut self, value: u16);

    fn off21(&self) -> u16;

    fn set_off21(&mut self, value: u16);

    fn off22(&self) -> u16;

    fn set_off22(&mut self, value: u16);

    fn off23(&self) -> u16;

    fn set_off23(&mut self, value: u16);

    fn off24(&self) -> u16;

    fn set_off24(&mut self, value: u16);

    fn off25(&self) -> u16;

    fn set_off25(&mut self, value: u16);

    fn off26(&self) -> u16;

    fn set_off26(&mut self, value: u16);

    fn off27(&self) -> u16;

    fn set_off27(&mut self, value: u16);

    fn off28(&self) -> u16;

    fn set_off28(&mut self, value: u16);

    fn off29(&self) -> u16;

    fn set_off29(&mut self, value: u16);

    fn off30(&self) -> u16;

    fn set_off30(&mut self, value: u16);

    fn off31(&self) -> u16;

    fn set_off31(&mut self, value: u16);

    fn off32(&self) -> u16;

    fn set_off32(&mut self, value: u16);

    fn off33(&self) -> u16;

    fn set_off33(&mut self, value: u16);

    fn off34(&self) -> u16;

    fn set_off34(&mut self, value: u16);

    fn off35(&self) -> u16;

    fn set_off35(&mut self, value: u16);

    fn off36(&self) -> u16;

    fn set_off36(&mut self, value: u16);

    fn off37(&self) -> u16;

    fn set_off37(&mut self, value: u16);

    fn off38(&self) -> u16;

    fn set_off38(&mut self, value: u16);

    fn off39(&self) -> u16;

    fn set_off39(&mut self, value: u16);

    fn off40(&self) -> u16;

    fn set_off40(&mut self, value: u16);

    fn off41(&self) -> u16;

    fn set_off41(&mut self, value: u16);

    fn off42(&self) -> u16;

    fn set_off42(&mut self, value: u16);

    fn off43(&self) -> u16;

    fn set_off43(&mut self, value: u16);

    fn off44(&self) -> u16;

    fn set_off44(&mut self, value: u16);

    fn off45(&self) -> u16;

    fn set_off45(&mut self, value: u16);

    fn off46(&self) -> u16;

    fn set_off46(&mut self, value: u16);

    fn off47(&self) -> u16;

    fn set_off47(&mut self, value: u16);

    fn off48(&self) -> u16;

    fn set_off48(&mut self, value: u16);

    fn off49(&self) -> u16;

    fn set_off49(&mut self, value: u16);

    fn off50(&self) -> u16;

    fn set_off50(&mut self, value: u16);

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
    /// Digital Signature ID
    ///
    /// User's role id 0-5
    fn role(&self) -> u16;

    /// Role
    ///
    /// Digital Signature ID
    ///
    /// User's role id 0-5
    fn set_role(&mut self, value: u16);

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg;

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16;
}

pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}