use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 100;

pub static POINTS: [ReadablePoint; 18] = [
    ReadablePoint {
        reference: PointReference::Static { value: 12 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 98 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::ConfigStatus,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::ChangeStatus,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::ConfigCapability,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::IPv4Config,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::Control,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::Ip },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::Netmask,
        },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::Gateway,
        },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::Dns1 },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::Dns2 },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::Ntp1 },
        size: 12,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::Ntp2 },
        size: 12,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::Domain,
        },
        size: 12,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 {
            point: Point::HostName,
        },
        size: 12,
        data_type: PointType::String,
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
    ConfigStatus,
    ChangeStatus,
    ConfigCapability,
    IPv4Config,
    Control,
    Ip,
    Netmask,
    Gateway,
    Dns1,
    Dns2,
    Ntp1,
    Ntp2,
    Domain,
    HostName,
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
        Point::ConfigStatus => serialisation::write_u16(model.config_status() as u16, buffer),
        Point::ChangeStatus => serialisation::write_u16(model.change_status(), buffer),
        Point::ConfigCapability => serialisation::write_u16(model.config_capability(), buffer),
        Point::IPv4Config => serialisation::write_u16(model.i_pv4_config() as u16, buffer),
        Point::Control => serialisation::write_u16(model.control() as u16, buffer),
        Point::Ip => serialisation::write_string(model.ip(), buffer, offset, limit),
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
        Point::Ntp1 => {
            if let Some(value) = model.ntp1() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Ntp2 => {
            if let Some(value) = model.ntp2() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Domain => {
            if let Some(value) = model.domain() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::HostName => {
            if let Some(value) = model.host_name() {
                serialisation::write_string(value, buffer, offset, limit);
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
    fn set_name(&mut self, value: &CStr) {}

    /// Config Status
    ///
    /// Configuration status
    fn config_status(&self) -> CfgSt;

    /// Change Status
    ///
    /// A configuration change is pending
    fn change_status(&self) -> u16;

    /// Config Capability
    ///
    /// Identify capable sources of configuration
    fn config_capability(&self) -> u16;

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn i_pv4_config(&self) -> Cfg;

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn set_i_pv4_config(&mut self, value: Cfg);

    /// Control
    ///
    /// Configure use of services
    fn control(&self) -> Ctl;

    /// Control
    ///
    /// Configure use of services
    fn set_control(&mut self, value: Ctl);

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn ip(&self) -> &CStr;

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn set_ip(&mut self, value: &CStr);

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn netmask(&self) -> &CStr;

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn set_netmask(&mut self, value: &CStr);

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn gateway(&self) -> Option<&CStr> {
        None
    }

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn set_gateway(&mut self, value: &CStr) {}

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns1(&self) -> Option<&CStr> {
        None
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns1(&mut self, value: &CStr) {}

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns2(&self) -> Option<&CStr> {
        None
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns2(&mut self, value: &CStr) {}

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp1(&self) -> Option<&CStr> {
        None
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp1(&mut self, value: &CStr) {}

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp2(&self) -> Option<&CStr> {
        None
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp2(&mut self, value: &CStr) {}

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn domain(&self) -> Option<&CStr> {
        None
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn set_domain(&mut self, value: &CStr) {}

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn host_name(&self) -> Option<&CStr> {
        None
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn set_host_name(&mut self, value: &CStr) {}
}

pub enum CfgSt {
    NotConfigured = 0,
    ValidSetting = 1,
    ValidHw = 2,
}

pub enum Cfg {
    Static = 0,
    Dhcp = 1,
    Bootp = 2,
    Zeroconf = 3,
}

pub enum Ctl {
    EnableDns = 0,
    EnableNtp = 1,
}

#[repr(C)]
pub struct Model12CallbackAdapter {
    name_callback: Option<extern "C" fn() -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char)>,
    config_status_callback: extern "C" fn() -> CfgSt,
    change_status_callback: extern "C" fn() -> u16,
    config_capability_callback: extern "C" fn() -> u16,
    i_pv4_config_callback: extern "C" fn() -> Cfg,
    set_i_pv4_config_callback: extern "C" fn(Cfg),
    control_callback: extern "C" fn() -> Ctl,
    set_control_callback: extern "C" fn(Ctl),
    ip_callback: extern "C" fn() -> *const c_char,
    set_ip_callback: extern "C" fn(*const c_char),
    netmask_callback: extern "C" fn() -> *const c_char,
    set_netmask_callback: extern "C" fn(*const c_char),
    gateway_callback: Option<extern "C" fn() -> *const c_char>,
    set_gateway_callback: Option<extern "C" fn(*const c_char)>,
    dns1_callback: Option<extern "C" fn() -> *const c_char>,
    set_dns1_callback: Option<extern "C" fn(*const c_char)>,
    dns2_callback: Option<extern "C" fn() -> *const c_char>,
    set_dns2_callback: Option<extern "C" fn(*const c_char)>,
    ntp1_callback: Option<extern "C" fn() -> *const c_char>,
    set_ntp1_callback: Option<extern "C" fn(*const c_char)>,
    ntp2_callback: Option<extern "C" fn() -> *const c_char>,
    set_ntp2_callback: Option<extern "C" fn(*const c_char)>,
    domain_callback: Option<extern "C" fn() -> *const c_char>,
    set_domain_callback: Option<extern "C" fn(*const c_char)>,
    host_name_callback: Option<extern "C" fn() -> *const c_char>,
    set_host_name_callback: Option<extern "C" fn(*const c_char)>,
}

impl ModelAdapter for Model12CallbackAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        self.name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_name_callback {
            (callback)(value.as_ptr());
        };
    }

    /// Config Status
    ///
    /// Configuration status
    fn config_status(&self) -> CfgSt {
        (self.config_status_callback)()
    }

    /// Change Status
    ///
    /// A configuration change is pending
    fn change_status(&self) -> u16 {
        (self.change_status_callback)()
    }

    /// Config Capability
    ///
    /// Identify capable sources of configuration
    fn config_capability(&self) -> u16 {
        (self.config_capability_callback)()
    }

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn i_pv4_config(&self) -> Cfg {
        (self.i_pv4_config_callback)()
    }

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn set_i_pv4_config(&mut self, value: Cfg) {
        (self.set_i_pv4_config_callback)(value);
    }

    /// Control
    ///
    /// Configure use of services
    fn control(&self) -> Ctl {
        (self.control_callback)()
    }

    /// Control
    ///
    /// Configure use of services
    fn set_control(&mut self, value: Ctl) {
        (self.set_control_callback)(value);
    }

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn ip(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.ip_callback)()) }
    }

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn set_ip(&mut self, value: &CStr) {
        (self.set_ip_callback)(value.as_ptr());
    }

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn netmask(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.netmask_callback)()) }
    }

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn set_netmask(&mut self, value: &CStr) {
        (self.set_netmask_callback)(value.as_ptr());
    }

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn gateway(&self) -> Option<&CStr> {
        self.gateway_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn set_gateway(&mut self, value: &CStr) {
        if let Some(callback) = self.set_gateway_callback {
            (callback)(value.as_ptr());
        };
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns1(&self) -> Option<&CStr> {
        self.dns1_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns1_callback {
            (callback)(value.as_ptr());
        };
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns2(&self) -> Option<&CStr> {
        self.dns2_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns2_callback {
            (callback)(value.as_ptr());
        };
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp1(&self) -> Option<&CStr> {
        self.ntp1_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_ntp1_callback {
            (callback)(value.as_ptr());
        };
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp2(&self) -> Option<&CStr> {
        self.ntp2_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_ntp2_callback {
            (callback)(value.as_ptr());
        };
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn domain(&self) -> Option<&CStr> {
        self.domain_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn set_domain(&mut self, value: &CStr) {
        if let Some(callback) = self.set_domain_callback {
            (callback)(value.as_ptr());
        };
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn host_name(&self) -> Option<&CStr> {
        self.host_name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn set_host_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_host_name_callback {
            (callback)(value.as_ptr());
        };
    }
}
