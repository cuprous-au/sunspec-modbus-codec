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
}

#[repr(C)]
pub struct Model714StatefulAdapter {
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
}

impl ModelAdapter for Model714StatefulAdapter {
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
}
