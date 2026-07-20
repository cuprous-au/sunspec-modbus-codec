use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 44;

pub static POINTS: [ReadablePoint; 28] = [
    ReadablePoint {
        reference: PointReference::Static { value: 805 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 42 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::StringIndex },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::ModuleIndex },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::ModuleCellCount },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::ModuleSoC },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::DepthOfDischarge },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::ModuleSoH },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::CycleCount },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::ModuleVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::MaxCellVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::MaxCellVoltageCell },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::MinCellVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::MinCellVoltageCell },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::AverageCellVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::MaxCellTemperature },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::MaxCellTemperatureCell },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::MinCellTemperature },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::MinCellTemperatureCell },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::AverageCellTemperature },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::BalancedCellCount },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::SerialNumber },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::SoCSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::SoHSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::DoDSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::CellVSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 { point: Point::TmpSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    StringIndex,
    ModuleIndex,
    ModuleCellCount,
    ModuleSoC,
    DepthOfDischarge,
    ModuleSoH,
    CycleCount,
    ModuleVoltage,
    MaxCellVoltage,
    MaxCellVoltageCell,
    MinCellVoltage,
    MinCellVoltageCell,
    AverageCellVoltage,
    MaxCellTemperature,
    MaxCellTemperatureCell,
    MinCellTemperature,
    MinCellTemperatureCell,
    AverageCellTemperature,
    BalancedCellCount,
    SerialNumber,
    SoCSf,
    SoHSf,
    DoDSf,
    VSf,
    CellVSf,
    TmpSf,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::StringIndex => serialisation::write_u16(model.string_index(), buffer),
        Point::ModuleIndex => serialisation::write_u16(model.module_index(), buffer),
        Point::ModuleCellCount => serialisation::write_u16(model.module_cell_count(), buffer),
        Point::ModuleSoC => if let Some(value) = model.module_so_c() { serialisation::write_u16(value, buffer); },
        Point::DepthOfDischarge => if let Some(value) = model.depth_of_discharge() { serialisation::write_u16(value, buffer); },
        Point::ModuleSoH => if let Some(value) = model.module_so_h() { serialisation::write_u16(value, buffer); },
        Point::CycleCount => if let Some(value) = model.cycle_count() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::ModuleVoltage => serialisation::write_u16(model.module_voltage(), buffer),
        Point::MaxCellVoltage => serialisation::write_u16(model.max_cell_voltage(), buffer),
        Point::MaxCellVoltageCell => if let Some(value) = model.max_cell_voltage_cell() { serialisation::write_u16(value, buffer); },
        Point::MinCellVoltage => serialisation::write_u16(model.min_cell_voltage(), buffer),
        Point::MinCellVoltageCell => if let Some(value) = model.min_cell_voltage_cell() { serialisation::write_u16(value, buffer); },
        Point::AverageCellVoltage => serialisation::write_u16(model.average_cell_voltage(), buffer),
        Point::MaxCellTemperature => serialisation::write_i16(model.max_cell_temperature(), buffer),
        Point::MaxCellTemperatureCell => if let Some(value) = model.max_cell_temperature_cell() { serialisation::write_u16(value, buffer); },
        Point::MinCellTemperature => serialisation::write_i16(model.min_cell_temperature(), buffer),
        Point::MinCellTemperatureCell => if let Some(value) = model.min_cell_temperature_cell() { serialisation::write_u16(value, buffer); },
        Point::AverageCellTemperature => serialisation::write_i16(model.average_cell_temperature(), buffer),
        Point::BalancedCellCount => if let Some(value) = model.balanced_cell_count() { serialisation::write_u16(value, buffer); },
        Point::SerialNumber => if let Some(value) = model.serial_number() { serialisation::write_string(value, buffer, offset, limit); },
        Point::SoCSf => if let Some(value) = model.so_c_sf() { serialisation::write_u16(value, buffer); },
        Point::SoHSf => if let Some(value) = model.so_h_sf() { serialisation::write_u16(value, buffer); },
        Point::DoDSf => if let Some(value) = model.do_d_sf() { serialisation::write_u16(value, buffer); },
        Point::VSf => serialisation::write_u16(model.v_sf(), buffer),
        Point::CellVSf => serialisation::write_u16(model.cell_v_sf(), buffer),
        Point::TmpSf => serialisation::write_u16(model.tmp_sf(), buffer),
    }
}

pub trait ModelAdapter {
    /// String Index
    ///
    /// Index of the string containing the module.
    ///
    /// Indices are one-based.
    fn string_index(&self) -> u16;

    /// Module Index
    ///
    /// Index of the module within the string.
    ///
    /// Indices are one-based.
    fn module_index(&self) -> u16;

    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    fn module_cell_count(&self) -> u16;

    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    fn module_so_c(&self) -> Option<u16> {
        None
    }

    /// Depth of Discharge
    ///
    /// Depth of discharge for the module.
    ///
    /// Measurement.
    fn depth_of_discharge(&self) -> Option<u16> {
        None
    }

    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    fn module_so_h(&self) -> Option<u16> {
        None
    }

    /// Cycle Count
    ///
    /// Count of cycles executed.
    fn cycle_count(&self) -> Option<u32> {
        None
    }

    /// Module Voltage
    ///
    /// Voltage of the module.
    ///
    /// Measurement.
    fn module_voltage(&self) -> u16;

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> u16;

    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum voltage.
    fn max_cell_voltage_cell(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> u16;

    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum voltage.
    fn min_cell_voltage_cell(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> u16;

    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    ///
    /// Measurement.
    fn max_cell_temperature(&self) -> i16;

    /// Max Cell Temperature Cell
    ///
    /// Cell with the maximum cell temperature.
    fn max_cell_temperature_cell(&self) -> Option<u16> {
        None
    }

    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    ///
    /// Measurement.
    fn min_cell_temperature(&self) -> i16;

    /// Min Cell Temperature Cell
    ///
    /// Cell with the minimum cell temperature.
    fn min_cell_temperature_cell(&self) -> Option<u16> {
        None
    }

    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    ///
    /// Calculation based on measurements.
    fn average_cell_temperature(&self) -> i16;

    /// Balanced Cell Count
    ///
    /// Number of cells currently being balanced in the module.
    fn balanced_cell_count(&self) -> Option<u16> {
        None
    }

    /// Serial Number
    ///
    /// Serial number for the module.
    fn serial_number(&self) -> Option<String<32>> {
        None
    }

    /// Scale factor for module state of charge.
    fn so_c_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for module state of health.
    fn so_h_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for module depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for module voltage.
    fn v_sf(&self) -> u16;

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for module temperature.
    fn tmp_sf(&self) -> u16;
}