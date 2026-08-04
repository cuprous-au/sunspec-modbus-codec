use core::ffi::c_void;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 33;

pub static POINTS: [ReadablePoint; 19] = [
    ReadablePoint {
        reference: PointReference::Static { value: 501 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 31 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::Status },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::VendorStatus },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::Events },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::VendorModuleEventFlags },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::Control },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::VendorControl },
        size: 2,
        data_type: PointType::Enum32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::ControlValue },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::Timestamp },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::OutputCurrent },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::OutputVoltage },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::OutputEnergy },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::OutputPower },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::Temp },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::InputCurrent },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::InputVoltage },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::InputEnergy },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model501 { point: Point::InputPower },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Status => {
            serialisation::write_u16(model.status() as u16, buffer);
        },
        Point::VendorStatus => {
            if let Some(value) = model.vendor_status() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Events => {
            serialisation::write_u32(model.events(), buffer, offset, limit);
        },
        Point::VendorModuleEventFlags => {
            if let Some(value) = model.vendor_module_event_flags() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Control => {
            if let Some(value) = model.control() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::VendorControl => {
            if let Some(value) = model.vendor_control() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::ControlValue => {
            if let Some(value) = model.control_value() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Timestamp => {
            if let Some(value) = model.timestamp() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::OutputCurrent => {
            if let Some(value) = model.output_current() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::OutputVoltage => {
            if let Some(value) = model.output_voltage() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::OutputEnergy => {
            if let Some(value) = model.output_energy() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::OutputPower => {
            if let Some(value) = model.output_power() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::InputCurrent => {
            if let Some(value) = model.input_current() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::InputVoltage => {
            if let Some(value) = model.input_voltage() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::InputEnergy => {
            if let Some(value) = model.input_energy() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::InputPower => {
            if let Some(value) = model.input_power() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
            else {
                buffer.fill(0)
            }
        }
    }
}

pub trait ModelAdapter {
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
    fn set_control(&mut self, value: u16) {
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn vendor_control(&self) -> Option<u32> {
        None
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn set_vendor_control(&mut self, value: u32) {
    }

    /// Control Value
    ///
    /// Module Control Value
    fn control_value(&self) -> Option<i32> {
        None
    }

    /// Control Value
    ///
    /// Module Control Value
    fn set_control_value(&mut self, value: i32) {
    }

    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    fn timestamp(&self) -> Option<u32> {
        None
    }

    /// Output Current
    ///
    /// Output Current
    fn output_current(&self) -> Option<f32> {
        None
    }

    /// Output Voltage
    ///
    /// Output Voltage
    fn output_voltage(&self) -> Option<f32> {
        None
    }

    /// Output Energy
    ///
    /// Output Energy
    fn output_energy(&self) -> Option<f32> {
        None
    }

    /// Output Power
    ///
    /// Output Power
    fn output_power(&self) -> Option<f32> {
        None
    }

    /// Temp
    ///
    /// Module Temperature
    fn temp(&self) -> Option<f32> {
        None
    }

    /// Input Current
    ///
    /// Input Current
    fn input_current(&self) -> Option<f32> {
        None
    }

    /// Input Voltage
    ///
    /// Input Voltage
    fn input_voltage(&self) -> Option<f32> {
        None
    }

    /// Input Energy
    ///
    /// Input Energy
    fn input_energy(&self) -> Option<f32> {
        None
    }

    /// Input Power
    ///
    /// Input Power
    fn input_power(&self) -> Option<f32> {
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
pub struct Model501CallbackAdapter {
    context: *mut c_void,
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
    output_current_callback: Option<extern "C" fn(*const c_void) -> f32>,
    output_voltage_callback: Option<extern "C" fn(*const c_void) -> f32>,
    output_energy_callback: Option<extern "C" fn(*const c_void) -> f32>,
    output_power_callback: Option<extern "C" fn(*const c_void) -> f32>,
    temp_callback: Option<extern "C" fn(*const c_void) -> f32>,
    input_current_callback: Option<extern "C" fn(*const c_void) -> f32>,
    input_voltage_callback: Option<extern "C" fn(*const c_void) -> f32>,
    input_energy_callback: Option<extern "C" fn(*const c_void) -> f32>,
    input_power_callback: Option<extern "C" fn(*const c_void) -> f32>,
}

impl ModelAdapter for Model501CallbackAdapter {
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
        self.vendor_status_callback.map(|callback| {
        (callback)(self.context)
        })
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
        self.vendor_module_event_flags_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Control
    ///
    /// Module Control
    fn control(&self) -> Option<u16> {
        self.control_callback.map(|callback| {
        (callback)(self.context)
        })
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
        self.vendor_control_callback.map(|callback| {
        (callback)(self.context)
        })
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
        self.control_value_callback.map(|callback| {
        (callback)(self.context)
        })
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
        self.timestamp_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Output Current
    ///
    /// Output Current
    fn output_current(&self) -> Option<f32> {
        self.output_current_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Output Voltage
    ///
    /// Output Voltage
    fn output_voltage(&self) -> Option<f32> {
        self.output_voltage_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Output Energy
    ///
    /// Output Energy
    fn output_energy(&self) -> Option<f32> {
        self.output_energy_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Output Power
    ///
    /// Output Power
    fn output_power(&self) -> Option<f32> {
        self.output_power_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Temp
    ///
    /// Module Temperature
    fn temp(&self) -> Option<f32> {
        self.temp_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Input Current
    ///
    /// Input Current
    fn input_current(&self) -> Option<f32> {
        self.input_current_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Input Voltage
    ///
    /// Input Voltage
    fn input_voltage(&self) -> Option<f32> {
        self.input_voltage_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Input Energy
    ///
    /// Input Energy
    fn input_energy(&self) -> Option<f32> {
        self.input_energy_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Input Power
    ///
    /// Input Power
    fn input_power(&self) -> Option<f32> {
        self.input_power_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model501StatefulAdapter {
    status: Stat,
    vendor_status: u16,
    events: u32,
    vendor_module_event_flags: u32,
    control: u16,
    vendor_control: u32,
    control_value: i32,
    timestamp: u32,
    output_current: f32,
    output_voltage: f32,
    output_energy: f32,
    output_power: f32,
    temp: f32,
    input_current: f32,
    input_voltage: f32,
    input_energy: f32,
    input_power: f32,
}

impl ModelAdapter for Model501StatefulAdapter {
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
        Some(
        self.vendor_status
        )
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
        Some(
        self.vendor_module_event_flags
        )
    }

    /// Control
    ///
    /// Module Control
    fn control(&self) -> Option<u16> {
        Some(
        self.control
        )
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
        Some(
        self.vendor_control
        )
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
        Some(
        self.control_value
        )
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
        Some(
        self.timestamp
        )
    }

    /// Output Current
    ///
    /// Output Current
    fn output_current(&self) -> Option<f32> {
        Some(
        self.output_current
        )
    }

    /// Output Voltage
    ///
    /// Output Voltage
    fn output_voltage(&self) -> Option<f32> {
        Some(
        self.output_voltage
        )
    }

    /// Output Energy
    ///
    /// Output Energy
    fn output_energy(&self) -> Option<f32> {
        Some(
        self.output_energy
        )
    }

    /// Output Power
    ///
    /// Output Power
    fn output_power(&self) -> Option<f32> {
        Some(
        self.output_power
        )
    }

    /// Temp
    ///
    /// Module Temperature
    fn temp(&self) -> Option<f32> {
        Some(
        self.temp
        )
    }

    /// Input Current
    ///
    /// Input Current
    fn input_current(&self) -> Option<f32> {
        Some(
        self.input_current
        )
    }

    /// Input Voltage
    ///
    /// Input Voltage
    fn input_voltage(&self) -> Option<f32> {
        Some(
        self.input_voltage
        )
    }

    /// Input Energy
    ///
    /// Input Energy
    fn input_energy(&self) -> Option<f32> {
        Some(
        self.input_energy
        )
    }

    /// Input Power
    ///
    /// Input Power
    fn input_power(&self) -> Option<f32> {
        Some(
        self.input_power
        )
    }
}