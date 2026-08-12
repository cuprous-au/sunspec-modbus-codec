use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 45;

static POINTS: [PointDetails<()>; 35] = [
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
        point: |()| Point::Amps,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::ASf,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::Voltage,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::VSf,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::Hz,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::HzSf,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::Watts,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::WSf,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::Va,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::VaSf,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::Var,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::VarSf,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::Pf,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::PfSf,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursExported,
        size: 2,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursImported,
        size: 2,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::TotWhSf,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursExported,
        size: 2,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursImported,
        size: 2,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::TotVAhSf,
        size: 1,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::TotalVarHoursImportedQ1,
        size: 2,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursImportedQ2,
        size: 2,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ3,
        size: 2,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ4,
        size: 2,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::TotVArhSf,
        size: 1,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::Events,
        size: 2,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::Rsrvd,
        size: 1,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::Timestamp,
        size: 2,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::Milliseconds,
        size: 1,
        start_address: 40,
    },
    PointDetails {
        point: |()| Point::Sequence,
        size: 1,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::Algorithm,
        size: 1,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::N,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::RepeatingDs,
        size: 1,
        start_address: 44,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    Amps,
    ASf,
    Voltage,
    VSf,
    Hz,
    HzSf,
    Watts,
    WSf,
    Va,
    VaSf,
    Var,
    VarSf,
    Pf,
    PfSf,
    TotalWattHoursExported,
    TotalWattHoursImported,
    TotWhSf,
    TotalVaHoursExported,
    TotalVaHoursImported,
    TotVAhSf,
    TotalVarHoursImportedQ1,
    TotalVArHoursImportedQ2,
    TotalVArHoursExportedQ3,
    TotalVArHoursExportedQ4,
    TotVArhSf,
    Events,
    Rsrvd,
    Timestamp,
    Milliseconds,
    Sequence,
    Algorithm,
    N,
    RepeatingDs,
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    45
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
            buffer::write_u16(220, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Amps => {
            buffer::write_i16(model.amps(), buffer);
        }
        Point::ASf => {
            buffer::write_u16(model.a_sf(), buffer);
        }
        Point::Voltage => {
            if let Some(value) = model.voltage() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VSf => {
            buffer::write_u16(model.v_sf(), buffer);
        }
        Point::Hz => {
            buffer::write_i16(model.hz(), buffer);
        }
        Point::HzSf => {
            if let Some(value) = model.hz_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Watts => {
            buffer::write_i16(model.watts(), buffer);
        }
        Point::WSf => {
            buffer::write_u16(model.w_sf(), buffer);
        }
        Point::Va => {
            if let Some(value) = model.va() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VaSf => {
            if let Some(value) = model.va_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Var => {
            if let Some(value) = model.var() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::VarSf => {
            if let Some(value) = model.var_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Pf => {
            if let Some(value) = model.pf() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::PfSf => {
            if let Some(value) = model.pf_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::TotalWattHoursExported => {
            buffer::write_u32(model.total_watt_hours_exported(), buffer, offset, limit);
        }
        Point::TotalWattHoursImported => {
            buffer::write_u32(model.total_watt_hours_imported(), buffer, offset, limit);
        }
        Point::TotWhSf => {
            buffer::write_u16(model.tot_wh_sf(), buffer);
        }
        Point::TotalVaHoursExported => {
            if let Some(value) = model.total_va_hours_exported() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursImported => {
            if let Some(value) = model.total_va_hours_imported() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotVAhSf => {
            if let Some(value) = model.tot_v_ah_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::TotalVarHoursImportedQ1 => {
            if let Some(value) = model.total_var_hours_imported_q1() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursImportedQ2 => {
            if let Some(value) = model.total_v_ar_hours_imported_q2() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ3 => {
            if let Some(value) = model.total_v_ar_hours_exported_q3() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ4 => {
            if let Some(value) = model.total_v_ar_hours_exported_q4() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotVArhSf => {
            if let Some(value) = model.tot_v_arh_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Events => {
            buffer::write_u32(model.events(), buffer, offset, limit);
        }
        Point::Rsrvd => {
            buffer::write_u16(0, buffer);
        }
        Point::Timestamp => {
            buffer::write_u32(model.timestamp(), buffer, offset, limit);
        }
        Point::Milliseconds => {
            buffer::write_u16(model.milliseconds(), buffer);
        }
        Point::Sequence => {
            buffer::write_u16(model.sequence(), buffer);
        }
        Point::Algorithm => {
            buffer::write_u16(model.algorithm() as u16, buffer);
        }
        Point::N => {
            buffer::write_u16(model.n(), buffer);
        }
        Point::RepeatingDs => {
            buffer::write_u16(model.repeating_ds(), buffer);
        }
    }
}

pub trait ModelAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> i16;

    /// Current scale factor
    fn a_sf(&self) -> u16;

    /// Voltage
    ///
    /// Average phase or line voltage
    fn voltage(&self) -> Option<i16> {
        None
    }

    /// Voltage scale factor
    fn v_sf(&self) -> u16;

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> i16;

    /// Frequency scale factor
    fn hz_sf(&self) -> Option<u16> {
        None
    }

    /// Watts
    ///
    /// Total Real Power
    fn watts(&self) -> i16;

    /// Real Power scale factor
    fn w_sf(&self) -> u16;

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        None
    }

    /// Apparent Power scale factor
    fn va_sf(&self) -> Option<u16> {
        None
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<i16> {
        None
    }

    /// Reactive Power scale factor
    fn var_sf(&self) -> Option<u16> {
        None
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<i16> {
        None
    }

    /// Power Factor scale factor
    fn pf_sf(&self) -> Option<u16> {
        None
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn total_watt_hours_exported(&self) -> u32;

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> u32;

    /// Real Energy scale factor
    fn tot_wh_sf(&self) -> u16;

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<u32> {
        None
    }

    /// Apparent Energy scale factor
    fn tot_v_ah_sf(&self) -> Option<u16> {
        None
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn total_var_hours_imported_q1(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<u32> {
        None
    }

    /// Reactive Energy scale factor
    fn tot_v_arh_sf(&self) -> Option<u16> {
        None
    }

    /// Events
    ///
    /// Meter Event Flags
    fn events(&self) -> u32;

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32;

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16;

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn sequence(&self) -> u16;

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg;

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16;

    fn repeating_ds(&self) -> u16;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}

#[repr(C)]
pub struct Model220CallbackAdapter {
    context: *mut c_void,
    amps_callback: extern "C" fn(*const c_void) -> i16,
    a_sf_callback: extern "C" fn(*const c_void) -> u16,
    voltage_callback: Option<extern "C" fn(*const c_void) -> i16>,
    v_sf_callback: extern "C" fn(*const c_void) -> u16,
    hz_callback: extern "C" fn(*const c_void) -> i16,
    hz_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    watts_callback: extern "C" fn(*const c_void) -> i16,
    w_sf_callback: extern "C" fn(*const c_void) -> u16,
    va_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    var_callback: Option<extern "C" fn(*const c_void) -> i16>,
    var_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    pf_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_watt_hours_exported_callback: extern "C" fn(*const c_void) -> u32,
    total_watt_hours_imported_callback: extern "C" fn(*const c_void) -> u32,
    tot_wh_sf_callback: extern "C" fn(*const c_void) -> u16,
    total_va_hours_exported_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_va_hours_imported_callback: Option<extern "C" fn(*const c_void) -> u32>,
    tot_v_ah_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_var_hours_imported_q1_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_imported_q2_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q3_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q4_callback: Option<extern "C" fn(*const c_void) -> u32>,
    tot_v_arh_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    events_callback: extern "C" fn(*const c_void) -> u32,
    timestamp_callback: extern "C" fn(*const c_void) -> u32,
    milliseconds_callback: extern "C" fn(*const c_void) -> u16,
    sequence_callback: extern "C" fn(*const c_void) -> u16,
    algorithm_callback: extern "C" fn(*const c_void) -> Alg,
    n_callback: extern "C" fn(*const c_void) -> u16,
    repeating_ds_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model220CallbackAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> i16 {
        (self.amps_callback)(self.context)
    }

    /// Current scale factor
    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)(self.context)
    }

    /// Voltage
    ///
    /// Average phase or line voltage
    fn voltage(&self) -> Option<i16> {
        self.voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage scale factor
    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)(self.context)
    }

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> i16 {
        (self.hz_callback)(self.context)
    }

    /// Frequency scale factor
    fn hz_sf(&self) -> Option<u16> {
        self.hz_sf_callback.map(|callback| (callback)(self.context))
    }

    /// Watts
    ///
    /// Total Real Power
    fn watts(&self) -> i16 {
        (self.watts_callback)(self.context)
    }

    /// Real Power scale factor
    fn w_sf(&self) -> u16 {
        (self.w_sf_callback)(self.context)
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        self.va_callback.map(|callback| (callback)(self.context))
    }

    /// Apparent Power scale factor
    fn va_sf(&self) -> Option<u16> {
        self.va_sf_callback.map(|callback| (callback)(self.context))
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<i16> {
        self.var_callback.map(|callback| (callback)(self.context))
    }

    /// Reactive Power scale factor
    fn var_sf(&self) -> Option<u16> {
        self.var_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<i16> {
        self.pf_callback.map(|callback| (callback)(self.context))
    }

    /// Power Factor scale factor
    fn pf_sf(&self) -> Option<u16> {
        self.pf_sf_callback.map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn total_watt_hours_exported(&self) -> u32 {
        (self.total_watt_hours_exported_callback)(self.context)
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> u32 {
        (self.total_watt_hours_imported_callback)(self.context)
    }

    /// Real Energy scale factor
    fn tot_wh_sf(&self) -> u16 {
        (self.tot_wh_sf_callback)(self.context)
    }

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<u32> {
        self.total_va_hours_exported_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<u32> {
        self.total_va_hours_imported_callback
            .map(|callback| (callback)(self.context))
    }

    /// Apparent Energy scale factor
    fn tot_v_ah_sf(&self) -> Option<u16> {
        self.tot_v_ah_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn total_var_hours_imported_q1(&self) -> Option<u32> {
        self.total_var_hours_imported_q1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reactive Energy scale factor
    fn tot_v_arh_sf(&self) -> Option<u16> {
        self.tot_v_arh_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Events
    ///
    /// Meter Event Flags
    fn events(&self) -> u32 {
        (self.events_callback)(self.context)
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        (self.timestamp_callback)(self.context)
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        (self.milliseconds_callback)(self.context)
    }

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn sequence(&self) -> u16 {
        (self.sequence_callback)(self.context)
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg {
        (self.algorithm_callback)(self.context)
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        (self.n_callback)(self.context)
    }

    fn repeating_ds(&self) -> u16 {
        (self.repeating_ds_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model220StatefulAdapter {
    amps: i16,
    a_sf: u16,
    voltage: i16,
    v_sf: u16,
    hz: i16,
    hz_sf: u16,
    watts: i16,
    w_sf: u16,
    va: i16,
    va_sf: u16,
    var: i16,
    var_sf: u16,
    pf: i16,
    pf_sf: u16,
    total_watt_hours_exported: u32,
    total_watt_hours_imported: u32,
    tot_wh_sf: u16,
    total_va_hours_exported: u32,
    total_va_hours_imported: u32,
    tot_v_ah_sf: u16,
    total_var_hours_imported_q1: u32,
    total_v_ar_hours_imported_q2: u32,
    total_v_ar_hours_exported_q3: u32,
    total_v_ar_hours_exported_q4: u32,
    tot_v_arh_sf: u16,
    events: u32,
    timestamp: u32,
    milliseconds: u16,
    sequence: u16,
    algorithm: Alg,
    n: u16,
    repeating_ds: u16,
}

impl ModelAdapter for Model220StatefulAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> i16 {
        self.amps
    }

    /// Current scale factor
    fn a_sf(&self) -> u16 {
        self.a_sf
    }

    /// Voltage
    ///
    /// Average phase or line voltage
    fn voltage(&self) -> Option<i16> {
        Some(self.voltage)
    }

    /// Voltage scale factor
    fn v_sf(&self) -> u16 {
        self.v_sf
    }

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> i16 {
        self.hz
    }

    /// Frequency scale factor
    fn hz_sf(&self) -> Option<u16> {
        Some(self.hz_sf)
    }

    /// Watts
    ///
    /// Total Real Power
    fn watts(&self) -> i16 {
        self.watts
    }

    /// Real Power scale factor
    fn w_sf(&self) -> u16 {
        self.w_sf
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        Some(self.va)
    }

    /// Apparent Power scale factor
    fn va_sf(&self) -> Option<u16> {
        Some(self.va_sf)
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<i16> {
        Some(self.var)
    }

    /// Reactive Power scale factor
    fn var_sf(&self) -> Option<u16> {
        Some(self.var_sf)
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<i16> {
        Some(self.pf)
    }

    /// Power Factor scale factor
    fn pf_sf(&self) -> Option<u16> {
        Some(self.pf_sf)
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn total_watt_hours_exported(&self) -> u32 {
        self.total_watt_hours_exported
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> u32 {
        self.total_watt_hours_imported
    }

    /// Real Energy scale factor
    fn tot_wh_sf(&self) -> u16 {
        self.tot_wh_sf
    }

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<u32> {
        Some(self.total_va_hours_exported)
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<u32> {
        Some(self.total_va_hours_imported)
    }

    /// Apparent Energy scale factor
    fn tot_v_ah_sf(&self) -> Option<u16> {
        Some(self.tot_v_ah_sf)
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn total_var_hours_imported_q1(&self) -> Option<u32> {
        Some(self.total_var_hours_imported_q1)
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_imported_q2)
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q3)
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q4)
    }

    /// Reactive Energy scale factor
    fn tot_v_arh_sf(&self) -> Option<u16> {
        Some(self.tot_v_arh_sf)
    }

    /// Events
    ///
    /// Meter Event Flags
    fn events(&self) -> u32 {
        self.events
    }

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn milliseconds(&self) -> u16 {
        self.milliseconds
    }

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn sequence(&self) -> u16 {
        self.sequence
    }

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn algorithm(&self) -> Alg {
        self.algorithm
    }

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16 {
        self.n
    }

    fn repeating_ds(&self) -> u16 {
        self.repeating_ds
    }
}
