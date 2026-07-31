use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 176;

pub static POINTS: [ReadablePoint; 18] = [
    ReadablePoint {
        reference: PointReference::Static { value: 13 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 174 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 {
            point: Point::ConfigStatus,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model13 {
            point: Point::ChangeStatus,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model13 {
            point: Point::ConfigCapability,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model13 {
            point: Point::IPv6Config,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 {
            point: Point::Control,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::Ip },
        size: 20,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::Cidr },
        size: 20,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 {
            point: Point::Gateway,
        },
        size: 20,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::Dns1 },
        size: 20,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::Dns2 },
        size: 20,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::Ntp1 },
        size: 20,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::Ntp2 },
        size: 20,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 {
            point: Point::Domain,
        },
        size: 12,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 {
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
    IPv6Config,
    Control,
    Ip,
    Cidr,
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
        Point::IPv6Config => serialisation::write_u16(model.i_pv6_config() as u16, buffer),
        Point::Control => serialisation::write_u16(model.control() as u16, buffer),
        Point::Ip => serialisation::write_string(model.ip(), buffer, offset, limit),
        Point::Cidr => {
            if let Some(value) = model.cidr() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
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

    /// IPv6 Config
    ///
    /// Configuration method used.
    fn i_pv6_config(&self) -> Cfg;

    /// IPv6 Config
    ///
    /// Configuration method used.
    fn set_i_pv6_config(&mut self, value: Cfg);

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
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn ip(&self) -> &CStr;

    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ip(&mut self, value: &CStr);

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn cidr(&self) -> Option<&CStr> {
        None
    }

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn set_cidr(&mut self, value: &CStr) {}

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn gateway(&self) -> Option<&CStr> {
        None
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_gateway(&mut self, value: &CStr) {}

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns1(&self) -> Option<&CStr> {
        None
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns1(&mut self, value: &CStr) {}

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns2(&self) -> Option<&CStr> {
        None
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns2(&mut self, value: &CStr) {}

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp1(&self) -> Option<&CStr> {
        None
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp1(&mut self, value: &CStr) {}

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp2(&self) -> Option<&CStr> {
        None
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
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
pub struct Model13CallbackAdapter {
    name_callback: Option<extern "C" fn() -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char)>,
    config_status_callback: extern "C" fn() -> CfgSt,
    change_status_callback: extern "C" fn() -> u16,
    config_capability_callback: extern "C" fn() -> u16,
    i_pv6_config_callback: extern "C" fn() -> Cfg,
    set_i_pv6_config_callback: extern "C" fn(Cfg),
    control_callback: extern "C" fn() -> Ctl,
    set_control_callback: extern "C" fn(Ctl),
    ip_callback: extern "C" fn() -> *const c_char,
    set_ip_callback: extern "C" fn(*const c_char),
    cidr_callback: Option<extern "C" fn() -> *const c_char>,
    set_cidr_callback: Option<extern "C" fn(*const c_char)>,
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

impl ModelAdapter for Model13CallbackAdapter {
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

    /// IPv6 Config
    ///
    /// Configuration method used.
    fn i_pv6_config(&self) -> Cfg {
        (self.i_pv6_config_callback)()
    }

    /// IPv6 Config
    ///
    /// Configuration method used.
    fn set_i_pv6_config(&mut self, value: Cfg) {
        (self.set_i_pv6_config_callback)(value);
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
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn ip(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.ip_callback)()) }
    }

    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ip(&mut self, value: &CStr) {
        (self.set_ip_callback)(value.as_ptr());
    }

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn cidr(&self) -> Option<&CStr> {
        self.cidr_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn set_cidr(&mut self, value: &CStr) {
        if let Some(callback) = self.set_cidr_callback {
            (callback)(value.as_ptr());
        };
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn gateway(&self) -> Option<&CStr> {
        self.gateway_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_gateway(&mut self, value: &CStr) {
        if let Some(callback) = self.set_gateway_callback {
            (callback)(value.as_ptr());
        };
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns1(&self) -> Option<&CStr> {
        self.dns1_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns1_callback {
            (callback)(value.as_ptr());
        };
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns2(&self) -> Option<&CStr> {
        self.dns2_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns2_callback {
            (callback)(value.as_ptr());
        };
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp1(&self) -> Option<&CStr> {
        self.ntp1_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_ntp1_callback {
            (callback)(value.as_ptr());
        };
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp2(&self) -> Option<&CStr> {
        self.ntp2_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
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
