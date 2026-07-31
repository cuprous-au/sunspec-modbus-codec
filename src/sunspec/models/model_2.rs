use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 16;

pub static POINTS: [ReadablePoint; 12] = [
    ReadablePoint {
        reference: PointReference::Static { value: 2 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 14 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 { point: Point::Aid },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 { point: Point::N },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 { point: Point::Un },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 {
            point: Point::Status,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 {
            point: Point::VendorStatus,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 {
            point: Point::EventCode,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 {
            point: Point::VendorEventCode,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 {
            point: Point::Control,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 {
            point: Point::VendorControl,
        },
        size: 2,
        data_type: PointType::Enum32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model2 {
            point: Point::ControlValue,
        },
        size: 2,
        data_type: PointType::Enum32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Aid,
    N,
    Un,
    Status,
    VendorStatus,
    EventCode,
    VendorEventCode,
    Control,
    VendorControl,
    ControlValue,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Aid => serialisation::write_u16(model.aid(), buffer),
        Point::N => serialisation::write_u16(model.n(), buffer),
        Point::Un => serialisation::write_u16(model.un(), buffer),
        Point::Status => serialisation::write_u16(model.status() as u16, buffer),
        Point::VendorStatus => {
            if let Some(value) = model.vendor_status() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::EventCode => serialisation::write_u32(model.event_code(), buffer, offset, limit),
        Point::VendorEventCode => {
            if let Some(value) = model.vendor_event_code() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::Control => {
            if let Some(value) = model.control() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::VendorControl => {
            if let Some(value) = model.vendor_control() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ControlValue => {
            if let Some(value) = model.control_value() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
    }
}

pub trait ModelAdapter {
    /// AID
    ///
    /// Aggregated model id
    fn aid(&self) -> u16;

    /// N
    ///
    /// Number of aggregated models
    fn n(&self) -> u16;

    /// UN
    ///
    /// Update Number. Incrementing number each time the mapping is changed. If the number is not changed from the last reading the direct access to a specific offset will result in reading the same logical model as before. Otherwise the entire model must be read to refresh the changes
    fn un(&self) -> u16;

    /// Status
    ///
    /// Enumerated status code
    fn status(&self) -> St;

    /// Vendor Status
    ///
    /// Vendor specific status code
    fn vendor_status(&self) -> Option<u16> {
        None
    }

    /// Event Code
    ///
    /// Bitmask event code
    fn event_code(&self) -> u32;

    /// Vendor Event Code
    ///
    /// Vendor specific event code
    fn vendor_event_code(&self) -> Option<u32> {
        None
    }

    /// Control
    ///
    /// Control register for all aggregated devices
    fn control(&self) -> Option<Ctl> {
        None
    }

    /// Vendor Control
    ///
    /// Vendor control register for all aggregated devices
    fn vendor_control(&self) -> Option<u32> {
        None
    }

    /// Control Value
    ///
    /// Numerical value used as a parameter to the control
    fn control_value(&self) -> Option<u32> {
        None
    }
}

#[repr(u16)]
pub enum St {
    Off = 1,
    On = 2,
    Full = 3,
    Fault = 4,
}

#[repr(u16)]
pub enum Ctl {
    None = 0,
    Automatic = 1,
    ForceOff = 2,
    Test = 3,
    Throttle = 4,
}

#[repr(C)]
pub struct Model2CallbackAdapter {
    aid_callback: extern "C" fn() -> u16,
    n_callback: extern "C" fn() -> u16,
    un_callback: extern "C" fn() -> u16,
    status_callback: extern "C" fn() -> St,
    vendor_status_callback: Option<extern "C" fn() -> u16>,
    event_code_callback: extern "C" fn() -> u32,
    vendor_event_code_callback: Option<extern "C" fn() -> u32>,
    control_callback: Option<extern "C" fn() -> Ctl>,
    vendor_control_callback: Option<extern "C" fn() -> u32>,
    control_value_callback: Option<extern "C" fn() -> u32>,
}

impl ModelAdapter for Model2CallbackAdapter {
    /// AID
    ///
    /// Aggregated model id
    fn aid(&self) -> u16 {
        (self.aid_callback)()
    }

    /// N
    ///
    /// Number of aggregated models
    fn n(&self) -> u16 {
        (self.n_callback)()
    }

    /// UN
    ///
    /// Update Number. Incrementing number each time the mapping is changed. If the number is not changed from the last reading the direct access to a specific offset will result in reading the same logical model as before. Otherwise the entire model must be read to refresh the changes
    fn un(&self) -> u16 {
        (self.un_callback)()
    }

    /// Status
    ///
    /// Enumerated status code
    fn status(&self) -> St {
        (self.status_callback)()
    }

    /// Vendor Status
    ///
    /// Vendor specific status code
    fn vendor_status(&self) -> Option<u16> {
        self.vendor_status_callback.map(|callback| (callback)())
    }

    /// Event Code
    ///
    /// Bitmask event code
    fn event_code(&self) -> u32 {
        (self.event_code_callback)()
    }

    /// Vendor Event Code
    ///
    /// Vendor specific event code
    fn vendor_event_code(&self) -> Option<u32> {
        self.vendor_event_code_callback.map(|callback| (callback)())
    }

    /// Control
    ///
    /// Control register for all aggregated devices
    fn control(&self) -> Option<Ctl> {
        self.control_callback.map(|callback| (callback)())
    }

    /// Vendor Control
    ///
    /// Vendor control register for all aggregated devices
    fn vendor_control(&self) -> Option<u32> {
        self.vendor_control_callback.map(|callback| (callback)())
    }

    /// Control Value
    ///
    /// Numerical value used as a parameter to the control
    fn control_value(&self) -> Option<u32> {
        self.control_value_callback.map(|callback| (callback)())
    }
}
