use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 30;

pub static POINTS: [ReadablePoint; 19] = [
    ReadablePoint {
        reference: PointReference::Static { value: 160 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::CurrentScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::PowerScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::EnergyScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::GlobalEvents,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::NumberOfModules,
        },
        size: 1,
        data_type: PointType::Count,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::TimestampPeriod,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleInputId,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleInputIdString,
        },
        size: 8,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleDcCurrent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleDcVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleDcPower,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleLifetimeEnergy,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleTimestamp,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleOperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model160 {
            point: Point::ModuleModuleEvents,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    CurrentScaleFactor,
    VoltageScaleFactor,
    PowerScaleFactor,
    EnergyScaleFactor,
    GlobalEvents,
    NumberOfModules,
    TimestampPeriod,
    ModuleInputId,
    ModuleInputIdString,
    ModuleDcCurrent,
    ModuleDcVoltage,
    ModuleDcPower,
    ModuleLifetimeEnergy,
    ModuleTimestamp,
    ModuleTemperature,
    ModuleOperatingState,
    ModuleModuleEvents,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    30
}

pub fn write_point<'a, 'b>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::CurrentScaleFactor => {
            if let Some(value) = model.current_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageScaleFactor => {
            if let Some(value) = model.voltage_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerScaleFactor => {
            if let Some(value) = model.power_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnergyScaleFactor => {
            if let Some(value) = model.energy_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::GlobalEvents => {
            if let Some(value) = model.global_events() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NumberOfModules => {
            if let Some(value) = model.number_of_modules() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TimestampPeriod => {
            if let Some(value) = model.timestamp_period() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleInputId => {
            if let Some(value) = model.module_input_id() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleInputIdString => {
            if let Some(value) = model.module_input_id_string() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleDcCurrent => {
            if let Some(value) = model.module_dc_current() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleDcVoltage => {
            if let Some(value) = model.module_dc_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleDcPower => {
            if let Some(value) = model.module_dc_power() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleLifetimeEnergy => {
            if let Some(value) = model.module_lifetime_energy() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleTimestamp => {
            if let Some(value) = model.module_timestamp() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleTemperature => {
            if let Some(value) = model.module_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleOperatingState => {
            if let Some(value) = model.module_operating_state() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ModuleModuleEvents => {
            if let Some(value) = model.module_module_events() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Current Scale Factor
    fn current_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Voltage Scale Factor
    fn voltage_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Power Scale Factor
    fn power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Energy Scale Factor
    fn energy_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Global Events
    fn global_events(&self) -> Option<u32> {
        None
    }

    /// Number of Modules
    fn number_of_modules(&self) -> Option<u16> {
        None
    }

    /// Timestamp Period
    fn timestamp_period(&self) -> Option<u16> {
        None
    }

    /// Input ID
    fn module_input_id(&self) -> Option<u16> {
        None
    }

    /// Input ID String
    fn module_input_id_string(&self) -> Option<&CStr> {
        None
    }

    /// DC Current
    fn module_dc_current(&self) -> Option<u16> {
        None
    }

    /// DC Voltage
    fn module_dc_voltage(&self) -> Option<u16> {
        None
    }

    /// DC Power
    fn module_dc_power(&self) -> Option<u16> {
        None
    }

    /// Lifetime Energy
    fn module_lifetime_energy(&self) -> Option<u32> {
        None
    }

    /// Timestamp
    fn module_timestamp(&self) -> Option<u32> {
        None
    }

    /// Temperature
    fn module_temperature(&self) -> Option<i16> {
        None
    }

    /// Operating State
    fn module_operating_state(&self) -> Option<DcSt> {
        None
    }

    /// Module Events
    fn module_module_events(&self) -> Option<u32> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum DcSt {
    Off = 1,
    Sleeping = 2,
    Starting = 3,
    Mppt = 4,
    Throttled = 5,
    ShuttingDown = 6,
    Fault = 7,
    Standby = 8,
    Test = 9,
    Reserved10 = 10,
}

#[repr(C)]
pub struct Model160CallbackAdapter {
    context: *mut c_void,
    current_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    voltage_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    energy_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    global_events_callback: Option<extern "C" fn(*const c_void) -> u32>,
    number_of_modules_callback: Option<extern "C" fn(*const c_void) -> u16>,
    timestamp_period_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_input_id_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_input_id_string_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    module_dc_current_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_dc_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_dc_power_callback: Option<extern "C" fn(*const c_void) -> u16>,
    module_lifetime_energy_callback: Option<extern "C" fn(*const c_void) -> u32>,
    module_timestamp_callback: Option<extern "C" fn(*const c_void) -> u32>,
    module_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    module_operating_state_callback: Option<extern "C" fn(*const c_void) -> DcSt>,
    module_module_events_callback: Option<extern "C" fn(*const c_void) -> u32>,
}

impl ModelAdapter for Model160CallbackAdapter {
    /// Current Scale Factor
    fn current_scale_factor(&self) -> Option<u16> {
        self.current_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Scale Factor
    fn voltage_scale_factor(&self) -> Option<u16> {
        self.voltage_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power Scale Factor
    fn power_scale_factor(&self) -> Option<u16> {
        self.power_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Energy Scale Factor
    fn energy_scale_factor(&self) -> Option<u16> {
        self.energy_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Global Events
    fn global_events(&self) -> Option<u32> {
        self.global_events_callback
            .map(|callback| (callback)(self.context))
    }

    /// Number of Modules
    fn number_of_modules(&self) -> Option<u16> {
        self.number_of_modules_callback
            .map(|callback| (callback)(self.context))
    }

    /// Timestamp Period
    fn timestamp_period(&self) -> Option<u16> {
        self.timestamp_period_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input ID
    fn module_input_id(&self) -> Option<u16> {
        self.module_input_id_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input ID String
    fn module_input_id_string(&self) -> Option<&CStr> {
        self.module_input_id_string_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// DC Current
    fn module_dc_current(&self) -> Option<u16> {
        self.module_dc_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Voltage
    fn module_dc_voltage(&self) -> Option<u16> {
        self.module_dc_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Power
    fn module_dc_power(&self) -> Option<u16> {
        self.module_dc_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// Lifetime Energy
    fn module_lifetime_energy(&self) -> Option<u32> {
        self.module_lifetime_energy_callback
            .map(|callback| (callback)(self.context))
    }

    /// Timestamp
    fn module_timestamp(&self) -> Option<u32> {
        self.module_timestamp_callback
            .map(|callback| (callback)(self.context))
    }

    /// Temperature
    fn module_temperature(&self) -> Option<i16> {
        self.module_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Operating State
    fn module_operating_state(&self) -> Option<DcSt> {
        self.module_operating_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Module Events
    fn module_module_events(&self) -> Option<u32> {
        self.module_module_events_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model160StatefulAdapter {
    current_scale_factor: u16,
    voltage_scale_factor: u16,
    power_scale_factor: u16,
    energy_scale_factor: u16,
    global_events: u32,
    number_of_modules: u16,
    timestamp_period: u16,
    module_input_id: u16,
    module_input_id_string: [c_char; 16],
    module_dc_current: u16,
    module_dc_voltage: u16,
    module_dc_power: u16,
    module_lifetime_energy: u32,
    module_timestamp: u32,
    module_temperature: i16,
    module_operating_state: DcSt,
    module_module_events: u32,
}

impl ModelAdapter for Model160StatefulAdapter {
    /// Current Scale Factor
    fn current_scale_factor(&self) -> Option<u16> {
        Some(self.current_scale_factor)
    }

    /// Voltage Scale Factor
    fn voltage_scale_factor(&self) -> Option<u16> {
        Some(self.voltage_scale_factor)
    }

    /// Power Scale Factor
    fn power_scale_factor(&self) -> Option<u16> {
        Some(self.power_scale_factor)
    }

    /// Energy Scale Factor
    fn energy_scale_factor(&self) -> Option<u16> {
        Some(self.energy_scale_factor)
    }

    /// Global Events
    fn global_events(&self) -> Option<u32> {
        Some(self.global_events)
    }

    /// Number of Modules
    fn number_of_modules(&self) -> Option<u16> {
        Some(self.number_of_modules)
    }

    /// Timestamp Period
    fn timestamp_period(&self) -> Option<u16> {
        Some(self.timestamp_period)
    }

    /// Input ID
    fn module_input_id(&self) -> Option<u16> {
        Some(self.module_input_id)
    }

    /// Input ID String
    fn module_input_id_string(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.module_input_id_string.as_ptr()) })
    }

    /// DC Current
    fn module_dc_current(&self) -> Option<u16> {
        Some(self.module_dc_current)
    }

    /// DC Voltage
    fn module_dc_voltage(&self) -> Option<u16> {
        Some(self.module_dc_voltage)
    }

    /// DC Power
    fn module_dc_power(&self) -> Option<u16> {
        Some(self.module_dc_power)
    }

    /// Lifetime Energy
    fn module_lifetime_energy(&self) -> Option<u32> {
        Some(self.module_lifetime_energy)
    }

    /// Timestamp
    fn module_timestamp(&self) -> Option<u32> {
        Some(self.module_timestamp)
    }

    /// Temperature
    fn module_temperature(&self) -> Option<i16> {
        Some(self.module_temperature)
    }

    /// Operating State
    fn module_operating_state(&self) -> Option<DcSt> {
        Some(self.module_operating_state)
    }

    /// Module Events
    fn module_module_events(&self) -> Option<u32> {
        Some(self.module_module_events)
    }
}
