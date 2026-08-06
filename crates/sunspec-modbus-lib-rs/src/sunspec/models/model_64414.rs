use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 88;

pub static POINTS: [ReadablePoint; 11] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64414 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64414 {
            point: Point::ModelLength,
        },
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
    ModelLength,
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

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    88
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
        Point::TimeOffset => {
            if let Some(value) = model.time_offset() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Temperature => {
            if let Some(value) = model.temperature() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::GridModelSource => {
            if let Some(value) = model.grid_model_source() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IrradianceModelSource => {
            if let Some(value) = model.irradiance_model_source() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Irradiance => {
            if let Some(value) = model.irradiance() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::GridVoltageA => {
            if let Some(value) = model.grid_voltage_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::GridVoltageB => {
            if let Some(value) = model.grid_voltage_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::GridVoltageC => {
            if let Some(value) = model.grid_voltage_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::GridFrequency => {
            if let Some(value) = model.grid_frequency() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
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
    context: *mut c_void,
    time_offset_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    temperature_callback: Option<extern "C" fn(*const c_void) -> f32>,
    set_temperature_callback: Option<extern "C" fn(f32, *mut c_void)>,
    grid_model_source_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_grid_model_source_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    irradiance_model_source_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_irradiance_model_source_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    irradiance_callback: Option<extern "C" fn(*const c_void) -> f32>,
    set_irradiance_callback: Option<extern "C" fn(f32, *mut c_void)>,
    grid_voltage_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    set_grid_voltage_a_callback: Option<extern "C" fn(f32, *mut c_void)>,
    grid_voltage_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    set_grid_voltage_b_callback: Option<extern "C" fn(f32, *mut c_void)>,
    grid_voltage_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    set_grid_voltage_c_callback: Option<extern "C" fn(f32, *mut c_void)>,
    grid_frequency_callback: Option<extern "C" fn(*const c_void) -> f32>,
    set_grid_frequency_callback: Option<extern "C" fn(f32, *mut c_void)>,
}

impl ModelAdapter for Model64414CallbackAdapter {
    /// Time offset
    ///
    /// Time offset for simulation formatted 'HH:MM:SS'
    fn time_offset(&self) -> Option<&CStr> {
        self.time_offset_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Ambient temperature (degrees Celsius)
    fn temperature(&self) -> Option<f32> {
        self.temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Ambient temperature (degrees Celsius)
    fn set_temperature(&mut self, value: f32) {
        if let Some(callback) = self.set_temperature_callback {
            (callback)(value, self.context);
        };
    }

    /// The data source for the grid model. 'csv' or 'const'
    fn grid_model_source(&self) -> Option<&CStr> {
        self.grid_model_source_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// The data source for the grid model. 'csv' or 'const'
    fn set_grid_model_source(&mut self, value: &CStr) {
        if let Some(callback) = self.set_grid_model_source_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// The data source for the irradiance model. 'csv' or 'const'
    fn irradiance_model_source(&self) -> Option<&CStr> {
        self.irradiance_model_source_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// The data source for the irradiance model. 'csv' or 'const'
    fn set_irradiance_model_source(&mut self, value: &CStr) {
        if let Some(callback) = self.set_irradiance_model_source_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn irradiance(&self) -> Option<f32> {
        self.irradiance_callback
            .map(|callback| (callback)(self.context))
    }

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn set_irradiance(&mut self, value: f32) {
        if let Some(callback) = self.set_irradiance_callback {
            (callback)(value, self.context);
        };
    }

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_a(&self) -> Option<f32> {
        self.grid_voltage_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_a(&mut self, value: f32) {
        if let Some(callback) = self.set_grid_voltage_a_callback {
            (callback)(value, self.context);
        };
    }

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_b(&self) -> Option<f32> {
        self.grid_voltage_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_b(&mut self, value: f32) {
        if let Some(callback) = self.set_grid_voltage_b_callback {
            (callback)(value, self.context);
        };
    }

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_c(&self) -> Option<f32> {
        self.grid_voltage_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_c(&mut self, value: f32) {
        if let Some(callback) = self.set_grid_voltage_c_callback {
            (callback)(value, self.context);
        };
    }

    /// Grid frequency (Hz) for the 'const' grid model
    fn grid_frequency(&self) -> Option<f32> {
        self.grid_frequency_callback
            .map(|callback| (callback)(self.context))
    }

    /// Grid frequency (Hz) for the 'const' grid model
    fn set_grid_frequency(&mut self, value: f32) {
        if let Some(callback) = self.set_grid_frequency_callback {
            (callback)(value, self.context);
        };
    }
}

#[repr(C)]
pub struct Model64414StatefulAdapter {
    time_offset: [c_char; 20],
    temperature: f32,
    grid_model_source: [c_char; 64],
    irradiance_model_source: [c_char; 64],
    irradiance: f32,
    grid_voltage_a: f32,
    grid_voltage_b: f32,
    grid_voltage_c: f32,
    grid_frequency: f32,
}

impl ModelAdapter for Model64414StatefulAdapter {
    /// Time offset
    ///
    /// Time offset for simulation formatted 'HH:MM:SS'
    fn time_offset(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.time_offset.as_ptr()) })
    }

    /// Ambient temperature (degrees Celsius)
    fn temperature(&self) -> Option<f32> {
        Some(self.temperature)
    }

    /// Ambient temperature (degrees Celsius)
    fn set_temperature(&mut self, value: f32) {
        self.temperature = value;
    }

    /// The data source for the grid model. 'csv' or 'const'
    fn grid_model_source(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.grid_model_source.as_ptr()) })
    }

    /// The data source for the grid model. 'csv' or 'const'
    fn set_grid_model_source(&mut self, value: &CStr) {
        for (dest, src) in self
            .grid_model_source
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// The data source for the irradiance model. 'csv' or 'const'
    fn irradiance_model_source(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.irradiance_model_source.as_ptr()) })
    }

    /// The data source for the irradiance model. 'csv' or 'const'
    fn set_irradiance_model_source(&mut self, value: &CStr) {
        for (dest, src) in self
            .irradiance_model_source
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn irradiance(&self) -> Option<f32> {
        Some(self.irradiance)
    }

    /// The irradiance on the DER device (W/m^2) for the 'const' irradiance model
    fn set_irradiance(&mut self, value: f32) {
        self.irradiance = value;
    }

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_a(&self) -> Option<f32> {
        Some(self.grid_voltage_a)
    }

    /// Phase A RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_a(&mut self, value: f32) {
        self.grid_voltage_a = value;
    }

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_b(&self) -> Option<f32> {
        Some(self.grid_voltage_b)
    }

    /// Phase B RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_b(&mut self, value: f32) {
        self.grid_voltage_b = value;
    }

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn grid_voltage_c(&self) -> Option<f32> {
        Some(self.grid_voltage_c)
    }

    /// Phase C RMS Voltage (pu) for the 'const' grid model
    fn set_grid_voltage_c(&mut self, value: f32) {
        self.grid_voltage_c = value;
    }

    /// Grid frequency (Hz) for the 'const' grid model
    fn grid_frequency(&self) -> Option<f32> {
        Some(self.grid_frequency)
    }

    /// Grid frequency (Hz) for the 'const' grid model
    fn set_grid_frequency(&mut self, value: f32) {
        self.grid_frequency = value;
    }
}
