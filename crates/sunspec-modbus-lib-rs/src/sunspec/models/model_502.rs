use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 30;

pub static POINTS: [ReadablePoint; 23] = [
    ReadablePoint {
        reference: PointReference::Static { value: 502 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 { point: Point::WSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 { point: Point::WhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::Status,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::VendorStatus,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::Events,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::VendorModuleEventFlags,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::Control,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::VendorControl,
        },
        size: 2,
        data_type: PointType::Enum32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::ControlValue,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::Timestamp,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::OutputCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::OutputVoltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::OutputEnergy,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::OutputPower,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 { point: Point::Temp },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::InputCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::InputVoltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::InputEnergy,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model502 {
            point: Point::InputPower,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    ASf,
    VSf,
    WSf,
    WhSf,
    Status,
    VendorStatus,
    Events,
    VendorModuleEventFlags,
    Control,
    VendorControl,
    ControlValue,
    Timestamp,
    OutputCurrent,
    OutputVoltage,
    OutputEnergy,
    OutputPower,
    Temp,
    InputCurrent,
    InputVoltage,
    InputEnergy,
    InputPower,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    30
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
        Point::ASf => {
            if let Some(value) = model.a_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VSf => {
            if let Some(value) = model.v_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WSf => {
            if let Some(value) = model.w_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WhSf => {
            if let Some(value) = model.wh_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Status => {
            buffer::write_u16(model.status() as u16, buffer);
        }
        Point::VendorStatus => {
            if let Some(value) = model.vendor_status() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Events => {
            buffer::write_u32(model.events(), buffer, offset, limit);
        }
        Point::VendorModuleEventFlags => {
            if let Some(value) = model.vendor_module_event_flags() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Control => {
            if let Some(value) = model.control() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VendorControl => {
            if let Some(value) = model.vendor_control() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ControlValue => {
            if let Some(value) = model.control_value() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Timestamp => {
            if let Some(value) = model.timestamp() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputCurrent => {
            if let Some(value) = model.output_current() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputVoltage => {
            if let Some(value) = model.output_voltage() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputEnergy => {
            if let Some(value) = model.output_energy() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputPower => {
            if let Some(value) = model.output_power() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputCurrent => {
            if let Some(value) = model.input_current() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputVoltage => {
            if let Some(value) = model.input_voltage() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputEnergy => {
            if let Some(value) = model.input_energy() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputPower => {
            if let Some(value) = model.input_power() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Current scale factor
    fn a_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage scale factor
    fn v_sf(&self) -> Option<u16> {
        None
    }

    /// Power scale factor
    fn w_sf(&self) -> Option<u16> {
        None
    }

    /// Energy scale factor
    fn wh_sf(&self) -> Option<u16> {
        None
    }

    /// Status
    ///
    /// Module Status Code
    fn status(&self) -> Stat;

    /// Vendor Status
    ///
    /// Module Vendor Status Code
    fn vendor_status(&self) -> Option<u16> {
        None
    }

    /// Events
    ///
    /// Module Event Flags
    fn events(&self) -> u32;

    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    fn vendor_module_event_flags(&self) -> Option<u32> {
        None
    }

    /// Control
    ///
    /// Module Control
    fn control(&self) -> Option<u16> {
        None
    }

    /// Control
    ///
    /// Module Control
    fn set_control(&mut self, value: u16) {}

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn vendor_control(&self) -> Option<u32> {
        None
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn set_vendor_control(&mut self, value: u32) {}

    /// Control Value
    ///
    /// Module Control Value
    fn control_value(&self) -> Option<i32> {
        None
    }

    /// Control Value
    ///
    /// Module Control Value
    fn set_control_value(&mut self, value: i32) {}

    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    fn timestamp(&self) -> Option<u32> {
        None
    }

    /// Output Current
    ///
    /// Output Current
    fn output_current(&self) -> Option<i16> {
        None
    }

    /// Output Voltage
    ///
    /// Output Voltage
    fn output_voltage(&self) -> Option<i16> {
        None
    }

    /// Output Energy
    ///
    /// Output Energy
    fn output_energy(&self) -> Option<u32> {
        None
    }

    /// Output Power
    ///
    /// Output Power
    fn output_power(&self) -> Option<i16> {
        None
    }

    /// Temp
    ///
    /// Module Temperature
    fn temp(&self) -> Option<i16> {
        None
    }

    /// Input Current
    ///
    /// Input Current
    fn input_current(&self) -> Option<i16> {
        None
    }

    /// Input Voltage
    ///
    /// Input Voltage
    fn input_voltage(&self) -> Option<i16> {
        None
    }

    /// Input Energy
    ///
    /// Input Energy
    fn input_energy(&self) -> Option<u32> {
        None
    }

    /// Input Power
    ///
    /// Input Power
    fn input_power(&self) -> Option<i16> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Stat {
    Off = 1,
    Sleeping = 2,
    Starting = 3,
    Mppt = 4,
    Throttled = 5,
    ShuttingDown = 6,
    Fault = 7,
    Standby = 8,
    Test = 9,
    Other = 10,
}

#[repr(C)]
pub struct Model502CallbackAdapter {
    context: *mut c_void,
    a_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    v_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    w_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    wh_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    status_callback: extern "C" fn(*const c_void) -> Stat,
    vendor_status_callback: Option<extern "C" fn(*const c_void) -> u16>,
    events_callback: extern "C" fn(*const c_void) -> u32,
    vendor_module_event_flags_callback: Option<extern "C" fn(*const c_void) -> u32>,
    control_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_control_callback: Option<extern "C" fn(u16, *mut c_void)>,
    vendor_control_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_vendor_control_callback: Option<extern "C" fn(u32, *mut c_void)>,
    control_value_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_control_value_callback: Option<extern "C" fn(i32, *mut c_void)>,
    timestamp_callback: Option<extern "C" fn(*const c_void) -> u32>,
    output_current_callback: Option<extern "C" fn(*const c_void) -> i16>,
    output_voltage_callback: Option<extern "C" fn(*const c_void) -> i16>,
    output_energy_callback: Option<extern "C" fn(*const c_void) -> u32>,
    output_power_callback: Option<extern "C" fn(*const c_void) -> i16>,
    temp_callback: Option<extern "C" fn(*const c_void) -> i16>,
    input_current_callback: Option<extern "C" fn(*const c_void) -> i16>,
    input_voltage_callback: Option<extern "C" fn(*const c_void) -> i16>,
    input_energy_callback: Option<extern "C" fn(*const c_void) -> u32>,
    input_power_callback: Option<extern "C" fn(*const c_void) -> i16>,
}

impl ModelAdapter for Model502CallbackAdapter {
    /// Current scale factor
    fn a_sf(&self) -> Option<u16> {
        self.a_sf_callback.map(|callback| (callback)(self.context))
    }

    /// Voltage scale factor
    fn v_sf(&self) -> Option<u16> {
        self.v_sf_callback.map(|callback| (callback)(self.context))
    }

    /// Power scale factor
    fn w_sf(&self) -> Option<u16> {
        self.w_sf_callback.map(|callback| (callback)(self.context))
    }

    /// Energy scale factor
    fn wh_sf(&self) -> Option<u16> {
        self.wh_sf_callback.map(|callback| (callback)(self.context))
    }

    /// Status
    ///
    /// Module Status Code
    fn status(&self) -> Stat {
        (self.status_callback)(self.context)
    }

    /// Vendor Status
    ///
    /// Module Vendor Status Code
    fn vendor_status(&self) -> Option<u16> {
        self.vendor_status_callback
            .map(|callback| (callback)(self.context))
    }

    /// Events
    ///
    /// Module Event Flags
    fn events(&self) -> u32 {
        (self.events_callback)(self.context)
    }

    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    fn vendor_module_event_flags(&self) -> Option<u32> {
        self.vendor_module_event_flags_callback
            .map(|callback| (callback)(self.context))
    }

    /// Control
    ///
    /// Module Control
    fn control(&self) -> Option<u16> {
        self.control_callback
            .map(|callback| (callback)(self.context))
    }

    /// Control
    ///
    /// Module Control
    fn set_control(&mut self, value: u16) {
        if let Some(callback) = self.set_control_callback {
            (callback)(value, self.context);
        };
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn vendor_control(&self) -> Option<u32> {
        self.vendor_control_callback
            .map(|callback| (callback)(self.context))
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn set_vendor_control(&mut self, value: u32) {
        if let Some(callback) = self.set_vendor_control_callback {
            (callback)(value, self.context);
        };
    }

    /// Control Value
    ///
    /// Module Control Value
    fn control_value(&self) -> Option<i32> {
        self.control_value_callback
            .map(|callback| (callback)(self.context))
    }

    /// Control Value
    ///
    /// Module Control Value
    fn set_control_value(&mut self, value: i32) {
        if let Some(callback) = self.set_control_value_callback {
            (callback)(value, self.context);
        };
    }

    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    fn timestamp(&self) -> Option<u32> {
        self.timestamp_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Current
    ///
    /// Output Current
    fn output_current(&self) -> Option<i16> {
        self.output_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Voltage
    ///
    /// Output Voltage
    fn output_voltage(&self) -> Option<i16> {
        self.output_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Energy
    ///
    /// Output Energy
    fn output_energy(&self) -> Option<u32> {
        self.output_energy_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Power
    ///
    /// Output Power
    fn output_power(&self) -> Option<i16> {
        self.output_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// Temp
    ///
    /// Module Temperature
    fn temp(&self) -> Option<i16> {
        self.temp_callback.map(|callback| (callback)(self.context))
    }

    /// Input Current
    ///
    /// Input Current
    fn input_current(&self) -> Option<i16> {
        self.input_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input Voltage
    ///
    /// Input Voltage
    fn input_voltage(&self) -> Option<i16> {
        self.input_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input Energy
    ///
    /// Input Energy
    fn input_energy(&self) -> Option<u32> {
        self.input_energy_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input Power
    ///
    /// Input Power
    fn input_power(&self) -> Option<i16> {
        self.input_power_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model502StatefulAdapter {
    a_sf: u16,
    v_sf: u16,
    w_sf: u16,
    wh_sf: u16,
    status: Stat,
    vendor_status: u16,
    events: u32,
    vendor_module_event_flags: u32,
    control: u16,
    vendor_control: u32,
    control_value: i32,
    timestamp: u32,
    output_current: i16,
    output_voltage: i16,
    output_energy: u32,
    output_power: i16,
    temp: i16,
    input_current: i16,
    input_voltage: i16,
    input_energy: u32,
    input_power: i16,
}

impl ModelAdapter for Model502StatefulAdapter {
    /// Current scale factor
    fn a_sf(&self) -> Option<u16> {
        Some(self.a_sf)
    }

    /// Voltage scale factor
    fn v_sf(&self) -> Option<u16> {
        Some(self.v_sf)
    }

    /// Power scale factor
    fn w_sf(&self) -> Option<u16> {
        Some(self.w_sf)
    }

    /// Energy scale factor
    fn wh_sf(&self) -> Option<u16> {
        Some(self.wh_sf)
    }

    /// Status
    ///
    /// Module Status Code
    fn status(&self) -> Stat {
        self.status
    }

    /// Vendor Status
    ///
    /// Module Vendor Status Code
    fn vendor_status(&self) -> Option<u16> {
        Some(self.vendor_status)
    }

    /// Events
    ///
    /// Module Event Flags
    fn events(&self) -> u32 {
        self.events
    }

    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    fn vendor_module_event_flags(&self) -> Option<u32> {
        Some(self.vendor_module_event_flags)
    }

    /// Control
    ///
    /// Module Control
    fn control(&self) -> Option<u16> {
        Some(self.control)
    }

    /// Control
    ///
    /// Module Control
    fn set_control(&mut self, value: u16) {
        self.control = value;
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn vendor_control(&self) -> Option<u32> {
        Some(self.vendor_control)
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn set_vendor_control(&mut self, value: u32) {
        self.vendor_control = value;
    }

    /// Control Value
    ///
    /// Module Control Value
    fn control_value(&self) -> Option<i32> {
        Some(self.control_value)
    }

    /// Control Value
    ///
    /// Module Control Value
    fn set_control_value(&mut self, value: i32) {
        self.control_value = value;
    }

    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    fn timestamp(&self) -> Option<u32> {
        Some(self.timestamp)
    }

    /// Output Current
    ///
    /// Output Current
    fn output_current(&self) -> Option<i16> {
        Some(self.output_current)
    }

    /// Output Voltage
    ///
    /// Output Voltage
    fn output_voltage(&self) -> Option<i16> {
        Some(self.output_voltage)
    }

    /// Output Energy
    ///
    /// Output Energy
    fn output_energy(&self) -> Option<u32> {
        Some(self.output_energy)
    }

    /// Output Power
    ///
    /// Output Power
    fn output_power(&self) -> Option<i16> {
        Some(self.output_power)
    }

    /// Temp
    ///
    /// Module Temperature
    fn temp(&self) -> Option<i16> {
        Some(self.temp)
    }

    /// Input Current
    ///
    /// Input Current
    fn input_current(&self) -> Option<i16> {
        Some(self.input_current)
    }

    /// Input Voltage
    ///
    /// Input Voltage
    fn input_voltage(&self) -> Option<i16> {
        Some(self.input_voltage)
    }

    /// Input Energy
    ///
    /// Input Energy
    fn input_energy(&self) -> Option<u32> {
        Some(self.input_energy)
    }

    /// Input Power
    ///
    /// Input Power
    fn input_power(&self) -> Option<i16> {
        Some(self.input_power)
    }
}
