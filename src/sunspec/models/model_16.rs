use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

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
        reference: PointReference::Model16 { point: Point::Config },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model16 { point: Point::Control },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 { point: Point::Address },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 { point: Point::Netmask },
        size: 8,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model16 { point: Point::Gateway },
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
        reference: PointReference::Model16 { point: Point::LinkControl },
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Name => if let Some(value) = model.name() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Config => serialisation::write_u16(model.config() as u16, buffer),
        Point::Control => serialisation::write_u16(model.control(), buffer),
        Point::Address => serialisation::write_string(model.address(), buffer, offset, limit),
        Point::Netmask => serialisation::write_string(model.netmask(), buffer, offset, limit),
        Point::Gateway => if let Some(value) = model.gateway() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Dns1 => if let Some(value) = model.dns1() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Dns2 => if let Some(value) = model.dns2() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Mac => if let Some(value) = model.mac() { serialisation::write_eui48(value, buffer, offset, limit); },
        Point::LinkControl => if let Some(value) = model.link_control() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// Name
    ///
    /// Interface name. (8 chars)
    fn name(&self) -> Option<String<8>> {
        None
    }

    /// Name
    ///
    /// Interface name. (8 chars)
    fn set_name(&mut self, value: String<8>) {
    }

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
    fn address(&self) -> String<16>;

    /// Address
    ///
    /// IP address
    fn set_address(&mut self, value: String<16>);

    /// Netmask
    ///
    /// Netmask
    fn netmask(&self) -> String<16>;

    /// Netmask
    ///
    /// Netmask
    fn set_netmask(&mut self, value: String<16>);

    /// Gateway
    ///
    /// Gateway IP address
    fn gateway(&self) -> Option<String<16>> {
        None
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn set_gateway(&mut self, value: String<16>) {
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn dns1(&self) -> Option<String<16>> {
        None
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn set_dns1(&mut self, value: String<16>) {
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn dns2(&self) -> Option<String<16>> {
        None
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn set_dns2(&mut self, value: String<16>) {
    }

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<[u8; 6]> {
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
    fn set_link_control(&mut self, value: u16) {
    }
}

pub enum Cfg {
    Static = 0,
    Dhcp = 1,
}