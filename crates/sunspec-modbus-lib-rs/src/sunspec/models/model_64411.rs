use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 1398;

pub static POINTS: [ReadablePoint; 50] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64411 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::ActivePhases,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::PhaseAngle,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::NominalVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MaximumVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MaximumCurrent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::Frequency,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::OutputState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::RelayState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::RegenerationState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageSetpoint,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageSetpointPhaseA,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageSetpointPhaseB,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageSetpointPhaseC,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::FrequencySlewRate,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageSlewRate,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MeasuredVoltagePhaseA,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MeasuredVoltagePhaseB,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MeasuredVoltagePhaseC,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MeasuredFrequency,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MeasuredCurrentPhaseA,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MeasuredCurrentPhaseB,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MeasuredCurrentPhaseC,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageHarmonicsPhaseA,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageHarmonicsPhaseB,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageHarmonicsPhaseC,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentHarmonicsPhaseA,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentHarmonicsPhaseB,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentHarmonicsPhaseC,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentInterharmonicsPhaseA,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentInterharmonicsPhaseB,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentInterharmonicsPhaseC,
        },
        size: 150,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageThdPhaseA,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageThdPhaseB,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageThdPhaseC,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentThdPhaseA,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentThdPhaseB,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentThdPhaseC,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::EnableProfile,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::ProfileResult,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::StoredProfileCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::MaxProfilePointCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::CurrentScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::TimeScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::FrequencyScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::FrequencySlewRateScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::VoltageSlewRateScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64411 {
            point: Point::ThdScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    ActivePhases,
    PhaseAngle,
    NominalVoltage,
    MaximumVoltage,
    MaximumCurrent,
    Frequency,
    OutputState,
    RelayState,
    RegenerationState,
    VoltageSetpoint,
    VoltageSetpointPhaseA,
    VoltageSetpointPhaseB,
    VoltageSetpointPhaseC,
    FrequencySlewRate,
    VoltageSlewRate,
    MeasuredVoltagePhaseA,
    MeasuredVoltagePhaseB,
    MeasuredVoltagePhaseC,
    MeasuredFrequency,
    MeasuredCurrentPhaseA,
    MeasuredCurrentPhaseB,
    MeasuredCurrentPhaseC,
    VoltageHarmonicsPhaseA,
    VoltageHarmonicsPhaseB,
    VoltageHarmonicsPhaseC,
    CurrentHarmonicsPhaseA,
    CurrentHarmonicsPhaseB,
    CurrentHarmonicsPhaseC,
    CurrentInterharmonicsPhaseA,
    CurrentInterharmonicsPhaseB,
    CurrentInterharmonicsPhaseC,
    VoltageThdPhaseA,
    VoltageThdPhaseB,
    VoltageThdPhaseC,
    CurrentThdPhaseA,
    CurrentThdPhaseB,
    CurrentThdPhaseC,
    EnableProfile,
    ProfileResult,
    StoredProfileCount,
    MaxProfilePointCount,
    VoltageScaleFactor,
    CurrentScaleFactor,
    TimeScaleFactor,
    FrequencyScaleFactor,
    FrequencySlewRateScaleFactor,
    VoltageSlewRateScaleFactor,
    ThdScaleFactor,
    ProfProfileName { prof_index: u16 },
    ProfActivePoints { prof_index: u16 },
    PtProfileTime { prof_index: u16, pt_index: u16 },
    PtVoltagePoint { prof_index: u16, pt_index: u16 },
    PtVoltagePointPhaseB { prof_index: u16, pt_index: u16 },
    PtVoltagePointPhaseC { prof_index: u16, pt_index: u16 },
    PtFrequencyPoint { prof_index: u16, pt_index: u16 },
    PtPhaseAngleA { prof_index: u16, pt_index: u16 },
    PtPhaseAngleB { prof_index: u16, pt_index: u16 },
    PtPhaseAngleC { prof_index: u16, pt_index: u16 },
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    1398 + model.stored_profile_count() * (33 + model.max_profile_point_count() * (8))
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
        Point::ActivePhases => {
            if let Some(value) = model.active_phases() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseAngle => {
            if let Some(value) = model.phase_angle() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NominalVoltage => {
            if let Some(value) = model.nominal_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaximumVoltage => {
            if let Some(value) = model.maximum_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaximumCurrent => {
            if let Some(value) = model.maximum_current() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Frequency => {
            if let Some(value) = model.frequency() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OutputState => {
            if let Some(value) = model.output_state() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RelayState => {
            if let Some(value) = model.relay_state() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::RegenerationState => {
            if let Some(value) = model.regeneration_state() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageSetpoint => {
            if let Some(value) = model.voltage_setpoint() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageSetpointPhaseA => {
            if let Some(value) = model.voltage_setpoint_phase_a() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageSetpointPhaseB => {
            if let Some(value) = model.voltage_setpoint_phase_b() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageSetpointPhaseC => {
            if let Some(value) = model.voltage_setpoint_phase_c() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::FrequencySlewRate => {
            if let Some(value) = model.frequency_slew_rate() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageSlewRate => {
            if let Some(value) = model.voltage_slew_rate() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredVoltagePhaseA => {
            if let Some(value) = model.measured_voltage_phase_a() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredVoltagePhaseB => {
            if let Some(value) = model.measured_voltage_phase_b() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredVoltagePhaseC => {
            if let Some(value) = model.measured_voltage_phase_c() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredFrequency => {
            if let Some(value) = model.measured_frequency() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredCurrentPhaseA => {
            if let Some(value) = model.measured_current_phase_a() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredCurrentPhaseB => {
            if let Some(value) = model.measured_current_phase_b() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredCurrentPhaseC => {
            if let Some(value) = model.measured_current_phase_c() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageHarmonicsPhaseA => {
            if let Some(value) = model.voltage_harmonics_phase_a() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageHarmonicsPhaseB => {
            if let Some(value) = model.voltage_harmonics_phase_b() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageHarmonicsPhaseC => {
            if let Some(value) = model.voltage_harmonics_phase_c() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentHarmonicsPhaseA => {
            if let Some(value) = model.current_harmonics_phase_a() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentHarmonicsPhaseB => {
            if let Some(value) = model.current_harmonics_phase_b() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentHarmonicsPhaseC => {
            if let Some(value) = model.current_harmonics_phase_c() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentInterharmonicsPhaseA => {
            if let Some(value) = model.current_interharmonics_phase_a() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentInterharmonicsPhaseB => {
            if let Some(value) = model.current_interharmonics_phase_b() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentInterharmonicsPhaseC => {
            if let Some(value) = model.current_interharmonics_phase_c() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageThdPhaseA => {
            if let Some(value) = model.voltage_thd_phase_a() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageThdPhaseB => {
            if let Some(value) = model.voltage_thd_phase_b() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageThdPhaseC => {
            if let Some(value) = model.voltage_thd_phase_c() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentThdPhaseA => {
            if let Some(value) = model.current_thd_phase_a() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentThdPhaseB => {
            if let Some(value) = model.current_thd_phase_b() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentThdPhaseC => {
            if let Some(value) = model.current_thd_phase_c() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnableProfile => {
            if let Some(value) = model.enable_profile() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ProfileResult => {
            buffer::write_u16(model.profile_result() as u16, buffer);
        }
        Point::StoredProfileCount => {
            buffer::write_u16(model.stored_profile_count(), buffer);
        }
        Point::MaxProfilePointCount => {
            buffer::write_u16(model.max_profile_point_count(), buffer);
        }
        Point::VoltageScaleFactor => {
            buffer::write_u16(model.voltage_scale_factor(), buffer);
        }
        Point::CurrentScaleFactor => {
            buffer::write_u16(model.current_scale_factor(), buffer);
        }
        Point::TimeScaleFactor => {
            buffer::write_u16(model.time_scale_factor(), buffer);
        }
        Point::FrequencyScaleFactor => {
            buffer::write_u16(model.frequency_scale_factor(), buffer);
        }
        Point::FrequencySlewRateScaleFactor => {
            buffer::write_u16(model.frequency_slew_rate_scale_factor(), buffer);
        }
        Point::VoltageSlewRateScaleFactor => {
            buffer::write_u16(model.voltage_slew_rate_scale_factor(), buffer);
        }
        Point::ThdScaleFactor => {
            buffer::write_u16(model.thd_scale_factor(), buffer);
        }
        Point::ProfProfileName { prof_index } => {
            if let Some(value) = model.prof_profile_name(*prof_index) {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ProfActivePoints { prof_index } => {
            buffer::write_u16(model.prof_active_points(*prof_index), buffer);
        }
        Point::PtProfileTime {
            prof_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_profile_time(*prof_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtVoltagePoint {
            prof_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_voltage_point(*prof_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtVoltagePointPhaseB {
            prof_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_voltage_point_phase_b(*prof_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtVoltagePointPhaseC {
            prof_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_voltage_point_phase_c(*prof_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtFrequencyPoint {
            prof_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_frequency_point(*prof_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtPhaseAngleA {
            prof_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_phase_angle_a(*prof_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtPhaseAngleB {
            prof_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_phase_angle_b(*prof_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PtPhaseAngleC {
            prof_index,
            pt_index,
        } => {
            if let Some(value) = model.pt_phase_angle_c(*prof_index, *pt_index) {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn active_phases(&self) -> Option<u16> {
        None
    }

    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn set_active_phases(&mut self, value: u16) {}

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn phase_angle(&self) -> Option<u16> {
        None
    }

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn set_phase_angle(&mut self, value: u16) {}

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn nominal_voltage(&self) -> Option<u16> {
        None
    }

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn set_nominal_voltage(&mut self, value: u16) {}

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn maximum_voltage(&self) -> Option<u16> {
        None
    }

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn set_maximum_voltage(&mut self, value: u16) {}

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn maximum_current(&self) -> Option<u16> {
        None
    }

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn set_maximum_current(&mut self, value: u16) {}

    /// Frequency
    ///
    /// Frequency Setpoint
    fn frequency(&self) -> Option<u16> {
        None
    }

    /// Frequency
    ///
    /// Frequency Setpoint
    fn set_frequency(&mut self, value: u16) {}

    /// Output State
    ///
    /// AC Output State
    fn output_state(&self) -> Option<Output> {
        None
    }

    /// Output State
    ///
    /// AC Output State
    fn set_output_state(&mut self, value: Output) {}

    /// Relay State
    ///
    /// AC Relay State
    fn relay_state(&self) -> Option<Relay> {
        None
    }

    /// Relay State
    ///
    /// AC Relay State
    fn set_relay_state(&mut self, value: Relay) {}

    /// Regeneration State
    ///
    /// Regeneration State
    fn regeneration_state(&self) -> Option<Regen> {
        None
    }

    /// Regeneration State
    ///
    /// Regeneration State
    fn set_regeneration_state(&mut self, value: Regen) {}

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn voltage_setpoint(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn set_voltage_setpoint(&mut self, value: u16) {}

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn voltage_setpoint_phase_a(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn set_voltage_setpoint_phase_a(&mut self, value: u16) {}

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn voltage_setpoint_phase_b(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn set_voltage_setpoint_phase_b(&mut self, value: u16) {}

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn voltage_setpoint_phase_c(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn set_voltage_setpoint_phase_c(&mut self, value: u16) {}

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn frequency_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn set_frequency_slew_rate(&mut self, value: u16) {}

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn voltage_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn set_voltage_slew_rate(&mut self, value: u16) {}

    /// Measured Voltage Phase A
    ///
    /// Measured Voltage Phase A
    fn measured_voltage_phase_a(&self) -> Option<i32> {
        None
    }

    /// Measured Voltage Phase B
    ///
    /// Measured Voltage Phase B
    fn measured_voltage_phase_b(&self) -> Option<i32> {
        None
    }

    /// Measured Voltage Phase C
    ///
    /// Measured Voltage Phase C
    fn measured_voltage_phase_c(&self) -> Option<i32> {
        None
    }

    /// Measured Frequency
    ///
    /// Measured Frequency
    fn measured_frequency(&self) -> Option<i32> {
        None
    }

    /// Measured Current Phase A
    ///
    /// Measured Current Phase A
    fn measured_current_phase_a(&self) -> Option<i32> {
        None
    }

    /// Measured Current Phase B
    ///
    /// Measured Current Phase B
    fn measured_current_phase_b(&self) -> Option<i32> {
        None
    }

    /// Measured Current Phase C
    ///
    /// Measured Current Phase C
    fn measured_current_phase_c(&self) -> Option<i32> {
        None
    }

    /// Voltage Harmonics Phase A
    ///
    /// Voltage Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_a(&self) -> Option<&CStr> {
        None
    }

    /// Voltage Harmonics Phase B
    ///
    /// Voltage Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_b(&self) -> Option<&CStr> {
        None
    }

    /// Voltage Harmonics Phase C
    ///
    /// Voltage Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_c(&self) -> Option<&CStr> {
        None
    }

    /// Current Harmonics Phase A
    ///
    /// Current Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_a(&self) -> Option<&CStr> {
        None
    }

    /// Current Harmonics Phase B
    ///
    /// Current Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_b(&self) -> Option<&CStr> {
        None
    }

    /// Current Harmonics Phase C
    ///
    /// Current Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_c(&self) -> Option<&CStr> {
        None
    }

    /// Current Interharmonics Phase A
    ///
    /// Current Interharmonics Pct, Phase A (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_a(&self) -> Option<&CStr> {
        None
    }

    /// Current Interharmonics Phase B
    ///
    /// Current Interharmonics Pct, Phase B (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_b(&self) -> Option<&CStr> {
        None
    }

    /// Current Interharmonics Phase C
    ///
    /// Current Interharmonics Pct, Phase C (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_c(&self) -> Option<&CStr> {
        None
    }

    /// Voltage THD Phase A
    ///
    /// Voltage THD Phase A
    fn voltage_thd_phase_a(&self) -> Option<u16> {
        None
    }

    /// Voltage THD Phase B
    ///
    /// Voltage THD Phase B
    fn voltage_thd_phase_b(&self) -> Option<u16> {
        None
    }

    /// Voltage THD Phase C
    ///
    /// Voltage THD Phase C
    fn voltage_thd_phase_c(&self) -> Option<u16> {
        None
    }

    /// Current THD Phase A
    ///
    /// Current THD Phase A
    fn current_thd_phase_a(&self) -> Option<u16> {
        None
    }

    /// Current THD Phase B
    ///
    /// Current THD Phase B
    fn current_thd_phase_b(&self) -> Option<u16> {
        None
    }

    /// Current THD Phase C
    ///
    /// Current THD Phase C
    fn current_thd_phase_c(&self) -> Option<u16> {
        None
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn enable_profile(&self) -> Option<EnaProf> {
        None
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn set_enable_profile(&mut self, value: EnaProf) {}

    /// Profile Result
    ///
    /// Result of last profile operation.
    fn profile_result(&self) -> ProfRslt;

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn stored_profile_count(&self) -> u16;

    /// Max Profile Point Count
    ///
    /// Max profile points in the profiles.
    fn max_profile_point_count(&self) -> u16;

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn voltage_scale_factor(&self) -> u16;

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn current_scale_factor(&self) -> u16;

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn time_scale_factor(&self) -> u16;

    /// Frequency Scale Factor
    ///
    /// Scale factor for frequency points.
    fn frequency_scale_factor(&self) -> u16;

    /// Frequency Slew Rate Scale Factor
    ///
    /// Scale factor for frequency slew rate.
    fn frequency_slew_rate_scale_factor(&self) -> u16;

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn voltage_slew_rate_scale_factor(&self) -> u16;

    /// THD Scale Factor
    ///
    /// Scale factor for THD values.
    fn thd_scale_factor(&self) -> u16;

    /// Profile Name
    ///
    /// Profile name.
    fn prof_profile_name(&self, prof_index: u16) -> Option<&CStr> {
        None
    }

    /// Profile Name
    ///
    /// Profile name.
    fn set_prof_profile_name(&mut self, value: &CStr, prof_index: u16) {}

    /// Active Points
    ///
    /// Number of active points.
    fn prof_active_points(&self, prof_index: u16) -> u16;

    /// Active Points
    ///
    /// Number of active points.
    fn set_prof_active_points(&mut self, value: u16, prof_index: u16);

    /// Profile Time
    ///
    /// Profile time.
    fn pt_profile_time(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Profile Time
    ///
    /// Profile time.
    fn set_pt_profile_time(&mut self, value: u16, prof_index: u16, pt_index: u16) {}

    /// Voltage Point
    ///
    /// Profile voltage phase A point in Volts.
    fn pt_voltage_point(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Voltage Point
    ///
    /// Profile voltage phase A point in Volts.
    fn set_pt_voltage_point(&mut self, value: u16, prof_index: u16, pt_index: u16) {}

    /// Voltage Point Phase B
    ///
    /// Profile voltage phase B point in Volts.
    fn pt_voltage_point_phase_b(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Voltage Point Phase B
    ///
    /// Profile voltage phase B point in Volts.
    fn set_pt_voltage_point_phase_b(&mut self, value: u16, prof_index: u16, pt_index: u16) {}

    /// Voltage Point Phase C
    ///
    /// Profile voltage phase C point in Volts.
    fn pt_voltage_point_phase_c(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Voltage Point Phase C
    ///
    /// Profile voltage phase C point in Volts.
    fn set_pt_voltage_point_phase_c(&mut self, value: u16, prof_index: u16, pt_index: u16) {}

    /// Frequency Point
    ///
    /// Profile frequency point in Hz.
    fn pt_frequency_point(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Frequency Point
    ///
    /// Profile frequency point in Hz.
    fn set_pt_frequency_point(&mut self, value: u16, prof_index: u16, pt_index: u16) {}

    /// Phase Angle A
    ///
    /// Profile phase A angle in degrees.
    fn pt_phase_angle_a(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Phase Angle A
    ///
    /// Profile phase A angle in degrees.
    fn set_pt_phase_angle_a(&mut self, value: u16, prof_index: u16, pt_index: u16) {}

    /// Phase Angle B
    ///
    /// Profile phase B angle in degrees.
    fn pt_phase_angle_b(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Phase Angle B
    ///
    /// Profile phase B angle in degrees.
    fn set_pt_phase_angle_b(&mut self, value: u16, prof_index: u16, pt_index: u16) {}

    /// Phase Angle C
    ///
    /// Profile phase C angle in degrees.
    fn pt_phase_angle_c(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        None
    }

    /// Phase Angle C
    ///
    /// Profile phase C angle in degrees.
    fn set_pt_phase_angle_c(&mut self, value: u16, prof_index: u16, pt_index: u16) {}
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum EnaProf {
    /// Stop Profile
    Stop = 0,
    /// Start Profile Immediately
    Start = 1,
    /// Start Profile via External Trigger Signal
    Trigger = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Output {
    /// Output Off
    Off = 0,
    /// Output On
    On = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ProfRslt {
    /// Profile update in progress.
    InProgress = 0,
    /// Profile update completed successfully.
    Completed = 1,
    /// Profile update failed.
    Failed = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Regen {
    /// Regen Off
    Off = 0,
    /// Regen On
    On = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Relay {
    /// Relay Open
    Open = 0,
    /// Relay Closed
    Closed = 1,
}

#[repr(C)]
pub struct Model64411CallbackAdapter {
    context: *mut c_void,
    active_phases_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_active_phases_callback: Option<extern "C" fn(u16, *mut c_void)>,
    phase_angle_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_phase_angle_callback: Option<extern "C" fn(u16, *mut c_void)>,
    nominal_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_nominal_voltage_callback: Option<extern "C" fn(u16, *mut c_void)>,
    maximum_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_maximum_voltage_callback: Option<extern "C" fn(u16, *mut c_void)>,
    maximum_current_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_maximum_current_callback: Option<extern "C" fn(u16, *mut c_void)>,
    frequency_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_frequency_callback: Option<extern "C" fn(u16, *mut c_void)>,
    output_state_callback: Option<extern "C" fn(*const c_void) -> Output>,
    set_output_state_callback: Option<extern "C" fn(Output, *mut c_void)>,
    relay_state_callback: Option<extern "C" fn(*const c_void) -> Relay>,
    set_relay_state_callback: Option<extern "C" fn(Relay, *mut c_void)>,
    regeneration_state_callback: Option<extern "C" fn(*const c_void) -> Regen>,
    set_regeneration_state_callback: Option<extern "C" fn(Regen, *mut c_void)>,
    voltage_setpoint_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_voltage_setpoint_callback: Option<extern "C" fn(u16, *mut c_void)>,
    voltage_setpoint_phase_a_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_voltage_setpoint_phase_a_callback: Option<extern "C" fn(u16, *mut c_void)>,
    voltage_setpoint_phase_b_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_voltage_setpoint_phase_b_callback: Option<extern "C" fn(u16, *mut c_void)>,
    voltage_setpoint_phase_c_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_voltage_setpoint_phase_c_callback: Option<extern "C" fn(u16, *mut c_void)>,
    frequency_slew_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_frequency_slew_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    voltage_slew_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_voltage_slew_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    measured_voltage_phase_a_callback: Option<extern "C" fn(*const c_void) -> i32>,
    measured_voltage_phase_b_callback: Option<extern "C" fn(*const c_void) -> i32>,
    measured_voltage_phase_c_callback: Option<extern "C" fn(*const c_void) -> i32>,
    measured_frequency_callback: Option<extern "C" fn(*const c_void) -> i32>,
    measured_current_phase_a_callback: Option<extern "C" fn(*const c_void) -> i32>,
    measured_current_phase_b_callback: Option<extern "C" fn(*const c_void) -> i32>,
    measured_current_phase_c_callback: Option<extern "C" fn(*const c_void) -> i32>,
    voltage_harmonics_phase_a_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    voltage_harmonics_phase_b_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    voltage_harmonics_phase_c_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    current_harmonics_phase_a_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    current_harmonics_phase_b_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    current_harmonics_phase_c_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    current_interharmonics_phase_a_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    current_interharmonics_phase_b_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    current_interharmonics_phase_c_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    voltage_thd_phase_a_callback: Option<extern "C" fn(*const c_void) -> u16>,
    voltage_thd_phase_b_callback: Option<extern "C" fn(*const c_void) -> u16>,
    voltage_thd_phase_c_callback: Option<extern "C" fn(*const c_void) -> u16>,
    current_thd_phase_a_callback: Option<extern "C" fn(*const c_void) -> u16>,
    current_thd_phase_b_callback: Option<extern "C" fn(*const c_void) -> u16>,
    current_thd_phase_c_callback: Option<extern "C" fn(*const c_void) -> u16>,
    enable_profile_callback: Option<extern "C" fn(*const c_void) -> EnaProf>,
    set_enable_profile_callback: Option<extern "C" fn(EnaProf, *mut c_void)>,
    profile_result_callback: extern "C" fn(*const c_void) -> ProfRslt,
    stored_profile_count_callback: extern "C" fn(*const c_void) -> u16,
    max_profile_point_count_callback: extern "C" fn(*const c_void) -> u16,
    voltage_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    current_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    time_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    frequency_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    frequency_slew_rate_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    voltage_slew_rate_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    thd_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    prof_profile_name_callback: Option<extern "C" fn(*const c_void, u16) -> *const c_char>,
    set_prof_profile_name_callback: Option<extern "C" fn(*const c_char, *mut c_void, u16)>,
    prof_active_points_callback: extern "C" fn(*const c_void, u16) -> u16,
    set_prof_active_points_callback: extern "C" fn(u16, *mut c_void, u16),
    pt_profile_time_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_profile_time_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
    pt_voltage_point_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_voltage_point_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
    pt_voltage_point_phase_b_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_voltage_point_phase_b_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
    pt_voltage_point_phase_c_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_voltage_point_phase_c_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
    pt_frequency_point_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_frequency_point_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
    pt_phase_angle_a_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_phase_angle_a_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
    pt_phase_angle_b_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_phase_angle_b_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
    pt_phase_angle_c_callback: Option<extern "C" fn(*const c_void, u16, u16) -> u16>,
    set_pt_phase_angle_c_callback: Option<extern "C" fn(u16, *mut c_void, u16, u16)>,
}

impl ModelAdapter for Model64411CallbackAdapter {
    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn active_phases(&self) -> Option<u16> {
        self.active_phases_callback
            .map(|callback| (callback)(self.context))
    }

    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn set_active_phases(&mut self, value: u16) {
        if let Some(callback) = self.set_active_phases_callback {
            (callback)(value, self.context);
        };
    }

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn phase_angle(&self) -> Option<u16> {
        self.phase_angle_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn set_phase_angle(&mut self, value: u16) {
        if let Some(callback) = self.set_phase_angle_callback {
            (callback)(value, self.context);
        };
    }

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn nominal_voltage(&self) -> Option<u16> {
        self.nominal_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn set_nominal_voltage(&mut self, value: u16) {
        if let Some(callback) = self.set_nominal_voltage_callback {
            (callback)(value, self.context);
        };
    }

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn maximum_voltage(&self) -> Option<u16> {
        self.maximum_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn set_maximum_voltage(&mut self, value: u16) {
        if let Some(callback) = self.set_maximum_voltage_callback {
            (callback)(value, self.context);
        };
    }

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn maximum_current(&self) -> Option<u16> {
        self.maximum_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn set_maximum_current(&mut self, value: u16) {
        if let Some(callback) = self.set_maximum_current_callback {
            (callback)(value, self.context);
        };
    }

    /// Frequency
    ///
    /// Frequency Setpoint
    fn frequency(&self) -> Option<u16> {
        self.frequency_callback
            .map(|callback| (callback)(self.context))
    }

    /// Frequency
    ///
    /// Frequency Setpoint
    fn set_frequency(&mut self, value: u16) {
        if let Some(callback) = self.set_frequency_callback {
            (callback)(value, self.context);
        };
    }

    /// Output State
    ///
    /// AC Output State
    fn output_state(&self) -> Option<Output> {
        self.output_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Output State
    ///
    /// AC Output State
    fn set_output_state(&mut self, value: Output) {
        if let Some(callback) = self.set_output_state_callback {
            (callback)(value, self.context);
        };
    }

    /// Relay State
    ///
    /// AC Relay State
    fn relay_state(&self) -> Option<Relay> {
        self.relay_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Relay State
    ///
    /// AC Relay State
    fn set_relay_state(&mut self, value: Relay) {
        if let Some(callback) = self.set_relay_state_callback {
            (callback)(value, self.context);
        };
    }

    /// Regeneration State
    ///
    /// Regeneration State
    fn regeneration_state(&self) -> Option<Regen> {
        self.regeneration_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Regeneration State
    ///
    /// Regeneration State
    fn set_regeneration_state(&mut self, value: Regen) {
        if let Some(callback) = self.set_regeneration_state_callback {
            (callback)(value, self.context);
        };
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn voltage_setpoint(&self) -> Option<u16> {
        self.voltage_setpoint_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn set_voltage_setpoint(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_callback {
            (callback)(value, self.context);
        };
    }

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn voltage_setpoint_phase_a(&self) -> Option<u16> {
        self.voltage_setpoint_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn set_voltage_setpoint_phase_a(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_phase_a_callback {
            (callback)(value, self.context);
        };
    }

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn voltage_setpoint_phase_b(&self) -> Option<u16> {
        self.voltage_setpoint_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn set_voltage_setpoint_phase_b(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_phase_b_callback {
            (callback)(value, self.context);
        };
    }

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn voltage_setpoint_phase_c(&self) -> Option<u16> {
        self.voltage_setpoint_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn set_voltage_setpoint_phase_c(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_phase_c_callback {
            (callback)(value, self.context);
        };
    }

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn frequency_slew_rate(&self) -> Option<u16> {
        self.frequency_slew_rate_callback
            .map(|callback| (callback)(self.context))
    }

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn set_frequency_slew_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_frequency_slew_rate_callback {
            (callback)(value, self.context);
        };
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn voltage_slew_rate(&self) -> Option<u16> {
        self.voltage_slew_rate_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn set_voltage_slew_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_slew_rate_callback {
            (callback)(value, self.context);
        };
    }

    /// Measured Voltage Phase A
    ///
    /// Measured Voltage Phase A
    fn measured_voltage_phase_a(&self) -> Option<i32> {
        self.measured_voltage_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Measured Voltage Phase B
    ///
    /// Measured Voltage Phase B
    fn measured_voltage_phase_b(&self) -> Option<i32> {
        self.measured_voltage_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Measured Voltage Phase C
    ///
    /// Measured Voltage Phase C
    fn measured_voltage_phase_c(&self) -> Option<i32> {
        self.measured_voltage_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Measured Frequency
    ///
    /// Measured Frequency
    fn measured_frequency(&self) -> Option<i32> {
        self.measured_frequency_callback
            .map(|callback| (callback)(self.context))
    }

    /// Measured Current Phase A
    ///
    /// Measured Current Phase A
    fn measured_current_phase_a(&self) -> Option<i32> {
        self.measured_current_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Measured Current Phase B
    ///
    /// Measured Current Phase B
    fn measured_current_phase_b(&self) -> Option<i32> {
        self.measured_current_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Measured Current Phase C
    ///
    /// Measured Current Phase C
    fn measured_current_phase_c(&self) -> Option<i32> {
        self.measured_current_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Harmonics Phase A
    ///
    /// Voltage Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_a(&self) -> Option<&CStr> {
        self.voltage_harmonics_phase_a_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Voltage Harmonics Phase B
    ///
    /// Voltage Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_b(&self) -> Option<&CStr> {
        self.voltage_harmonics_phase_b_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Voltage Harmonics Phase C
    ///
    /// Voltage Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_c(&self) -> Option<&CStr> {
        self.voltage_harmonics_phase_c_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Current Harmonics Phase A
    ///
    /// Current Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_a(&self) -> Option<&CStr> {
        self.current_harmonics_phase_a_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Current Harmonics Phase B
    ///
    /// Current Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_b(&self) -> Option<&CStr> {
        self.current_harmonics_phase_b_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Current Harmonics Phase C
    ///
    /// Current Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_c(&self) -> Option<&CStr> {
        self.current_harmonics_phase_c_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Current Interharmonics Phase A
    ///
    /// Current Interharmonics Pct, Phase A (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_a(&self) -> Option<&CStr> {
        self.current_interharmonics_phase_a_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Current Interharmonics Phase B
    ///
    /// Current Interharmonics Pct, Phase B (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_b(&self) -> Option<&CStr> {
        self.current_interharmonics_phase_b_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Current Interharmonics Phase C
    ///
    /// Current Interharmonics Pct, Phase C (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_c(&self) -> Option<&CStr> {
        self.current_interharmonics_phase_c_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Voltage THD Phase A
    ///
    /// Voltage THD Phase A
    fn voltage_thd_phase_a(&self) -> Option<u16> {
        self.voltage_thd_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage THD Phase B
    ///
    /// Voltage THD Phase B
    fn voltage_thd_phase_b(&self) -> Option<u16> {
        self.voltage_thd_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage THD Phase C
    ///
    /// Voltage THD Phase C
    fn voltage_thd_phase_c(&self) -> Option<u16> {
        self.voltage_thd_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Current THD Phase A
    ///
    /// Current THD Phase A
    fn current_thd_phase_a(&self) -> Option<u16> {
        self.current_thd_phase_a_callback
            .map(|callback| (callback)(self.context))
    }

    /// Current THD Phase B
    ///
    /// Current THD Phase B
    fn current_thd_phase_b(&self) -> Option<u16> {
        self.current_thd_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Current THD Phase C
    ///
    /// Current THD Phase C
    fn current_thd_phase_c(&self) -> Option<u16> {
        self.current_thd_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn enable_profile(&self) -> Option<EnaProf> {
        self.enable_profile_callback
            .map(|callback| (callback)(self.context))
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn set_enable_profile(&mut self, value: EnaProf) {
        if let Some(callback) = self.set_enable_profile_callback {
            (callback)(value, self.context);
        };
    }

    /// Profile Result
    ///
    /// Result of last profile operation.
    fn profile_result(&self) -> ProfRslt {
        (self.profile_result_callback)(self.context)
    }

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn stored_profile_count(&self) -> u16 {
        (self.stored_profile_count_callback)(self.context)
    }

    /// Max Profile Point Count
    ///
    /// Max profile points in the profiles.
    fn max_profile_point_count(&self) -> u16 {
        (self.max_profile_point_count_callback)(self.context)
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        (self.voltage_scale_factor_callback)(self.context)
    }

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn current_scale_factor(&self) -> u16 {
        (self.current_scale_factor_callback)(self.context)
    }

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn time_scale_factor(&self) -> u16 {
        (self.time_scale_factor_callback)(self.context)
    }

    /// Frequency Scale Factor
    ///
    /// Scale factor for frequency points.
    fn frequency_scale_factor(&self) -> u16 {
        (self.frequency_scale_factor_callback)(self.context)
    }

    /// Frequency Slew Rate Scale Factor
    ///
    /// Scale factor for frequency slew rate.
    fn frequency_slew_rate_scale_factor(&self) -> u16 {
        (self.frequency_slew_rate_scale_factor_callback)(self.context)
    }

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn voltage_slew_rate_scale_factor(&self) -> u16 {
        (self.voltage_slew_rate_scale_factor_callback)(self.context)
    }

    /// THD Scale Factor
    ///
    /// Scale factor for THD values.
    fn thd_scale_factor(&self) -> u16 {
        (self.thd_scale_factor_callback)(self.context)
    }

    /// Profile Name
    ///
    /// Profile name.
    fn prof_profile_name(&self, prof_index: u16) -> Option<&CStr> {
        self.prof_profile_name_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context, prof_index)) })
    }

    /// Profile Name
    ///
    /// Profile name.
    fn set_prof_profile_name(&mut self, value: &CStr, prof_index: u16) {
        if let Some(callback) = self.set_prof_profile_name_callback {
            (callback)(value.as_ptr(), self.context, prof_index);
        };
    }

    /// Active Points
    ///
    /// Number of active points.
    fn prof_active_points(&self, prof_index: u16) -> u16 {
        (self.prof_active_points_callback)(self.context, prof_index)
    }

    /// Active Points
    ///
    /// Number of active points.
    fn set_prof_active_points(&mut self, value: u16, prof_index: u16) {
        (self.set_prof_active_points_callback)(value, self.context, prof_index);
    }

    /// Profile Time
    ///
    /// Profile time.
    fn pt_profile_time(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_profile_time_callback
            .map(|callback| (callback)(self.context, prof_index, pt_index))
    }

    /// Profile Time
    ///
    /// Profile time.
    fn set_pt_profile_time(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_profile_time_callback {
            (callback)(value, self.context, prof_index, pt_index);
        };
    }

    /// Voltage Point
    ///
    /// Profile voltage phase A point in Volts.
    fn pt_voltage_point(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_voltage_point_callback
            .map(|callback| (callback)(self.context, prof_index, pt_index))
    }

    /// Voltage Point
    ///
    /// Profile voltage phase A point in Volts.
    fn set_pt_voltage_point(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_voltage_point_callback {
            (callback)(value, self.context, prof_index, pt_index);
        };
    }

    /// Voltage Point Phase B
    ///
    /// Profile voltage phase B point in Volts.
    fn pt_voltage_point_phase_b(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_voltage_point_phase_b_callback
            .map(|callback| (callback)(self.context, prof_index, pt_index))
    }

    /// Voltage Point Phase B
    ///
    /// Profile voltage phase B point in Volts.
    fn set_pt_voltage_point_phase_b(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_voltage_point_phase_b_callback {
            (callback)(value, self.context, prof_index, pt_index);
        };
    }

    /// Voltage Point Phase C
    ///
    /// Profile voltage phase C point in Volts.
    fn pt_voltage_point_phase_c(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_voltage_point_phase_c_callback
            .map(|callback| (callback)(self.context, prof_index, pt_index))
    }

    /// Voltage Point Phase C
    ///
    /// Profile voltage phase C point in Volts.
    fn set_pt_voltage_point_phase_c(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_voltage_point_phase_c_callback {
            (callback)(value, self.context, prof_index, pt_index);
        };
    }

    /// Frequency Point
    ///
    /// Profile frequency point in Hz.
    fn pt_frequency_point(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_frequency_point_callback
            .map(|callback| (callback)(self.context, prof_index, pt_index))
    }

    /// Frequency Point
    ///
    /// Profile frequency point in Hz.
    fn set_pt_frequency_point(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_frequency_point_callback {
            (callback)(value, self.context, prof_index, pt_index);
        };
    }

    /// Phase Angle A
    ///
    /// Profile phase A angle in degrees.
    fn pt_phase_angle_a(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_phase_angle_a_callback
            .map(|callback| (callback)(self.context, prof_index, pt_index))
    }

    /// Phase Angle A
    ///
    /// Profile phase A angle in degrees.
    fn set_pt_phase_angle_a(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_phase_angle_a_callback {
            (callback)(value, self.context, prof_index, pt_index);
        };
    }

    /// Phase Angle B
    ///
    /// Profile phase B angle in degrees.
    fn pt_phase_angle_b(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_phase_angle_b_callback
            .map(|callback| (callback)(self.context, prof_index, pt_index))
    }

    /// Phase Angle B
    ///
    /// Profile phase B angle in degrees.
    fn set_pt_phase_angle_b(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_phase_angle_b_callback {
            (callback)(value, self.context, prof_index, pt_index);
        };
    }

    /// Phase Angle C
    ///
    /// Profile phase C angle in degrees.
    fn pt_phase_angle_c(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        self.pt_phase_angle_c_callback
            .map(|callback| (callback)(self.context, prof_index, pt_index))
    }

    /// Phase Angle C
    ///
    /// Profile phase C angle in degrees.
    fn set_pt_phase_angle_c(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        if let Some(callback) = self.set_pt_phase_angle_c_callback {
            (callback)(value, self.context, prof_index, pt_index);
        };
    }
}

#[repr(C)]
pub struct Model64411StatefulAdapter<
    const STORED_PROFILE_COUNT: usize,
    const MAX_PROFILE_POINT_COUNT: usize,
> {
    active_phases: u16,
    phase_angle: u16,
    nominal_voltage: u16,
    maximum_voltage: u16,
    maximum_current: u16,
    frequency: u16,
    output_state: Output,
    relay_state: Relay,
    regeneration_state: Regen,
    voltage_setpoint: u16,
    voltage_setpoint_phase_a: u16,
    voltage_setpoint_phase_b: u16,
    voltage_setpoint_phase_c: u16,
    frequency_slew_rate: u16,
    voltage_slew_rate: u16,
    measured_voltage_phase_a: i32,
    measured_voltage_phase_b: i32,
    measured_voltage_phase_c: i32,
    measured_frequency: i32,
    measured_current_phase_a: i32,
    measured_current_phase_b: i32,
    measured_current_phase_c: i32,
    voltage_harmonics_phase_a: [c_char; 300],
    voltage_harmonics_phase_b: [c_char; 300],
    voltage_harmonics_phase_c: [c_char; 300],
    current_harmonics_phase_a: [c_char; 300],
    current_harmonics_phase_b: [c_char; 300],
    current_harmonics_phase_c: [c_char; 300],
    current_interharmonics_phase_a: [c_char; 300],
    current_interharmonics_phase_b: [c_char; 300],
    current_interharmonics_phase_c: [c_char; 300],
    voltage_thd_phase_a: u16,
    voltage_thd_phase_b: u16,
    voltage_thd_phase_c: u16,
    current_thd_phase_a: u16,
    current_thd_phase_b: u16,
    current_thd_phase_c: u16,
    enable_profile: EnaProf,
    profile_result: ProfRslt,
    stored_profile_count: u16,
    max_profile_point_count: u16,
    voltage_scale_factor: u16,
    current_scale_factor: u16,
    time_scale_factor: u16,
    frequency_scale_factor: u16,
    frequency_slew_rate_scale_factor: u16,
    voltage_slew_rate_scale_factor: u16,
    thd_scale_factor: u16,
    stored_ac_profiles: [Model64411StoredAcProfiles<MAX_PROFILE_POINT_COUNT>; STORED_PROFILE_COUNT],
}

#[repr(C)]
pub struct Model64411StoredAcProfiles<const MAX_PROFILE_POINT_COUNT: usize> {
    prof_profile_name: [c_char; 64],
    prof_active_points: u16,
    stored_ac_profile_points: [Model64411StoredAcProfilePoints; MAX_PROFILE_POINT_COUNT],
}

#[repr(C)]
pub struct Model64411StoredAcProfilePoints {
    pt_profile_time: u16,
    pt_voltage_point: u16,
    pt_voltage_point_phase_b: u16,
    pt_voltage_point_phase_c: u16,
    pt_frequency_point: u16,
    pt_phase_angle_a: u16,
    pt_phase_angle_b: u16,
    pt_phase_angle_c: u16,
}

impl<const STORED_PROFILE_COUNT: usize, const MAX_PROFILE_POINT_COUNT: usize> ModelAdapter
    for Model64411StatefulAdapter<STORED_PROFILE_COUNT, MAX_PROFILE_POINT_COUNT>
{
    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn active_phases(&self) -> Option<u16> {
        Some(self.active_phases)
    }

    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn set_active_phases(&mut self, value: u16) {
        self.active_phases = value;
    }

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn phase_angle(&self) -> Option<u16> {
        Some(self.phase_angle)
    }

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn set_phase_angle(&mut self, value: u16) {
        self.phase_angle = value;
    }

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn nominal_voltage(&self) -> Option<u16> {
        Some(self.nominal_voltage)
    }

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn set_nominal_voltage(&mut self, value: u16) {
        self.nominal_voltage = value;
    }

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn maximum_voltage(&self) -> Option<u16> {
        Some(self.maximum_voltage)
    }

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn set_maximum_voltage(&mut self, value: u16) {
        self.maximum_voltage = value;
    }

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn maximum_current(&self) -> Option<u16> {
        Some(self.maximum_current)
    }

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn set_maximum_current(&mut self, value: u16) {
        self.maximum_current = value;
    }

    /// Frequency
    ///
    /// Frequency Setpoint
    fn frequency(&self) -> Option<u16> {
        Some(self.frequency)
    }

    /// Frequency
    ///
    /// Frequency Setpoint
    fn set_frequency(&mut self, value: u16) {
        self.frequency = value;
    }

    /// Output State
    ///
    /// AC Output State
    fn output_state(&self) -> Option<Output> {
        Some(self.output_state)
    }

    /// Output State
    ///
    /// AC Output State
    fn set_output_state(&mut self, value: Output) {
        self.output_state = value;
    }

    /// Relay State
    ///
    /// AC Relay State
    fn relay_state(&self) -> Option<Relay> {
        Some(self.relay_state)
    }

    /// Relay State
    ///
    /// AC Relay State
    fn set_relay_state(&mut self, value: Relay) {
        self.relay_state = value;
    }

    /// Regeneration State
    ///
    /// Regeneration State
    fn regeneration_state(&self) -> Option<Regen> {
        Some(self.regeneration_state)
    }

    /// Regeneration State
    ///
    /// Regeneration State
    fn set_regeneration_state(&mut self, value: Regen) {
        self.regeneration_state = value;
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn voltage_setpoint(&self) -> Option<u16> {
        Some(self.voltage_setpoint)
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn set_voltage_setpoint(&mut self, value: u16) {
        self.voltage_setpoint = value;
    }

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn voltage_setpoint_phase_a(&self) -> Option<u16> {
        Some(self.voltage_setpoint_phase_a)
    }

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn set_voltage_setpoint_phase_a(&mut self, value: u16) {
        self.voltage_setpoint_phase_a = value;
    }

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn voltage_setpoint_phase_b(&self) -> Option<u16> {
        Some(self.voltage_setpoint_phase_b)
    }

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn set_voltage_setpoint_phase_b(&mut self, value: u16) {
        self.voltage_setpoint_phase_b = value;
    }

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn voltage_setpoint_phase_c(&self) -> Option<u16> {
        Some(self.voltage_setpoint_phase_c)
    }

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn set_voltage_setpoint_phase_c(&mut self, value: u16) {
        self.voltage_setpoint_phase_c = value;
    }

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn frequency_slew_rate(&self) -> Option<u16> {
        Some(self.frequency_slew_rate)
    }

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn set_frequency_slew_rate(&mut self, value: u16) {
        self.frequency_slew_rate = value;
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn voltage_slew_rate(&self) -> Option<u16> {
        Some(self.voltage_slew_rate)
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn set_voltage_slew_rate(&mut self, value: u16) {
        self.voltage_slew_rate = value;
    }

    /// Measured Voltage Phase A
    ///
    /// Measured Voltage Phase A
    fn measured_voltage_phase_a(&self) -> Option<i32> {
        Some(self.measured_voltage_phase_a)
    }

    /// Measured Voltage Phase B
    ///
    /// Measured Voltage Phase B
    fn measured_voltage_phase_b(&self) -> Option<i32> {
        Some(self.measured_voltage_phase_b)
    }

    /// Measured Voltage Phase C
    ///
    /// Measured Voltage Phase C
    fn measured_voltage_phase_c(&self) -> Option<i32> {
        Some(self.measured_voltage_phase_c)
    }

    /// Measured Frequency
    ///
    /// Measured Frequency
    fn measured_frequency(&self) -> Option<i32> {
        Some(self.measured_frequency)
    }

    /// Measured Current Phase A
    ///
    /// Measured Current Phase A
    fn measured_current_phase_a(&self) -> Option<i32> {
        Some(self.measured_current_phase_a)
    }

    /// Measured Current Phase B
    ///
    /// Measured Current Phase B
    fn measured_current_phase_b(&self) -> Option<i32> {
        Some(self.measured_current_phase_b)
    }

    /// Measured Current Phase C
    ///
    /// Measured Current Phase C
    fn measured_current_phase_c(&self) -> Option<i32> {
        Some(self.measured_current_phase_c)
    }

    /// Voltage Harmonics Phase A
    ///
    /// Voltage Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_a(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.voltage_harmonics_phase_a.as_ptr()) })
    }

    /// Voltage Harmonics Phase B
    ///
    /// Voltage Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_b(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.voltage_harmonics_phase_b.as_ptr()) })
    }

    /// Voltage Harmonics Phase C
    ///
    /// Voltage Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_c(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.voltage_harmonics_phase_c.as_ptr()) })
    }

    /// Current Harmonics Phase A
    ///
    /// Current Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_a(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.current_harmonics_phase_a.as_ptr()) })
    }

    /// Current Harmonics Phase B
    ///
    /// Current Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_b(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.current_harmonics_phase_b.as_ptr()) })
    }

    /// Current Harmonics Phase C
    ///
    /// Current Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_c(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.current_harmonics_phase_c.as_ptr()) })
    }

    /// Current Interharmonics Phase A
    ///
    /// Current Interharmonics Pct, Phase A (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_a(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.current_interharmonics_phase_a.as_ptr()) })
    }

    /// Current Interharmonics Phase B
    ///
    /// Current Interharmonics Pct, Phase B (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_b(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.current_interharmonics_phase_b.as_ptr()) })
    }

    /// Current Interharmonics Phase C
    ///
    /// Current Interharmonics Pct, Phase C (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_c(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.current_interharmonics_phase_c.as_ptr()) })
    }

    /// Voltage THD Phase A
    ///
    /// Voltage THD Phase A
    fn voltage_thd_phase_a(&self) -> Option<u16> {
        Some(self.voltage_thd_phase_a)
    }

    /// Voltage THD Phase B
    ///
    /// Voltage THD Phase B
    fn voltage_thd_phase_b(&self) -> Option<u16> {
        Some(self.voltage_thd_phase_b)
    }

    /// Voltage THD Phase C
    ///
    /// Voltage THD Phase C
    fn voltage_thd_phase_c(&self) -> Option<u16> {
        Some(self.voltage_thd_phase_c)
    }

    /// Current THD Phase A
    ///
    /// Current THD Phase A
    fn current_thd_phase_a(&self) -> Option<u16> {
        Some(self.current_thd_phase_a)
    }

    /// Current THD Phase B
    ///
    /// Current THD Phase B
    fn current_thd_phase_b(&self) -> Option<u16> {
        Some(self.current_thd_phase_b)
    }

    /// Current THD Phase C
    ///
    /// Current THD Phase C
    fn current_thd_phase_c(&self) -> Option<u16> {
        Some(self.current_thd_phase_c)
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn enable_profile(&self) -> Option<EnaProf> {
        Some(self.enable_profile)
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn set_enable_profile(&mut self, value: EnaProf) {
        self.enable_profile = value;
    }

    /// Profile Result
    ///
    /// Result of last profile operation.
    fn profile_result(&self) -> ProfRslt {
        self.profile_result
    }

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn stored_profile_count(&self) -> u16 {
        self.stored_profile_count
    }

    /// Max Profile Point Count
    ///
    /// Max profile points in the profiles.
    fn max_profile_point_count(&self) -> u16 {
        self.max_profile_point_count
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        self.voltage_scale_factor
    }

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn current_scale_factor(&self) -> u16 {
        self.current_scale_factor
    }

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn time_scale_factor(&self) -> u16 {
        self.time_scale_factor
    }

    /// Frequency Scale Factor
    ///
    /// Scale factor for frequency points.
    fn frequency_scale_factor(&self) -> u16 {
        self.frequency_scale_factor
    }

    /// Frequency Slew Rate Scale Factor
    ///
    /// Scale factor for frequency slew rate.
    fn frequency_slew_rate_scale_factor(&self) -> u16 {
        self.frequency_slew_rate_scale_factor
    }

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn voltage_slew_rate_scale_factor(&self) -> u16 {
        self.voltage_slew_rate_scale_factor
    }

    /// THD Scale Factor
    ///
    /// Scale factor for THD values.
    fn thd_scale_factor(&self) -> u16 {
        self.thd_scale_factor
    }

    /// Profile Name
    ///
    /// Profile name.
    fn prof_profile_name(&self, prof_index: u16) -> Option<&CStr> {
        Some(unsafe {
            CStr::from_ptr(
                self.stored_ac_profiles[prof_index as usize]
                    .prof_profile_name
                    .as_ptr(),
            )
        })
    }

    /// Profile Name
    ///
    /// Profile name.
    fn set_prof_profile_name(&mut self, value: &CStr, prof_index: u16) {
        for (dest, src) in self.stored_ac_profiles[prof_index as usize]
            .prof_profile_name
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// Active Points
    ///
    /// Number of active points.
    fn prof_active_points(&self, prof_index: u16) -> u16 {
        self.stored_ac_profiles[prof_index as usize].prof_active_points
    }

    /// Active Points
    ///
    /// Number of active points.
    fn set_prof_active_points(&mut self, value: u16, prof_index: u16) {
        self.stored_ac_profiles[prof_index as usize].prof_active_points = value;
    }

    /// Profile Time
    ///
    /// Profile time.
    fn pt_profile_time(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points
                [pt_index as usize]
                .pt_profile_time,
        )
    }

    /// Profile Time
    ///
    /// Profile time.
    fn set_pt_profile_time(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points[pt_index as usize]
            .pt_profile_time = value;
    }

    /// Voltage Point
    ///
    /// Profile voltage phase A point in Volts.
    fn pt_voltage_point(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points
                [pt_index as usize]
                .pt_voltage_point,
        )
    }

    /// Voltage Point
    ///
    /// Profile voltage phase A point in Volts.
    fn set_pt_voltage_point(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points[pt_index as usize]
            .pt_voltage_point = value;
    }

    /// Voltage Point Phase B
    ///
    /// Profile voltage phase B point in Volts.
    fn pt_voltage_point_phase_b(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points
                [pt_index as usize]
                .pt_voltage_point_phase_b,
        )
    }

    /// Voltage Point Phase B
    ///
    /// Profile voltage phase B point in Volts.
    fn set_pt_voltage_point_phase_b(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points[pt_index as usize]
            .pt_voltage_point_phase_b = value;
    }

    /// Voltage Point Phase C
    ///
    /// Profile voltage phase C point in Volts.
    fn pt_voltage_point_phase_c(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points
                [pt_index as usize]
                .pt_voltage_point_phase_c,
        )
    }

    /// Voltage Point Phase C
    ///
    /// Profile voltage phase C point in Volts.
    fn set_pt_voltage_point_phase_c(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points[pt_index as usize]
            .pt_voltage_point_phase_c = value;
    }

    /// Frequency Point
    ///
    /// Profile frequency point in Hz.
    fn pt_frequency_point(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points
                [pt_index as usize]
                .pt_frequency_point,
        )
    }

    /// Frequency Point
    ///
    /// Profile frequency point in Hz.
    fn set_pt_frequency_point(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points[pt_index as usize]
            .pt_frequency_point = value;
    }

    /// Phase Angle A
    ///
    /// Profile phase A angle in degrees.
    fn pt_phase_angle_a(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points
                [pt_index as usize]
                .pt_phase_angle_a,
        )
    }

    /// Phase Angle A
    ///
    /// Profile phase A angle in degrees.
    fn set_pt_phase_angle_a(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points[pt_index as usize]
            .pt_phase_angle_a = value;
    }

    /// Phase Angle B
    ///
    /// Profile phase B angle in degrees.
    fn pt_phase_angle_b(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points
                [pt_index as usize]
                .pt_phase_angle_b,
        )
    }

    /// Phase Angle B
    ///
    /// Profile phase B angle in degrees.
    fn set_pt_phase_angle_b(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points[pt_index as usize]
            .pt_phase_angle_b = value;
    }

    /// Phase Angle C
    ///
    /// Profile phase C angle in degrees.
    fn pt_phase_angle_c(&self, prof_index: u16, pt_index: u16) -> Option<u16> {
        Some(
            self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points
                [pt_index as usize]
                .pt_phase_angle_c,
        )
    }

    /// Phase Angle C
    ///
    /// Profile phase C angle in degrees.
    fn set_pt_phase_angle_c(&mut self, value: u16, prof_index: u16, pt_index: u16) {
        self.stored_ac_profiles[prof_index as usize].stored_ac_profile_points[pt_index as usize]
            .pt_phase_angle_c = value;
    }
}
