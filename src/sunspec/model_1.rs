use heapless::{String, Vec};

use crate::util::{RegisterReader, write_string, write_u16};

pub type Model1 = Common;

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
pub fn register_readers() -> Vec<(usize, RegisterReader<dyn ModelAdapter>), 9> {
    let mut vec: Vec<(usize, RegisterReader<dyn ModelAdapter>), 9> = Vec::new();
    vec.extend_from_slice(&[
        // Model id
        (1, |_, buf, _, _| write_u16(1_u16, buf)),
        // Model length
        (1, |_, buf, _, _| write_u16(66_u16, buf)),
        // Manufacturer
        (16, |model, buf, offset, len| {
            write_string(model.mn(), buf, offset, len)
        }),
        // Model
        (16, |model, buf, offset, len| {
            write_string(model.md(), buf, offset, len)
        }),
        // Options
        (8, |model, buf, offset, len| {
            if let Some(str) = model.opt() {
                write_string(str, buf, offset, len)
            }
        }),
        // Version
        (8, |model, buf, offset, len| {
            if let Some(str) = model.vr() {
                write_string(str, buf, offset, len)
            }
        }),
        // Serial Number
        (16, |model, buf, offset, len| {
            write_string(model.sn(), buf, offset, len)
        }),
        // Device Address
        (1, |model, buf, _, _| {
            if let Some(value) = model.da() {
                write_u16(value, buf);
            };
        }),
        // Pad
        (1, |_, _, _, _| ()),
    ])
    .unwrap();
    vec
}

pub fn size() -> u16 {
    68
}

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
