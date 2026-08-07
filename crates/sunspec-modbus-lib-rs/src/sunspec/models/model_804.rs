use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 48;

pub static POINTS: [ReadablePoint; 41] = [
    ReadablePoint {
        reference: PointReference::Static { value: 804 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringIndex,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::ModuleCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringStatus,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::ConnectionFailureReason,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringCellBalancingCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringStateOfCharge,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringDepthOfDischarge,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringCycleCount,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringStateOfHealth,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::MaxCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::MaxCellVoltageModule,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::MinCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::MinCellVoltageModule,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::AverageCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::MaxModuleTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::MaxModuleTemperatureModule,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::MinModuleTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::MinModuleTemperatureModule,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::AverageModuleTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::ContactorStatus,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringEvent1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::StringEvent2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::VendorEventBitfield1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::VendorEventBitfield2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::EnableDisableString,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::ConnectDisconnectString,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::SoCSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::SoHSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::DoDSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::CellVSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model804 {
            point: Point::ModTmpSf,
        },
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
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
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
    ModelLength,
    StringIndex,
    ModuleCount,
    StringStatus,
    ConnectionFailureReason,
    StringCellBalancingCount,
    StringStateOfCharge,
    StringDepthOfDischarge,
    StringCycleCount,
    StringStateOfHealth,
    StringCurrent,
    StringVoltage,
    MaxCellVoltage,
    MaxCellVoltageModule,
    MinCellVoltage,
    MinCellVoltageModule,
    AverageCellVoltage,
    MaxModuleTemperature,
    MaxModuleTemperatureModule,
    MinModuleTemperature,
    MinModuleTemperatureModule,
    AverageModuleTemperature,
    ContactorStatus,
    StringEvent1,
    StringEvent2,
    VendorEventBitfield1,
    VendorEventBitfield2,
    EnableDisableString,
    ConnectDisconnectString,
    SoCSf,
    SoHSf,
    DoDSf,
    ASf,
    VSf,
    CellVSf,
    ModTmpSf,
    LithiumIonStringModuleModuleCellCount {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleModuleSoC {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleModuleSoH {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleMaxCellVoltage {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleMaxCellVoltageCell {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleMinCellVoltage {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleMinCellVoltageCell {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleAverageCellVoltage {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleMaxCellTemperature {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleMaxCellTemperatureCell {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleMinCellTemperature {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleMinCellTemperatureCell {
        lithium_ion_string_module_index: u16,
    },
    LithiumIonStringModuleAverageCellTemperature {
        lithium_ion_string_module_index: u16,
    },
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    48 + model.module_count() * (16)
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::StringIndex => {
            buffer::write_u16(model.string_index(), buffer);
        }
        Point::ModuleCount => {
            buffer::write_u16(model.module_count(), buffer);
        }
        Point::StringStatus => {
            buffer::write_u32(model.string_status(), buffer, offset, limit);
        }
        Point::ConnectionFailureReason => {
            if let Some(value) = model.connection_failure_reason() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringCellBalancingCount => {
            if let Some(value) = model.string_cell_balancing_count() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringStateOfCharge => {
            buffer::write_u16(model.string_state_of_charge(), buffer);
        }
        Point::StringDepthOfDischarge => {
            if let Some(value) = model.string_depth_of_discharge() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringCycleCount => {
            if let Some(value) = model.string_cycle_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringStateOfHealth => {
            if let Some(value) = model.string_state_of_health() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringCurrent => {
            buffer::write_i16(model.string_current(), buffer);
        }
        Point::StringVoltage => {
            if let Some(value) = model.string_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxCellVoltage => {
            buffer::write_u16(model.max_cell_voltage(), buffer);
        }
        Point::MaxCellVoltageModule => {
            if let Some(value) = model.max_cell_voltage_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellVoltage => {
            buffer::write_u16(model.min_cell_voltage(), buffer);
        }
        Point::MinCellVoltageModule => {
            if let Some(value) = model.min_cell_voltage_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageCellVoltage => {
            buffer::write_u16(model.average_cell_voltage(), buffer);
        }
        Point::MaxModuleTemperature => {
            buffer::write_i16(model.max_module_temperature(), buffer);
        }
        Point::MaxModuleTemperatureModule => {
            buffer::write_u16(model.max_module_temperature_module(), buffer);
        }
        Point::MinModuleTemperature => {
            buffer::write_i16(model.min_module_temperature(), buffer);
        }
        Point::MinModuleTemperatureModule => {
            buffer::write_u16(model.min_module_temperature_module(), buffer);
        }
        Point::AverageModuleTemperature => {
            buffer::write_i16(model.average_module_temperature(), buffer);
        }
        Point::ContactorStatus => {
            if let Some(value) = model.contactor_status() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringEvent1 => {
            buffer::write_u32(model.string_event_1(), buffer, offset, limit);
        }
        Point::StringEvent2 => {
            if let Some(value) = model.string_event_2() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VendorEventBitfield1 => {
            if let Some(value) = model.vendor_event_bitfield_1() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VendorEventBitfield2 => {
            if let Some(value) = model.vendor_event_bitfield_2() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnableDisableString => {
            if let Some(value) = model.enable_disable_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ConnectDisconnectString => {
            if let Some(value) = model.connect_disconnect_string() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SoCSf => {
            buffer::write_u16(model.so_c_sf(), buffer);
        }
        Point::SoHSf => {
            if let Some(value) = model.so_h_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DoDSf => {
            if let Some(value) = model.do_d_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ASf => {
            buffer::write_u16(model.a_sf(), buffer);
        }
        Point::VSf => {
            if let Some(value) = model.v_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CellVSf => {
            buffer::write_u16(model.cell_v_sf(), buffer);
        }
        Point::ModTmpSf => {
            buffer::write_u16(model.mod_tmp_sf(), buffer);
        }
        Point::LithiumIonStringModuleModuleCellCount {
            lithium_ion_string_module_index,
        } => {
            buffer::write_u16(
                model.lithium_ion_string_module_module_cell_count(*lithium_ion_string_module_index),
                buffer,
            );
        }
        Point::LithiumIonStringModuleModuleSoC {
            lithium_ion_string_module_index,
        } => {
            if let Some(value) =
                model.lithium_ion_string_module_module_so_c(*lithium_ion_string_module_index)
            {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LithiumIonStringModuleModuleSoH {
            lithium_ion_string_module_index,
        } => {
            if let Some(value) =
                model.lithium_ion_string_module_module_so_h(*lithium_ion_string_module_index)
            {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LithiumIonStringModuleMaxCellVoltage {
            lithium_ion_string_module_index,
        } => {
            buffer::write_u16(
                model.lithium_ion_string_module_max_cell_voltage(*lithium_ion_string_module_index),
                buffer,
            );
        }
        Point::LithiumIonStringModuleMaxCellVoltageCell {
            lithium_ion_string_module_index,
        } => {
            if let Some(value) = model
                .lithium_ion_string_module_max_cell_voltage_cell(*lithium_ion_string_module_index)
            {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LithiumIonStringModuleMinCellVoltage {
            lithium_ion_string_module_index,
        } => {
            buffer::write_u16(
                model.lithium_ion_string_module_min_cell_voltage(*lithium_ion_string_module_index),
                buffer,
            );
        }
        Point::LithiumIonStringModuleMinCellVoltageCell {
            lithium_ion_string_module_index,
        } => {
            if let Some(value) = model
                .lithium_ion_string_module_min_cell_voltage_cell(*lithium_ion_string_module_index)
            {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LithiumIonStringModuleAverageCellVoltage {
            lithium_ion_string_module_index,
        } => {
            buffer::write_u16(
                model.lithium_ion_string_module_average_cell_voltage(
                    *lithium_ion_string_module_index,
                ),
                buffer,
            );
        }
        Point::LithiumIonStringModuleMaxCellTemperature {
            lithium_ion_string_module_index,
        } => {
            buffer::write_i16(
                model.lithium_ion_string_module_max_cell_temperature(
                    *lithium_ion_string_module_index,
                ),
                buffer,
            );
        }
        Point::LithiumIonStringModuleMaxCellTemperatureCell {
            lithium_ion_string_module_index,
        } => {
            if let Some(value) = model.lithium_ion_string_module_max_cell_temperature_cell(
                *lithium_ion_string_module_index,
            ) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LithiumIonStringModuleMinCellTemperature {
            lithium_ion_string_module_index,
        } => {
            buffer::write_i16(
                model.lithium_ion_string_module_min_cell_temperature(
                    *lithium_ion_string_module_index,
                ),
                buffer,
            );
        }
        Point::LithiumIonStringModuleMinCellTemperatureCell {
            lithium_ion_string_module_index,
        } => {
            if let Some(value) = model.lithium_ion_string_module_min_cell_temperature_cell(
                *lithium_ion_string_module_index,
            ) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LithiumIonStringModuleAverageCellTemperature {
            lithium_ion_string_module_index,
        } => {
            buffer::write_i16(
                model.lithium_ion_string_module_average_cell_temperature(
                    *lithium_ion_string_module_index,
                ),
                buffer,
            );
        }
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
    /// Count of modules in the string.
    fn module_count(&self) -> u16;

    /// String Status
    ///
    /// Current status of the string.
    fn string_status(&self) -> u32;

    /// Connection Failure Reason
    fn connection_failure_reason(&self) -> Option<ConFail> {
        None
    }

    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    fn string_cell_balancing_count(&self) -> Option<u16> {
        None
    }

    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn string_state_of_charge(&self) -> u16;

    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Measurement.
    fn string_depth_of_discharge(&self) -> Option<u16> {
        None
    }

    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    fn string_cycle_count(&self) -> Option<u32> {
        None
    }

    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Measurement.
    fn string_state_of_health(&self) -> Option<u16> {
        None
    }

    /// String Current
    ///
    /// String current measurement.
    ///
    /// Measurement.
    fn string_current(&self) -> i16;

    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Measurement.
    fn string_voltage(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> u16;

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> u16;

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> u16;

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Measurement.
    fn max_module_temperature(&self) -> i16;

    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    fn max_module_temperature_module(&self) -> u16;

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Measurement.
    fn min_module_temperature(&self) -> i16;

    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    fn min_module_temperature_module(&self) -> u16;

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_module_temperature(&self) -> i16;

    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    fn contactor_status(&self) -> Option<u32> {
        None
    }

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn string_event_1(&self) -> u32;

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    ///
    /// Reserved for future use.
    fn string_event_2(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> Option<u32> {
        None
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn enable_disable_string(&self) -> Option<u16> {
        None
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn set_enable_disable_string(&mut self, value: u16) {}

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn connect_disconnect_string(&self) -> Option<SetCon> {
        None
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn set_connect_disconnect_string(&mut self, value: SetCon) {}

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16;

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for string depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for string current.
    fn a_sf(&self) -> u16;

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for module temperature.
    fn mod_tmp_sf(&self) -> u16;

    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    fn lithium_ion_string_module_module_cell_count(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16;

    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    fn lithium_ion_string_module_module_so_c(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        None
    }

    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    fn lithium_ion_string_module_module_so_h(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        None
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    fn lithium_ion_string_module_max_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16;

    /// Max Cell Voltage Cell
    ///
    /// Cell with maximum voltage.
    fn lithium_ion_string_module_max_cell_voltage_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    fn lithium_ion_string_module_min_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16;

    /// Min Cell Voltage Cell
    ///
    /// Cell with minimum voltage.
    fn lithium_ion_string_module_min_cell_voltage_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    fn lithium_ion_string_module_average_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16;

    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    fn lithium_ion_string_module_max_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16;

    /// Max Cell Temperature Cell
    ///
    /// Cell with maximum temperature.
    fn lithium_ion_string_module_max_cell_temperature_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        None
    }

    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    fn lithium_ion_string_module_min_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16;

    /// Min Cell Temperature Cell
    ///
    /// Cell with minimum temperature.
    fn lithium_ion_string_module_min_cell_temperature_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        None
    }

    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    fn lithium_ion_string_module_average_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ConFail {
    NoFailure = 0,
    ButtonPushed = 1,
    StrGroundFault = 2,
    OutsideVoltageRange = 3,
    StringNotEnabled = 4,
    FuseOpen = 5,
    ContactorFailure = 6,
    PrechargeFailure = 7,
    /// See Evt1 for more information.
    StringFault = 8,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum SetCon {
    ConnectString = 1,
    DisconnectString = 2,
}

#[repr(C)]
pub struct Model804CallbackAdapter {
    context: *mut c_void,
    string_index_callback: extern "C" fn(*const c_void) -> u16,
    module_count_callback: extern "C" fn(*const c_void) -> u16,
    string_status_callback: extern "C" fn(*const c_void) -> u32,
    connection_failure_reason_callback: Option<extern "C" fn(*const c_void) -> ConFail>,
    string_cell_balancing_count_callback: Option<extern "C" fn(*const c_void) -> u16>,
    string_state_of_charge_callback: extern "C" fn(*const c_void) -> u16,
    string_depth_of_discharge_callback: Option<extern "C" fn(*const c_void) -> u16>,
    string_cycle_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    string_state_of_health_callback: Option<extern "C" fn(*const c_void) -> u16>,
    string_current_callback: extern "C" fn(*const c_void) -> i16,
    string_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_cell_voltage_callback: extern "C" fn(*const c_void) -> u16,
    max_cell_voltage_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_voltage_callback: extern "C" fn(*const c_void) -> u16,
    min_cell_voltage_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_cell_voltage_callback: extern "C" fn(*const c_void) -> u16,
    max_module_temperature_callback: extern "C" fn(*const c_void) -> i16,
    max_module_temperature_module_callback: extern "C" fn(*const c_void) -> u16,
    min_module_temperature_callback: extern "C" fn(*const c_void) -> i16,
    min_module_temperature_module_callback: extern "C" fn(*const c_void) -> u16,
    average_module_temperature_callback: extern "C" fn(*const c_void) -> i16,
    contactor_status_callback: Option<extern "C" fn(*const c_void) -> u32>,
    string_event_1_callback: extern "C" fn(*const c_void) -> u32,
    string_event_2_callback: Option<extern "C" fn(*const c_void) -> u32>,
    vendor_event_bitfield_1_callback: Option<extern "C" fn(*const c_void) -> u32>,
    vendor_event_bitfield_2_callback: Option<extern "C" fn(*const c_void) -> u32>,
    enable_disable_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_enable_disable_string_callback: Option<extern "C" fn(u16, *mut c_void)>,
    connect_disconnect_string_callback: Option<extern "C" fn(*const c_void) -> SetCon>,
    set_connect_disconnect_string_callback: Option<extern "C" fn(SetCon, *mut c_void)>,
    so_c_sf_callback: extern "C" fn(*const c_void) -> u16,
    so_h_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    do_d_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    a_sf_callback: extern "C" fn(*const c_void) -> u16,
    v_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    cell_v_sf_callback: extern "C" fn(*const c_void) -> u16,
    mod_tmp_sf_callback: extern "C" fn(*const c_void) -> u16,
    lithium_ion_string_module_module_cell_count_callback: extern "C" fn(*const c_void, u16) -> u16,
    lithium_ion_string_module_module_so_c_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    lithium_ion_string_module_module_so_h_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    lithium_ion_string_module_max_cell_voltage_callback: extern "C" fn(*const c_void, u16) -> u16,
    lithium_ion_string_module_max_cell_voltage_cell_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    lithium_ion_string_module_min_cell_voltage_callback: extern "C" fn(*const c_void, u16) -> u16,
    lithium_ion_string_module_min_cell_voltage_cell_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    lithium_ion_string_module_average_cell_voltage_callback:
        extern "C" fn(*const c_void, u16) -> u16,
    lithium_ion_string_module_max_cell_temperature_callback:
        extern "C" fn(*const c_void, u16) -> i16,
    lithium_ion_string_module_max_cell_temperature_cell_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    lithium_ion_string_module_min_cell_temperature_callback:
        extern "C" fn(*const c_void, u16) -> i16,
    lithium_ion_string_module_min_cell_temperature_cell_callback:
        Option<extern "C" fn(*const c_void, u16) -> u16>,
    lithium_ion_string_module_average_cell_temperature_callback:
        extern "C" fn(*const c_void, u16) -> i16,
}

impl ModelAdapter for Model804CallbackAdapter {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Indices are one-based.
    fn string_index(&self) -> u16 {
        (self.string_index_callback)(self.context)
    }

    /// Module Count
    ///
    /// Count of modules in the string.
    fn module_count(&self) -> u16 {
        (self.module_count_callback)(self.context)
    }

    /// String Status
    ///
    /// Current status of the string.
    fn string_status(&self) -> u32 {
        (self.string_status_callback)(self.context)
    }

    /// Connection Failure Reason
    fn connection_failure_reason(&self) -> Option<ConFail> {
        self.connection_failure_reason_callback
            .map(|callback| (callback)(self.context))
    }

    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    fn string_cell_balancing_count(&self) -> Option<u16> {
        self.string_cell_balancing_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn string_state_of_charge(&self) -> u16 {
        (self.string_state_of_charge_callback)(self.context)
    }

    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Measurement.
    fn string_depth_of_discharge(&self) -> Option<u16> {
        self.string_depth_of_discharge_callback
            .map(|callback| (callback)(self.context))
    }

    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    fn string_cycle_count(&self) -> Option<u32> {
        self.string_cycle_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Measurement.
    fn string_state_of_health(&self) -> Option<u16> {
        self.string_state_of_health_callback
            .map(|callback| (callback)(self.context))
    }

    /// String Current
    ///
    /// String current measurement.
    ///
    /// Measurement.
    fn string_current(&self) -> i16 {
        (self.string_current_callback)(self.context)
    }

    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Measurement.
    fn string_voltage(&self) -> Option<u16> {
        self.string_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> u16 {
        (self.max_cell_voltage_callback)(self.context)
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        self.max_cell_voltage_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> u16 {
        (self.min_cell_voltage_callback)(self.context)
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        self.min_cell_voltage_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> u16 {
        (self.average_cell_voltage_callback)(self.context)
    }

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Measurement.
    fn max_module_temperature(&self) -> i16 {
        (self.max_module_temperature_callback)(self.context)
    }

    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    fn max_module_temperature_module(&self) -> u16 {
        (self.max_module_temperature_module_callback)(self.context)
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Measurement.
    fn min_module_temperature(&self) -> i16 {
        (self.min_module_temperature_callback)(self.context)
    }

    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    fn min_module_temperature_module(&self) -> u16 {
        (self.min_module_temperature_module_callback)(self.context)
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_module_temperature(&self) -> i16 {
        (self.average_module_temperature_callback)(self.context)
    }

    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    fn contactor_status(&self) -> Option<u32> {
        self.contactor_status_callback
            .map(|callback| (callback)(self.context))
    }

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn string_event_1(&self) -> u32 {
        (self.string_event_1_callback)(self.context)
    }

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    ///
    /// Reserved for future use.
    fn string_event_2(&self) -> Option<u32> {
        self.string_event_2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> Option<u32> {
        self.vendor_event_bitfield_1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> Option<u32> {
        self.vendor_event_bitfield_2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn enable_disable_string(&self) -> Option<u16> {
        self.enable_disable_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn set_enable_disable_string(&mut self, value: u16) {
        if let Some(callback) = self.set_enable_disable_string_callback {
            (callback)(value, self.context);
        };
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn connect_disconnect_string(&self) -> Option<SetCon> {
        self.connect_disconnect_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn set_connect_disconnect_string(&mut self, value: SetCon) {
        if let Some(callback) = self.set_connect_disconnect_string_callback {
            (callback)(value, self.context);
        };
    }

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16 {
        (self.so_c_sf_callback)(self.context)
    }

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        self.so_h_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for string depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        self.do_d_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for string current.
    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)(self.context)
    }

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        self.v_sf_callback.map(|callback| (callback)(self.context))
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        (self.cell_v_sf_callback)(self.context)
    }

    /// Scale factor for module temperature.
    fn mod_tmp_sf(&self) -> u16 {
        (self.mod_tmp_sf_callback)(self.context)
    }

    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    fn lithium_ion_string_module_module_cell_count(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16 {
        (self.lithium_ion_string_module_module_cell_count_callback)(
            self.context,
            lithium_ion_string_module_index,
        )
    }

    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    fn lithium_ion_string_module_module_so_c(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        self.lithium_ion_string_module_module_so_c_callback
            .map(|callback| (callback)(self.context, lithium_ion_string_module_index))
    }

    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    fn lithium_ion_string_module_module_so_h(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        self.lithium_ion_string_module_module_so_h_callback
            .map(|callback| (callback)(self.context, lithium_ion_string_module_index))
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    fn lithium_ion_string_module_max_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16 {
        (self.lithium_ion_string_module_max_cell_voltage_callback)(
            self.context,
            lithium_ion_string_module_index,
        )
    }

    /// Max Cell Voltage Cell
    ///
    /// Cell with maximum voltage.
    fn lithium_ion_string_module_max_cell_voltage_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        self.lithium_ion_string_module_max_cell_voltage_cell_callback
            .map(|callback| (callback)(self.context, lithium_ion_string_module_index))
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    fn lithium_ion_string_module_min_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16 {
        (self.lithium_ion_string_module_min_cell_voltage_callback)(
            self.context,
            lithium_ion_string_module_index,
        )
    }

    /// Min Cell Voltage Cell
    ///
    /// Cell with minimum voltage.
    fn lithium_ion_string_module_min_cell_voltage_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        self.lithium_ion_string_module_min_cell_voltage_cell_callback
            .map(|callback| (callback)(self.context, lithium_ion_string_module_index))
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    fn lithium_ion_string_module_average_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16 {
        (self.lithium_ion_string_module_average_cell_voltage_callback)(
            self.context,
            lithium_ion_string_module_index,
        )
    }

    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    fn lithium_ion_string_module_max_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16 {
        (self.lithium_ion_string_module_max_cell_temperature_callback)(
            self.context,
            lithium_ion_string_module_index,
        )
    }

    /// Max Cell Temperature Cell
    ///
    /// Cell with maximum temperature.
    fn lithium_ion_string_module_max_cell_temperature_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        self.lithium_ion_string_module_max_cell_temperature_cell_callback
            .map(|callback| (callback)(self.context, lithium_ion_string_module_index))
    }

    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    fn lithium_ion_string_module_min_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16 {
        (self.lithium_ion_string_module_min_cell_temperature_callback)(
            self.context,
            lithium_ion_string_module_index,
        )
    }

    /// Min Cell Temperature Cell
    ///
    /// Cell with minimum temperature.
    fn lithium_ion_string_module_min_cell_temperature_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        self.lithium_ion_string_module_min_cell_temperature_cell_callback
            .map(|callback| (callback)(self.context, lithium_ion_string_module_index))
    }

    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    fn lithium_ion_string_module_average_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16 {
        (self.lithium_ion_string_module_average_cell_temperature_callback)(
            self.context,
            lithium_ion_string_module_index,
        )
    }
}

#[repr(C)]
pub struct Model804StatefulAdapter<const MODULE_COUNT: usize> {
    string_index: u16,
    module_count: u16,
    string_status: u32,
    connection_failure_reason: ConFail,
    string_cell_balancing_count: u16,
    string_state_of_charge: u16,
    string_depth_of_discharge: u16,
    string_cycle_count: u32,
    string_state_of_health: u16,
    string_current: i16,
    string_voltage: u16,
    max_cell_voltage: u16,
    max_cell_voltage_module: u16,
    min_cell_voltage: u16,
    min_cell_voltage_module: u16,
    average_cell_voltage: u16,
    max_module_temperature: i16,
    max_module_temperature_module: u16,
    min_module_temperature: i16,
    min_module_temperature_module: u16,
    average_module_temperature: i16,
    contactor_status: u32,
    string_event_1: u32,
    string_event_2: u32,
    vendor_event_bitfield_1: u32,
    vendor_event_bitfield_2: u32,
    enable_disable_string: u16,
    connect_disconnect_string: SetCon,
    so_c_sf: u16,
    so_h_sf: u16,
    do_d_sf: u16,
    a_sf: u16,
    v_sf: u16,
    cell_v_sf: u16,
    mod_tmp_sf: u16,
    lithium_ion_string_module: [Model804LithiumIonStringModule; MODULE_COUNT],
}

#[repr(C)]
pub struct Model804LithiumIonStringModule {
    lithium_ion_string_module_module_cell_count: u16,
    lithium_ion_string_module_module_so_c: u16,
    lithium_ion_string_module_module_so_h: u16,
    lithium_ion_string_module_max_cell_voltage: u16,
    lithium_ion_string_module_max_cell_voltage_cell: u16,
    lithium_ion_string_module_min_cell_voltage: u16,
    lithium_ion_string_module_min_cell_voltage_cell: u16,
    lithium_ion_string_module_average_cell_voltage: u16,
    lithium_ion_string_module_max_cell_temperature: i16,
    lithium_ion_string_module_max_cell_temperature_cell: u16,
    lithium_ion_string_module_min_cell_temperature: i16,
    lithium_ion_string_module_min_cell_temperature_cell: u16,
    lithium_ion_string_module_average_cell_temperature: i16,
}

impl<const MODULE_COUNT: usize> ModelAdapter for Model804StatefulAdapter<MODULE_COUNT> {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Indices are one-based.
    fn string_index(&self) -> u16 {
        self.string_index
    }

    /// Module Count
    ///
    /// Count of modules in the string.
    fn module_count(&self) -> u16 {
        self.module_count
    }

    /// String Status
    ///
    /// Current status of the string.
    fn string_status(&self) -> u32 {
        self.string_status
    }

    /// Connection Failure Reason
    fn connection_failure_reason(&self) -> Option<ConFail> {
        Some(self.connection_failure_reason)
    }

    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    fn string_cell_balancing_count(&self) -> Option<u16> {
        Some(self.string_cell_balancing_count)
    }

    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn string_state_of_charge(&self) -> u16 {
        self.string_state_of_charge
    }

    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Measurement.
    fn string_depth_of_discharge(&self) -> Option<u16> {
        Some(self.string_depth_of_discharge)
    }

    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    fn string_cycle_count(&self) -> Option<u32> {
        Some(self.string_cycle_count)
    }

    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Measurement.
    fn string_state_of_health(&self) -> Option<u16> {
        Some(self.string_state_of_health)
    }

    /// String Current
    ///
    /// String current measurement.
    ///
    /// Measurement.
    fn string_current(&self) -> i16 {
        self.string_current
    }

    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Measurement.
    fn string_voltage(&self) -> Option<u16> {
        Some(self.string_voltage)
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> u16 {
        self.max_cell_voltage
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        Some(self.max_cell_voltage_module)
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> u16 {
        self.min_cell_voltage
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        Some(self.min_cell_voltage_module)
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> u16 {
        self.average_cell_voltage
    }

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Measurement.
    fn max_module_temperature(&self) -> i16 {
        self.max_module_temperature
    }

    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    fn max_module_temperature_module(&self) -> u16 {
        self.max_module_temperature_module
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Measurement.
    fn min_module_temperature(&self) -> i16 {
        self.min_module_temperature
    }

    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    fn min_module_temperature_module(&self) -> u16 {
        self.min_module_temperature_module
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_module_temperature(&self) -> i16 {
        self.average_module_temperature
    }

    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    fn contactor_status(&self) -> Option<u32> {
        Some(self.contactor_status)
    }

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn string_event_1(&self) -> u32 {
        self.string_event_1
    }

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    ///
    /// Reserved for future use.
    fn string_event_2(&self) -> Option<u32> {
        Some(self.string_event_2)
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> Option<u32> {
        Some(self.vendor_event_bitfield_1)
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> Option<u32> {
        Some(self.vendor_event_bitfield_2)
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn enable_disable_string(&self) -> Option<u16> {
        Some(self.enable_disable_string)
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn set_enable_disable_string(&mut self, value: u16) {
        self.enable_disable_string = value;
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn connect_disconnect_string(&self) -> Option<SetCon> {
        Some(self.connect_disconnect_string)
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn set_connect_disconnect_string(&mut self, value: SetCon) {
        self.connect_disconnect_string = value;
    }

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16 {
        self.so_c_sf
    }

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        Some(self.so_h_sf)
    }

    /// Scale factor for string depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        Some(self.do_d_sf)
    }

    /// Scale factor for string current.
    fn a_sf(&self) -> u16 {
        self.a_sf
    }

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        Some(self.v_sf)
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        self.cell_v_sf
    }

    /// Scale factor for module temperature.
    fn mod_tmp_sf(&self) -> u16 {
        self.mod_tmp_sf
    }

    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    fn lithium_ion_string_module_module_cell_count(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16 {
        self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
            .lithium_ion_string_module_module_cell_count
    }

    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    fn lithium_ion_string_module_module_so_c(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        Some(
            self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
                .lithium_ion_string_module_module_so_c,
        )
    }

    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    fn lithium_ion_string_module_module_so_h(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        Some(
            self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
                .lithium_ion_string_module_module_so_h,
        )
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    fn lithium_ion_string_module_max_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16 {
        self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
            .lithium_ion_string_module_max_cell_voltage
    }

    /// Max Cell Voltage Cell
    ///
    /// Cell with maximum voltage.
    fn lithium_ion_string_module_max_cell_voltage_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        Some(
            self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
                .lithium_ion_string_module_max_cell_voltage_cell,
        )
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    fn lithium_ion_string_module_min_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16 {
        self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
            .lithium_ion_string_module_min_cell_voltage
    }

    /// Min Cell Voltage Cell
    ///
    /// Cell with minimum voltage.
    fn lithium_ion_string_module_min_cell_voltage_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        Some(
            self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
                .lithium_ion_string_module_min_cell_voltage_cell,
        )
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    fn lithium_ion_string_module_average_cell_voltage(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> u16 {
        self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
            .lithium_ion_string_module_average_cell_voltage
    }

    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    fn lithium_ion_string_module_max_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16 {
        self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
            .lithium_ion_string_module_max_cell_temperature
    }

    /// Max Cell Temperature Cell
    ///
    /// Cell with maximum temperature.
    fn lithium_ion_string_module_max_cell_temperature_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        Some(
            self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
                .lithium_ion_string_module_max_cell_temperature_cell,
        )
    }

    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    fn lithium_ion_string_module_min_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16 {
        self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
            .lithium_ion_string_module_min_cell_temperature
    }

    /// Min Cell Temperature Cell
    ///
    /// Cell with minimum temperature.
    fn lithium_ion_string_module_min_cell_temperature_cell(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> Option<u16> {
        Some(
            self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
                .lithium_ion_string_module_min_cell_temperature_cell,
        )
    }

    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    fn lithium_ion_string_module_average_cell_temperature(
        &self,
        lithium_ion_string_module_index: u16,
    ) -> i16 {
        self.lithium_ion_string_module[lithium_ion_string_module_index as usize]
            .lithium_ion_string_module_average_cell_temperature
    }
}
