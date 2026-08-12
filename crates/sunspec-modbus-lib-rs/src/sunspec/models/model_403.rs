use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 26;

static POINTS: [PointDetails<()>; 20] = [
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
        point: |()| Point::DcaSf,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::DcAhrSf,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::DcvSf,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::Rating,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::N,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::Event,
        size: 2,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::VendorEvent,
        size: 2,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::Amps,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::AmpHours,
        size: 2,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::Voltage,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::Temp,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::InDcaSf,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::InDcAhrSf,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::StringId,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::StringInputEvent,
        size: 2,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::StringInputEventVendor,
        size: 2,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::StringAmps,
        size: 1,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::StringAmpHours,
        size: 2,
        start_address: 24,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    DcaSf,
    DcAhrSf,
    DcvSf,
    Rating,
    N,
    Event,
    VendorEvent,
    Amps,
    AmpHours,
    Voltage,
    Temp,
    InDcaSf,
    InDcAhrSf,
    StringId,
    StringInputEvent,
    StringInputEventVendor,
    StringAmps,
    StringAmpHours,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    26
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
            buffer::write_u16(403, buffer);
        }
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
                buffer::zero(buffer, 1);
            }
        }
        Point::DcvSf => {
            if let Some(value) = model.dcv_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
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
                buffer::zero(buffer, 2);
            }
        }
        Point::Amps => {
            buffer::write_i16(model.amps(), buffer);
        }
        Point::AmpHours => {
            if let Some(value) = model.amp_hours() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Voltage => {
            if let Some(value) = model.voltage() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Temp => {
            if let Some(value) = model.temp() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InDcaSf => {
            if let Some(value) = model.in_dca_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::InDcAhrSf => {
            if let Some(value) = model.in_dc_ahr_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
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
                buffer::zero(buffer, 2);
            }
        }
        Point::StringAmps => {
            buffer::write_i16(model.string_amps(), buffer);
        }
        Point::StringAmpHours => {
            if let Some(value) = model.string_amp_hours() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
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

    /// Current scale factor for inputs
    fn in_dca_sf(&self) -> Option<u16> {
        None
    }

    /// Amp-hour scale factor for inputs
    fn in_dc_ahr_sf(&self) -> Option<u16> {
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
}

#[repr(C)]
pub struct Model403CallbackAdapter {
    context: *mut c_void,
    dca_sf_callback: extern "C" fn(*const c_void) -> u16,
    dc_ahr_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dcv_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    rating_callback: extern "C" fn(*const c_void) -> u16,
    n_callback: extern "C" fn(*const c_void) -> u16,
    event_callback: extern "C" fn(*const c_void) -> u32,
    vendor_event_callback: Option<extern "C" fn(*const c_void) -> u32>,
    amps_callback: extern "C" fn(*const c_void) -> i16,
    amp_hours_callback: Option<extern "C" fn(*const c_void) -> u32>,
    voltage_callback: Option<extern "C" fn(*const c_void) -> i16>,
    temp_callback: Option<extern "C" fn(*const c_void) -> i16>,
    in_dca_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    in_dc_ahr_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    string_id_callback: extern "C" fn(*const c_void) -> u16,
    string_input_event_callback: extern "C" fn(*const c_void) -> u32,
    string_input_event_vendor_callback: Option<extern "C" fn(*const c_void) -> u32>,
    string_amps_callback: extern "C" fn(*const c_void) -> i16,
    string_amp_hours_callback: Option<extern "C" fn(*const c_void) -> u32>,
}

impl ModelAdapter for Model403CallbackAdapter {
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
}

#[repr(C)]
pub struct Model403StatefulAdapter {
    dca_sf: u16,
    dc_ahr_sf: u16,
    dcv_sf: u16,
    rating: u16,
    n: u16,
    event: u32,
    vendor_event: u32,
    amps: i16,
    amp_hours: u32,
    voltage: i16,
    temp: i16,
    in_dca_sf: u16,
    in_dc_ahr_sf: u16,
    string_id: u16,
    string_input_event: u32,
    string_input_event_vendor: u32,
    string_amps: i16,
    string_amp_hours: u32,
}

impl ModelAdapter for Model403StatefulAdapter {
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

    /// Current scale factor for inputs
    fn in_dca_sf(&self) -> Option<u16> {
        Some(self.in_dca_sf)
    }

    /// Amp-hour scale factor for inputs
    fn in_dc_ahr_sf(&self) -> Option<u16> {
        Some(self.in_dc_ahr_sf)
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
}
