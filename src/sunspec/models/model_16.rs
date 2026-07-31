use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 54;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 16 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 52 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model16 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 {
            point: Point::Config,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model16 {
            point: Point::Control,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 {
            point: Point::Address,
        },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 {
            point: Point::Netmask,
        },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 {
            point: Point::Gateway,
        },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 { point: Point::Dns1 },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 { point: Point::Dns2 },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 { point: Point::Mac },
        size: 4,
        data_type: PointType::Eui48,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model16 {
            point: Point::LinkControl,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
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
    Config,
    Control,
    Address,
    Netmask,
    Gateway,
    Dns1,
    Dns2,
    Mac,
    LinkControl,
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
        Point::Config => serialisation::write_u16(model.config() as u16, buffer),
        Point::Control => serialisation::write_u16(model.control(), buffer),
        Point::Address => serialisation::write_string(model.address(), buffer, offset, limit),
        Point::Netmask => serialisation::write_string(model.netmask(), buffer, offset, limit),
        Point::Gateway => {
            if let Some(value) = model.gateway() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Dns1 => {
            if let Some(value) = model.dns1() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Dns2 => {
            if let Some(value) = model.dns2() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Mac => {
            if let Some(value) = model.mac() {
                serialisation::write_eui48(value, buffer, offset, limit);
            }
        }
        Point::LinkControl => {
            if let Some(value) = model.link_control() {
                serialisation::write_u16(value, buffer);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Name
    ///
    /// Interface name. (8 chars)
    fn name(&self) -> Option<&CStr> {
        None
    }

    /// Name
    ///
    /// Interface name. (8 chars)
    fn set_name(&mut self, value: &CStr) {}

    /// Config
    ///
    /// Force IPv4 configuration method
    fn config(&self) -> Cfg;

    /// Control
    ///
    /// Configure use of services
    fn control(&self) -> u16;

    /// Control
    ///
    /// Configure use of services
    fn set_control(&mut self, value: u16);

    /// Address
    ///
    /// IP address
    fn address(&self) -> &CStr;

    /// Address
    ///
    /// IP address
    fn set_address(&mut self, value: &CStr);

    /// Netmask
    ///
    /// Netmask
    fn netmask(&self) -> &CStr;

    /// Netmask
    ///
    /// Netmask
    fn set_netmask(&mut self, value: &CStr);

    /// Gateway
    ///
    /// Gateway IP address
    fn gateway(&self) -> Option<&CStr> {
        None
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn set_gateway(&mut self, value: &CStr) {}

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn dns1(&self) -> Option<&CStr> {
        None
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn set_dns1(&mut self, value: &CStr) {}

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn dns2(&self) -> Option<&CStr> {
        None
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn set_dns2(&mut self, value: &CStr) {}

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<&[u8; 6]> {
        None
    }

    /// Link Control
    ///
    /// Link control flags
    fn link_control(&self) -> Option<u16> {
        None
    }

    /// Link Control
    ///
    /// Link control flags
    fn set_link_control(&mut self, value: u16) {}
}

#[repr(u16)]
pub enum Cfg {
    Static = 0,
    Dhcp = 1,
}

#[repr(C)]
pub struct Model16CallbackAdapter {
    name_callback: Option<extern "C" fn() -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char)>,
    config_callback: extern "C" fn() -> Cfg,
    control_callback: extern "C" fn() -> u16,
    set_control_callback: extern "C" fn(u16),
    address_callback: extern "C" fn() -> *const c_char,
    set_address_callback: extern "C" fn(*const c_char),
    netmask_callback: extern "C" fn() -> *const c_char,
    set_netmask_callback: extern "C" fn(*const c_char),
    gateway_callback: Option<extern "C" fn() -> *const c_char>,
    set_gateway_callback: Option<extern "C" fn(*const c_char)>,
    dns1_callback: Option<extern "C" fn() -> *const c_char>,
    set_dns1_callback: Option<extern "C" fn(*const c_char)>,
    dns2_callback: Option<extern "C" fn() -> *const c_char>,
    set_dns2_callback: Option<extern "C" fn(*const c_char)>,
    mac_callback: Option<extern "C" fn() -> *const u8>,
    link_control_callback: Option<extern "C" fn() -> u16>,
    set_link_control_callback: Option<extern "C" fn(u16)>,
}

impl ModelAdapter for Model16CallbackAdapter {
    /// Name
    ///
    /// Interface name. (8 chars)
    fn name(&self) -> Option<&CStr> {
        self.name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Name
    ///
    /// Interface name. (8 chars)
    fn set_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_name_callback {
            (callback)(value.as_ptr());
        };
    }

    /// Config
    ///
    /// Force IPv4 configuration method
    fn config(&self) -> Cfg {
        (self.config_callback)()
    }

    /// Control
    ///
    /// Configure use of services
    fn control(&self) -> u16 {
        (self.control_callback)()
    }

    /// Control
    ///
    /// Configure use of services
    fn set_control(&mut self, value: u16) {
        (self.set_control_callback)(value);
    }

    /// Address
    ///
    /// IP address
    fn address(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.address_callback)()) }
    }

    /// Address
    ///
    /// IP address
    fn set_address(&mut self, value: &CStr) {
        (self.set_address_callback)(value.as_ptr());
    }

    /// Netmask
    ///
    /// Netmask
    fn netmask(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.netmask_callback)()) }
    }

    /// Netmask
    ///
    /// Netmask
    fn set_netmask(&mut self, value: &CStr) {
        (self.set_netmask_callback)(value.as_ptr());
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn gateway(&self) -> Option<&CStr> {
        self.gateway_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn set_gateway(&mut self, value: &CStr) {
        if let Some(callback) = self.set_gateway_callback {
            (callback)(value.as_ptr());
        };
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn dns1(&self) -> Option<&CStr> {
        self.dns1_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn set_dns1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns1_callback {
            (callback)(value.as_ptr());
        };
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn dns2(&self) -> Option<&CStr> {
        self.dns2_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn set_dns2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns2_callback {
            (callback)(value.as_ptr());
        };
    }

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<&[u8; 6]> {
        self.mac_callback
            .map(|callback| unsafe { &*((callback)() as *const [u8; 6]) })
    }

    /// Link Control
    ///
    /// Link control flags
    fn link_control(&self) -> Option<u16> {
        self.link_control_callback.map(|callback| (callback)())
    }

    /// Link Control
    ///
    /// Link control flags
    fn set_link_control(&mut self, value: u16) {
        if let Some(callback) = self.set_link_control_callback {
            (callback)(value);
        };
    }
}
