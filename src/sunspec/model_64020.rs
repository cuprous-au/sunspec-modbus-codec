pub struct Model64020 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Aux 0 temperature
    aux0_tmp: Option<i16>,
    /// Aux 1 temperature
    aux1_tmp: Option<i16>,
    /// Aux 2 temperature
    aux2_tmp: Option<i16>,
    /// Aux 3 temperature
    aux3_tmp: Option<i16>,
    /// Aux 4 temperature
    aux4_tmp: Option<i16>,
    /// Probe Temperature
    probe_tmp: i16,
    /// Main Temperature
    main_tmp: i16,
    /// Voltage scale factor for the sensors
    sensor_v_sf: u16,
    /// Current scale factor for the sensors
    sensor_a_sf: u16,
    /// Frequency scale factor for the sensors
    sensor_hz_sf: u16,
    /// Sensor1 Voltage
    ///
    /// scale of 0-10V
    sensor1_voltage: Option<i16>,
    /// Sensor2 Voltage
    ///
    /// scale of 0-10V
    sensor2_voltage: Option<i16>,
    /// Sensor3 Voltage
    ///
    /// scale of 0-10V
    sensor3_voltage: Option<i16>,
    /// Sensor4 Voltage
    ///
    /// scale of 0-10V
    sensor4_voltage: Option<i16>,
    /// Sensor5 Voltage
    ///
    /// scale of 0-10V
    sensor5_voltage: Option<i16>,
    /// Sensor6 Voltage
    ///
    /// scale of 0-10V
    sensor6_voltage: Option<i16>,
    /// Sensor7 Voltage
    ///
    /// scale of 0-10V
    sensor7_voltage: Option<i16>,
    /// Sensor1 Current
    ///
    /// scale of 4-20mA
    sensor1_current: Option<i16>,
    /// Sensor2 Current
    ///
    /// in 4-20mA or 4-20mA
    sensor2_current: Option<i16>,
    /// Sensor3 Current
    ///
    /// in 4-20mA or 4-20mA
    sensor3_current: Option<i16>,
    /// Sensor4 Current
    ///
    /// in 4-20mA or 4-20mA
    sensor4_current: Option<i16>,
    /// Sensor5 Current
    ///
    /// in 4-20mA or 4-20mA
    sensor5_current: Option<i16>,
    /// Sensor6 Current
    ///
    /// in 4-20mA or 4-20mA
    sensor6_current: Option<i16>,
    /// Sensor7 Current
    ///
    /// in 4-20mA or 4-20mA
    sensor7_current: Option<i16>,
    /// Sensor8 frequency
    ///
    /// frequency in Hz
    sensor8: Option<u16>,
    /// Relay 1 state
    relay1: Option<u16>,
    /// Relay 2 state
    relay2: Option<u16>,
    /// Relay 3 state
    relay3: Option<u16>,
    /// Reset the accumulators
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting them
    reset_accumulators: Option<u16>,
    /// Reset the system
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting the system
    reset: Option<u16>,
}

trait Model64020Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Aux 0 temperature
    fn aux0_tmp(&self) -> Option<i16> {
        None
    }

    /// Aux 1 temperature
    fn aux1_tmp(&self) -> Option<i16> {
        None
    }

    /// Aux 2 temperature
    fn aux2_tmp(&self) -> Option<i16> {
        None
    }

    /// Aux 3 temperature
    fn aux3_tmp(&self) -> Option<i16> {
        None
    }

    /// Aux 4 temperature
    fn aux4_tmp(&self) -> Option<i16> {
        None
    }

    /// Probe Temperature
    fn probe_tmp(&self) -> i16;

    /// Main Temperature
    fn main_tmp(&self) -> i16;

    /// Voltage scale factor for the sensors
    fn sensor_v_sf(&self) -> u16;

    /// Current scale factor for the sensors
    fn sensor_a_sf(&self) -> u16;

    /// Frequency scale factor for the sensors
    fn sensor_hz_sf(&self) -> u16;

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
    fn sensor8(&self) -> Option<u16> {
        None
    }

    /// Relay 1 state
    fn relay1(&self) -> Option<u16> {
        None
    }

    /// Relay 2 state
    fn relay2(&self) -> Option<u16> {
        None
    }

    /// Relay 3 state
    fn relay3(&self) -> Option<u16> {
        None
    }

    /// Reset the accumulators
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting them
    fn reset_accumulators(&self) -> Option<u16> {
        None
    }

    /// Reset the system
    ///
    /// always 0 in reading, used the code 0xC0DA during the writing for resetting the system
    fn reset(&self) -> Option<u16> {
        None
    }
}
