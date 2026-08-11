use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 6;

static POINTS: [PointDetails<()>; 6] = [
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
        point: |()| Point::Ghi,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::Amps,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::Voltage,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::Temperature,
        size: 1,
        start_address: 5,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    Ghi,
    Amps,
    Voltage,
    Temperature,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    6
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
            buffer::write_u16(306, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Ghi => {
            if let Some(value) = model.ghi() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Amps => {
            if let Some(value) = model.amps() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Voltage => {
            if let Some(value) = model.voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Temperature => {
            if let Some(value) = model.temperature() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        None
    }

    /// Amps
    ///
    /// Current measurement at reference point
    fn amps(&self) -> Option<u16> {
        None
    }

    /// Voltage
    ///
    /// Voltage measurement at reference point
    fn voltage(&self) -> Option<u16> {
        None
    }

    /// Temperature
    ///
    /// Temperature measurement at reference point
    fn temperature(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model306CallbackAdapter {
    context: *mut c_void,
    ghi_callback: Option<extern "C" fn(*const c_void) -> u16>,
    amps_callback: Option<extern "C" fn(*const c_void) -> u16>,
    voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    temperature_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model306CallbackAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        self.ghi_callback.map(|callback| (callback)(self.context))
    }

    /// Amps
    ///
    /// Current measurement at reference point
    fn amps(&self) -> Option<u16> {
        self.amps_callback.map(|callback| (callback)(self.context))
    }

    /// Voltage
    ///
    /// Voltage measurement at reference point
    fn voltage(&self) -> Option<u16> {
        self.voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Temperature
    ///
    /// Temperature measurement at reference point
    fn temperature(&self) -> Option<u16> {
        self.temperature_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model306StatefulAdapter {
    ghi: u16,
    amps: u16,
    voltage: u16,
    temperature: u16,
}

impl ModelAdapter for Model306StatefulAdapter {
    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        Some(self.ghi)
    }

    /// Amps
    ///
    /// Current measurement at reference point
    fn amps(&self) -> Option<u16> {
        Some(self.amps)
    }

    /// Voltage
    ///
    /// Voltage measurement at reference point
    fn voltage(&self) -> Option<u16> {
        Some(self.voltage)
    }

    /// Temperature
    ///
    /// Temperature measurement at reference point
    fn temperature(&self) -> Option<u16> {
        Some(self.temperature)
    }
}
