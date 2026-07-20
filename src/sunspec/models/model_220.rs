use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 44;

pub static POINTS: [ReadablePoint; 34] = [
    ReadablePoint {
        reference: PointReference::Static { value: 220 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 42 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Amps },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Voltage },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Hz },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::HzSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Watts },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::WSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Va },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::VaSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Var },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::VarSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Pf },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::PfSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotalWattHoursExported },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotalWattHoursImported },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotWhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotalVaHoursExported },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotalVaHoursImported },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotVAhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotalVarHoursImportedQ1 },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotalVArHoursImportedQ2 },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotalVArHoursExportedQ3 },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotalVArHoursExportedQ4 },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::TotVArhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Events },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Timestamp },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Milliseconds },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Sequence },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::Algorithm },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model220 { point: Point::N },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
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
    Timestamp,
    Milliseconds,
    Sequence,
    Algorithm,
    N,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Amps => serialisation::write_i16(model.amps(), buffer),
        Point::ASf => serialisation::write_u16(model.a_sf(), buffer),
        Point::Voltage => if let Some(value) = model.voltage() { serialisation::write_i16(value, buffer); },
        Point::VSf => serialisation::write_u16(model.v_sf(), buffer),
        Point::Hz => serialisation::write_i16(model.hz(), buffer),
        Point::HzSf => if let Some(value) = model.hz_sf() { serialisation::write_u16(value, buffer); },
        Point::Watts => serialisation::write_i16(model.watts(), buffer),
        Point::WSf => serialisation::write_u16(model.w_sf(), buffer),
        Point::Va => if let Some(value) = model.va() { serialisation::write_i16(value, buffer); },
        Point::VaSf => if let Some(value) = model.va_sf() { serialisation::write_u16(value, buffer); },
        Point::Var => if let Some(value) = model.var() { serialisation::write_i16(value, buffer); },
        Point::VarSf => if let Some(value) = model.var_sf() { serialisation::write_u16(value, buffer); },
        Point::Pf => if let Some(value) = model.pf() { serialisation::write_i16(value, buffer); },
        Point::PfSf => if let Some(value) = model.pf_sf() { serialisation::write_u16(value, buffer); },
        Point::TotalWattHoursExported => serialisation::write_u32(model.total_watt_hours_exported(), buffer, offset, limit),
        Point::TotalWattHoursImported => serialisation::write_u32(model.total_watt_hours_imported(), buffer, offset, limit),
        Point::TotWhSf => serialisation::write_u16(model.tot_wh_sf(), buffer),
        Point::TotalVaHoursExported => if let Some(value) = model.total_va_hours_exported() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::TotalVaHoursImported => if let Some(value) = model.total_va_hours_imported() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::TotVAhSf => if let Some(value) = model.tot_v_ah_sf() { serialisation::write_u16(value, buffer); },
        Point::TotalVarHoursImportedQ1 => if let Some(value) = model.total_var_hours_imported_q1() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::TotalVArHoursImportedQ2 => if let Some(value) = model.total_v_ar_hours_imported_q2() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::TotalVArHoursExportedQ3 => if let Some(value) = model.total_v_ar_hours_exported_q3() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::TotalVArHoursExportedQ4 => if let Some(value) = model.total_v_ar_hours_exported_q4() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::TotVArhSf => if let Some(value) = model.tot_v_arh_sf() { serialisation::write_u16(value, buffer); },
        Point::Events => serialisation::write_u32(model.events(), buffer, offset, limit),
        Point::Timestamp => serialisation::write_u32(model.timestamp(), buffer, offset, limit),
        Point::Milliseconds => serialisation::write_u16(model.milliseconds(), buffer),
        Point::Sequence => serialisation::write_u16(model.sequence(), buffer),
        Point::Algorithm => serialisation::write_u16(model.algorithm() as u16, buffer),
        Point::N => serialisation::write_u16(model.n(), buffer),
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
}

pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}