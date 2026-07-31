use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 15;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 11 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 13 },
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
    EthernetLinkSpeed,
    InterfaceStatusFlags,
    LinkState,
    Mac,
    Name,
    Control,
    ForcedSpeed,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::EthernetLinkSpeed => serialisation::write_u16(model.ethernet_link_speed(), buffer),
        Point::InterfaceStatusFlags => {
            serialisation::write_u16(model.interface_status_flags(), buffer)
        }
        Point::LinkState => serialisation::write_u16(model.link_state() as u16, buffer),
        Point::Mac => {
            if let Some(value) = model.mac() {
                serialisation::write_eui48(value, buffer, offset, limit);
            }
        }
        Point::Name => {
            if let Some(value) = model.name() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Control => {
            if let Some(value) = model.control() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ForcedSpeed => {
            if let Some(value) = model.forced_speed() {
                serialisation::write_u16(value, buffer);
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

#[repr(u16)]
pub enum St {
    Unknown = 0,
    Enabled = 1,
    Disabled = 2,
    Testing = 3,
}

#[repr(C)]
pub struct Model11CallbackAdapter {
    ethernet_link_speed_callback: extern "C" fn() -> u16,
    interface_status_flags_callback: extern "C" fn() -> u16,
    link_state_callback: extern "C" fn() -> St,
    mac_callback: Option<extern "C" fn() -> *const u8>,
    name_callback: Option<extern "C" fn() -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char)>,
    control_callback: Option<extern "C" fn() -> u16>,
    set_control_callback: Option<extern "C" fn(u16)>,
    forced_speed_callback: Option<extern "C" fn() -> u16>,
    set_forced_speed_callback: Option<extern "C" fn(u16)>,
}

impl ModelAdapter for Model11CallbackAdapter {
    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    fn ethernet_link_speed(&self) -> u16 {
        (self.ethernet_link_speed_callback)()
    }

    /// Interface Status Flags
    ///
    /// Interface flags.
    fn interface_status_flags(&self) -> u16 {
        (self.interface_status_flags_callback)()
    }

    /// Link State
    ///
    /// State information for this interface
    fn link_state(&self) -> St {
        (self.link_state_callback)()
    }

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<&[u8; 6]> {
        self.mac_callback
            .map(|callback| unsafe { &*((callback)() as *const [u8; 6]) })
    }

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

    /// Control
    ///
    /// Control flags
    fn control(&self) -> Option<u16> {
        self.control_callback.map(|callback| (callback)())
    }

    /// Control
    ///
    /// Control flags
    fn set_control(&mut self, value: u16) {
        if let Some(callback) = self.set_control_callback {
            (callback)(value);
        };
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn forced_speed(&self) -> Option<u16> {
        self.forced_speed_callback.map(|callback| (callback)())
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn set_forced_speed(&mut self, value: u16) {
        if let Some(callback) = self.set_forced_speed_callback {
            (callback)(value);
        };
    }
}
