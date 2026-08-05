use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 15;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 11 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 {
            point: Point::EthernetLinkSpeed,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 {
            point: Point::InterfaceStatusFlags,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 {
            point: Point::LinkState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::Mac },
        size: 4,
        data_type: PointType::Eui48,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model11 {
            point: Point::Control,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model11 {
            point: Point::ForcedSpeed,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    EthernetLinkSpeed,
    InterfaceStatusFlags,
    LinkState,
    Mac,
    Name,
    Control,
    ForcedSpeed,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    15
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
        Point::EthernetLinkSpeed => {
            buffer::write_u16(model.ethernet_link_speed(), buffer);
        }
        Point::InterfaceStatusFlags => {
            buffer::write_u16(model.interface_status_flags(), buffer);
        }
        Point::LinkState => {
            buffer::write_u16(model.link_state() as u16, buffer);
        }
        Point::Mac => {
            if let Some(value) = model.mac() {
                buffer::write_eui48(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Name => {
            if let Some(value) = model.name() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Control => {
            if let Some(value) = model.control() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ForcedSpeed => {
            if let Some(value) = model.forced_speed() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    fn ethernet_link_speed(&self) -> u16;

    /// Interface Status Flags
    ///
    /// Interface flags.
    fn interface_status_flags(&self) -> u16;

    /// Link State
    ///
    /// State information for this interface
    fn link_state(&self) -> St;

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<&[u8; 6]> {
        None
    }

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

    /// Control
    ///
    /// Control flags
    fn control(&self) -> Option<u16> {
        None
    }

    /// Control
    ///
    /// Control flags
    fn set_control(&mut self, value: u16) {}

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn forced_speed(&self) -> Option<u16> {
        None
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn set_forced_speed(&mut self, value: u16) {}
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum St {
    Unknown = 0,
    Enabled = 1,
    Disabled = 2,
    Testing = 3,
}

#[repr(C)]
pub struct Model11CallbackAdapter {
    context: *mut c_void,
    ethernet_link_speed_callback: extern "C" fn(*const c_void) -> u16,
    interface_status_flags_callback: extern "C" fn(*const c_void) -> u16,
    link_state_callback: extern "C" fn(*const c_void) -> St,
    mac_callback: Option<extern "C" fn(*const c_void) -> *const u8>,
    name_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    control_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_control_callback: Option<extern "C" fn(u16, *mut c_void)>,
    forced_speed_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_forced_speed_callback: Option<extern "C" fn(u16, *mut c_void)>,
}

impl ModelAdapter for Model11CallbackAdapter {
    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    fn ethernet_link_speed(&self) -> u16 {
        (self.ethernet_link_speed_callback)(self.context)
    }

    /// Interface Status Flags
    ///
    /// Interface flags.
    fn interface_status_flags(&self) -> u16 {
        (self.interface_status_flags_callback)(self.context)
    }

    /// Link State
    ///
    /// State information for this interface
    fn link_state(&self) -> St {
        (self.link_state_callback)(self.context)
    }

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<&[u8; 6]> {
        self.mac_callback
            .map(|callback| unsafe { &*((callback)(self.context) as *const [u8; 6]) })
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn name(&self) -> Option<&CStr> {
        self.name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn set_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_name_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// Control
    ///
    /// Control flags
    fn control(&self) -> Option<u16> {
        self.control_callback
            .map(|callback| (callback)(self.context))
    }

    /// Control
    ///
    /// Control flags
    fn set_control(&mut self, value: u16) {
        if let Some(callback) = self.set_control_callback {
            (callback)(value, self.context);
        };
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn forced_speed(&self) -> Option<u16> {
        self.forced_speed_callback
            .map(|callback| (callback)(self.context))
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn set_forced_speed(&mut self, value: u16) {
        if let Some(callback) = self.set_forced_speed_callback {
            (callback)(value, self.context);
        };
    }
}

#[repr(C)]
pub struct Model11StatefulAdapter {
    ethernet_link_speed: u16,
    interface_status_flags: u16,
    link_state: St,
    mac: [u8; 6],
    name: [c_char; 8],
    control: u16,
    forced_speed: u16,
}

impl ModelAdapter for Model11StatefulAdapter {
    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    fn ethernet_link_speed(&self) -> u16 {
        self.ethernet_link_speed
    }

    /// Interface Status Flags
    ///
    /// Interface flags.
    fn interface_status_flags(&self) -> u16 {
        self.interface_status_flags
    }

    /// Link State
    ///
    /// State information for this interface
    fn link_state(&self) -> St {
        self.link_state
    }

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<&[u8; 6]> {
        Some(unsafe { &*(self.mac.as_ptr() as *const [u8; 6]) })
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn name(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.name.as_ptr()) })
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn set_name(&mut self, value: &CStr) {
        for (dest, src) in self.name.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Control
    ///
    /// Control flags
    fn control(&self) -> Option<u16> {
        Some(self.control)
    }

    /// Control
    ///
    /// Control flags
    fn set_control(&mut self, value: u16) {
        self.control = value;
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn forced_speed(&self) -> Option<u16> {
        Some(self.forced_speed)
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn set_forced_speed(&mut self, value: u16) {
        self.forced_speed = value;
    }
}
