use core::ffi::{CStr, c_char, c_void};
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

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
        reference: PointReference::Model13 { point: Point::ConfigStatus },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::ChangeStatus },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::ConfigCapability },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::IPv6Config },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::Control },
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
        reference: PointReference::Model13 { point: Point::Gateway },
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
        reference: PointReference::Model13 { point: Point::Domain },
        size: 12,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model13 { point: Point::HostName },
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
        Point::ConfigStatus => {
            serialisation::write_u16(model.config_status() as u16, buffer);
        },
        Point::ChangeStatus => {
            serialisation::write_u16(model.change_status(), buffer);
        },
        Point::ConfigCapability => {
            serialisation::write_u16(model.config_capability(), buffer);
        },
        Point::IPv6Config => {
            serialisation::write_u16(model.i_pv6_config() as u16, buffer);
        },
        Point::Control => {
            serialisation::write_u16(model.control() as u16, buffer);
        },
        Point::Ip => {
            serialisation::write_string(model.ip(), buffer, offset, limit);
        },
        Point::Cidr => {
            if let Some(value) = model.cidr() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Gateway => {
            if let Some(value) = model.gateway() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Dns1 => {
            if let Some(value) = model.dns1() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Dns2 => {
            if let Some(value) = model.dns2() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Ntp1 => {
            if let Some(value) = model.ntp1() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Ntp2 => {
            if let Some(value) = model.ntp2() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Domain => {
            if let Some(value) = model.domain() {
                serialisation::write_string(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::HostName => {
            if let Some(value) = model.host_name() {
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
    fn set_cidr(&mut self, value: &CStr) {
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn gateway(&self) -> Option<&CStr> {
        None
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_gateway(&mut self, value: &CStr) {
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns1(&self) -> Option<&CStr> {
        None
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns1(&mut self, value: &CStr) {
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns2(&self) -> Option<&CStr> {
        None
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns2(&mut self, value: &CStr) {
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp1(&self) -> Option<&CStr> {
        None
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp1(&mut self, value: &CStr) {
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp2(&self) -> Option<&CStr> {
        None
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp2(&mut self, value: &CStr) {
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn domain(&self) -> Option<&CStr> {
        None
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn set_domain(&mut self, value: &CStr) {
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn host_name(&self) -> Option<&CStr> {
        None
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn set_host_name(&mut self, value: &CStr) {
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum CfgSt {
    NotConfigured = 0,
    ValidSetting = 1,
    ValidHw = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Cfg {
    Static = 0,
    Dhcp = 1,
    Bootp = 2,
    Zeroconf = 3,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Ctl {
    EnableDns = 0,
    EnableNtp = 1,
}

#[repr(C)]
pub struct Model13CallbackAdapter {
    context: *mut c_void,
    name_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    config_status_callback: extern "C" fn(*const c_void) -> CfgSt,
    change_status_callback: extern "C" fn(*const c_void) -> u16,
    config_capability_callback: extern "C" fn(*const c_void) -> u16,
    i_pv6_config_callback: extern "C" fn(*const c_void) -> Cfg,
    set_i_pv6_config_callback: extern "C" fn(Cfg, *mut c_void),
    control_callback: extern "C" fn(*const c_void) -> Ctl,
    set_control_callback: extern "C" fn(Ctl, *mut c_void),
    ip_callback: extern "C" fn(*const c_void) -> *const c_char,
    set_ip_callback: extern "C" fn(*const c_char, *mut c_void),
    cidr_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_cidr_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    gateway_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_gateway_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    dns1_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_dns1_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    dns2_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_dns2_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    ntp1_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_ntp1_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    ntp2_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_ntp2_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    domain_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_domain_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    host_name_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_host_name_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
}

impl ModelAdapter for Model13CallbackAdapter {
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

    /// Config Status
    ///
    /// Configuration status
    fn config_status(&self) -> CfgSt {
        (self.config_status_callback)(self.context)
    }

    /// Change Status
    ///
    /// A configuration change is pending
    fn change_status(&self) -> u16 {
        (self.change_status_callback)(self.context)
    }

    /// Config Capability
    ///
    /// Identify capable sources of configuration
    fn config_capability(&self) -> u16 {
        (self.config_capability_callback)(self.context)
    }

    /// IPv6 Config
    ///
    /// Configuration method used.
    fn i_pv6_config(&self) -> Cfg {
        (self.i_pv6_config_callback)(self.context)
    }

    /// IPv6 Config
    ///
    /// Configuration method used.
    fn set_i_pv6_config(&mut self, value: Cfg) {
        (self.set_i_pv6_config_callback)(value, self.context);
    }

    /// Control
    ///
    /// Configure use of services
    fn control(&self) -> Ctl {
        (self.control_callback)(self.context)
    }

    /// Control
    ///
    /// Configure use of services
    fn set_control(&mut self, value: Ctl) {
        (self.set_control_callback)(value, self.context);
    }

    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn ip(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.ip_callback)(self.context)) }
    }

    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ip(&mut self, value: &CStr) {
        (self.set_ip_callback)(value.as_ptr(), self.context);
    }

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn cidr(&self) -> Option<&CStr> {
        self.cidr_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn set_cidr(&mut self, value: &CStr) {
        if let Some(callback) = self.set_cidr_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn gateway(&self) -> Option<&CStr> {
        self.gateway_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_gateway(&mut self, value: &CStr) {
        if let Some(callback) = self.set_gateway_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns1(&self) -> Option<&CStr> {
        self.dns1_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns1_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns2(&self) -> Option<&CStr> {
        self.dns2_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns2_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp1(&self) -> Option<&CStr> {
        self.ntp1_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_ntp1_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp2(&self) -> Option<&CStr> {
        self.ntp2_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_ntp2_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn domain(&self) -> Option<&CStr> {
        self.domain_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn set_domain(&mut self, value: &CStr) {
        if let Some(callback) = self.set_domain_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn host_name(&self) -> Option<&CStr> {
        self.host_name_callback.map(|callback| {
        unsafe { CStr::from_ptr((callback)(self.context)) }
        })
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn set_host_name(&mut self, value: &CStr) {
        if let Some(callback) = self.set_host_name_callback {
        (callback)(value.as_ptr(), self.context);
        };
    }
}

#[repr(C)]
pub struct Model13StatefulAdapter {
    name: [c_char; 8],
    config_status: CfgSt,
    change_status: u16,
    config_capability: u16,
    i_pv6_config: Cfg,
    control: Ctl,
    ip: [c_char; 40],
    cidr: [c_char; 40],
    gateway: [c_char; 40],
    dns1: [c_char; 40],
    dns2: [c_char; 40],
    ntp1: [c_char; 40],
    ntp2: [c_char; 40],
    domain: [c_char; 24],
    host_name: [c_char; 24],
}

impl ModelAdapter for Model13StatefulAdapter {
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

    /// Config Status
    ///
    /// Configuration status
    fn config_status(&self) -> CfgSt {
        self.config_status
    }

    /// Change Status
    ///
    /// A configuration change is pending
    fn change_status(&self) -> u16 {
        self.change_status
    }

    /// Config Capability
    ///
    /// Identify capable sources of configuration
    fn config_capability(&self) -> u16 {
        self.config_capability
    }

    /// IPv6 Config
    ///
    /// Configuration method used.
    fn i_pv6_config(&self) -> Cfg {
        self.i_pv6_config
    }

    /// IPv6 Config
    ///
    /// Configuration method used.
    fn set_i_pv6_config(&mut self, value: Cfg) {
        self.i_pv6_config = value;
    }

    /// Control
    ///
    /// Configure use of services
    fn control(&self) -> Ctl {
        self.control
    }

    /// Control
    ///
    /// Configure use of services
    fn set_control(&mut self, value: Ctl) {
        self.control = value;
    }

    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn ip(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ip.as_ptr()) }
    }

    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ip(&mut self, value: &CStr) {
        for (dest, src) in self.ip.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn cidr(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.cidr.as_ptr()) }
        )
    }

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn set_cidr(&mut self, value: &CStr) {
        for (dest, src) in self.cidr.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn gateway(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.gateway.as_ptr()) }
        )
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_gateway(&mut self, value: &CStr) {
        for (dest, src) in self.gateway.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns1(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.dns1.as_ptr()) }
        )
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns1(&mut self, value: &CStr) {
        for (dest, src) in self.dns1.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns2(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.dns2.as_ptr()) }
        )
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns2(&mut self, value: &CStr) {
        for (dest, src) in self.dns2.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp1(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.ntp1.as_ptr()) }
        )
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp1(&mut self, value: &CStr) {
        for (dest, src) in self.ntp1.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp2(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.ntp2.as_ptr()) }
        )
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp2(&mut self, value: &CStr) {
        for (dest, src) in self.ntp2.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn domain(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.domain.as_ptr()) }
        )
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn set_domain(&mut self, value: &CStr) {
        for (dest, src) in self.domain.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn host_name(&self) -> Option<&CStr> {
        Some(
        unsafe { CStr::from_ptr(self.host_name.as_ptr()) }
        )
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn set_host_name(&mut self, value: &CStr) {
        for (dest, src) in self.host_name.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }
}