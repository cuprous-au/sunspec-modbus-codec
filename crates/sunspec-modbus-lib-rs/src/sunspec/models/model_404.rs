use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 41;

pub static POINTS: [ReadablePoint; 33] = [
    ReadablePoint {
        reference: PointReference::Static { value: 404 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::ModelLength,
        },
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
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringId,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringInputEvent,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringInputEventVendor,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringAmps,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringAmpHours,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringVoltage,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringWatts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringWattHours,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringPr,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model404 {
            point: Point::StringN,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
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
    StringId,
    StringInputEvent,
    StringInputEventVendor,
    StringAmps,
    StringAmpHours,
    StringVoltage,
    StringWatts,
    StringWattHours,
    StringPr,
    StringN,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    41
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
        Point::DcaSf => {
            buffer::write_u16(model.dca_sf(), buffer);
        }
        Point::DcAhrSf => {
            if let Some(value) = model.dc_ahr_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcvSf => {
            if let Some(value) = model.dcv_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcwSf => {
            if let Some(value) = model.dcw_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcWhSf => {
            if let Some(value) = model.dc_wh_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Rating => {
            buffer::write_u16(model.rating(), buffer);
        }
        Point::N => {
            buffer::write_u16(model.n(), buffer);
        }
        Point::Event => {
            buffer::write_u32(model.event(), buffer, offset, limit);
        }
        Point::VendorEvent => {
            if let Some(value) = model.vendor_event() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Amps => {
            buffer::write_i16(model.amps(), buffer);
        }
        Point::AmpHours => {
            if let Some(value) = model.amp_hours() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Voltage => {
            if let Some(value) = model.voltage() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Watts => {
            if let Some(value) = model.watts() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Pr => {
            if let Some(value) = model.pr() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WattHours => {
            if let Some(value) = model.watt_hours() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InDcaSf => {
            if let Some(value) = model.in_dca_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InDcAhrSf => {
            if let Some(value) = model.in_dc_ahr_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InDcvSf => {
            if let Some(value) = model.in_dcv_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InDcwSf => {
            if let Some(value) = model.in_dcw_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InDcWhSf => {
            if let Some(value) = model.in_dc_wh_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringId => {
            buffer::write_u16(model.string_id(), buffer);
        }
        Point::StringInputEvent => {
            buffer::write_u32(model.string_input_event(), buffer, offset, limit);
        }
        Point::StringInputEventVendor => {
            if let Some(value) = model.string_input_event_vendor() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringAmps => {
            buffer::write_i16(model.string_amps(), buffer);
        }
        Point::StringAmpHours => {
            if let Some(value) = model.string_amp_hours() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringVoltage => {
            if let Some(value) = model.string_voltage() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringWatts => {
            if let Some(value) = model.string_watts() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringWattHours => {
            if let Some(value) = model.string_watt_hours() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringPr => {
            if let Some(value) = model.string_pr() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StringN => {
            if let Some(value) = model.string_n() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
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

    /// ID
    ///
    /// Uniquely identifies this input set
    fn string_id(&self) -> u16;

    /// Input Event
    ///
    /// String Input Event Flags
    fn string_input_event(&self) -> u32;

    /// Input Event Vendor
    ///
    /// String Input Vendor Event Flags
    fn string_input_event_vendor(&self) -> Option<u32> {
        None
    }

    /// Amps
    ///
    /// String Input Current
    fn string_amps(&self) -> i16;

    /// Amp-hours
    ///
    /// String Input Amp-Hours
    fn string_amp_hours(&self) -> Option<u32> {
        None
    }

    /// Voltage
    ///
    /// String Input Voltage
    fn string_voltage(&self) -> Option<i16> {
        None
    }

    /// Watts
    ///
    /// String Input Power
    fn string_watts(&self) -> Option<i16> {
        None
    }

    /// Watt-hours
    ///
    /// String Input Energy
    fn string_watt_hours(&self) -> Option<u32> {
        None
    }

    /// PR
    ///
    /// String Performance Ratio
    fn string_pr(&self) -> Option<u16> {
        None
    }

    /// N
    ///
    /// Number of modules in this input string
    fn string_n(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model404CallbackAdapter {
    context: *mut c_void,
    dca_sf_callback: extern "C" fn(*const c_void) -> u16,
    dc_ahr_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dcv_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dcw_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dc_wh_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    rating_callback: extern "C" fn(*const c_void) -> u16,
    n_callback: extern "C" fn(*const c_void) -> u16,
    event_callback: extern "C" fn(*const c_void) -> u32,
    vendor_event_callback: Option<extern "C" fn(*const c_void) -> u32>,
    amps_callback: extern "C" fn(*const c_void) -> i16,
    amp_hours_callback: Option<extern "C" fn(*const c_void) -> u32>,
    voltage_callback: Option<extern "C" fn(*const c_void) -> i16>,
    temp_callback: Option<extern "C" fn(*const c_void) -> i16>,
    watts_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pr_callback: Option<extern "C" fn(*const c_void) -> i16>,
    watt_hours_callback: Option<extern "C" fn(*const c_void) -> u32>,
    in_dca_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    in_dc_ahr_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    in_dcv_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    in_dcw_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    in_dc_wh_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    string_id_callback: extern "C" fn(*const c_void) -> u16,
    string_input_event_callback: extern "C" fn(*const c_void) -> u32,
    string_input_event_vendor_callback: Option<extern "C" fn(*const c_void) -> u32>,
    string_amps_callback: extern "C" fn(*const c_void) -> i16,
    string_amp_hours_callback: Option<extern "C" fn(*const c_void) -> u32>,
    string_voltage_callback: Option<extern "C" fn(*const c_void) -> i16>,
    string_watts_callback: Option<extern "C" fn(*const c_void) -> i16>,
    string_watt_hours_callback: Option<extern "C" fn(*const c_void) -> u32>,
    string_pr_callback: Option<extern "C" fn(*const c_void) -> u16>,
    string_n_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model404CallbackAdapter {
    /// Current scale factor
    fn dca_sf(&self) -> u16 {
        (self.dca_sf_callback)(self.context)
    }

    /// Amp-hour scale factor
    fn dc_ahr_sf(&self) -> Option<u16> {
        self.dc_ahr_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage scale factor
    fn dcv_sf(&self) -> Option<u16> {
        self.dcv_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power scale factor
    fn dcw_sf(&self) -> Option<u16> {
        self.dcw_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Energy scale factor
    fn dc_wh_sf(&self) -> Option<u16> {
        self.dc_wh_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Rating
    ///
    /// Maximum DC Current Rating
    fn rating(&self) -> u16 {
        (self.rating_callback)(self.context)
    }

    /// N
    ///
    /// Number of Inputs
    fn n(&self) -> u16 {
        (self.n_callback)(self.context)
    }

    /// Event
    ///
    /// Events
    fn event(&self) -> u32 {
        (self.event_callback)(self.context)
    }

    /// Vendor Event
    ///
    /// Vendor defined events
    fn vendor_event(&self) -> Option<u32> {
        self.vendor_event_callback
            .map(|callback| (callback)(self.context))
    }

    /// Amps
    ///
    /// Total measured current
    fn amps(&self) -> i16 {
        (self.amps_callback)(self.context)
    }

    /// Amp-hours
    ///
    /// Total metered Amp-hours
    fn amp_hours(&self) -> Option<u32> {
        self.amp_hours_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage
    ///
    /// Output Voltage
    fn voltage(&self) -> Option<i16> {
        self.voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Temp
    ///
    /// Internal operating temperature
    fn temp(&self) -> Option<i16> {
        self.temp_callback.map(|callback| (callback)(self.context))
    }

    /// Watts
    ///
    /// Output power
    fn watts(&self) -> Option<i16> {
        self.watts_callback.map(|callback| (callback)(self.context))
    }

    /// PR
    ///
    /// DC Performance ratio value
    fn pr(&self) -> Option<i16> {
        self.pr_callback.map(|callback| (callback)(self.context))
    }

    /// Watt-hours
    ///
    /// Output energy
    fn watt_hours(&self) -> Option<u32> {
        self.watt_hours_callback
            .map(|callback| (callback)(self.context))
    }

    /// Current scale factor for inputs
    fn in_dca_sf(&self) -> Option<u16> {
        self.in_dca_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Amp-hour scale factor for inputs
    fn in_dc_ahr_sf(&self) -> Option<u16> {
        self.in_dc_ahr_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage scale factor for inputs
    fn in_dcv_sf(&self) -> Option<u16> {
        self.in_dcv_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power scale factor for inputs
    fn in_dcw_sf(&self) -> Option<u16> {
        self.in_dcw_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Energy scale factor for inputs
    fn in_dc_wh_sf(&self) -> Option<u16> {
        self.in_dc_wh_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// ID
    ///
    /// Uniquely identifies this input set
    fn string_id(&self) -> u16 {
        (self.string_id_callback)(self.context)
    }

    /// Input Event
    ///
    /// String Input Event Flags
    fn string_input_event(&self) -> u32 {
        (self.string_input_event_callback)(self.context)
    }

    /// Input Event Vendor
    ///
    /// String Input Vendor Event Flags
    fn string_input_event_vendor(&self) -> Option<u32> {
        self.string_input_event_vendor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Amps
    ///
    /// String Input Current
    fn string_amps(&self) -> i16 {
        (self.string_amps_callback)(self.context)
    }

    /// Amp-hours
    ///
    /// String Input Amp-Hours
    fn string_amp_hours(&self) -> Option<u32> {
        self.string_amp_hours_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage
    ///
    /// String Input Voltage
    fn string_voltage(&self) -> Option<i16> {
        self.string_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts
    ///
    /// String Input Power
    fn string_watts(&self) -> Option<i16> {
        self.string_watts_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watt-hours
    ///
    /// String Input Energy
    fn string_watt_hours(&self) -> Option<u32> {
        self.string_watt_hours_callback
            .map(|callback| (callback)(self.context))
    }

    /// PR
    ///
    /// String Performance Ratio
    fn string_pr(&self) -> Option<u16> {
        self.string_pr_callback
            .map(|callback| (callback)(self.context))
    }

    /// N
    ///
    /// Number of modules in this input string
    fn string_n(&self) -> Option<u16> {
        self.string_n_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model404StatefulAdapter {
    dca_sf: u16,
    dc_ahr_sf: u16,
    dcv_sf: u16,
    dcw_sf: u16,
    dc_wh_sf: u16,
    rating: u16,
    n: u16,
    event: u32,
    vendor_event: u32,
    amps: i16,
    amp_hours: u32,
    voltage: i16,
    temp: i16,
    watts: i16,
    pr: i16,
    watt_hours: u32,
    in_dca_sf: u16,
    in_dc_ahr_sf: u16,
    in_dcv_sf: u16,
    in_dcw_sf: u16,
    in_dc_wh_sf: u16,
    string_id: u16,
    string_input_event: u32,
    string_input_event_vendor: u32,
    string_amps: i16,
    string_amp_hours: u32,
    string_voltage: i16,
    string_watts: i16,
    string_watt_hours: u32,
    string_pr: u16,
    string_n: u16,
}

impl ModelAdapter for Model404StatefulAdapter {
    /// Current scale factor
    fn dca_sf(&self) -> u16 {
        self.dca_sf
    }

    /// Amp-hour scale factor
    fn dc_ahr_sf(&self) -> Option<u16> {
        Some(self.dc_ahr_sf)
    }

    /// Voltage scale factor
    fn dcv_sf(&self) -> Option<u16> {
        Some(self.dcv_sf)
    }

    /// Power scale factor
    fn dcw_sf(&self) -> Option<u16> {
        Some(self.dcw_sf)
    }

    /// Energy scale factor
    fn dc_wh_sf(&self) -> Option<u16> {
        Some(self.dc_wh_sf)
    }

    /// Rating
    ///
    /// Maximum DC Current Rating
    fn rating(&self) -> u16 {
        self.rating
    }

    /// N
    ///
    /// Number of Inputs
    fn n(&self) -> u16 {
        self.n
    }

    /// Event
    ///
    /// Events
    fn event(&self) -> u32 {
        self.event
    }

    /// Vendor Event
    ///
    /// Vendor defined events
    fn vendor_event(&self) -> Option<u32> {
        Some(self.vendor_event)
    }

    /// Amps
    ///
    /// Total measured current
    fn amps(&self) -> i16 {
        self.amps
    }

    /// Amp-hours
    ///
    /// Total metered Amp-hours
    fn amp_hours(&self) -> Option<u32> {
        Some(self.amp_hours)
    }

    /// Voltage
    ///
    /// Output Voltage
    fn voltage(&self) -> Option<i16> {
        Some(self.voltage)
    }

    /// Temp
    ///
    /// Internal operating temperature
    fn temp(&self) -> Option<i16> {
        Some(self.temp)
    }

    /// Watts
    ///
    /// Output power
    fn watts(&self) -> Option<i16> {
        Some(self.watts)
    }

    /// PR
    ///
    /// DC Performance ratio value
    fn pr(&self) -> Option<i16> {
        Some(self.pr)
    }

    /// Watt-hours
    ///
    /// Output energy
    fn watt_hours(&self) -> Option<u32> {
        Some(self.watt_hours)
    }

    /// Current scale factor for inputs
    fn in_dca_sf(&self) -> Option<u16> {
        Some(self.in_dca_sf)
    }

    /// Amp-hour scale factor for inputs
    fn in_dc_ahr_sf(&self) -> Option<u16> {
        Some(self.in_dc_ahr_sf)
    }

    /// Voltage scale factor for inputs
    fn in_dcv_sf(&self) -> Option<u16> {
        Some(self.in_dcv_sf)
    }

    /// Power scale factor for inputs
    fn in_dcw_sf(&self) -> Option<u16> {
        Some(self.in_dcw_sf)
    }

    /// Energy scale factor for inputs
    fn in_dc_wh_sf(&self) -> Option<u16> {
        Some(self.in_dc_wh_sf)
    }

    /// ID
    ///
    /// Uniquely identifies this input set
    fn string_id(&self) -> u16 {
        self.string_id
    }

    /// Input Event
    ///
    /// String Input Event Flags
    fn string_input_event(&self) -> u32 {
        self.string_input_event
    }

    /// Input Event Vendor
    ///
    /// String Input Vendor Event Flags
    fn string_input_event_vendor(&self) -> Option<u32> {
        Some(self.string_input_event_vendor)
    }

    /// Amps
    ///
    /// String Input Current
    fn string_amps(&self) -> i16 {
        self.string_amps
    }

    /// Amp-hours
    ///
    /// String Input Amp-Hours
    fn string_amp_hours(&self) -> Option<u32> {
        Some(self.string_amp_hours)
    }

    /// Voltage
    ///
    /// String Input Voltage
    fn string_voltage(&self) -> Option<i16> {
        Some(self.string_voltage)
    }

    /// Watts
    ///
    /// String Input Power
    fn string_watts(&self) -> Option<i16> {
        Some(self.string_watts)
    }

    /// Watt-hours
    ///
    /// String Input Energy
    fn string_watt_hours(&self) -> Option<u32> {
        Some(self.string_watt_hours)
    }

    /// PR
    ///
    /// String Performance Ratio
    fn string_pr(&self) -> Option<u16> {
        Some(self.string_pr)
    }

    /// N
    ///
    /// Number of modules in this input string
    fn string_n(&self) -> Option<u16> {
        Some(self.string_n)
    }
}
