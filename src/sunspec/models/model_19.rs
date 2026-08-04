use core::ffi::{CStr, c_char, c_void};
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 32;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 19 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 30 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Rate },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Bits },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Parity },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Duplex },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::FlowControl },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Authentication },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Username },
        size: 12,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model19 { point: Point::Password },
        size: 6,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Name,
    Rate,
    Bits,
    Parity,
    Duplex,
    FlowControl,
    Authentication,
    Username,
    Password,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Name => {
            if let Some(value) = model.name() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Rate => {
            serialisation::write_u32(model.rate(), buffer, offset, limit);
        },
        Point::Bits => {
            serialisation::write_u16(model.bits(), buffer);
        },
        Point::Parity => {
            serialisation::write_u16(model.parity() as u16, buffer);
        },
        Point::Duplex => {
            if let Some(value) = model.duplex() {
                serialisation::write_u16(value as u16, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::FlowControl => {
            if let Some(value) = model.flow_control() {
                serialisation::write_u16(value as u16, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Authentication => {
            if let Some(value) = model.authentication() {
                serialisation::write_u16(value as u16, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Username => {
            if let Some(value) = model.username() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Password => {
            if let Some(value) = model.password() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
    }
}

pub trait ModelAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        None
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: &CStr) {
    }

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn rate(&self) -> u32;

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn set_rate(&mut self, value: u32);

    /// Bits
    ///
    /// Number of data bits per character
    fn bits(&self) -> u16;

    /// Bits
    ///
    /// Number of data bits per character
    fn set_bits(&mut self, value: u16);

    /// Parity
    ///
    /// Parity setting
    fn parity(&self) -> Pty;

    /// Parity
    ///
    /// Parity setting
    fn set_parity(&mut self, value: Pty);

    /// Duplex
    ///
    /// Duplex mode
    fn duplex(&self) -> Option<Dup> {
        None
    }

    /// Duplex
    ///
    /// Duplex mode
    fn set_duplex(&mut self, value: Dup) {
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn flow_control(&self) -> Option<Flw> {
        None
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn set_flow_control(&mut self, value: Flw) {
    }

    /// Authentication
    ///
    /// Authentication method
    fn authentication(&self) -> Option<Auth> {
        None
    }

    /// Username
    ///
    /// Username for authentication
    fn username(&self) -> Option<&CStr> {
        None
    }

    /// Password
    ///
    /// Password for authentication
    fn password(&self) -> Option<&CStr> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Pty {
    None = 0,
    Odd = 1,
    Even = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Dup {
    Full = 0,
    Half = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Flw {
    None = 0,
    Hw = 1,
    Xonxoff = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Auth {
    None = 0,
    Pap = 1,
    Chap = 2,
}

#[repr(C)]
pub struct Model19CallbackAdapter {
    context: *mut c_void,
    name_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    rate_callback: extern "C" fn(*const c_void) -> u32,
    set_rate_callback: extern "C" fn(u32, *mut c_void),
    bits_callback: extern "C" fn(*const c_void) -> u16,
    set_bits_callback: extern "C" fn(u16, *mut c_void),
    parity_callback: extern "C" fn(*const c_void) -> Pty,
    set_parity_callback: extern "C" fn(Pty, *mut c_void),
    duplex_callback: Option<extern "C" fn(*const c_void) -> Dup>,
    set_duplex_callback: Option<extern "C" fn(Dup, *mut c_void)>,
    flow_control_callback: Option<extern "C" fn(*const c_void) -> Flw>,
    set_flow_control_callback: Option<extern "C" fn(Flw, *mut c_void)>,
    authentication_callback: Option<extern "C" fn(*const c_void) -> Auth>,
    username_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    password_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
}

impl ModelAdapter for Model19CallbackAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        self.name_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_name_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn rate(&self) -> u32 {
        (self.rate_callback)(self.context)
    }

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn set_rate(&mut self, value: u32) {
        (self.set_rate_callback)(value, self.context);
    }

    /// Bits
    ///
    /// Number of data bits per character
    fn bits(&self) -> u16 {
        (self.bits_callback)(self.context)
    }

    /// Bits
    ///
    /// Number of data bits per character
    fn set_bits(&mut self, value: u16) {
        (self.set_bits_callback)(value, self.context);
    }

    /// Parity
    ///
    /// Parity setting
    fn parity(&self) -> Pty {
        (self.parity_callback)(self.context)
    }

    /// Parity
    ///
    /// Parity setting
    fn set_parity(&mut self, value: Pty) {
        (self.set_parity_callback)(value, self.context);
    }

    /// Duplex
    ///
    /// Duplex mode
    fn duplex(&self) -> Option<Dup> {
        self.duplex_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Duplex
    ///
    /// Duplex mode
    fn set_duplex(&mut self, value: Dup) {
        if let Some(callback) = self.set_duplex_callback {
        (callback)(value, self.context);
        };
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn flow_control(&self) -> Option<Flw> {
        self.flow_control_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn set_flow_control(&mut self, value: Flw) {
        if let Some(callback) = self.set_flow_control_callback {
        (callback)(value, self.context);
        };
    }

    /// Authentication
    ///
    /// Authentication method
    fn authentication(&self) -> Option<Auth> {
        self.authentication_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Username
    ///
    /// Username for authentication
    fn username(&self) -> Option<&CStr> {
        self.username_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// Password
    ///
    /// Password for authentication
    fn password(&self) -> Option<&CStr> {
        self.password_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }
}

#[repr(C)]
pub struct Model19StatefulAdapter {
    name: [c_char; 8],
    rate: u32,
    bits: u16,
    parity: Pty,
    duplex: Dup,
    flow_control: Flw,
    authentication: Auth,
    username: [c_char; 24],
    password: [c_char; 12],
}

impl ModelAdapter for Model19StatefulAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.name.as_ptr()) }
        )
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: &CStr) {
        for (dest, src) in self.name.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn rate(&self) -> u32 {
        self.rate
    }

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn set_rate(&mut self, value: u32) {
        self.rate = value;
    }

    /// Bits
    ///
    /// Number of data bits per character
    fn bits(&self) -> u16 {
        self.bits
    }

    /// Bits
    ///
    /// Number of data bits per character
    fn set_bits(&mut self, value: u16) {
        self.bits = value;
    }

    /// Parity
    ///
    /// Parity setting
    fn parity(&self) -> Pty {
        self.parity
    }

    /// Parity
    ///
    /// Parity setting
    fn set_parity(&mut self, value: Pty) {
        self.parity = value;
    }

    /// Duplex
    ///
    /// Duplex mode
    fn duplex(&self) -> Option<Dup> {
        Some(
        self.duplex
        )
    }

    /// Duplex
    ///
    /// Duplex mode
    fn set_duplex(&mut self, value: Dup) {
        self.duplex = value;
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn flow_control(&self) -> Option<Flw> {
        Some(
        self.flow_control
        )
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn set_flow_control(&mut self, value: Flw) {
        self.flow_control = value;
    }

    /// Authentication
    ///
    /// Authentication method
    fn authentication(&self) -> Option<Auth> {
        Some(
        self.authentication
        )
    }

    /// Username
    ///
    /// Username for authentication
    fn username(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.username.as_ptr()) }
        )
    }

    /// Password
    ///
    /// Password for authentication
    fn password(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.password.as_ptr()) }
        )
    }
}