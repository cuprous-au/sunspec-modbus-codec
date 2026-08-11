use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 60;

static POINTS: [PointDetails<()>; 52] = [
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
        point: |()| Point::StringIndex,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::ModuleCount,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::ConnectedModuleCount,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::MaxModuleVoltage,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::MaxModuleVoltageModule,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::MinModuleVoltage,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::MinModuleVoltageModule,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::AverageModuleVoltage,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::MaxCellVoltage,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::MaxCellVoltageModule,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::MaxCellVoltageStack,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::MinCellVoltage,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::MinCellVoltageModule,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::MinCellVoltageStack,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::AverageCellVoltage,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::MaxTemperature,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::MaxTemperatureModule,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::MinTemperature,
        size: 1,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::MinTemperatureModule,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::AverageTemperature,
        size: 1,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::StringEvent1,
        size: 2,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::StringEvent2,
        size: 2,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::VendorEventBitfield1,
        size: 2,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::VendorEventBitfield2,
        size: 2,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::ModVSf,
        size: 1,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::CellVSf,
        size: 1,
        start_address: 31,
    },
    PointDetails {
        point: |()| Point::TmpSf,
        size: 1,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::SoCSf,
        size: 1,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::OcvSf,
        size: 1,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::Pad,
        size: 1,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::ModuleModuleIndex,
        size: 1,
        start_address: 36,
    },
    PointDetails {
        point: |()| Point::ModuleStackCount,
        size: 1,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::ModuleModuleStatus,
        size: 2,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::ModuleModuleStateOfCharge,
        size: 1,
        start_address: 40,
    },
    PointDetails {
        point: |()| Point::ModuleOpenCircuitVoltage,
        size: 1,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::ModuleExternalVoltage,
        size: 1,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::ModuleMaximumCellVoltage,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::ModuleMaxCellVoltageCell,
        size: 1,
        start_address: 44,
    },
    PointDetails {
        point: |()| Point::ModuleMinimumCellVoltage,
        size: 1,
        start_address: 45,
    },
    PointDetails {
        point: |()| Point::ModuleMinCellVoltageCell,
        size: 1,
        start_address: 46,
    },
    PointDetails {
        point: |()| Point::ModuleAverageCellVoltage,
        size: 1,
        start_address: 47,
    },
    PointDetails {
        point: |()| Point::ModuleAnolyteTemperature,
        size: 1,
        start_address: 48,
    },
    PointDetails {
        point: |()| Point::ModuleCatholyteTemperature,
        size: 1,
        start_address: 49,
    },
    PointDetails {
        point: |()| Point::ModuleContactorStatus,
        size: 2,
        start_address: 50,
    },
    PointDetails {
        point: |()| Point::ModuleModuleEvent1,
        size: 2,
        start_address: 52,
    },
    PointDetails {
        point: |()| Point::ModuleModuleEvent2,
        size: 2,
        start_address: 54,
    },
    PointDetails {
        point: |()| Point::ModuleConnectionFailureReason,
        size: 1,
        start_address: 56,
    },
    PointDetails {
        point: |()| Point::ModuleEnableDisableModule,
        size: 1,
        start_address: 57,
    },
    PointDetails {
        point: |()| Point::ModuleConnectDisconnectModule,
        size: 1,
        start_address: 58,
    },
    PointDetails {
        point: |()| Point::ModuleDisabledReason,
        size: 1,
        start_address: 59,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
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
    Pad,
    ModuleModuleIndex,
    ModuleStackCount,
    ModuleModuleStatus,
    ModuleModuleStateOfCharge,
    ModuleOpenCircuitVoltage,
    ModuleExternalVoltage,
    ModuleMaximumCellVoltage,
    ModuleMaxCellVoltageCell,
    ModuleMinimumCellVoltage,
    ModuleMinCellVoltageCell,
    ModuleAverageCellVoltage,
    ModuleAnolyteTemperature,
    ModuleCatholyteTemperature,
    ModuleContactorStatus,
    ModuleModuleEvent1,
    ModuleModuleEvent2,
    ModuleConnectionFailureReason,
    ModuleEnableDisableModule,
    ModuleConnectDisconnectModule,
    ModuleDisabledReason,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    60
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
            buffer::write_u16(807, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::StringIndex => {
            buffer::write_u16(model.string_index(), buffer);
        }
        Point::ModuleCount => {
            buffer::write_u16(model.module_count(), buffer);
        }
        Point::ConnectedModuleCount => {
            buffer::write_u16(model.connected_module_count(), buffer);
        }
        Point::MaxModuleVoltage => {
            buffer::write_u16(model.max_module_voltage(), buffer);
        }
        Point::MaxModuleVoltageModule => {
            if let Some(value) = model.max_module_voltage_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinModuleVoltage => {
            buffer::write_u16(model.min_module_voltage(), buffer);
        }
        Point::MinModuleVoltageModule => {
            if let Some(value) = model.min_module_voltage_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageModuleVoltage => {
            buffer::write_u16(model.average_module_voltage(), buffer);
        }
        Point::MaxCellVoltage => {
            if let Some(value) = model.max_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxCellVoltageModule => {
            if let Some(value) = model.max_cell_voltage_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxCellVoltageStack => {
            if let Some(value) = model.max_cell_voltage_stack() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellVoltage => {
            if let Some(value) = model.min_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellVoltageModule => {
            if let Some(value) = model.min_cell_voltage_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellVoltageStack => {
            if let Some(value) = model.min_cell_voltage_stack() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageCellVoltage => {
            if let Some(value) = model.average_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxTemperature => {
            buffer::write_i16(model.max_temperature(), buffer);
        }
        Point::MaxTemperatureModule => {
            if let Some(value) = model.max_temperature_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinTemperature => {
            buffer::write_i16(model.min_temperature(), buffer);
        }
        Point::MinTemperatureModule => {
            if let Some(value) = model.min_temperature_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageTemperature => {
            buffer::write_i16(model.average_temperature(), buffer);
        }
        Point::StringEvent1 => {
            buffer::write_u32(model.string_event_1(), buffer, offset, limit);
        }
        Point::StringEvent2 => {
            buffer::write_u32(model.string_event_2(), buffer, offset, limit);
        }
        Point::VendorEventBitfield1 => {
            buffer::write_u32(model.vendor_event_bitfield_1(), buffer, offset, limit);
        }
        Point::VendorEventBitfield2 => {
            buffer::write_u32(model.vendor_event_bitfield_2(), buffer, offset, limit);
        }
        Point::ModVSf => {
            buffer::write_u16(model.mod_v_sf(), buffer);
        }
        Point::CellVSf => {
            buffer::write_u16(model.cell_v_sf(), buffer);
        }
        Point::TmpSf => {
            buffer::write_u16(model.tmp_sf(), buffer);
        }
        Point::SoCSf => {
            buffer::write_u16(model.so_c_sf(), buffer);
        }
        Point::OcvSf => {
            buffer::write_u16(model.ocv_sf(), buffer);
        }
        Point::Pad => {
            buffer::write_u16(0, buffer);
        }
        Point::ModuleModuleIndex => {
            buffer::write_u16(model.module_module_index(), buffer);
        }
        Point::ModuleStackCount => {
            buffer::write_u16(model.module_stack_count(), buffer);
        }
        Point::ModuleModuleStatus => {
            buffer::write_u32(model.module_module_status(), buffer, offset, limit);
        }
        Point::ModuleModuleStateOfCharge => {
            buffer::write_u16(model.module_module_state_of_charge(), buffer);
        }
        Point::ModuleOpenCircuitVoltage => {
            buffer::write_u16(model.module_open_circuit_voltage(), buffer);
        }
        Point::ModuleExternalVoltage => {
            buffer::write_u16(model.module_external_voltage(), buffer);
        }
        Point::ModuleMaximumCellVoltage => {
            if let Some(value) = model.module_maximum_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleMaxCellVoltageCell => {
            if let Some(value) = model.module_max_cell_voltage_cell() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleMinimumCellVoltage => {
            if let Some(value) = model.module_minimum_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleMinCellVoltageCell => {
            if let Some(value) = model.module_min_cell_voltage_cell() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleAverageCellVoltage => {
            if let Some(value) = model.module_average_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleAnolyteTemperature => {
            if let Some(value) = model.module_anolyte_temperature() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleCatholyteTemperature => {
            if let Some(value) = model.module_catholyte_temperature() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleContactorStatus => {
            if let Some(value) = model.module_contactor_status() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleModuleEvent1 => {
            buffer::write_u32(model.module_module_event_1(), buffer, offset, limit);
        }
        Point::ModuleModuleEvent2 => {
            buffer::write_u32(model.module_module_event_2(), buffer, offset, limit);
        }
        Point::ModuleConnectionFailureReason => {
            if let Some(value) = model.module_connection_failure_reason() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleEnableDisableModule => {
            if let Some(value) = model.module_enable_disable_module() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleConnectDisconnectModule => {
            if let Some(value) = model.module_connect_disconnect_module() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleDisabledReason => {
            if let Some(value) = model.module_disabled_reason() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
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

    /// Module Index
    ///
    /// Index of the module within the string.
    fn module_module_index(&self) -> u16;

    /// Stack Count
    ///
    /// Number of stacks in this module.
    fn module_stack_count(&self) -> u16;

    /// Module Status
    ///
    /// Current status of the module.
    fn module_module_status(&self) -> u32;

    /// Module State of Charge
    ///
    /// State of charge for this module.
    fn module_module_state_of_charge(&self) -> u16;

    /// Open Circuit Voltage
    ///
    /// Open circuit voltage for this module.
    fn module_open_circuit_voltage(&self) -> u16;

    /// External Voltage
    ///
    /// External voltage fo this module.
    fn module_external_voltage(&self) -> u16;

    /// Maximum Cell Voltage
    ///
    /// Maximum voltage for all cells in this module.
    fn module_maximum_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum cell voltage.
    fn module_max_cell_voltage_cell(&self) -> Option<u16> {
        None
    }

    /// Minimum Cell Voltage
    ///
    /// Minimum voltage for all cells in this module.
    fn module_minimum_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum cell voltage.
    fn module_min_cell_voltage_cell(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in this module.
    fn module_average_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Anolyte Temperature
    fn module_anolyte_temperature(&self) -> Option<u16> {
        None
    }

    /// Catholyte Temperature
    fn module_catholyte_temperature(&self) -> Option<u16> {
        None
    }

    /// Contactor Status
    fn module_contactor_status(&self) -> Option<u32> {
        None
    }

    /// Module Event 1
    ///
    /// Alarms, warnings and status values.
    fn module_module_event_1(&self) -> u32;

    /// Module Event 2
    ///
    /// Alarms, warnings and status values.
    fn module_module_event_2(&self) -> u32;

    /// Connection Failure Reason
    fn module_connection_failure_reason(&self) -> Option<ModConFail> {
        None
    }

    /// Enable/Disable Module
    ///
    /// Enables and disables the module.
    fn module_enable_disable_module(&self) -> Option<ModSetEna> {
        None
    }

    /// Enable/Disable Module
    ///
    /// Enables and disables the module.
    fn set_module_enable_disable_module(&mut self, value: ModSetEna) {}

    /// Connect/Disconnect Module
    ///
    /// Connects and disconnects the module.
    fn module_connect_disconnect_module(&self) -> Option<ModSetCon> {
        None
    }

    /// Connect/Disconnect Module
    ///
    /// Connects and disconnects the module.
    fn set_module_connect_disconnect_module(&mut self, value: ModSetCon) {}

    /// Disabled Reason
    ///
    /// Reason why the module is currently disabled.
    fn module_disabled_reason(&self) -> Option<ModDisRsn> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ModConFail {
    NoFailure = 0,
    ButtonPushed = 1,
    ModuleGroundFault = 2,
    OutsideVoltageRange = 3,
    ModuleNotEnabled = 4,
    FuseOpen = 5,
    ContactorFailure = 6,
    PrechargeFailure = 7,
    ModuleFault = 8,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ModDisRsn {
    None = 0,
    Fault = 1,
    Maintenance = 2,
    External = 3,
    Other = 4,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ModSetCon {
    ConnectModule = 1,
    DisconnectModule = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ModSetEna {
    EnableModule = 1,
    DisableModule = 2,
}

#[repr(C)]
pub struct Model807CallbackAdapter {
    context: *mut c_void,
    string_index_callback: extern "C" fn(*const c_void) -> u16,
    module_count_callback: extern "C" fn(*const c_void) -> u16,
    connected_module_count_callback: extern "C" fn(*const c_void) -> u16,
    max_module_voltage_callback: extern "C" fn(*const c_void) -> u16,
    max_module_voltage_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_module_voltage_callback: extern "C" fn(*const c_void) -> u16,
    min_module_voltage_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_module_voltage_callback: extern "C" fn(*const c_void) -> u16,
    max_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_cell_voltage_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_cell_voltage_stack_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_voltage_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_voltage_stack_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_temperature_callback: extern "C" fn(*const c_void) -> i16,
    max_temperature_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_temperature_callback: extern "C" fn(*const c_void) -> i16,
    min_temperature_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_temperature_callback: extern "C" fn(*const c_void) -> i16,
    string_event_1_callback: extern "C" fn(*const c_void) -> u32,
    string_event_2_callback: extern "C" fn(*const c_void) -> u32,
    vendor_event_bitfield_1_callback: extern "C" fn(*const c_void) -> u32,
    vendor_event_bitfield_2_callback: extern "C" fn(*const c_void) -> u32,
    mod_v_sf_callback: extern "C" fn(*const c_void) -> u16,
    cell_v_sf_callback: extern "C" fn(*const c_void) -> u16,
    tmp_sf_callback: extern "C" fn(*const c_void) -> u16,
    so_c_sf_callback: extern "C" fn(*const c_void) -> u16,
    ocv_sf_callback: extern "C" fn(*const c_void) -> u16,
    module_module_index_callback: extern "C" fn(*const c_void) -> u16,
    module_stack_count_callback: extern "C" fn(*const c_void) -> u16,
    module_module_status_callback: extern "C" fn(*const c_void) -> u32,
    module_module_state_of_charge_callback: extern "C" fn(*const c_void) -> u16,
    module_open_circuit_voltage_callback: extern "C" fn(*const c_void) -> u16,
    module_external_voltage_callback: extern "C" fn(*const c_void) -> u16,
    module_maximum_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_max_cell_voltage_cell_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_minimum_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_min_cell_voltage_cell_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_average_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_anolyte_temperature_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_catholyte_temperature_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_contactor_status_callback: Option<extern "C" fn(*const c_void) -> u32>,
    module_module_event_1_callback: extern "C" fn(*const c_void) -> u32,
    module_module_event_2_callback: extern "C" fn(*const c_void) -> u32,
    module_connection_failure_reason_callback: Option<extern "C" fn(*const c_void) -> ModConFail>,
    module_enable_disable_module_callback: Option<extern "C" fn(*const c_void) -> ModSetEna>,
    set_module_enable_disable_module_callback: Option<extern "C" fn(ModSetEna, *mut c_void)>,
    module_connect_disconnect_module_callback: Option<extern "C" fn(*const c_void) -> ModSetCon>,
    set_module_connect_disconnect_module_callback: Option<extern "C" fn(ModSetCon, *mut c_void)>,
    module_disabled_reason_callback: Option<extern "C" fn(*const c_void) -> ModDisRsn>,
}

impl ModelAdapter for Model807CallbackAdapter {
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
    /// Number of modules in this string.
    fn module_count(&self) -> u16 {
        (self.module_count_callback)(self.context)
    }

    /// Connected Module Count
    ///
    /// Number of electrically connected modules in this string.
    fn connected_module_count(&self) -> u16 {
        (self.connected_module_count_callback)(self.context)
    }

    /// Max Module Voltage
    ///
    /// Maximum voltage for all modules in the string.
    ///
    /// Measurement.
    fn max_module_voltage(&self) -> u16 {
        (self.max_module_voltage_callback)(self.context)
    }

    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    fn max_module_voltage_module(&self) -> Option<u16> {
        self.max_module_voltage_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Measurement.
    fn min_module_voltage(&self) -> u16 {
        (self.min_module_voltage_callback)(self.context)
    }

    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    fn min_module_voltage_module(&self) -> Option<u16> {
        self.min_module_voltage_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_module_voltage(&self) -> u16 {
        (self.average_module_voltage_callback)(self.context)
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> Option<u16> {
        self.max_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with the maximum voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        self.max_cell_voltage_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Cell Voltage Stack
    ///
    /// Stack containing the cell with the maximum voltage.
    fn max_cell_voltage_stack(&self) -> Option<u16> {
        self.max_cell_voltage_stack_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> Option<u16> {
        self.min_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with the minimum voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        self.min_cell_voltage_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage Stack
    ///
    /// Stack containing the cell with the minimum voltage.
    fn min_cell_voltage_stack(&self) -> Option<u16> {
        self.min_cell_voltage_stack_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> Option<u16> {
        self.average_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    fn max_temperature(&self) -> i16 {
        (self.max_temperature_callback)(self.context)
    }

    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    fn max_temperature_module(&self) -> Option<u16> {
        self.max_temperature_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    fn min_temperature(&self) -> i16 {
        (self.min_temperature_callback)(self.context)
    }

    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    fn min_temperature_module(&self) -> Option<u16> {
        self.min_temperature_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_temperature(&self) -> i16 {
        (self.average_temperature_callback)(self.context)
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
    fn string_event_2(&self) -> u32 {
        (self.string_event_2_callback)(self.context)
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> u32 {
        (self.vendor_event_bitfield_1_callback)(self.context)
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> u32 {
        (self.vendor_event_bitfield_2_callback)(self.context)
    }

    fn mod_v_sf(&self) -> u16 {
        (self.mod_v_sf_callback)(self.context)
    }

    /// Scale factor for voltage.
    fn cell_v_sf(&self) -> u16 {
        (self.cell_v_sf_callback)(self.context)
    }

    /// Scale factor for temperature.
    fn tmp_sf(&self) -> u16 {
        (self.tmp_sf_callback)(self.context)
    }

    /// Scale factor for state of charge.
    fn so_c_sf(&self) -> u16 {
        (self.so_c_sf_callback)(self.context)
    }

    /// Scale factor for open circuit voltage.
    fn ocv_sf(&self) -> u16 {
        (self.ocv_sf_callback)(self.context)
    }

    /// Module Index
    ///
    /// Index of the module within the string.
    fn module_module_index(&self) -> u16 {
        (self.module_module_index_callback)(self.context)
    }

    /// Stack Count
    ///
    /// Number of stacks in this module.
    fn module_stack_count(&self) -> u16 {
        (self.module_stack_count_callback)(self.context)
    }

    /// Module Status
    ///
    /// Current status of the module.
    fn module_module_status(&self) -> u32 {
        (self.module_module_status_callback)(self.context)
    }

    /// Module State of Charge
    ///
    /// State of charge for this module.
    fn module_module_state_of_charge(&self) -> u16 {
        (self.module_module_state_of_charge_callback)(self.context)
    }

    /// Open Circuit Voltage
    ///
    /// Open circuit voltage for this module.
    fn module_open_circuit_voltage(&self) -> u16 {
        (self.module_open_circuit_voltage_callback)(self.context)
    }

    /// External Voltage
    ///
    /// External voltage fo this module.
    fn module_external_voltage(&self) -> u16 {
        (self.module_external_voltage_callback)(self.context)
    }

    /// Maximum Cell Voltage
    ///
    /// Maximum voltage for all cells in this module.
    fn module_maximum_cell_voltage(&self) -> Option<u16> {
        self.module_maximum_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum cell voltage.
    fn module_max_cell_voltage_cell(&self) -> Option<u16> {
        self.module_max_cell_voltage_cell_callback
            .map(|callback| (callback)(self.context))
    }

    /// Minimum Cell Voltage
    ///
    /// Minimum voltage for all cells in this module.
    fn module_minimum_cell_voltage(&self) -> Option<u16> {
        self.module_minimum_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum cell voltage.
    fn module_min_cell_voltage_cell(&self) -> Option<u16> {
        self.module_min_cell_voltage_cell_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in this module.
    fn module_average_cell_voltage(&self) -> Option<u16> {
        self.module_average_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Anolyte Temperature
    fn module_anolyte_temperature(&self) -> Option<u16> {
        self.module_anolyte_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Catholyte Temperature
    fn module_catholyte_temperature(&self) -> Option<u16> {
        self.module_catholyte_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Contactor Status
    fn module_contactor_status(&self) -> Option<u32> {
        self.module_contactor_status_callback
            .map(|callback| (callback)(self.context))
    }

    /// Module Event 1
    ///
    /// Alarms, warnings and status values.
    fn module_module_event_1(&self) -> u32 {
        (self.module_module_event_1_callback)(self.context)
    }

    /// Module Event 2
    ///
    /// Alarms, warnings and status values.
    fn module_module_event_2(&self) -> u32 {
        (self.module_module_event_2_callback)(self.context)
    }

    /// Connection Failure Reason
    fn module_connection_failure_reason(&self) -> Option<ModConFail> {
        self.module_connection_failure_reason_callback
            .map(|callback| (callback)(self.context))
    }

    /// Enable/Disable Module
    ///
    /// Enables and disables the module.
    fn module_enable_disable_module(&self) -> Option<ModSetEna> {
        self.module_enable_disable_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Enable/Disable Module
    ///
    /// Enables and disables the module.
    fn set_module_enable_disable_module(&mut self, value: ModSetEna) {
        if let Some(callback) = self.set_module_enable_disable_module_callback {
            (callback)(value, self.context);
        };
    }

    /// Connect/Disconnect Module
    ///
    /// Connects and disconnects the module.
    fn module_connect_disconnect_module(&self) -> Option<ModSetCon> {
        self.module_connect_disconnect_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Connect/Disconnect Module
    ///
    /// Connects and disconnects the module.
    fn set_module_connect_disconnect_module(&mut self, value: ModSetCon) {
        if let Some(callback) = self.set_module_connect_disconnect_module_callback {
            (callback)(value, self.context);
        };
    }

    /// Disabled Reason
    ///
    /// Reason why the module is currently disabled.
    fn module_disabled_reason(&self) -> Option<ModDisRsn> {
        self.module_disabled_reason_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model807StatefulAdapter {
    string_index: u16,
    module_count: u16,
    connected_module_count: u16,
    max_module_voltage: u16,
    max_module_voltage_module: u16,
    min_module_voltage: u16,
    min_module_voltage_module: u16,
    average_module_voltage: u16,
    max_cell_voltage: u16,
    max_cell_voltage_module: u16,
    max_cell_voltage_stack: u16,
    min_cell_voltage: u16,
    min_cell_voltage_module: u16,
    min_cell_voltage_stack: u16,
    average_cell_voltage: u16,
    max_temperature: i16,
    max_temperature_module: u16,
    min_temperature: i16,
    min_temperature_module: u16,
    average_temperature: i16,
    string_event_1: u32,
    string_event_2: u32,
    vendor_event_bitfield_1: u32,
    vendor_event_bitfield_2: u32,
    mod_v_sf: u16,
    cell_v_sf: u16,
    tmp_sf: u16,
    so_c_sf: u16,
    ocv_sf: u16,
    module_module_index: u16,
    module_stack_count: u16,
    module_module_status: u32,
    module_module_state_of_charge: u16,
    module_open_circuit_voltage: u16,
    module_external_voltage: u16,
    module_maximum_cell_voltage: u16,
    module_max_cell_voltage_cell: u16,
    module_minimum_cell_voltage: u16,
    module_min_cell_voltage_cell: u16,
    module_average_cell_voltage: u16,
    module_anolyte_temperature: u16,
    module_catholyte_temperature: u16,
    module_contactor_status: u32,
    module_module_event_1: u32,
    module_module_event_2: u32,
    module_connection_failure_reason: ModConFail,
    module_enable_disable_module: ModSetEna,
    module_connect_disconnect_module: ModSetCon,
    module_disabled_reason: ModDisRsn,
}

impl ModelAdapter for Model807StatefulAdapter {
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
    /// Number of modules in this string.
    fn module_count(&self) -> u16 {
        self.module_count
    }

    /// Connected Module Count
    ///
    /// Number of electrically connected modules in this string.
    fn connected_module_count(&self) -> u16 {
        self.connected_module_count
    }

    /// Max Module Voltage
    ///
    /// Maximum voltage for all modules in the string.
    ///
    /// Measurement.
    fn max_module_voltage(&self) -> u16 {
        self.max_module_voltage
    }

    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    fn max_module_voltage_module(&self) -> Option<u16> {
        Some(self.max_module_voltage_module)
    }

    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Measurement.
    fn min_module_voltage(&self) -> u16 {
        self.min_module_voltage
    }

    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    fn min_module_voltage_module(&self) -> Option<u16> {
        Some(self.min_module_voltage_module)
    }

    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_module_voltage(&self) -> u16 {
        self.average_module_voltage
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> Option<u16> {
        Some(self.max_cell_voltage)
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with the maximum voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        Some(self.max_cell_voltage_module)
    }

    /// Max Cell Voltage Stack
    ///
    /// Stack containing the cell with the maximum voltage.
    fn max_cell_voltage_stack(&self) -> Option<u16> {
        Some(self.max_cell_voltage_stack)
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> Option<u16> {
        Some(self.min_cell_voltage)
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with the minimum voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        Some(self.min_cell_voltage_module)
    }

    /// Min Cell Voltage Stack
    ///
    /// Stack containing the cell with the minimum voltage.
    fn min_cell_voltage_stack(&self) -> Option<u16> {
        Some(self.min_cell_voltage_stack)
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> Option<u16> {
        Some(self.average_cell_voltage)
    }

    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    fn max_temperature(&self) -> i16 {
        self.max_temperature
    }

    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    fn max_temperature_module(&self) -> Option<u16> {
        Some(self.max_temperature_module)
    }

    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    fn min_temperature(&self) -> i16 {
        self.min_temperature
    }

    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    fn min_temperature_module(&self) -> Option<u16> {
        Some(self.min_temperature_module)
    }

    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_temperature(&self) -> i16 {
        self.average_temperature
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
    fn string_event_2(&self) -> u32 {
        self.string_event_2
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> u32 {
        self.vendor_event_bitfield_1
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> u32 {
        self.vendor_event_bitfield_2
    }

    fn mod_v_sf(&self) -> u16 {
        self.mod_v_sf
    }

    /// Scale factor for voltage.
    fn cell_v_sf(&self) -> u16 {
        self.cell_v_sf
    }

    /// Scale factor for temperature.
    fn tmp_sf(&self) -> u16 {
        self.tmp_sf
    }

    /// Scale factor for state of charge.
    fn so_c_sf(&self) -> u16 {
        self.so_c_sf
    }

    /// Scale factor for open circuit voltage.
    fn ocv_sf(&self) -> u16 {
        self.ocv_sf
    }

    /// Module Index
    ///
    /// Index of the module within the string.
    fn module_module_index(&self) -> u16 {
        self.module_module_index
    }

    /// Stack Count
    ///
    /// Number of stacks in this module.
    fn module_stack_count(&self) -> u16 {
        self.module_stack_count
    }

    /// Module Status
    ///
    /// Current status of the module.
    fn module_module_status(&self) -> u32 {
        self.module_module_status
    }

    /// Module State of Charge
    ///
    /// State of charge for this module.
    fn module_module_state_of_charge(&self) -> u16 {
        self.module_module_state_of_charge
    }

    /// Open Circuit Voltage
    ///
    /// Open circuit voltage for this module.
    fn module_open_circuit_voltage(&self) -> u16 {
        self.module_open_circuit_voltage
    }

    /// External Voltage
    ///
    /// External voltage fo this module.
    fn module_external_voltage(&self) -> u16 {
        self.module_external_voltage
    }

    /// Maximum Cell Voltage
    ///
    /// Maximum voltage for all cells in this module.
    fn module_maximum_cell_voltage(&self) -> Option<u16> {
        Some(self.module_maximum_cell_voltage)
    }

    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum cell voltage.
    fn module_max_cell_voltage_cell(&self) -> Option<u16> {
        Some(self.module_max_cell_voltage_cell)
    }

    /// Minimum Cell Voltage
    ///
    /// Minimum voltage for all cells in this module.
    fn module_minimum_cell_voltage(&self) -> Option<u16> {
        Some(self.module_minimum_cell_voltage)
    }

    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum cell voltage.
    fn module_min_cell_voltage_cell(&self) -> Option<u16> {
        Some(self.module_min_cell_voltage_cell)
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in this module.
    fn module_average_cell_voltage(&self) -> Option<u16> {
        Some(self.module_average_cell_voltage)
    }

    /// Anolyte Temperature
    fn module_anolyte_temperature(&self) -> Option<u16> {
        Some(self.module_anolyte_temperature)
    }

    /// Catholyte Temperature
    fn module_catholyte_temperature(&self) -> Option<u16> {
        Some(self.module_catholyte_temperature)
    }

    /// Contactor Status
    fn module_contactor_status(&self) -> Option<u32> {
        Some(self.module_contactor_status)
    }

    /// Module Event 1
    ///
    /// Alarms, warnings and status values.
    fn module_module_event_1(&self) -> u32 {
        self.module_module_event_1
    }

    /// Module Event 2
    ///
    /// Alarms, warnings and status values.
    fn module_module_event_2(&self) -> u32 {
        self.module_module_event_2
    }

    /// Connection Failure Reason
    fn module_connection_failure_reason(&self) -> Option<ModConFail> {
        Some(self.module_connection_failure_reason)
    }

    /// Enable/Disable Module
    ///
    /// Enables and disables the module.
    fn module_enable_disable_module(&self) -> Option<ModSetEna> {
        Some(self.module_enable_disable_module)
    }

    /// Enable/Disable Module
    ///
    /// Enables and disables the module.
    fn set_module_enable_disable_module(&mut self, value: ModSetEna) {
        self.module_enable_disable_module = value;
    }

    /// Connect/Disconnect Module
    ///
    /// Connects and disconnects the module.
    fn module_connect_disconnect_module(&self) -> Option<ModSetCon> {
        Some(self.module_connect_disconnect_module)
    }

    /// Connect/Disconnect Module
    ///
    /// Connects and disconnects the module.
    fn set_module_connect_disconnect_module(&mut self, value: ModSetCon) {
        self.module_connect_disconnect_module = value;
    }

    /// Disabled Reason
    ///
    /// Reason why the module is currently disabled.
    fn module_disabled_reason(&self) -> Option<ModDisRsn> {
        Some(self.module_disabled_reason)
    }
}
