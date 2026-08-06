use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 26;

pub static POINTS: [ReadablePoint; 15] = [
    ReadablePoint {
        reference: PointReference::Static { value: 15 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::Clear,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::InputCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::InputUnicastCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::InputNonUnicastCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::InputDiscardedCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::InputErrorCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::InputUnknownCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::OutputCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::OutputUnicastCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::OutputNonUnicastCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::OutputDiscardedCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model15 {
            point: Point::OutputErrorCount,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    Clear,
    InputCount,
    InputUnicastCount,
    InputNonUnicastCount,
    InputDiscardedCount,
    InputErrorCount,
    InputUnknownCount,
    OutputCount,
    OutputUnicastCount,
    OutputNonUnicastCount,
    OutputDiscardedCount,
    OutputErrorCount,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    26
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
        Point::Clear => {
            if let Some(value) = model.clear() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputCount => {
            if let Some(value) = model.input_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputUnicastCount => {
            if let Some(value) = model.input_unicast_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputNonUnicastCount => {
            if let Some(value) = model.input_non_unicast_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputDiscardedCount => {
            if let Some(value) = model.input_discarded_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputErrorCount => {
            if let Some(value) = model.input_error_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InputUnknownCount => {
            if let Some(value) = model.input_unknown_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputCount => {
            if let Some(value) = model.output_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputUnicastCount => {
            if let Some(value) = model.output_unicast_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputNonUnicastCount => {
            if let Some(value) = model.output_non_unicast_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputDiscardedCount => {
            if let Some(value) = model.output_discarded_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputErrorCount => {
            if let Some(value) = model.output_error_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn clear(&self) -> Option<u16> {
        None
    }

    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn set_clear(&mut self, value: u16) {}

    /// Input Count
    ///
    /// Number of bytes received
    fn input_count(&self) -> Option<u32> {
        None
    }

    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    fn input_unicast_count(&self) -> Option<u32> {
        None
    }

    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    fn input_non_unicast_count(&self) -> Option<u32> {
        None
    }

    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    fn input_discarded_count(&self) -> Option<u32> {
        None
    }

    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    fn input_error_count(&self) -> Option<u32> {
        None
    }

    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    fn input_unknown_count(&self) -> Option<u32> {
        None
    }

    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    fn output_count(&self) -> Option<u32> {
        None
    }

    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    fn output_unicast_count(&self) -> Option<u32> {
        None
    }

    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    fn output_non_unicast_count(&self) -> Option<u32> {
        None
    }

    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    fn output_discarded_count(&self) -> Option<u32> {
        None
    }

    /// Output Error Count
    ///
    /// Number of outbound error packets
    fn output_error_count(&self) -> Option<u32> {
        None
    }
}

#[repr(C)]
pub struct Model15CallbackAdapter {
    context: *mut c_void,
    clear_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_clear_callback: Option<extern "C" fn(u16, *mut c_void)>,
    input_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    input_unicast_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    input_non_unicast_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    input_discarded_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    input_error_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    input_unknown_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    output_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    output_unicast_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    output_non_unicast_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    output_discarded_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    output_error_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
}

impl ModelAdapter for Model15CallbackAdapter {
    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn clear(&self) -> Option<u16> {
        self.clear_callback.map(|callback| (callback)(self.context))
    }

    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn set_clear(&mut self, value: u16) {
        if let Some(callback) = self.set_clear_callback {
            (callback)(value, self.context);
        };
    }

    /// Input Count
    ///
    /// Number of bytes received
    fn input_count(&self) -> Option<u32> {
        self.input_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    fn input_unicast_count(&self) -> Option<u32> {
        self.input_unicast_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    fn input_non_unicast_count(&self) -> Option<u32> {
        self.input_non_unicast_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    fn input_discarded_count(&self) -> Option<u32> {
        self.input_discarded_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    fn input_error_count(&self) -> Option<u32> {
        self.input_error_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    fn input_unknown_count(&self) -> Option<u32> {
        self.input_unknown_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    fn output_count(&self) -> Option<u32> {
        self.output_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    fn output_unicast_count(&self) -> Option<u32> {
        self.output_unicast_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    fn output_non_unicast_count(&self) -> Option<u32> {
        self.output_non_unicast_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    fn output_discarded_count(&self) -> Option<u32> {
        self.output_discarded_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output Error Count
    ///
    /// Number of outbound error packets
    fn output_error_count(&self) -> Option<u32> {
        self.output_error_count_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model15StatefulAdapter {
    clear: u16,
    input_count: u32,
    input_unicast_count: u32,
    input_non_unicast_count: u32,
    input_discarded_count: u32,
    input_error_count: u32,
    input_unknown_count: u32,
    output_count: u32,
    output_unicast_count: u32,
    output_non_unicast_count: u32,
    output_discarded_count: u32,
    output_error_count: u32,
}

impl ModelAdapter for Model15StatefulAdapter {
    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn clear(&self) -> Option<u16> {
        Some(self.clear)
    }

    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn set_clear(&mut self, value: u16) {
        self.clear = value;
    }

    /// Input Count
    ///
    /// Number of bytes received
    fn input_count(&self) -> Option<u32> {
        Some(self.input_count)
    }

    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    fn input_unicast_count(&self) -> Option<u32> {
        Some(self.input_unicast_count)
    }

    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    fn input_non_unicast_count(&self) -> Option<u32> {
        Some(self.input_non_unicast_count)
    }

    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    fn input_discarded_count(&self) -> Option<u32> {
        Some(self.input_discarded_count)
    }

    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    fn input_error_count(&self) -> Option<u32> {
        Some(self.input_error_count)
    }

    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    fn input_unknown_count(&self) -> Option<u32> {
        Some(self.input_unknown_count)
    }

    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    fn output_count(&self) -> Option<u32> {
        Some(self.output_count)
    }

    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    fn output_unicast_count(&self) -> Option<u32> {
        Some(self.output_unicast_count)
    }

    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    fn output_non_unicast_count(&self) -> Option<u32> {
        Some(self.output_non_unicast_count)
    }

    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    fn output_discarded_count(&self) -> Option<u32> {
        Some(self.output_discarded_count)
    }

    /// Output Error Count
    ///
    /// Number of outbound error packets
    fn output_error_count(&self) -> Option<u32> {
        Some(self.output_error_count)
    }
}
