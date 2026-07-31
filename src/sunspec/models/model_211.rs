use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 126;

pub static POINTS: [ReadablePoint; 64] = [
    ReadablePoint {
        reference: PointReference::Static { value: 211 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 124 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 { point: Point::Amps },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::AmpsPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::AmpsPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::AmpsPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::VoltageLn,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PhaseVoltageAn,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PhaseVoltageBn,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PhaseVoltageCn,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::VoltageLl,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PhaseVoltageAb,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PhaseVoltageBc,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PhaseVoltageCa,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 { point: Point::Hz },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::Watts,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::WattsPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::WattsPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::WattsPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 { point: Point::Va },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::VaPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::VaPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::VaPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 { point: Point::Var },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::VarPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::VarPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::VarPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 { point: Point::Pf },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PfPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PfPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::PfPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalWattHoursExported,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalWattHoursExportedPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalWattHoursExportedPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalWattHoursExportedPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalWattHoursImported,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalWattHoursImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalWattHoursImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalWattHoursImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVaHoursExported,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVaHoursExportedPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVaHoursExportedPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVaHoursExportedPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVaHoursImported,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVaHoursImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVaHoursImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVaHoursImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVarHoursImportedQ1,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursImportedQ1PhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursImportedQ1PhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursImportedQ1PhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursImportedQ2,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursImportedQ2PhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursImportedQ2PhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursImportedQ2PhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursExportedQ3,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursExportedQ3PhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursExportedQ3PhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursExportedQ3PhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursExportedQ4,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model211 {
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
    VoltageLn,
    PhaseVoltageAn,
    PhaseVoltageBn,
    PhaseVoltageCn,
    VoltageLl,
    PhaseVoltageAb,
    PhaseVoltageBc,
    PhaseVoltageCa,
    Hz,
    Watts,
    WattsPhaseA,
    WattsPhaseB,
    WattsPhaseC,
    Va,
    VaPhaseA,
    VaPhaseB,
    VaPhaseC,
    Var,
    VarPhaseA,
    VarPhaseB,
    VarPhaseC,
    Pf,
    PfPhaseA,
    PfPhaseB,
    PfPhaseC,
    TotalWattHoursExported,
    TotalWattHoursExportedPhaseA,
    TotalWattHoursExportedPhaseB,
    TotalWattHoursExportedPhaseC,
    TotalWattHoursImported,
    TotalWattHoursImportedPhaseA,
    TotalWattHoursImportedPhaseB,
    TotalWattHoursImportedPhaseC,
    TotalVaHoursExported,
    TotalVaHoursExportedPhaseA,
    TotalVaHoursExportedPhaseB,
    TotalVaHoursExportedPhaseC,
    TotalVaHoursImported,
    TotalVaHoursImportedPhaseA,
    TotalVaHoursImportedPhaseB,
    TotalVaHoursImportedPhaseC,
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
        Point::Amps => serialisation::write_f32(model.amps(), buffer, offset, limit),
        Point::AmpsPhaseA => serialisation::write_f32(model.amps_phase_a(), buffer, offset, limit),
        Point::AmpsPhaseB => {
            if let Some(value) = model.amps_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::AmpsPhaseC => {
            if let Some(value) = model.amps_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::VoltageLn => {
            if let Some(value) = model.voltage_ln() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PhaseVoltageAn => {
            if let Some(value) = model.phase_voltage_an() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PhaseVoltageBn => {
            if let Some(value) = model.phase_voltage_bn() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PhaseVoltageCn => {
            if let Some(value) = model.phase_voltage_cn() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::VoltageLl => {
            if let Some(value) = model.voltage_ll() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PhaseVoltageAb => {
            if let Some(value) = model.phase_voltage_ab() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PhaseVoltageBc => {
            if let Some(value) = model.phase_voltage_bc() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PhaseVoltageCa => {
            if let Some(value) = model.phase_voltage_ca() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::Hz => serialisation::write_f32(model.hz(), buffer, offset, limit),
        Point::Watts => serialisation::write_f32(model.watts(), buffer, offset, limit),
        Point::WattsPhaseA => {
            if let Some(value) = model.watts_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::WattsPhaseB => {
            if let Some(value) = model.watts_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::WattsPhaseC => {
            if let Some(value) = model.watts_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::Va => {
            if let Some(value) = model.va() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::VaPhaseA => {
            if let Some(value) = model.va_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::VaPhaseB => {
            if let Some(value) = model.va_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::VaPhaseC => {
            if let Some(value) = model.va_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::Var => {
            if let Some(value) = model.var() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::VarPhaseA => {
            if let Some(value) = model.var_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::VarPhaseB => {
            if let Some(value) = model.var_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::VarPhaseC => {
            if let Some(value) = model.var_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::Pf => {
            if let Some(value) = model.pf() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PfPhaseA => {
            if let Some(value) = model.pf_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PfPhaseB => {
            if let Some(value) = model.pf_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::PfPhaseC => {
            if let Some(value) = model.pf_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursExported => {
            serialisation::write_f32(model.total_watt_hours_exported(), buffer, offset, limit)
        }
        Point::TotalWattHoursExportedPhaseA => {
            if let Some(value) = model.total_watt_hours_exported_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursExportedPhaseB => {
            if let Some(value) = model.total_watt_hours_exported_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursExportedPhaseC => {
            if let Some(value) = model.total_watt_hours_exported_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursImported => {
            serialisation::write_f32(model.total_watt_hours_imported(), buffer, offset, limit)
        }
        Point::TotalWattHoursImportedPhaseA => {
            if let Some(value) = model.total_watt_hours_imported_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursImportedPhaseB => {
            if let Some(value) = model.total_watt_hours_imported_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursImportedPhaseC => {
            if let Some(value) = model.total_watt_hours_imported_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursExported => {
            if let Some(value) = model.total_va_hours_exported() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursExportedPhaseA => {
            if let Some(value) = model.total_va_hours_exported_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursExportedPhaseB => {
            if let Some(value) = model.total_va_hours_exported_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursExportedPhaseC => {
            if let Some(value) = model.total_va_hours_exported_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursImported => {
            if let Some(value) = model.total_va_hours_imported() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursImportedPhaseA => {
            if let Some(value) = model.total_va_hours_imported_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursImportedPhaseB => {
            if let Some(value) = model.total_va_hours_imported_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVaHoursImportedPhaseC => {
            if let Some(value) = model.total_va_hours_imported_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVarHoursImportedQ1 => {
            if let Some(value) = model.total_var_hours_imported_q1() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseA => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseB => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseC => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ2 => {
            if let Some(value) = model.total_v_ar_hours_imported_q2() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseA => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseB => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseC => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ3 => {
            if let Some(value) = model.total_v_ar_hours_exported_q3() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseA => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseB => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseC => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ4 => {
            if let Some(value) = model.total_v_ar_hours_exported_q4() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseA => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_a() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseB => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_b() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseC => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_c() {
                serialisation::write_f32(value, buffer, offset, limit);
            }
        }
        Point::Events => serialisation::write_u32(model.events(), buffer, offset, limit),
    }
}

pub trait ModelAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> f32;

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> f32;

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<f32> {
        None
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<f32> {
        None
    }

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    fn voltage_ln(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<f32> {
        None
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn voltage_ll(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<f32> {
        None
    }

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> f32;

    /// Watts
    ///
    /// Total Real Power
    fn watts(&self) -> f32;

    /// Watts phase A
    fn watts_phase_a(&self) -> Option<f32> {
        None
    }

    /// Watts phase B
    fn watts_phase_b(&self) -> Option<f32> {
        None
    }

    /// Watts phase C
    fn watts_phase_c(&self) -> Option<f32> {
        None
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        None
    }

    /// VA phase A
    fn va_phase_a(&self) -> Option<f32> {
        None
    }

    /// VA phase B
    fn va_phase_b(&self) -> Option<f32> {
        None
    }

    /// VA phase C
    fn va_phase_c(&self) -> Option<f32> {
        None
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<f32> {
        None
    }

    /// VAR phase A
    fn var_phase_a(&self) -> Option<f32> {
        None
    }

    /// VAR phase B
    fn var_phase_b(&self) -> Option<f32> {
        None
    }

    /// VAR phase C
    fn var_phase_c(&self) -> Option<f32> {
        None
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<f32> {
        None
    }

    /// PF phase A
    fn pf_phase_a(&self) -> Option<f32> {
        None
    }

    /// PF phase B
    fn pf_phase_b(&self) -> Option<f32> {
        None
    }

    /// PF phase C
    fn pf_phase_c(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn total_watt_hours_exported(&self) -> f32;

    /// Total Watt-hours Exported phase A
    fn total_watt_hours_exported_phase_a(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Exported phase B
    fn total_watt_hours_exported_phase_b(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Exported phase C
    fn total_watt_hours_exported_phase_c(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> f32;

    /// Total Watt-hours Imported phase A
    fn total_watt_hours_imported_phase_a(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Imported phase B
    fn total_watt_hours_imported_phase_b(&self) -> Option<f32> {
        None
    }

    /// Total Watt-hours Imported phase C
    fn total_watt_hours_imported_phase_c(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Exported phase A
    fn total_va_hours_exported_phase_a(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Exported phase B
    fn total_va_hours_exported_phase_b(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Exported phase C
    fn total_va_hours_exported_phase_c(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Imported phase A
    fn total_va_hours_imported_phase_a(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Imported phase B
    fn total_va_hours_imported_phase_b(&self) -> Option<f32> {
        None
    }

    /// Total VA-hours Imported phase C
    fn total_va_hours_imported_phase_c(&self) -> Option<f32> {
        None
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn total_var_hours_imported_q1(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase A
    fn total_v_ar_hours_imported_q1_phase_a(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase B
    fn total_v_ar_hours_imported_q1_phase_b(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q1 phase C
    fn total_v_ar_hours_imported_q1_phase_c(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase A
    fn total_v_ar_hours_imported_q2_phase_a(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase B
    fn total_v_ar_hours_imported_q2_phase_b(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Imported Q2 phase C
    fn total_v_ar_hours_imported_q2_phase_c(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase A
    fn total_v_ar_hours_exported_q3_phase_a(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase B
    fn total_v_ar_hours_exported_q3_phase_b(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q3 phase C
    fn total_v_ar_hours_exported_q3_phase_c(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn total_v_ar_hours_exported_q4_imported_phase_a(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn total_v_ar_hours_exported_q4_imported_phase_b(&self) -> Option<f32> {
        None
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn total_v_ar_hours_exported_q4_imported_phase_c(&self) -> Option<f32> {
        None
    }

    /// Events
    ///
    /// Meter Event Flags
    fn events(&self) -> u32;
}

#[repr(C)]
pub struct Model211CallbackAdapter {
    amps_callback: extern "C" fn() -> f32,
    amps_phase_a_callback: extern "C" fn() -> f32,
    amps_phase_b_callback: Option<extern "C" fn() -> f32>,
    amps_phase_c_callback: Option<extern "C" fn() -> f32>,
    voltage_ln_callback: Option<extern "C" fn() -> f32>,
    phase_voltage_an_callback: Option<extern "C" fn() -> f32>,
    phase_voltage_bn_callback: Option<extern "C" fn() -> f32>,
    phase_voltage_cn_callback: Option<extern "C" fn() -> f32>,
    voltage_ll_callback: Option<extern "C" fn() -> f32>,
    phase_voltage_ab_callback: Option<extern "C" fn() -> f32>,
    phase_voltage_bc_callback: Option<extern "C" fn() -> f32>,
    phase_voltage_ca_callback: Option<extern "C" fn() -> f32>,
    hz_callback: extern "C" fn() -> f32,
    watts_callback: extern "C" fn() -> f32,
    watts_phase_a_callback: Option<extern "C" fn() -> f32>,
    watts_phase_b_callback: Option<extern "C" fn() -> f32>,
    watts_phase_c_callback: Option<extern "C" fn() -> f32>,
    va_callback: Option<extern "C" fn() -> f32>,
    va_phase_a_callback: Option<extern "C" fn() -> f32>,
    va_phase_b_callback: Option<extern "C" fn() -> f32>,
    va_phase_c_callback: Option<extern "C" fn() -> f32>,
    var_callback: Option<extern "C" fn() -> f32>,
    var_phase_a_callback: Option<extern "C" fn() -> f32>,
    var_phase_b_callback: Option<extern "C" fn() -> f32>,
    var_phase_c_callback: Option<extern "C" fn() -> f32>,
    pf_callback: Option<extern "C" fn() -> f32>,
    pf_phase_a_callback: Option<extern "C" fn() -> f32>,
    pf_phase_b_callback: Option<extern "C" fn() -> f32>,
    pf_phase_c_callback: Option<extern "C" fn() -> f32>,
    total_watt_hours_exported_callback: extern "C" fn() -> f32,
    total_watt_hours_exported_phase_a_callback: Option<extern "C" fn() -> f32>,
    total_watt_hours_exported_phase_b_callback: Option<extern "C" fn() -> f32>,
    total_watt_hours_exported_phase_c_callback: Option<extern "C" fn() -> f32>,
    total_watt_hours_imported_callback: extern "C" fn() -> f32,
    total_watt_hours_imported_phase_a_callback: Option<extern "C" fn() -> f32>,
    total_watt_hours_imported_phase_b_callback: Option<extern "C" fn() -> f32>,
    total_watt_hours_imported_phase_c_callback: Option<extern "C" fn() -> f32>,
    total_va_hours_exported_callback: Option<extern "C" fn() -> f32>,
    total_va_hours_exported_phase_a_callback: Option<extern "C" fn() -> f32>,
    total_va_hours_exported_phase_b_callback: Option<extern "C" fn() -> f32>,
    total_va_hours_exported_phase_c_callback: Option<extern "C" fn() -> f32>,
    total_va_hours_imported_callback: Option<extern "C" fn() -> f32>,
    total_va_hours_imported_phase_a_callback: Option<extern "C" fn() -> f32>,
    total_va_hours_imported_phase_b_callback: Option<extern "C" fn() -> f32>,
    total_va_hours_imported_phase_c_callback: Option<extern "C" fn() -> f32>,
    total_var_hours_imported_q1_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_imported_q1_phase_a_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_imported_q1_phase_b_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_imported_q1_phase_c_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_imported_q2_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_imported_q2_phase_a_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_imported_q2_phase_b_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_imported_q2_phase_c_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_exported_q3_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_exported_q3_phase_a_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_exported_q3_phase_b_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_exported_q3_phase_c_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_exported_q4_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_exported_q4_imported_phase_a_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_exported_q4_imported_phase_b_callback: Option<extern "C" fn() -> f32>,
    total_v_ar_hours_exported_q4_imported_phase_c_callback: Option<extern "C" fn() -> f32>,
    events_callback: extern "C" fn() -> u32,
}

impl ModelAdapter for Model211CallbackAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> f32 {
        (self.amps_callback)()
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> f32 {
        (self.amps_phase_a_callback)()
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<f32> {
        self.amps_phase_b_callback.map(|callback| (callback)())
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<f32> {
        self.amps_phase_c_callback.map(|callback| (callback)())
    }

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    fn voltage_ln(&self) -> Option<f32> {
        self.voltage_ln_callback.map(|callback| (callback)())
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> Option<f32> {
        self.phase_voltage_an_callback.map(|callback| (callback)())
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<f32> {
        self.phase_voltage_bn_callback.map(|callback| (callback)())
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<f32> {
        self.phase_voltage_cn_callback.map(|callback| (callback)())
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn voltage_ll(&self) -> Option<f32> {
        self.voltage_ll_callback.map(|callback| (callback)())
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<f32> {
        self.phase_voltage_ab_callback.map(|callback| (callback)())
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<f32> {
        self.phase_voltage_bc_callback.map(|callback| (callback)())
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<f32> {
        self.phase_voltage_ca_callback.map(|callback| (callback)())
    }

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> f32 {
        (self.hz_callback)()
    }

    /// Watts
    ///
    /// Total Real Power
    fn watts(&self) -> f32 {
        (self.watts_callback)()
    }

    /// Watts phase A
    fn watts_phase_a(&self) -> Option<f32> {
        self.watts_phase_a_callback.map(|callback| (callback)())
    }

    /// Watts phase B
    fn watts_phase_b(&self) -> Option<f32> {
        self.watts_phase_b_callback.map(|callback| (callback)())
    }

    /// Watts phase C
    fn watts_phase_c(&self) -> Option<f32> {
        self.watts_phase_c_callback.map(|callback| (callback)())
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        self.va_callback.map(|callback| (callback)())
    }

    /// VA phase A
    fn va_phase_a(&self) -> Option<f32> {
        self.va_phase_a_callback.map(|callback| (callback)())
    }

    /// VA phase B
    fn va_phase_b(&self) -> Option<f32> {
        self.va_phase_b_callback.map(|callback| (callback)())
    }

    /// VA phase C
    fn va_phase_c(&self) -> Option<f32> {
        self.va_phase_c_callback.map(|callback| (callback)())
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<f32> {
        self.var_callback.map(|callback| (callback)())
    }

    /// VAR phase A
    fn var_phase_a(&self) -> Option<f32> {
        self.var_phase_a_callback.map(|callback| (callback)())
    }

    /// VAR phase B
    fn var_phase_b(&self) -> Option<f32> {
        self.var_phase_b_callback.map(|callback| (callback)())
    }

    /// VAR phase C
    fn var_phase_c(&self) -> Option<f32> {
        self.var_phase_c_callback.map(|callback| (callback)())
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<f32> {
        self.pf_callback.map(|callback| (callback)())
    }

    /// PF phase A
    fn pf_phase_a(&self) -> Option<f32> {
        self.pf_phase_a_callback.map(|callback| (callback)())
    }

    /// PF phase B
    fn pf_phase_b(&self) -> Option<f32> {
        self.pf_phase_b_callback.map(|callback| (callback)())
    }

    /// PF phase C
    fn pf_phase_c(&self) -> Option<f32> {
        self.pf_phase_c_callback.map(|callback| (callback)())
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn total_watt_hours_exported(&self) -> f32 {
        (self.total_watt_hours_exported_callback)()
    }

    /// Total Watt-hours Exported phase A
    fn total_watt_hours_exported_phase_a(&self) -> Option<f32> {
        self.total_watt_hours_exported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Exported phase B
    fn total_watt_hours_exported_phase_b(&self) -> Option<f32> {
        self.total_watt_hours_exported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Exported phase C
    fn total_watt_hours_exported_phase_c(&self) -> Option<f32> {
        self.total_watt_hours_exported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> f32 {
        (self.total_watt_hours_imported_callback)()
    }

    /// Total Watt-hours Imported phase A
    fn total_watt_hours_imported_phase_a(&self) -> Option<f32> {
        self.total_watt_hours_imported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Imported phase B
    fn total_watt_hours_imported_phase_b(&self) -> Option<f32> {
        self.total_watt_hours_imported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-hours Imported phase C
    fn total_watt_hours_imported_phase_c(&self) -> Option<f32> {
        self.total_watt_hours_imported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<f32> {
        self.total_va_hours_exported_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Exported phase A
    fn total_va_hours_exported_phase_a(&self) -> Option<f32> {
        self.total_va_hours_exported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Exported phase B
    fn total_va_hours_exported_phase_b(&self) -> Option<f32> {
        self.total_va_hours_exported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Exported phase C
    fn total_va_hours_exported_phase_c(&self) -> Option<f32> {
        self.total_va_hours_exported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<f32> {
        self.total_va_hours_imported_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Imported phase A
    fn total_va_hours_imported_phase_a(&self) -> Option<f32> {
        self.total_va_hours_imported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Imported phase B
    fn total_va_hours_imported_phase_b(&self) -> Option<f32> {
        self.total_va_hours_imported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VA-hours Imported phase C
    fn total_va_hours_imported_phase_c(&self) -> Option<f32> {
        self.total_va_hours_imported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn total_var_hours_imported_q1(&self) -> Option<f32> {
        self.total_var_hours_imported_q1_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q1 phase A
    fn total_v_ar_hours_imported_q1_phase_a(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q1_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q1 phase B
    fn total_v_ar_hours_imported_q1_phase_b(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q1_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q1 phase C
    fn total_v_ar_hours_imported_q1_phase_c(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q1_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q2_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q2 phase A
    fn total_v_ar_hours_imported_q2_phase_a(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q2_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q2 phase B
    fn total_v_ar_hours_imported_q2_phase_b(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q2_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Imported Q2 phase C
    fn total_v_ar_hours_imported_q2_phase_c(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q2_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q3_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q3 phase A
    fn total_v_ar_hours_exported_q3_phase_a(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q3_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q3 phase B
    fn total_v_ar_hours_exported_q3_phase_b(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q3_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q3 phase C
    fn total_v_ar_hours_exported_q3_phase_c(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q3_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q4_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn total_v_ar_hours_exported_q4_imported_phase_a(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q4_imported_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn total_v_ar_hours_exported_q4_imported_phase_b(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q4_imported_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn total_v_ar_hours_exported_q4_imported_phase_c(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q4_imported_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Events
    ///
    /// Meter Event Flags
    fn events(&self) -> u32 {
        (self.events_callback)()
    }
}
