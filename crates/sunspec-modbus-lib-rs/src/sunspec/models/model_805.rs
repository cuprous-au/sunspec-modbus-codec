use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 48;

pub static POINTS: [ReadablePoint; 31] = [
    ReadablePoint {
        reference: PointReference::Static { value: 805 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::StringIndex,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::ModuleIndex,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::ModuleCellCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::ModuleSoC,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::DepthOfDischarge,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::ModuleSoH,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::CycleCount,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::ModuleVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::MaxCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::MaxCellVoltageCell,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::MinCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::MinCellVoltageCell,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::AverageCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::MaxCellTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::MaxCellTemperatureCell,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::MinCellTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::MinCellTemperatureCell,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::AverageCellTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::BalancedCellCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::SerialNumber,
        },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::SoCSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::SoHSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::DoDSf,
        },
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
        reference: PointReference::Model805 {
            point: Point::CellVSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::TmpSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::LithiumIonModuleCellCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::LithiumIonModuleCellCellTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model805 {
            point: Point::LithiumIonModuleCellCellStatus,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
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
    LithiumIonModuleCellCellVoltage,
    LithiumIonModuleCellCellTemperature,
    LithiumIonModuleCellCellStatus,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    48
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
        Point::ModuleIndex => {
            buffer::write_u16(model.module_index(), buffer);
        }
        Point::ModuleCellCount => {
            buffer::write_u16(model.module_cell_count(), buffer);
        }
        Point::ModuleSoC => {
            if let Some(value) = model.module_so_c() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DepthOfDischarge => {
            if let Some(value) = model.depth_of_discharge() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleSoH => {
            if let Some(value) = model.module_so_h() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CycleCount => {
            if let Some(value) = model.cycle_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleVoltage => {
            buffer::write_u16(model.module_voltage(), buffer);
        }
        Point::MaxCellVoltage => {
            buffer::write_u16(model.max_cell_voltage(), buffer);
        }
        Point::MaxCellVoltageCell => {
            if let Some(value) = model.max_cell_voltage_cell() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellVoltage => {
            buffer::write_u16(model.min_cell_voltage(), buffer);
        }
        Point::MinCellVoltageCell => {
            if let Some(value) = model.min_cell_voltage_cell() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageCellVoltage => {
            buffer::write_u16(model.average_cell_voltage(), buffer);
        }
        Point::MaxCellTemperature => {
            buffer::write_i16(model.max_cell_temperature(), buffer);
        }
        Point::MaxCellTemperatureCell => {
            if let Some(value) = model.max_cell_temperature_cell() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellTemperature => {
            buffer::write_i16(model.min_cell_temperature(), buffer);
        }
        Point::MinCellTemperatureCell => {
            if let Some(value) = model.min_cell_temperature_cell() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageCellTemperature => {
            buffer::write_i16(model.average_cell_temperature(), buffer);
        }
        Point::BalancedCellCount => {
            if let Some(value) = model.balanced_cell_count() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SerialNumber => {
            if let Some(value) = model.serial_number() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SoCSf => {
            if let Some(value) = model.so_c_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
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
        Point::VSf => {
            buffer::write_u16(model.v_sf(), buffer);
        }
        Point::CellVSf => {
            buffer::write_u16(model.cell_v_sf(), buffer);
        }
        Point::TmpSf => {
            buffer::write_u16(model.tmp_sf(), buffer);
        }
        Point::LithiumIonModuleCellCellVoltage => {
            buffer::write_u16(model.lithium_ion_module_cell_cell_voltage(), buffer);
        }
        Point::LithiumIonModuleCellCellTemperature => {
            buffer::write_i16(model.lithium_ion_module_cell_cell_temperature(), buffer);
        }
        Point::LithiumIonModuleCellCellStatus => {
            if let Some(value) = model.lithium_ion_module_cell_cell_status() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
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
    fn serial_number(&self) -> Option<&CStr> {
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

    /// Cell Voltage
    ///
    /// Cell terminal voltage.
    fn lithium_ion_module_cell_cell_voltage(&self) -> u16;

    /// Cell Temperature
    ///
    /// Cell temperature.
    fn lithium_ion_module_cell_cell_temperature(&self) -> i16;

    /// Cell Status
    ///
    /// Status of the cell.
    fn lithium_ion_module_cell_cell_status(&self) -> Option<u32> {
        None
    }
}

#[repr(C)]
pub struct Model805CallbackAdapter {
    context: *mut c_void,
    string_index_callback: extern "C" fn(*const c_void) -> u16,
    module_index_callback: extern "C" fn(*const c_void) -> u16,
    module_cell_count_callback: extern "C" fn(*const c_void) -> u16,
    module_so_c_callback: Option<extern "C" fn(*const c_void) -> u16>,
    depth_of_discharge_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_so_h_callback: Option<extern "C" fn(*const c_void) -> u16>,
    cycle_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    module_voltage_callback: extern "C" fn(*const c_void) -> u16,
    max_cell_voltage_callback: extern "C" fn(*const c_void) -> u16,
    max_cell_voltage_cell_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_voltage_callback: extern "C" fn(*const c_void) -> u16,
    min_cell_voltage_cell_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_cell_voltage_callback: extern "C" fn(*const c_void) -> u16,
    max_cell_temperature_callback: extern "C" fn(*const c_void) -> i16,
    max_cell_temperature_cell_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_temperature_callback: extern "C" fn(*const c_void) -> i16,
    min_cell_temperature_cell_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_cell_temperature_callback: extern "C" fn(*const c_void) -> i16,
    balanced_cell_count_callback: Option<extern "C" fn(*const c_void) -> u16>,
    serial_number_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    so_c_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    so_h_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    do_d_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    v_sf_callback: extern "C" fn(*const c_void) -> u16,
    cell_v_sf_callback: extern "C" fn(*const c_void) -> u16,
    tmp_sf_callback: extern "C" fn(*const c_void) -> u16,
    lithium_ion_module_cell_cell_voltage_callback: extern "C" fn(*const c_void) -> u16,
    lithium_ion_module_cell_cell_temperature_callback: extern "C" fn(*const c_void) -> i16,
    lithium_ion_module_cell_cell_status_callback: Option<extern "C" fn(*const c_void) -> u32>,
}

impl ModelAdapter for Model805CallbackAdapter {
    /// String Index
    ///
    /// Index of the string containing the module.
    ///
    /// Indices are one-based.
    fn string_index(&self) -> u16 {
        (self.string_index_callback)(self.context)
    }

    /// Module Index
    ///
    /// Index of the module within the string.
    ///
    /// Indices are one-based.
    fn module_index(&self) -> u16 {
        (self.module_index_callback)(self.context)
    }

    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    fn module_cell_count(&self) -> u16 {
        (self.module_cell_count_callback)(self.context)
    }

    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    fn module_so_c(&self) -> Option<u16> {
        self.module_so_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Depth of Discharge
    ///
    /// Depth of discharge for the module.
    ///
    /// Measurement.
    fn depth_of_discharge(&self) -> Option<u16> {
        self.depth_of_discharge_callback
            .map(|callback| (callback)(self.context))
    }

    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    fn module_so_h(&self) -> Option<u16> {
        self.module_so_h_callback
            .map(|callback| (callback)(self.context))
    }

    /// Cycle Count
    ///
    /// Count of cycles executed.
    fn cycle_count(&self) -> Option<u32> {
        self.cycle_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Module Voltage
    ///
    /// Voltage of the module.
    ///
    /// Measurement.
    fn module_voltage(&self) -> u16 {
        (self.module_voltage_callback)(self.context)
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> u16 {
        (self.max_cell_voltage_callback)(self.context)
    }

    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum voltage.
    fn max_cell_voltage_cell(&self) -> Option<u16> {
        self.max_cell_voltage_cell_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> u16 {
        (self.min_cell_voltage_callback)(self.context)
    }

    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum voltage.
    fn min_cell_voltage_cell(&self) -> Option<u16> {
        self.min_cell_voltage_cell_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> u16 {
        (self.average_cell_voltage_callback)(self.context)
    }

    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    ///
    /// Measurement.
    fn max_cell_temperature(&self) -> i16 {
        (self.max_cell_temperature_callback)(self.context)
    }

    /// Max Cell Temperature Cell
    ///
    /// Cell with the maximum cell temperature.
    fn max_cell_temperature_cell(&self) -> Option<u16> {
        self.max_cell_temperature_cell_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    ///
    /// Measurement.
    fn min_cell_temperature(&self) -> i16 {
        (self.min_cell_temperature_callback)(self.context)
    }

    /// Min Cell Temperature Cell
    ///
    /// Cell with the minimum cell temperature.
    fn min_cell_temperature_cell(&self) -> Option<u16> {
        self.min_cell_temperature_cell_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    ///
    /// Calculation based on measurements.
    fn average_cell_temperature(&self) -> i16 {
        (self.average_cell_temperature_callback)(self.context)
    }

    /// Balanced Cell Count
    ///
    /// Number of cells currently being balanced in the module.
    fn balanced_cell_count(&self) -> Option<u16> {
        self.balanced_cell_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Serial Number
    ///
    /// Serial number for the module.
    fn serial_number(&self) -> Option<&CStr> {
        self.serial_number_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Scale factor for module state of charge.
    fn so_c_sf(&self) -> Option<u16> {
        self.so_c_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for module state of health.
    fn so_h_sf(&self) -> Option<u16> {
        self.so_h_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for module depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        self.do_d_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for module voltage.
    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)(self.context)
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        (self.cell_v_sf_callback)(self.context)
    }

    /// Scale factor for module temperature.
    fn tmp_sf(&self) -> u16 {
        (self.tmp_sf_callback)(self.context)
    }

    /// Cell Voltage
    ///
    /// Cell terminal voltage.
    fn lithium_ion_module_cell_cell_voltage(&self) -> u16 {
        (self.lithium_ion_module_cell_cell_voltage_callback)(self.context)
    }

    /// Cell Temperature
    ///
    /// Cell temperature.
    fn lithium_ion_module_cell_cell_temperature(&self) -> i16 {
        (self.lithium_ion_module_cell_cell_temperature_callback)(self.context)
    }

    /// Cell Status
    ///
    /// Status of the cell.
    fn lithium_ion_module_cell_cell_status(&self) -> Option<u32> {
        self.lithium_ion_module_cell_cell_status_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model805StatefulAdapter {
    string_index: u16,
    module_index: u16,
    module_cell_count: u16,
    module_so_c: u16,
    depth_of_discharge: u16,
    module_so_h: u16,
    cycle_count: u32,
    module_voltage: u16,
    max_cell_voltage: u16,
    max_cell_voltage_cell: u16,
    min_cell_voltage: u16,
    min_cell_voltage_cell: u16,
    average_cell_voltage: u16,
    max_cell_temperature: i16,
    max_cell_temperature_cell: u16,
    min_cell_temperature: i16,
    min_cell_temperature_cell: u16,
    average_cell_temperature: i16,
    balanced_cell_count: u16,
    serial_number: [c_char; 32],
    so_c_sf: u16,
    so_h_sf: u16,
    do_d_sf: u16,
    v_sf: u16,
    cell_v_sf: u16,
    tmp_sf: u16,
    lithium_ion_module_cell_cell_voltage: u16,
    lithium_ion_module_cell_cell_temperature: i16,
    lithium_ion_module_cell_cell_status: u32,
}

impl ModelAdapter for Model805StatefulAdapter {
    /// String Index
    ///
    /// Index of the string containing the module.
    ///
    /// Indices are one-based.
    fn string_index(&self) -> u16 {
        self.string_index
    }

    /// Module Index
    ///
    /// Index of the module within the string.
    ///
    /// Indices are one-based.
    fn module_index(&self) -> u16 {
        self.module_index
    }

    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    fn module_cell_count(&self) -> u16 {
        self.module_cell_count
    }

    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    fn module_so_c(&self) -> Option<u16> {
        Some(self.module_so_c)
    }

    /// Depth of Discharge
    ///
    /// Depth of discharge for the module.
    ///
    /// Measurement.
    fn depth_of_discharge(&self) -> Option<u16> {
        Some(self.depth_of_discharge)
    }

    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    fn module_so_h(&self) -> Option<u16> {
        Some(self.module_so_h)
    }

    /// Cycle Count
    ///
    /// Count of cycles executed.
    fn cycle_count(&self) -> Option<u32> {
        Some(self.cycle_count)
    }

    /// Module Voltage
    ///
    /// Voltage of the module.
    ///
    /// Measurement.
    fn module_voltage(&self) -> u16 {
        self.module_voltage
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> u16 {
        self.max_cell_voltage
    }

    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum voltage.
    fn max_cell_voltage_cell(&self) -> Option<u16> {
        Some(self.max_cell_voltage_cell)
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> u16 {
        self.min_cell_voltage
    }

    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum voltage.
    fn min_cell_voltage_cell(&self) -> Option<u16> {
        Some(self.min_cell_voltage_cell)
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> u16 {
        self.average_cell_voltage
    }

    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    ///
    /// Measurement.
    fn max_cell_temperature(&self) -> i16 {
        self.max_cell_temperature
    }

    /// Max Cell Temperature Cell
    ///
    /// Cell with the maximum cell temperature.
    fn max_cell_temperature_cell(&self) -> Option<u16> {
        Some(self.max_cell_temperature_cell)
    }

    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    ///
    /// Measurement.
    fn min_cell_temperature(&self) -> i16 {
        self.min_cell_temperature
    }

    /// Min Cell Temperature Cell
    ///
    /// Cell with the minimum cell temperature.
    fn min_cell_temperature_cell(&self) -> Option<u16> {
        Some(self.min_cell_temperature_cell)
    }

    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    ///
    /// Calculation based on measurements.
    fn average_cell_temperature(&self) -> i16 {
        self.average_cell_temperature
    }

    /// Balanced Cell Count
    ///
    /// Number of cells currently being balanced in the module.
    fn balanced_cell_count(&self) -> Option<u16> {
        Some(self.balanced_cell_count)
    }

    /// Serial Number
    ///
    /// Serial number for the module.
    fn serial_number(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.serial_number.as_ptr()) })
    }

    /// Scale factor for module state of charge.
    fn so_c_sf(&self) -> Option<u16> {
        Some(self.so_c_sf)
    }

    /// Scale factor for module state of health.
    fn so_h_sf(&self) -> Option<u16> {
        Some(self.so_h_sf)
    }

    /// Scale factor for module depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        Some(self.do_d_sf)
    }

    /// Scale factor for module voltage.
    fn v_sf(&self) -> u16 {
        self.v_sf
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        self.cell_v_sf
    }

    /// Scale factor for module temperature.
    fn tmp_sf(&self) -> u16 {
        self.tmp_sf
    }

    /// Cell Voltage
    ///
    /// Cell terminal voltage.
    fn lithium_ion_module_cell_cell_voltage(&self) -> u16 {
        self.lithium_ion_module_cell_cell_voltage
    }

    /// Cell Temperature
    ///
    /// Cell temperature.
    fn lithium_ion_module_cell_cell_temperature(&self) -> i16 {
        self.lithium_ion_module_cell_cell_temperature
    }

    /// Cell Status
    ///
    /// Status of the cell.
    fn lithium_ion_module_cell_cell_status(&self) -> Option<u32> {
        Some(self.lithium_ion_module_cell_cell_status)
    }
}
