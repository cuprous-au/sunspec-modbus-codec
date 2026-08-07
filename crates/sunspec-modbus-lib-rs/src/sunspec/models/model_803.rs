use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 28;

pub static POINTS: [ReadablePoint; 28] = [
    ReadablePoint {
        reference: PointReference::Static { value: 803 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::StringCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::ConnectedStringCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MaxModuleTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MaxModuleTemperatureString,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MaxModuleTemperatureModule,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MinModuleTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MinModuleTemperatureString,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MinModuleTemperatureModule,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::AverageModuleTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MaxStringVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MaxStringVoltageString,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MinStringVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MinStringVoltageString,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::AverageStringVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MaxStringCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MaxStringCurrentString,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MinStringCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::MinStringCurrentString,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::AverageStringCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::BatteryCellBalancingCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::CellVSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::ModTmpSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::SoHSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 {
            point: Point::SoCSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model803 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    StringCount,
    ConnectedStringCount,
    MaxModuleTemperature,
    MaxModuleTemperatureString,
    MaxModuleTemperatureModule,
    MinModuleTemperature,
    MinModuleTemperatureString,
    MinModuleTemperatureModule,
    AverageModuleTemperature,
    MaxStringVoltage,
    MaxStringVoltageString,
    MinStringVoltage,
    MinStringVoltageString,
    AverageStringVoltage,
    MaxStringCurrent,
    MaxStringCurrentString,
    MinStringCurrent,
    MinStringCurrentString,
    AverageStringCurrent,
    BatteryCellBalancingCount,
    CellVSf,
    ModTmpSf,
    ASf,
    SoHSf,
    SoCSf,
    VSf,
    StringModuleCount { string_index: u16 },
    StringStringStatus { string_index: u16 },
    StringConnectionFailureReason { string_index: u16 },
    StringStringStateOfCharge { string_index: u16 },
    StringStringStateOfHealth { string_index: u16 },
    StringStringCurrent { string_index: u16 },
    StringMaxCellVoltage { string_index: u16 },
    StringMaxCellVoltageModule { string_index: u16 },
    StringMinCellVoltage { string_index: u16 },
    StringMinCellVoltageModule { string_index: u16 },
    StringAverageCellVoltage { string_index: u16 },
    StringMaxModuleTemperature { string_index: u16 },
    StringMaxModuleTemperatureModule { string_index: u16 },
    StringMinModuleTemperature { string_index: u16 },
    StringMinModuleTemperatureModule { string_index: u16 },
    StringAverageModuleTemperature { string_index: u16 },
    StringDisabledReason { string_index: u16 },
    StringContactorStatus { string_index: u16 },
    StringStringEvent1 { string_index: u16 },
    StringStringEvent2 { string_index: u16 },
    StringVendorStringEventBitfield1 { string_index: u16 },
    StringVendorStringEventBitfield2 { string_index: u16 },
    StringEnableDisableString { string_index: u16 },
    StringConnectDisconnectString { string_index: u16 },
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    28 + model.string_count() * (32)
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
        Point::StringCount => {
            buffer::write_u16(model.string_count(), buffer);
        }
        Point::ConnectedStringCount => {
            buffer::write_u16(model.connected_string_count(), buffer);
        }
        Point::MaxModuleTemperature => {
            buffer::write_i16(model.max_module_temperature(), buffer);
        }
        Point::MaxModuleTemperatureString => {
            if let Some(value) = model.max_module_temperature_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxModuleTemperatureModule => {
            if let Some(value) = model.max_module_temperature_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinModuleTemperature => {
            buffer::write_i16(model.min_module_temperature(), buffer);
        }
        Point::MinModuleTemperatureString => {
            if let Some(value) = model.min_module_temperature_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinModuleTemperatureModule => {
            if let Some(value) = model.min_module_temperature_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageModuleTemperature => {
            if let Some(value) = model.average_module_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxStringVoltage => {
            if let Some(value) = model.max_string_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxStringVoltageString => {
            if let Some(value) = model.max_string_voltage_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinStringVoltage => {
            if let Some(value) = model.min_string_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinStringVoltageString => {
            if let Some(value) = model.min_string_voltage_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageStringVoltage => {
            if let Some(value) = model.average_string_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxStringCurrent => {
            if let Some(value) = model.max_string_current() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxStringCurrentString => {
            if let Some(value) = model.max_string_current_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinStringCurrent => {
            if let Some(value) = model.min_string_current() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinStringCurrentString => {
            if let Some(value) = model.min_string_current_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageStringCurrent => {
            if let Some(value) = model.average_string_current() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::BatteryCellBalancingCount => {
            if let Some(value) = model.battery_cell_balancing_count() {
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
        Point::ASf => {
            buffer::write_u16(model.a_sf(), buffer);
        }
        Point::SoHSf => {
            if let Some(value) = model.so_h_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SoCSf => {
            buffer::write_u16(model.so_c_sf(), buffer);
        }
        Point::VSf => {
            if let Some(value) = model.v_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringModuleCount { string_index } => {
            buffer::write_u16(model.string_module_count(*string_index), buffer);
        }
        Point::StringStringStatus { string_index } => {
            buffer::write_u32(
                model.string_string_status(*string_index),
                buffer,
                offset,
                limit,
            );
        }
        Point::StringConnectionFailureReason { string_index } => {
            if let Some(value) = model.string_connection_failure_reason(*string_index) {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringStringStateOfCharge { string_index } => {
            buffer::write_u16(model.string_string_state_of_charge(*string_index), buffer);
        }
        Point::StringStringStateOfHealth { string_index } => {
            if let Some(value) = model.string_string_state_of_health(*string_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringStringCurrent { string_index } => {
            buffer::write_i16(model.string_string_current(*string_index), buffer);
        }
        Point::StringMaxCellVoltage { string_index } => {
            buffer::write_u16(model.string_max_cell_voltage(*string_index), buffer);
        }
        Point::StringMaxCellVoltageModule { string_index } => {
            if let Some(value) = model.string_max_cell_voltage_module(*string_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringMinCellVoltage { string_index } => {
            buffer::write_u16(model.string_min_cell_voltage(*string_index), buffer);
        }
        Point::StringMinCellVoltageModule { string_index } => {
            if let Some(value) = model.string_min_cell_voltage_module(*string_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringAverageCellVoltage { string_index } => {
            buffer::write_u16(model.string_average_cell_voltage(*string_index), buffer);
        }
        Point::StringMaxModuleTemperature { string_index } => {
            buffer::write_i16(model.string_max_module_temperature(*string_index), buffer);
        }
        Point::StringMaxModuleTemperatureModule { string_index } => {
            if let Some(value) = model.string_max_module_temperature_module(*string_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringMinModuleTemperature { string_index } => {
            buffer::write_i16(model.string_min_module_temperature(*string_index), buffer);
        }
        Point::StringMinModuleTemperatureModule { string_index } => {
            if let Some(value) = model.string_min_module_temperature_module(*string_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringAverageModuleTemperature { string_index } => {
            buffer::write_i16(
                model.string_average_module_temperature(*string_index),
                buffer,
            );
        }
        Point::StringDisabledReason { string_index } => {
            if let Some(value) = model.string_disabled_reason(*string_index) {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringContactorStatus { string_index } => {
            if let Some(value) = model.string_contactor_status(*string_index) {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringStringEvent1 { string_index } => {
            buffer::write_u32(
                model.string_string_event_1(*string_index),
                buffer,
                offset,
                limit,
            );
        }
        Point::StringStringEvent2 { string_index } => {
            if let Some(value) = model.string_string_event_2(*string_index) {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringVendorStringEventBitfield1 { string_index } => {
            if let Some(value) = model.string_vendor_string_event_bitfield_1(*string_index) {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringVendorStringEventBitfield2 { string_index } => {
            if let Some(value) = model.string_vendor_string_event_bitfield_2(*string_index) {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringEnableDisableString { string_index } => {
            if let Some(value) = model.string_enable_disable_string(*string_index) {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringConnectDisconnectString { string_index } => {
            if let Some(value) = model.string_connect_disconnect_string(*string_index) {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// String Count
    ///
    /// Number of strings in the bank.
    fn string_count(&self) -> u16;

    /// Connected String Count
    ///
    /// Number of strings with contactor closed.
    fn connected_string_count(&self) -> u16;

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    ///
    /// Measurement.
    fn max_module_temperature(&self) -> i16;

    /// Max Module Temperature String
    ///
    /// String containing the module with maximum temperature.
    fn max_module_temperature_string(&self) -> Option<u16> {
        None
    }

    /// Max Module Temperature Module
    ///
    /// Module with maximum temperature.
    fn max_module_temperature_module(&self) -> Option<u16> {
        None
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    ///
    /// Measurement.
    fn min_module_temperature(&self) -> i16;

    /// Min Module Temperature String
    ///
    /// String containing the module with minimum temperature.
    fn min_module_temperature_string(&self) -> Option<u16> {
        None
    }

    /// Min Module Temperature Module
    ///
    /// Module with minimum temperature.
    fn min_module_temperature_module(&self) -> Option<u16> {
        None
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    ///
    /// Calculation based on measurements.
    fn average_module_temperature(&self) -> Option<i16> {
        None
    }

    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Measurement.
    fn max_string_voltage(&self) -> Option<u16> {
        None
    }

    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    fn max_string_voltage_string(&self) -> Option<u16> {
        None
    }

    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Measurement.
    fn min_string_voltage(&self) -> Option<u16> {
        None
    }

    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    fn min_string_voltage_string(&self) -> Option<u16> {
        None
    }

    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Calculation based on measurements.
    fn average_string_voltage(&self) -> Option<u16> {
        None
    }

    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Measurement.
    fn max_string_current(&self) -> Option<i16> {
        None
    }

    /// Max String Current String
    ///
    /// String with the maximum current.
    fn max_string_current_string(&self) -> Option<u16> {
        None
    }

    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Measurement.
    fn min_string_current(&self) -> Option<i16> {
        None
    }

    /// Min String Current String
    ///
    /// String with the minimum current.
    fn min_string_current_string(&self) -> Option<u16> {
        None
    }

    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Calculation based on measurements.
    fn average_string_current(&self) -> Option<i16> {
        None
    }

    /// Battery Cell Balancing Count
    ///
    /// Total number of cells that are currently being balanced.
    fn battery_cell_balancing_count(&self) -> Option<u16> {
        None
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for module temperatures.
    fn mod_tmp_sf(&self) -> u16;

    /// Scale factor for string currents.
    fn a_sf(&self) -> u16;

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16;

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        None
    }

    /// Module Count
    ///
    /// Count of modules in the string.
    fn string_module_count(&self, string_index: u16) -> u16;

    /// String Status
    ///
    /// Current status of the string.
    fn string_string_status(&self, string_index: u16) -> u32;

    /// Connection Failure Reason
    fn string_connection_failure_reason(&self, string_index: u16) -> Option<StrConFail> {
        None
    }

    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    fn string_string_state_of_charge(&self, string_index: u16) -> u16;

    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    fn string_string_state_of_health(&self, string_index: u16) -> Option<u16> {
        None
    }

    /// String Current
    ///
    /// String current measurement.
    fn string_string_current(&self, string_index: u16) -> i16;

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    fn string_max_cell_voltage(&self, string_index: u16) -> u16;

    /// Max Cell Voltage Module
    ///
    /// Module containing the maximum cell voltage.
    fn string_max_cell_voltage_module(&self, string_index: u16) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    fn string_min_cell_voltage(&self, string_index: u16) -> u16;

    /// Min Cell Voltage Module
    ///
    /// Module containing the minimum cell voltage.
    fn string_min_cell_voltage_module(&self, string_index: u16) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    fn string_average_cell_voltage(&self, string_index: u16) -> u16;

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    fn string_max_module_temperature(&self, string_index: u16) -> i16;

    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    fn string_max_module_temperature_module(&self, string_index: u16) -> Option<u16> {
        None
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    fn string_min_module_temperature(&self, string_index: u16) -> i16;

    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    fn string_min_module_temperature_module(&self, string_index: u16) -> Option<u16> {
        None
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    fn string_average_module_temperature(&self, string_index: u16) -> i16;

    /// Disabled Reason
    ///
    /// Reason why the string is currently disabled.
    fn string_disabled_reason(&self, string_index: u16) -> Option<StrDisRsn> {
        None
    }

    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    fn string_contactor_status(&self, string_index: u16) -> Option<u32> {
        None
    }

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn string_string_event_1(&self, string_index: u16) -> u32;

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    fn string_string_event_2(&self, string_index: u16) -> Option<u32> {
        None
    }

    /// Vendor String Event Bitfield 1
    ///
    /// Vendor defined events.
    fn string_vendor_string_event_bitfield_1(&self, string_index: u16) -> Option<u32> {
        None
    }

    /// Vendor String Event Bitfield 2
    ///
    /// Vendor defined events.
    fn string_vendor_string_event_bitfield_2(&self, string_index: u16) -> Option<u32> {
        None
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string.
    fn string_enable_disable_string(&self, string_index: u16) -> Option<StrSetEna> {
        None
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string.
    fn set_string_enable_disable_string(&mut self, value: StrSetEna, string_index: u16) {}

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    fn string_connect_disconnect_string(&self, string_index: u16) -> Option<StrSetCon> {
        None
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    fn set_string_connect_disconnect_string(&mut self, value: StrSetCon, string_index: u16) {}
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum StrConFail {
    NoFailure = 0,
    ButtonPushed = 1,
    StrGroundFault = 2,
    OutsideVoltageRange = 3,
    StringNotEnabled = 4,
    FuseOpen = 5,
    ContactorFailure = 6,
    PrechargeFailure = 7,
    StringFault = 8,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum StrDisRsn {
    None = 0,
    Fault = 1,
    Maintenance = 2,
    External = 3,
    Other = 4,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum StrSetCon {
    ConnectString = 1,
    DisconnectString = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum StrSetEna {
    EnableString = 1,
    DisableString = 2,
}

#[repr(C)]
pub struct Model803CallbackAdapter {
    context: *mut c_void,
    string_count_callback: extern "C" fn(*const c_void) -> u16,
    connected_string_count_callback: extern "C" fn(*const c_void) -> u16,
    max_module_temperature_callback: extern "C" fn(*const c_void) -> i16,
    max_module_temperature_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_module_temperature_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_module_temperature_callback: extern "C" fn(*const c_void) -> i16,
    min_module_temperature_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_module_temperature_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_module_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    max_string_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_string_voltage_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_string_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_string_voltage_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_string_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_string_current_callback: Option<extern "C" fn(*const c_void) -> i16>,
    max_string_current_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_string_current_callback: Option<extern "C" fn(*const c_void) -> i16>,
    min_string_current_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_string_current_callback: Option<extern "C" fn(*const c_void) -> i16>,
    battery_cell_balancing_count_callback: Option<extern "C" fn(*const c_void) -> u16>,
    cell_v_sf_callback: extern "C" fn(*const c_void) -> u16,
    mod_tmp_sf_callback: extern "C" fn(*const c_void) -> u16,
    a_sf_callback: extern "C" fn(*const c_void) -> u16,
    so_h_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    so_c_sf_callback: extern "C" fn(*const c_void) -> u16,
    v_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    string_module_count_callback: extern "C" fn(*const c_void, u16) -> u16,
    string_string_status_callback: extern "C" fn(*const c_void, u16) -> u32,
    string_connection_failure_reason_callback:
        Option<extern "C" fn(*const c_void, u16) -> StrConFail>,
    string_string_state_of_charge_callback: extern "C" fn(*const c_void, u16) -> u16,
    string_string_state_of_health_callback: Option<extern "C" fn(*const c_void, u16) -> u16>,
    string_string_current_callback: extern "C" fn(*const c_void, u16) -> i16,
    string_max_cell_voltage_callback: extern "C" fn(*const c_void, u16) -> u16,
    string_max_cell_voltage_module_callback: Option<extern "C" fn(*const c_void, u16) -> u16>,
    string_min_cell_voltage_callback: extern "C" fn(*const c_void, u16) -> u16,
    string_min_cell_voltage_module_callback: Option<extern "C" fn(*const c_void, u16) -> u16>,
    string_average_cell_voltage_callback: extern "C" fn(*const c_void, u16) -> u16,
    string_max_module_temperature_callback: extern "C" fn(*const c_void, u16) -> i16,
    string_max_module_temperature_module_callback: Option<extern "C" fn(*const c_void, u16) -> u16>,
    string_min_module_temperature_callback: extern "C" fn(*const c_void, u16) -> i16,
    string_min_module_temperature_module_callback: Option<extern "C" fn(*const c_void, u16) -> u16>,
    string_average_module_temperature_callback: extern "C" fn(*const c_void, u16) -> i16,
    string_disabled_reason_callback: Option<extern "C" fn(*const c_void, u16) -> StrDisRsn>,
    string_contactor_status_callback: Option<extern "C" fn(*const c_void, u16) -> u32>,
    string_string_event_1_callback: extern "C" fn(*const c_void, u16) -> u32,
    string_string_event_2_callback: Option<extern "C" fn(*const c_void, u16) -> u32>,
    string_vendor_string_event_bitfield_1_callback:
        Option<extern "C" fn(*const c_void, u16) -> u32>,
    string_vendor_string_event_bitfield_2_callback:
        Option<extern "C" fn(*const c_void, u16) -> u32>,
    string_enable_disable_string_callback: Option<extern "C" fn(*const c_void, u16) -> StrSetEna>,
    set_string_enable_disable_string_callback: Option<extern "C" fn(StrSetEna, *mut c_void, u16)>,
    string_connect_disconnect_string_callback:
        Option<extern "C" fn(*const c_void, u16) -> StrSetCon>,
    set_string_connect_disconnect_string_callback:
        Option<extern "C" fn(StrSetCon, *mut c_void, u16)>,
}

impl ModelAdapter for Model803CallbackAdapter {
    /// String Count
    ///
    /// Number of strings in the bank.
    fn string_count(&self) -> u16 {
        (self.string_count_callback)(self.context)
    }

    /// Connected String Count
    ///
    /// Number of strings with contactor closed.
    fn connected_string_count(&self) -> u16 {
        (self.connected_string_count_callback)(self.context)
    }

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    ///
    /// Measurement.
    fn max_module_temperature(&self) -> i16 {
        (self.max_module_temperature_callback)(self.context)
    }

    /// Max Module Temperature String
    ///
    /// String containing the module with maximum temperature.
    fn max_module_temperature_string(&self) -> Option<u16> {
        self.max_module_temperature_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Module Temperature Module
    ///
    /// Module with maximum temperature.
    fn max_module_temperature_module(&self) -> Option<u16> {
        self.max_module_temperature_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    ///
    /// Measurement.
    fn min_module_temperature(&self) -> i16 {
        (self.min_module_temperature_callback)(self.context)
    }

    /// Min Module Temperature String
    ///
    /// String containing the module with minimum temperature.
    fn min_module_temperature_string(&self) -> Option<u16> {
        self.min_module_temperature_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Module Temperature Module
    ///
    /// Module with minimum temperature.
    fn min_module_temperature_module(&self) -> Option<u16> {
        self.min_module_temperature_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    ///
    /// Calculation based on measurements.
    fn average_module_temperature(&self) -> Option<i16> {
        self.average_module_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Measurement.
    fn max_string_voltage(&self) -> Option<u16> {
        self.max_string_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    fn max_string_voltage_string(&self) -> Option<u16> {
        self.max_string_voltage_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Measurement.
    fn min_string_voltage(&self) -> Option<u16> {
        self.min_string_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    fn min_string_voltage_string(&self) -> Option<u16> {
        self.min_string_voltage_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Calculation based on measurements.
    fn average_string_voltage(&self) -> Option<u16> {
        self.average_string_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Measurement.
    fn max_string_current(&self) -> Option<i16> {
        self.max_string_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max String Current String
    ///
    /// String with the maximum current.
    fn max_string_current_string(&self) -> Option<u16> {
        self.max_string_current_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Measurement.
    fn min_string_current(&self) -> Option<i16> {
        self.min_string_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min String Current String
    ///
    /// String with the minimum current.
    fn min_string_current_string(&self) -> Option<u16> {
        self.min_string_current_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Calculation based on measurements.
    fn average_string_current(&self) -> Option<i16> {
        self.average_string_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Battery Cell Balancing Count
    ///
    /// Total number of cells that are currently being balanced.
    fn battery_cell_balancing_count(&self) -> Option<u16> {
        self.battery_cell_balancing_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        (self.cell_v_sf_callback)(self.context)
    }

    /// Scale factor for module temperatures.
    fn mod_tmp_sf(&self) -> u16 {
        (self.mod_tmp_sf_callback)(self.context)
    }

    /// Scale factor for string currents.
    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)(self.context)
    }

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        self.so_h_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16 {
        (self.so_c_sf_callback)(self.context)
    }

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        self.v_sf_callback.map(|callback| (callback)(self.context))
    }

    /// Module Count
    ///
    /// Count of modules in the string.
    fn string_module_count(&self, string_index: u16) -> u16 {
        (self.string_module_count_callback)(self.context, string_index)
    }

    /// String Status
    ///
    /// Current status of the string.
    fn string_string_status(&self, string_index: u16) -> u32 {
        (self.string_string_status_callback)(self.context, string_index)
    }

    /// Connection Failure Reason
    fn string_connection_failure_reason(&self, string_index: u16) -> Option<StrConFail> {
        self.string_connection_failure_reason_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    fn string_string_state_of_charge(&self, string_index: u16) -> u16 {
        (self.string_string_state_of_charge_callback)(self.context, string_index)
    }

    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    fn string_string_state_of_health(&self, string_index: u16) -> Option<u16> {
        self.string_string_state_of_health_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// String Current
    ///
    /// String current measurement.
    fn string_string_current(&self, string_index: u16) -> i16 {
        (self.string_string_current_callback)(self.context, string_index)
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    fn string_max_cell_voltage(&self, string_index: u16) -> u16 {
        (self.string_max_cell_voltage_callback)(self.context, string_index)
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the maximum cell voltage.
    fn string_max_cell_voltage_module(&self, string_index: u16) -> Option<u16> {
        self.string_max_cell_voltage_module_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    fn string_min_cell_voltage(&self, string_index: u16) -> u16 {
        (self.string_min_cell_voltage_callback)(self.context, string_index)
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the minimum cell voltage.
    fn string_min_cell_voltage_module(&self, string_index: u16) -> Option<u16> {
        self.string_min_cell_voltage_module_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    fn string_average_cell_voltage(&self, string_index: u16) -> u16 {
        (self.string_average_cell_voltage_callback)(self.context, string_index)
    }

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    fn string_max_module_temperature(&self, string_index: u16) -> i16 {
        (self.string_max_module_temperature_callback)(self.context, string_index)
    }

    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    fn string_max_module_temperature_module(&self, string_index: u16) -> Option<u16> {
        self.string_max_module_temperature_module_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    fn string_min_module_temperature(&self, string_index: u16) -> i16 {
        (self.string_min_module_temperature_callback)(self.context, string_index)
    }

    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    fn string_min_module_temperature_module(&self, string_index: u16) -> Option<u16> {
        self.string_min_module_temperature_module_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    fn string_average_module_temperature(&self, string_index: u16) -> i16 {
        (self.string_average_module_temperature_callback)(self.context, string_index)
    }

    /// Disabled Reason
    ///
    /// Reason why the string is currently disabled.
    fn string_disabled_reason(&self, string_index: u16) -> Option<StrDisRsn> {
        self.string_disabled_reason_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    fn string_contactor_status(&self, string_index: u16) -> Option<u32> {
        self.string_contactor_status_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn string_string_event_1(&self, string_index: u16) -> u32 {
        (self.string_string_event_1_callback)(self.context, string_index)
    }

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    fn string_string_event_2(&self, string_index: u16) -> Option<u32> {
        self.string_string_event_2_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Vendor String Event Bitfield 1
    ///
    /// Vendor defined events.
    fn string_vendor_string_event_bitfield_1(&self, string_index: u16) -> Option<u32> {
        self.string_vendor_string_event_bitfield_1_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Vendor String Event Bitfield 2
    ///
    /// Vendor defined events.
    fn string_vendor_string_event_bitfield_2(&self, string_index: u16) -> Option<u32> {
        self.string_vendor_string_event_bitfield_2_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string.
    fn string_enable_disable_string(&self, string_index: u16) -> Option<StrSetEna> {
        self.string_enable_disable_string_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string.
    fn set_string_enable_disable_string(&mut self, value: StrSetEna, string_index: u16) {
        if let Some(callback) = self.set_string_enable_disable_string_callback {
            (callback)(value, self.context, string_index);
        };
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    fn string_connect_disconnect_string(&self, string_index: u16) -> Option<StrSetCon> {
        self.string_connect_disconnect_string_callback
            .map(|callback| (callback)(self.context, string_index))
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    fn set_string_connect_disconnect_string(&mut self, value: StrSetCon, string_index: u16) {
        if let Some(callback) = self.set_string_connect_disconnect_string_callback {
            (callback)(value, self.context, string_index);
        };
    }
}

#[repr(C)]
pub struct Model803StatefulAdapter<const STRING_COUNT: usize> {
    string_count: u16,
    connected_string_count: u16,
    max_module_temperature: i16,
    max_module_temperature_string: u16,
    max_module_temperature_module: u16,
    min_module_temperature: i16,
    min_module_temperature_string: u16,
    min_module_temperature_module: u16,
    average_module_temperature: i16,
    max_string_voltage: u16,
    max_string_voltage_string: u16,
    min_string_voltage: u16,
    min_string_voltage_string: u16,
    average_string_voltage: u16,
    max_string_current: i16,
    max_string_current_string: u16,
    min_string_current: i16,
    min_string_current_string: u16,
    average_string_current: i16,
    battery_cell_balancing_count: u16,
    cell_v_sf: u16,
    mod_tmp_sf: u16,
    a_sf: u16,
    so_h_sf: u16,
    so_c_sf: u16,
    v_sf: u16,
    string: [Model803String; STRING_COUNT],
}

#[repr(C)]
pub struct Model803String {
    string_module_count: u16,
    string_string_status: u32,
    string_connection_failure_reason: StrConFail,
    string_string_state_of_charge: u16,
    string_string_state_of_health: u16,
    string_string_current: i16,
    string_max_cell_voltage: u16,
    string_max_cell_voltage_module: u16,
    string_min_cell_voltage: u16,
    string_min_cell_voltage_module: u16,
    string_average_cell_voltage: u16,
    string_max_module_temperature: i16,
    string_max_module_temperature_module: u16,
    string_min_module_temperature: i16,
    string_min_module_temperature_module: u16,
    string_average_module_temperature: i16,
    string_disabled_reason: StrDisRsn,
    string_contactor_status: u32,
    string_string_event_1: u32,
    string_string_event_2: u32,
    string_vendor_string_event_bitfield_1: u32,
    string_vendor_string_event_bitfield_2: u32,
    string_enable_disable_string: StrSetEna,
    string_connect_disconnect_string: StrSetCon,
}

impl<const STRING_COUNT: usize> ModelAdapter for Model803StatefulAdapter<STRING_COUNT> {
    /// String Count
    ///
    /// Number of strings in the bank.
    fn string_count(&self) -> u16 {
        self.string_count
    }

    /// Connected String Count
    ///
    /// Number of strings with contactor closed.
    fn connected_string_count(&self) -> u16 {
        self.connected_string_count
    }

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    ///
    /// Measurement.
    fn max_module_temperature(&self) -> i16 {
        self.max_module_temperature
    }

    /// Max Module Temperature String
    ///
    /// String containing the module with maximum temperature.
    fn max_module_temperature_string(&self) -> Option<u16> {
        Some(self.max_module_temperature_string)
    }

    /// Max Module Temperature Module
    ///
    /// Module with maximum temperature.
    fn max_module_temperature_module(&self) -> Option<u16> {
        Some(self.max_module_temperature_module)
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    ///
    /// Measurement.
    fn min_module_temperature(&self) -> i16 {
        self.min_module_temperature
    }

    /// Min Module Temperature String
    ///
    /// String containing the module with minimum temperature.
    fn min_module_temperature_string(&self) -> Option<u16> {
        Some(self.min_module_temperature_string)
    }

    /// Min Module Temperature Module
    ///
    /// Module with minimum temperature.
    fn min_module_temperature_module(&self) -> Option<u16> {
        Some(self.min_module_temperature_module)
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    ///
    /// Calculation based on measurements.
    fn average_module_temperature(&self) -> Option<i16> {
        Some(self.average_module_temperature)
    }

    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Measurement.
    fn max_string_voltage(&self) -> Option<u16> {
        Some(self.max_string_voltage)
    }

    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    fn max_string_voltage_string(&self) -> Option<u16> {
        Some(self.max_string_voltage_string)
    }

    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Measurement.
    fn min_string_voltage(&self) -> Option<u16> {
        Some(self.min_string_voltage)
    }

    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    fn min_string_voltage_string(&self) -> Option<u16> {
        Some(self.min_string_voltage_string)
    }

    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Calculation based on measurements.
    fn average_string_voltage(&self) -> Option<u16> {
        Some(self.average_string_voltage)
    }

    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Measurement.
    fn max_string_current(&self) -> Option<i16> {
        Some(self.max_string_current)
    }

    /// Max String Current String
    ///
    /// String with the maximum current.
    fn max_string_current_string(&self) -> Option<u16> {
        Some(self.max_string_current_string)
    }

    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Measurement.
    fn min_string_current(&self) -> Option<i16> {
        Some(self.min_string_current)
    }

    /// Min String Current String
    ///
    /// String with the minimum current.
    fn min_string_current_string(&self) -> Option<u16> {
        Some(self.min_string_current_string)
    }

    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Calculation based on measurements.
    fn average_string_current(&self) -> Option<i16> {
        Some(self.average_string_current)
    }

    /// Battery Cell Balancing Count
    ///
    /// Total number of cells that are currently being balanced.
    fn battery_cell_balancing_count(&self) -> Option<u16> {
        Some(self.battery_cell_balancing_count)
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        self.cell_v_sf
    }

    /// Scale factor for module temperatures.
    fn mod_tmp_sf(&self) -> u16 {
        self.mod_tmp_sf
    }

    /// Scale factor for string currents.
    fn a_sf(&self) -> u16 {
        self.a_sf
    }

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        Some(self.so_h_sf)
    }

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16 {
        self.so_c_sf
    }

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        Some(self.v_sf)
    }

    /// Module Count
    ///
    /// Count of modules in the string.
    fn string_module_count(&self, string_index: u16) -> u16 {
        self.string[string_index as usize].string_module_count
    }

    /// String Status
    ///
    /// Current status of the string.
    fn string_string_status(&self, string_index: u16) -> u32 {
        self.string[string_index as usize].string_string_status
    }

    /// Connection Failure Reason
    fn string_connection_failure_reason(&self, string_index: u16) -> Option<StrConFail> {
        Some(self.string[string_index as usize].string_connection_failure_reason)
    }

    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    fn string_string_state_of_charge(&self, string_index: u16) -> u16 {
        self.string[string_index as usize].string_string_state_of_charge
    }

    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    fn string_string_state_of_health(&self, string_index: u16) -> Option<u16> {
        Some(self.string[string_index as usize].string_string_state_of_health)
    }

    /// String Current
    ///
    /// String current measurement.
    fn string_string_current(&self, string_index: u16) -> i16 {
        self.string[string_index as usize].string_string_current
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    fn string_max_cell_voltage(&self, string_index: u16) -> u16 {
        self.string[string_index as usize].string_max_cell_voltage
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the maximum cell voltage.
    fn string_max_cell_voltage_module(&self, string_index: u16) -> Option<u16> {
        Some(self.string[string_index as usize].string_max_cell_voltage_module)
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    fn string_min_cell_voltage(&self, string_index: u16) -> u16 {
        self.string[string_index as usize].string_min_cell_voltage
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the minimum cell voltage.
    fn string_min_cell_voltage_module(&self, string_index: u16) -> Option<u16> {
        Some(self.string[string_index as usize].string_min_cell_voltage_module)
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    fn string_average_cell_voltage(&self, string_index: u16) -> u16 {
        self.string[string_index as usize].string_average_cell_voltage
    }

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    fn string_max_module_temperature(&self, string_index: u16) -> i16 {
        self.string[string_index as usize].string_max_module_temperature
    }

    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    fn string_max_module_temperature_module(&self, string_index: u16) -> Option<u16> {
        Some(self.string[string_index as usize].string_max_module_temperature_module)
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    fn string_min_module_temperature(&self, string_index: u16) -> i16 {
        self.string[string_index as usize].string_min_module_temperature
    }

    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    fn string_min_module_temperature_module(&self, string_index: u16) -> Option<u16> {
        Some(self.string[string_index as usize].string_min_module_temperature_module)
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    fn string_average_module_temperature(&self, string_index: u16) -> i16 {
        self.string[string_index as usize].string_average_module_temperature
    }

    /// Disabled Reason
    ///
    /// Reason why the string is currently disabled.
    fn string_disabled_reason(&self, string_index: u16) -> Option<StrDisRsn> {
        Some(self.string[string_index as usize].string_disabled_reason)
    }

    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    fn string_contactor_status(&self, string_index: u16) -> Option<u32> {
        Some(self.string[string_index as usize].string_contactor_status)
    }

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn string_string_event_1(&self, string_index: u16) -> u32 {
        self.string[string_index as usize].string_string_event_1
    }

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    fn string_string_event_2(&self, string_index: u16) -> Option<u32> {
        Some(self.string[string_index as usize].string_string_event_2)
    }

    /// Vendor String Event Bitfield 1
    ///
    /// Vendor defined events.
    fn string_vendor_string_event_bitfield_1(&self, string_index: u16) -> Option<u32> {
        Some(self.string[string_index as usize].string_vendor_string_event_bitfield_1)
    }

    /// Vendor String Event Bitfield 2
    ///
    /// Vendor defined events.
    fn string_vendor_string_event_bitfield_2(&self, string_index: u16) -> Option<u32> {
        Some(self.string[string_index as usize].string_vendor_string_event_bitfield_2)
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string.
    fn string_enable_disable_string(&self, string_index: u16) -> Option<StrSetEna> {
        Some(self.string[string_index as usize].string_enable_disable_string)
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string.
    fn set_string_enable_disable_string(&mut self, value: StrSetEna, string_index: u16) {
        self.string[string_index as usize].string_enable_disable_string = value;
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    fn string_connect_disconnect_string(&self, string_index: u16) -> Option<StrSetCon> {
        Some(self.string[string_index as usize].string_connect_disconnect_string)
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    fn set_string_connect_disconnect_string(&mut self, value: StrSetCon, string_index: u16) {
        self.string[string_index as usize].string_connect_disconnect_string = value;
    }
}
