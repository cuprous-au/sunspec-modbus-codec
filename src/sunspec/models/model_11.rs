use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 15;

pub static POINTS: [ReadablePoint; 9] = [
    ReadablePoint {
        reference: PointReference::Static { value: 11 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 13 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::EthernetLinkSpeed },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::InterfaceStatusFlags },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::LinkState },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::Mac },
        size: 4,
        data_type: PointType::Eui48,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::Name },
        size: 4,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::Control },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model11 { point: Point::ForcedSpeed },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    EthernetLinkSpeed,
    InterfaceStatusFlags,
    LinkState,
    Mac,
    Name,
    Control,
    ForcedSpeed,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::EthernetLinkSpeed => serialisation::write_u16(model.ethernet_link_speed(), buffer),
        Point::InterfaceStatusFlags => serialisation::write_u16(model.interface_status_flags(), buffer),
        Point::LinkState => serialisation::write_u16(model.link_state() as u16, buffer),
        Point::Mac => if let Some(value) = model.mac() { serialisation::write_eui48(value, buffer, offset, limit); },
        Point::Name => if let Some(value) = model.name() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Control => if let Some(value) = model.control() { serialisation::write_u16(value, buffer); },
        Point::ForcedSpeed => if let Some(value) = model.forced_speed() { serialisation::write_u16(value, buffer); },
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
    fn mac(&self) -> Option<[u8; 6]> {
        None
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn name(&self) -> Option<String<8>> {
        None
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn set_name(&mut self, value: String<8>) {
    }

    /// Control
    ///
    /// Control flags
    fn control(&self) -> Option<u16> {
        None
    }

    /// Control
    ///
    /// Control flags
    fn set_control(&mut self, value: u16) {
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn forced_speed(&self) -> Option<u16> {
        None
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn set_forced_speed(&mut self, value: u16) {
    }
}

pub enum St {
    Unknown = 0,
    Enabled = 1,
    Disabled = 2,
    Testing = 3,
}