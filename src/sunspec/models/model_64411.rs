use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 1398;

pub static POINTS: [ReadablePoint; 50] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64411 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 1396 },
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
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ActivePhases => {
            if let Some(value) = model.active_phases() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PhaseAngle => {
            if let Some(value) = model.phase_angle() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::NominalVoltage => {
            if let Some(value) = model.nominal_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaximumVoltage => {
            if let Some(value) = model.maximum_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaximumCurrent => {
            if let Some(value) = model.maximum_current() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Frequency => {
            if let Some(value) = model.frequency() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::OutputState => {
            if let Some(value) = model.output_state() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::RelayState => {
            if let Some(value) = model.relay_state() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::RegenerationState => {
            if let Some(value) = model.regeneration_state() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::VoltageSetpoint => {
            if let Some(value) = model.voltage_setpoint() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageSetpointPhaseA => {
            if let Some(value) = model.voltage_setpoint_phase_a() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageSetpointPhaseB => {
            if let Some(value) = model.voltage_setpoint_phase_b() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageSetpointPhaseC => {
            if let Some(value) = model.voltage_setpoint_phase_c() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::FrequencySlewRate => {
            if let Some(value) = model.frequency_slew_rate() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageSlewRate => {
            if let Some(value) = model.voltage_slew_rate() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MeasuredVoltagePhaseA => {
            if let Some(value) = model.measured_voltage_phase_a() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::MeasuredVoltagePhaseB => {
            if let Some(value) = model.measured_voltage_phase_b() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::MeasuredVoltagePhaseC => {
            if let Some(value) = model.measured_voltage_phase_c() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::MeasuredFrequency => {
            if let Some(value) = model.measured_frequency() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::MeasuredCurrentPhaseA => {
            if let Some(value) = model.measured_current_phase_a() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::MeasuredCurrentPhaseB => {
            if let Some(value) = model.measured_current_phase_b() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::MeasuredCurrentPhaseC => {
            if let Some(value) = model.measured_current_phase_c() {
                serialisation::write_i32(value, buffer, offset, limit);
            }
        }
        Point::VoltageHarmonicsPhaseA => {
            if let Some(value) = model.voltage_harmonics_phase_a() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::VoltageHarmonicsPhaseB => {
            if let Some(value) = model.voltage_harmonics_phase_b() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::VoltageHarmonicsPhaseC => {
            if let Some(value) = model.voltage_harmonics_phase_c() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::CurrentHarmonicsPhaseA => {
            if let Some(value) = model.current_harmonics_phase_a() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::CurrentHarmonicsPhaseB => {
            if let Some(value) = model.current_harmonics_phase_b() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::CurrentHarmonicsPhaseC => {
            if let Some(value) = model.current_harmonics_phase_c() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::CurrentInterharmonicsPhaseA => {
            if let Some(value) = model.current_interharmonics_phase_a() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::CurrentInterharmonicsPhaseB => {
            if let Some(value) = model.current_interharmonics_phase_b() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::CurrentInterharmonicsPhaseC => {
            if let Some(value) = model.current_interharmonics_phase_c() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::VoltageThdPhaseA => {
            if let Some(value) = model.voltage_thd_phase_a() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageThdPhaseB => {
            if let Some(value) = model.voltage_thd_phase_b() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VoltageThdPhaseC => {
            if let Some(value) = model.voltage_thd_phase_c() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::CurrentThdPhaseA => {
            if let Some(value) = model.current_thd_phase_a() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::CurrentThdPhaseB => {
            if let Some(value) = model.current_thd_phase_b() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::CurrentThdPhaseC => {
            if let Some(value) = model.current_thd_phase_c() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::EnableProfile => {
            if let Some(value) = model.enable_profile() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::ProfileResult => serialisation::write_u16(model.profile_result() as u16, buffer),
        Point::StoredProfileCount => serialisation::write_u16(model.stored_profile_count(), buffer),
        Point::MaxProfilePointCount => {
            serialisation::write_u16(model.max_profile_point_count(), buffer)
        }
        Point::VoltageScaleFactor => serialisation::write_u16(model.voltage_scale_factor(), buffer),
        Point::CurrentScaleFactor => serialisation::write_u16(model.current_scale_factor(), buffer),
        Point::TimeScaleFactor => serialisation::write_u16(model.time_scale_factor(), buffer),
        Point::FrequencyScaleFactor => {
            serialisation::write_u16(model.frequency_scale_factor(), buffer)
        }
        Point::FrequencySlewRateScaleFactor => {
            serialisation::write_u16(model.frequency_slew_rate_scale_factor(), buffer)
        }
        Point::VoltageSlewRateScaleFactor => {
            serialisation::write_u16(model.voltage_slew_rate_scale_factor(), buffer)
        }
        Point::ThdScaleFactor => serialisation::write_u16(model.thd_scale_factor(), buffer),
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
}

pub enum Output {
    /// Output Off
    Off = 0,
    /// Output On
    On = 1,
}

pub enum Relay {
    /// Relay Open
    Open = 0,
    /// Relay Closed
    Closed = 1,
}

pub enum Regen {
    /// Regen Off
    Off = 0,
    /// Regen On
    On = 1,
}

pub enum EnaProf {
    /// Stop Profile
    Stop = 0,
    /// Start Profile Immediately
    Start = 1,
    /// Start Profile via External Trigger Signal
    Trigger = 2,
}

pub enum ProfRslt {
    /// Profile update in progress.
    InProgress = 0,
    /// Profile update completed successfully.
    Completed = 1,
    /// Profile update failed.
    Failed = 2,
}

#[repr(C)]
pub struct Model64411CallbackAdapter {
    active_phases_callback: Option<extern "C" fn() -> u16>,
    set_active_phases_callback: Option<extern "C" fn(u16)>,
    phase_angle_callback: Option<extern "C" fn() -> u16>,
    set_phase_angle_callback: Option<extern "C" fn(u16)>,
    nominal_voltage_callback: Option<extern "C" fn() -> u16>,
    set_nominal_voltage_callback: Option<extern "C" fn(u16)>,
    maximum_voltage_callback: Option<extern "C" fn() -> u16>,
    set_maximum_voltage_callback: Option<extern "C" fn(u16)>,
    maximum_current_callback: Option<extern "C" fn() -> u16>,
    set_maximum_current_callback: Option<extern "C" fn(u16)>,
    frequency_callback: Option<extern "C" fn() -> u16>,
    set_frequency_callback: Option<extern "C" fn(u16)>,
    output_state_callback: Option<extern "C" fn() -> Output>,
    set_output_state_callback: Option<extern "C" fn(Output)>,
    relay_state_callback: Option<extern "C" fn() -> Relay>,
    set_relay_state_callback: Option<extern "C" fn(Relay)>,
    regeneration_state_callback: Option<extern "C" fn() -> Regen>,
    set_regeneration_state_callback: Option<extern "C" fn(Regen)>,
    voltage_setpoint_callback: Option<extern "C" fn() -> u16>,
    set_voltage_setpoint_callback: Option<extern "C" fn(u16)>,
    voltage_setpoint_phase_a_callback: Option<extern "C" fn() -> u16>,
    set_voltage_setpoint_phase_a_callback: Option<extern "C" fn(u16)>,
    voltage_setpoint_phase_b_callback: Option<extern "C" fn() -> u16>,
    set_voltage_setpoint_phase_b_callback: Option<extern "C" fn(u16)>,
    voltage_setpoint_phase_c_callback: Option<extern "C" fn() -> u16>,
    set_voltage_setpoint_phase_c_callback: Option<extern "C" fn(u16)>,
    frequency_slew_rate_callback: Option<extern "C" fn() -> u16>,
    set_frequency_slew_rate_callback: Option<extern "C" fn(u16)>,
    voltage_slew_rate_callback: Option<extern "C" fn() -> u16>,
    set_voltage_slew_rate_callback: Option<extern "C" fn(u16)>,
    measured_voltage_phase_a_callback: Option<extern "C" fn() -> i32>,
    measured_voltage_phase_b_callback: Option<extern "C" fn() -> i32>,
    measured_voltage_phase_c_callback: Option<extern "C" fn() -> i32>,
    measured_frequency_callback: Option<extern "C" fn() -> i32>,
    measured_current_phase_a_callback: Option<extern "C" fn() -> i32>,
    measured_current_phase_b_callback: Option<extern "C" fn() -> i32>,
    measured_current_phase_c_callback: Option<extern "C" fn() -> i32>,
    voltage_harmonics_phase_a_callback: Option<extern "C" fn() -> *const c_char>,
    voltage_harmonics_phase_b_callback: Option<extern "C" fn() -> *const c_char>,
    voltage_harmonics_phase_c_callback: Option<extern "C" fn() -> *const c_char>,
    current_harmonics_phase_a_callback: Option<extern "C" fn() -> *const c_char>,
    current_harmonics_phase_b_callback: Option<extern "C" fn() -> *const c_char>,
    current_harmonics_phase_c_callback: Option<extern "C" fn() -> *const c_char>,
    current_interharmonics_phase_a_callback: Option<extern "C" fn() -> *const c_char>,
    current_interharmonics_phase_b_callback: Option<extern "C" fn() -> *const c_char>,
    current_interharmonics_phase_c_callback: Option<extern "C" fn() -> *const c_char>,
    voltage_thd_phase_a_callback: Option<extern "C" fn() -> u16>,
    voltage_thd_phase_b_callback: Option<extern "C" fn() -> u16>,
    voltage_thd_phase_c_callback: Option<extern "C" fn() -> u16>,
    current_thd_phase_a_callback: Option<extern "C" fn() -> u16>,
    current_thd_phase_b_callback: Option<extern "C" fn() -> u16>,
    current_thd_phase_c_callback: Option<extern "C" fn() -> u16>,
    enable_profile_callback: Option<extern "C" fn() -> EnaProf>,
    set_enable_profile_callback: Option<extern "C" fn(EnaProf)>,
    profile_result_callback: extern "C" fn() -> ProfRslt,
    stored_profile_count_callback: extern "C" fn() -> u16,
    max_profile_point_count_callback: extern "C" fn() -> u16,
    voltage_scale_factor_callback: extern "C" fn() -> u16,
    current_scale_factor_callback: extern "C" fn() -> u16,
    time_scale_factor_callback: extern "C" fn() -> u16,
    frequency_scale_factor_callback: extern "C" fn() -> u16,
    frequency_slew_rate_scale_factor_callback: extern "C" fn() -> u16,
    voltage_slew_rate_scale_factor_callback: extern "C" fn() -> u16,
    thd_scale_factor_callback: extern "C" fn() -> u16,
}

impl ModelAdapter for Model64411CallbackAdapter {
    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn active_phases(&self) -> Option<u16> {
        self.active_phases_callback.map(|callback| (callback)())
    }

    /// Active Phases
    ///
    /// Set the number of active phases for the power supply
    fn set_active_phases(&mut self, value: u16) {
        if let Some(callback) = self.set_active_phases_callback {
            (callback)(value);
        };
    }

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn phase_angle(&self) -> Option<u16> {
        self.phase_angle_callback.map(|callback| (callback)())
    }

    /// Phase Angle
    ///
    /// Phase angle (deg) between phases. 0 for signle phase, 120 for two phase, 120 for three phase.
    fn set_phase_angle(&mut self, value: u16) {
        if let Some(callback) = self.set_phase_angle_callback {
            (callback)(value);
        };
    }

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn nominal_voltage(&self) -> Option<u16> {
        self.nominal_voltage_callback.map(|callback| (callback)())
    }

    /// Nominal Voltage
    ///
    /// Nominal L-N Voltage
    fn set_nominal_voltage(&mut self, value: u16) {
        if let Some(callback) = self.set_nominal_voltage_callback {
            (callback)(value);
        };
    }

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn maximum_voltage(&self) -> Option<u16> {
        self.maximum_voltage_callback.map(|callback| (callback)())
    }

    /// Maximum Voltage
    ///
    /// Maximum Voltage Protection Level
    fn set_maximum_voltage(&mut self, value: u16) {
        if let Some(callback) = self.set_maximum_voltage_callback {
            (callback)(value);
        };
    }

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn maximum_current(&self) -> Option<u16> {
        self.maximum_current_callback.map(|callback| (callback)())
    }

    /// Maximum Current
    ///
    /// Maximum Current Protection Level
    fn set_maximum_current(&mut self, value: u16) {
        if let Some(callback) = self.set_maximum_current_callback {
            (callback)(value);
        };
    }

    /// Frequency
    ///
    /// Frequency Setpoint
    fn frequency(&self) -> Option<u16> {
        self.frequency_callback.map(|callback| (callback)())
    }

    /// Frequency
    ///
    /// Frequency Setpoint
    fn set_frequency(&mut self, value: u16) {
        if let Some(callback) = self.set_frequency_callback {
            (callback)(value);
        };
    }

    /// Output State
    ///
    /// AC Output State
    fn output_state(&self) -> Option<Output> {
        self.output_state_callback.map(|callback| (callback)())
    }

    /// Output State
    ///
    /// AC Output State
    fn set_output_state(&mut self, value: Output) {
        if let Some(callback) = self.set_output_state_callback {
            (callback)(value);
        };
    }

    /// Relay State
    ///
    /// AC Relay State
    fn relay_state(&self) -> Option<Relay> {
        self.relay_state_callback.map(|callback| (callback)())
    }

    /// Relay State
    ///
    /// AC Relay State
    fn set_relay_state(&mut self, value: Relay) {
        if let Some(callback) = self.set_relay_state_callback {
            (callback)(value);
        };
    }

    /// Regeneration State
    ///
    /// Regeneration State
    fn regeneration_state(&self) -> Option<Regen> {
        self.regeneration_state_callback
            .map(|callback| (callback)())
    }

    /// Regeneration State
    ///
    /// Regeneration State
    fn set_regeneration_state(&mut self, value: Regen) {
        if let Some(callback) = self.set_regeneration_state_callback {
            (callback)(value);
        };
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn voltage_setpoint(&self) -> Option<u16> {
        self.voltage_setpoint_callback.map(|callback| (callback)())
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint (all phases)
    fn set_voltage_setpoint(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_callback {
            (callback)(value);
        };
    }

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn voltage_setpoint_phase_a(&self) -> Option<u16> {
        self.voltage_setpoint_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Voltage Setpoint Phase A
    ///
    /// Voltage Setpoint Phase A
    fn set_voltage_setpoint_phase_a(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_phase_a_callback {
            (callback)(value);
        };
    }

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn voltage_setpoint_phase_b(&self) -> Option<u16> {
        self.voltage_setpoint_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Voltage Setpoint Phase B
    ///
    /// Voltage Setpoint Phase B
    fn set_voltage_setpoint_phase_b(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_phase_b_callback {
            (callback)(value);
        };
    }

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn voltage_setpoint_phase_c(&self) -> Option<u16> {
        self.voltage_setpoint_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Voltage Setpoint Phase C
    ///
    /// Voltage Setpoint Phase C
    fn set_voltage_setpoint_phase_c(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_phase_c_callback {
            (callback)(value);
        };
    }

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn frequency_slew_rate(&self) -> Option<u16> {
        self.frequency_slew_rate_callback
            .map(|callback| (callback)())
    }

    /// Frequency Slew Rate
    ///
    /// Frequency Slew Rate
    fn set_frequency_slew_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_frequency_slew_rate_callback {
            (callback)(value);
        };
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn voltage_slew_rate(&self) -> Option<u16> {
        self.voltage_slew_rate_callback.map(|callback| (callback)())
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn set_voltage_slew_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_slew_rate_callback {
            (callback)(value);
        };
    }

    /// Measured Voltage Phase A
    ///
    /// Measured Voltage Phase A
    fn measured_voltage_phase_a(&self) -> Option<i32> {
        self.measured_voltage_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Measured Voltage Phase B
    ///
    /// Measured Voltage Phase B
    fn measured_voltage_phase_b(&self) -> Option<i32> {
        self.measured_voltage_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Measured Voltage Phase C
    ///
    /// Measured Voltage Phase C
    fn measured_voltage_phase_c(&self) -> Option<i32> {
        self.measured_voltage_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Measured Frequency
    ///
    /// Measured Frequency
    fn measured_frequency(&self) -> Option<i32> {
        self.measured_frequency_callback
            .map(|callback| (callback)())
    }

    /// Measured Current Phase A
    ///
    /// Measured Current Phase A
    fn measured_current_phase_a(&self) -> Option<i32> {
        self.measured_current_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Measured Current Phase B
    ///
    /// Measured Current Phase B
    fn measured_current_phase_b(&self) -> Option<i32> {
        self.measured_current_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Measured Current Phase C
    ///
    /// Measured Current Phase C
    fn measured_current_phase_c(&self) -> Option<i32> {
        self.measured_current_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Voltage Harmonics Phase A
    ///
    /// Voltage Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_a(&self) -> Option<&CStr> {
        self.voltage_harmonics_phase_a_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Voltage Harmonics Phase B
    ///
    /// Voltage Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_b(&self) -> Option<&CStr> {
        self.voltage_harmonics_phase_b_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Voltage Harmonics Phase C
    ///
    /// Voltage Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn voltage_harmonics_phase_c(&self) -> Option<&CStr> {
        self.voltage_harmonics_phase_c_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Current Harmonics Phase A
    ///
    /// Current Harmonics Pct, Phase A (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_a(&self) -> Option<&CStr> {
        self.current_harmonics_phase_a_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Current Harmonics Phase B
    ///
    /// Current Harmonics Pct, Phase B (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_b(&self) -> Option<&CStr> {
        self.current_harmonics_phase_b_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Current Harmonics Phase C
    ///
    /// Current Harmonics Pct, Phase C (comma seperated string for harmonics 1-50)
    fn current_harmonics_phase_c(&self) -> Option<&CStr> {
        self.current_harmonics_phase_c_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Current Interharmonics Phase A
    ///
    /// Current Interharmonics Pct, Phase A (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_a(&self) -> Option<&CStr> {
        self.current_interharmonics_phase_a_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Current Interharmonics Phase B
    ///
    /// Current Interharmonics Pct, Phase B (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_b(&self) -> Option<&CStr> {
        self.current_interharmonics_phase_b_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Current Interharmonics Phase C
    ///
    /// Current Interharmonics Pct, Phase C (comma seperated string for interharmonics 1-50)
    fn current_interharmonics_phase_c(&self) -> Option<&CStr> {
        self.current_interharmonics_phase_c_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Voltage THD Phase A
    ///
    /// Voltage THD Phase A
    fn voltage_thd_phase_a(&self) -> Option<u16> {
        self.voltage_thd_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Voltage THD Phase B
    ///
    /// Voltage THD Phase B
    fn voltage_thd_phase_b(&self) -> Option<u16> {
        self.voltage_thd_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Voltage THD Phase C
    ///
    /// Voltage THD Phase C
    fn voltage_thd_phase_c(&self) -> Option<u16> {
        self.voltage_thd_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Current THD Phase A
    ///
    /// Current THD Phase A
    fn current_thd_phase_a(&self) -> Option<u16> {
        self.current_thd_phase_a_callback
            .map(|callback| (callback)())
    }

    /// Current THD Phase B
    ///
    /// Current THD Phase B
    fn current_thd_phase_b(&self) -> Option<u16> {
        self.current_thd_phase_b_callback
            .map(|callback| (callback)())
    }

    /// Current THD Phase C
    ///
    /// Current THD Phase C
    fn current_thd_phase_c(&self) -> Option<u16> {
        self.current_thd_phase_c_callback
            .map(|callback| (callback)())
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn enable_profile(&self) -> Option<EnaProf> {
        self.enable_profile_callback.map(|callback| (callback)())
    }

    /// Enable Profile
    ///
    /// Start/Stop the AC Profile
    fn set_enable_profile(&mut self, value: EnaProf) {
        if let Some(callback) = self.set_enable_profile_callback {
            (callback)(value);
        };
    }

    /// Profile Result
    ///
    /// Result of last profile operation.
    fn profile_result(&self) -> ProfRslt {
        (self.profile_result_callback)()
    }

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn stored_profile_count(&self) -> u16 {
        (self.stored_profile_count_callback)()
    }

    /// Max Profile Point Count
    ///
    /// Max profile points in the profiles.
    fn max_profile_point_count(&self) -> u16 {
        (self.max_profile_point_count_callback)()
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        (self.voltage_scale_factor_callback)()
    }

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn current_scale_factor(&self) -> u16 {
        (self.current_scale_factor_callback)()
    }

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn time_scale_factor(&self) -> u16 {
        (self.time_scale_factor_callback)()
    }

    /// Frequency Scale Factor
    ///
    /// Scale factor for frequency points.
    fn frequency_scale_factor(&self) -> u16 {
        (self.frequency_scale_factor_callback)()
    }

    /// Frequency Slew Rate Scale Factor
    ///
    /// Scale factor for frequency slew rate.
    fn frequency_slew_rate_scale_factor(&self) -> u16 {
        (self.frequency_slew_rate_scale_factor_callback)()
    }

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn voltage_slew_rate_scale_factor(&self) -> u16 {
        (self.voltage_slew_rate_scale_factor_callback)()
    }

    /// THD Scale Factor
    ///
    /// Scale factor for THD values.
    fn thd_scale_factor(&self) -> u16 {
        (self.thd_scale_factor_callback)()
    }
}
