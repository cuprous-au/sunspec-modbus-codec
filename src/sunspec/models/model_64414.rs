use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 88;

pub static POINTS: [ReadablePoint; 11] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64414 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 86 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::TimeOffset,
        },
        size: 10,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::Temperature,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::GridModelSource,
        },
        size: 32,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::IrradianceModelSource,
        },
        size: 32,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::Irradiance,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::GridVoltageA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::GridVoltageB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::GridVoltageC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::GridFrequency,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    TimeOffset,
    Temperature,
    GridModelSource,
    IrradianceModelSource,
    Irradiance,
    GridVoltageA,
    GridVoltageB,
    GridVoltageC,
    GridFrequency,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::TimeOffset => {
            if let Some(value) = model.time_offset() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Temperature => {
            if let Some(value) = model.temperature() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::GridModelSource => {
            if let Some(value) = model.grid_model_source() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::IrradianceModelSource => {
            if let Some(value) = model.irradiance_model_source() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Irradiance => {
            if let Some(value) = model.irradiance() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::GridVoltageA => {
            if let Some(value) = model.grid_voltage_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::GridVoltageB => {
            if let Some(value) = model.grid_voltage_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::GridVoltageC => {
            if let Some(value) = model.grid_voltage_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::GridFrequency => {
            if let Some(value) = model.grid_frequency() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Time offset
    ///
    /// Time offset for simulation formatted 'HH:MM:SS'
    fn time_offset(&self) -> Option<&CStr> {
        None
    }

    /// Ambient temperature (degrees Celsius)
    fn temperature(&self) -> Option<f32> {
        None
    }

    /// Ambient temperature (degrees Celsius)
    fn set_temperature(&mut self, value: f32) {}

    /// The data source for the grid model. 'csv' or 'const'
    fn grid_model_source(&self) -> Option<&CStr> {
        None
    }

    /// The data source for the grid model. 'csv' or 'const'
    fn set_grid_model_source(&mut self, value: &CStr) {}

    /// The data source for the irradiance model. 'csv' or 'const'
    fn irradiance_model_source(&self) -> Option<&CStr> {
        None
    }

    /// The data source for the irradiance model. 'csv' or 'const'
    fn set_irradiance_model_source(&mut self, value: &CStr) {}

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn irradiance(&self) -> Option<f32> {
        None
    }

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn set_irradiance(&mut self, value: f32) {}

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_a(&self) -> Option<f32> {
        None
    }

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_a(&mut self, value: f32) {}

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_b(&self) -> Option<f32> {
        None
    }

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_b(&mut self, value: f32) {}

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_c(&self) -> Option<f32> {
        None
    }

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_c(&mut self, value: f32) {}

    /// Grid frequency (Hz) for the 'const' grid model
    fn grid_frequency(&self) -> Option<f32> {
        None
    }

    /// Grid frequency (Hz) for the 'const' grid model
    fn set_grid_frequency(&mut self, value: f32) {}
}

#[repr(C)]
pub struct Model64414CallbackAdapter {
    time_offset_callback: Option<extern "C" fn() -> *const c_char>,
    temperature_callback: Option<extern "C" fn() -> f32>,
    set_temperature_callback: Option<extern "C" fn(f32)>,
    grid_model_source_callback: Option<extern "C" fn() -> *const c_char>,
    set_grid_model_source_callback: Option<extern "C" fn(*const c_char)>,
    irradiance_model_source_callback: Option<extern "C" fn() -> *const c_char>,
    set_irradiance_model_source_callback: Option<extern "C" fn(*const c_char)>,
    irradiance_callback: Option<extern "C" fn() -> f32>,
    set_irradiance_callback: Option<extern "C" fn(f32)>,
    grid_voltage_a_callback: Option<extern "C" fn() -> f32>,
    set_grid_voltage_a_callback: Option<extern "C" fn(f32)>,
    grid_voltage_b_callback: Option<extern "C" fn() -> f32>,
    set_grid_voltage_b_callback: Option<extern "C" fn(f32)>,
    grid_voltage_c_callback: Option<extern "C" fn() -> f32>,
    set_grid_voltage_c_callback: Option<extern "C" fn(f32)>,
    grid_frequency_callback: Option<extern "C" fn() -> f32>,
    set_grid_frequency_callback: Option<extern "C" fn(f32)>,
}

impl ModelAdapter for Model64414CallbackAdapter {
    /// Time offset
    ///
    /// Time offset for simulation formatted 'HH:MM:SS'
    fn time_offset(&self) -> Option<&CStr> {
        self.time_offset_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Ambient temperature (degrees Celsius)
    fn temperature(&self) -> Option<f32> {
        self.temperature_callback.map(|callback| (callback)())
    }

    /// Ambient temperature (degrees Celsius)
    fn set_temperature(&mut self, value: f32) {
        if let Some(callback) = self.set_temperature_callback {
            (callback)(value);
        };
    }

    /// The data source for the grid model. 'csv' or 'const'
    fn grid_model_source(&self) -> Option<&CStr> {
        self.grid_model_source_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// The data source for the grid model. 'csv' or 'const'
    fn set_grid_model_source(&mut self, value: &CStr) {
        if let Some(callback) = self.set_grid_model_source_callback {
            (callback)(value.as_ptr());
        };
    }

    /// The data source for the irradiance model. 'csv' or 'const'
    fn irradiance_model_source(&self) -> Option<&CStr> {
        self.irradiance_model_source_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// The data source for the irradiance model. 'csv' or 'const'
    fn set_irradiance_model_source(&mut self, value: &CStr) {
        if let Some(callback) = self.set_irradiance_model_source_callback {
            (callback)(value.as_ptr());
        };
    }

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn irradiance(&self) -> Option<f32> {
        self.irradiance_callback.map(|callback| (callback)())
    }

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn set_irradiance(&mut self, value: f32) {
        if let Some(callback) = self.set_irradiance_callback {
            (callback)(value);
        };
    }

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_a(&self) -> Option<f32> {
        self.grid_voltage_a_callback.map(|callback| (callback)())
    }

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_a(&mut self, value: f32) {
        if let Some(callback) = self.set_grid_voltage_a_callback {
            (callback)(value);
        };
    }

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_b(&self) -> Option<f32> {
        self.grid_voltage_b_callback.map(|callback| (callback)())
    }

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_b(&mut self, value: f32) {
        if let Some(callback) = self.set_grid_voltage_b_callback {
            (callback)(value);
        };
    }

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_c(&self) -> Option<f32> {
        self.grid_voltage_c_callback.map(|callback| (callback)())
    }

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_c(&mut self, value: f32) {
        if let Some(callback) = self.set_grid_voltage_c_callback {
            (callback)(value);
        };
    }

    /// Grid frequency (Hz) for the 'const' grid model
    fn grid_frequency(&self) -> Option<f32> {
        self.grid_frequency_callback.map(|callback| (callback)())
    }

    /// Grid frequency (Hz) for the 'const' grid model
    fn set_grid_frequency(&mut self, value: f32) {
        if let Some(callback) = self.set_grid_frequency_callback {
            (callback)(value);
        };
    }
}
