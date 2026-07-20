use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

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
        reference: PointReference::Model12 { point: Point::ConfigStatus },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::ChangeStatus },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::ConfigCapability },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::IPv4Config },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::Control },
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
        reference: PointReference::Model12 { point: Point::Netmask },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::Gateway },
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
        reference: PointReference::Model12 { point: Point::Domain },
        size: 12,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model12 { point: Point::HostName },
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Name => if let Some(value) = model.name() { serialisation::write_string(value, buffer, offset, limit); },
        Point::ConfigStatus => serialisation::write_u16(model.config_status() as u16, buffer),
        Point::ChangeStatus => serialisation::write_u16(model.change_status(), buffer),
        Point::ConfigCapability => serialisation::write_u16(model.config_capability(), buffer),
        Point::IPv4Config => serialisation::write_u16(model.i_pv4_config() as u16, buffer),
        Point::Control => serialisation::write_u16(model.control() as u16, buffer),
        Point::Ip => serialisation::write_string(model.ip(), buffer, offset, limit),
        Point::Netmask => serialisation::write_string(model.netmask(), buffer, offset, limit),
        Point::Gateway => if let Some(value) = model.gateway() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Dns1 => if let Some(value) = model.dns1() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Dns2 => if let Some(value) = model.dns2() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Ntp1 => if let Some(value) = model.ntp1() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Ntp2 => if let Some(value) = model.ntp2() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Domain => if let Some(value) = model.domain() { serialisation::write_string(value, buffer, offset, limit); },
        Point::HostName => if let Some(value) = model.host_name() { serialisation::write_string(value, buffer, offset, limit); },
    }
}

pub trait ModelAdapter {
    /// Name
    ///
    /// Interface name
    fn name(&self) -> Option<String<8>> {
        None
    }

    /// Name
    ///
    /// Interface name
    fn set_name(&mut self, value: String<8>) {
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
    fn ip(&self) -> String<16>;

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn set_ip(&mut self, value: String<16>);

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn netmask(&self) -> String<16>;

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn set_netmask(&mut self, value: String<16>);

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn gateway(&self) -> Option<String<16>> {
        None
    }

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn set_gateway(&mut self, value: String<16>) {
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns1(&self) -> Option<String<16>> {
        None
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns1(&mut self, value: String<16>) {
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns2(&self) -> Option<String<16>> {
        None
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns2(&mut self, value: String<16>) {
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp1(&self) -> Option<String<24>> {
        None
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp1(&mut self, value: String<24>) {
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp2(&self) -> Option<String<24>> {
        None
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp2(&mut self, value: String<24>) {
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn domain(&self) -> Option<String<24>> {
        None
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn set_domain(&mut self, value: String<24>) {
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn host_name(&self) -> Option<String<24>> {
        None
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn set_host_name(&mut self, value: String<24>) {
    }
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