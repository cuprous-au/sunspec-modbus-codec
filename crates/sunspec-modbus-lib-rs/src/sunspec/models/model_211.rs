use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 126;

static POINTS: [PointDetails<()>; 64] = [
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
        size: 2,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::AmpsPhaseA,
        size: 2,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::AmpsPhaseB,
        size: 2,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::AmpsPhaseC,
        size: 2,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::VoltageLn,
        size: 2,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::PhaseVoltageAn,
        size: 2,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::PhaseVoltageBn,
        size: 2,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::PhaseVoltageCn,
        size: 2,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::VoltageLl,
        size: 2,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::PhaseVoltageAb,
        size: 2,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::PhaseVoltageBc,
        size: 2,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::PhaseVoltageCa,
        size: 2,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::Hz,
        size: 2,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::Watts,
        size: 2,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::WattsPhaseA,
        size: 2,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::WattsPhaseB,
        size: 2,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::WattsPhaseC,
        size: 2,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::Va,
        size: 2,
        start_address: 36,
    },
    PointDetails {
        point: |()| Point::VaPhaseA,
        size: 2,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::VaPhaseB,
        size: 2,
        start_address: 40,
    },
    PointDetails {
        point: |()| Point::VaPhaseC,
        size: 2,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::Var,
        size: 2,
        start_address: 44,
    },
    PointDetails {
        point: |()| Point::VarPhaseA,
        size: 2,
        start_address: 46,
    },
    PointDetails {
        point: |()| Point::VarPhaseB,
        size: 2,
        start_address: 48,
    },
    PointDetails {
        point: |()| Point::VarPhaseC,
        size: 2,
        start_address: 50,
    },
    PointDetails {
        point: |()| Point::Pf,
        size: 2,
        start_address: 52,
    },
    PointDetails {
        point: |()| Point::PfPhaseA,
        size: 2,
        start_address: 54,
    },
    PointDetails {
        point: |()| Point::PfPhaseB,
        size: 2,
        start_address: 56,
    },
    PointDetails {
        point: |()| Point::PfPhaseC,
        size: 2,
        start_address: 58,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursExported,
        size: 2,
        start_address: 60,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursExportedPhaseA,
        size: 2,
        start_address: 62,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursExportedPhaseB,
        size: 2,
        start_address: 64,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursExportedPhaseC,
        size: 2,
        start_address: 66,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursImported,
        size: 2,
        start_address: 68,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursImportedPhaseA,
        size: 2,
        start_address: 70,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursImportedPhaseB,
        size: 2,
        start_address: 72,
    },
    PointDetails {
        point: |()| Point::TotalWattHoursImportedPhaseC,
        size: 2,
        start_address: 74,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursExported,
        size: 2,
        start_address: 76,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursExportedPhaseA,
        size: 2,
        start_address: 78,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursExportedPhaseB,
        size: 2,
        start_address: 80,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursExportedPhaseC,
        size: 2,
        start_address: 82,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursImported,
        size: 2,
        start_address: 84,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursImportedPhaseA,
        size: 2,
        start_address: 86,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursImportedPhaseB,
        size: 2,
        start_address: 88,
    },
    PointDetails {
        point: |()| Point::TotalVaHoursImportedPhaseC,
        size: 2,
        start_address: 90,
    },
    PointDetails {
        point: |()| Point::TotalVarHoursImportedQ1,
        size: 2,
        start_address: 92,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursImportedQ1PhaseA,
        size: 2,
        start_address: 94,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursImportedQ1PhaseB,
        size: 2,
        start_address: 96,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursImportedQ1PhaseC,
        size: 2,
        start_address: 98,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursImportedQ2,
        size: 2,
        start_address: 100,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursImportedQ2PhaseA,
        size: 2,
        start_address: 102,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursImportedQ2PhaseB,
        size: 2,
        start_address: 104,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursImportedQ2PhaseC,
        size: 2,
        start_address: 106,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ3,
        size: 2,
        start_address: 108,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ3PhaseA,
        size: 2,
        start_address: 110,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ3PhaseB,
        size: 2,
        start_address: 112,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ3PhaseC,
        size: 2,
        start_address: 114,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ4,
        size: 2,
        start_address: 116,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ4ImportedPhaseA,
        size: 2,
        start_address: 118,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ4ImportedPhaseB,
        size: 2,
        start_address: 120,
    },
    PointDetails {
        point: |()| Point::TotalVArHoursExportedQ4ImportedPhaseC,
        size: 2,
        start_address: 122,
    },
    PointDetails {
        point: |()| Point::Events,
        size: 2,
        start_address: 124,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
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

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    126
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
            buffer::write_u16(211, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Amps => {
            buffer::write_f32(model.amps(), buffer, offset, limit);
        }
        Point::AmpsPhaseA => {
            buffer::write_f32(model.amps_phase_a(), buffer, offset, limit);
        }
        Point::AmpsPhaseB => {
            if let Some(value) = model.amps_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::AmpsPhaseC => {
            if let Some(value) = model.amps_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::VoltageLn => {
            if let Some(value) = model.voltage_ln() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PhaseVoltageAn => {
            if let Some(value) = model.phase_voltage_an() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PhaseVoltageBn => {
            if let Some(value) = model.phase_voltage_bn() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PhaseVoltageCn => {
            if let Some(value) = model.phase_voltage_cn() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::VoltageLl => {
            if let Some(value) = model.voltage_ll() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PhaseVoltageAb => {
            if let Some(value) = model.phase_voltage_ab() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PhaseVoltageBc => {
            if let Some(value) = model.phase_voltage_bc() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PhaseVoltageCa => {
            if let Some(value) = model.phase_voltage_ca() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Hz => {
            buffer::write_f32(model.hz(), buffer, offset, limit);
        }
        Point::Watts => {
            buffer::write_f32(model.watts(), buffer, offset, limit);
        }
        Point::WattsPhaseA => {
            if let Some(value) = model.watts_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::WattsPhaseB => {
            if let Some(value) = model.watts_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::WattsPhaseC => {
            if let Some(value) = model.watts_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Va => {
            if let Some(value) = model.va() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::VaPhaseA => {
            if let Some(value) = model.va_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::VaPhaseB => {
            if let Some(value) = model.va_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::VaPhaseC => {
            if let Some(value) = model.va_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Var => {
            if let Some(value) = model.var() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::VarPhaseA => {
            if let Some(value) = model.var_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::VarPhaseB => {
            if let Some(value) = model.var_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::VarPhaseC => {
            if let Some(value) = model.var_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Pf => {
            if let Some(value) = model.pf() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PfPhaseA => {
            if let Some(value) = model.pf_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PfPhaseB => {
            if let Some(value) = model.pf_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::PfPhaseC => {
            if let Some(value) = model.pf_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalWattHoursExported => {
            buffer::write_f32(model.total_watt_hours_exported(), buffer, offset, limit);
        }
        Point::TotalWattHoursExportedPhaseA => {
            if let Some(value) = model.total_watt_hours_exported_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalWattHoursExportedPhaseB => {
            if let Some(value) = model.total_watt_hours_exported_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalWattHoursExportedPhaseC => {
            if let Some(value) = model.total_watt_hours_exported_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalWattHoursImported => {
            buffer::write_f32(model.total_watt_hours_imported(), buffer, offset, limit);
        }
        Point::TotalWattHoursImportedPhaseA => {
            if let Some(value) = model.total_watt_hours_imported_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalWattHoursImportedPhaseB => {
            if let Some(value) = model.total_watt_hours_imported_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalWattHoursImportedPhaseC => {
            if let Some(value) = model.total_watt_hours_imported_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursExported => {
            if let Some(value) = model.total_va_hours_exported() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursExportedPhaseA => {
            if let Some(value) = model.total_va_hours_exported_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursExportedPhaseB => {
            if let Some(value) = model.total_va_hours_exported_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursExportedPhaseC => {
            if let Some(value) = model.total_va_hours_exported_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursImported => {
            if let Some(value) = model.total_va_hours_imported() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursImportedPhaseA => {
            if let Some(value) = model.total_va_hours_imported_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursImportedPhaseB => {
            if let Some(value) = model.total_va_hours_imported_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVaHoursImportedPhaseC => {
            if let Some(value) = model.total_va_hours_imported_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVarHoursImportedQ1 => {
            if let Some(value) = model.total_var_hours_imported_q1() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseA => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseB => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseC => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursImportedQ2 => {
            if let Some(value) = model.total_v_ar_hours_imported_q2() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseA => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseB => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseC => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ3 => {
            if let Some(value) = model.total_v_ar_hours_exported_q3() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseA => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseB => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseC => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ4 => {
            if let Some(value) = model.total_v_ar_hours_exported_q4() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseA => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_a() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseB => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseC => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Events => {
            buffer::write_u32(model.events(), buffer, offset, limit);
        }
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
    context: *mut c_void,
    amps_callback: extern "C" fn(*const c_void) -> f32,
    amps_phase_a_callback: extern "C" fn(*const c_void) -> f32,
    amps_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    amps_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    voltage_ln_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_an_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_bn_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_cn_callback: Option<extern "C" fn(*const c_void) -> f32>,
    voltage_ll_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_ab_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_bc_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_ca_callback: Option<extern "C" fn(*const c_void) -> f32>,
    hz_callback: extern "C" fn(*const c_void) -> f32,
    watts_callback: extern "C" fn(*const c_void) -> f32,
    watts_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    watts_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    watts_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    va_callback: Option<extern "C" fn(*const c_void) -> f32>,
    va_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    va_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    va_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    var_callback: Option<extern "C" fn(*const c_void) -> f32>,
    var_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    var_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    var_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    pf_callback: Option<extern "C" fn(*const c_void) -> f32>,
    pf_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    pf_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    pf_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_watt_hours_exported_callback: extern "C" fn(*const c_void) -> f32,
    total_watt_hours_exported_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_watt_hours_exported_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_watt_hours_exported_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_watt_hours_imported_callback: extern "C" fn(*const c_void) -> f32,
    total_watt_hours_imported_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_watt_hours_imported_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_watt_hours_imported_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_va_hours_exported_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_va_hours_exported_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_va_hours_exported_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_va_hours_exported_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_va_hours_imported_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_va_hours_imported_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_va_hours_imported_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_va_hours_imported_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_var_hours_imported_q1_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_imported_q1_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_imported_q1_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_imported_q1_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_imported_q2_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_imported_q2_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_imported_q2_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_imported_q2_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_exported_q3_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_exported_q3_phase_a_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_exported_q3_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_exported_q3_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_exported_q4_callback: Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_exported_q4_imported_phase_a_callback:
        Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_exported_q4_imported_phase_b_callback:
        Option<extern "C" fn(*const c_void) -> f32>,
    total_v_ar_hours_exported_q4_imported_phase_c_callback:
        Option<extern "C" fn(*const c_void) -> f32>,
    events_callback: extern "C" fn(*const c_void) -> u32,
}

impl ModelAdapter for Model211CallbackAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> f32 {
        (self.amps_callback)(self.context)
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> f32 {
        (self.amps_phase_a_callback)(self.context)
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<f32> {
        self.amps_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<f32> {
        self.amps_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    fn voltage_ln(&self) -> Option<f32> {
        self.voltage_ln_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> Option<f32> {
        self.phase_voltage_an_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<f32> {
        self.phase_voltage_bn_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<f32> {
        self.phase_voltage_cn_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn voltage_ll(&self) -> Option<f32> {
        self.voltage_ll_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<f32> {
        self.phase_voltage_ab_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<f32> {
        self.phase_voltage_bc_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<f32> {
        self.phase_voltage_ca_callback
            .map(|callback| (callback)(self.context))
    }

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> f32 {
        (self.hz_callback)(self.context)
    }

    /// Watts
    ///
    /// Total Real Power
    fn watts(&self) -> f32 {
        (self.watts_callback)(self.context)
    }

    /// Watts phase A
    fn watts_phase_a(&self) -> Option<f32> {
        self.watts_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts phase B
    fn watts_phase_b(&self) -> Option<f32> {
        self.watts_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts phase C
    fn watts_phase_c(&self) -> Option<f32> {
        self.watts_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        self.va_callback.map(|callback| (callback)(self.context))
    }

    /// VA phase A
    fn va_phase_a(&self) -> Option<f32> {
        self.va_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// VA phase B
    fn va_phase_b(&self) -> Option<f32> {
        self.va_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// VA phase C
    fn va_phase_c(&self) -> Option<f32> {
        self.va_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<f32> {
        self.var_callback.map(|callback| (callback)(self.context))
    }

    /// VAR phase A
    fn var_phase_a(&self) -> Option<f32> {
        self.var_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAR phase B
    fn var_phase_b(&self) -> Option<f32> {
        self.var_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAR phase C
    fn var_phase_c(&self) -> Option<f32> {
        self.var_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<f32> {
        self.pf_callback.map(|callback| (callback)(self.context))
    }

    /// PF phase A
    fn pf_phase_a(&self) -> Option<f32> {
        self.pf_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF phase B
    fn pf_phase_b(&self) -> Option<f32> {
        self.pf_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF phase C
    fn pf_phase_c(&self) -> Option<f32> {
        self.pf_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn total_watt_hours_exported(&self) -> f32 {
        (self.total_watt_hours_exported_callback)(self.context)
    }

    /// Total Watt-hours Exported phase A
    fn total_watt_hours_exported_phase_a(&self) -> Option<f32> {
        self.total_watt_hours_exported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Exported phase B
    fn total_watt_hours_exported_phase_b(&self) -> Option<f32> {
        self.total_watt_hours_exported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Exported phase C
    fn total_watt_hours_exported_phase_c(&self) -> Option<f32> {
        self.total_watt_hours_exported_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> f32 {
        (self.total_watt_hours_imported_callback)(self.context)
    }

    /// Total Watt-hours Imported phase A
    fn total_watt_hours_imported_phase_a(&self) -> Option<f32> {
        self.total_watt_hours_imported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Imported phase B
    fn total_watt_hours_imported_phase_b(&self) -> Option<f32> {
        self.total_watt_hours_imported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Imported phase C
    fn total_watt_hours_imported_phase_c(&self) -> Option<f32> {
        self.total_watt_hours_imported_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<f32> {
        self.total_va_hours_exported_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Exported phase A
    fn total_va_hours_exported_phase_a(&self) -> Option<f32> {
        self.total_va_hours_exported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Exported phase B
    fn total_va_hours_exported_phase_b(&self) -> Option<f32> {
        self.total_va_hours_exported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Exported phase C
    fn total_va_hours_exported_phase_c(&self) -> Option<f32> {
        self.total_va_hours_exported_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<f32> {
        self.total_va_hours_imported_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported phase A
    fn total_va_hours_imported_phase_a(&self) -> Option<f32> {
        self.total_va_hours_imported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported phase B
    fn total_va_hours_imported_phase_b(&self) -> Option<f32> {
        self.total_va_hours_imported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported phase C
    fn total_va_hours_imported_phase_c(&self) -> Option<f32> {
        self.total_va_hours_imported_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn total_var_hours_imported_q1(&self) -> Option<f32> {
        self.total_var_hours_imported_q1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q1 phase A
    fn total_v_ar_hours_imported_q1_phase_a(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q1_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q1 phase B
    fn total_v_ar_hours_imported_q1_phase_b(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q1_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q1 phase C
    fn total_v_ar_hours_imported_q1_phase_c(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q1_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2 phase A
    fn total_v_ar_hours_imported_q2_phase_a(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q2_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2 phase B
    fn total_v_ar_hours_imported_q2_phase_b(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q2_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2 phase C
    fn total_v_ar_hours_imported_q2_phase_c(&self) -> Option<f32> {
        self.total_v_ar_hours_imported_q2_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3 phase A
    fn total_v_ar_hours_exported_q3_phase_a(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q3_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3 phase B
    fn total_v_ar_hours_exported_q3_phase_b(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q3_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3 phase C
    fn total_v_ar_hours_exported_q3_phase_c(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q3_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q4_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn total_v_ar_hours_exported_q4_imported_phase_a(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q4_imported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn total_v_ar_hours_exported_q4_imported_phase_b(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q4_imported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn total_v_ar_hours_exported_q4_imported_phase_c(&self) -> Option<f32> {
        self.total_v_ar_hours_exported_q4_imported_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Events
    ///
    /// Meter Event Flags
    fn events(&self) -> u32 {
        (self.events_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model211StatefulAdapter {
    amps: f32,
    amps_phase_a: f32,
    amps_phase_b: f32,
    amps_phase_c: f32,
    voltage_ln: f32,
    phase_voltage_an: f32,
    phase_voltage_bn: f32,
    phase_voltage_cn: f32,
    voltage_ll: f32,
    phase_voltage_ab: f32,
    phase_voltage_bc: f32,
    phase_voltage_ca: f32,
    hz: f32,
    watts: f32,
    watts_phase_a: f32,
    watts_phase_b: f32,
    watts_phase_c: f32,
    va: f32,
    va_phase_a: f32,
    va_phase_b: f32,
    va_phase_c: f32,
    var: f32,
    var_phase_a: f32,
    var_phase_b: f32,
    var_phase_c: f32,
    pf: f32,
    pf_phase_a: f32,
    pf_phase_b: f32,
    pf_phase_c: f32,
    total_watt_hours_exported: f32,
    total_watt_hours_exported_phase_a: f32,
    total_watt_hours_exported_phase_b: f32,
    total_watt_hours_exported_phase_c: f32,
    total_watt_hours_imported: f32,
    total_watt_hours_imported_phase_a: f32,
    total_watt_hours_imported_phase_b: f32,
    total_watt_hours_imported_phase_c: f32,
    total_va_hours_exported: f32,
    total_va_hours_exported_phase_a: f32,
    total_va_hours_exported_phase_b: f32,
    total_va_hours_exported_phase_c: f32,
    total_va_hours_imported: f32,
    total_va_hours_imported_phase_a: f32,
    total_va_hours_imported_phase_b: f32,
    total_va_hours_imported_phase_c: f32,
    total_var_hours_imported_q1: f32,
    total_v_ar_hours_imported_q1_phase_a: f32,
    total_v_ar_hours_imported_q1_phase_b: f32,
    total_v_ar_hours_imported_q1_phase_c: f32,
    total_v_ar_hours_imported_q2: f32,
    total_v_ar_hours_imported_q2_phase_a: f32,
    total_v_ar_hours_imported_q2_phase_b: f32,
    total_v_ar_hours_imported_q2_phase_c: f32,
    total_v_ar_hours_exported_q3: f32,
    total_v_ar_hours_exported_q3_phase_a: f32,
    total_v_ar_hours_exported_q3_phase_b: f32,
    total_v_ar_hours_exported_q3_phase_c: f32,
    total_v_ar_hours_exported_q4: f32,
    total_v_ar_hours_exported_q4_imported_phase_a: f32,
    total_v_ar_hours_exported_q4_imported_phase_b: f32,
    total_v_ar_hours_exported_q4_imported_phase_c: f32,
    events: u32,
}

impl ModelAdapter for Model211StatefulAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> f32 {
        self.amps
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> f32 {
        self.amps_phase_a
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<f32> {
        Some(self.amps_phase_b)
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<f32> {
        Some(self.amps_phase_c)
    }

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    fn voltage_ln(&self) -> Option<f32> {
        Some(self.voltage_ln)
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> Option<f32> {
        Some(self.phase_voltage_an)
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<f32> {
        Some(self.phase_voltage_bn)
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<f32> {
        Some(self.phase_voltage_cn)
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn voltage_ll(&self) -> Option<f32> {
        Some(self.voltage_ll)
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<f32> {
        Some(self.phase_voltage_ab)
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<f32> {
        Some(self.phase_voltage_bc)
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<f32> {
        Some(self.phase_voltage_ca)
    }

    /// Hz
    ///
    /// Frequency
    fn hz(&self) -> f32 {
        self.hz
    }

    /// Watts
    ///
    /// Total Real Power
    fn watts(&self) -> f32 {
        self.watts
    }

    /// Watts phase A
    fn watts_phase_a(&self) -> Option<f32> {
        Some(self.watts_phase_a)
    }

    /// Watts phase B
    fn watts_phase_b(&self) -> Option<f32> {
        Some(self.watts_phase_b)
    }

    /// Watts phase C
    fn watts_phase_c(&self) -> Option<f32> {
        Some(self.watts_phase_c)
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        Some(self.va)
    }

    /// VA phase A
    fn va_phase_a(&self) -> Option<f32> {
        Some(self.va_phase_a)
    }

    /// VA phase B
    fn va_phase_b(&self) -> Option<f32> {
        Some(self.va_phase_b)
    }

    /// VA phase C
    fn va_phase_c(&self) -> Option<f32> {
        Some(self.va_phase_c)
    }

    /// VAR
    ///
    /// Reactive Power
    fn var(&self) -> Option<f32> {
        Some(self.var)
    }

    /// VAR phase A
    fn var_phase_a(&self) -> Option<f32> {
        Some(self.var_phase_a)
    }

    /// VAR phase B
    fn var_phase_b(&self) -> Option<f32> {
        Some(self.var_phase_b)
    }

    /// VAR phase C
    fn var_phase_c(&self) -> Option<f32> {
        Some(self.var_phase_c)
    }

    /// PF
    ///
    /// Power Factor
    fn pf(&self) -> Option<f32> {
        Some(self.pf)
    }

    /// PF phase A
    fn pf_phase_a(&self) -> Option<f32> {
        Some(self.pf_phase_a)
    }

    /// PF phase B
    fn pf_phase_b(&self) -> Option<f32> {
        Some(self.pf_phase_b)
    }

    /// PF phase C
    fn pf_phase_c(&self) -> Option<f32> {
        Some(self.pf_phase_c)
    }

    /// Total Watt-hours Exported
    ///
    /// Total Real Energy Exported
    fn total_watt_hours_exported(&self) -> f32 {
        self.total_watt_hours_exported
    }

    /// Total Watt-hours Exported phase A
    fn total_watt_hours_exported_phase_a(&self) -> Option<f32> {
        Some(self.total_watt_hours_exported_phase_a)
    }

    /// Total Watt-hours Exported phase B
    fn total_watt_hours_exported_phase_b(&self) -> Option<f32> {
        Some(self.total_watt_hours_exported_phase_b)
    }

    /// Total Watt-hours Exported phase C
    fn total_watt_hours_exported_phase_c(&self) -> Option<f32> {
        Some(self.total_watt_hours_exported_phase_c)
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> f32 {
        self.total_watt_hours_imported
    }

    /// Total Watt-hours Imported phase A
    fn total_watt_hours_imported_phase_a(&self) -> Option<f32> {
        Some(self.total_watt_hours_imported_phase_a)
    }

    /// Total Watt-hours Imported phase B
    fn total_watt_hours_imported_phase_b(&self) -> Option<f32> {
        Some(self.total_watt_hours_imported_phase_b)
    }

    /// Total Watt-hours Imported phase C
    fn total_watt_hours_imported_phase_c(&self) -> Option<f32> {
        Some(self.total_watt_hours_imported_phase_c)
    }

    /// Total VA-hours Exported
    ///
    /// Total Apparent Energy Exported
    fn total_va_hours_exported(&self) -> Option<f32> {
        Some(self.total_va_hours_exported)
    }

    /// Total VA-hours Exported phase A
    fn total_va_hours_exported_phase_a(&self) -> Option<f32> {
        Some(self.total_va_hours_exported_phase_a)
    }

    /// Total VA-hours Exported phase B
    fn total_va_hours_exported_phase_b(&self) -> Option<f32> {
        Some(self.total_va_hours_exported_phase_b)
    }

    /// Total VA-hours Exported phase C
    fn total_va_hours_exported_phase_c(&self) -> Option<f32> {
        Some(self.total_va_hours_exported_phase_c)
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<f32> {
        Some(self.total_va_hours_imported)
    }

    /// Total VA-hours Imported phase A
    fn total_va_hours_imported_phase_a(&self) -> Option<f32> {
        Some(self.total_va_hours_imported_phase_a)
    }

    /// Total VA-hours Imported phase B
    fn total_va_hours_imported_phase_b(&self) -> Option<f32> {
        Some(self.total_va_hours_imported_phase_b)
    }

    /// Total VA-hours Imported phase C
    fn total_va_hours_imported_phase_c(&self) -> Option<f32> {
        Some(self.total_va_hours_imported_phase_c)
    }

    /// Total VAR-hours Imported Q1
    ///
    /// Total Reactive Energy Imported Quadrant 1
    fn total_var_hours_imported_q1(&self) -> Option<f32> {
        Some(self.total_var_hours_imported_q1)
    }

    /// Total VAr-hours Imported Q1 phase A
    fn total_v_ar_hours_imported_q1_phase_a(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_imported_q1_phase_a)
    }

    /// Total VAr-hours Imported Q1 phase B
    fn total_v_ar_hours_imported_q1_phase_b(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_imported_q1_phase_b)
    }

    /// Total VAr-hours Imported Q1 phase C
    fn total_v_ar_hours_imported_q1_phase_c(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_imported_q1_phase_c)
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_imported_q2)
    }

    /// Total VAr-hours Imported Q2 phase A
    fn total_v_ar_hours_imported_q2_phase_a(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_imported_q2_phase_a)
    }

    /// Total VAr-hours Imported Q2 phase B
    fn total_v_ar_hours_imported_q2_phase_b(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_imported_q2_phase_b)
    }

    /// Total VAr-hours Imported Q2 phase C
    fn total_v_ar_hours_imported_q2_phase_c(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_imported_q2_phase_c)
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_exported_q3)
    }

    /// Total VAr-hours Exported Q3 phase A
    fn total_v_ar_hours_exported_q3_phase_a(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_exported_q3_phase_a)
    }

    /// Total VAr-hours Exported Q3 phase B
    fn total_v_ar_hours_exported_q3_phase_b(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_exported_q3_phase_b)
    }

    /// Total VAr-hours Exported Q3 phase C
    fn total_v_ar_hours_exported_q3_phase_c(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_exported_q3_phase_c)
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_exported_q4)
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn total_v_ar_hours_exported_q4_imported_phase_a(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_exported_q4_imported_phase_a)
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn total_v_ar_hours_exported_q4_imported_phase_b(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_exported_q4_imported_phase_b)
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn total_v_ar_hours_exported_q4_imported_phase_c(&self) -> Option<f32> {
        Some(self.total_v_ar_hours_exported_q4_imported_phase_c)
    }

    /// Events
    ///
    /// Meter Event Flags
    fn events(&self) -> u32 {
        self.events
    }
}
