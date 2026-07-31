use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 48;

pub static POINTS: [ReadablePoint; 41] = [
    ReadablePoint {
        reference: PointReference::Static { value: 804 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 46 },
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
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::StringIndex => serialisation::write_u16(model.string_index(), buffer),
        Point::ModuleCount => serialisation::write_u16(model.module_count(), buffer),
        Point::StringStatus => {
            serialisation::write_u32(model.string_status(), buffer, offset, limit)
        }
        Point::ConnectionFailureReason => {
            if let Some(value) = model.connection_failure_reason() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::StringCellBalancingCount => {
            if let Some(value) = model.string_cell_balancing_count() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::StringStateOfCharge => {
            serialisation::write_u16(model.string_state_of_charge(), buffer)
        }
        Point::StringDepthOfDischarge => {
            if let Some(value) = model.string_depth_of_discharge() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::StringCycleCount => {
            if let Some(value) = model.string_cycle_count() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::StringStateOfHealth => {
            if let Some(value) = model.string_state_of_health() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::StringCurrent => serialisation::write_i16(model.string_current(), buffer),
        Point::StringVoltage => {
            if let Some(value) = model.string_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaxCellVoltage => serialisation::write_u16(model.max_cell_voltage(), buffer),
        Point::MaxCellVoltageModule => {
            if let Some(value) = model.max_cell_voltage_module() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MinCellVoltage => serialisation::write_u16(model.min_cell_voltage(), buffer),
        Point::MinCellVoltageModule => {
            if let Some(value) = model.min_cell_voltage_module() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::AverageCellVoltage => serialisation::write_u16(model.average_cell_voltage(), buffer),
        Point::MaxModuleTemperature => {
            serialisation::write_i16(model.max_module_temperature(), buffer)
        }
        Point::MaxModuleTemperatureModule => {
            serialisation::write_u16(model.max_module_temperature_module(), buffer)
        }
        Point::MinModuleTemperature => {
            serialisation::write_i16(model.min_module_temperature(), buffer)
        }
        Point::MinModuleTemperatureModule => {
            serialisation::write_u16(model.min_module_temperature_module(), buffer)
        }
        Point::AverageModuleTemperature => {
            serialisation::write_i16(model.average_module_temperature(), buffer)
        }
        Point::ContactorStatus => {
            if let Some(value) = model.contactor_status() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::StringEvent1 => {
            serialisation::write_u32(model.string_event_1(), buffer, offset, limit)
        }
        Point::StringEvent2 => {
            if let Some(value) = model.string_event_2() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::VendorEventBitfield1 => {
            if let Some(value) = model.vendor_event_bitfield_1() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::VendorEventBitfield2 => {
            if let Some(value) = model.vendor_event_bitfield_2() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::EnableDisableString => {
            if let Some(value) = model.enable_disable_string() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::ConnectDisconnectString => {
            if let Some(value) = model.connect_disconnect_string() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SoCSf => serialisation::write_u16(model.so_c_sf(), buffer),
        Point::SoHSf => {
            if let Some(value) = model.so_h_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DoDSf => {
            if let Some(value) = model.do_d_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ASf => serialisation::write_u16(model.a_sf(), buffer),
        Point::VSf => {
            if let Some(value) = model.v_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::CellVSf => serialisation::write_u16(model.cell_v_sf(), buffer),
        Point::ModTmpSf => serialisation::write_u16(model.mod_tmp_sf(), buffer),
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
    fn enable_disable_string(&self) -> Option<SetEna> {
        None
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn set_enable_disable_string(&mut self, value: SetEna) {}

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
}

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

pub enum SetEna {}

pub enum SetCon {
    ConnectString = 1,
    DisconnectString = 2,
}

#[repr(C)]
pub struct Model804CallbackAdapter {
    string_index_callback: extern "C" fn() -> u16,
    module_count_callback: extern "C" fn() -> u16,
    string_status_callback: extern "C" fn() -> u32,
    connection_failure_reason_callback: Option<extern "C" fn() -> ConFail>,
    string_cell_balancing_count_callback: Option<extern "C" fn() -> u16>,
    string_state_of_charge_callback: extern "C" fn() -> u16,
    string_depth_of_discharge_callback: Option<extern "C" fn() -> u16>,
    string_cycle_count_callback: Option<extern "C" fn() -> u32>,
    string_state_of_health_callback: Option<extern "C" fn() -> u16>,
    string_current_callback: extern "C" fn() -> i16,
    string_voltage_callback: Option<extern "C" fn() -> u16>,
    max_cell_voltage_callback: extern "C" fn() -> u16,
    max_cell_voltage_module_callback: Option<extern "C" fn() -> u16>,
    min_cell_voltage_callback: extern "C" fn() -> u16,
    min_cell_voltage_module_callback: Option<extern "C" fn() -> u16>,
    average_cell_voltage_callback: extern "C" fn() -> u16,
    max_module_temperature_callback: extern "C" fn() -> i16,
    max_module_temperature_module_callback: extern "C" fn() -> u16,
    min_module_temperature_callback: extern "C" fn() -> i16,
    min_module_temperature_module_callback: extern "C" fn() -> u16,
    average_module_temperature_callback: extern "C" fn() -> i16,
    contactor_status_callback: Option<extern "C" fn() -> u32>,
    string_event_1_callback: extern "C" fn() -> u32,
    string_event_2_callback: Option<extern "C" fn() -> u32>,
    vendor_event_bitfield_1_callback: Option<extern "C" fn() -> u32>,
    vendor_event_bitfield_2_callback: Option<extern "C" fn() -> u32>,
    enable_disable_string_callback: Option<extern "C" fn() -> SetEna>,
    set_enable_disable_string_callback: Option<extern "C" fn(SetEna)>,
    connect_disconnect_string_callback: Option<extern "C" fn() -> SetCon>,
    set_connect_disconnect_string_callback: Option<extern "C" fn(SetCon)>,
    so_c_sf_callback: extern "C" fn() -> u16,
    so_h_sf_callback: Option<extern "C" fn() -> u16>,
    do_d_sf_callback: Option<extern "C" fn() -> u16>,
    a_sf_callback: extern "C" fn() -> u16,
    v_sf_callback: Option<extern "C" fn() -> u16>,
    cell_v_sf_callback: extern "C" fn() -> u16,
    mod_tmp_sf_callback: extern "C" fn() -> u16,
}

impl ModelAdapter for Model804CallbackAdapter {
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Indices are one-based.
    fn string_index(&self) -> u16 {
        (self.string_index_callback)()
    }

    /// Module Count
    ///
    /// Count of modules in the string.
    fn module_count(&self) -> u16 {
        (self.module_count_callback)()
    }

    /// String Status
    ///
    /// Current status of the string.
    fn string_status(&self) -> u32 {
        (self.string_status_callback)()
    }

    /// Connection Failure Reason
    fn connection_failure_reason(&self) -> Option<ConFail> {
        self.connection_failure_reason_callback
            .map(|callback| (callback)())
    }

    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    fn string_cell_balancing_count(&self) -> Option<u16> {
        self.string_cell_balancing_count_callback
            .map(|callback| (callback)())
    }

    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn string_state_of_charge(&self) -> u16 {
        (self.string_state_of_charge_callback)()
    }

    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Measurement.
    fn string_depth_of_discharge(&self) -> Option<u16> {
        self.string_depth_of_discharge_callback
            .map(|callback| (callback)())
    }

    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    fn string_cycle_count(&self) -> Option<u32> {
        self.string_cycle_count_callback
            .map(|callback| (callback)())
    }

    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Measurement.
    fn string_state_of_health(&self) -> Option<u16> {
        self.string_state_of_health_callback
            .map(|callback| (callback)())
    }

    /// String Current
    ///
    /// String current measurement.
    ///
    /// Measurement.
    fn string_current(&self) -> i16 {
        (self.string_current_callback)()
    }

    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Measurement.
    fn string_voltage(&self) -> Option<u16> {
        self.string_voltage_callback.map(|callback| (callback)())
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> u16 {
        (self.max_cell_voltage_callback)()
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        self.max_cell_voltage_module_callback
            .map(|callback| (callback)())
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> u16 {
        (self.min_cell_voltage_callback)()
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        self.min_cell_voltage_module_callback
            .map(|callback| (callback)())
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> u16 {
        (self.average_cell_voltage_callback)()
    }

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Measurement.
    fn max_module_temperature(&self) -> i16 {
        (self.max_module_temperature_callback)()
    }

    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    fn max_module_temperature_module(&self) -> u16 {
        (self.max_module_temperature_module_callback)()
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Measurement.
    fn min_module_temperature(&self) -> i16 {
        (self.min_module_temperature_callback)()
    }

    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    fn min_module_temperature_module(&self) -> u16 {
        (self.min_module_temperature_module_callback)()
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn average_module_temperature(&self) -> i16 {
        (self.average_module_temperature_callback)()
    }

    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    fn contactor_status(&self) -> Option<u32> {
        self.contactor_status_callback.map(|callback| (callback)())
    }

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn string_event_1(&self) -> u32 {
        (self.string_event_1_callback)()
    }

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    ///
    /// Reserved for future use.
    fn string_event_2(&self) -> Option<u32> {
        self.string_event_2_callback.map(|callback| (callback)())
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> Option<u32> {
        self.vendor_event_bitfield_1_callback
            .map(|callback| (callback)())
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> Option<u32> {
        self.vendor_event_bitfield_2_callback
            .map(|callback| (callback)())
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn enable_disable_string(&self) -> Option<SetEna> {
        self.enable_disable_string_callback
            .map(|callback| (callback)())
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn set_enable_disable_string(&mut self, value: SetEna) {
        if let Some(callback) = self.set_enable_disable_string_callback {
            (callback)(value);
        };
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn connect_disconnect_string(&self) -> Option<SetCon> {
        self.connect_disconnect_string_callback
            .map(|callback| (callback)())
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn set_connect_disconnect_string(&mut self, value: SetCon) {
        if let Some(callback) = self.set_connect_disconnect_string_callback {
            (callback)(value);
        };
    }

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16 {
        (self.so_c_sf_callback)()
    }

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        self.so_h_sf_callback.map(|callback| (callback)())
    }

    /// Scale factor for string depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        self.do_d_sf_callback.map(|callback| (callback)())
    }

    /// Scale factor for string current.
    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)()
    }

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        self.v_sf_callback.map(|callback| (callback)())
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        (self.cell_v_sf_callback)()
    }

    /// Scale factor for module temperature.
    fn mod_tmp_sf(&self) -> u16 {
        (self.mod_tmp_sf_callback)()
    }
}
