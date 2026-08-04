use core::ffi::c_void;
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
        Point::X => {
            serialisation::write_u16(model.x(), buffer);
        },
        Point::Offset1 => {
            serialisation::write_u16(model.offset1(), buffer);
        },
        Point::Off2 => {
            serialisation::write_u16(model.off2(), buffer);
        },
        Point::Off3 => {
            serialisation::write_u16(model.off3(), buffer);
        },
        Point::Off4 => {
            serialisation::write_u16(model.off4(), buffer);
        },
        Point::Off5 => {
            serialisation::write_u16(model.off5(), buffer);
        },
        Point::Off6 => {
            serialisation::write_u16(model.off6(), buffer);
        },
        Point::Off7 => {
            serialisation::write_u16(model.off7(), buffer);
        },
        Point::Off8 => {
            serialisation::write_u16(model.off8(), buffer);
        },
        Point::Off9 => {
            serialisation::write_u16(model.off9(), buffer);
        },
        Point::Off10 => {
            serialisation::write_u16(model.off10(), buffer);
        },
        Point::Off11 => {
            serialisation::write_u16(model.off11(), buffer);
        },
        Point::Off12 => {
            serialisation::write_u16(model.off12(), buffer);
        },
        Point::Off13 => {
            serialisation::write_u16(model.off13(), buffer);
        },
        Point::Off14 => {
            serialisation::write_u16(model.off14(), buffer);
        },
        Point::Off15 => {
            serialisation::write_u16(model.off15(), buffer);
        },
        Point::Off16 => {
            serialisation::write_u16(model.off16(), buffer);
        },
        Point::Off17 => {
            serialisation::write_u16(model.off17(), buffer);
        },
        Point::Off18 => {
            serialisation::write_u16(model.off18(), buffer);
        },
        Point::Off19 => {
            serialisation::write_u16(model.off19(), buffer);
        },
        Point::Off20 => {
            serialisation::write_u16(model.off20(), buffer);
        },
        Point::Off21 => {
            serialisation::write_u16(model.off21(), buffer);
        },
        Point::Off22 => {
            serialisation::write_u16(model.off22(), buffer);
        },
        Point::Off23 => {
            serialisation::write_u16(model.off23(), buffer);
        },
        Point::Off24 => {
            serialisation::write_u16(model.off24(), buffer);
        },
        Point::Off25 => {
            serialisation::write_u16(model.off25(), buffer);
        },
        Point::Off26 => {
            serialisation::write_u16(model.off26(), buffer);
        },
        Point::Off27 => {
            serialisation::write_u16(model.off27(), buffer);
        },
        Point::Off28 => {
            serialisation::write_u16(model.off28(), buffer);
        },
        Point::Off29 => {
            serialisation::write_u16(model.off29(), buffer);
        },
        Point::Off30 => {
            serialisation::write_u16(model.off30(), buffer);
        },
        Point::Off31 => {
            serialisation::write_u16(model.off31(), buffer);
        },
        Point::Off32 => {
            serialisation::write_u16(model.off32(), buffer);
        },
        Point::Off33 => {
            serialisation::write_u16(model.off33(), buffer);
        },
        Point::Off34 => {
            serialisation::write_u16(model.off34(), buffer);
        },
        Point::Off35 => {
            serialisation::write_u16(model.off35(), buffer);
        },
        Point::Off36 => {
            serialisation::write_u16(model.off36(), buffer);
        },
        Point::Off37 => {
            serialisation::write_u16(model.off37(), buffer);
        },
        Point::Off38 => {
            serialisation::write_u16(model.off38(), buffer);
        },
        Point::Off39 => {
            serialisation::write_u16(model.off39(), buffer);
        },
        Point::Off40 => {
            serialisation::write_u16(model.off40(), buffer);
        },
        Point::Off41 => {
            serialisation::write_u16(model.off41(), buffer);
        },
        Point::Off42 => {
            serialisation::write_u16(model.off42(), buffer);
        },
        Point::Off43 => {
            serialisation::write_u16(model.off43(), buffer);
        },
        Point::Off44 => {
            serialisation::write_u16(model.off44(), buffer);
        },
        Point::Off45 => {
            serialisation::write_u16(model.off45(), buffer);
        },
        Point::Off46 => {
            serialisation::write_u16(model.off46(), buffer);
        },
        Point::Off47 => {
            serialisation::write_u16(model.off47(), buffer);
        },
        Point::Off48 => {
            serialisation::write_u16(model.off48(), buffer);
        },
        Point::Off49 => {
            serialisation::write_u16(model.off49(), buffer);
        },
        Point::Off50 => {
            serialisation::write_u16(model.off50(), buffer);
        },
        Point::Timestamp => {
            serialisation::write_u32(model.timestamp(), buffer, offset, limit);
        },
        Point::Milliseconds => {
            serialisation::write_u16(model.milliseconds(), buffer);
        },
        Point::Sequence => {
            serialisation::write_u16(model.sequence(), buffer);
        },
        Point::Role => {
            serialisation::write_u16(model.role(), buffer);
        },
        Point::Algorithm => {
            serialisation::write_u16(model.algorithm() as u16, buffer);
        },
        Point::N => {
            serialisation::write_u16(model.n(), buffer);
        },
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

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}

