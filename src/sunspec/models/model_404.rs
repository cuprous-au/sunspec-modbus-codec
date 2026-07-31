use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 27;

pub static POINTS: [ReadablePoint; 23] = [
    ReadablePoint {
        reference: PointReference::Static { value: 404 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 25 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::DcaSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::DcAhrSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::DcvSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::DcwSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::DcWhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::Rating,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 { point: Point::N },
        size: 1,
        data_type: PointType::Count,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::Event,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::VendorEvent,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 { point: Point::Amps },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::AmpHours,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::Voltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 { point: Point::Temp },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::Watts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 { point: Point::Pr },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::WattHours,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::InDcaSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::InDcAhrSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::InDcvSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::InDcwSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::InDcWhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
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
    InDcaSf,
    InDcAhrSf,
    InDcvSf,
    InDcwSf,
    InDcWhSf,
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
        Point::DcWhSf => {
            if let Some(value) = model.dc_wh_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Rating => serialisation::write_u16(model.rating(), buffer),
        Point::N => serialisation::write_u16(model.n(), buffer),
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
                serialisation::write_i16(value, buffer);
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
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WattHours => {
            if let Some(value) = model.watt_hours() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::InDcaSf => {
            if let Some(value) = model.in_dca_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::InDcAhrSf => {
            if let Some(value) = model.in_dc_ahr_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::InDcvSf => {
            if let Some(value) = model.in_dcv_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::InDcwSf => {
            if let Some(value) = model.in_dcw_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::InDcWhSf => {
            if let Some(value) = model.in_dc_wh_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
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
    fn dc_wh_sf(&self) -> Option<u16> {
        None
    }

    /// Rating
    ///
    /// Maximum DC Current Rating
    fn rating(&self) -> u16;

    /// N
    ///
    /// Number of Inputs
    fn n(&self) -> u16;

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
    fn voltage(&self) -> Option<i16> {
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
    fn pr(&self) -> Option<i16> {
        None
    }

    /// Watt-hours
    ///
    /// Output energy
    fn watt_hours(&self) -> Option<u32> {
        None
    }

    /// Current scale factor for inputs
    fn in_dca_sf(&self) -> Option<u16> {
        None
    }

    /// Amp-hour scale factor for inputs
    fn in_dc_ahr_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage scale factor for inputs
    fn in_dcv_sf(&self) -> Option<u16> {
        None
    }

    /// Power scale factor for inputs
    fn in_dcw_sf(&self) -> Option<u16> {
        None
    }

    /// Energy scale factor for inputs
    fn in_dc_wh_sf(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model404CallbackAdapter {
    dca_sf_callback: extern "C" fn() -> u16,
    dc_ahr_sf_callback: Option<extern "C" fn() -> u16>,
    dcv_sf_callback: Option<extern "C" fn() -> u16>,
    dcw_sf_callback: Option<extern "C" fn() -> u16>,
    dc_wh_sf_callback: Option<extern "C" fn() -> u16>,
    rating_callback: extern "C" fn() -> u16,
    n_callback: extern "C" fn() -> u16,
    event_callback: extern "C" fn() -> u32,
    vendor_event_callback: Option<extern "C" fn() -> u32>,
    amps_callback: extern "C" fn() -> i16,
    amp_hours_callback: Option<extern "C" fn() -> u32>,
    voltage_callback: Option<extern "C" fn() -> i16>,
    temp_callback: Option<extern "C" fn() -> i16>,
    watts_callback: Option<extern "C" fn() -> i16>,
    pr_callback: Option<extern "C" fn() -> i16>,
    watt_hours_callback: Option<extern "C" fn() -> u32>,
    in_dca_sf_callback: Option<extern "C" fn() -> u16>,
    in_dc_ahr_sf_callback: Option<extern "C" fn() -> u16>,
    in_dcv_sf_callback: Option<extern "C" fn() -> u16>,
    in_dcw_sf_callback: Option<extern "C" fn() -> u16>,
    in_dc_wh_sf_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model404CallbackAdapter {
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
    fn dc_wh_sf(&self) -> Option<u16> {
        self.dc_wh_sf_callback.map(|callback| (callback)())
    }

    /// Rating
    ///
    /// Maximum DC Current Rating
    fn rating(&self) -> u16 {
        (self.rating_callback)()
    }

    /// N
    ///
    /// Number of Inputs
    fn n(&self) -> u16 {
        (self.n_callback)()
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
    fn voltage(&self) -> Option<i16> {
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
    fn pr(&self) -> Option<i16> {
        self.pr_callback.map(|callback| (callback)())
    }

    /// Watt-hours
    ///
    /// Output energy
    fn watt_hours(&self) -> Option<u32> {
        self.watt_hours_callback.map(|callback| (callback)())
    }

    /// Current scale factor for inputs
    fn in_dca_sf(&self) -> Option<u16> {
        self.in_dca_sf_callback.map(|callback| (callback)())
    }

    /// Amp-hour scale factor for inputs
    fn in_dc_ahr_sf(&self) -> Option<u16> {
        self.in_dc_ahr_sf_callback.map(|callback| (callback)())
    }

    /// Voltage scale factor for inputs
    fn in_dcv_sf(&self) -> Option<u16> {
        self.in_dcv_sf_callback.map(|callback| (callback)())
    }

    /// Power scale factor for inputs
    fn in_dcw_sf(&self) -> Option<u16> {
        self.in_dcw_sf_callback.map(|callback| (callback)())
    }

    /// Energy scale factor for inputs
    fn in_dc_wh_sf(&self) -> Option<u16> {
        self.in_dc_wh_sf_callback.map(|callback| (callback)())
    }
}
