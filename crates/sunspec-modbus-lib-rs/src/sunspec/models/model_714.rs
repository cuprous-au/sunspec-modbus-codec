use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 20;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 714 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::PortAlarms,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::NumberOfPorts,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::DcCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::DcPower,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::DcEnergyInjected,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::DcEnergyAbsorbed,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::DcCurrentScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::DcVoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::DcPowerScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::DcEnergyScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model714 {
            point: Point::TemperatureScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    PortAlarms,
    NumberOfPorts,
    DcCurrent,
    DcPower,
    DcEnergyInjected,
    DcEnergyAbsorbed,
    DcCurrentScaleFactor,
    DcVoltageScaleFactor,
    DcPowerScaleFactor,
    DcEnergyScaleFactor,
    TemperatureScaleFactor,
    PrtPortType { prt_index: u16 },
    PrtPortId { prt_index: u16 },
    PrtPortIdString { prt_index: u16 },
    PrtDcCurrent { prt_index: u16 },
    PrtDcVoltage { prt_index: u16 },
    PrtDcPower { prt_index: u16 },
    PrtDcEnergyInjected { prt_index: u16 },
    PrtDcEnergyAbsorbed { prt_index: u16 },
    PrtDcPortTemperature { prt_index: u16 },
    PrtDcPortStatus { prt_index: u16 },
    PrtDcPortAlarm { prt_index: u16 },
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    20 + model.number_of_ports().unwrap_or(0) * (25)
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
        Point::PortAlarms => {
            if let Some(value) = model.port_alarms() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NumberOfPorts => {
            if let Some(value) = model.number_of_ports() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcCurrent => {
            if let Some(value) = model.dc_current() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcPower => {
            if let Some(value) = model.dc_power() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcEnergyInjected => {
            if let Some(value) = model.dc_energy_injected() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcEnergyAbsorbed => {
            if let Some(value) = model.dc_energy_absorbed() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcCurrentScaleFactor => {
            if let Some(value) = model.dc_current_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcVoltageScaleFactor => {
            if let Some(value) = model.dc_voltage_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcPowerScaleFactor => {
            if let Some(value) = model.dc_power_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcEnergyScaleFactor => {
            if let Some(value) = model.dc_energy_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TemperatureScaleFactor => {
            if let Some(value) = model.temperature_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtPortType { prt_index } => {
            if let Some(value) = model.prt_port_type(*prt_index) {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtPortId { prt_index } => {
            if let Some(value) = model.prt_port_id(*prt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtPortIdString { prt_index } => {
            if let Some(value) = model.prt_port_id_string(*prt_index) {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtDcCurrent { prt_index } => {
            if let Some(value) = model.prt_dc_current(*prt_index) {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtDcVoltage { prt_index } => {
            if let Some(value) = model.prt_dc_voltage(*prt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtDcPower { prt_index } => {
            if let Some(value) = model.prt_dc_power(*prt_index) {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtDcEnergyInjected { prt_index } => {
            if let Some(value) = model.prt_dc_energy_injected(*prt_index) {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtDcEnergyAbsorbed { prt_index } => {
            if let Some(value) = model.prt_dc_energy_absorbed(*prt_index) {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtDcPortTemperature { prt_index } => {
            if let Some(value) = model.prt_dc_port_temperature(*prt_index) {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtDcPortStatus { prt_index } => {
            if let Some(value) = model.prt_dc_port_status(*prt_index) {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PrtDcPortAlarm { prt_index } => {
            if let Some(value) = model.prt_dc_port_alarm(*prt_index) {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    fn port_alarms(&self) -> Option<u32> {
        None
    }

    /// Number Of Ports
    ///
    /// Number of DC ports.
    fn number_of_ports(&self) -> Option<u16> {
        None
    }

    /// DC Current
    ///
    /// Total DC current for all ports.
    fn dc_current(&self) -> Option<i16> {
        None
    }

    /// DC Power
    ///
    /// Total DC power for all ports.
    fn dc_power(&self) -> Option<i16> {
        None
    }

    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for all ports.
    fn dc_energy_injected(&self) -> Option<u64> {
        None
    }

    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    fn dc_energy_absorbed(&self) -> Option<u64> {
        None
    }

    /// DC Current Scale Factor
    ///
    /// DC current scale factor.
    fn dc_current_scale_factor(&self) -> Option<u16> {
        None
    }

    /// DC Voltage Scale Factor
    ///
    /// DC voltage scale factor.
    fn dc_voltage_scale_factor(&self) -> Option<u16> {
        None
    }

    /// DC Power Scale Factor
    ///
    /// DC power scale factor.
    fn dc_power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// DC Energy Scale Factor
    ///
    /// DC energy scale factor.
    fn dc_energy_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Temperature Scale Factor
    ///
    /// Temperature Scale Factor.
    fn temperature_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Port Type
    ///
    /// Port type.
    fn prt_port_type(&self, prt_index: u16) -> Option<PrtTyp> {
        None
    }

    /// Port ID
    ///
    /// Port ID.
    fn prt_port_id(&self, prt_index: u16) -> Option<u16> {
        None
    }

    /// Port ID String
    ///
    /// Port ID string.
    fn prt_port_id_string(&self, prt_index: u16) -> Option<&CStr> {
        None
    }

    /// DC Current
    ///
    /// DC current for the port.
    fn prt_dc_current(&self, prt_index: u16) -> Option<i16> {
        None
    }

    /// DC Voltage
    ///
    /// DC voltage for the port.
    fn prt_dc_voltage(&self, prt_index: u16) -> Option<u16> {
        None
    }

    /// DC Power
    ///
    /// DC power for the port.
    fn prt_dc_power(&self, prt_index: u16) -> Option<i16> {
        None
    }

    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for the port.
    fn prt_dc_energy_injected(&self, prt_index: u16) -> Option<u64> {
        None
    }

    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for the port.
    fn prt_dc_energy_absorbed(&self, prt_index: u16) -> Option<u64> {
        None
    }

    /// DC Port Temperature
    ///
    /// DC port temperature.
    fn prt_dc_port_temperature(&self, prt_index: u16) -> Option<i16> {
        None
    }

    /// DC Port Status
    ///
    /// DC port status.
    fn prt_dc_port_status(&self, prt_index: u16) -> Option<DcSta> {
        None
    }

    /// DC Port Alarm
    ///
    /// DC port alarm.
    fn prt_dc_port_alarm(&self, prt_index: u16) -> Option<u32> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum DcSta {
    /// Off
    Off = 0,
    /// On
    On = 1,
    /// Warning
    Warning = 2,
    /// Error
    Error = 3,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum PrtTyp {
    /// Photovoltaic
    Pv = 0,
    /// Energy Storage System
    Ess = 1,
    /// Electric Vehicle
    Ev = 2,
    /// Generic Injecting
    Inj = 3,
    /// Generic Absorbing
    Abs = 4,
    /// Generic Bidirectional
    Bidir = 5,
    /// DC to DC
    DcDc = 6,
}

#[repr(C)]
pub struct Model714CallbackAdapter {
    context: *mut c_void,
    port_alarms_callback: Option<extern "C" fn(*const c_void) -> u32>,
    number_of_ports_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dc_current_callback: Option<extern "C" fn(*const c_void) -> i16>,
    dc_power_callback: Option<extern "C" fn(*const c_void) -> i16>,
    dc_energy_injected_callback: Option<extern "C" fn(*const c_void) -> u64>,
    dc_energy_absorbed_callback: Option<extern "C" fn(*const c_void) -> u64>,
    dc_current_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dc_voltage_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dc_power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dc_energy_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    temperature_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    prt_port_type_callback: Option<extern "C" fn(*const c_void, u16) -> PrtTyp>,
    prt_port_id_callback: Option<extern "C" fn(*const c_void, u16) -> u16>,
    prt_port_id_string_callback: Option<extern "C" fn(*const c_void, u16) -> *const c_char>,
    prt_dc_current_callback: Option<extern "C" fn(*const c_void, u16) -> i16>,
    prt_dc_voltage_callback: Option<extern "C" fn(*const c_void, u16) -> u16>,
    prt_dc_power_callback: Option<extern "C" fn(*const c_void, u16) -> i16>,
    prt_dc_energy_injected_callback: Option<extern "C" fn(*const c_void, u16) -> u64>,
    prt_dc_energy_absorbed_callback: Option<extern "C" fn(*const c_void, u16) -> u64>,
    prt_dc_port_temperature_callback: Option<extern "C" fn(*const c_void, u16) -> i16>,
    prt_dc_port_status_callback: Option<extern "C" fn(*const c_void, u16) -> DcSta>,
    prt_dc_port_alarm_callback: Option<extern "C" fn(*const c_void, u16) -> u32>,
}

impl ModelAdapter for Model714CallbackAdapter {
    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    fn port_alarms(&self) -> Option<u32> {
        self.port_alarms_callback
            .map(|callback| (callback)(self.context))
    }

    /// Number Of Ports
    ///
    /// Number of DC ports.
    fn number_of_ports(&self) -> Option<u16> {
        self.number_of_ports_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Current
    ///
    /// Total DC current for all ports.
    fn dc_current(&self) -> Option<i16> {
        self.dc_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Power
    ///
    /// Total DC power for all ports.
    fn dc_power(&self) -> Option<i16> {
        self.dc_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for all ports.
    fn dc_energy_injected(&self) -> Option<u64> {
        self.dc_energy_injected_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    fn dc_energy_absorbed(&self) -> Option<u64> {
        self.dc_energy_absorbed_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Current Scale Factor
    ///
    /// DC current scale factor.
    fn dc_current_scale_factor(&self) -> Option<u16> {
        self.dc_current_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Voltage Scale Factor
    ///
    /// DC voltage scale factor.
    fn dc_voltage_scale_factor(&self) -> Option<u16> {
        self.dc_voltage_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Power Scale Factor
    ///
    /// DC power scale factor.
    fn dc_power_scale_factor(&self) -> Option<u16> {
        self.dc_power_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Energy Scale Factor
    ///
    /// DC energy scale factor.
    fn dc_energy_scale_factor(&self) -> Option<u16> {
        self.dc_energy_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Temperature Scale Factor
    ///
    /// Temperature Scale Factor.
    fn temperature_scale_factor(&self) -> Option<u16> {
        self.temperature_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Port Type
    ///
    /// Port type.
    fn prt_port_type(&self, prt_index: u16) -> Option<PrtTyp> {
        self.prt_port_type_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// Port ID
    ///
    /// Port ID.
    fn prt_port_id(&self, prt_index: u16) -> Option<u16> {
        self.prt_port_id_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// Port ID String
    ///
    /// Port ID string.
    fn prt_port_id_string(&self, prt_index: u16) -> Option<&CStr> {
        self.prt_port_id_string_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context, prt_index)) })
    }

    /// DC Current
    ///
    /// DC current for the port.
    fn prt_dc_current(&self, prt_index: u16) -> Option<i16> {
        self.prt_dc_current_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// DC Voltage
    ///
    /// DC voltage for the port.
    fn prt_dc_voltage(&self, prt_index: u16) -> Option<u16> {
        self.prt_dc_voltage_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// DC Power
    ///
    /// DC power for the port.
    fn prt_dc_power(&self, prt_index: u16) -> Option<i16> {
        self.prt_dc_power_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for the port.
    fn prt_dc_energy_injected(&self, prt_index: u16) -> Option<u64> {
        self.prt_dc_energy_injected_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for the port.
    fn prt_dc_energy_absorbed(&self, prt_index: u16) -> Option<u64> {
        self.prt_dc_energy_absorbed_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// DC Port Temperature
    ///
    /// DC port temperature.
    fn prt_dc_port_temperature(&self, prt_index: u16) -> Option<i16> {
        self.prt_dc_port_temperature_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// DC Port Status
    ///
    /// DC port status.
    fn prt_dc_port_status(&self, prt_index: u16) -> Option<DcSta> {
        self.prt_dc_port_status_callback
            .map(|callback| (callback)(self.context, prt_index))
    }

    /// DC Port Alarm
    ///
    /// DC port alarm.
    fn prt_dc_port_alarm(&self, prt_index: u16) -> Option<u32> {
        self.prt_dc_port_alarm_callback
            .map(|callback| (callback)(self.context, prt_index))
    }
}

#[repr(C)]
pub struct Model714StatefulAdapter<const NUMBER_OF_PORTS: usize> {
    port_alarms: u32,
    number_of_ports: u16,
    dc_current: i16,
    dc_power: i16,
    dc_energy_injected: u64,
    dc_energy_absorbed: u64,
    dc_current_scale_factor: u16,
    dc_voltage_scale_factor: u16,
    dc_power_scale_factor: u16,
    dc_energy_scale_factor: u16,
    temperature_scale_factor: u16,
    prt: [Model714Prt; NUMBER_OF_PORTS],
}

#[repr(C)]
pub struct Model714Prt {
    prt_port_type: PrtTyp,
    prt_port_id: u16,
    prt_port_id_string: [c_char; 16],
    prt_dc_current: i16,
    prt_dc_voltage: u16,
    prt_dc_power: i16,
    prt_dc_energy_injected: u64,
    prt_dc_energy_absorbed: u64,
    prt_dc_port_temperature: i16,
    prt_dc_port_status: DcSta,
    prt_dc_port_alarm: u32,
}

impl<const NUMBER_OF_PORTS: usize> ModelAdapter for Model714StatefulAdapter<NUMBER_OF_PORTS> {
    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    fn port_alarms(&self) -> Option<u32> {
        Some(self.port_alarms)
    }

    /// Number Of Ports
    ///
    /// Number of DC ports.
    fn number_of_ports(&self) -> Option<u16> {
        Some(self.number_of_ports)
    }

    /// DC Current
    ///
    /// Total DC current for all ports.
    fn dc_current(&self) -> Option<i16> {
        Some(self.dc_current)
    }

    /// DC Power
    ///
    /// Total DC power for all ports.
    fn dc_power(&self) -> Option<i16> {
        Some(self.dc_power)
    }

    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for all ports.
    fn dc_energy_injected(&self) -> Option<u64> {
        Some(self.dc_energy_injected)
    }

    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    fn dc_energy_absorbed(&self) -> Option<u64> {
        Some(self.dc_energy_absorbed)
    }

    /// DC Current Scale Factor
    ///
    /// DC current scale factor.
    fn dc_current_scale_factor(&self) -> Option<u16> {
        Some(self.dc_current_scale_factor)
    }

    /// DC Voltage Scale Factor
    ///
    /// DC voltage scale factor.
    fn dc_voltage_scale_factor(&self) -> Option<u16> {
        Some(self.dc_voltage_scale_factor)
    }

    /// DC Power Scale Factor
    ///
    /// DC power scale factor.
    fn dc_power_scale_factor(&self) -> Option<u16> {
        Some(self.dc_power_scale_factor)
    }

    /// DC Energy Scale Factor
    ///
    /// DC energy scale factor.
    fn dc_energy_scale_factor(&self) -> Option<u16> {
        Some(self.dc_energy_scale_factor)
    }

    /// Temperature Scale Factor
    ///
    /// Temperature Scale Factor.
    fn temperature_scale_factor(&self) -> Option<u16> {
        Some(self.temperature_scale_factor)
    }

    /// Port Type
    ///
    /// Port type.
    fn prt_port_type(&self, prt_index: u16) -> Option<PrtTyp> {
        Some(self.prt[prt_index as usize].prt_port_type)
    }

    /// Port ID
    ///
    /// Port ID.
    fn prt_port_id(&self, prt_index: u16) -> Option<u16> {
        Some(self.prt[prt_index as usize].prt_port_id)
    }

    /// Port ID String
    ///
    /// Port ID string.
    fn prt_port_id_string(&self, prt_index: u16) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.prt[prt_index as usize].prt_port_id_string.as_ptr()) })
    }

    /// DC Current
    ///
    /// DC current for the port.
    fn prt_dc_current(&self, prt_index: u16) -> Option<i16> {
        Some(self.prt[prt_index as usize].prt_dc_current)
    }

    /// DC Voltage
    ///
    /// DC voltage for the port.
    fn prt_dc_voltage(&self, prt_index: u16) -> Option<u16> {
        Some(self.prt[prt_index as usize].prt_dc_voltage)
    }

    /// DC Power
    ///
    /// DC power for the port.
    fn prt_dc_power(&self, prt_index: u16) -> Option<i16> {
        Some(self.prt[prt_index as usize].prt_dc_power)
    }

    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for the port.
    fn prt_dc_energy_injected(&self, prt_index: u16) -> Option<u64> {
        Some(self.prt[prt_index as usize].prt_dc_energy_injected)
    }

    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for the port.
    fn prt_dc_energy_absorbed(&self, prt_index: u16) -> Option<u64> {
        Some(self.prt[prt_index as usize].prt_dc_energy_absorbed)
    }

    /// DC Port Temperature
    ///
    /// DC port temperature.
    fn prt_dc_port_temperature(&self, prt_index: u16) -> Option<i16> {
        Some(self.prt[prt_index as usize].prt_dc_port_temperature)
    }

    /// DC Port Status
    ///
    /// DC port status.
    fn prt_dc_port_status(&self, prt_index: u16) -> Option<DcSta> {
        Some(self.prt[prt_index as usize].prt_dc_port_status)
    }

    /// DC Port Alarm
    ///
    /// DC port alarm.
    fn prt_dc_port_alarm(&self, prt_index: u16) -> Option<u32> {
        Some(self.prt[prt_index as usize].prt_dc_port_alarm)
    }
}
