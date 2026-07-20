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
        Point::Status => serialisation::write_u16(model.status() as u16, buffer),
        Point::VendorStatus => if let Some(value) = model.vendor_status() { serialisation::write_u16(value as u16, buffer); },
        Point::Events => serialisation::write_u32(model.events(), buffer, offset, limit),
        Point::VendorModuleEventFlags => if let Some(value) = model.vendor_module_event_flags() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::Control => if let Some(value) = model.control() { serialisation::write_u16(value as u16, buffer); },
        Point::VendorControl => if let Some(value) = model.vendor_control() { serialisation::write_u32(value as u32, buffer, offset, limit); },
        Point::ControlValue => if let Some(value) = model.control_value() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::Timestamp => if let Some(value) = model.timestamp() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::OutputCurrent => if let Some(value) = model.output_current() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::OutputVoltage => if let Some(value) = model.output_voltage() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::OutputEnergy => if let Some(value) = model.output_energy() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::OutputPower => if let Some(value) = model.output_power() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::Temp => if let Some(value) = model.temp() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::InputCurrent => if let Some(value) = model.input_current() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::InputVoltage => if let Some(value) = model.input_voltage() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::InputEnergy => if let Some(value) = model.input_energy() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::InputPower => if let Some(value) = model.input_power() { serialisation::write_f32(value, buffer, offset, limit); },
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
    fn vendor_status(&self) -> Option<StatVend> {
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
    fn control(&self) -> Option<Ctl> {
        None
    }

    /// Control
    ///
    /// Module Control
    fn set_control(&mut self, value: Ctl) {
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn vendor_control(&self) -> Option<CtlVend> {
        None
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn set_vendor_control(&mut self, value: CtlVend) {
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

pub enum StatVend {
}

pub enum Ctl {
}

pub enum CtlVend {
}