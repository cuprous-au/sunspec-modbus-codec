use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 88;

static POINTS: [PointDetails<()>; 11] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::TimeOffset,
        size: 10,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::Temperature,
        size: 2,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::GridModelSource,
        size: 32,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::IrradianceModelSource,
        size: 32,
        start_address: 46,
    },
    PointDetails {
        point: |()| Point::Irradiance,
        size: 2,
        start_address: 78,
    },
    PointDetails {
        point: |()| Point::GridVoltageA,
        size: 2,
        start_address: 80,
    },
    PointDetails {
        point: |()| Point::GridVoltageB,
        size: 2,
        start_address: 82,
    },
    PointDetails {
        point: |()| Point::GridVoltageC,
        size: 2,
        start_address: 84,
    },
    PointDetails {
        point: |()| Point::GridFrequency,
        size: 2,
        start_address: 86,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
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

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    88
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(64414, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::TimeOffset => {
            if let Some(value) = model.time_offset() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 10);
            }
        }
        Point::Temperature => {
            if let Some(value) = model.temperature() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::GridModelSource => {
            if let Some(value) = model.grid_model_source() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 32);
            }
        }
        Point::IrradianceModelSource => {
            if let Some(value) = model.irradiance_model_source() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 32);
            }
        }
        Point::Irradiance => {
            if let Some(value) = model.irradiance() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::GridVoltageA => {
            if let Some(value) = model.grid_voltage_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::GridVoltageB => {
            if let Some(value) = model.grid_voltage_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::GridVoltageC => {
            if let Some(value) = model.grid_voltage_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::GridFrequency => {
            if let Some(value) = model.grid_frequency() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
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
