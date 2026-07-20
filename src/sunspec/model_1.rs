use heapless::String;

use crate::{
    sunspec::core::{PointReference, PointType, ReadablePoint},
    util::{write_string, write_u16},
};

pub type Model1 = Common;

#[derive(Debug)]
pub enum Point {
    Id,
    L,
    Mn,
    Md,
    Opt,
    Vr,
    Sn,
    Da,
}

/// All SunSpec compliant devices must include this as the first model
pub struct Common {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    mn: String<32>,
    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    md: String<32>,
    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    opt: Option<String<16>>,
    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    vr: Option<String<16>>,
    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    sn: String<32>,
    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    da: Option<u16>,
}

// BELOW HAS BEEN MANUALLY EDITED TO ESTABLISH THE PATTERN
// CODEGEN YET TO BE UPDATED
pub static POINTS: [ReadablePoint; 9] = [
    // Model id
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Id },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    // Model length
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::L },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    // Manufacturer
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Mn },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    // Model
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Md },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    // Options
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Opt },
        size: 8,
        data_type: PointType::String,
        writeable: false,
    },
    // Version
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Vr },
        size: 8,
        data_type: PointType::String,
        writeable: false,
    },
    // Serial Number
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Sn },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    // Device Address
    ReadablePoint {
        reference: PointReference::Model1 { point: Point::Da },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    // Padding
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) -> () {
    match point {
        Point::Id => write_u16(1_u16, buffer),
        Point::L => write_u16(66_u16, buffer),
        Point::Mn => write_string(model.mn(), buffer, offset, limit),
        Point::Md => write_string(model.md(), buffer, offset, limit),
        Point::Opt => {
            if let Some(str) = model.opt() {
                write_string(str, buffer, offset, limit)
            }
        }
        Point::Vr => {
            if let Some(str) = model.vr() {
                write_string(str, buffer, offset, limit)
            }
        }
        Point::Sn => write_string(model.sn(), buffer, offset, limit),
        Point::Da => {
            if let Some(value) = model.da() {
                write_u16(value, buffer);
            }
        }
    }
}

pub const SIZE: u16 = 68;

pub trait ModelAdapter {
    /// Model ID
    ///
    /// Model identifier
    // fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    // fn l(&self) -> u16;

    /// Manufacturer
    ///
    /// Well known value registered with SunSpec for compliance
    fn mn(&self) -> String<32>;

    /// Model
    ///
    /// Manufacturer specific value (32 chars)
    fn md(&self) -> String<32>;

    /// Options
    ///
    /// Manufacturer specific value (16 chars)
    fn opt(&self) -> Option<String<16>> {
        None
    }

    /// Version
    ///
    /// Manufacturer specific value (16 chars)
    fn vr(&self) -> Option<String<16>> {
        None
    }

    /// Serial Number
    ///
    /// Manufacturer specific value (32 chars)
    fn sn(&self) -> String<32>;

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn da(&self) -> Option<u16> {
        None
    }

    /// Device Address
    ///
    /// Modbus device address
    ///
    /// This point is mandatory for all SunSpec RTU devices and, for those devices, they must support values from 1-247.
    fn set_da(&mut self, value: u16) {}
}
