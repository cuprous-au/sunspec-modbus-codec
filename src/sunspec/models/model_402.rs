use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 22;

pub static POINTS: [ReadablePoint; 18] = [
    ReadablePoint {
        reference: PointReference::Static { value: 402 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 20 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::DcaSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::DcAhrSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::DcvSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::DcwSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::DcWhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::Rating,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 { point: Point::N },
        size: 1,
        data_type: PointType::Count,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::Event,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::VendorEvent,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 { point: Point::Amps },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::AmpHours,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::Voltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 { point: Point::Temp },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::Watts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 { point: Point::Pr },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model402 {
            point: Point::WattHours,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    DcaSf,
    DcAhrSf,
    DcvSf,
    DcwSf,
    DcWhSf,
    Rating,
    N,
    Event,
    VendorEvent,
    Amps,
    AmpHours,
    Voltage,
    Temp,
    Watts,
    Pr,
    WattHours,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::DcaSf => serialisation::write_u16(model.dca_sf(), buffer),
        Point::DcAhrSf => {
            if let Some(value) = model.dc_ahr_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DcvSf => {
            if let Some(value) = model.dcv_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DcwSf => {
            if let Some(value) = model.dcw_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DcWhSf => serialisation::write_u16(model.dc_wh_sf(), buffer),
        Point::Rating => {
            if let Some(value) = model.rating() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::N => {
            if let Some(value) = model.n() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Event => serialisation::write_u32(model.event(), buffer, offset, limit),
        Point::VendorEvent => {
            if let Some(value) = model.vendor_event() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::Amps => serialisation::write_i16(model.amps(), buffer),
        Point::AmpHours => {
            if let Some(value) = model.amp_hours() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::Voltage => {
            if let Some(value) = model.voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Watts => {
            if let Some(value) = model.watts() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::Pr => {
            if let Some(value) = model.pr() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WattHours => serialisation::write_u32(model.watt_hours(), buffer, offset, limit),
    }
}

pub trait ModelAdapter {
    /// Current scale factor
    fn dca_sf(&self) -> u16;

    /// Amp-hour scale factor
    fn dc_ahr_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage scale factor
    fn dcv_sf(&self) -> Option<u16> {
        None
    }

    /// Power scale factor
    fn dcw_sf(&self) -> Option<u16> {
        None
    }

    /// Energy scale factor
    fn dc_wh_sf(&self) -> u16;

    /// Rating
    ///
    /// Maximum DC Current Rating
    fn rating(&self) -> Option<u16> {
        None
    }

    /// N
    ///
    /// Number of Inputs
    fn n(&self) -> Option<u16> {
        None
    }

    /// Event
    ///
    /// Events
    fn event(&self) -> u32;

    /// Vendor Event
    ///
    /// Vendor defined events
    fn vendor_event(&self) -> Option<u32> {
        None
    }

    /// Amps
    ///
    /// Total measured current
    fn amps(&self) -> i16;

    /// Amp-hours
    ///
    /// Total metered Amp-hours
    fn amp_hours(&self) -> Option<u32> {
        None
    }

    /// Voltage
    ///
    /// Output Voltage
    fn voltage(&self) -> Option<u16> {
        None
    }

    /// Temp
    ///
    /// Internal operating temperature
    fn temp(&self) -> Option<i16> {
        None
    }

    /// Watts
    ///
    /// Output power
    fn watts(&self) -> Option<i16> {
        None
    }

    /// PR
    ///
    /// DC Performance ratio value
    fn pr(&self) -> Option<u16> {
        None
    }

    /// Watt-hours
    ///
    /// Output energy
    fn watt_hours(&self) -> u32;
}

#[repr(C)]
pub struct Model402CallbackAdapter {
    dca_sf_callback: extern "C" fn() -> u16,
    dc_ahr_sf_callback: Option<extern "C" fn() -> u16>,
    dcv_sf_callback: Option<extern "C" fn() -> u16>,
    dcw_sf_callback: Option<extern "C" fn() -> u16>,
    dc_wh_sf_callback: extern "C" fn() -> u16,
    rating_callback: Option<extern "C" fn() -> u16>,
    n_callback: Option<extern "C" fn() -> u16>,
    event_callback: extern "C" fn() -> u32,
    vendor_event_callback: Option<extern "C" fn() -> u32>,
    amps_callback: extern "C" fn() -> i16,
    amp_hours_callback: Option<extern "C" fn() -> u32>,
    voltage_callback: Option<extern "C" fn() -> u16>,
    temp_callback: Option<extern "C" fn() -> i16>,
    watts_callback: Option<extern "C" fn() -> i16>,
    pr_callback: Option<extern "C" fn() -> u16>,
    watt_hours_callback: extern "C" fn() -> u32,
}

impl ModelAdapter for Model402CallbackAdapter {
    /// Current scale factor
    fn dca_sf(&self) -> u16 {
        (self.dca_sf_callback)()
    }

    /// Amp-hour scale factor
    fn dc_ahr_sf(&self) -> Option<u16> {
        self.dc_ahr_sf_callback.map(|callback| (callback)())
    }

    /// Voltage scale factor
    fn dcv_sf(&self) -> Option<u16> {
        self.dcv_sf_callback.map(|callback| (callback)())
    }

    /// Power scale factor
    fn dcw_sf(&self) -> Option<u16> {
        self.dcw_sf_callback.map(|callback| (callback)())
    }

    /// Energy scale factor
    fn dc_wh_sf(&self) -> u16 {
        (self.dc_wh_sf_callback)()
    }

    /// Rating
    ///
    /// Maximum DC Current Rating
    fn rating(&self) -> Option<u16> {
        self.rating_callback.map(|callback| (callback)())
    }

    /// N
    ///
    /// Number of Inputs
    fn n(&self) -> Option<u16> {
        self.n_callback.map(|callback| (callback)())
    }

    /// Event
    ///
    /// Events
    fn event(&self) -> u32 {
        (self.event_callback)()
    }

    /// Vendor Event
    ///
    /// Vendor defined events
    fn vendor_event(&self) -> Option<u32> {
        self.vendor_event_callback.map(|callback| (callback)())
    }

    /// Amps
    ///
    /// Total measured current
    fn amps(&self) -> i16 {
        (self.amps_callback)()
    }

    /// Amp-hours
    ///
    /// Total metered Amp-hours
    fn amp_hours(&self) -> Option<u32> {
        self.amp_hours_callback.map(|callback| (callback)())
    }

    /// Voltage
    ///
    /// Output Voltage
    fn voltage(&self) -> Option<u16> {
        self.voltage_callback.map(|callback| (callback)())
    }

    /// Temp
    ///
    /// Internal operating temperature
    fn temp(&self) -> Option<i16> {
        self.temp_callback.map(|callback| (callback)())
    }

    /// Watts
    ///
    /// Output power
    fn watts(&self) -> Option<i16> {
        self.watts_callback.map(|callback| (callback)())
    }

    /// PR
    ///
    /// DC Performance ratio value
    fn pr(&self) -> Option<u16> {
        self.pr_callback.map(|callback| (callback)())
    }

    /// Watt-hours
    ///
    /// Output energy
    fn watt_hours(&self) -> u32 {
        (self.watt_hours_callback)()
    }
}
