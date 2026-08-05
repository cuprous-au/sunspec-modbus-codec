use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 54;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 16 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model16 {
            point: Point::ModelLength,
        },
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
    ModelLength,
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

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    54
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
        Point::Name => {
            if let Some(value) = model.name() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Config => {
            buffer::write_u16(model.config() as u16, buffer);
        }
        Point::Control => {
            buffer::write_u16(model.control(), buffer);
        }
        Point::Address => {
            buffer::write_string(model.address(), buffer, offset, limit);
        }
        Point::Netmask => {
            buffer::write_string(model.netmask(), buffer, offset, limit);
        }
        Point::Gateway => {
            if let Some(value) = model.gateway() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Dns1 => {
            if let Some(value) = model.dns1() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Dns2 => {
            if let Some(value) = model.dns2() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Mac => {
            if let Some(value) = model.mac() {
                buffer::write_eui48(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LinkControl => {
            if let Some(value) = model.link_control() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
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

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Cfg {
    Static = 0,
    Dhcp = 1,
}

#[repr(C)]
pub struct Model16CallbackAdapter {
    context: *mut c_void,
    name_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    config_callback: extern "C" fn(*const c_void) -> Cfg,
    control_callback: extern "C" fn(*const c_void) -> u16,
    set_control_callback: extern "C" fn(u16, *mut c_void),
    address_callback: extern "C" fn(*const c_void) -> *const c_char,
    set_address_callback: extern "C" fn(*const c_char, *mut c_void),
    netmask_callback: extern "C" fn(*const c_void) -> *const c_char,
    set_netmask_callback: extern "C" fn(*const c_char, *mut c_void),
    gateway_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_gateway_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    dns1_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_dns1_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    dns2_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_dns2_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    mac_callback: Option<extern "C" fn(*const c_void) -> *const u8>,
    link_control_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_link_control_callback: Option<extern "C" fn(u16, *mut c_void)>,
}

impl ModelAdapter for Model16CallbackAdapter {
    /// Name
    ///
    /// Interface name. (8 chars)
    fn name(&self) -> Option<&CStr> {
        self.name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Name
    ///
    /// Interface name. (8 chars)
    fn set_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_name_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// Config
    ///
    /// Force IPv4 configuration method
    fn config(&self) -> Cfg {
        (self.config_callback)(self.context)
    }

    /// Control
    ///
    /// Configure use of services
    fn control(&self) -> u16 {
        (self.control_callback)(self.context)
    }

    /// Control
    ///
    /// Configure use of services
    fn set_control(&mut self, value: u16) {
        (self.set_control_callback)(value, self.context);
    }

    /// Address
    ///
    /// IP address
    fn address(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.address_callback)(self.context)) }
    }

    /// Address
    ///
    /// IP address
    fn set_address(&mut self, value: &CStr) {
        (self.set_address_callback)(value.as_ptr(), self.context);
    }

    /// Netmask
    ///
    /// Netmask
    fn netmask(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.netmask_callback)(self.context)) }
    }

    /// Netmask
    ///
    /// Netmask
    fn set_netmask(&mut self, value: &CStr) {
        (self.set_netmask_callback)(value.as_ptr(), self.context);
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn gateway(&self) -> Option<&CStr> {
        self.gateway_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn set_gateway(&mut self, value: &CStr) {
        if let Some(callback) = self.set_gateway_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn dns1(&self) -> Option<&CStr> {
        self.dns1_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn set_dns1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns1_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn dns2(&self) -> Option<&CStr> {
        self.dns2_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn set_dns2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns2_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<&[u8; 6]> {
        self.mac_callback
            .map(|callback| unsafe { &*((callback)(self.context) as *const [u8; 6]) })
    }

    /// Link Control
    ///
    /// Link control flags
    fn link_control(&self) -> Option<u16> {
        self.link_control_callback
            .map(|callback| (callback)(self.context))
    }

    /// Link Control
    ///
    /// Link control flags
    fn set_link_control(&mut self, value: u16) {
        if let Some(callback) = self.set_link_control_callback {
            (callback)(value, self.context);
        };
    }
}

#[repr(C)]
pub struct Model16StatefulAdapter {
    name: [c_char; 8],
    config: Cfg,
    control: u16,
    address: [c_char; 16],
    netmask: [c_char; 16],
    gateway: [c_char; 16],
    dns1: [c_char; 16],
    dns2: [c_char; 16],
    mac: [u8; 6],
    link_control: u16,
}

impl ModelAdapter for Model16StatefulAdapter {
    /// Name
    ///
    /// Interface name. (8 chars)
    fn name(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.name.as_ptr()) })
    }

    /// Name
    ///
    /// Interface name. (8 chars)
    fn set_name(&mut self, value: &CStr) {
        for (dest, src) in self.name.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Config
    ///
    /// Force IPv4 configuration method
    fn config(&self) -> Cfg {
        self.config
    }

    /// Control
    ///
    /// Configure use of services
    fn control(&self) -> u16 {
        self.control
    }

    /// Control
    ///
    /// Configure use of services
    fn set_control(&mut self, value: u16) {
        self.control = value;
    }

    /// Address
    ///
    /// IP address
    fn address(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.address.as_ptr()) }
    }

    /// Address
    ///
    /// IP address
    fn set_address(&mut self, value: &CStr) {
        for (dest, src) in self
            .address
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// Netmask
    ///
    /// Netmask
    fn netmask(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.netmask.as_ptr()) }
    }

    /// Netmask
    ///
    /// Netmask
    fn set_netmask(&mut self, value: &CStr) {
        for (dest, src) in self
            .netmask
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn gateway(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.gateway.as_ptr()) })
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn set_gateway(&mut self, value: &CStr) {
        for (dest, src) in self
            .gateway
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn dns1(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.dns1.as_ptr()) })
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn set_dns1(&mut self, value: &CStr) {
        for (dest, src) in self.dns1.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn dns2(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.dns2.as_ptr()) })
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn set_dns2(&mut self, value: &CStr) {
        for (dest, src) in self.dns2.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<&[u8; 6]> {
        Some(unsafe { &*(self.mac.as_ptr() as *const [u8; 6]) })
    }

    /// Link Control
    ///
    /// Link control flags
    fn link_control(&self) -> Option<u16> {
        Some(self.link_control)
    }

    /// Link Control
    ///
    /// Link control flags
    fn set_link_control(&mut self, value: u16) {
        self.link_control = value;
    }
}
