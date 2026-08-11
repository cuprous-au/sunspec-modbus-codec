use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 4;

static POINTS: [PointDetails<()>; 4] = [
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
        point: |()| Point::BatteryPointsToBeDetermined,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::BatteryStringBatteryStringPointsToBeDetermined,
        size: 1,
        start_address: 3,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    BatteryPointsToBeDetermined,
    BatteryStringBatteryStringPointsToBeDetermined,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    4
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
            buffer::write_u16(806, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::BatteryPointsToBeDetermined => {
            buffer::write_u16(model.battery_points_to_be_determined(), buffer);
        }
        Point::BatteryStringBatteryStringPointsToBeDetermined => {
            buffer::write_u16(
                model.battery_string_battery_string_points_to_be_determined(),
                buffer,
            );
        }
    }
}

pub trait ModelAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16;

    /// Battery String Points To Be Determined
    fn battery_string_battery_string_points_to_be_determined(&self) -> u16;
}

#[repr(C)]
pub struct Model806CallbackAdapter {
    context: *mut c_void,
    battery_points_to_be_determined_callback: extern "C" fn(*const c_void) -> u16,
    battery_string_battery_string_points_to_be_determined_callback:
        extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model806CallbackAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16 {
        (self.battery_points_to_be_determined_callback)(self.context)
    }

    /// Battery String Points To Be Determined
    fn battery_string_battery_string_points_to_be_determined(&self) -> u16 {
        (self.battery_string_battery_string_points_to_be_determined_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model806StatefulAdapter {
    battery_points_to_be_determined: u16,
    battery_string_battery_string_points_to_be_determined: u16,
}

impl ModelAdapter for Model806StatefulAdapter {
    /// Battery Points To Be Determined
    fn battery_points_to_be_determined(&self) -> u16 {
        self.battery_points_to_be_determined
    }

    /// Battery String Points To Be Determined
    fn battery_string_battery_string_points_to_be_determined(&self) -> u16 {
        self.battery_string_battery_string_points_to_be_determined
    }
}
