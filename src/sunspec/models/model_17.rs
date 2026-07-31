use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 14;

pub static POINTS: [ReadablePoint; 10] = [
    ReadablePoint {
        reference: PointReference::Static { value: 17 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 12 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model17 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model17 { point: Point::Rate },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model17 { point: Point::Bits },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model17 {
            point: Point::Parity,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model17 {
            point: Point::Duplex,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model17 {
            point: Point::FlowControl,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model17 {
            point: Point::InterfaceType,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model17 {
            point: Point::Protocol,
        },
        size: 1,
        data_type: PointType::Enum16,
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
    InterfaceType,
    Protocol,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Name => {
            if let Some(value) = model.name() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Rate => serialisation::write_u32(model.rate(), buffer, offset, limit),
        Point::Bits => serialisation::write_u16(model.bits(), buffer),
        Point::Parity => serialisation::write_u16(model.parity() as u16, buffer),
        Point::Duplex => {
            if let Some(value) = model.duplex() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::FlowControl => {
            if let Some(value) = model.flow_control() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::InterfaceType => {
            if let Some(value) = model.interface_type() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::Protocol => {
            if let Some(value) = model.protocol() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Name
    ///
    /// Interface name (8 chars)
    fn name(&self) -> Option<&CStr> {
        None
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn set_name(&mut self, value: &CStr) {}

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
    fn set_duplex(&mut self, value: Dup) {}

    /// Flow Control
    ///
    /// Flow Control Method
    fn flow_control(&self) -> Option<Flw> {
        None
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn set_flow_control(&mut self, value: Flw) {}

    /// Interface Type
    ///
    /// Interface type
    fn interface_type(&self) -> Option<Typ> {
        None
    }

    /// Protocol
    ///
    /// Serial protocol selection
    fn protocol(&self) -> Option<Pcol> {
        None
    }
}

pub enum Pty {
    None = 0,
    Odd = 1,
    Even = 2,
}

pub enum Dup {
    Full = 0,
    Half = 1,
}

pub enum Flw {
    None = 0,
    Hw = 1,
    Xonxoff = 2,
}

pub enum Typ {
    Unknown = 0,
    Rs232 = 1,
    Rs485 = 2,
}

pub enum Pcol {
    Unknown = 0,
    Modbus = 1,
    Vendor = 2,
}

#[repr(C)]
pub struct Model17CallbackAdapter {
    name_callback: Option<extern "C" fn() -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char)>,
    rate_callback: extern "C" fn() -> u32,
    set_rate_callback: extern "C" fn(u32),
    bits_callback: extern "C" fn() -> u16,
    set_bits_callback: extern "C" fn(u16),
    parity_callback: extern "C" fn() -> Pty,
    set_parity_callback: extern "C" fn(Pty),
    duplex_callback: Option<extern "C" fn() -> Dup>,
    set_duplex_callback: Option<extern "C" fn(Dup)>,
    flow_control_callback: Option<extern "C" fn() -> Flw>,
    set_flow_control_callback: Option<extern "C" fn(Flw)>,
    interface_type_callback: Option<extern "C" fn() -> Typ>,
    protocol_callback: Option<extern "C" fn() -> Pcol>,
}

impl ModelAdapter for Model17CallbackAdapter {
    /// Name
    ///
    /// Interface name (8 chars)
    fn name(&self) -> Option<&CStr> {
        self.name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn set_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_name_callback {
            (callback)(value.as_ptr());
        };
    }

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn rate(&self) -> u32 {
        (self.rate_callback)()
    }

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn set_rate(&mut self, value: u32) {
        (self.set_rate_callback)(value);
    }

    /// Bits
    ///
    /// Number of data bits per character
    fn bits(&self) -> u16 {
        (self.bits_callback)()
    }

    /// Bits
    ///
    /// Number of data bits per character
    fn set_bits(&mut self, value: u16) {
        (self.set_bits_callback)(value);
    }

    /// Parity
    ///
    /// Parity setting
    fn parity(&self) -> Pty {
        (self.parity_callback)()
    }

    /// Parity
    ///
    /// Parity setting
    fn set_parity(&mut self, value: Pty) {
        (self.set_parity_callback)(value);
    }

    /// Duplex
    ///
    /// Duplex mode
    fn duplex(&self) -> Option<Dup> {
        self.duplex_callback.map(|callback| (callback)())
    }

    /// Duplex
    ///
    /// Duplex mode
    fn set_duplex(&mut self, value: Dup) {
        if let Some(callback) = self.set_duplex_callback {
            (callback)(value);
        };
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn flow_control(&self) -> Option<Flw> {
        self.flow_control_callback.map(|callback| (callback)())
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn set_flow_control(&mut self, value: Flw) {
        if let Some(callback) = self.set_flow_control_callback {
            (callback)(value);
        };
    }

    /// Interface Type
    ///
    /// Interface type
    fn interface_type(&self) -> Option<Typ> {
        self.interface_type_callback.map(|callback| (callback)())
    }

    /// Protocol
    ///
    /// Serial protocol selection
    fn protocol(&self) -> Option<Pcol> {
        self.protocol_callback.map(|callback| (callback)())
    }
}