#[repr(C)]
pub struct Model3CallbackAdapter {
    context: *mut c_void,
    x_callback: extern "C" fn(*const c_void) -> u16,
    set_x_callback: extern "C" fn(u16, *mut c_void),
    offset1_callback: extern "C" fn(*const c_void) -> u16,
    set_offset1_callback: extern "C" fn(u16, *mut c_void),
    off2_callback: extern "C" fn(*const c_void) -> u16,
    set_off2_callback: extern "C" fn(u16, *mut c_void),
    off3_callback: extern "C" fn(*const c_void) -> u16,
    set_off3_callback: extern "C" fn(u16, *mut c_void),
    off4_callback: extern "C" fn(*const c_void) -> u16,
    set_off4_callback: extern "C" fn(u16, *mut c_void),
    off5_callback: extern "C" fn(*const c_void) -> u16,
    set_off5_callback: extern "C" fn(u16, *mut c_void),
    off6_callback: extern "C" fn(*const c_void) -> u16,
    set_off6_callback: extern "C" fn(u16, *mut c_void),
    off7_callback: extern "C" fn(*const c_void) -> u16,
    set_off7_callback: extern "C" fn(u16, *mut c_void),
    off8_callback: extern "C" fn(*const c_void) -> u16,
    set_off8_callback: extern "C" fn(u16, *mut c_void),
    off9_callback: extern "C" fn(*const c_void) -> u16,
    set_off9_callback: extern "C" fn(u16, *mut c_void),
    off10_callback: extern "C" fn(*const c_void) -> u16,
    set_off10_callback: extern "C" fn(u16, *mut c_void),
    off11_callback: extern "C" fn(*const c_void) -> u16,
    set_off11_callback: extern "C" fn(u16, *mut c_void),
    off12_callback: extern "C" fn(*const c_void) -> u16,
    set_off12_callback: extern "C" fn(u16, *mut c_void),
    off13_callback: extern "C" fn(*const c_void) -> u16,
    set_off13_callback: extern "C" fn(u16, *mut c_void),
    off14_callback: extern "C" fn(*const c_void) -> u16,
    set_off14_callback: extern "C" fn(u16, *mut c_void),
    off15_callback: extern "C" fn(*const c_void) -> u16,
    set_off15_callback: extern "C" fn(u16, *mut c_void),
    off16_callback: extern "C" fn(*const c_void) -> u16,
    set_off16_callback: extern "C" fn(u16, *mut c_void),
    off17_callback: extern "C" fn(*const c_void) -> u16,
    set_off17_callback: extern "C" fn(u16, *mut c_void),
    off18_callback: extern "C" fn(*const c_void) -> u16,
    set_off18_callback: extern "C" fn(u16, *mut c_void),
    off19_callback: extern "C" fn(*const c_void) -> u16,
    set_off19_callback: extern "C" fn(u16, *mut c_void),
    off20_callback: extern "C" fn(*const c_void) -> u16,
    set_off20_callback: extern "C" fn(u16, *mut c_void),
    off21_callback: extern "C" fn(*const c_void) -> u16,
    set_off21_callback: extern "C" fn(u16, *mut c_void),
    off22_callback: extern "C" fn(*const c_void) -> u16,
    set_off22_callback: extern "C" fn(u16, *mut c_void),
    off23_callback: extern "C" fn(*const c_void) -> u16,
    set_off23_callback: extern "C" fn(u16, *mut c_void),
    off24_callback: extern "C" fn(*const c_void) -> u16,
    set_off24_callback: extern "C" fn(u16, *mut c_void),
    off25_callback: extern "C" fn(*const c_void) -> u16,
    set_off25_callback: extern "C" fn(u16, *mut c_void),
    off26_callback: extern "C" fn(*const c_void) -> u16,
    set_off26_callback: extern "C" fn(u16, *mut c_void),
    off27_callback: extern "C" fn(*const c_void) -> u16,
    set_off27_callback: extern "C" fn(u16, *mut c_void),
    off28_callback: extern "C" fn(*const c_void) -> u16,
    set_off28_callback: extern "C" fn(u16, *mut c_void),
    off29_callback: extern "C" fn(*const c_void) -> u16,
    set_off29_callback: extern "C" fn(u16, *mut c_void),
    off30_callback: extern "C" fn(*const c_void) -> u16,
    set_off30_callback: extern "C" fn(u16, *mut c_void),
    off31_callback: extern "C" fn(*const c_void) -> u16,
    set_off31_callback: extern "C" fn(u16, *mut c_void),
    off32_callback: extern "C" fn(*const c_void) -> u16,
    set_off32_callback: extern "C" fn(u16, *mut c_void),
    off33_callback: extern "C" fn(*const c_void) -> u16,
    set_off33_callback: extern "C" fn(u16, *mut c_void),
    off34_callback: extern "C" fn(*const c_void) -> u16,
    set_off34_callback: extern "C" fn(u16, *mut c_void),
    off35_callback: extern "C" fn(*const c_void) -> u16,
    set_off35_callback: extern "C" fn(u16, *mut c_void),
    off36_callback: extern "C" fn(*const c_void) -> u16,
    set_off36_callback: extern "C" fn(u16, *mut c_void),
    off37_callback: extern "C" fn(*const c_void) -> u16,
    set_off37_callback: extern "C" fn(u16, *mut c_void),
    off38_callback: extern "C" fn(*const c_void) -> u16,
    set_off38_callback: extern "C" fn(u16, *mut c_void),
    off39_callback: extern "C" fn(*const c_void) -> u16,
    set_off39_callback: extern "C" fn(u16, *mut c_void),
    off40_callback: extern "C" fn(*const c_void) -> u16,
    set_off40_callback: extern "C" fn(u16, *mut c_void),
    off41_callback: extern "C" fn(*const c_void) -> u16,
    set_off41_callback: extern "C" fn(u16, *mut c_void),
    off42_callback: extern "C" fn(*const c_void) -> u16,
    set_off42_callback: extern "C" fn(u16, *mut c_void),
    off43_callback: extern "C" fn(*const c_void) -> u16,
    set_off43_callback: extern "C" fn(u16, *mut c_void),
    off44_callback: extern "C" fn(*const c_void) -> u16,
    set_off44_callback: extern "C" fn(u16, *mut c_void),
    off45_callback: extern "C" fn(*const c_void) -> u16,
    set_off45_callback: extern "C" fn(u16, *mut c_void),
    off46_callback: extern "C" fn(*const c_void) -> u16,
    set_off46_callback: extern "C" fn(u16, *mut c_void),
    off47_callback: extern "C" fn(*const c_void) -> u16,
    set_off47_callback: extern "C" fn(u16, *mut c_void),
    off48_callback: extern "C" fn(*const c_void) -> u16,
    set_off48_callback: extern "C" fn(u16, *mut c_void),
    off49_callback: extern "C" fn(*const c_void) -> u16,
    set_off49_callback: extern "C" fn(u16, *mut c_void),
    off50_callback: extern "C" fn(*const c_void) -> u16,
    set_off50_callback: extern "C" fn(u16, *mut c_void),
    timestamp_callback: extern "C" fn(*const c_void) -> u32,
    set_timestamp_callback: extern "C" fn(u32, *mut c_void),
    milliseconds_callback: extern "C" fn(*const c_void) -> u16,
    set_milliseconds_callback: extern "C" fn(u16, *mut c_void),
    sequence_callback: extern "C" fn(*const c_void) -> u16,
    set_sequence_callback: extern "C" fn(u16, *mut c_void),
    role_callback: extern "C" fn(*const c_void) -> u16,
    set_role_callback: extern "C" fn(u16, *mut c_void),
    algorithm_callback: extern "C" fn(*const c_void) -> Alg,
    n_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model3CallbackAdapter {
    /// X
    ///
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    fn x(&self) -> u16 {
        (self.x_callback)(self.context)
    }

    /// X
    ///
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    fn set_x(&mut self, value: u16) {
        (self.set_x_callback)(value, self.context);
    }

    /// Offset1
    ///
    /// Offset of value to read
    fn offset1(&self) -> u16 {
        (self.offset1_callback)(self.context)
    }

    /// Offset1
    ///
    /// Offset of value to read
    fn set_offset1(&mut self, value: u16) {
        (self.set_offset1_callback)(value, self.context);
    }

    fn off2(&self) -> u16 {
        (self.off2_callback)(self.context)
    }

    fn set_off2(&mut self, value: u16) {
        (self.set_off2_callback)(value, self.context);
    }

    fn off3(&self) -> u16 {
        (self.off3_callback)(self.context)
    }

    fn set_off3(&mut self, value: u16) {
        (self.set_off3_callback)(value, self.context);
    }

    fn off4(&self) -> u16 {
        (self.off4_callback)(self.context)
    }

    fn set_off4(&mut self, value: u16) {
        (self.set_off4_callback)(value, self.context);
    }

    fn off5(&self) -> u16 {
        (self.off5_callback)(self.context)
    }

    fn set_off5(&mut self, value: u16) {
        (self.set_off5_callback)(value, self.context);
    }

    fn off6(&self) -> u16 {
        (self.off6_callback)(self.context)
    }

    fn set_off6(&mut self, value: u16) {
        (self.set_off6_callback)(value, self.context);
    }

    fn off7(&self) -> u16 {
        (self.off7_callback)(self.context)
    }

    fn set_off7(&mut self, value: u16) {
        (self.set_off7_callback)(value, self.context);
    }

    fn off8(&self) -> u16 {
        (self.off8_callback)(self.context)
    }

    fn set_off8(&mut self, value: u16) {
        (self.set_off8_callback)(value, self.context);
    }

    fn off9(&self) -> u16 {
        (self.off9_callback)(self.context)
    }

    fn set_off9(&mut self, value: u16) {
        (self.set_off9_callback)(value, self.context);
    }

    fn off10(&self) -> u16 {
        (self.off10_callback)(self.context)
    }

    fn set_off10(&mut self, value: u16) {
        (self.set_off10_callback)(value, self.context);
    }

    fn off11(&self) -> u16 {
        (self.off11_callback)(self.context)
    }

    fn set_off11(&mut self, value: u16) {
        (self.set_off11_callback)(value, self.context);
    }

    fn off12(&self) -> u16 {
        (self.off12_callback)(self.context)
    }

    fn set_off12(&mut self, value: u16) {
        (self.set_off12_callback)(value, self.context);
    }

    fn off13(&self) -> u16 {
        (self.off13_callback)(self.context)
    }

    fn set_off13(&mut self, value: u16) {
        (self.set_off13_callback)(value, self.context);
    }

    fn off14(&self) -> u16 {
        (self.off14_callback)(self.context)
    }

    fn set_off14(&mut self, value: u16) {
        (self.set_off14_callback)(value, self.context);
    }

    fn off15(&self) -> u16 {
        (self.off15_callback)(self.context)
    }

    fn set_off15(&mut self, value: u16) {
        (self.set_off15_callback)(value, self.context);
    }

    fn off16(&self) -> u16 {
        (self.off16_callback)(self.context)
    }

    fn set_off16(&mut self, value: u16) {
        (self.set_off16_callback)(value, self.context);
    }

    fn off17(&self) -> u16 {
        (self.off17_callback)(self.context)
    }

    fn set_off17(&mut self, value: u16) {
        (self.set_off17_callback)(value, self.context);
    }

    fn off18(&self) -> u16 {
        (self.off18_callback)(self.context)
    }

    fn set_off18(&mut self, value: u16) {
        (self.set_off18_callback)(value, self.context);
    }

    fn off19(&self) -> u16 {
        (self.off19_callback)(self.context)
    }

    fn set_off19(&mut self, value: u16) {
        (self.set_off19_callback)(value, self.context);
    }

    fn off20(&self) -> u16 {
        (self.off20_callback)(self.context)
    }

    fn set_off20(&mut self, value: u16) {
        (self.set_off20_callback)(value, self.context);
    }

    fn off21(&self) -> u16 {
        (self.off21_callback)(self.context)
    }

    fn set_off21(&mut self, value: u16) {
        (self.set_off21_callback)(value, self.context);
    }

    fn off22(&self) -> u16 {
        (self.off22_callback)(self.context)
    }

    fn set_off22(&mut self, value: u16) {
        (self.set_off22_callback)(value, self.context);
    }

    fn off23(&self) -> u16 {
        (self.off23_callback)(self.context)
    }

    fn set_off23(&mut self, value: u16) {
        (self.set_off23_callback)(value, self.context);
    }

    fn off24(&self) -> u16 {
        (self.off24_callback)(self.context)
    }

    fn set_off24(&mut self, value: u16) {
        (self.set_off24_callback)(value, self.context);
    }

    fn off25(&self) -> u16 {
        (self.off25_callback)(self.context)
    }

    fn set_off25(&mut self, value: u16) {
        (self.set_off25_callback)(value, self.context);
    }

    fn off26(&self) -> u16 {
        (self.off26_callback)(self.context)
    }

    fn set_off26(&mut self, value: u16) {
        (self.set_off26_callback)(value, self.context);
    }

    fn off27(&self) -> u16 {
        (self.off27_callback)(self.context)
    }

    fn set_off27(&mut self, value: u16) {
        (self.set_off27_callback)(value, self.context);
    }

    fn off28(&self) -> u16 {
        (self.off28_callback)(self.context)
    }

    fn set_off28(&mut self, value: u16) {
        (self.set_off28_callback)(value, self.context);
    }

    fn off29(&self) -> u16 {
        (self.off29_callback)(self.context)
    }

    fn set_off29(&mut self, value: u16) {
        (self.set_off29_callback)(value, self.context);
    }

    fn off30(&self) -> u16 {
        (self.off30_callback)(self.context)
    }

    fn set_off30(&mut self, value: u16) {
        (self.set_off30_callback)(value, self.context);
    }

    fn off31(&self) -> u16 {
        (self.off31_callback)(self.context)
    }

    fn set_off31(&mut self, value: u16) {
        (self.set_off31_callback)(value, self.context);
    }

    fn off32(&self) -> u16 {
        (self.off32_callback)(self.context)
    }

    fn set_off32(&mut self, value: u16) {
        (self.set_off32_callback)(value, self.context);
    }

    fn off33(&self) -> u16 {
        (self.off33_callback)(self.context)
    }

    fn set_off33(&mut self, value: u16) {
        (self.set_off33_callback)(value, self.context);
    }

    fn off34(&self) -> u16 {
        (self.off34_callback)(self.context)
    }

    fn set_off34(&mut self, value: u16) {
        (self.set_off34_callback)(value, self.context);
    }

    fn off35(&self) -> u16 {
        (self.off35_callback)(self.context)
    }

    fn set_off35(&mut self, value: u16) {
        (self.set_off35_callback)(value, self.context);
    }

    fn off36(&self) -> u16 {
        (self.off36_callback)(self.context)
    }

    fn set_off36(&mut self, value: u16) {
        (self.set_off36_callback)(value, self.context);
    }

    fn off37(&self) -> u16 {
        (self.off37_callback)(self.context)
    }

    fn set_off37(&mut self, value: u16) {
        (self.set_off37_callback)(value, self.context);
    }

    fn off38(&self) -> u16 {
        (self.off38_callback)(self.context)
    }

    fn set_off38(&mut self, value: u16) {
        (self.set_off38_callback)(value, self.context);
    }

    fn off39(&self) -> u16 {
        (self.off39_callback)(self.context)
    }

    fn set_off39(&mut self, value: u16) {
        (self.set_off39_callback)(value, self.context);
    }

    fn off40(&self) -> u16 {
        (self.off40_callback)(self.context)
    }

    fn set_off40(&mut self, value: u16) {
        (self.set_off40_callback)(value, self.context);
    }

    fn off41(&self) -> u16 {
        (self.off41_callback)(self.context)
    }

    fn set_off41(&mut self, value: u16) {
        (self.set_off41_callback)(value, self.context);
    }

    fn off42(&self) -> u16 {
        (self.off42_callback)(self.context)
    }

    fn set_off42(&mut self, value: u16) {
        (self.set_off42_callback)(value, self.context);
    }

    fn off43(&self) -> u16 {
        (self.off43_callback)(self.context)
    }

    fn set_off43(&mut self, value: u16) {
        (self.set_off43_callback)(value, self.context);
    }

    fn off44(&self) -> u16 {
        (self.off44_callback)(self.context)
    }

    fn set_off44(&mut self, value: u16) {
        (self.set_off44_callback)(value, self.context);
    }

    fn off45(&self) -> u16 {
        (self.off45_callback)(self.context)
    }

    fn set_off45(&mut self, value: u16) {
        (self.set_off45_callback)(value, self.context);
    }

    fn off46(&self) -> u16 {
        (self.off46_callback)(self.context)
    }

    fn set_off46(&mut self, value: u16) {
        (self.set_off46_callback)(value, self.context);
    }

    fn off47(&self) -> u16 {
        (self.off47_callback)(self.context)
    }

    fn set_off47(&mut self, value: u16) {
        (self.set_off47_callback)(value, self.context);
    }

    fn off48(&self) -> u16 {
        (self.off48_callback)(self.context)
    }

    fn set_off48(&mut self, value: u16) {
        (self.set_off48_callback)(value, self.context);
    }

    fn off49(&self) -> u16 {
        (self.off49_callback)(self.context)
    }

    fn set_off49(&mut self, value: u16) {
        (self.set_off49_callback)(value, self.context);
    }

    fn off50(&self) -> u16 {
        (self.off50_callback)(self.context)
    }

    fn set_off50(&mut self, value: u16) {
        (self.set_off50_callback)(value, self.context);
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
    /// Digital Signature ID
    ///
    /// User's role id 0-5
    fn role(&self) -> u16 {
        (self.role_callback)(self.context)
    }

    /// Role
    ///
    /// Digital Signature ID
    ///
    /// User's role id 0-5
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

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        (self.n_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model3StatefulAdapter {
    x: u16,
    offset1: u16,
    off2: u16,
    off3: u16,
    off4: u16,
    off5: u16,
    off6: u16,
    off7: u16,
    off8: u16,
    off9: u16,
    off10: u16,
    off11: u16,
    off12: u16,
    off13: u16,
    off14: u16,
    off15: u16,
    off16: u16,
    off17: u16,
    off18: u16,
    off19: u16,
    off20: u16,
    off21: u16,
    off22: u16,
    off23: u16,
    off24: u16,
    off25: u16,
    off26: u16,
    off27: u16,
    off28: u16,
    off29: u16,
    off30: u16,
    off31: u16,
    off32: u16,
    off33: u16,
    off34: u16,
    off35: u16,
    off36: u16,
    off37: u16,
    off38: u16,
    off39: u16,
    off40: u16,
    off41: u16,
    off42: u16,
    off43: u16,
    off44: u16,
    off45: u16,
    off46: u16,
    off47: u16,
    off48: u16,
    off49: u16,
    off50: u16,
    timestamp: u32,
    milliseconds: u16,
    sequence: u16,
    role: u16,
    algorithm: Alg,
    n: u16,
}

impl ModelAdapter for Model3StatefulAdapter {
    /// X
    ///
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    fn x(&self) -> u16 {
        self.x
    }

    /// X
    ///
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    fn set_x(&mut self, value: u16) {
        self.x = value;
    }

    /// Offset1
    ///
    /// Offset of value to read
    fn offset1(&self) -> u16 {
        self.offset1
    }

    /// Offset1
    ///
    /// Offset of value to read
    fn set_offset1(&mut self, value: u16) {
        self.offset1 = value;
    }

    fn off2(&self) -> u16 {
        self.off2
    }

    fn set_off2(&mut self, value: u16) {
        self.off2 = value;
    }

    fn off3(&self) -> u16 {
        self.off3
    }

    fn set_off3(&mut self, value: u16) {
        self.off3 = value;
    }

    fn off4(&self) -> u16 {
        self.off4
    }

    fn set_off4(&mut self, value: u16) {
        self.off4 = value;
    }

    fn off5(&self) -> u16 {
        self.off5
    }

    fn set_off5(&mut self, value: u16) {
        self.off5 = value;
    }

    fn off6(&self) -> u16 {
        self.off6
    }

    fn set_off6(&mut self, value: u16) {
        self.off6 = value;
    }

    fn off7(&self) -> u16 {
        self.off7
    }

    fn set_off7(&mut self, value: u16) {
        self.off7 = value;
    }

    fn off8(&self) -> u16 {
        self.off8
    }

    fn set_off8(&mut self, value: u16) {
        self.off8 = value;
    }

    fn off9(&self) -> u16 {
        self.off9
    }

    fn set_off9(&mut self, value: u16) {
        self.off9 = value;
    }

    fn off10(&self) -> u16 {
        self.off10
    }

    fn set_off10(&mut self, value: u16) {
        self.off10 = value;
    }

    fn off11(&self) -> u16 {
        self.off11
    }

    fn set_off11(&mut self, value: u16) {
        self.off11 = value;
    }

    fn off12(&self) -> u16 {
        self.off12
    }

    fn set_off12(&mut self, value: u16) {
        self.off12 = value;
    }

    fn off13(&self) -> u16 {
        self.off13
    }

    fn set_off13(&mut self, value: u16) {
        self.off13 = value;
    }

    fn off14(&self) -> u16 {
        self.off14
    }

    fn set_off14(&mut self, value: u16) {
        self.off14 = value;
    }

    fn off15(&self) -> u16 {
        self.off15
    }

    fn set_off15(&mut self, value: u16) {
        self.off15 = value;
    }

    fn off16(&self) -> u16 {
        self.off16
    }

    fn set_off16(&mut self, value: u16) {
        self.off16 = value;
    }

    fn off17(&self) -> u16 {
        self.off17
    }

    fn set_off17(&mut self, value: u16) {
        self.off17 = value;
    }

    fn off18(&self) -> u16 {
        self.off18
    }

    fn set_off18(&mut self, value: u16) {
        self.off18 = value;
    }

    fn off19(&self) -> u16 {
        self.off19
    }

    fn set_off19(&mut self, value: u16) {
        self.off19 = value;
    }

    fn off20(&self) -> u16 {
        self.off20
    }

    fn set_off20(&mut self, value: u16) {
        self.off20 = value;
    }

    fn off21(&self) -> u16 {
        self.off21
    }

    fn set_off21(&mut self, value: u16) {
        self.off21 = value;
    }

    fn off22(&self) -> u16 {
        self.off22
    }

    fn set_off22(&mut self, value: u16) {
        self.off22 = value;
    }

    fn off23(&self) -> u16 {
        self.off23
    }

    fn set_off23(&mut self, value: u16) {
        self.off23 = value;
    }

    fn off24(&self) -> u16 {
        self.off24
    }

    fn set_off24(&mut self, value: u16) {
        self.off24 = value;
    }

    fn off25(&self) -> u16 {
        self.off25
    }

    fn set_off25(&mut self, value: u16) {
        self.off25 = value;
    }

    fn off26(&self) -> u16 {
        self.off26
    }

    fn set_off26(&mut self, value: u16) {
        self.off26 = value;
    }

    fn off27(&self) -> u16 {
        self.off27
    }

    fn set_off27(&mut self, value: u16) {
        self.off27 = value;
    }

    fn off28(&self) -> u16 {
        self.off28
    }

    fn set_off28(&mut self, value: u16) {
        self.off28 = value;
    }

    fn off29(&self) -> u16 {
        self.off29
    }

    fn set_off29(&mut self, value: u16) {
        self.off29 = value;
    }

    fn off30(&self) -> u16 {
        self.off30
    }

    fn set_off30(&mut self, value: u16) {
        self.off30 = value;
    }

    fn off31(&self) -> u16 {
        self.off31
    }

    fn set_off31(&mut self, value: u16) {
        self.off31 = value;
    }

    fn off32(&self) -> u16 {
        self.off32
    }

    fn set_off32(&mut self, value: u16) {
        self.off32 = value;
    }

    fn off33(&self) -> u16 {
        self.off33
    }

    fn set_off33(&mut self, value: u16) {
        self.off33 = value;
    }

    fn off34(&self) -> u16 {
        self.off34
    }

    fn set_off34(&mut self, value: u16) {
        self.off34 = value;
    }

    fn off35(&self) -> u16 {
        self.off35
    }

    fn set_off35(&mut self, value: u16) {
        self.off35 = value;
    }

    fn off36(&self) -> u16 {
        self.off36
    }

    fn set_off36(&mut self, value: u16) {
        self.off36 = value;
    }

    fn off37(&self) -> u16 {
        self.off37
    }

    fn set_off37(&mut self, value: u16) {
        self.off37 = value;
    }

    fn off38(&self) -> u16 {
        self.off38
    }

    fn set_off38(&mut self, value: u16) {
        self.off38 = value;
    }

    fn off39(&self) -> u16 {
        self.off39
    }

    fn set_off39(&mut self, value: u16) {
        self.off39 = value;
    }

    fn off40(&self) -> u16 {
        self.off40
    }

    fn set_off40(&mut self, value: u16) {
        self.off40 = value;
    }

    fn off41(&self) -> u16 {
        self.off41
    }

    fn set_off41(&mut self, value: u16) {
        self.off41 = value;
    }

    fn off42(&self) -> u16 {
        self.off42
    }

    fn set_off42(&mut self, value: u16) {
        self.off42 = value;
    }

    fn off43(&self) -> u16 {
        self.off43
    }

    fn set_off43(&mut self, value: u16) {
        self.off43 = value;
    }

    fn off44(&self) -> u16 {
        self.off44
    }

    fn set_off44(&mut self, value: u16) {
        self.off44 = value;
    }

    fn off45(&self) -> u16 {
        self.off45
    }

    fn set_off45(&mut self, value: u16) {
        self.off45 = value;
    }

    fn off46(&self) -> u16 {
        self.off46
    }

    fn set_off46(&mut self, value: u16) {
        self.off46 = value;
    }

    fn off47(&self) -> u16 {
        self.off47
    }

    fn set_off47(&mut self, value: u16) {
        self.off47 = value;
    }

    fn off48(&self) -> u16 {
        self.off48
    }

    fn set_off48(&mut self, value: u16) {
        self.off48 = value;
    }

    fn off49(&self) -> u16 {
        self.off49
    }

    fn set_off49(&mut self, value: u16) {
        self.off49 = value;
    }

    fn off50(&self) -> u16 {
        self.off50
    }

    fn set_off50(&mut self, value: u16) {
        self.off50 = value;
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
    /// Digital Signature ID
    ///
    /// User's role id 0-5
    fn role(&self) -> u16 {
        self.role
    }

    /// Role
    ///
    /// Digital Signature ID
    ///
    /// User's role id 0-5
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

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        self.n
    }
}