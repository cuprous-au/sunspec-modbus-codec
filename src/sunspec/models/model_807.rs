use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 36;

pub static POINTS: [ReadablePoint; 32] = [
    ReadablePoint {
        reference: PointReference::Static { value: 807 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 34 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::StringIndex },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::ModuleCount },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::ConnectedModuleCount },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MaxModuleVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MaxModuleVoltageModule },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MinModuleVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MinModuleVoltageModule },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::AverageModuleVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MaxCellVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MaxCellVoltageModule },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MaxCellVoltageStack },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MinCellVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MinCellVoltageModule },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MinCellVoltageStack },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::AverageCellVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MaxTemperature },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MaxTemperatureModule },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MinTemperature },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::MinTemperatureModule },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::AverageTemperature },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::StringEvent1 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::StringEvent2 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::VendorEventBitfield1 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::VendorEventBitfield2 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::ModVSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::CellVSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::TmpSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::SoCSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model807 { point: Point::OcvSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
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
    StringIndex,
    ModuleCount,
    ConnectedModuleCount,
    MaxModuleVoltage,
    MaxModuleVoltageModule,
    MinModuleVoltage,
    MinModuleVoltageModule,
    AverageModuleVoltage,
    MaxCellVoltage,
    MaxCellVoltageModule,
    MaxCellVoltageStack,
    MinCellVoltage,
    MinCellVoltageModule,
    MinCellVoltageStack,
    AverageCellVoltage,
    MaxTemperature,
    MaxTemperatureModule,
    MinTemperature,
    MinTemperatureModule,
    AverageTemperature,
    StringEvent1,
    StringEvent2,
    VendorEventBitfield1,
    VendorEventBitfield2,
    ModVSf,
    CellVSf,
    TmpSf,
    SoCSf,
    OcvSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::StringIndex => serialisation::write_u16(model.string_index(), buffer),
        Point::ModuleCount => serialisation::write_u16(model.module_count(), buffer),
        Point::ConnectedModuleCount => serialisation::write_u16(model.connected_module_count(), buffer),
        Point::MaxModuleVoltage => serialisation::write_u16(model.max_module_voltage(), buffer),
        Point::MaxModuleVoltageModule => if let Some(value) = model.max_module_voltage_module() { serialisation::write_u16(value, buffer); },
        Point::MinModuleVoltage => serialisation::write_u16(model.min_module_voltage(), buffer),
        Point::MinModuleVoltageModule => if let Some(value) = model.min_module_voltage_module() { serialisation::write_u16(value, buffer); },
        Point::AverageModuleVoltage => serialisation::write_u16(model.average_module_voltage(), buffer),
        Point::MaxCellVoltage => if let Some(value) = model.max_cell_voltage() { serialisation::write_u16(value, buffer); },
        Point::MaxCellVoltageModule => if let Some(value) = model.max_cell_voltage_module() { serialisation::write_u16(value, buffer); },
        Point::MaxCellVoltageStack => if let Some(value) = model.max_cell_voltage_stack() { serialisation::write_u16(value, buffer); },
        Point::MinCellVoltage => if let Some(value) = model.min_cell_voltage() { serialisation::write_u16(value, buffer); },
        Point::MinCellVoltageModule => if let Some(value) = model.min_cell_voltage_module() { serialisation::write_u16(value, buffer); },
        Point::MinCellVoltageStack => if let Some(value) = model.min_cell_voltage_stack() { serialisation::write_u16(value, buffer); },
        Point::AverageCellVoltage => if let Some(value) = model.average_cell_voltage() { serialisation::write_u16(value, buffer); },
        Point::MaxTemperature => serialisation::write_i16(model.max_temperature(), buffer),
        Point::MaxTemperatureModule => if let Some(value) = model.max_temperature_module() { serialisation::write_u16(value, buffer); },
        Point::MinTemperature => serialisation::write_i16(model.min_temperature(), buffer),
        Point::MinTemperatureModule => if let Some(value) = model.min_temperature_module() { serialisation::write_u16(value, buffer); },
        Point::AverageTemperature => serialisation::write_i16(model.average_temperature(), buffer),
        Point::StringEvent1 => serialisation::write_u32(model.string_event_1(), buffer, offset, limit),
        Point::StringEvent2 => serialisation::write_u32(model.string_event_2(), buffer, offset, limit),
        Point::VendorEventBitfield1 => serialisation::write_u32(model.vendor_event_bitfield_1(), buffer, offset, limit),
        Point::VendorEventBitfield2 => serialisation::write_u32(model.vendor_event_bitfield_2(), buffer, offset, limit),
        Point::ModVSf => serialisation::write_u16(model.mod_v_sf(), buffer),
        Point::CellVSf => serialisation::write_u16(model.cell_v_sf(), buffer),
        Point::TmpSf => serialisation::write_u16(model.tmp_sf(), buffer),
        Point::SoCSf => serialisation::write_u16(model.so_c_sf(), buffer),
        Point::OcvSf => serialisation::write_u16(model.ocv_sf(), buffer),
    }
}

pub trait ModelAdapter {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Indices are one-based.
    fn string_index(&self) -> u16;

    /// Module Count
    ///
    /// Number of modules in this string.
    fn module_count(&self) -> u16;

    /// Connected Module Count
    ///
    /// Number of electrically connected modules in this string.
    fn connected_module_count(&self) -> u16;

    /// Max Module Voltage
    ///
    /// Maximum voltage for all modules in the string.
    ///
    /// Measurement.
    fn max_module_voltage(&self) -> u16;

    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    fn max_module_voltage_module(&self) -> Option<u16> {
        None
    }

    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Measurement.
    fn min_module_voltage(&self) -> u16;

    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    fn min_module_voltage_module(&self) -> Option<u16> {
        None
    }

    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_module_voltage(&self) -> u16;

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with the maximum voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage Stack
    ///
    /// Stack containing the cell with the maximum voltage.
    fn max_cell_voltage_stack(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with the minimum voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage Stack
    ///
    /// Stack containing the cell with the minimum voltage.
    fn min_cell_voltage_stack(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    fn max_temperature(&self) -> i16;

    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    fn max_temperature_module(&self) -> Option<u16> {
        None
    }

    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    fn min_temperature(&self) -> i16;

    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    fn min_temperature_module(&self) -> Option<u16> {
        None
    }

    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_temperature(&self) -> i16;

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn string_event_1(&self) -> u32;

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    fn string_event_2(&self) -> u32;

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> u32;

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> u32;

    fn mod_v_sf(&self) -> u16;

    /// Scale factor for voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for temperature.
    fn tmp_sf(&self) -> u16;

    /// Scale factor for state of charge.
    fn so_c_sf(&self) -> u16;

    /// Scale factor for open circuit voltage.
    fn ocv_sf(&self) -> u16;
}