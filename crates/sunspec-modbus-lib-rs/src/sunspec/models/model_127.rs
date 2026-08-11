use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 12;

static POINTS: [PointDetails<()>; 12] = [
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
        point: |()| Point::WGra,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::HzStr,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::HzStop,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::HysEna,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::ModEna,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::HzStopWGra,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::WGraSf,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::HzStrStopSf,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::RmpIncDecSf,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::Pad,
        size: 1,
        start_address: 11,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    WGra,
    HzStr,
    HzStop,
    HysEna,
    ModEna,
    HzStopWGra,
    WGraSf,
    HzStrStopSf,
    RmpIncDecSf,
    Pad,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    12
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
            buffer::write_u16(127, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::WGra => {
            buffer::write_u16(model.w_gra(), buffer);
        }
        Point::HzStr => {
            buffer::write_i16(model.hz_str(), buffer);
        }
        Point::HzStop => {
            buffer::write_i16(model.hz_stop(), buffer);
        }
        Point::HysEna => {
            buffer::write_u16(model.hys_ena(), buffer);
        }
        Point::ModEna => {
            buffer::write_u16(model.mod_ena(), buffer);
        }
        Point::HzStopWGra => {
            if let Some(value) = model.hz_stop_w_gra() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WGraSf => {
            if let Some(value) = model.w_gra_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::HzStrStopSf => {
            if let Some(value) = model.hz_str_stop_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RmpIncDecSf => {
            if let Some(value) = model.rmp_inc_dec_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Pad => {
            buffer::write_u16(0, buffer);
        }
    }
}

pub trait ModelAdapter {
    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn w_gra(&self) -> u16;

    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn set_w_gra(&mut self, value: u16);

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn hz_str(&self) -> i16;

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn set_hz_str(&mut self, value: i16);

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn hz_stop(&self) -> i16;

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn set_hz_stop(&mut self, value: i16);

    /// HysEna
    ///
    /// Enable hysteresis
    fn hys_ena(&self) -> u16;

    /// HysEna
    ///
    /// Enable hysteresis
    fn set_hys_ena(&mut self, value: u16);

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16);

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn hz_stop_w_gra(&self) -> Option<u16> {
        None
    }

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn set_hz_stop_w_gra(&mut self, value: u16) {}

    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    fn w_gra_sf(&self) -> Option<u16> {
        None
    }

    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    fn hz_str_stop_sf(&self) -> Option<u16> {
        None
    }

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model127CallbackAdapter {
    context: *mut c_void,
    w_gra_callback: extern "C" fn(*const c_void) -> u16,
    set_w_gra_callback: extern "C" fn(u16, *mut c_void),
    hz_str_callback: extern "C" fn(*const c_void) -> i16,
    set_hz_str_callback: extern "C" fn(i16, *mut c_void),
    hz_stop_callback: extern "C" fn(*const c_void) -> i16,
    set_hz_stop_callback: extern "C" fn(i16, *mut c_void),
    hys_ena_callback: extern "C" fn(*const c_void) -> u16,
    set_hys_ena_callback: extern "C" fn(u16, *mut c_void),
    mod_ena_callback: extern "C" fn(*const c_void) -> u16,
    set_mod_ena_callback: extern "C" fn(u16, *mut c_void),
    hz_stop_w_gra_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_hz_stop_w_gra_callback: Option<extern "C" fn(u16, *mut c_void)>,
    w_gra_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    hz_str_stop_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    rmp_inc_dec_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model127CallbackAdapter {
    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn w_gra(&self) -> u16 {
        (self.w_gra_callback)(self.context)
    }

    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn set_w_gra(&mut self, value: u16) {
        (self.set_w_gra_callback)(value, self.context);
    }

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn hz_str(&self) -> i16 {
        (self.hz_str_callback)(self.context)
    }

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn set_hz_str(&mut self, value: i16) {
        (self.set_hz_str_callback)(value, self.context);
    }

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn hz_stop(&self) -> i16 {
        (self.hz_stop_callback)(self.context)
    }

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn set_hz_stop(&mut self, value: i16) {
        (self.set_hz_stop_callback)(value, self.context);
    }

    /// HysEna
    ///
    /// Enable hysteresis
    fn hys_ena(&self) -> u16 {
        (self.hys_ena_callback)(self.context)
    }

    /// HysEna
    ///
    /// Enable hysteresis
    fn set_hys_ena(&mut self, value: u16) {
        (self.set_hys_ena_callback)(value, self.context);
    }

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn hz_stop_w_gra(&self) -> Option<u16> {
        self.hz_stop_w_gra_callback
            .map(|callback| (callback)(self.context))
    }

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn set_hz_stop_w_gra(&mut self, value: u16) {
        if let Some(callback) = self.set_hz_stop_w_gra_callback {
            (callback)(value, self.context);
        };
    }

    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    fn w_gra_sf(&self) -> Option<u16> {
        self.w_gra_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    fn hz_str_stop_sf(&self) -> Option<u16> {
        self.hz_str_stop_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        self.rmp_inc_dec_sf_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model127StatefulAdapter {
    w_gra: u16,
    hz_str: i16,
    hz_stop: i16,
    hys_ena: u16,
    mod_ena: u16,
    hz_stop_w_gra: u16,
    w_gra_sf: u16,
    hz_str_stop_sf: u16,
    rmp_inc_dec_sf: u16,
}

impl ModelAdapter for Model127StatefulAdapter {
    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn w_gra(&self) -> u16 {
        self.w_gra
    }

    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn set_w_gra(&mut self, value: u16) {
        self.w_gra = value;
    }

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn hz_str(&self) -> i16 {
        self.hz_str
    }

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn set_hz_str(&mut self, value: i16) {
        self.hz_str = value;
    }

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn hz_stop(&self) -> i16 {
        self.hz_stop
    }

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn set_hz_stop(&mut self, value: i16) {
        self.hz_stop = value;
    }

    /// HysEna
    ///
    /// Enable hysteresis
    fn hys_ena(&self) -> u16 {
        self.hys_ena
    }

    /// HysEna
    ///
    /// Enable hysteresis
    fn set_hys_ena(&mut self, value: u16) {
        self.hys_ena = value;
    }

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn hz_stop_w_gra(&self) -> Option<u16> {
        Some(self.hz_stop_w_gra)
    }

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn set_hz_stop_w_gra(&mut self, value: u16) {
        self.hz_stop_w_gra = value;
    }

    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    fn w_gra_sf(&self) -> Option<u16> {
        Some(self.w_gra_sf)
    }

    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    fn hz_str_stop_sf(&self) -> Option<u16> {
        Some(self.hz_str_stop_sf)
    }

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        Some(self.rmp_inc_dec_sf)
    }
}
