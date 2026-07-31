use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 107;

pub static POINTS: [ReadablePoint; 74] = [
    ReadablePoint {
        reference: PointReference::Static { value: 204 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 105 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::Amps },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::AmpsPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::AmpsPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::AmpsPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VoltageLn,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PhaseVoltageAn,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PhaseVoltageBn,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PhaseVoltageCn,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VoltageLl,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PhaseVoltageAb,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PhaseVoltageBc,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PhaseVoltageCa,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::Hz },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::HzSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::Watts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::WattsPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::WattsPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::WattsPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::WSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::Va },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VaPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VaPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VaPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::VaSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::Var },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VarPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VarPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VarPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::VarSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::Pf },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PfPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PfPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::PfPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 { point: Point::PfSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalWattHoursExported,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalWattHoursExportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalWattHoursExportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalWattHoursExportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalWattHoursImported,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalWattHoursImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalWattHoursImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalWattHoursImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotWhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVaHoursExported,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVaHoursExportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVaHoursExportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVaHoursExportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVaHoursImported,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVaHoursImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVaHoursImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVaHoursImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotVAhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVarHoursImportedQ1,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursImportedQ1PhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursImportedQ1PhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursImportedQ1PhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursImportedQ2,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursImportedQ2PhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursImportedQ2PhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursImportedQ2PhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursExportedQ3,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursExportedQ3PhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursExportedQ3PhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursExportedQ3PhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursExportedQ4,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::TotVArhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model204 {
            point: Point::Events,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    Amps,
    AmpsPhaseA,
    AmpsPhaseB,
    AmpsPhaseC,
    ASf,
    VoltageLn,
    PhaseVoltageAn,
    PhaseVoltageBn,
    PhaseVoltageCn,
    VoltageLl,
    PhaseVoltageAb,
    PhaseVoltageBc,
    PhaseVoltageCa,
    VSf,
    Hz,
    HzSf,
    Watts,
    WattsPhaseA,
    WattsPhaseB,
    WattsPhaseC,
    WSf,
    Va,
    VaPhaseA,
    VaPhaseB,
    VaPhaseC,
    VaSf,
    Var,
    VarPhaseA,
    VarPhaseB,
    VarPhaseC,
    VarSf,
    Pf,
    PfPhaseA,
    PfPhaseB,
    PfPhaseC,
    PfSf,
    TotalWattHoursExported,
    TotalWattHoursExportedPhaseA,
    TotalWattHoursExportedPhaseB,
    TotalWattHoursExportedPhaseC,
    TotalWattHoursImported,
    TotalWattHoursImportedPhaseA,
    TotalWattHoursImportedPhaseB,
    TotalWattHoursImportedPhaseC,
    TotWhSf,
    TotalVaHoursExported,
    TotalVaHoursExportedPhaseA,
    TotalVaHoursExportedPhaseB,
    TotalVaHoursExportedPhaseC,
    TotalVaHoursImported,
    TotalVaHoursImportedPhaseA,
    TotalVaHoursImportedPhaseB,
    TotalVaHoursImportedPhaseC,
    TotVAhSf,
    TotalVarHoursImportedQ1,
    TotalVArHoursImportedQ1PhaseA,
    TotalVArHoursImportedQ1PhaseB,
    TotalVArHoursImportedQ1PhaseC,
    TotalVArHoursImportedQ2,
    TotalVArHoursImportedQ2PhaseA,
    TotalVArHoursImportedQ2PhaseB,
    TotalVArHoursImportedQ2PhaseC,
    TotalVArHoursExportedQ3,
    TotalVArHoursExportedQ3PhaseA,
    TotalVArHoursExportedQ3PhaseB,
    TotalVArHoursExportedQ3PhaseC,
    TotalVArHoursExportedQ4,
    TotalVArHoursExportedQ4ImportedPhaseA,
    TotalVArHoursExportedQ4ImportedPhaseB,
    TotalVArHoursExportedQ4ImportedPhaseC,
    TotVArhSf,
    Events,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Amps => serialisation::write_i16(model.amps(), buffer),
        Point::AmpsPhaseA => serialisation::write_i16(model.amps_phase_a(), buffer),
        Point::AmpsPhaseB => serialisation::write_i16(model.amps_phase_b(), buffer),
        Point::AmpsPhaseC => serialisation::write_i16(model.amps_phase_c(), buffer),
        Point::ASf => serialisation::write_u16(model.a_sf(), buffer),
        Point::VoltageLn => {
            if let Some(value) = model.voltage_ln() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PhaseVoltageAn => {
            if let Some(value) = model.phase_voltage_an() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PhaseVoltageBn => {
            if let Some(value) = model.phase_voltage_bn() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PhaseVoltageCn => {
            if let Some(value) = model.phase_voltage_cn() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VoltageLl => serialisation::write_i16(model.voltage_ll(), buffer),
        Point::PhaseVoltageAb => serialisation::write_i16(model.phase_voltage_ab(), buffer),
        Point::PhaseVoltageBc => serialisation::write_i16(model.phase_voltage_bc(), buffer),
        Point::PhaseVoltageCa => serialisation::write_i16(model.phase_voltage_ca(), buffer),
        Point::VSf => serialisation::write_u16(model.v_sf(), buffer),
        Point::Hz => serialisation::write_i16(model.hz(), buffer),
        Point::HzSf => {
            if let Some(value) = model.hz_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Watts => serialisation::write_i16(model.watts(), buffer),
        Point::WattsPhaseA => {
            if let Some(value) = model.watts_phase_a() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WattsPhaseB => {
            if let Some(value) = model.watts_phase_b() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WattsPhaseC => {
            if let Some(value) = model.watts_phase_c() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WSf => serialisation::write_u16(model.w_sf(), buffer),
        Point::Va => {
            if let Some(value) = model.va() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VaPhaseA => {
            if let Some(value) = model.va_phase_a() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VaPhaseB => {
            if let Some(value) = model.va_phase_b() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VaPhaseC => {
            if let Some(value) = model.va_phase_c() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VaSf => {
            if let Some(value) = model.va_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Var => {
            if let Some(value) = model.var() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VarPhaseA => {
            if let Some(value) = model.var_phase_a() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VarPhaseB => {
            if let Some(value) = model.var_phase_b() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VarPhaseC => {
            if let Some(value) = model.var_phase_c() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VarSf => {
            if let Some(value) = model.var_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Pf => {
            if let Some(value) = model.pf() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfPhaseA => {
            if let Some(value) = model.pf_phase_a() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfPhaseB => {
            if let Some(value) = model.pf_phase_b() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfPhaseC => {
            if let Some(value) = model.pf_phase_c() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfSf => {
            if let Some(value) = model.pf_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TotalWattHoursExported => {
            serialisation::write_u32(model.total_watt_hours_exported(), buffer, offset, limit)
        }
        Point::TotalWattHoursExportedPhaseA => {
            if let Some(value) = model.total_watt_hours_exported_phase_a() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursExportedPhaseB => {
            if let Some(value) = model.total_watt_hours_exported_phase_b() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursExportedPhaseC => {
            if let Some(value) = model.total_watt_hours_exported_phase_c() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursImported => {
            serialisation::write_u32(model.total_watt_hours_imported(), buffer, offset, limit)
        }
        Point::TotalWattHoursImportedPhaseA => {
            if let Some(value) = model.total_watt_hours_imported_phase_a() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursImportedPhaseB => {
            if let Some(value) = model.total_watt_hours_imported_phase_b() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursImportedPhaseC => {
            if let Some(value) = model.total_watt_hours_imported_phase_c() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotWhSf => serialisation::write_u16(model.tot_wh_sf(), buffer),
        Point::TotalVaHoursExported => {
            if let Some(value) = model.total_va_hours_exported() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursExportedPhaseA => {
            if let Some(value) = model.total_va_hours_exported_phase_a() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursExportedPhaseB => {
            if let Some(value) = model.total_va_hours_exported_phase_b() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursExportedPhaseC => {
            if let Some(value) = model.total_va_hours_exported_phase_c() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursImported => {
            if let Some(value) = model.total_va_hours_imported() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursImportedPhaseA => {
            if let Some(value) = model.total_va_hours_imported_phase_a() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursImportedPhaseB => {
            if let Some(value) = model.total_va_hours_imported_phase_b() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursImportedPhaseC => {
            if let Some(value) = model.total_va_hours_imported_phase_c() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotVAhSf => {
            if let Some(value) = model.tot_v_ah_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TotalVarHoursImportedQ1 => {
            if let Some(value) = model.total_var_hours_imported_q1() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseA => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_a() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseB => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_b() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseC => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_c() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ2 => {
            if let Some(value) = model.total_v_ar_hours_imported_q2() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseA => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_a() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseB => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_b() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseC => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_c() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ3 => {
            if let Some(value) = model.total_v_ar_hours_exported_q3() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseA => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_a() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseB => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_b() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseC => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_c() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ4 => {
            if let Some(value) = model.total_v_ar_hours_exported_q4() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseA => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_a() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseB => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_b() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseC => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_c() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotVArhSf => {
            if let Some(value) = model.tot_v_arh_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Events => serialisation::write_u32(model.events(), buffer, offset, limit),
    }
}

pub trait ModelAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> i16;

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> i16;

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> i16;

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> i16;

    /// Current scale factor
    fn a_sf(&self) -> u16;

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    fn voltage_ln(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<i16> {
        None
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn voltage_ll(&self) -> i16;

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> i16;

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> i16;

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> i16;

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

    /// Watts phase A
    fn watts_phase_a(&self) -> Option<i16> {
        None
    }

    /// Watts phase B
    fn watts_phase_b(&self) -> Option<i16> {
        None
    }

    /// Watts phase C
    fn watts_phase_c(&self) -> Option<i16> {
        None
    }

    /// Real Power scale factor
    fn w_sf(&self) -> u16;

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        None
    }

    /// VA phase A
    fn va_phase_a(&self) -> Option<i16> {
        None
    }

    /// VA phase B
    fn va_phase_b(&self) -> Option<i16> {
        None
    }

    /// VA phase C
    fn va_phase_c(&self) -> Option<i16> {
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

    /// VAR phase A
    fn var_phase_a(&self) -> Option<i16> {
        None
    }

    /// VAR phase B
    fn var_phase_b(&self) -> Option<i16> {
        None
    }

    /// VAR phase C
    fn var_phase_c(&self) -> Option<i16> {
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

    /// PF phase A
    fn pf_phase_a(&self) -> Option<i16> {
        None
    }

    /// PF phase B
    fn pf_phase_b(&self) -> Option<i16> {
        None
    }

    /// PF phase C
    fn pf_phase_c(&self) -> Option<i16> {
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

    /// Total Watt-hours Exported phase A
    fn total_watt_hours_exported_phase_a(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Exported phase B
    fn total_watt_hours_exported_phase_b(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Exported phase C
    fn total_watt_hours_exported_phase_c(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> u32;

    /// Total Watt-hours Imported phase A
    fn total_watt_hours_imported_phase_a(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Imported phase B
    fn total_watt_hours_imported_phase_b(&self) -> Option<u32> {
        None
    }

    /// Total Watt-hours Imported phase C
    fn total_watt_hours_imported_phase_c(&self) -> Option<u32> {
        None
    }

    /// Real Energy scale factor
    fn tot_wh_sf(&self) -> u16;

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Exported phase A
    fn total_va_hours_exported_phase_a(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Exported phase B
    fn total_va_hours_exported_phase_b(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Exported phase C
    fn total_va_hours_exported_phase_c(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported phase A
    fn total_va_hours_imported_phase_a(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported phase B
    fn total_va_hours_imported_phase_b(&self) -> Option<u32> {
        None
    }

    /// Total VA-hours Imported phase C
    fn total_va_hours_imported_phase_c(&self) -> Option<u32> {
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

    /// Total VAr-hours Imported Q1 phase A
    fn total_v_ar_hours_imported_q1_phase_a(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase B
    fn total_v_ar_hours_imported_q1_phase_b(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase C
    fn total_v_ar_hours_imported_q1_phase_c(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase A
    fn total_v_ar_hours_imported_q2_phase_a(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase B
    fn total_v_ar_hours_imported_q2_phase_b(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase C
    fn total_v_ar_hours_imported_q2_phase_c(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase A
    fn total_v_ar_hours_exported_q3_phase_a(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase B
    fn total_v_ar_hours_exported_q3_phase_b(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase C
    fn total_v_ar_hours_exported_q3_phase_c(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn total_v_ar_hours_exported_q4_imported_phase_a(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn total_v_ar_hours_exported_q4_imported_phase_b(&self) -> Option<u32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn total_v_ar_hours_exported_q4_imported_phase_c(&self) -> Option<u32> {
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
}

#[repr(C)]
pub struct Model204CallbackAdapter {
    amps_callback: extern "C" fn() -> i16,
    amps_phase_a_callback: extern "C" fn() -> i16,
    amps_phase_b_callback: extern "C" fn() -> i16,
    amps_phase_c_callback: extern "C" fn() -> i16,
    a_sf_callback: extern "C" fn() -> u16,
    voltage_ln_callback: Option<extern "C" fn() -> i16>,
    phase_voltage_an_callback: Option<extern "C" fn() -> i16>,
    phase_voltage_bn_callback: Option<extern "C" fn() -> i16>,
    phase_voltage_cn_callback: Option<extern "C" fn() -> i16>,
    voltage_ll_callback: extern "C" fn() -> i16,
    phase_voltage_ab_callback: extern "C" fn() -> i16,
    phase_voltage_bc_callback: extern "C" fn() -> i16,
    phase_voltage_ca_callback: extern "C" fn() -> i16,
    v_sf_callback: extern "C" fn() -> u16,
    hz_callback: extern "C" fn() -> i16,
    hz_sf_callback: Option<extern "C" fn() -> u16>,
    watts_callback: extern "C" fn() -> i16,
    watts_phase_a_callback: Option<extern "C" fn() -> i16>,
    watts_phase_b_callback: Option<extern "C" fn() -> i16>,
    watts_phase_c_callback: Option<extern "C" fn() -> i16>,
    w_sf_callback: extern "C" fn() -> u16,
    va_callback: Option<extern "C" fn() -> i16>,
    va_phase_a_callback: Option<extern "C" fn() -> i16>,
    va_phase_b_callback: Option<extern "C" fn() -> i16>,
    va_phase_c_callback: Option<extern "C" fn() -> i16>,
    va_sf_callback: Option<extern "C" fn() -> u16>,
    var_callback: Option<extern "C" fn() -> i16>,
    var_phase_a_callback: Option<extern "C" fn() -> i16>,
    var_phase_b_callback: Option<extern "C" fn() -> i16>,
    var_phase_c_callback: Option<extern "C" fn() -> i16>,
    var_sf_callback: Option<extern "C" fn() -> u16>,
    pf_callback: Option<extern "C" fn() -> i16>,
    pf_phase_a_callback: Option<extern "C" fn() -> i16>,
    pf_phase_b_callback: Option<extern "C" fn() -> i16>,
    pf_phase_c_callback: Option<extern "C" fn() -> i16>,
    pf_sf_callback: Option<extern "C" fn() -> u16>,
    total_watt_hours_exported_callback: extern "C" fn() -> u32,
    total_watt_hours_exported_phase_a_callback: Option<extern "C" fn() -> u32>,
    total_watt_hours_exported_phase_b_callback: Option<extern "C" fn() -> u32>,
    total_watt_hours_exported_phase_c_callback: Option<extern "C" fn() -> u32>,
    total_watt_hours_imported_callback: extern "C" fn() -> u32,
    total_watt_hours_imported_phase_a_callback: Option<extern "C" fn() -> u32>,
    total_watt_hours_imported_phase_b_callback: Option<extern "C" fn() -> u32>,
    total_watt_hours_imported_phase_c_callback: Option<extern "C" fn() -> u32>,
    tot_wh_sf_callback: extern "C" fn() -> u16,
    total_va_hours_exported_callback: Option<extern "C" fn() -> u32>,
    total_va_hours_exported_phase_a_callback: Option<extern "C" fn() -> u32>,
    total_va_hours_exported_phase_b_callback: Option<extern "C" fn() -> u32>,
    total_va_hours_exported_phase_c_callback: Option<extern "C" fn() -> u32>,
    total_va_hours_imported_callback: Option<extern "C" fn() -> u32>,
    total_va_hours_imported_phase_a_callback: Option<extern "C" fn() -> u32>,
    total_va_hours_imported_phase_b_callback: Option<extern "C" fn() -> u32>,
    total_va_hours_imported_phase_c_callback: Option<extern "C" fn() -> u32>,
    tot_v_ah_sf_callback: Option<extern "C" fn() -> u16>,
    total_var_hours_imported_q1_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_imported_q1_phase_a_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_imported_q1_phase_b_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_imported_q1_phase_c_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_imported_q2_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_imported_q2_phase_a_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_imported_q2_phase_b_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_imported_q2_phase_c_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_exported_q3_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_exported_q3_phase_a_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_exported_q3_phase_b_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_exported_q3_phase_c_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_exported_q4_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_exported_q4_imported_phase_a_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_exported_q4_imported_phase_b_callback: Option<extern "C" fn() -> u32>,
    total_v_ar_hours_exported_q4_imported_phase_c_callback: Option<extern "C" fn() -> u32>,
    tot_v_arh_sf_callback: Option<extern "C" fn() -> u16>,
    events_callback: extern "C" fn() -> u32,
}

impl ModelAdapter for Model204CallbackAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> i16 {
        (self.amps_callback)()
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> i16 {
        (self.amps_phase_a_callback)()
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> i16 {
        (self.amps_phase_b_callback)()
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> i16 {
        (self.amps_phase_c_callback)()
    }

    /// Current scale factor
    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)()
    }

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    fn voltage_ln(&self) -> Option<i16> {
        self.voltage_ln_callback.map(|callback| (callback)())
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> Option<i16> {
        self.phase_voltage_an_callback.map(|callback| (callback)())
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<i16> {
        self.phase_voltage_bn_callback.map(|callback| (callback)())
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<i16> {
        self.phase_voltage_cn_callback.map(|callback| (callback)())
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn voltage_ll(&self) -> i16 {
        (self.voltage_ll_callback)()
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> i16 {
        (self.phase_voltage_ab_callback)()
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> i16 {
        (self.phase_voltage_bc_callback)()
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> i16 {
        (self.phase_voltage_ca_callback)()
    }

    /// Voltage scale factor
    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)()
    }

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> i16 {
        (self.hz_callback)()
    }

    /// Frequency scale factor
    fn hz_sf(&self) -> Option<u16> {
        self.hz_sf_callback.map(|callback| (callback)())
    }

    /// Watts
    ///
    /// Total Real Power
    fn watts(&self) -> i16 {
        (self.watts_callback)()
    }

    /// Watts phase A
    fn watts_phase_a(&self) -> Option<i16> {
        self.watts_phase_a_callback.map(|callback| (callback)())
    }

    /// Watts phase B
    fn watts_phase_b(&self) -> Option<i16> {
        self.watts_phase_b_callback.map(|callback| (callback)())
    }

    /// Watts phase C
    fn watts_phase_c(&self) -> Option<i16> {
        self.watts_phase_c_callback.map(|callback| (callback)())
    }

    /// Real Power scale factor
    fn w_sf(&self) -> u16 {
        (self.w_sf_callback)()
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        self.va_callback.map(|callback| (callback)())
    }

    /// VA phase A
    fn va_phase_a(&self) -> Option<i16> {
        self.va_phase_a_callback.map(|callback| (callback)())
    }

    /// VA phase B
    fn va_phase_b(&self) -> Option<i16> {
        self.va_phase_b_callback.map(|callback| (callback)())
    }

    /// VA phase C
    fn va_phase_c(&self) -> Option<i16> {
        self.va_phase_c_callback.map(|callback| (callback)())
    }

    /// Apparent Power scale factor
    fn va_sf(&self) -> Option<u16> {
        self.va_sf_callback.map(|callback| (callback)())
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<i16> {
        self.var_callback.map(|callback| (callback)())
    }

    /// VAR phase A
    fn var_phase_a(&self) -> Option<i16> {
        self.var_phase_a_callback.map(|callback| (callback)())
    }

    /// VAR phase B
    fn var_phase_b(&self) -> Option<i16> {
        self.var_phase_b_callback.map(|callback| (callback)())
    }

    /// VAR phase C
    fn var_phase_c(&self) -> Option<i16> {
        self.var_phase_c_callback.map(|callback| (callback)())
    }

    /// Reactive Power scale factor
    fn var_sf(&self) -> Option<u16> {
        self.var_sf_callback.map(|callback| (callback)())
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<i16> {
        self.pf_callback.map(|callback| (callback)())
    }

    /// PF phase A
    fn pf_phase_a(&self) -> Option<i16> {
        self.pf_phase_a_callback.map(|callback| (callback)())
    }

    /// PF phase B
    fn pf_phase_b(&self) -> Option<i16> {
        self.pf_phase_b_callback.map(|callback| (callback)())
    }

    /// PF phase C
    fn pf_phase_c(&self) -> Option<i16> {
        self.pf_phase_c_callback.map(|callback| (callback)())
    }

    /// Power Factor scale factor
    fn pf_sf(&self) -> Option<u16> {
        self.pf_sf_callback.map(|callback| (callback)())
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn total_watt_hours_exported(&self) -> u32 {
        (self.total_watt_hours_exported_callback)()
    }

    /// Total Watt-hours Exported phase A
    fn total_watt_hours_exported_phase_a(&self) -> Option<u32> {
        self.total_watt_hours_exported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Exported phase B
    fn total_watt_hours_exported_phase_b(&self) -> Option<u32> {
        self.total_watt_hours_exported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Exported phase C
    fn total_watt_hours_exported_phase_c(&self) -> Option<u32> {
        self.total_watt_hours_exported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> u32 {
        (self.total_watt_hours_imported_callback)()
    }

    /// Total Watt-hours Imported phase A
    fn total_watt_hours_imported_phase_a(&self) -> Option<u32> {
        self.total_watt_hours_imported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Imported phase B
    fn total_watt_hours_imported_phase_b(&self) -> Option<u32> {
        self.total_watt_hours_imported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Imported phase C
    fn total_watt_hours_imported_phase_c(&self) -> Option<u32> {
        self.total_watt_hours_imported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Real Energy scale factor
    fn tot_wh_sf(&self) -> u16 {
        (self.tot_wh_sf_callback)()
    }

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<u32> {
        self.total_va_hours_exported_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Exported phase A
    fn total_va_hours_exported_phase_a(&self) -> Option<u32> {
        self.total_va_hours_exported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Exported phase B
    fn total_va_hours_exported_phase_b(&self) -> Option<u32> {
        self.total_va_hours_exported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Exported phase C
    fn total_va_hours_exported_phase_c(&self) -> Option<u32> {
        self.total_va_hours_exported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<u32> {
        self.total_va_hours_imported_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Imported phase A
    fn total_va_hours_imported_phase_a(&self) -> Option<u32> {
        self.total_va_hours_imported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Imported phase B
    fn total_va_hours_imported_phase_b(&self) -> Option<u32> {
        self.total_va_hours_imported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Imported phase C
    fn total_va_hours_imported_phase_c(&self) -> Option<u32> {
        self.total_va_hours_imported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Apparent Energy scale factor
    fn tot_v_ah_sf(&self) -> Option<u16> {
        self.tot_v_ah_sf_callback.map(|callback| (callback)())
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn total_var_hours_imported_q1(&self) -> Option<u32> {
        self.total_var_hours_imported_q1_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q1 phase A
    fn total_v_ar_hours_imported_q1_phase_a(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q1_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q1 phase B
    fn total_v_ar_hours_imported_q1_phase_b(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q1_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q1 phase C
    fn total_v_ar_hours_imported_q1_phase_c(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q1_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q2 phase A
    fn total_v_ar_hours_imported_q2_phase_a(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q2 phase B
    fn total_v_ar_hours_imported_q2_phase_b(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q2 phase C
    fn total_v_ar_hours_imported_q2_phase_c(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q3 phase A
    fn total_v_ar_hours_exported_q3_phase_a(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q3 phase B
    fn total_v_ar_hours_exported_q3_phase_b(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q3 phase C
    fn total_v_ar_hours_exported_q3_phase_c(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn total_v_ar_hours_exported_q4_imported_phase_a(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_imported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn total_v_ar_hours_exported_q4_imported_phase_b(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_imported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn total_v_ar_hours_exported_q4_imported_phase_c(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_imported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Reactive Energy scale factor
    fn tot_v_arh_sf(&self) -> Option<u16> {
        self.tot_v_arh_sf_callback.map(|callback| (callback)())
    }

    /// Events
    ///
    /// Meter Event Flags
    fn events(&self) -> u32 {
        (self.events_callback)()
    }
}
