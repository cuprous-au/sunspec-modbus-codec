use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 20;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 714 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 18 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::PortAlarms },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::NumberOfPorts },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::DcCurrent },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::DcPower },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::DcEnergyInjected },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::DcEnergyAbsorbed },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::DcCurrentScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::DcVoltageScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::DcPowerScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::DcEnergyScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 { point: Point::TemperatureScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    PortAlarms,
    NumberOfPorts,
    DcCurrent,
    DcPower,
    DcEnergyInjected,
    DcEnergyAbsorbed,
    DcCurrentScaleFactor,
    DcVoltageScaleFactor,
    DcPowerScaleFactor,
    DcEnergyScaleFactor,
    TemperatureScaleFactor,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::PortAlarms => if let Some(value) = model.port_alarms() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::NumberOfPorts => if let Some(value) = model.number_of_ports() { serialisation::write_u16(value, buffer); },
        Point::DcCurrent => if let Some(value) = model.dc_current() { serialisation::write_i16(value, buffer); },
        Point::DcPower => if let Some(value) = model.dc_power() { serialisation::write_i16(value, buffer); },
        Point::DcEnergyInjected => if let Some(value) = model.dc_energy_injected() { serialisation::write_u64(value, buffer, offset, limit); },
        Point::DcEnergyAbsorbed => if let Some(value) = model.dc_energy_absorbed() { serialisation::write_u64(value, buffer, offset, limit); },
        Point::DcCurrentScaleFactor => if let Some(value) = model.dc_current_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::DcVoltageScaleFactor => if let Some(value) = model.dc_voltage_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::DcPowerScaleFactor => if let Some(value) = model.dc_power_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::DcEnergyScaleFactor => if let Some(value) = model.dc_energy_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::TemperatureScaleFactor => if let Some(value) = model.temperature_scale_factor() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    fn port_alarms(&self) -> Option<u32> {
        None
    }

    /// Number Of Ports
    ///
    /// Number of DC ports.
    fn number_of_ports(&self) -> Option<u16> {
        None
    }

    /// DC Current
    ///
    /// Total DC current for all ports.
    fn dc_current(&self) -> Option<i16> {
        None
    }

    /// DC Power
    ///
    /// Total DC power for all ports.
    fn dc_power(&self) -> Option<i16> {
        None
    }

    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for all ports.
    fn dc_energy_injected(&self) -> Option<u64> {
        None
    }

    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    fn dc_energy_absorbed(&self) -> Option<u64> {
        None
    }

    /// DC Current Scale Factor
    ///
    /// DC current scale factor.
    fn dc_current_scale_factor(&self) -> Option<u16> {
        None
    }

    /// DC Voltage Scale Factor
    ///
    /// DC voltage scale factor.
    fn dc_voltage_scale_factor(&self) -> Option<u16> {
        None
    }

    /// DC Power Scale Factor
    ///
    /// DC power scale factor.
    fn dc_power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// DC Energy Scale Factor
    ///
    /// DC energy scale factor.
    fn dc_energy_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Temperature Scale Factor
    ///
    /// Temperature Scale Factor.
    fn temperature_scale_factor(&self) -> Option<u16> {
        None
    }
}