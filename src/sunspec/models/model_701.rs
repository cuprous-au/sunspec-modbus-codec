use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::AcWiringType => serialisation::write_u16(model.ac_wiring_type() as u16, buffer),
        Point::OperatingState => {
            if let Some(value) = model.operating_state() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::InverterState => {
            if let Some(value) = model.inverter_state() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::GridConnectionState => {
            if let Some(value) = model.grid_connection_state() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::AlarmBitfield => {
            if let Some(value) = model.alarm_bitfield() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::DerOperationalCharacteristics => {
            if let Some(value) = model.der_operational_characteristics() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ActivePower => {
            if let Some(value) = model.active_power() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::ApparentPower => {
            if let Some(value) = model.apparent_power() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::ReactivePower => {
            if let Some(value) = model.reactive_power() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PowerFactor => {
            if let Some(value) = model.power_factor() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::TotalAcCurrent => {
            if let Some(value) = model.total_ac_current() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VoltageLl => {
            if let Some(value) = model.voltage_ll() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageLn => {
            if let Some(value) = model.voltage_ln() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Frequency => {
            if let Some(value) = model.frequency() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::TotalEnergyInjected => {
            if let Some(value) = model.total_energy_injected() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalEnergyAbsorbed => {
            if let Some(value) = model.total_energy_absorbed() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalReactiveEnergyInj => {
            if let Some(value) = model.total_reactive_energy_inj() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalReactiveEnergyAbs => {
            if let Some(value) = model.total_reactive_energy_abs() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::AmbientTemperature => {
            if let Some(value) = model.ambient_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::CabinetTemperature => {
            if let Some(value) = model.cabinet_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::HeatSinkTemperature => {
            if let Some(value) = model.heat_sink_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::TransformerTemperature => {
            if let Some(value) = model.transformer_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::IgbtMosfetTemperature => {
            if let Some(value) = model.igbt_mosfet_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::OtherTemperature => {
            if let Some(value) = model.other_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::WattsL1 => {
            if let Some(value) = model.watts_l1() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VaL1 => {
            if let Some(value) = model.va_l1() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VarL1 => {
            if let Some(value) = model.var_l1() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfL1 => {
            if let Some(value) = model.pf_l1() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::AmpsL1 => {
            if let Some(value) = model.amps_l1() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PhaseVoltageL1L2 => {
            if let Some(value) = model.phase_voltage_l1_l2() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PhaseVoltageL1N => {
            if let Some(value) = model.phase_voltage_l1_n() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TotalWattHoursInjL1 => {
            if let Some(value) = model.total_watt_hours_inj_l1() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursAbsL1 => {
            if let Some(value) = model.total_watt_hours_abs_l1() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalVarHoursInjL1 => {
            if let Some(value) = model.total_var_hours_inj_l1() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalVarHoursAbsL1 => {
            if let Some(value) = model.total_var_hours_abs_l1() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::WattsL2 => {
            if let Some(value) = model.watts_l2() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VaL2 => {
            if let Some(value) = model.va_l2() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VarL2 => {
            if let Some(value) = model.var_l2() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfL2 => {
            if let Some(value) = model.pf_l2() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::AmpsL2 => {
            if let Some(value) = model.amps_l2() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PhaseVoltageL2L3 => {
            if let Some(value) = model.phase_voltage_l2_l3() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PhaseVoltageL2N => {
            if let Some(value) = model.phase_voltage_l2_n() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TotalWattHoursInjL2 => {
            if let Some(value) = model.total_watt_hours_inj_l2() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursAbsL2 => {
            if let Some(value) = model.total_watt_hours_abs_l2() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalVarHoursInjL2 => {
            if let Some(value) = model.total_var_hours_inj_l2() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalVarHoursAbsL2 => {
            if let Some(value) = model.total_var_hours_abs_l2() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::WattsL3 => {
            if let Some(value) = model.watts_l3() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VaL3 => {
            if let Some(value) = model.va_l3() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VarL3 => {
            if let Some(value) = model.var_l3() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfL3 => {
            if let Some(value) = model.pf_l3() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::AmpsL3 => {
            if let Some(value) = model.amps_l3() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PhaseVoltageL3L1 => {
            if let Some(value) = model.phase_voltage_l3_l1() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PhaseVoltageL3N => {
            if let Some(value) = model.phase_voltage_l3_n() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TotalWattHoursInjL3 => {
            if let Some(value) = model.total_watt_hours_inj_l3() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalWattHoursAbsL3 => {
            if let Some(value) = model.total_watt_hours_abs_l3() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalVarHoursInjL3 => {
            if let Some(value) = model.total_var_hours_inj_l3() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::TotalVarHoursAbsL3 => {
            if let Some(value) = model.total_var_hours_abs_l3() {
                serialisation::write_u64(value, buffer, offset, limit);
            }
        }
        Point::ThrottlingInPct => {
            if let Some(value) = model.throttling_in_pct() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ThrottleSourceInformation => {
            if let Some(value) = model.throttle_source_information() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::CurrentScaleFactor => {
            if let Some(value) = model.current_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageScaleFactor => {
            if let Some(value) = model.voltage_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::FrequencyScaleFactor => {
            if let Some(value) = model.frequency_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ActivePowerScaleFactor => {
            if let Some(value) = model.active_power_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PowerFactorScaleFactor => {
            if let Some(value) = model.power_factor_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ApparentPowerScaleFactor => {
            if let Some(value) = model.apparent_power_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ReactivePowerScaleFactor => {
            if let Some(value) = model.reactive_power_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ActiveEnergyScaleFactor => {
            if let Some(value) = model.active_energy_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ReactiveEnergyScaleFactor => {
            if let Some(value) = model.reactive_energy_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TemperatureScaleFactor => {
            if let Some(value) = model.temperature_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ManufacturerAlarmInfo => {
            if let Some(value) = model.manufacturer_alarm_info() {
                serialisation::write_string(value, buffer, offset, limit);
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

pub enum AcType {
    /// Single Phase
    SinglePhase = 0,
    /// Split Phase
    SplitPhase = 1,
    /// Three Phase
    ThreePhase = 2,
}

pub enum St {
    /// Off
    Off = 0,
    /// On
    On = 1,
}

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

#[repr(C)]
pub struct Model701CallbackAdapter {
    ac_wiring_type_callback: extern "C" fn() -> AcType,
    operating_state_callback: Option<extern "C" fn() -> St>,
    inverter_state_callback: Option<extern "C" fn() -> InvSt>,
    grid_connection_state_callback: Option<extern "C" fn() -> ConnSt>,
    alarm_bitfield_callback: Option<extern "C" fn() -> u32>,
    der_operational_characteristics_callback: Option<extern "C" fn() -> u32>,
    active_power_callback: Option<extern "C" fn() -> i16>,
    apparent_power_callback: Option<extern "C" fn() -> i16>,
    reactive_power_callback: Option<extern "C" fn() -> i16>,
    power_factor_callback: Option<extern "C" fn() -> i16>,
    total_ac_current_callback: Option<extern "C" fn() -> i16>,
    voltage_ll_callback: Option<extern "C" fn() -> u16>,
    voltage_ln_callback: Option<extern "C" fn() -> u16>,
    frequency_callback: Option<extern "C" fn() -> u32>,
    total_energy_injected_callback: Option<extern "C" fn() -> u64>,
    total_energy_absorbed_callback: Option<extern "C" fn() -> u64>,
    total_reactive_energy_inj_callback: Option<extern "C" fn() -> u64>,
    total_reactive_energy_abs_callback: Option<extern "C" fn() -> u64>,
    ambient_temperature_callback: Option<extern "C" fn() -> i16>,
    cabinet_temperature_callback: Option<extern "C" fn() -> i16>,
    heat_sink_temperature_callback: Option<extern "C" fn() -> i16>,
    transformer_temperature_callback: Option<extern "C" fn() -> i16>,
    igbt_mosfet_temperature_callback: Option<extern "C" fn() -> i16>,
    other_temperature_callback: Option<extern "C" fn() -> i16>,
    watts_l1_callback: Option<extern "C" fn() -> i16>,
    va_l1_callback: Option<extern "C" fn() -> i16>,
    var_l1_callback: Option<extern "C" fn() -> i16>,
    pf_l1_callback: Option<extern "C" fn() -> i16>,
    amps_l1_callback: Option<extern "C" fn() -> i16>,
    phase_voltage_l1_l2_callback: Option<extern "C" fn() -> u16>,
    phase_voltage_l1_n_callback: Option<extern "C" fn() -> u16>,
    total_watt_hours_inj_l1_callback: Option<extern "C" fn() -> u64>,
    total_watt_hours_abs_l1_callback: Option<extern "C" fn() -> u64>,
    total_var_hours_inj_l1_callback: Option<extern "C" fn() -> u64>,
    total_var_hours_abs_l1_callback: Option<extern "C" fn() -> u64>,
    watts_l2_callback: Option<extern "C" fn() -> i16>,
    va_l2_callback: Option<extern "C" fn() -> i16>,
    var_l2_callback: Option<extern "C" fn() -> i16>,
    pf_l2_callback: Option<extern "C" fn() -> i16>,
    amps_l2_callback: Option<extern "C" fn() -> i16>,
    phase_voltage_l2_l3_callback: Option<extern "C" fn() -> u16>,
    phase_voltage_l2_n_callback: Option<extern "C" fn() -> u16>,
    total_watt_hours_inj_l2_callback: Option<extern "C" fn() -> u64>,
    total_watt_hours_abs_l2_callback: Option<extern "C" fn() -> u64>,
    total_var_hours_inj_l2_callback: Option<extern "C" fn() -> u64>,
    total_var_hours_abs_l2_callback: Option<extern "C" fn() -> u64>,
    watts_l3_callback: Option<extern "C" fn() -> i16>,
    va_l3_callback: Option<extern "C" fn() -> i16>,
    var_l3_callback: Option<extern "C" fn() -> i16>,
    pf_l3_callback: Option<extern "C" fn() -> i16>,
    amps_l3_callback: Option<extern "C" fn() -> i16>,
    phase_voltage_l3_l1_callback: Option<extern "C" fn() -> u16>,
    phase_voltage_l3_n_callback: Option<extern "C" fn() -> u16>,
    total_watt_hours_inj_l3_callback: Option<extern "C" fn() -> u64>,
    total_watt_hours_abs_l3_callback: Option<extern "C" fn() -> u64>,
    total_var_hours_inj_l3_callback: Option<extern "C" fn() -> u64>,
    total_var_hours_abs_l3_callback: Option<extern "C" fn() -> u64>,
    throttling_in_pct_callback: Option<extern "C" fn() -> u16>,
    throttle_source_information_callback: Option<extern "C" fn() -> u32>,
    current_scale_factor_callback: Option<extern "C" fn() -> u16>,
    voltage_scale_factor_callback: Option<extern "C" fn() -> u16>,
    frequency_scale_factor_callback: Option<extern "C" fn() -> u16>,
    active_power_scale_factor_callback: Option<extern "C" fn() -> u16>,
    power_factor_scale_factor_callback: Option<extern "C" fn() -> u16>,
    apparent_power_scale_factor_callback: Option<extern "C" fn() -> u16>,
    reactive_power_scale_factor_callback: Option<extern "C" fn() -> u16>,
    active_energy_scale_factor_callback: Option<extern "C" fn() -> u16>,
    reactive_energy_scale_factor_callback: Option<extern "C" fn() -> u16>,
    temperature_scale_factor_callback: Option<extern "C" fn() -> u16>,
    manufacturer_alarm_info_callback: Option<extern "C" fn() -> *const c_char>,
}

impl ModelAdapter for Model701CallbackAdapter {
    /// AC Wiring Type
    ///
    /// AC wiring type.
    fn ac_wiring_type(&self) -> AcType {
        (self.ac_wiring_type_callback)()
    }

    /// Operating State
    ///
    /// Operating state of the DER.
    fn operating_state(&self) -> Option<St> {
        self.operating_state_callback.map(|callback| (callback)())
    }

    /// Inverter State
    ///
    /// Inverter state.
    fn inverter_state(&self) -> Option<InvSt> {
        self.inverter_state_callback.map(|callback| (callback)())
    }

    /// Grid Connection State
    ///
    /// Grid connection state of the DER.
    fn grid_connection_state(&self) -> Option<ConnSt> {
        self.grid_connection_state_callback
            .map(|callback| (callback)())
    }

    /// Alarm Bitfield
    ///
    /// Active alarms for the DER.
    fn alarm_bitfield(&self) -> Option<u32> {
        self.alarm_bitfield_callback.map(|callback| (callback)())
    }

    /// DER Operational Characteristics
    ///
    /// Current operational characteristics of the DER.
    fn der_operational_characteristics(&self) -> Option<u32> {
        self.der_operational_characteristics_callback
            .map(|callback| (callback)())
    }

    /// Active Power
    ///
    /// Total active power. Active power is positive for DER generation and negative for absorption.
    fn active_power(&self) -> Option<i16> {
        self.active_power_callback.map(|callback| (callback)())
    }

    /// Apparent Power
    ///
    /// Total apparent power.
    fn apparent_power(&self) -> Option<i16> {
        self.apparent_power_callback.map(|callback| (callback)())
    }

    /// Reactive Power
    ///
    /// Total reactive power.
    fn reactive_power(&self) -> Option<i16> {
        self.reactive_power_callback.map(|callback| (callback)())
    }

    /// Power Factor
    ///
    /// Power factor. The sign of power factor should be the sign of active power.
    fn power_factor(&self) -> Option<i16> {
        self.power_factor_callback.map(|callback| (callback)())
    }

    /// Total AC Current
    ///
    /// Total AC current.
    fn total_ac_current(&self) -> Option<i16> {
        self.total_ac_current_callback.map(|callback| (callback)())
    }

    /// Voltage LL
    ///
    /// Line to line AC voltage as an average of active phases.
    fn voltage_ll(&self) -> Option<u16> {
        self.voltage_ll_callback.map(|callback| (callback)())
    }

    /// Voltage LN
    ///
    /// Line to neutral AC voltage as an average of active phases.
    fn voltage_ln(&self) -> Option<u16> {
        self.voltage_ln_callback.map(|callback| (callback)())
    }

    /// Frequency
    ///
    /// AC frequency.
    fn frequency(&self) -> Option<u32> {
        self.frequency_callback.map(|callback| (callback)())
    }

    /// Total Energy Injected
    ///
    /// Total active energy injected (Quadrants 1 & 4).
    fn total_energy_injected(&self) -> Option<u64> {
        self.total_energy_injected_callback
            .map(|callback| (callback)())
    }

    /// Total Energy Absorbed
    ///
    /// Total active energy absorbed (Quadrants 2 & 3).
    fn total_energy_absorbed(&self) -> Option<u64> {
        self.total_energy_absorbed_callback
            .map(|callback| (callback)())
    }

    /// Total Reactive Energy Inj
    ///
    /// Total reactive energy injected (Quadrants 1 & 2).
    fn total_reactive_energy_inj(&self) -> Option<u64> {
        self.total_reactive_energy_inj_callback
            .map(|callback| (callback)())
    }

    /// Total Reactive Energy Abs
    ///
    /// Total reactive energy absorbed (Quadrants 3 & 4).
    fn total_reactive_energy_abs(&self) -> Option<u64> {
        self.total_reactive_energy_abs_callback
            .map(|callback| (callback)())
    }

    /// Ambient Temperature
    ///
    /// Ambient temperature.
    fn ambient_temperature(&self) -> Option<i16> {
        self.ambient_temperature_callback
            .map(|callback| (callback)())
    }

    /// Cabinet Temperature
    ///
    /// Cabinet temperature.
    fn cabinet_temperature(&self) -> Option<i16> {
        self.cabinet_temperature_callback
            .map(|callback| (callback)())
    }

    /// Heat Sink Temperature
    ///
    /// Heat sink temperature.
    fn heat_sink_temperature(&self) -> Option<i16> {
        self.heat_sink_temperature_callback
            .map(|callback| (callback)())
    }

    /// Transformer Temperature
    ///
    /// Transformer temperature.
    fn transformer_temperature(&self) -> Option<i16> {
        self.transformer_temperature_callback
            .map(|callback| (callback)())
    }

    /// IGBT/MOSFET Temperature
    ///
    /// IGBT/MOSFET temperature.
    fn igbt_mosfet_temperature(&self) -> Option<i16> {
        self.igbt_mosfet_temperature_callback
            .map(|callback| (callback)())
    }

    /// Other Temperature
    ///
    /// Other temperature.
    fn other_temperature(&self) -> Option<i16> {
        self.other_temperature_callback.map(|callback| (callback)())
    }

    /// Watts L1
    ///
    /// Active power L1.
    fn watts_l1(&self) -> Option<i16> {
        self.watts_l1_callback.map(|callback| (callback)())
    }

    /// VA L1
    ///
    /// Apparent power L1.
    fn va_l1(&self) -> Option<i16> {
        self.va_l1_callback.map(|callback| (callback)())
    }

    /// Var L1
    ///
    /// Reactive power L1.
    fn var_l1(&self) -> Option<i16> {
        self.var_l1_callback.map(|callback| (callback)())
    }

    /// PF L1
    ///
    /// Power factor phase L1.
    fn pf_l1(&self) -> Option<i16> {
        self.pf_l1_callback.map(|callback| (callback)())
    }

    /// Amps L1
    ///
    /// Current phase L1.
    fn amps_l1(&self) -> Option<i16> {
        self.amps_l1_callback.map(|callback| (callback)())
    }

    /// Phase Voltage L1-L2
    ///
    /// Phase voltage L1-L2.
    fn phase_voltage_l1_l2(&self) -> Option<u16> {
        self.phase_voltage_l1_l2_callback
            .map(|callback| (callback)())
    }

    /// Phase Voltage L1-N
    ///
    /// Phase voltage L1-N.
    fn phase_voltage_l1_n(&self) -> Option<u16> {
        self.phase_voltage_l1_n_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-Hours Inj L1
    ///
    /// Total active energy injected L1.
    fn total_watt_hours_inj_l1(&self) -> Option<u64> {
        self.total_watt_hours_inj_l1_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-Hours Abs L1
    ///
    /// Total active energy absorbed L1.
    fn total_watt_hours_abs_l1(&self) -> Option<u64> {
        self.total_watt_hours_abs_l1_callback
            .map(|callback| (callback)())
    }

    /// Total Var-Hours Inj L1
    ///
    /// Total reactive energy injected L1.
    fn total_var_hours_inj_l1(&self) -> Option<u64> {
        self.total_var_hours_inj_l1_callback
            .map(|callback| (callback)())
    }

    /// Total Var-Hours Abs L1
    ///
    /// Total reactive energy absorbed L1.
    fn total_var_hours_abs_l1(&self) -> Option<u64> {
        self.total_var_hours_abs_l1_callback
            .map(|callback| (callback)())
    }

    /// Watts L2
    ///
    /// Active power L2.
    fn watts_l2(&self) -> Option<i16> {
        self.watts_l2_callback.map(|callback| (callback)())
    }

    /// VA L2
    ///
    /// Apparent power L2.
    fn va_l2(&self) -> Option<i16> {
        self.va_l2_callback.map(|callback| (callback)())
    }

    /// Var L2
    ///
    /// Reactive power L2.
    fn var_l2(&self) -> Option<i16> {
        self.var_l2_callback.map(|callback| (callback)())
    }

    /// PF L2
    ///
    /// Power factor L2.
    fn pf_l2(&self) -> Option<i16> {
        self.pf_l2_callback.map(|callback| (callback)())
    }

    /// Amps L2
    ///
    /// Current L2.
    fn amps_l2(&self) -> Option<i16> {
        self.amps_l2_callback.map(|callback| (callback)())
    }

    /// Phase Voltage L2-L3
    ///
    /// Phase voltage L2-L3.
    fn phase_voltage_l2_l3(&self) -> Option<u16> {
        self.phase_voltage_l2_l3_callback
            .map(|callback| (callback)())
    }

    /// Phase Voltage L2-N
    ///
    /// Phase voltage L2-N.
    fn phase_voltage_l2_n(&self) -> Option<u16> {
        self.phase_voltage_l2_n_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-Hours Inj L2
    ///
    /// Total active energy injected L2.
    fn total_watt_hours_inj_l2(&self) -> Option<u64> {
        self.total_watt_hours_inj_l2_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-Hours Abs L2
    ///
    /// Total active energy absorbed L2.
    fn total_watt_hours_abs_l2(&self) -> Option<u64> {
        self.total_watt_hours_abs_l2_callback
            .map(|callback| (callback)())
    }

    /// Total Var-Hours Inj L2
    ///
    /// Total reactive energy injected L2.
    fn total_var_hours_inj_l2(&self) -> Option<u64> {
        self.total_var_hours_inj_l2_callback
            .map(|callback| (callback)())
    }

    /// Total Var-Hours Abs L2
    ///
    /// Total reactive energy absorbed L2.
    fn total_var_hours_abs_l2(&self) -> Option<u64> {
        self.total_var_hours_abs_l2_callback
            .map(|callback| (callback)())
    }

    /// Watts L3
    ///
    /// Active power L3.
    fn watts_l3(&self) -> Option<i16> {
        self.watts_l3_callback.map(|callback| (callback)())
    }

    /// VA L3
    ///
    /// Apparent power L3.
    fn va_l3(&self) -> Option<i16> {
        self.va_l3_callback.map(|callback| (callback)())
    }

    /// Var L3
    ///
    /// Reactive power L3.
    fn var_l3(&self) -> Option<i16> {
        self.var_l3_callback.map(|callback| (callback)())
    }

    /// PF L3
    ///
    /// Power factor L3.
    fn pf_l3(&self) -> Option<i16> {
        self.pf_l3_callback.map(|callback| (callback)())
    }

    /// Amps L3
    ///
    /// Current L3.
    fn amps_l3(&self) -> Option<i16> {
        self.amps_l3_callback.map(|callback| (callback)())
    }

    /// Phase Voltage L3-L1
    ///
    /// Phase voltage L3-L1.
    fn phase_voltage_l3_l1(&self) -> Option<u16> {
        self.phase_voltage_l3_l1_callback
            .map(|callback| (callback)())
    }

    /// Phase Voltage L3-N
    ///
    /// Phase voltage L3-N.
    fn phase_voltage_l3_n(&self) -> Option<u16> {
        self.phase_voltage_l3_n_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-Hours Inj L3
    ///
    /// Total active energy injected L3.
    fn total_watt_hours_inj_l3(&self) -> Option<u64> {
        self.total_watt_hours_inj_l3_callback
            .map(|callback| (callback)())
    }

    /// Total Watt-Hours Abs L3
    ///
    /// Total active energy absorbed L3.
    fn total_watt_hours_abs_l3(&self) -> Option<u64> {
        self.total_watt_hours_abs_l3_callback
            .map(|callback| (callback)())
    }

    /// Total Var-Hours Inj L3
    ///
    /// Total reactive energy injected L3.
    fn total_var_hours_inj_l3(&self) -> Option<u64> {
        self.total_var_hours_inj_l3_callback
            .map(|callback| (callback)())
    }

    /// Total Var-Hours Abs L3
    ///
    /// Total reactive energy absorbed L3.
    fn total_var_hours_abs_l3(&self) -> Option<u64> {
        self.total_var_hours_abs_l3_callback
            .map(|callback| (callback)())
    }

    /// Throttling In Pct
    ///
    /// Throttling in pct of maximum active power.
    fn throttling_in_pct(&self) -> Option<u16> {
        self.throttling_in_pct_callback.map(|callback| (callback)())
    }

    /// Throttle Source Information
    ///
    /// Active throttling source.
    fn throttle_source_information(&self) -> Option<u32> {
        self.throttle_source_information_callback
            .map(|callback| (callback)())
    }

    /// Current Scale Factor
    ///
    /// Current scale factor.
    fn current_scale_factor(&self) -> Option<u16> {
        self.current_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    fn voltage_scale_factor(&self) -> Option<u16> {
        self.voltage_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn frequency_scale_factor(&self) -> Option<u16> {
        self.frequency_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn active_power_scale_factor(&self) -> Option<u16> {
        self.active_power_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn power_factor_scale_factor(&self) -> Option<u16> {
        self.power_factor_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    fn apparent_power_scale_factor(&self) -> Option<u16> {
        self.apparent_power_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn reactive_power_scale_factor(&self) -> Option<u16> {
        self.reactive_power_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Active Energy Scale Factor
    ///
    /// Active energy scale factor.
    fn active_energy_scale_factor(&self) -> Option<u16> {
        self.active_energy_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Reactive Energy Scale Factor
    ///
    /// Reactive energy scale factor.
    fn reactive_energy_scale_factor(&self) -> Option<u16> {
        self.reactive_energy_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Temperature Scale Factor
    ///
    /// Temperature scale factor.
    fn temperature_scale_factor(&self) -> Option<u16> {
        self.temperature_scale_factor_callback
            .map(|callback| (callback)())
    }

    /// Manufacturer Alarm Info
    ///
    /// Manufacturer alarm information. Valid if MANUFACTURER_ALRM indication is active.
    fn manufacturer_alarm_info(&self) -> Option<&CStr> {
        self.manufacturer_alarm_info_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }
}
