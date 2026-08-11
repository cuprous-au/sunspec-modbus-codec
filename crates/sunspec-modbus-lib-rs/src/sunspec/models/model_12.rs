use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 100;

static POINTS: [PointDetails<()>; 18] = [
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
        point: |()| Point::Name,
        size: 4,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::ConfigStatus,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::ChangeStatus,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::ConfigCapability,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::IPv4Config,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::Control,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::Ip,
        size: 8,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::Netmask,
        size: 8,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::Gateway,
        size: 8,
        start_address: 27,
    },
    PointDetails {
        point: |()| Point::Dns1,
        size: 8,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::Dns2,
        size: 8,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::Ntp1,
        size: 12,
        start_address: 51,
    },
    PointDetails {
        point: |()| Point::Ntp2,
        size: 12,
        start_address: 63,
    },
    PointDetails {
        point: |()| Point::Domain,
        size: 12,
        start_address: 75,
    },
    PointDetails {
        point: |()| Point::HostName,
        size: 12,
        start_address: 87,
    },
    PointDetails {
        point: |()| Point::Pad,
        size: 1,
        start_address: 99,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
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
    Pad,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    100
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
            buffer::write_u16(12, buffer);
        }
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
        Point::ConfigStatus => {
            buffer::write_u16(model.config_status() as u16, buffer);
        }
        Point::ChangeStatus => {
            buffer::write_u16(model.change_status(), buffer);
        }
        Point::ConfigCapability => {
            buffer::write_u16(model.config_capability(), buffer);
        }
        Point::IPv4Config => {
            buffer::write_u16(model.i_pv4_config() as u16, buffer);
        }
        Point::Control => {
            buffer::write_u16(model.control() as u16, buffer);
        }
        Point::Ip => {
            buffer::write_string(model.ip(), buffer, offset, limit);
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
        Point::Ntp1 => {
            if let Some(value) = model.ntp1() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Ntp2 => {
            if let Some(value) = model.ntp2() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Domain => {
            if let Some(value) = model.domain() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::HostName => {
            if let Some(value) = model.host_name() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Pad => {
            buffer::write_u16(0, buffer);
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
pub enum CfgSt {
    NotConfigured = 0,
    ValidSetting = 1,
    ValidHw = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Ctl {
    EnableDns = 0,
    EnableNtp = 1,
}

#[repr(C)]
pub struct Model12CallbackAdapter {
    context: *mut c_void,
    name_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_name_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    config_status_callback: extern "C" fn(*const c_void) -> CfgSt,
    change_status_callback: extern "C" fn(*const c_void) -> u16,
    config_capability_callback: extern "C" fn(*const c_void) -> u16,
    i_pv4_config_callback: extern "C" fn(*const c_void) -> Cfg,
    set_i_pv4_config_callback: extern "C" fn(Cfg, *mut c_void),
    control_callback: extern "C" fn(*const c_void) -> Ctl,
    set_control_callback: extern "C" fn(Ctl, *mut c_void),
    ip_callback: extern "C" fn(*const c_void) -> *const c_char,
    set_ip_callback: extern "C" fn(*const c_char, *mut c_void),
    netmask_callback: extern "C" fn(*const c_void) -> *const c_char,
    set_netmask_callback: extern "C" fn(*const c_char, *mut c_void),
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

impl ModelAdapter for Model12CallbackAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        self.name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
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

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn i_pv4_config(&self) -> Cfg {
        (self.i_pv4_config_callback)(self.context)
    }

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn set_i_pv4_config(&mut self, value: Cfg) {
        (self.set_i_pv4_config_callback)(value, self.context);
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
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn ip(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.ip_callback)(self.context)) }
    }

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn set_ip(&mut self, value: &CStr) {
        (self.set_ip_callback)(value.as_ptr(), self.context);
    }

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn netmask(&self) -> &CStr {
        unsafe { CStr::from_ptr((self.netmask_callback)(self.context)) }
    }

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn set_netmask(&mut self, value: &CStr) {
        (self.set_netmask_callback)(value.as_ptr(), self.context);
    }

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn gateway(&self) -> Option<&CStr> {
        self.gateway_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn set_gateway(&mut self, value: &CStr) {
        if let Some(callback) = self.set_gateway_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns1(&self) -> Option<&CStr> {
        self.dns1_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns1_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns2(&self) -> Option<&CStr> {
        self.dns2_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_dns2_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp1(&self) -> Option<&CStr> {
        self.ntp1_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp1(&mut self, value: &CStr) {
        if let Some(callback) = self.set_ntp1_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp2(&self) -> Option<&CStr> {
        self.ntp2_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp2(&mut self, value: &CStr) {
        if let Some(callback) = self.set_ntp2_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn domain(&self) -> Option<&CStr> {
        self.domain_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
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
        self.host_name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
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
pub struct Model12StatefulAdapter {
    name: [c_char; 8],
    config_status: CfgSt,
    change_status: u16,
    config_capability: u16,
    i_pv4_config: Cfg,
    control: Ctl,
    ip: [c_char; 16],
    netmask: [c_char; 16],
    gateway: [c_char; 16],
    dns1: [c_char; 16],
    dns2: [c_char; 16],
    ntp1: [c_char; 24],
    ntp2: [c_char; 24],
    domain: [c_char; 24],
    host_name: [c_char; 24],
}

impl ModelAdapter for Model12StatefulAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.name.as_ptr()) })
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

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn i_pv4_config(&self) -> Cfg {
        self.i_pv4_config
    }

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn set_i_pv4_config(&mut self, value: Cfg) {
        self.i_pv4_config = value;
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
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn ip(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.ip.as_ptr()) }
    }

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn set_ip(&mut self, value: &CStr) {
        for (dest, src) in self.ip.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn netmask(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.netmask.as_ptr()) }
    }

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
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
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn gateway(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.gateway.as_ptr()) })
    }

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
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
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns1(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.dns1.as_ptr()) })
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns1(&mut self, value: &CStr) {
        for (dest, src) in self.dns1.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns2(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.dns2.as_ptr()) })
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns2(&mut self, value: &CStr) {
        for (dest, src) in self.dns2.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp1(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.ntp1.as_ptr()) })
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp1(&mut self, value: &CStr) {
        for (dest, src) in self.ntp1.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp2(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.ntp2.as_ptr()) })
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp2(&mut self, value: &CStr) {
        for (dest, src) in self.ntp2.iter_mut().zip(value.to_bytes_with_nul().iter()) {
            *dest = *src as c_char;
        }
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn domain(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.domain.as_ptr()) })
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
        Some(unsafe { CStr::from_ptr(self.host_name.as_ptr()) })
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn set_host_name(&mut self, value: &CStr) {
        for (dest, src) in self
            .host_name
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }
}
