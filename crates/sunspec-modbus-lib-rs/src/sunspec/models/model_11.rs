use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 15;

static POINTS: [PointDetails<()>; 9] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::EthernetLinkSpeed,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::InterfaceStatusFlags,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::LinkState,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::Mac,
        size: 4,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::Name,
        size: 4,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::Control,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::ForcedSpeed,
        size: 1,
        start_address: 14,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    EthernetLinkSpeed,
    InterfaceStatusFlags,
    LinkState,
    Mac,
    Name,
    Control,
    ForcedSpeed,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    15
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(11, buffer);
        }
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
