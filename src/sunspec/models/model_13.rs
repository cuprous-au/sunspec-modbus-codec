use heapless::String;
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
        Point::Name => if let Some(value) = model.name() { serialisation::write_string(value, buffer, offset, limit); },
        Point::ConfigStatus => serialisation::write_u16(model.config_status() as u16, buffer),
        Point::ChangeStatus => serialisation::write_u16(model.change_status(), buffer),
        Point::ConfigCapability => serialisation::write_u16(model.config_capability(), buffer),
        Point::IPv6Config => serialisation::write_u16(model.i_pv6_config() as u16, buffer),
        Point::Control => serialisation::write_u16(model.control() as u16, buffer),
        Point::Ip => serialisation::write_string(model.ip(), buffer, offset, limit),
        Point::Cidr => if let Some(value) = model.cidr() { serialisation::write_string(value, buffer, offset, limit); },
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
    fn ip(&self) -> String<40>;

    /// IP
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ip(&mut self, value: String<40>);

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn cidr(&self) -> Option<String<40>> {
        None
    }

    /// CIDR
    ///
    /// Classless Inter-Domain Routing Number
    fn set_cidr(&mut self, value: String<40>) {
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn gateway(&self) -> Option<String<40>> {
        None
    }

    /// Gateway
    ///
    /// IPv6 numeric address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_gateway(&mut self, value: String<40>) {
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns1(&self) -> Option<String<40>> {
        None
    }

    /// DNS1
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns1(&mut self, value: String<40>) {
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn dns2(&self) -> Option<String<40>> {
        None
    }

    /// DNS2
    ///
    /// IPv6 numeric DNS address as a dotted string xxxx.xxxx.xxxx.xxxx
    fn set_dns2(&mut self, value: String<40>) {
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp1(&self) -> Option<String<40>> {
        None
    }

    /// NTP1
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp1(&mut self, value: String<40>) {
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn ntp2(&self) -> Option<String<40>> {
        None
    }

    /// NTP2
    ///
    /// IPv6 numeric NTP address as a name or dotted string xxxx.xxxx.xxxx.xxxx
    fn set_ntp2(&mut self, value: String<40>) {
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