use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 155;

pub static POINTS: [ReadablePoint; 72] = [
    ReadablePoint {
        reference: PointReference::Static { value: 701 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 153 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::AcWiringType,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::OperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::InverterState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::GridConnectionState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::AlarmBitfield,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::DerOperationalCharacteristics,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ActivePower,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ApparentPower,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ReactivePower,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::PowerFactor,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalAcCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::VoltageLl,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::VoltageLn,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::Frequency,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalEnergyInjected,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalEnergyAbsorbed,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalReactiveEnergyInj,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalReactiveEnergyAbs,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::AmbientTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::CabinetTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::HeatSinkTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TransformerTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::IgbtMosfetTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::OtherTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::WattsL1,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 { point: Point::VaL1 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::VarL1,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 { point: Point::PfL1 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::AmpsL1,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::PhaseVoltageL1L2,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::PhaseVoltageL1N,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalWattHoursInjL1,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalWattHoursAbsL1,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalVarHoursInjL1,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalVarHoursAbsL1,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::WattsL2,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 { point: Point::VaL2 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::VarL2,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 { point: Point::PfL2 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::AmpsL2,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::PhaseVoltageL2L3,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::PhaseVoltageL2N,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalWattHoursInjL2,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalWattHoursAbsL2,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalVarHoursInjL2,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalVarHoursAbsL2,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::WattsL3,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 { point: Point::VaL3 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::VarL3,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 { point: Point::PfL3 },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::AmpsL3,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::PhaseVoltageL3L1,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::PhaseVoltageL3N,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalWattHoursInjL3,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalWattHoursAbsL3,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalVarHoursInjL3,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TotalVarHoursAbsL3,
        },
        size: 4,
        data_type: PointType::Uint64,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ThrottlingInPct,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ThrottleSourceInformation,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::CurrentScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::FrequencyScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ActivePowerScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::PowerFactorScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ApparentPowerScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ReactivePowerScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ActiveEnergyScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ReactiveEnergyScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::TemperatureScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model701 {
            point: Point::ManufacturerAlarmInfo,
        },
        size: 32,
        data_type: PointType::String,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    AcWiringType,
    OperatingState,
    InverterState,
    GridConnectionState,
    AlarmBitfield,
    DerOperationalCharacteristics,
    ActivePower,
    ApparentPower,
    ReactivePower,
    PowerFactor,
    TotalAcCurrent,
    VoltageLl,
    VoltageLn,
    Frequency,
    TotalEnergyInjected,
    TotalEnergyAbsorbed,
    TotalReactiveEnergyInj,
    TotalReactiveEnergyAbs,
    AmbientTemperature,
    CabinetTemperature,
    HeatSinkTemperature,
    TransformerTemperature,
    IgbtMosfetTemperature,
    OtherTemperature,
    WattsL1,
    VaL1,
    VarL1,
    PfL1,
    AmpsL1,
    PhaseVoltageL1L2,
    PhaseVoltageL1N,
    TotalWattHoursInjL1,
    TotalWattHoursAbsL1,
    TotalVarHoursInjL1,
    TotalVarHoursAbsL1,
    WattsL2,
    VaL2,
    VarL2,
    PfL2,
    AmpsL2,
    PhaseVoltageL2L3,
    PhaseVoltageL2N,
    TotalWattHoursInjL2,
    TotalWattHoursAbsL2,
    TotalVarHoursInjL2,
    TotalVarHoursAbsL2,
    WattsL3,
    VaL3,
    VarL3,
    PfL3,
    AmpsL3,
    PhaseVoltageL3L1,
    PhaseVoltageL3N,
    TotalWattHoursInjL3,
    TotalWattHoursAbsL3,
    TotalVarHoursInjL3,
    TotalVarHoursAbsL3,
    ThrottlingInPct,
    ThrottleSourceInformation,
    CurrentScaleFactor,
    VoltageScaleFactor,
    FrequencyScaleFactor,
    ActivePowerScaleFactor,
    PowerFactorScaleFactor,
    ApparentPowerScaleFactor,
    ReactivePowerScaleFactor,
    ActiveEnergyScaleFactor,
    ReactiveEnergyScaleFactor,
    TemperatureScaleFactor,
    ManufacturerAlarmInfo,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    155
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::AcWiringType => {
            buffer::write_u16(model.ac_wiring_type() as u16, buffer);
        }
        Point::OperatingState => {
            if let Some(value) = model.operating_state() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::InverterState => {
            if let Some(value) = model.inverter_state() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::GridConnectionState => {
            if let Some(value) = model.grid_connection_state() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AlarmBitfield => {
            if let Some(value) = model.alarm_bitfield() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DerOperationalCharacteristics => {
            if let Some(value) = model.der_operational_characteristics() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePower => {
            if let Some(value) = model.active_power() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ApparentPower => {
            if let Some(value) = model.apparent_power() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePower => {
            if let Some(value) = model.reactive_power() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerFactor => {
            if let Some(value) = model.power_factor() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalAcCurrent => {
            if let Some(value) = model.total_ac_current() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageLl => {
            if let Some(value) = model.voltage_ll() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageLn => {
            if let Some(value) = model.voltage_ln() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Frequency => {
            if let Some(value) = model.frequency() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalEnergyInjected => {
            if let Some(value) = model.total_energy_injected() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalEnergyAbsorbed => {
            if let Some(value) = model.total_energy_absorbed() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalReactiveEnergyInj => {
            if let Some(value) = model.total_reactive_energy_inj() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalReactiveEnergyAbs => {
            if let Some(value) = model.total_reactive_energy_abs() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AmbientTemperature => {
            if let Some(value) = model.ambient_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CabinetTemperature => {
            if let Some(value) = model.cabinet_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::HeatSinkTemperature => {
            if let Some(value) = model.heat_sink_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TransformerTemperature => {
            if let Some(value) = model.transformer_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IgbtMosfetTemperature => {
            if let Some(value) = model.igbt_mosfet_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OtherTemperature => {
            if let Some(value) = model.other_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WattsL1 => {
            if let Some(value) = model.watts_l1() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VaL1 => {
            if let Some(value) = model.va_l1() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VarL1 => {
            if let Some(value) = model.var_l1() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfL1 => {
            if let Some(value) = model.pf_l1() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AmpsL1 => {
            if let Some(value) = model.amps_l1() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageL1L2 => {
            if let Some(value) = model.phase_voltage_l1_l2() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageL1N => {
            if let Some(value) = model.phase_voltage_l1_n() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursInjL1 => {
            if let Some(value) = model.total_watt_hours_inj_l1() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursAbsL1 => {
            if let Some(value) = model.total_watt_hours_abs_l1() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVarHoursInjL1 => {
            if let Some(value) = model.total_var_hours_inj_l1() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVarHoursAbsL1 => {
            if let Some(value) = model.total_var_hours_abs_l1() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WattsL2 => {
            if let Some(value) = model.watts_l2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VaL2 => {
            if let Some(value) = model.va_l2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VarL2 => {
            if let Some(value) = model.var_l2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfL2 => {
            if let Some(value) = model.pf_l2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AmpsL2 => {
            if let Some(value) = model.amps_l2() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageL2L3 => {
            if let Some(value) = model.phase_voltage_l2_l3() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageL2N => {
            if let Some(value) = model.phase_voltage_l2_n() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursInjL2 => {
            if let Some(value) = model.total_watt_hours_inj_l2() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursAbsL2 => {
            if let Some(value) = model.total_watt_hours_abs_l2() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVarHoursInjL2 => {
            if let Some(value) = model.total_var_hours_inj_l2() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVarHoursAbsL2 => {
            if let Some(value) = model.total_var_hours_abs_l2() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WattsL3 => {
            if let Some(value) = model.watts_l3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VaL3 => {
            if let Some(value) = model.va_l3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VarL3 => {
            if let Some(value) = model.var_l3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfL3 => {
            if let Some(value) = model.pf_l3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AmpsL3 => {
            if let Some(value) = model.amps_l3() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageL3L1 => {
            if let Some(value) = model.phase_voltage_l3_l1() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageL3N => {
            if let Some(value) = model.phase_voltage_l3_n() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursInjL3 => {
            if let Some(value) = model.total_watt_hours_inj_l3() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalWattHoursAbsL3 => {
            if let Some(value) = model.total_watt_hours_abs_l3() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVarHoursInjL3 => {
            if let Some(value) = model.total_var_hours_inj_l3() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalVarHoursAbsL3 => {
            if let Some(value) = model.total_var_hours_abs_l3() {
                buffer::write_u64(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ThrottlingInPct => {
            if let Some(value) = model.throttling_in_pct() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ThrottleSourceInformation => {
            if let Some(value) = model.throttle_source_information() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentScaleFactor => {
            if let Some(value) = model.current_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageScaleFactor => {
            if let Some(value) = model.voltage_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::FrequencyScaleFactor => {
            if let Some(value) = model.frequency_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerScaleFactor => {
            if let Some(value) = model.active_power_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerFactorScaleFactor => {
            if let Some(value) = model.power_factor_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ApparentPowerScaleFactor => {
            if let Some(value) = model.apparent_power_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerScaleFactor => {
            if let Some(value) = model.reactive_power_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActiveEnergyScaleFactor => {
            if let Some(value) = model.active_energy_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactiveEnergyScaleFactor => {
            if let Some(value) = model.reactive_energy_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TemperatureScaleFactor => {
            if let Some(value) = model.temperature_scale_factor() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ManufacturerAlarmInfo => {
            if let Some(value) = model.manufacturer_alarm_info() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// AC Wiring Type
    ///
    /// AC wiring type.
    fn ac_wiring_type(&self) -> AcType;

    /// Operating State
    ///
    /// Operating state of the DER.
    fn operating_state(&self) -> Option<St> {
        None
    }

    /// Inverter State
    ///
    /// Inverter state.
    fn inverter_state(&self) -> Option<InvSt> {
        None
    }

    /// Grid Connection State
    ///
    /// Grid connection state of the DER.
    fn grid_connection_state(&self) -> Option<ConnSt> {
        None
    }

    /// Alarm Bitfield
    ///
    /// Active alarms for the DER.
    fn alarm_bitfield(&self) -> Option<u32> {
        None
    }

    /// DER Operational Characteristics
    ///
    /// Current operational characteristics of the DER.
    fn der_operational_characteristics(&self) -> Option<u32> {
        None
    }

    /// Active Power
    ///
    /// Total active power. Active power is positive for DER generation and negative for absorption.
    fn active_power(&self) -> Option<i16> {
        None
    }

    /// Apparent Power
    ///
    /// Total apparent power.
    fn apparent_power(&self) -> Option<i16> {
        None
    }

    /// Reactive Power
    ///
    /// Total reactive power.
    fn reactive_power(&self) -> Option<i16> {
        None
    }

    /// Power Factor
    ///
    /// Power factor. The sign of power factor should be the sign of active power.
    fn power_factor(&self) -> Option<i16> {
        None
    }

    /// Total AC Current
    ///
    /// Total AC current.
    fn total_ac_current(&self) -> Option<i16> {
        None
    }

    /// Voltage LL
    ///
    /// Line to line AC voltage as an average of active phases.
    fn voltage_ll(&self) -> Option<u16> {
        None
    }

    /// Voltage LN
    ///
    /// Line to neutral AC voltage as an average of active phases.
    fn voltage_ln(&self) -> Option<u16> {
        None
    }

    /// Frequency
    ///
    /// AC frequency.
    fn frequency(&self) -> Option<u32> {
        None
    }

    /// Total Energy Injected
    ///
    /// Total active energy injected (Quadrants 1 & 4).
    fn total_energy_injected(&self) -> Option<u64> {
        None
    }

    /// Total Energy Absorbed
    ///
    /// Total active energy absorbed (Quadrants 2 & 3).
    fn total_energy_absorbed(&self) -> Option<u64> {
        None
    }

    /// Total Reactive Energy Inj
    ///
    /// Total reactive energy injected (Quadrants 1 & 2).
    fn total_reactive_energy_inj(&self) -> Option<u64> {
        None
    }

    /// Total Reactive Energy Abs
    ///
    /// Total reactive energy absorbed (Quadrants 3 & 4).
    fn total_reactive_energy_abs(&self) -> Option<u64> {
        None
    }

    /// Ambient Temperature
    ///
    /// Ambient temperature.
    fn ambient_temperature(&self) -> Option<i16> {
        None
    }

    /// Cabinet Temperature
    ///
    /// Cabinet temperature.
    fn cabinet_temperature(&self) -> Option<i16> {
        None
    }

    /// Heat Sink Temperature
    ///
    /// Heat sink temperature.
    fn heat_sink_temperature(&self) -> Option<i16> {
        None
    }

    /// Transformer Temperature
    ///
    /// Transformer temperature.
    fn transformer_temperature(&self) -> Option<i16> {
        None
    }

    /// IGBT/MOSFET Temperature
    ///
    /// IGBT/MOSFET temperature.
    fn igbt_mosfet_temperature(&self) -> Option<i16> {
        None
    }

    /// Other Temperature
    ///
    /// Other temperature.
    fn other_temperature(&self) -> Option<i16> {
        None
    }

    /// Watts L1
    ///
    /// Active power L1.
    fn watts_l1(&self) -> Option<i16> {
        None
    }

    /// VA L1
    ///
    /// Apparent power L1.
    fn va_l1(&self) -> Option<i16> {
        None
    }

    /// Var L1
    ///
    /// Reactive power L1.
    fn var_l1(&self) -> Option<i16> {
        None
    }

    /// PF L1
    ///
    /// Power factor phase L1.
    fn pf_l1(&self) -> Option<i16> {
        None
    }

    /// Amps L1
    ///
    /// Current phase L1.
    fn amps_l1(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage L1-L2
    ///
    /// Phase voltage L1-L2.
    fn phase_voltage_l1_l2(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage L1-N
    ///
    /// Phase voltage L1-N.
    fn phase_voltage_l1_n(&self) -> Option<u16> {
        None
    }

    /// Total Watt-Hours Inj L1
    ///
    /// Total active energy injected L1.
    fn total_watt_hours_inj_l1(&self) -> Option<u64> {
        None
    }

    /// Total Watt-Hours Abs L1
    ///
    /// Total active energy absorbed L1.
    fn total_watt_hours_abs_l1(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Inj L1
    ///
    /// Total reactive energy injected L1.
    fn total_var_hours_inj_l1(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Abs L1
    ///
    /// Total reactive energy absorbed L1.
    fn total_var_hours_abs_l1(&self) -> Option<u64> {
        None
    }

    /// Watts L2
    ///
    /// Active power L2.
    fn watts_l2(&self) -> Option<i16> {
        None
    }

    /// VA L2
    ///
    /// Apparent power L2.
    fn va_l2(&self) -> Option<i16> {
        None
    }

    /// Var L2
    ///
    /// Reactive power L2.
    fn var_l2(&self) -> Option<i16> {
        None
    }

    /// PF L2
    ///
    /// Power factor L2.
    fn pf_l2(&self) -> Option<i16> {
        None
    }

    /// Amps L2
    ///
    /// Current L2.
    fn amps_l2(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage L2-L3
    ///
    /// Phase voltage L2-L3.
    fn phase_voltage_l2_l3(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage L2-N
    ///
    /// Phase voltage L2-N.
    fn phase_voltage_l2_n(&self) -> Option<u16> {
        None
    }

    /// Total Watt-Hours Inj L2
    ///
    /// Total active energy injected L2.
    fn total_watt_hours_inj_l2(&self) -> Option<u64> {
        None
    }

    /// Total Watt-Hours Abs L2
    ///
    /// Total active energy absorbed L2.
    fn total_watt_hours_abs_l2(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Inj L2
    ///
    /// Total reactive energy injected L2.
    fn total_var_hours_inj_l2(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Abs L2
    ///
    /// Total reactive energy absorbed L2.
    fn total_var_hours_abs_l2(&self) -> Option<u64> {
        None
    }

    /// Watts L3
    ///
    /// Active power L3.
    fn watts_l3(&self) -> Option<i16> {
        None
    }

    /// VA L3
    ///
    /// Apparent power L3.
    fn va_l3(&self) -> Option<i16> {
        None
    }

    /// Var L3
    ///
    /// Reactive power L3.
    fn var_l3(&self) -> Option<i16> {
        None
    }

    /// PF L3
    ///
    /// Power factor L3.
    fn pf_l3(&self) -> Option<i16> {
        None
    }

    /// Amps L3
    ///
    /// Current L3.
    fn amps_l3(&self) -> Option<i16> {
        None
    }

    /// Phase Voltage L3-L1
    ///
    /// Phase voltage L3-L1.
    fn phase_voltage_l3_l1(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage L3-N
    ///
    /// Phase voltage L3-N.
    fn phase_voltage_l3_n(&self) -> Option<u16> {
        None
    }

    /// Total Watt-Hours Inj L3
    ///
    /// Total active energy injected L3.
    fn total_watt_hours_inj_l3(&self) -> Option<u64> {
        None
    }

    /// Total Watt-Hours Abs L3
    ///
    /// Total active energy absorbed L3.
    fn total_watt_hours_abs_l3(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Inj L3
    ///
    /// Total reactive energy injected L3.
    fn total_var_hours_inj_l3(&self) -> Option<u64> {
        None
    }

    /// Total Var-Hours Abs L3
    ///
    /// Total reactive energy absorbed L3.
    fn total_var_hours_abs_l3(&self) -> Option<u64> {
        None
    }

    /// Throttling In Pct
    ///
    /// Throttling in pct of maximum active power.
    fn throttling_in_pct(&self) -> Option<u16> {
        None
    }

    /// Throttle Source Information
    ///
    /// Active throttling source.
    fn throttle_source_information(&self) -> Option<u32> {
        None
    }

    /// Current Scale Factor
    ///
    /// Current scale factor.
    fn current_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    fn voltage_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn frequency_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn active_power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn power_factor_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    fn apparent_power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn reactive_power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Active Energy Scale Factor
    ///
    /// Active energy scale factor.
    fn active_energy_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Reactive Energy Scale Factor
    ///
    /// Reactive energy scale factor.
    fn reactive_energy_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Temperature Scale Factor
    ///
    /// Temperature scale factor.
    fn temperature_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Manufacturer Alarm Info
    ///
    /// Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.
    fn manufacturer_alarm_info(&self) -> Option<&CStr> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum AcType {
    /// Single Phase
    SinglePhase = 0,
    /// Split Phase
    SplitPhase = 1,
    /// Three Phase
    ThreePhase = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ConnSt {
    /// Disconnected
    ///
    /// Disconnected from the grid.
    Disconnected = 0,
    /// Connected
    ///
    /// Connected to the grid.
    Connected = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum InvSt {
    Off = 0,
    Sleeping = 1,
    Starting = 2,
    Running = 3,
    Throttled = 4,
    ShuttingDown = 5,
    Fault = 6,
    Standby = 7,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum St {
    /// Off
    Off = 0,
    /// On
    On = 1,
}

#[repr(C)]
pub struct Model701CallbackAdapter {
    context: *mut c_void,
    ac_wiring_type_callback: extern "C" fn(*const c_void) -> AcType,
    operating_state_callback: Option<extern "C" fn(*const c_void) -> St>,
    inverter_state_callback: Option<extern "C" fn(*const c_void) -> InvSt>,
    grid_connection_state_callback: Option<extern "C" fn(*const c_void) -> ConnSt>,
    alarm_bitfield_callback: Option<extern "C" fn(*const c_void) -> u32>,
    der_operational_characteristics_callback: Option<extern "C" fn(*const c_void) -> u32>,
    active_power_callback: Option<extern "C" fn(*const c_void) -> i16>,
    apparent_power_callback: Option<extern "C" fn(*const c_void) -> i16>,
    reactive_power_callback: Option<extern "C" fn(*const c_void) -> i16>,
    power_factor_callback: Option<extern "C" fn(*const c_void) -> i16>,
    total_ac_current_callback: Option<extern "C" fn(*const c_void) -> i16>,
    voltage_ll_callback: Option<extern "C" fn(*const c_void) -> u16>,
    voltage_ln_callback: Option<extern "C" fn(*const c_void) -> u16>,
    frequency_callback: Option<extern "C" fn(*const c_void) -> u32>,
    total_energy_injected_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_energy_absorbed_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_reactive_energy_inj_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_reactive_energy_abs_callback: Option<extern "C" fn(*const c_void) -> u64>,
    ambient_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    cabinet_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    heat_sink_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    transformer_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    igbt_mosfet_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    other_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    watts_l1_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_l1_callback: Option<extern "C" fn(*const c_void) -> i16>,
    var_l1_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_l1_callback: Option<extern "C" fn(*const c_void) -> i16>,
    amps_l1_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_l1_l2_callback: Option<extern "C" fn(*const c_void) -> u16>,
    phase_voltage_l1_n_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_watt_hours_inj_l1_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_watt_hours_abs_l1_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_var_hours_inj_l1_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_var_hours_abs_l1_callback: Option<extern "C" fn(*const c_void) -> u64>,
    watts_l2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_l2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    var_l2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_l2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    amps_l2_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_l2_l3_callback: Option<extern "C" fn(*const c_void) -> u16>,
    phase_voltage_l2_n_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_watt_hours_inj_l2_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_watt_hours_abs_l2_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_var_hours_inj_l2_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_var_hours_abs_l2_callback: Option<extern "C" fn(*const c_void) -> u64>,
    watts_l3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_l3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    var_l3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_l3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    amps_l3_callback: Option<extern "C" fn(*const c_void) -> i16>,
    phase_voltage_l3_l1_callback: Option<extern "C" fn(*const c_void) -> u16>,
    phase_voltage_l3_n_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_watt_hours_inj_l3_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_watt_hours_abs_l3_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_var_hours_inj_l3_callback: Option<extern "C" fn(*const c_void) -> u64>,
    total_var_hours_abs_l3_callback: Option<extern "C" fn(*const c_void) -> u64>,
    throttling_in_pct_callback: Option<extern "C" fn(*const c_void) -> u16>,
    throttle_source_information_callback: Option<extern "C" fn(*const c_void) -> u32>,
    current_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    voltage_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    frequency_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    active_power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    power_factor_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    apparent_power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    reactive_power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    active_energy_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    reactive_energy_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    temperature_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    manufacturer_alarm_info_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
}

impl ModelAdapter for Model701CallbackAdapter {
    /// AC Wiring Type
    ///
    /// AC wiring type.
    fn ac_wiring_type(&self) -> AcType {
        (self.ac_wiring_type_callback)(self.context)
    }

    /// Operating State
    ///
    /// Operating state of the DER.
    fn operating_state(&self) -> Option<St> {
        self.operating_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Inverter State
    ///
    /// Inverter state.
    fn inverter_state(&self) -> Option<InvSt> {
        self.inverter_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Grid Connection State
    ///
    /// Grid connection state of the DER.
    fn grid_connection_state(&self) -> Option<ConnSt> {
        self.grid_connection_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Alarm Bitfield
    ///
    /// Active alarms for the DER.
    fn alarm_bitfield(&self) -> Option<u32> {
        self.alarm_bitfield_callback
            .map(|callback| (callback)(self.context))
    }

    /// DER Operational Characteristics
    ///
    /// Current operational characteristics of the DER.
    fn der_operational_characteristics(&self) -> Option<u32> {
        self.der_operational_characteristics_callback
            .map(|callback| (callback)(self.context))
    }

    /// Active Power
    ///
    /// Total active power. Active power is positive for DER generation and negative for absorption.
    fn active_power(&self) -> Option<i16> {
        self.active_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// Apparent Power
    ///
    /// Total apparent power.
    fn apparent_power(&self) -> Option<i16> {
        self.apparent_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reactive Power
    ///
    /// Total reactive power.
    fn reactive_power(&self) -> Option<i16> {
        self.reactive_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power Factor
    ///
    /// Power factor. The sign of power factor should be the sign of active power.
    fn power_factor(&self) -> Option<i16> {
        self.power_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total AC Current
    ///
    /// Total AC current.
    fn total_ac_current(&self) -> Option<i16> {
        self.total_ac_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage LL
    ///
    /// Line to line AC voltage as an average of active phases.
    fn voltage_ll(&self) -> Option<u16> {
        self.voltage_ll_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage LN
    ///
    /// Line to neutral AC voltage as an average of active phases.
    fn voltage_ln(&self) -> Option<u16> {
        self.voltage_ln_callback
            .map(|callback| (callback)(self.context))
    }

    /// Frequency
    ///
    /// AC frequency.
    fn frequency(&self) -> Option<u32> {
        self.frequency_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Energy Injected
    ///
    /// Total active energy injected (Quadrants 1 & 4).
    fn total_energy_injected(&self) -> Option<u64> {
        self.total_energy_injected_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Energy Absorbed
    ///
    /// Total active energy absorbed (Quadrants 2 & 3).
    fn total_energy_absorbed(&self) -> Option<u64> {
        self.total_energy_absorbed_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Reactive Energy Inj
    ///
    /// Total reactive energy injected (Quadrants 1 & 2).
    fn total_reactive_energy_inj(&self) -> Option<u64> {
        self.total_reactive_energy_inj_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Reactive Energy Abs
    ///
    /// Total reactive energy absorbed (Quadrants 3 & 4).
    fn total_reactive_energy_abs(&self) -> Option<u64> {
        self.total_reactive_energy_abs_callback
            .map(|callback| (callback)(self.context))
    }

    /// Ambient Temperature
    ///
    /// Ambient temperature.
    fn ambient_temperature(&self) -> Option<i16> {
        self.ambient_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Cabinet Temperature
    ///
    /// Cabinet temperature.
    fn cabinet_temperature(&self) -> Option<i16> {
        self.cabinet_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Heat Sink Temperature
    ///
    /// Heat sink temperature.
    fn heat_sink_temperature(&self) -> Option<i16> {
        self.heat_sink_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Transformer Temperature
    ///
    /// Transformer temperature.
    fn transformer_temperature(&self) -> Option<i16> {
        self.transformer_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// IGBT/MOSFET Temperature
    ///
    /// IGBT/MOSFET temperature.
    fn igbt_mosfet_temperature(&self) -> Option<i16> {
        self.igbt_mosfet_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Other Temperature
    ///
    /// Other temperature.
    fn other_temperature(&self) -> Option<i16> {
        self.other_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts L1
    ///
    /// Active power L1.
    fn watts_l1(&self) -> Option<i16> {
        self.watts_l1_callback
            .map(|callback| (callback)(self.context))
    }

    /// VA L1
    ///
    /// Apparent power L1.
    fn va_l1(&self) -> Option<i16> {
        self.va_l1_callback.map(|callback| (callback)(self.context))
    }

    /// Var L1
    ///
    /// Reactive power L1.
    fn var_l1(&self) -> Option<i16> {
        self.var_l1_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF L1
    ///
    /// Power factor phase L1.
    fn pf_l1(&self) -> Option<i16> {
        self.pf_l1_callback.map(|callback| (callback)(self.context))
    }

    /// Amps L1
    ///
    /// Current phase L1.
    fn amps_l1(&self) -> Option<i16> {
        self.amps_l1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage L1-L2
    ///
    /// Phase voltage L1-L2.
    fn phase_voltage_l1_l2(&self) -> Option<u16> {
        self.phase_voltage_l1_l2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage L1-N
    ///
    /// Phase voltage L1-N.
    fn phase_voltage_l1_n(&self) -> Option<u16> {
        self.phase_voltage_l1_n_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-Hours Inj L1
    ///
    /// Total active energy injected L1.
    fn total_watt_hours_inj_l1(&self) -> Option<u64> {
        self.total_watt_hours_inj_l1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-Hours Abs L1
    ///
    /// Total active energy absorbed L1.
    fn total_watt_hours_abs_l1(&self) -> Option<u64> {
        self.total_watt_hours_abs_l1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Var-Hours Inj L1
    ///
    /// Total reactive energy injected L1.
    fn total_var_hours_inj_l1(&self) -> Option<u64> {
        self.total_var_hours_inj_l1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Var-Hours Abs L1
    ///
    /// Total reactive energy absorbed L1.
    fn total_var_hours_abs_l1(&self) -> Option<u64> {
        self.total_var_hours_abs_l1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts L2
    ///
    /// Active power L2.
    fn watts_l2(&self) -> Option<i16> {
        self.watts_l2_callback
            .map(|callback| (callback)(self.context))
    }

    /// VA L2
    ///
    /// Apparent power L2.
    fn va_l2(&self) -> Option<i16> {
        self.va_l2_callback.map(|callback| (callback)(self.context))
    }

    /// Var L2
    ///
    /// Reactive power L2.
    fn var_l2(&self) -> Option<i16> {
        self.var_l2_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF L2
    ///
    /// Power factor L2.
    fn pf_l2(&self) -> Option<i16> {
        self.pf_l2_callback.map(|callback| (callback)(self.context))
    }

    /// Amps L2
    ///
    /// Current L2.
    fn amps_l2(&self) -> Option<i16> {
        self.amps_l2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage L2-L3
    ///
    /// Phase voltage L2-L3.
    fn phase_voltage_l2_l3(&self) -> Option<u16> {
        self.phase_voltage_l2_l3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage L2-N
    ///
    /// Phase voltage L2-N.
    fn phase_voltage_l2_n(&self) -> Option<u16> {
        self.phase_voltage_l2_n_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-Hours Inj L2
    ///
    /// Total active energy injected L2.
    fn total_watt_hours_inj_l2(&self) -> Option<u64> {
        self.total_watt_hours_inj_l2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-Hours Abs L2
    ///
    /// Total active energy absorbed L2.
    fn total_watt_hours_abs_l2(&self) -> Option<u64> {
        self.total_watt_hours_abs_l2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Var-Hours Inj L2
    ///
    /// Total reactive energy injected L2.
    fn total_var_hours_inj_l2(&self) -> Option<u64> {
        self.total_var_hours_inj_l2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Var-Hours Abs L2
    ///
    /// Total reactive energy absorbed L2.
    fn total_var_hours_abs_l2(&self) -> Option<u64> {
        self.total_var_hours_abs_l2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts L3
    ///
    /// Active power L3.
    fn watts_l3(&self) -> Option<i16> {
        self.watts_l3_callback
            .map(|callback| (callback)(self.context))
    }

    /// VA L3
    ///
    /// Apparent power L3.
    fn va_l3(&self) -> Option<i16> {
        self.va_l3_callback.map(|callback| (callback)(self.context))
    }

    /// Var L3
    ///
    /// Reactive power L3.
    fn var_l3(&self) -> Option<i16> {
        self.var_l3_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF L3
    ///
    /// Power factor L3.
    fn pf_l3(&self) -> Option<i16> {
        self.pf_l3_callback.map(|callback| (callback)(self.context))
    }

    /// Amps L3
    ///
    /// Current L3.
    fn amps_l3(&self) -> Option<i16> {
        self.amps_l3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage L3-L1
    ///
    /// Phase voltage L3-L1.
    fn phase_voltage_l3_l1(&self) -> Option<u16> {
        self.phase_voltage_l3_l1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage L3-N
    ///
    /// Phase voltage L3-N.
    fn phase_voltage_l3_n(&self) -> Option<u16> {
        self.phase_voltage_l3_n_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-Hours Inj L3
    ///
    /// Total active energy injected L3.
    fn total_watt_hours_inj_l3(&self) -> Option<u64> {
        self.total_watt_hours_inj_l3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Watt-Hours Abs L3
    ///
    /// Total active energy absorbed L3.
    fn total_watt_hours_abs_l3(&self) -> Option<u64> {
        self.total_watt_hours_abs_l3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Var-Hours Inj L3
    ///
    /// Total reactive energy injected L3.
    fn total_var_hours_inj_l3(&self) -> Option<u64> {
        self.total_var_hours_inj_l3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Var-Hours Abs L3
    ///
    /// Total reactive energy absorbed L3.
    fn total_var_hours_abs_l3(&self) -> Option<u64> {
        self.total_var_hours_abs_l3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Throttling In Pct
    ///
    /// Throttling in pct of maximum active power.
    fn throttling_in_pct(&self) -> Option<u16> {
        self.throttling_in_pct_callback
            .map(|callback| (callback)(self.context))
    }

    /// Throttle Source Information
    ///
    /// Active throttling source.
    fn throttle_source_information(&self) -> Option<u32> {
        self.throttle_source_information_callback
            .map(|callback| (callback)(self.context))
    }

    /// Current Scale Factor
    ///
    /// Current scale factor.
    fn current_scale_factor(&self) -> Option<u16> {
        self.current_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    fn voltage_scale_factor(&self) -> Option<u16> {
        self.voltage_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn frequency_scale_factor(&self) -> Option<u16> {
        self.frequency_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn active_power_scale_factor(&self) -> Option<u16> {
        self.active_power_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn power_factor_scale_factor(&self) -> Option<u16> {
        self.power_factor_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    fn apparent_power_scale_factor(&self) -> Option<u16> {
        self.apparent_power_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn reactive_power_scale_factor(&self) -> Option<u16> {
        self.reactive_power_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Active Energy Scale Factor
    ///
    /// Active energy scale factor.
    fn active_energy_scale_factor(&self) -> Option<u16> {
        self.active_energy_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reactive Energy Scale Factor
    ///
    /// Reactive energy scale factor.
    fn reactive_energy_scale_factor(&self) -> Option<u16> {
        self.reactive_energy_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Temperature Scale Factor
    ///
    /// Temperature scale factor.
    fn temperature_scale_factor(&self) -> Option<u16> {
        self.temperature_scale_factor_callback
            .map(|callback| (callback)(self.context))
    }

    /// Manufacturer Alarm Info
    ///
    /// Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.
    fn manufacturer_alarm_info(&self) -> Option<&CStr> {
        self.manufacturer_alarm_info_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }
}

#[repr(C)]
pub struct Model701StatefulAdapter {
    ac_wiring_type: AcType,
    operating_state: St,
    inverter_state: InvSt,
    grid_connection_state: ConnSt,
    alarm_bitfield: u32,
    der_operational_characteristics: u32,
    active_power: i16,
    apparent_power: i16,
    reactive_power: i16,
    power_factor: i16,
    total_ac_current: i16,
    voltage_ll: u16,
    voltage_ln: u16,
    frequency: u32,
    total_energy_injected: u64,
    total_energy_absorbed: u64,
    total_reactive_energy_inj: u64,
    total_reactive_energy_abs: u64,
    ambient_temperature: i16,
    cabinet_temperature: i16,
    heat_sink_temperature: i16,
    transformer_temperature: i16,
    igbt_mosfet_temperature: i16,
    other_temperature: i16,
    watts_l1: i16,
    va_l1: i16,
    var_l1: i16,
    pf_l1: i16,
    amps_l1: i16,
    phase_voltage_l1_l2: u16,
    phase_voltage_l1_n: u16,
    total_watt_hours_inj_l1: u64,
    total_watt_hours_abs_l1: u64,
    total_var_hours_inj_l1: u64,
    total_var_hours_abs_l1: u64,
    watts_l2: i16,
    va_l2: i16,
    var_l2: i16,
    pf_l2: i16,
    amps_l2: i16,
    phase_voltage_l2_l3: u16,
    phase_voltage_l2_n: u16,
    total_watt_hours_inj_l2: u64,
    total_watt_hours_abs_l2: u64,
    total_var_hours_inj_l2: u64,
    total_var_hours_abs_l2: u64,
    watts_l3: i16,
    va_l3: i16,
    var_l3: i16,
    pf_l3: i16,
    amps_l3: i16,
    phase_voltage_l3_l1: u16,
    phase_voltage_l3_n: u16,
    total_watt_hours_inj_l3: u64,
    total_watt_hours_abs_l3: u64,
    total_var_hours_inj_l3: u64,
    total_var_hours_abs_l3: u64,
    throttling_in_pct: u16,
    throttle_source_information: u32,
    current_scale_factor: u16,
    voltage_scale_factor: u16,
    frequency_scale_factor: u16,
    active_power_scale_factor: u16,
    power_factor_scale_factor: u16,
    apparent_power_scale_factor: u16,
    reactive_power_scale_factor: u16,
    active_energy_scale_factor: u16,
    reactive_energy_scale_factor: u16,
    temperature_scale_factor: u16,
    manufacturer_alarm_info: [c_char; 64],
}

impl ModelAdapter for Model701StatefulAdapter {
    /// AC Wiring Type
    ///
    /// AC wiring type.
    fn ac_wiring_type(&self) -> AcType {
        self.ac_wiring_type
    }

    /// Operating State
    ///
    /// Operating state of the DER.
    fn operating_state(&self) -> Option<St> {
        Some(self.operating_state)
    }

    /// Inverter State
    ///
    /// Inverter state.
    fn inverter_state(&self) -> Option<InvSt> {
        Some(self.inverter_state)
    }

    /// Grid Connection State
    ///
    /// Grid connection state of the DER.
    fn grid_connection_state(&self) -> Option<ConnSt> {
        Some(self.grid_connection_state)
    }

    /// Alarm Bitfield
    ///
    /// Active alarms for the DER.
    fn alarm_bitfield(&self) -> Option<u32> {
        Some(self.alarm_bitfield)
    }

    /// DER Operational Characteristics
    ///
    /// Current operational characteristics of the DER.
    fn der_operational_characteristics(&self) -> Option<u32> {
        Some(self.der_operational_characteristics)
    }

    /// Active Power
    ///
    /// Total active power. Active power is positive for DER generation and negative for absorption.
    fn active_power(&self) -> Option<i16> {
        Some(self.active_power)
    }

    /// Apparent Power
    ///
    /// Total apparent power.
    fn apparent_power(&self) -> Option<i16> {
        Some(self.apparent_power)
    }

    /// Reactive Power
    ///
    /// Total reactive power.
    fn reactive_power(&self) -> Option<i16> {
        Some(self.reactive_power)
    }

    /// Power Factor
    ///
    /// Power factor. The sign of power factor should be the sign of active power.
    fn power_factor(&self) -> Option<i16> {
        Some(self.power_factor)
    }

    /// Total AC Current
    ///
    /// Total AC current.
    fn total_ac_current(&self) -> Option<i16> {
        Some(self.total_ac_current)
    }

    /// Voltage LL
    ///
    /// Line to line AC voltage as an average of active phases.
    fn voltage_ll(&self) -> Option<u16> {
        Some(self.voltage_ll)
    }

    /// Voltage LN
    ///
    /// Line to neutral AC voltage as an average of active phases.
    fn voltage_ln(&self) -> Option<u16> {
        Some(self.voltage_ln)
    }

    /// Frequency
    ///
    /// AC frequency.
    fn frequency(&self) -> Option<u32> {
        Some(self.frequency)
    }

    /// Total Energy Injected
    ///
    /// Total active energy injected (Quadrants 1 & 4).
    fn total_energy_injected(&self) -> Option<u64> {
        Some(self.total_energy_injected)
    }

    /// Total Energy Absorbed
    ///
    /// Total active energy absorbed (Quadrants 2 & 3).
    fn total_energy_absorbed(&self) -> Option<u64> {
        Some(self.total_energy_absorbed)
    }

    /// Total Reactive Energy Inj
    ///
    /// Total reactive energy injected (Quadrants 1 & 2).
    fn total_reactive_energy_inj(&self) -> Option<u64> {
        Some(self.total_reactive_energy_inj)
    }

    /// Total Reactive Energy Abs
    ///
    /// Total reactive energy absorbed (Quadrants 3 & 4).
    fn total_reactive_energy_abs(&self) -> Option<u64> {
        Some(self.total_reactive_energy_abs)
    }

    /// Ambient Temperature
    ///
    /// Ambient temperature.
    fn ambient_temperature(&self) -> Option<i16> {
        Some(self.ambient_temperature)
    }

    /// Cabinet Temperature
    ///
    /// Cabinet temperature.
    fn cabinet_temperature(&self) -> Option<i16> {
        Some(self.cabinet_temperature)
    }

    /// Heat Sink Temperature
    ///
    /// Heat sink temperature.
    fn heat_sink_temperature(&self) -> Option<i16> {
        Some(self.heat_sink_temperature)
    }

    /// Transformer Temperature
    ///
    /// Transformer temperature.
    fn transformer_temperature(&self) -> Option<i16> {
        Some(self.transformer_temperature)
    }

    /// IGBT/MOSFET Temperature
    ///
    /// IGBT/MOSFET temperature.
    fn igbt_mosfet_temperature(&self) -> Option<i16> {
        Some(self.igbt_mosfet_temperature)
    }

    /// Other Temperature
    ///
    /// Other temperature.
    fn other_temperature(&self) -> Option<i16> {
        Some(self.other_temperature)
    }

    /// Watts L1
    ///
    /// Active power L1.
    fn watts_l1(&self) -> Option<i16> {
        Some(self.watts_l1)
    }

    /// VA L1
    ///
    /// Apparent power L1.
    fn va_l1(&self) -> Option<i16> {
        Some(self.va_l1)
    }

    /// Var L1
    ///
    /// Reactive power L1.
    fn var_l1(&self) -> Option<i16> {
        Some(self.var_l1)
    }

    /// PF L1
    ///
    /// Power factor phase L1.
    fn pf_l1(&self) -> Option<i16> {
        Some(self.pf_l1)
    }

    /// Amps L1
    ///
    /// Current phase L1.
    fn amps_l1(&self) -> Option<i16> {
        Some(self.amps_l1)
    }

    /// Phase Voltage L1-L2
    ///
    /// Phase voltage L1-L2.
    fn phase_voltage_l1_l2(&self) -> Option<u16> {
        Some(self.phase_voltage_l1_l2)
    }

    /// Phase Voltage L1-N
    ///
    /// Phase voltage L1-N.
    fn phase_voltage_l1_n(&self) -> Option<u16> {
        Some(self.phase_voltage_l1_n)
    }

    /// Total Watt-Hours Inj L1
    ///
    /// Total active energy injected L1.
    fn total_watt_hours_inj_l1(&self) -> Option<u64> {
        Some(self.total_watt_hours_inj_l1)
    }

    /// Total Watt-Hours Abs L1
    ///
    /// Total active energy absorbed L1.
    fn total_watt_hours_abs_l1(&self) -> Option<u64> {
        Some(self.total_watt_hours_abs_l1)
    }

    /// Total Var-Hours Inj L1
    ///
    /// Total reactive energy injected L1.
    fn total_var_hours_inj_l1(&self) -> Option<u64> {
        Some(self.total_var_hours_inj_l1)
    }

    /// Total Var-Hours Abs L1
    ///
    /// Total reactive energy absorbed L1.
    fn total_var_hours_abs_l1(&self) -> Option<u64> {
        Some(self.total_var_hours_abs_l1)
    }

    /// Watts L2
    ///
    /// Active power L2.
    fn watts_l2(&self) -> Option<i16> {
        Some(self.watts_l2)
    }

    /// VA L2
    ///
    /// Apparent power L2.
    fn va_l2(&self) -> Option<i16> {
        Some(self.va_l2)
    }

    /// Var L2
    ///
    /// Reactive power L2.
    fn var_l2(&self) -> Option<i16> {
        Some(self.var_l2)
    }

    /// PF L2
    ///
    /// Power factor L2.
    fn pf_l2(&self) -> Option<i16> {
        Some(self.pf_l2)
    }

    /// Amps L2
    ///
    /// Current L2.
    fn amps_l2(&self) -> Option<i16> {
        Some(self.amps_l2)
    }

    /// Phase Voltage L2-L3
    ///
    /// Phase voltage L2-L3.
    fn phase_voltage_l2_l3(&self) -> Option<u16> {
        Some(self.phase_voltage_l2_l3)
    }

    /// Phase Voltage L2-N
    ///
    /// Phase voltage L2-N.
    fn phase_voltage_l2_n(&self) -> Option<u16> {
        Some(self.phase_voltage_l2_n)
    }

    /// Total Watt-Hours Inj L2
    ///
    /// Total active energy injected L2.
    fn total_watt_hours_inj_l2(&self) -> Option<u64> {
        Some(self.total_watt_hours_inj_l2)
    }

    /// Total Watt-Hours Abs L2
    ///
    /// Total active energy absorbed L2.
    fn total_watt_hours_abs_l2(&self) -> Option<u64> {
        Some(self.total_watt_hours_abs_l2)
    }

    /// Total Var-Hours Inj L2
    ///
    /// Total reactive energy injected L2.
    fn total_var_hours_inj_l2(&self) -> Option<u64> {
        Some(self.total_var_hours_inj_l2)
    }

    /// Total Var-Hours Abs L2
    ///
    /// Total reactive energy absorbed L2.
    fn total_var_hours_abs_l2(&self) -> Option<u64> {
        Some(self.total_var_hours_abs_l2)
    }

    /// Watts L3
    ///
    /// Active power L3.
    fn watts_l3(&self) -> Option<i16> {
        Some(self.watts_l3)
    }

    /// VA L3
    ///
    /// Apparent power L3.
    fn va_l3(&self) -> Option<i16> {
        Some(self.va_l3)
    }

    /// Var L3
    ///
    /// Reactive power L3.
    fn var_l3(&self) -> Option<i16> {
        Some(self.var_l3)
    }

    /// PF L3
    ///
    /// Power factor L3.
    fn pf_l3(&self) -> Option<i16> {
        Some(self.pf_l3)
    }

    /// Amps L3
    ///
    /// Current L3.
    fn amps_l3(&self) -> Option<i16> {
        Some(self.amps_l3)
    }

    /// Phase Voltage L3-L1
    ///
    /// Phase voltage L3-L1.
    fn phase_voltage_l3_l1(&self) -> Option<u16> {
        Some(self.phase_voltage_l3_l1)
    }

    /// Phase Voltage L3-N
    ///
    /// Phase voltage L3-N.
    fn phase_voltage_l3_n(&self) -> Option<u16> {
        Some(self.phase_voltage_l3_n)
    }

    /// Total Watt-Hours Inj L3
    ///
    /// Total active energy injected L3.
    fn total_watt_hours_inj_l3(&self) -> Option<u64> {
        Some(self.total_watt_hours_inj_l3)
    }

    /// Total Watt-Hours Abs L3
    ///
    /// Total active energy absorbed L3.
    fn total_watt_hours_abs_l3(&self) -> Option<u64> {
        Some(self.total_watt_hours_abs_l3)
    }

    /// Total Var-Hours Inj L3
    ///
    /// Total reactive energy injected L3.
    fn total_var_hours_inj_l3(&self) -> Option<u64> {
        Some(self.total_var_hours_inj_l3)
    }

    /// Total Var-Hours Abs L3
    ///
    /// Total reactive energy absorbed L3.
    fn total_var_hours_abs_l3(&self) -> Option<u64> {
        Some(self.total_var_hours_abs_l3)
    }

    /// Throttling In Pct
    ///
    /// Throttling in pct of maximum active power.
    fn throttling_in_pct(&self) -> Option<u16> {
        Some(self.throttling_in_pct)
    }

    /// Throttle Source Information
    ///
    /// Active throttling source.
    fn throttle_source_information(&self) -> Option<u32> {
        Some(self.throttle_source_information)
    }

    /// Current Scale Factor
    ///
    /// Current scale factor.
    fn current_scale_factor(&self) -> Option<u16> {
        Some(self.current_scale_factor)
    }

    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    fn voltage_scale_factor(&self) -> Option<u16> {
        Some(self.voltage_scale_factor)
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn frequency_scale_factor(&self) -> Option<u16> {
        Some(self.frequency_scale_factor)
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn active_power_scale_factor(&self) -> Option<u16> {
        Some(self.active_power_scale_factor)
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn power_factor_scale_factor(&self) -> Option<u16> {
        Some(self.power_factor_scale_factor)
    }

    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    fn apparent_power_scale_factor(&self) -> Option<u16> {
        Some(self.apparent_power_scale_factor)
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn reactive_power_scale_factor(&self) -> Option<u16> {
        Some(self.reactive_power_scale_factor)
    }

    /// Active Energy Scale Factor
    ///
    /// Active energy scale factor.
    fn active_energy_scale_factor(&self) -> Option<u16> {
        Some(self.active_energy_scale_factor)
    }

    /// Reactive Energy Scale Factor
    ///
    /// Reactive energy scale factor.
    fn reactive_energy_scale_factor(&self) -> Option<u16> {
        Some(self.reactive_energy_scale_factor)
    }

    /// Temperature Scale Factor
    ///
    /// Temperature scale factor.
    fn temperature_scale_factor(&self) -> Option<u16> {
        Some(self.temperature_scale_factor)
    }

    /// Manufacturer Alarm Info
    ///
    /// Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.
    fn manufacturer_alarm_info(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.manufacturer_alarm_info.as_ptr()) })
    }
}
