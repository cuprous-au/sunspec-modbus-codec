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
}

#[repr(C)]
pub struct Model803StatefulAdapter {
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
}

impl ModelAdapter for Model803StatefulAdapter {
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
}
