use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 63;

pub static POINTS: [ReadablePoint; 62] = [
    ReadablePoint {
        reference: PointReference::Static { value: 4 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::RequestSequence,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Status,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::X },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Value1,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::Val2 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::Val3 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::Val4 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::Val5 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::Val6 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::Val7 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::Val8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::Val9 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val10,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val11,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val12,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val13,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val14,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val15,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val16,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val17,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val18,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val19,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val20,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val21,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val22,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val23,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val24,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val25,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val26,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val27,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val28,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val29,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val30,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val31,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val32,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val33,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val34,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val35,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val36,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val37,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val38,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val39,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val40,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val41,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val42,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val43,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val44,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val45,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val46,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val47,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val48,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val49,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Val50,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Timestamp,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Milliseconds,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Sequence,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Alarm,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::Algorithm,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 { point: Point::N },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model4 {
            point: Point::RepeatingDs,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    RequestSequence,
    Status,
    X,
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
    Timestamp,
    Milliseconds,
    Sequence,
    Alarm,
    Algorithm,
    N,
    RepeatingDs,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    63
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
        Point::RequestSequence => {
            buffer::write_u16(model.request_sequence(), buffer);
        }
        Point::Status => {
            buffer::write_u16(model.status() as u16, buffer);
        }
        Point::X => {
            buffer::write_u16(model.x(), buffer);
        }
        Point::Value1 => {
            buffer::write_u16(model.value1(), buffer);
        }
        Point::Val2 => {
            buffer::write_u16(model.val2(), buffer);
        }
        Point::Val3 => {
            buffer::write_u16(model.val3(), buffer);
        }
        Point::Val4 => {
            buffer::write_u16(model.val4(), buffer);
        }
        Point::Val5 => {
            buffer::write_u16(model.val5(), buffer);
        }
        Point::Val6 => {
            buffer::write_u16(model.val6(), buffer);
        }
        Point::Val7 => {
            buffer::write_u16(model.val7(), buffer);
        }
        Point::Val8 => {
            buffer::write_u16(model.val8(), buffer);
        }
        Point::Val9 => {
            buffer::write_u16(model.val9(), buffer);
        }
        Point::Val10 => {
            buffer::write_u16(model.val10(), buffer);
        }
        Point::Val11 => {
            buffer::write_u16(model.val11(), buffer);
        }
        Point::Val12 => {
            buffer::write_u16(model.val12(), buffer);
        }
        Point::Val13 => {
            buffer::write_u16(model.val13(), buffer);
        }
        Point::Val14 => {
            buffer::write_u16(model.val14(), buffer);
        }
        Point::Val15 => {
            buffer::write_u16(model.val15(), buffer);
        }
        Point::Val16 => {
            buffer::write_u16(model.val16(), buffer);
        }
        Point::Val17 => {
            buffer::write_u16(model.val17(), buffer);
        }
        Point::Val18 => {
            buffer::write_u16(model.val18(), buffer);
        }
        Point::Val19 => {
            buffer::write_u16(model.val19(), buffer);
        }
        Point::Val20 => {
            buffer::write_u16(model.val20(), buffer);
        }
        Point::Val21 => {
            buffer::write_u16(model.val21(), buffer);
        }
        Point::Val22 => {
            buffer::write_u16(model.val22(), buffer);
        }
        Point::Val23 => {
            buffer::write_u16(model.val23(), buffer);
        }
        Point::Val24 => {
            buffer::write_u16(model.val24(), buffer);
        }
        Point::Val25 => {
            buffer::write_u16(model.val25(), buffer);
        }
        Point::Val26 => {
            buffer::write_u16(model.val26(), buffer);
        }
        Point::Val27 => {
            buffer::write_u16(model.val27(), buffer);
        }
        Point::Val28 => {
            buffer::write_u16(model.val28(), buffer);
        }
        Point::Val29 => {
            buffer::write_u16(model.val29(), buffer);
        }
        Point::Val30 => {
            buffer::write_u16(model.val30(), buffer);
        }
        Point::Val31 => {
            buffer::write_u16(model.val31(), buffer);
        }
        Point::Val32 => {
            buffer::write_u16(model.val32(), buffer);
        }
        Point::Val33 => {
            buffer::write_u16(model.val33(), buffer);
        }
        Point::Val34 => {
            buffer::write_u16(model.val34(), buffer);
        }
        Point::Val35 => {
            buffer::write_u16(model.val35(), buffer);
        }
        Point::Val36 => {
            buffer::write_u16(model.val36(), buffer);
        }
        Point::Val37 => {
            buffer::write_u16(model.val37(), buffer);
        }
        Point::Val38 => {
            buffer::write_u16(model.val38(), buffer);
        }
        Point::Val39 => {
            buffer::write_u16(model.val39(), buffer);
        }
        Point::Val40 => {
            buffer::write_u16(model.val40(), buffer);
        }
        Point::Val41 => {
            buffer::write_u16(model.val41(), buffer);
        }
        Point::Val42 => {
            buffer::write_u16(model.val42(), buffer);
        }
        Point::Val43 => {
            buffer::write_u16(model.val43(), buffer);
        }
        Point::Val44 => {
            buffer::write_u16(model.val44(), buffer);
        }
        Point::Val45 => {
            buffer::write_u16(model.val45(), buffer);
        }
        Point::Val46 => {
            buffer::write_u16(model.val46(), buffer);
        }
        Point::Val47 => {
            buffer::write_u16(model.val47(), buffer);
        }
        Point::Val48 => {
            buffer::write_u16(model.val48(), buffer);
        }
        Point::Val49 => {
            buffer::write_u16(model.val49(), buffer);
        }
        Point::Val50 => {
            buffer::write_u16(model.val50(), buffer);
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
        Point::Alarm => {
            buffer::write_u16(model.alarm() as u16, buffer);
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
    /// Request Sequence
    ///
    /// Sequence number from the request
    fn request_sequence(&self) -> u16;

    /// Status
    ///
    /// Status of last read operation
    fn status(&self) -> Sts;

    /// X
    ///
    /// Number of values from the request
    ///
    /// A max of 50 values are allocated
    fn x(&self) -> u16;

    /// Value1
    ///
    /// Copy of value from register Off1.
    ///
    /// Unused values shall return 0xFFFF (unimplemented)
    fn value1(&self) -> u16;

    fn val2(&self) -> u16;

    fn val3(&self) -> u16;

    fn val4(&self) -> u16;

    fn val5(&self) -> u16;

    fn val6(&self) -> u16;

    fn val7(&self) -> u16;

    fn val8(&self) -> u16;

    fn val9(&self) -> u16;

    fn val10(&self) -> u16;

    fn val11(&self) -> u16;

    fn val12(&self) -> u16;

    fn val13(&self) -> u16;

    fn val14(&self) -> u16;

    fn val15(&self) -> u16;

    fn val16(&self) -> u16;

    fn val17(&self) -> u16;

    fn val18(&self) -> u16;

    fn val19(&self) -> u16;

    fn val20(&self) -> u16;

    fn val21(&self) -> u16;

    fn val22(&self) -> u16;

    fn val23(&self) -> u16;

    fn val24(&self) -> u16;

    fn val25(&self) -> u16;

    fn val26(&self) -> u16;

    fn val27(&self) -> u16;

    fn val28(&self) -> u16;

    fn val29(&self) -> u16;

    fn val30(&self) -> u16;

    fn val31(&self) -> u16;

    fn val32(&self) -> u16;

    fn val33(&self) -> u16;

    fn val34(&self) -> u16;

    fn val35(&self) -> u16;

    fn val36(&self) -> u16;

    fn val37(&self) -> u16;

    fn val38(&self) -> u16;

    fn val39(&self) -> u16;

    fn val40(&self) -> u16;

    fn val41(&self) -> u16;

    fn val42(&self) -> u16;

    fn val43(&self) -> u16;

    fn val44(&self) -> u16;

    fn val45(&self) -> u16;

    fn val46(&self) -> u16;

    fn val47(&self) -> u16;

    fn val48(&self) -> u16;

    fn val49(&self) -> u16;

    fn val50(&self) -> u16;

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32;

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16;

    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Shall be advanced for each response
    fn sequence(&self) -> u16;

    /// Alarm
    ///
    /// Bitmask alarm code
    fn alarm(&self) -> Alm;

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

    /// DS
    ///
    /// Digital Signature
    fn repeating_ds(&self) -> u16;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Alm {
    None = 0,
    /// Tampered
    Alm = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Sts {
    Success = 0,
    Ds = 1,
    /// One or more registers were not writable by this role
    Acl = 2,
    /// Offset out of range or missing from multi-register value
    Off = 3,
}

#[repr(C)]
pub struct Model4CallbackAdapter {
    context: *mut c_void,
    request_sequence_callback: extern "C" fn(*const c_void) -> u16,
    status_callback: extern "C" fn(*const c_void) -> Sts,
    x_callback: extern "C" fn(*const c_void) -> u16,
    value1_callback: extern "C" fn(*const c_void) -> u16,
    val2_callback: extern "C" fn(*const c_void) -> u16,
    val3_callback: extern "C" fn(*const c_void) -> u16,
    val4_callback: extern "C" fn(*const c_void) -> u16,
    val5_callback: extern "C" fn(*const c_void) -> u16,
    val6_callback: extern "C" fn(*const c_void) -> u16,
    val7_callback: extern "C" fn(*const c_void) -> u16,
    val8_callback: extern "C" fn(*const c_void) -> u16,
    val9_callback: extern "C" fn(*const c_void) -> u16,
    val10_callback: extern "C" fn(*const c_void) -> u16,
    val11_callback: extern "C" fn(*const c_void) -> u16,
    val12_callback: extern "C" fn(*const c_void) -> u16,
    val13_callback: extern "C" fn(*const c_void) -> u16,
    val14_callback: extern "C" fn(*const c_void) -> u16,
    val15_callback: extern "C" fn(*const c_void) -> u16,
    val16_callback: extern "C" fn(*const c_void) -> u16,
    val17_callback: extern "C" fn(*const c_void) -> u16,
    val18_callback: extern "C" fn(*const c_void) -> u16,
    val19_callback: extern "C" fn(*const c_void) -> u16,
    val20_callback: extern "C" fn(*const c_void) -> u16,
    val21_callback: extern "C" fn(*const c_void) -> u16,
    val22_callback: extern "C" fn(*const c_void) -> u16,
    val23_callback: extern "C" fn(*const c_void) -> u16,
    val24_callback: extern "C" fn(*const c_void) -> u16,
    val25_callback: extern "C" fn(*const c_void) -> u16,
    val26_callback: extern "C" fn(*const c_void) -> u16,
    val27_callback: extern "C" fn(*const c_void) -> u16,
    val28_callback: extern "C" fn(*const c_void) -> u16,
    val29_callback: extern "C" fn(*const c_void) -> u16,
    val30_callback: extern "C" fn(*const c_void) -> u16,
    val31_callback: extern "C" fn(*const c_void) -> u16,
    val32_callback: extern "C" fn(*const c_void) -> u16,
    val33_callback: extern "C" fn(*const c_void) -> u16,
    val34_callback: extern "C" fn(*const c_void) -> u16,
    val35_callback: extern "C" fn(*const c_void) -> u16,
    val36_callback: extern "C" fn(*const c_void) -> u16,
    val37_callback: extern "C" fn(*const c_void) -> u16,
    val38_callback: extern "C" fn(*const c_void) -> u16,
    val39_callback: extern "C" fn(*const c_void) -> u16,
    val40_callback: extern "C" fn(*const c_void) -> u16,
    val41_callback: extern "C" fn(*const c_void) -> u16,
    val42_callback: extern "C" fn(*const c_void) -> u16,
    val43_callback: extern "C" fn(*const c_void) -> u16,
    val44_callback: extern "C" fn(*const c_void) -> u16,
    val45_callback: extern "C" fn(*const c_void) -> u16,
    val46_callback: extern "C" fn(*const c_void) -> u16,
    val47_callback: extern "C" fn(*const c_void) -> u16,
    val48_callback: extern "C" fn(*const c_void) -> u16,
    val49_callback: extern "C" fn(*const c_void) -> u16,
    val50_callback: extern "C" fn(*const c_void) -> u16,
    timestamp_callback: extern "C" fn(*const c_void) -> u32,
    milliseconds_callback: extern "C" fn(*const c_void) -> u16,
    sequence_callback: extern "C" fn(*const c_void) -> u16,
    alarm_callback: extern "C" fn(*const c_void) -> Alm,
    algorithm_callback: extern "C" fn(*const c_void) -> Alg,
    n_callback: extern "C" fn(*const c_void) -> u16,
    repeating_ds_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model4CallbackAdapter {
    /// Request Sequence
    ///
    /// Sequence number from the request
    fn request_sequence(&self) -> u16 {
        (self.request_sequence_callback)(self.context)
    }

    /// Status
    ///
    /// Status of last read operation
    fn status(&self) -> Sts {
        (self.status_callback)(self.context)
    }

    /// X
    ///
    /// Number of values from the request
    ///
    /// A max of 50 values are allocated
    fn x(&self) -> u16 {
        (self.x_callback)(self.context)
    }

    /// Value1
    ///
    /// Copy of value from register Off1.
    ///
    /// Unused values shall return 0xFFFF (unimplemented)
    fn value1(&self) -> u16 {
        (self.value1_callback)(self.context)
    }

    fn val2(&self) -> u16 {
        (self.val2_callback)(self.context)
    }

    fn val3(&self) -> u16 {
        (self.val3_callback)(self.context)
    }

    fn val4(&self) -> u16 {
        (self.val4_callback)(self.context)
    }

    fn val5(&self) -> u16 {
        (self.val5_callback)(self.context)
    }

    fn val6(&self) -> u16 {
        (self.val6_callback)(self.context)
    }

    fn val7(&self) -> u16 {
        (self.val7_callback)(self.context)
    }

    fn val8(&self) -> u16 {
        (self.val8_callback)(self.context)
    }

    fn val9(&self) -> u16 {
        (self.val9_callback)(self.context)
    }

    fn val10(&self) -> u16 {
        (self.val10_callback)(self.context)
    }

    fn val11(&self) -> u16 {
        (self.val11_callback)(self.context)
    }

    fn val12(&self) -> u16 {
        (self.val12_callback)(self.context)
    }

    fn val13(&self) -> u16 {
        (self.val13_callback)(self.context)
    }

    fn val14(&self) -> u16 {
        (self.val14_callback)(self.context)
    }

    fn val15(&self) -> u16 {
        (self.val15_callback)(self.context)
    }

    fn val16(&self) -> u16 {
        (self.val16_callback)(self.context)
    }

    fn val17(&self) -> u16 {
        (self.val17_callback)(self.context)
    }

    fn val18(&self) -> u16 {
        (self.val18_callback)(self.context)
    }

    fn val19(&self) -> u16 {
        (self.val19_callback)(self.context)
    }

    fn val20(&self) -> u16 {
        (self.val20_callback)(self.context)
    }

    fn val21(&self) -> u16 {
        (self.val21_callback)(self.context)
    }

    fn val22(&self) -> u16 {
        (self.val22_callback)(self.context)
    }

    fn val23(&self) -> u16 {
        (self.val23_callback)(self.context)
    }

    fn val24(&self) -> u16 {
        (self.val24_callback)(self.context)
    }

    fn val25(&self) -> u16 {
        (self.val25_callback)(self.context)
    }

    fn val26(&self) -> u16 {
        (self.val26_callback)(self.context)
    }

    fn val27(&self) -> u16 {
        (self.val27_callback)(self.context)
    }

    fn val28(&self) -> u16 {
        (self.val28_callback)(self.context)
    }

    fn val29(&self) -> u16 {
        (self.val29_callback)(self.context)
    }

    fn val30(&self) -> u16 {
        (self.val30_callback)(self.context)
    }

    fn val31(&self) -> u16 {
        (self.val31_callback)(self.context)
    }

    fn val32(&self) -> u16 {
        (self.val32_callback)(self.context)
    }

    fn val33(&self) -> u16 {
        (self.val33_callback)(self.context)
    }

    fn val34(&self) -> u16 {
        (self.val34_callback)(self.context)
    }

    fn val35(&self) -> u16 {
        (self.val35_callback)(self.context)
    }

    fn val36(&self) -> u16 {
        (self.val36_callback)(self.context)
    }

    fn val37(&self) -> u16 {
        (self.val37_callback)(self.context)
    }

    fn val38(&self) -> u16 {
        (self.val38_callback)(self.context)
    }

    fn val39(&self) -> u16 {
        (self.val39_callback)(self.context)
    }

    fn val40(&self) -> u16 {
        (self.val40_callback)(self.context)
    }

    fn val41(&self) -> u16 {
        (self.val41_callback)(self.context)
    }

    fn val42(&self) -> u16 {
        (self.val42_callback)(self.context)
    }

    fn val43(&self) -> u16 {
        (self.val43_callback)(self.context)
    }

    fn val44(&self) -> u16 {
        (self.val44_callback)(self.context)
    }

    fn val45(&self) -> u16 {
        (self.val45_callback)(self.context)
    }

    fn val46(&self) -> u16 {
        (self.val46_callback)(self.context)
    }

    fn val47(&self) -> u16 {
        (self.val47_callback)(self.context)
    }

    fn val48(&self) -> u16 {
        (self.val48_callback)(self.context)
    }

    fn val49(&self) -> u16 {
        (self.val49_callback)(self.context)
    }

    fn val50(&self) -> u16 {
        (self.val50_callback)(self.context)
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        (self.timestamp_callback)(self.context)
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        (self.milliseconds_callback)(self.context)
    }

    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Shall be advanced for each response
    fn sequence(&self) -> u16 {
        (self.sequence_callback)(self.context)
    }

    /// Alarm
    ///
    /// Bitmask alarm code
    fn alarm(&self) -> Alm {
        (self.alarm_callback)(self.context)
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

    /// DS
    ///
    /// Digital Signature
    fn repeating_ds(&self) -> u16 {
        (self.repeating_ds_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model4StatefulAdapter {
    request_sequence: u16,
    status: Sts,
    x: u16,
    value1: u16,
    val2: u16,
    val3: u16,
    val4: u16,
    val5: u16,
    val6: u16,
    val7: u16,
    val8: u16,
    val9: u16,
    val10: u16,
    val11: u16,
    val12: u16,
    val13: u16,
    val14: u16,
    val15: u16,
    val16: u16,
    val17: u16,
    val18: u16,
    val19: u16,
    val20: u16,
    val21: u16,
    val22: u16,
    val23: u16,
    val24: u16,
    val25: u16,
    val26: u16,
    val27: u16,
    val28: u16,
    val29: u16,
    val30: u16,
    val31: u16,
    val32: u16,
    val33: u16,
    val34: u16,
    val35: u16,
    val36: u16,
    val37: u16,
    val38: u16,
    val39: u16,
    val40: u16,
    val41: u16,
    val42: u16,
    val43: u16,
    val44: u16,
    val45: u16,
    val46: u16,
    val47: u16,
    val48: u16,
    val49: u16,
    val50: u16,
    timestamp: u32,
    milliseconds: u16,
    sequence: u16,
    alarm: Alm,
    algorithm: Alg,
    n: u16,
    repeating_ds: u16,
}

impl ModelAdapter for Model4StatefulAdapter {
    /// Request Sequence
    ///
    /// Sequence number from the request
    fn request_sequence(&self) -> u16 {
        self.request_sequence
    }

    /// Status
    ///
    /// Status of last read operation
    fn status(&self) -> Sts {
        self.status
    }

    /// X
    ///
    /// Number of values from the request
    ///
    /// A max of 50 values are allocated
    fn x(&self) -> u16 {
        self.x
    }

    /// Value1
    ///
    /// Copy of value from register Off1.
    ///
    /// Unused values shall return 0xFFFF (unimplemented)
    fn value1(&self) -> u16 {
        self.value1
    }

    fn val2(&self) -> u16 {
        self.val2
    }

    fn val3(&self) -> u16 {
        self.val3
    }

    fn val4(&self) -> u16 {
        self.val4
    }

    fn val5(&self) -> u16 {
        self.val5
    }

    fn val6(&self) -> u16 {
        self.val6
    }

    fn val7(&self) -> u16 {
        self.val7
    }

    fn val8(&self) -> u16 {
        self.val8
    }

    fn val9(&self) -> u16 {
        self.val9
    }

    fn val10(&self) -> u16 {
        self.val10
    }

    fn val11(&self) -> u16 {
        self.val11
    }

    fn val12(&self) -> u16 {
        self.val12
    }

    fn val13(&self) -> u16 {
        self.val13
    }

    fn val14(&self) -> u16 {
        self.val14
    }

    fn val15(&self) -> u16 {
        self.val15
    }

    fn val16(&self) -> u16 {
        self.val16
    }

    fn val17(&self) -> u16 {
        self.val17
    }

    fn val18(&self) -> u16 {
        self.val18
    }

    fn val19(&self) -> u16 {
        self.val19
    }

    fn val20(&self) -> u16 {
        self.val20
    }

    fn val21(&self) -> u16 {
        self.val21
    }

    fn val22(&self) -> u16 {
        self.val22
    }

    fn val23(&self) -> u16 {
        self.val23
    }

    fn val24(&self) -> u16 {
        self.val24
    }

    fn val25(&self) -> u16 {
        self.val25
    }

    fn val26(&self) -> u16 {
        self.val26
    }

    fn val27(&self) -> u16 {
        self.val27
    }

    fn val28(&self) -> u16 {
        self.val28
    }

    fn val29(&self) -> u16 {
        self.val29
    }

    fn val30(&self) -> u16 {
        self.val30
    }

    fn val31(&self) -> u16 {
        self.val31
    }

    fn val32(&self) -> u16 {
        self.val32
    }

    fn val33(&self) -> u16 {
        self.val33
    }

    fn val34(&self) -> u16 {
        self.val34
    }

    fn val35(&self) -> u16 {
        self.val35
    }

    fn val36(&self) -> u16 {
        self.val36
    }

    fn val37(&self) -> u16 {
        self.val37
    }

    fn val38(&self) -> u16 {
        self.val38
    }

    fn val39(&self) -> u16 {
        self.val39
    }

    fn val40(&self) -> u16 {
        self.val40
    }

    fn val41(&self) -> u16 {
        self.val41
    }

    fn val42(&self) -> u16 {
        self.val42
    }

    fn val43(&self) -> u16 {
        self.val43
    }

    fn val44(&self) -> u16 {
        self.val44
    }

    fn val45(&self) -> u16 {
        self.val45
    }

    fn val46(&self) -> u16 {
        self.val46
    }

    fn val47(&self) -> u16 {
        self.val47
    }

    fn val48(&self) -> u16 {
        self.val48
    }

    fn val49(&self) -> u16 {
        self.val49
    }

    fn val50(&self) -> u16 {
        self.val50
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Shall be advanced for each response
    fn sequence(&self) -> u16 {
        self.sequence
    }

    /// Alarm
    ///
    /// Bitmask alarm code
    fn alarm(&self) -> Alm {
        self.alarm
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

    /// DS
    ///
    /// Digital Signature
    fn repeating_ds(&self) -> u16 {
        self.repeating_ds
    }
}
