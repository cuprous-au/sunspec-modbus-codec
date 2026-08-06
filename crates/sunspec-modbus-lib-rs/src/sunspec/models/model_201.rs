use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 107;

pub static POINTS: [ReadablePoint; 74] = [
    ReadablePoint {
        reference: PointReference::Static { value: 201 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::Amps },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::AmpsPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::AmpsPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::AmpsPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VoltageLn,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PhaseVoltageAn,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PhaseVoltageBn,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PhaseVoltageCn,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VoltageLl,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PhaseVoltageAb,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PhaseVoltageBc,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PhaseVoltageCa,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::Hz },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::HzSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::Watts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::WattsPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::WattsPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::WattsPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::WSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::Va },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VaPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VaPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VaPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::VaSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::Var },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VarPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VarPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VarPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::VarSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::Pf },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PfPhaseA,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PfPhaseB,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::PfPhaseC,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 { point: Point::PfSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalWattHoursExported,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalWattHoursExportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalWattHoursExportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalWattHoursExportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalWattHoursImported,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalWattHoursImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalWattHoursImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalWattHoursImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotWhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVaHoursExported,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVaHoursExportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVaHoursExportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVaHoursExportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVaHoursImported,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVaHoursImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVaHoursImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVaHoursImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotVAhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVarHoursImportedQ1,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursImportedQ1PhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursImportedQ1PhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursImportedQ1PhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursImportedQ2,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursImportedQ2PhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursImportedQ2PhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursImportedQ2PhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursExportedQ3,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursExportedQ3PhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursExportedQ3PhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursExportedQ3PhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursExportedQ4,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseA,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseB,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotalVArHoursExportedQ4ImportedPhaseC,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::TotVArhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model201 {
            point: Point::Events,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
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

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    107
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
        Point::Amps => {
            buffer::write_i16(model.amps(), buffer);
        }
        Point::AmpsPhaseA => {
            buffer::write_i16(model.amps_phase_a(), buffer);
        }
        Point::AmpsPhaseB => {
            if let Some(value) = model.amps_phase_b() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AmpsPhaseC => {
            if let Some(value) = model.amps_phase_c() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ASf => {
            buffer::write_u16(model.a_sf(), buffer);
        }
        Point::VoltageLn => {
            if let Some(value) = model.voltage_ln() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageAn => {
            if let Some(value) = model.phase_voltage_an() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageBn => {
            if let Some(value) = model.phase_voltage_bn() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageCn => {
            if let Some(value) = model.phase_voltage_cn() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageLl => {
            if let Some(value) = model.voltage_ll() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageAb => {
            if let Some(value) = model.phase_voltage_ab() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageBc => {
            if let Some(value) = model.phase_voltage_bc() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageCa => {
            if let Some(value) = model.phase_voltage_ca() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
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
                buffer::zero(buffer, offset);
            }
        }
        Point::Watts => {
            buffer::write_i16(model.watts(), buffer);
        }
        Point::WattsPhaseA => {
            if let Some(value) = model.watts_phase_a() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WattsPhaseB => {
            if let Some(value) = model.watts_phase_b() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WattsPhaseC => {
            if let Some(value) = model.watts_phase_c() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WSf => {
            buffer::write_u16(model.w_sf(), buffer);
        }
        Point::Va => {
            if let Some(value) = model.va() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VaPhaseA => {
            if let Some(value) = model.va_phase_a() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VaPhaseB => {
            if let Some(value) = model.va_phase_b() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VaPhaseC => {
            if let Some(value) = model.va_phase_c() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VaSf => {
            if let Some(value) = model.va_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Var => {
            if let Some(value) = model.var() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VarPhaseA => {
            if let Some(value) = model.var_phase_a() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VarPhaseB => {
            if let Some(value) = model.var_phase_b() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VarPhaseC => {
            if let Some(value) = model.var_phase_c() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VarSf => {
            if let Some(value) = model.var_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Pf => {
            if let Some(value) = model.pf() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfPhaseA => {
            if let Some(value) = model.pf_phase_a() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfPhaseB => {
            if let Some(value) = model.pf_phase_b() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfPhaseC => {
            if let Some(value) = model.pf_phase_c() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfSf => {
            if let Some(value) = model.pf_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursExported => {
            buffer::write_u32(model.total_watt_hours_exported(), buffer, offset, limit);
        }
        Point::TotalWattHoursExportedPhaseA => {
            if let Some(value) = model.total_watt_hours_exported_phase_a() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursExportedPhaseB => {
            if let Some(value) = model.total_watt_hours_exported_phase_b() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursExportedPhaseC => {
            if let Some(value) = model.total_watt_hours_exported_phase_c() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursImported => {
            buffer::write_u32(model.total_watt_hours_imported(), buffer, offset, limit);
        }
        Point::TotalWattHoursImportedPhaseA => {
            if let Some(value) = model.total_watt_hours_imported_phase_a() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursImportedPhaseB => {
            if let Some(value) = model.total_watt_hours_imported_phase_b() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursImportedPhaseC => {
            if let Some(value) = model.total_watt_hours_imported_phase_c() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotWhSf => {
            buffer::write_u16(model.tot_wh_sf(), buffer);
        }
        Point::TotalVaHoursExported => {
            if let Some(value) = model.total_va_hours_exported() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVaHoursExportedPhaseA => {
            if let Some(value) = model.total_va_hours_exported_phase_a() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVaHoursExportedPhaseB => {
            if let Some(value) = model.total_va_hours_exported_phase_b() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVaHoursExportedPhaseC => {
            if let Some(value) = model.total_va_hours_exported_phase_c() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVaHoursImported => {
            if let Some(value) = model.total_va_hours_imported() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVaHoursImportedPhaseA => {
            if let Some(value) = model.total_va_hours_imported_phase_a() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVaHoursImportedPhaseB => {
            if let Some(value) = model.total_va_hours_imported_phase_b() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVaHoursImportedPhaseC => {
            if let Some(value) = model.total_va_hours_imported_phase_c() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotVAhSf => {
            if let Some(value) = model.tot_v_ah_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVarHoursImportedQ1 => {
            if let Some(value) = model.total_var_hours_imported_q1() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseA => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_a() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseB => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_b() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursImportedQ1PhaseC => {
            if let Some(value) = model.total_v_ar_hours_imported_q1_phase_c() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursImportedQ2 => {
            if let Some(value) = model.total_v_ar_hours_imported_q2() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseA => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_a() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseB => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_b() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursImportedQ2PhaseC => {
            if let Some(value) = model.total_v_ar_hours_imported_q2_phase_c() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursExportedQ3 => {
            if let Some(value) = model.total_v_ar_hours_exported_q3() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseA => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_a() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseB => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_b() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursExportedQ3PhaseC => {
            if let Some(value) = model.total_v_ar_hours_exported_q3_phase_c() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursExportedQ4 => {
            if let Some(value) = model.total_v_ar_hours_exported_q4() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseA => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_a() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseB => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_b() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVArHoursExportedQ4ImportedPhaseC => {
            if let Some(value) = model.total_v_ar_hours_exported_q4_imported_phase_c() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotVArhSf => {
            if let Some(value) = model.tot_v_arh_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
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
    fn amps(&self) -> i16;

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> i16;

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<i16> {
        None
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<i16> {
        None
    }

    /// Current scale factor
    fn a_sf(&self) -> u16;

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    ///
    /// Conditional AN connection
    fn voltage_ln(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ///
    /// Conditional AN connection
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
    fn voltage_ll(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    ///
    /// Conditional AB connection
    fn phase_voltage_ab(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<i16> {
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
pub struct Model201CallbackAdapter {
    context: *mut c_void,
    amps_callback: extern "C" fn(*const c_void) -> i16,
    amps_phase_a_callback: extern "C" fn(*const c_void) -> i16,
    amps_phase_b_callback: Option<extern "C" fn(*const c_void) -> i16>,
    amps_phase_c_callback: Option<extern "C" fn(*const c_void) -> i16>,
    a_sf_callback: extern "C" fn(*const c_void) -> u16,
    voltage_ln_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_an_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_bn_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_cn_callback: Option<extern "C" fn(*const c_void) -> i16>,
    voltage_ll_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_ab_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_bc_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_ca_callback: Option<extern "C" fn(*const c_void) -> i16>,
    v_sf_callback: extern "C" fn(*const c_void) -> u16,
    hz_callback: extern "C" fn(*const c_void) -> i16,
    hz_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    watts_callback: extern "C" fn(*const c_void) -> i16,
    watts_phase_a_callback: Option<extern "C" fn(*const c_void) -> i16>,
    watts_phase_b_callback: Option<extern "C" fn(*const c_void) -> i16>,
    watts_phase_c_callback: Option<extern "C" fn(*const c_void) -> i16>,
    w_sf_callback: extern "C" fn(*const c_void) -> u16,
    va_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_phase_a_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_phase_b_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_phase_c_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    var_callback: Option<extern "C" fn(*const c_void) -> i16>,
    var_phase_a_callback: Option<extern "C" fn(*const c_void) -> i16>,
    var_phase_b_callback: Option<extern "C" fn(*const c_void) -> i16>,
    var_phase_c_callback: Option<extern "C" fn(*const c_void) -> i16>,
    var_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    pf_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_phase_a_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_phase_b_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_phase_c_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_watt_hours_exported_callback: extern "C" fn(*const c_void) -> u32,
    total_watt_hours_exported_phase_a_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_watt_hours_exported_phase_b_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_watt_hours_exported_phase_c_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_watt_hours_imported_callback: extern "C" fn(*const c_void) -> u32,
    total_watt_hours_imported_phase_a_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_watt_hours_imported_phase_b_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_watt_hours_imported_phase_c_callback: Option<extern "C" fn(*const c_void) -> u32>,
    tot_wh_sf_callback: extern "C" fn(*const c_void) -> u16,
    total_va_hours_exported_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_va_hours_exported_phase_a_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_va_hours_exported_phase_b_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_va_hours_exported_phase_c_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_va_hours_imported_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_va_hours_imported_phase_a_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_va_hours_imported_phase_b_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_va_hours_imported_phase_c_callback: Option<extern "C" fn(*const c_void) -> u32>,
    tot_v_ah_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_var_hours_imported_q1_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_imported_q1_phase_a_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_imported_q1_phase_b_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_imported_q1_phase_c_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_imported_q2_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_imported_q2_phase_a_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_imported_q2_phase_b_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_imported_q2_phase_c_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q3_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q3_phase_a_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q3_phase_b_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q3_phase_c_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q4_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q4_imported_phase_a_callback:
        Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q4_imported_phase_b_callback:
        Option<extern "C" fn(*const c_void) -> u32>,
    total_v_ar_hours_exported_q4_imported_phase_c_callback:
        Option<extern "C" fn(*const c_void) -> u32>,
    tot_v_arh_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    events_callback: extern "C" fn(*const c_void) -> u32,
}

impl ModelAdapter for Model201CallbackAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> i16 {
        (self.amps_callback)(self.context)
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> i16 {
        (self.amps_phase_a_callback)(self.context)
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<i16> {
        self.amps_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<i16> {
        self.amps_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Current scale factor
    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)(self.context)
    }

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    ///
    /// Conditional AN connection
    fn voltage_ln(&self) -> Option<i16> {
        self.voltage_ln_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ///
    /// Conditional AN connection
    fn phase_voltage_an(&self) -> Option<i16> {
        self.phase_voltage_an_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<i16> {
        self.phase_voltage_bn_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<i16> {
        self.phase_voltage_cn_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn voltage_ll(&self) -> Option<i16> {
        self.voltage_ll_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    ///
    /// Conditional AB connection
    fn phase_voltage_ab(&self) -> Option<i16> {
        self.phase_voltage_ab_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<i16> {
        self.phase_voltage_bc_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<i16> {
        self.phase_voltage_ca_callback
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

    /// Watts phase A
    fn watts_phase_a(&self) -> Option<i16> {
        self.watts_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts phase B
    fn watts_phase_b(&self) -> Option<i16> {
        self.watts_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts phase C
    fn watts_phase_c(&self) -> Option<i16> {
        self.watts_phase_c_callback
            .map(|callback| (callback)(self.context))
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

    /// VA phase A
    fn va_phase_a(&self) -> Option<i16> {
        self.va_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// VA phase B
    fn va_phase_b(&self) -> Option<i16> {
        self.va_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// VA phase C
    fn va_phase_c(&self) -> Option<i16> {
        self.va_phase_c_callback
            .map(|callback| (callback)(self.context))
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

    /// VAR phase A
    fn var_phase_a(&self) -> Option<i16> {
        self.var_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAR phase B
    fn var_phase_b(&self) -> Option<i16> {
        self.var_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// VAR phase C
    fn var_phase_c(&self) -> Option<i16> {
        self.var_phase_c_callback
            .map(|callback| (callback)(self.context))
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

    /// PF phase A
    fn pf_phase_a(&self) -> Option<i16> {
        self.pf_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF phase B
    fn pf_phase_b(&self) -> Option<i16> {
        self.pf_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF phase C
    fn pf_phase_c(&self) -> Option<i16> {
        self.pf_phase_c_callback
            .map(|callback| (callback)(self.context))
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

    /// Total Watt-hours Exported phase A
    fn total_watt_hours_exported_phase_a(&self) -> Option<u32> {
        self.total_watt_hours_exported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Exported phase B
    fn total_watt_hours_exported_phase_b(&self) -> Option<u32> {
        self.total_watt_hours_exported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Exported phase C
    fn total_watt_hours_exported_phase_c(&self) -> Option<u32> {
        self.total_watt_hours_exported_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> u32 {
        (self.total_watt_hours_imported_callback)(self.context)
    }

    /// Total Watt-hours Imported phase A
    fn total_watt_hours_imported_phase_a(&self) -> Option<u32> {
        self.total_watt_hours_imported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Imported phase B
    fn total_watt_hours_imported_phase_b(&self) -> Option<u32> {
        self.total_watt_hours_imported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-hours Imported phase C
    fn total_watt_hours_imported_phase_c(&self) -> Option<u32> {
        self.total_watt_hours_imported_phase_c_callback
            .map(|callback| (callback)(self.context))
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

    /// Total VA-hours Exported phase A
    fn total_va_hours_exported_phase_a(&self) -> Option<u32> {
        self.total_va_hours_exported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Exported phase B
    fn total_va_hours_exported_phase_b(&self) -> Option<u32> {
        self.total_va_hours_exported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Exported phase C
    fn total_va_hours_exported_phase_c(&self) -> Option<u32> {
        self.total_va_hours_exported_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<u32> {
        self.total_va_hours_imported_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported phase A
    fn total_va_hours_imported_phase_a(&self) -> Option<u32> {
        self.total_va_hours_imported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported phase B
    fn total_va_hours_imported_phase_b(&self) -> Option<u32> {
        self.total_va_hours_imported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VA-hours Imported phase C
    fn total_va_hours_imported_phase_c(&self) -> Option<u32> {
        self.total_va_hours_imported_phase_c_callback
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

    /// Total VAr-hours Imported Q1 phase A
    fn total_v_ar_hours_imported_q1_phase_a(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q1_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q1 phase B
    fn total_v_ar_hours_imported_q1_phase_b(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q1_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q1 phase C
    fn total_v_ar_hours_imported_q1_phase_c(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q1_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2 phase A
    fn total_v_ar_hours_imported_q2_phase_a(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2 phase B
    fn total_v_ar_hours_imported_q2_phase_b(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Imported Q2 phase C
    fn total_v_ar_hours_imported_q2_phase_c(&self) -> Option<u32> {
        self.total_v_ar_hours_imported_q2_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3 phase A
    fn total_v_ar_hours_exported_q3_phase_a(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3 phase B
    fn total_v_ar_hours_exported_q3_phase_b(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q3 phase C
    fn total_v_ar_hours_exported_q3_phase_c(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q3_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn total_v_ar_hours_exported_q4_imported_phase_a(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_imported_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn total_v_ar_hours_exported_q4_imported_phase_b(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_imported_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn total_v_ar_hours_exported_q4_imported_phase_c(&self) -> Option<u32> {
        self.total_v_ar_hours_exported_q4_imported_phase_c_callback
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
}

#[repr(C)]
pub struct Model201StatefulAdapter {
    amps: i16,
    amps_phase_a: i16,
    amps_phase_b: i16,
    amps_phase_c: i16,
    a_sf: u16,
    voltage_ln: i16,
    phase_voltage_an: i16,
    phase_voltage_bn: i16,
    phase_voltage_cn: i16,
    voltage_ll: i16,
    phase_voltage_ab: i16,
    phase_voltage_bc: i16,
    phase_voltage_ca: i16,
    v_sf: u16,
    hz: i16,
    hz_sf: u16,
    watts: i16,
    watts_phase_a: i16,
    watts_phase_b: i16,
    watts_phase_c: i16,
    w_sf: u16,
    va: i16,
    va_phase_a: i16,
    va_phase_b: i16,
    va_phase_c: i16,
    va_sf: u16,
    var: i16,
    var_phase_a: i16,
    var_phase_b: i16,
    var_phase_c: i16,
    var_sf: u16,
    pf: i16,
    pf_phase_a: i16,
    pf_phase_b: i16,
    pf_phase_c: i16,
    pf_sf: u16,
    total_watt_hours_exported: u32,
    total_watt_hours_exported_phase_a: u32,
    total_watt_hours_exported_phase_b: u32,
    total_watt_hours_exported_phase_c: u32,
    total_watt_hours_imported: u32,
    total_watt_hours_imported_phase_a: u32,
    total_watt_hours_imported_phase_b: u32,
    total_watt_hours_imported_phase_c: u32,
    tot_wh_sf: u16,
    total_va_hours_exported: u32,
    total_va_hours_exported_phase_a: u32,
    total_va_hours_exported_phase_b: u32,
    total_va_hours_exported_phase_c: u32,
    total_va_hours_imported: u32,
    total_va_hours_imported_phase_a: u32,
    total_va_hours_imported_phase_b: u32,
    total_va_hours_imported_phase_c: u32,
    tot_v_ah_sf: u16,
    total_var_hours_imported_q1: u32,
    total_v_ar_hours_imported_q1_phase_a: u32,
    total_v_ar_hours_imported_q1_phase_b: u32,
    total_v_ar_hours_imported_q1_phase_c: u32,
    total_v_ar_hours_imported_q2: u32,
    total_v_ar_hours_imported_q2_phase_a: u32,
    total_v_ar_hours_imported_q2_phase_b: u32,
    total_v_ar_hours_imported_q2_phase_c: u32,
    total_v_ar_hours_exported_q3: u32,
    total_v_ar_hours_exported_q3_phase_a: u32,
    total_v_ar_hours_exported_q3_phase_b: u32,
    total_v_ar_hours_exported_q3_phase_c: u32,
    total_v_ar_hours_exported_q4: u32,
    total_v_ar_hours_exported_q4_imported_phase_a: u32,
    total_v_ar_hours_exported_q4_imported_phase_b: u32,
    total_v_ar_hours_exported_q4_imported_phase_c: u32,
    tot_v_arh_sf: u16,
    events: u32,
}

impl ModelAdapter for Model201StatefulAdapter {
    /// Amps
    ///
    /// Total AC Current
    fn amps(&self) -> i16 {
        self.amps
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    fn amps_phase_a(&self) -> i16 {
        self.amps_phase_a
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<i16> {
        Some(self.amps_phase_b)
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<i16> {
        Some(self.amps_phase_c)
    }

    /// Current scale factor
    fn a_sf(&self) -> u16 {
        self.a_sf
    }

    /// Voltage LN
    ///
    /// Line to Neutral AC Voltage (average of active phases)
    ///
    /// Conditional AN connection
    fn voltage_ln(&self) -> Option<i16> {
        Some(self.voltage_ln)
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    ///
    /// Conditional AN connection
    fn phase_voltage_an(&self) -> Option<i16> {
        Some(self.phase_voltage_an)
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<i16> {
        Some(self.phase_voltage_bn)
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<i16> {
        Some(self.phase_voltage_cn)
    }

    /// Voltage LL
    ///
    /// Line to Line AC Voltage (average of active phases)
    fn voltage_ll(&self) -> Option<i16> {
        Some(self.voltage_ll)
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    ///
    /// Conditional AB connection
    fn phase_voltage_ab(&self) -> Option<i16> {
        Some(self.phase_voltage_ab)
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<i16> {
        Some(self.phase_voltage_bc)
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<i16> {
        Some(self.phase_voltage_ca)
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

    /// Watts phase A
    fn watts_phase_a(&self) -> Option<i16> {
        Some(self.watts_phase_a)
    }

    /// Watts phase B
    fn watts_phase_b(&self) -> Option<i16> {
        Some(self.watts_phase_b)
    }

    /// Watts phase C
    fn watts_phase_c(&self) -> Option<i16> {
        Some(self.watts_phase_c)
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

    /// VA phase A
    fn va_phase_a(&self) -> Option<i16> {
        Some(self.va_phase_a)
    }

    /// VA phase B
    fn va_phase_b(&self) -> Option<i16> {
        Some(self.va_phase_b)
    }

    /// VA phase C
    fn va_phase_c(&self) -> Option<i16> {
        Some(self.va_phase_c)
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

    /// VAR phase A
    fn var_phase_a(&self) -> Option<i16> {
        Some(self.var_phase_a)
    }

    /// VAR phase B
    fn var_phase_b(&self) -> Option<i16> {
        Some(self.var_phase_b)
    }

    /// VAR phase C
    fn var_phase_c(&self) -> Option<i16> {
        Some(self.var_phase_c)
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

    /// PF phase A
    fn pf_phase_a(&self) -> Option<i16> {
        Some(self.pf_phase_a)
    }

    /// PF phase B
    fn pf_phase_b(&self) -> Option<i16> {
        Some(self.pf_phase_b)
    }

    /// PF phase C
    fn pf_phase_c(&self) -> Option<i16> {
        Some(self.pf_phase_c)
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

    /// Total Watt-hours Exported phase A
    fn total_watt_hours_exported_phase_a(&self) -> Option<u32> {
        Some(self.total_watt_hours_exported_phase_a)
    }

    /// Total Watt-hours Exported phase B
    fn total_watt_hours_exported_phase_b(&self) -> Option<u32> {
        Some(self.total_watt_hours_exported_phase_b)
    }

    /// Total Watt-hours Exported phase C
    fn total_watt_hours_exported_phase_c(&self) -> Option<u32> {
        Some(self.total_watt_hours_exported_phase_c)
    }

    /// Total Watt-hours Imported
    ///
    /// Total Real Energy Imported
    fn total_watt_hours_imported(&self) -> u32 {
        self.total_watt_hours_imported
    }

    /// Total Watt-hours Imported phase A
    fn total_watt_hours_imported_phase_a(&self) -> Option<u32> {
        Some(self.total_watt_hours_imported_phase_a)
    }

    /// Total Watt-hours Imported phase B
    fn total_watt_hours_imported_phase_b(&self) -> Option<u32> {
        Some(self.total_watt_hours_imported_phase_b)
    }

    /// Total Watt-hours Imported phase C
    fn total_watt_hours_imported_phase_c(&self) -> Option<u32> {
        Some(self.total_watt_hours_imported_phase_c)
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

    /// Total VA-hours Exported phase A
    fn total_va_hours_exported_phase_a(&self) -> Option<u32> {
        Some(self.total_va_hours_exported_phase_a)
    }

    /// Total VA-hours Exported phase B
    fn total_va_hours_exported_phase_b(&self) -> Option<u32> {
        Some(self.total_va_hours_exported_phase_b)
    }

    /// Total VA-hours Exported phase C
    fn total_va_hours_exported_phase_c(&self) -> Option<u32> {
        Some(self.total_va_hours_exported_phase_c)
    }

    /// Total VA-hours Imported
    ///
    /// Total Apparent Energy Imported
    fn total_va_hours_imported(&self) -> Option<u32> {
        Some(self.total_va_hours_imported)
    }

    /// Total VA-hours Imported phase A
    fn total_va_hours_imported_phase_a(&self) -> Option<u32> {
        Some(self.total_va_hours_imported_phase_a)
    }

    /// Total VA-hours Imported phase B
    fn total_va_hours_imported_phase_b(&self) -> Option<u32> {
        Some(self.total_va_hours_imported_phase_b)
    }

    /// Total VA-hours Imported phase C
    fn total_va_hours_imported_phase_c(&self) -> Option<u32> {
        Some(self.total_va_hours_imported_phase_c)
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

    /// Total VAr-hours Imported Q1 phase A
    fn total_v_ar_hours_imported_q1_phase_a(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_imported_q1_phase_a)
    }

    /// Total VAr-hours Imported Q1 phase B
    fn total_v_ar_hours_imported_q1_phase_b(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_imported_q1_phase_b)
    }

    /// Total VAr-hours Imported Q1 phase C
    fn total_v_ar_hours_imported_q1_phase_c(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_imported_q1_phase_c)
    }

    /// Total VAr-hours Imported Q2
    ///
    /// Total Reactive Power Imported Quadrant 2
    fn total_v_ar_hours_imported_q2(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_imported_q2)
    }

    /// Total VAr-hours Imported Q2 phase A
    fn total_v_ar_hours_imported_q2_phase_a(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_imported_q2_phase_a)
    }

    /// Total VAr-hours Imported Q2 phase B
    fn total_v_ar_hours_imported_q2_phase_b(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_imported_q2_phase_b)
    }

    /// Total VAr-hours Imported Q2 phase C
    fn total_v_ar_hours_imported_q2_phase_c(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_imported_q2_phase_c)
    }

    /// Total VAr-hours Exported Q3
    ///
    /// Total Reactive Power Exported Quadrant 3
    fn total_v_ar_hours_exported_q3(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q3)
    }

    /// Total VAr-hours Exported Q3 phase A
    fn total_v_ar_hours_exported_q3_phase_a(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q3_phase_a)
    }

    /// Total VAr-hours Exported Q3 phase B
    fn total_v_ar_hours_exported_q3_phase_b(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q3_phase_b)
    }

    /// Total VAr-hours Exported Q3 phase C
    fn total_v_ar_hours_exported_q3_phase_c(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q3_phase_c)
    }

    /// Total VAr-hours Exported Q4
    ///
    /// Total Reactive Power Exported Quadrant 4
    fn total_v_ar_hours_exported_q4(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q4)
    }

    /// Total VAr-hours Exported Q4 Imported phase A
    fn total_v_ar_hours_exported_q4_imported_phase_a(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q4_imported_phase_a)
    }

    /// Total VAr-hours Exported Q4 Imported phase B
    fn total_v_ar_hours_exported_q4_imported_phase_b(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q4_imported_phase_b)
    }

    /// Total VAr-hours Exported Q4 Imported phase C
    fn total_v_ar_hours_exported_q4_imported_phase_c(&self) -> Option<u32> {
        Some(self.total_v_ar_hours_exported_q4_imported_phase_c)
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
}
