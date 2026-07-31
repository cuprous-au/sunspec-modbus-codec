use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 32;

pub static POINTS: [ReadablePoint; 32] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64020 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 30 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Aux0Temperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Aux1Temperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Aux2Temperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Aux3Temperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Aux4Temperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::ProbeTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::MainTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::VoltageScaleFactorForTheSensors,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::CurrentScaleFactorForTheSensors,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::FrequencyScaleFactorForTheSensors,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor1Voltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor2Voltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor3Voltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor4Voltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor5Voltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor6Voltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor7Voltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor1Current,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor2Current,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor3Current,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor4Current,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor5Current,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor6Current,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor7Current,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Sensor8Frequency,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Relay1State,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Relay2State,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::Relay3State,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::ResetTheAccumulators,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64020 {
            point: Point::ResetTheSystem,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Aux0Temperature,
    Aux1Temperature,
    Aux2Temperature,
    Aux3Temperature,
    Aux4Temperature,
    ProbeTemperature,
    MainTemperature,
    VoltageScaleFactorForTheSensors,
    CurrentScaleFactorForTheSensors,
    FrequencyScaleFactorForTheSensors,
    Sensor1Voltage,
    Sensor2Voltage,
    Sensor3Voltage,
    Sensor4Voltage,
    Sensor5Voltage,
    Sensor6Voltage,
    Sensor7Voltage,
    Sensor1Current,
    Sensor2Current,
    Sensor3Current,
    Sensor4Current,
    Sensor5Current,
    Sensor6Current,
    Sensor7Current,
    Sensor8Frequency,
    Relay1State,
    Relay2State,
    Relay3State,
    ResetTheAccumulators,
    ResetTheSystem,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Aux0Temperature => {
            if let Some(value) = model.aux_0_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Aux1Temperature => {
            if let Some(value) = model.aux_1_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Aux2Temperature => {
            if let Some(value) = model.aux_2_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Aux3Temperature => {
            if let Some(value) = model.aux_3_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Aux4Temperature => {
            if let Some(value) = model.aux_4_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::ProbeTemperature => serialisation::write_i16(model.probe_temperature(), buffer),
        Point::MainTemperature => serialisation::write_i16(model.main_temperature(), buffer),
        Point::VoltageScaleFactorForTheSensors => {
            serialisation::write_u16(model.voltage_scale_factor_for_the_sensors(), buffer)
        }
        Point::CurrentScaleFactorForTheSensors => {
            serialisation::write_u16(model.current_scale_factor_for_the_sensors(), buffer)
        }
        Point::FrequencyScaleFactorForTheSensors => {
            serialisation::write_u16(model.frequency_scale_factor_for_the_sensors(), buffer)
        }
        Point::Sensor1Voltage => {
            if let Some(value) = model.sensor1_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor2Voltage => {
            if let Some(value) = model.sensor2_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor3Voltage => {
            if let Some(value) = model.sensor3_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor4Voltage => {
            if let Some(value) = model.sensor4_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor5Voltage => {
            if let Some(value) = model.sensor5_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor6Voltage => {
            if let Some(value) = model.sensor6_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor7Voltage => {
            if let Some(value) = model.sensor7_voltage() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor1Current => {
            if let Some(value) = model.sensor1_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor2Current => {
            if let Some(value) = model.sensor2_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor3Current => {
            if let Some(value) = model.sensor3_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor4Current => {
            if let Some(value) = model.sensor4_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor5Current => {
            if let Some(value) = model.sensor5_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor6Current => {
            if let Some(value) = model.sensor6_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor7Current => {
            if let Some(value) = model.sensor7_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Sensor8Frequency => {
            if let Some(value) = model.sensor8_frequency() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Relay1State => {
            if let Some(value) = model.relay_1_state() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Relay2State => {
            if let Some(value) = model.relay_2_state() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Relay3State => {
            if let Some(value) = model.relay_3_state() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ResetTheAccumulators => {
            if let Some(value) = model.reset_the_accumulators() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ResetTheSystem => {
            if let Some(value) = model.reset_the_system() {
                serialisation::write_u16(value, buffer);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Aux 0 temperature
    fn aux_0_temperature(&self) -> Option<i16> {
        None
    }

    /// Aux 1 temperature
    fn aux_1_temperature(&self) -> Option<i16> {
        None
    }

    /// Aux 2 temperature
    fn aux_2_temperature(&self) -> Option<i16> {
        None
    }

    /// Aux 3 temperature
    fn aux_3_temperature(&self) -> Option<i16> {
        None
    }

    /// Aux 4 temperature
    fn aux_4_temperature(&self) -> Option<i16> {
        None
    }

    /// Probe Temperature
    fn probe_temperature(&self) -> i16;

    /// Main Temperature
    fn main_temperature(&self) -> i16;

    /// Voltage scale factor for the sensors
    fn voltage_scale_factor_for_the_sensors(&self) -> u16;

    /// Current scale factor for the sensors
    fn current_scale_factor_for_the_sensors(&self) -> u16;

    /// Frequency scale factor for the sensors
    fn frequency_scale_factor_for_the_sensors(&self) -> u16;

    /// Sensor1 Voltage
    ///
    /// scale of 0-10V
    fn sensor1_voltage(&self) -> Option<i16> {
        None
    }

    /// Sensor2 Voltage
    ///
    /// scale of 0-10V
    fn sensor2_voltage(&self) -> Option<i16> {
        None
    }

    /// Sensor3 Voltage
    ///
    /// scale of 0-10V
    fn sensor3_voltage(&self) -> Option<i16> {
        None
    }

    /// Sensor4 Voltage
    ///
    /// scale of 0-10V
    fn sensor4_voltage(&self) -> Option<i16> {
        None
    }

    /// Sensor5 Voltage
    ///
    /// scale of 0-10V
    fn sensor5_voltage(&self) -> Option<i16> {
        None
    }

    /// Sensor6 Voltage
    ///
    /// scale of 0-10V
    fn sensor6_voltage(&self) -> Option<i16> {
        None
    }

    /// Sensor7 Voltage
    ///
    /// scale of 0-10V
    fn sensor7_voltage(&self) -> Option<i16> {
        None
    }

    /// Sensor1 Current
    ///
    /// scale of 4-20mA
    fn sensor1_current(&self) -> Option<i16> {
        None
    }

    /// Sensor2 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor2_current(&self) -> Option<i16> {
        None
    }

    /// Sensor3 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor3_current(&self) -> Option<i16> {
        None
    }

    /// Sensor4 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor4_current(&self) -> Option<i16> {
        None
    }

    /// Sensor5 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor5_current(&self) -> Option<i16> {
        None
    }

    /// Sensor6 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor6_current(&self) -> Option<i16> {
        None
    }

    /// Sensor7 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor7_current(&self) -> Option<i16> {
        None
    }

    /// Sensor8 frequency
    ///
    /// frequency in Hz
    fn sensor8_frequency(&self) -> Option<u16> {
        None
    }

    /// Relay 1 state
    fn relay_1_state(&self) -> Option<u16> {
        None
    }

    /// Relay 2 state
    fn relay_2_state(&self) -> Option<u16> {
        None
    }

    /// Relay 3 state
    fn relay_3_state(&self) -> Option<u16> {
        None
    }

    /// Reset the accumulators
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting them
    fn reset_the_accumulators(&self) -> Option<u16> {
        None
    }

    /// Reset the system
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting the system
    fn reset_the_system(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model64020CallbackAdapter {
    aux_0_temperature_callback: Option<extern "C" fn() -> i16>,
    aux_1_temperature_callback: Option<extern "C" fn() -> i16>,
    aux_2_temperature_callback: Option<extern "C" fn() -> i16>,
    aux_3_temperature_callback: Option<extern "C" fn() -> i16>,
    aux_4_temperature_callback: Option<extern "C" fn() -> i16>,
    probe_temperature_callback: extern "C" fn() -> i16,
    main_temperature_callback: extern "C" fn() -> i16,
    voltage_scale_factor_for_the_sensors_callback: extern "C" fn() -> u16,
    current_scale_factor_for_the_sensors_callback: extern "C" fn() -> u16,
    frequency_scale_factor_for_the_sensors_callback: extern "C" fn() -> u16,
    sensor1_voltage_callback: Option<extern "C" fn() -> i16>,
    sensor2_voltage_callback: Option<extern "C" fn() -> i16>,
    sensor3_voltage_callback: Option<extern "C" fn() -> i16>,
    sensor4_voltage_callback: Option<extern "C" fn() -> i16>,
    sensor5_voltage_callback: Option<extern "C" fn() -> i16>,
    sensor6_voltage_callback: Option<extern "C" fn() -> i16>,
    sensor7_voltage_callback: Option<extern "C" fn() -> i16>,
    sensor1_current_callback: Option<extern "C" fn() -> i16>,
    sensor2_current_callback: Option<extern "C" fn() -> i16>,
    sensor3_current_callback: Option<extern "C" fn() -> i16>,
    sensor4_current_callback: Option<extern "C" fn() -> i16>,
    sensor5_current_callback: Option<extern "C" fn() -> i16>,
    sensor6_current_callback: Option<extern "C" fn() -> i16>,
    sensor7_current_callback: Option<extern "C" fn() -> i16>,
    sensor8_frequency_callback: Option<extern "C" fn() -> u16>,
    relay_1_state_callback: Option<extern "C" fn() -> u16>,
    relay_2_state_callback: Option<extern "C" fn() -> u16>,
    relay_3_state_callback: Option<extern "C" fn() -> u16>,
    reset_the_accumulators_callback: Option<extern "C" fn() -> u16>,
    reset_the_system_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model64020CallbackAdapter {
    /// Aux 0 temperature
    fn aux_0_temperature(&self) -> Option<i16> {
        self.aux_0_temperature_callback.map(|callback| (callback)())
    }

    /// Aux 1 temperature
    fn aux_1_temperature(&self) -> Option<i16> {
        self.aux_1_temperature_callback.map(|callback| (callback)())
    }

    /// Aux 2 temperature
    fn aux_2_temperature(&self) -> Option<i16> {
        self.aux_2_temperature_callback.map(|callback| (callback)())
    }

    /// Aux 3 temperature
    fn aux_3_temperature(&self) -> Option<i16> {
        self.aux_3_temperature_callback.map(|callback| (callback)())
    }

    /// Aux 4 temperature
    fn aux_4_temperature(&self) -> Option<i16> {
        self.aux_4_temperature_callback.map(|callback| (callback)())
    }

    /// Probe Temperature
    fn probe_temperature(&self) -> i16 {
        (self.probe_temperature_callback)()
    }

    /// Main Temperature
    fn main_temperature(&self) -> i16 {
        (self.main_temperature_callback)()
    }

    /// Voltage scale factor for the sensors
    fn voltage_scale_factor_for_the_sensors(&self) -> u16 {
        (self.voltage_scale_factor_for_the_sensors_callback)()
    }

    /// Current scale factor for the sensors
    fn current_scale_factor_for_the_sensors(&self) -> u16 {
        (self.current_scale_factor_for_the_sensors_callback)()
    }

    /// Frequency scale factor for the sensors
    fn frequency_scale_factor_for_the_sensors(&self) -> u16 {
        (self.frequency_scale_factor_for_the_sensors_callback)()
    }

    /// Sensor1 Voltage
    ///
    /// scale of 0-10V
    fn sensor1_voltage(&self) -> Option<i16> {
        self.sensor1_voltage_callback.map(|callback| (callback)())
    }

    /// Sensor2 Voltage
    ///
    /// scale of 0-10V
    fn sensor2_voltage(&self) -> Option<i16> {
        self.sensor2_voltage_callback.map(|callback| (callback)())
    }

    /// Sensor3 Voltage
    ///
    /// scale of 0-10V
    fn sensor3_voltage(&self) -> Option<i16> {
        self.sensor3_voltage_callback.map(|callback| (callback)())
    }

    /// Sensor4 Voltage
    ///
    /// scale of 0-10V
    fn sensor4_voltage(&self) -> Option<i16> {
        self.sensor4_voltage_callback.map(|callback| (callback)())
    }

    /// Sensor5 Voltage
    ///
    /// scale of 0-10V
    fn sensor5_voltage(&self) -> Option<i16> {
        self.sensor5_voltage_callback.map(|callback| (callback)())
    }

    /// Sensor6 Voltage
    ///
    /// scale of 0-10V
    fn sensor6_voltage(&self) -> Option<i16> {
        self.sensor6_voltage_callback.map(|callback| (callback)())
    }

    /// Sensor7 Voltage
    ///
    /// scale of 0-10V
    fn sensor7_voltage(&self) -> Option<i16> {
        self.sensor7_voltage_callback.map(|callback| (callback)())
    }

    /// Sensor1 Current
    ///
    /// scale of 4-20mA
    fn sensor1_current(&self) -> Option<i16> {
        self.sensor1_current_callback.map(|callback| (callback)())
    }

    /// Sensor2 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor2_current(&self) -> Option<i16> {
        self.sensor2_current_callback.map(|callback| (callback)())
    }

    /// Sensor3 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor3_current(&self) -> Option<i16> {
        self.sensor3_current_callback.map(|callback| (callback)())
    }

    /// Sensor4 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor4_current(&self) -> Option<i16> {
        self.sensor4_current_callback.map(|callback| (callback)())
    }

    /// Sensor5 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor5_current(&self) -> Option<i16> {
        self.sensor5_current_callback.map(|callback| (callback)())
    }

    /// Sensor6 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor6_current(&self) -> Option<i16> {
        self.sensor6_current_callback.map(|callback| (callback)())
    }

    /// Sensor7 Current
    ///
    /// in 4-20mA or 4-20mA
    fn sensor7_current(&self) -> Option<i16> {
        self.sensor7_current_callback.map(|callback| (callback)())
    }

    /// Sensor8 frequency
    ///
    /// frequency in Hz
    fn sensor8_frequency(&self) -> Option<u16> {
        self.sensor8_frequency_callback.map(|callback| (callback)())
    }

    /// Relay 1 state
    fn relay_1_state(&self) -> Option<u16> {
        self.relay_1_state_callback.map(|callback| (callback)())
    }

    /// Relay 2 state
    fn relay_2_state(&self) -> Option<u16> {
        self.relay_2_state_callback.map(|callback| (callback)())
    }

    /// Relay 3 state
    fn relay_3_state(&self) -> Option<u16> {
        self.relay_3_state_callback.map(|callback| (callback)())
    }

    /// Reset the accumulators
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting them
    fn reset_the_accumulators(&self) -> Option<u16> {
        self.reset_the_accumulators_callback
            .map(|callback| (callback)())
    }

    /// Reset the system
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting the system
    fn reset_the_system(&self) -> Option<u16> {
        self.reset_the_system_callback.map(|callback| (callback)())
    }
}
