use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 30;

pub static POINTS: [ReadablePoint; 23] = [
    ReadablePoint {
        reference: PointReference::Static { value: 502 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 28 },
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ASf => {
            if let Some(value) = model.a_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VSf => {
            if let Some(value) = model.v_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WSf => {
            if let Some(value) = model.w_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WhSf => {
            if let Some(value) = model.wh_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Status => serialisation::write_u16(model.status() as u16, buffer),
        Point::VendorStatus => {
            if let Some(value) = model.vendor_status() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Events => serialisation::write_u32(model.events(), buffer, offset, limit),
        Point::VendorModuleEventFlags => {
            if let Some(value) = model.vendor_module_event_flags() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::Control => {
            if let Some(value) = model.control() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VendorControl => {
            if let Some(value) = model.vendor_control() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ControlValue => {
            if let Some(value) = model.control_value() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::Timestamp => {
            if let Some(value) = model.timestamp() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::OutputCurrent => {
            if let Some(value) = model.output_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::OutputVoltage => {
            if let Some(value) = model.output_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::OutputEnergy => {
            if let Some(value) = model.output_energy() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::OutputPower => {
            if let Some(value) = model.output_power() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::InputCurrent => {
            if let Some(value) = model.input_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::InputVoltage => {
            if let Some(value) = model.input_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::InputEnergy => {
            if let Some(value) = model.input_energy() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::InputPower => {
            if let Some(value) = model.input_power() {
                serialisation::write_i16(value, buffer);
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
    a_sf_callback: Option<extern "C" fn() -> u16>,
    v_sf_callback: Option<extern "C" fn() -> u16>,
    w_sf_callback: Option<extern "C" fn() -> u16>,
    wh_sf_callback: Option<extern "C" fn() -> u16>,
    status_callback: extern "C" fn() -> Stat,
    vendor_status_callback: Option<extern "C" fn() -> u16>,
    events_callback: extern "C" fn() -> u32,
    vendor_module_event_flags_callback: Option<extern "C" fn() -> u32>,
    control_callback: Option<extern "C" fn() -> u16>,
    set_control_callback: Option<extern "C" fn(u16)>,
    vendor_control_callback: Option<extern "C" fn() -> u32>,
    set_vendor_control_callback: Option<extern "C" fn(u32)>,
    control_value_callback: Option<extern "C" fn() -> i32>,
    set_control_value_callback: Option<extern "C" fn(i32)>,
    timestamp_callback: Option<extern "C" fn() -> u32>,
    output_current_callback: Option<extern "C" fn() -> i16>,
    output_voltage_callback: Option<extern "C" fn() -> i16>,
    output_energy_callback: Option<extern "C" fn() -> u32>,
    output_power_callback: Option<extern "C" fn() -> i16>,
    temp_callback: Option<extern "C" fn() -> i16>,
    input_current_callback: Option<extern "C" fn() -> i16>,
    input_voltage_callback: Option<extern "C" fn() -> i16>,
    input_energy_callback: Option<extern "C" fn() -> u32>,
    input_power_callback: Option<extern "C" fn() -> i16>,
}

impl ModelAdapter for Model502CallbackAdapter {
    /// Current scale factor
    fn a_sf(&self) -> Option<u16> {
        self.a_sf_callback.map(|callback| (callback)())
    }

    /// Voltage scale factor
    fn v_sf(&self) -> Option<u16> {
        self.v_sf_callback.map(|callback| (callback)())
    }

    /// Power scale factor
    fn w_sf(&self) -> Option<u16> {
        self.w_sf_callback.map(|callback| (callback)())
    }

    /// Energy scale factor
    fn wh_sf(&self) -> Option<u16> {
        self.wh_sf_callback.map(|callback| (callback)())
    }

    /// Status
    ///
    /// Module Status Code
    fn status(&self) -> Stat {
        (self.status_callback)()
    }

    /// Vendor Status
    ///
    /// Module Vendor Status Code
    fn vendor_status(&self) -> Option<u16> {
        self.vendor_status_callback.map(|callback| (callback)())
    }

    /// Events
    ///
    /// Module Event Flags
    fn events(&self) -> u32 {
        (self.events_callback)()
    }

    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    fn vendor_module_event_flags(&self) -> Option<u32> {
        self.vendor_module_event_flags_callback
            .map(|callback| (callback)())
    }

    /// Control
    ///
    /// Module Control
    fn control(&self) -> Option<u16> {
        self.control_callback.map(|callback| (callback)())
    }

    /// Control
    ///
    /// Module Control
    fn set_control(&mut self, value: u16) {
        if let Some(callback) = self.set_control_callback {
            (callback)(value);
        };
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn vendor_control(&self) -> Option<u32> {
        self.vendor_control_callback.map(|callback| (callback)())
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn set_vendor_control(&mut self, value: u32) {
        if let Some(callback) = self.set_vendor_control_callback {
            (callback)(value);
        };
    }

    /// Control Value
    ///
    /// Module Control Value
    fn control_value(&self) -> Option<i32> {
        self.control_value_callback.map(|callback| (callback)())
    }

    /// Control Value
    ///
    /// Module Control Value
    fn set_control_value(&mut self, value: i32) {
        if let Some(callback) = self.set_control_value_callback {
            (callback)(value);
        };
    }

    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    fn timestamp(&self) -> Option<u32> {
        self.timestamp_callback.map(|callback| (callback)())
    }

    /// Output Current
    ///
    /// Output Current
    fn output_current(&self) -> Option<i16> {
        self.output_current_callback.map(|callback| (callback)())
    }

    /// Output Voltage
    ///
    /// Output Voltage
    fn output_voltage(&self) -> Option<i16> {
        self.output_voltage_callback.map(|callback| (callback)())
    }

    /// Output Energy
    ///
    /// Output Energy
    fn output_energy(&self) -> Option<u32> {
        self.output_energy_callback.map(|callback| (callback)())
    }

    /// Output Power
    ///
    /// Output Power
    fn output_power(&self) -> Option<i16> {
        self.output_power_callback.map(|callback| (callback)())
    }

    /// Temp
    ///
    /// Module Temperature
    fn temp(&self) -> Option<i16> {
        self.temp_callback.map(|callback| (callback)())
    }

    /// Input Current
    ///
    /// Input Current
    fn input_current(&self) -> Option<i16> {
        self.input_current_callback.map(|callback| (callback)())
    }

    /// Input Voltage
    ///
    /// Input Voltage
    fn input_voltage(&self) -> Option<i16> {
        self.input_voltage_callback.map(|callback| (callback)())
    }

    /// Input Energy
    ///
    /// Input Energy
    fn input_energy(&self) -> Option<u32> {
        self.input_energy_callback.map(|callback| (callback)())
    }

    /// Input Power
    ///
    /// Input Power
    fn input_power(&self) -> Option<i16> {
        self.input_power_callback.map(|callback| (callback)())
    }
}
