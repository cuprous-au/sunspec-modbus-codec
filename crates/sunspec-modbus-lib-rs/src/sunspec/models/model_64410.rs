use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 70;

pub static POINTS: [ReadablePoint; 36] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64410 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::MaximumVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::MaximumPower,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::MaximumCurrent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::CvOrCcMode,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::PowerOnOff,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::ResetDevice,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::VoltageSetpoint,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::PowerSetpoint,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::CurrentSetpoint,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::En50530Mode,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::En50530MppVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::En50530MppPower,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::IrradianceSetpoint,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::VoltageSlewRate,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::PowerSlewRate,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::CurrentSlewRate,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::EnableProfile,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::ProfileAdoptionRequest,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::AdoptProfileResult,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::MeasuredVoltage,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::MeasuredPower,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::MeasuredCurrent,
        },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::Errors,
        },
        size: 32,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::NumberOfPoints,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::StoredProfileCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::PowerScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::CurrentScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::IrradianceScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::TimeScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::VoltageSlewRateScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::PowerSlewRateScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::CurrentSlewRateScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 {
            point: Point::PercentScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    MaximumVoltage,
    MaximumPower,
    MaximumCurrent,
    CvOrCcMode,
    PowerOnOff,
    ResetDevice,
    VoltageSetpoint,
    PowerSetpoint,
    CurrentSetpoint,
    En50530Mode,
    En50530MppVoltage,
    En50530MppPower,
    IrradianceSetpoint,
    VoltageSlewRate,
    PowerSlewRate,
    CurrentSlewRate,
    EnableProfile,
    ProfileAdoptionRequest,
    AdoptProfileResult,
    MeasuredVoltage,
    MeasuredPower,
    MeasuredCurrent,
    Errors,
    NumberOfPoints,
    StoredProfileCount,
    PowerScaleFactor,
    VoltageScaleFactor,
    CurrentScaleFactor,
    IrradianceScaleFactor,
    TimeScaleFactor,
    VoltageSlewRateScaleFactor,
    PowerSlewRateScaleFactor,
    CurrentSlewRateScaleFactor,
    PercentScaleFactor,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    70 + model.stored_profile_count() * (3 + model.number_of_points() * (5))
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
        Point::MaximumVoltage => {
            if let Some(value) = model.maximum_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaximumPower => {
            if let Some(value) = model.maximum_power() {
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
        Point::CvOrCcMode => {
            if let Some(value) = model.cv_or_cc_mode() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerOnOff => {
            if let Some(value) = model.power_on_off() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ResetDevice => {
            if let Some(value) = model.reset_device() {
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
        Point::PowerSetpoint => {
            if let Some(value) = model.power_setpoint() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentSetpoint => {
            if let Some(value) = model.current_setpoint() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::En50530Mode => {
            if let Some(value) = model.en50530_mode() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::En50530MppVoltage => {
            if let Some(value) = model.en50530_mpp_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::En50530MppPower => {
            if let Some(value) = model.en50530_mpp_power() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::IrradianceSetpoint => {
            if let Some(value) = model.irradiance_setpoint() {
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
        Point::PowerSlewRate => {
            if let Some(value) = model.power_slew_rate() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CurrentSlewRate => {
            if let Some(value) = model.current_slew_rate() {
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
        Point::ProfileAdoptionRequest => {
            if let Some(value) = model.profile_adoption_request() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AdoptProfileResult => {
            buffer::write_u16(model.adopt_profile_result() as u16, buffer);
        }
        Point::MeasuredVoltage => {
            if let Some(value) = model.measured_voltage() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredPower => {
            if let Some(value) = model.measured_power() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MeasuredCurrent => {
            if let Some(value) = model.measured_current() {
                buffer::write_i32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Errors => {
            if let Some(value) = model.errors() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NumberOfPoints => {
            buffer::write_u16(model.number_of_points(), buffer);
        }
        Point::StoredProfileCount => {
            buffer::write_u16(model.stored_profile_count(), buffer);
        }
        Point::PowerScaleFactor => {
            buffer::write_u16(model.power_scale_factor(), buffer);
        }
        Point::VoltageScaleFactor => {
            buffer::write_u16(model.voltage_scale_factor(), buffer);
        }
        Point::CurrentScaleFactor => {
            buffer::write_u16(model.current_scale_factor(), buffer);
        }
        Point::IrradianceScaleFactor => {
            buffer::write_u16(model.irradiance_scale_factor(), buffer);
        }
        Point::TimeScaleFactor => {
            buffer::write_u16(model.time_scale_factor(), buffer);
        }
        Point::VoltageSlewRateScaleFactor => {
            buffer::write_u16(model.voltage_slew_rate_scale_factor(), buffer);
        }
        Point::PowerSlewRateScaleFactor => {
            buffer::write_u16(model.power_slew_rate_scale_factor(), buffer);
        }
        Point::CurrentSlewRateScaleFactor => {
            buffer::write_u16(model.current_slew_rate_scale_factor(), buffer);
        }
        Point::PercentScaleFactor => {
            buffer::write_u16(model.percent_scale_factor(), buffer);
        }
    }
}

pub trait ModelAdapter {
    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    fn maximum_voltage(&self) -> Option<u16> {
        None
    }

    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    fn set_maximum_voltage(&mut self, value: u16) {}

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn maximum_power(&self) -> Option<u16> {
        None
    }

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn set_maximum_power(&mut self, value: u16) {}

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn maximum_current(&self) -> Option<u16> {
        None
    }

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn set_maximum_current(&mut self, value: u16) {}

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn cv_or_cc_mode(&self) -> Option<Mode> {
        None
    }

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn set_cv_or_cc_mode(&mut self, value: Mode) {}

    /// Power On/Off
    ///
    /// Power On/Off
    fn power_on_off(&self) -> Option<Ena> {
        None
    }

    /// Power On/Off
    ///
    /// Power On/Off
    fn set_power_on_off(&mut self, value: Ena) {}

    /// Reset Device
    ///
    /// Reset Device
    fn reset_device(&self) -> Option<Reset> {
        None
    }

    /// Reset Device
    ///
    /// Reset Device
    fn set_reset_device(&mut self, value: Reset) {}

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn voltage_setpoint(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn set_voltage_setpoint(&mut self, value: u16) {}

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn power_setpoint(&self) -> Option<u16> {
        None
    }

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn set_power_setpoint(&mut self, value: u16) {}

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn current_setpoint(&self) -> Option<u16> {
        None
    }

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn set_current_setpoint(&mut self, value: u16) {}

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn en50530_mode(&self) -> Option<En50530> {
        None
    }

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn set_en50530_mode(&mut self, value: En50530) {}

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn en50530_mpp_voltage(&self) -> Option<u16> {
        None
    }

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn set_en50530_mpp_voltage(&mut self, value: u16) {}

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn en50530_mpp_power(&self) -> Option<u16> {
        None
    }

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn set_en50530_mpp_power(&mut self, value: u16) {}

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn irradiance_setpoint(&self) -> Option<u16> {
        None
    }

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn set_irradiance_setpoint(&mut self, value: u16) {}

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

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn power_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn set_power_slew_rate(&mut self, value: u16) {}

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn current_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn set_current_slew_rate(&mut self, value: u16) {}

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn enable_profile(&self) -> Option<EnaProf> {
        None
    }

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn set_enable_profile(&mut self, value: EnaProf) {}

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn profile_adoption_request(&self) -> Option<u16> {
        None
    }

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn set_profile_adoption_request(&mut self, value: u16) {}

    /// Adopt Profile Result
    ///
    /// Result of last adopt profile operation.
    fn adopt_profile_result(&self) -> AdptProfRslt;

    /// Measured Voltage
    ///
    /// Measured Voltage
    fn measured_voltage(&self) -> Option<i32> {
        None
    }

    /// Measured Power
    ///
    /// Measured Power
    fn measured_power(&self) -> Option<i32> {
        None
    }

    /// Measured Current
    ///
    /// Measured Current
    fn measured_current(&self) -> Option<i32> {
        None
    }

    /// Errors
    ///
    /// Error States
    fn errors(&self) -> Option<&CStr> {
        None
    }

    /// Number Of Points
    ///
    /// Number of profile points supported.
    fn number_of_points(&self) -> u16;

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn stored_profile_count(&self) -> u16;

    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    fn power_scale_factor(&self) -> u16;

    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    fn set_power_scale_factor(&mut self, value: u16);

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn voltage_scale_factor(&self) -> u16;

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn set_voltage_scale_factor(&mut self, value: u16);

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn current_scale_factor(&self) -> u16;

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn set_current_scale_factor(&mut self, value: u16);

    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    fn irradiance_scale_factor(&self) -> u16;

    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    fn set_irradiance_scale_factor(&mut self, value: u16);

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn time_scale_factor(&self) -> u16;

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn set_time_scale_factor(&mut self, value: u16);

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn voltage_slew_rate_scale_factor(&self) -> u16;

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn set_voltage_slew_rate_scale_factor(&mut self, value: u16);

    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    fn power_slew_rate_scale_factor(&self) -> u16;

    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    fn set_power_slew_rate_scale_factor(&mut self, value: u16);

    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    fn current_slew_rate_scale_factor(&self) -> u16;

    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    fn set_current_slew_rate_scale_factor(&mut self, value: u16);

    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    fn percent_scale_factor(&self) -> u16;

    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    fn set_percent_scale_factor(&mut self, value: u16);
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum AdptProfRslt {
    /// Update In Progress
    ///
    /// Profile update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Profile update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Profile update failed.
    Failed = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum En50530 {
    /// EN50530 Mode
    ///
    /// EN50530 Mode
    En50530 = 1,
    /// Do Not Use EN50530 Mode
    ///
    /// Do Not Use EN50530 Mode
    DoNotEn50530 = 0,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Ena {
    /// Power On
    ///
    /// Power On
    On = 1,
    /// Power Off
    ///
    /// Power Off
    Off = 0,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum EnaProf {
    /// Start Profile
    ///
    /// Start the Profile
    Start = 1,
    /// Stop Profile
    ///
    /// Stop the Profile
    Stop = 0,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Mode {
    /// CV Mode
    ///
    /// Constant Voltage (CV) Mode
    Cv = 0,
    /// CC Mode
    ///
    /// Constant Current (CC) Mode.
    Cc = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Reset {
    /// Reset Device
    ///
    /// Reset Device
    Reset = 1,
    /// Do Not Reset Device
    ///
    /// Do Not Reset Device
    DoNotReset = 0,
}

#[repr(C)]
pub struct Model64410CallbackAdapter {
    context: *mut c_void,
    maximum_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_maximum_voltage_callback: Option<extern "C" fn(u16, *mut c_void)>,
    maximum_power_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_maximum_power_callback: Option<extern "C" fn(u16, *mut c_void)>,
    maximum_current_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_maximum_current_callback: Option<extern "C" fn(u16, *mut c_void)>,
    cv_or_cc_mode_callback: Option<extern "C" fn(*const c_void) -> Mode>,
    set_cv_or_cc_mode_callback: Option<extern "C" fn(Mode, *mut c_void)>,
    power_on_off_callback: Option<extern "C" fn(*const c_void) -> Ena>,
    set_power_on_off_callback: Option<extern "C" fn(Ena, *mut c_void)>,
    reset_device_callback: Option<extern "C" fn(*const c_void) -> Reset>,
    set_reset_device_callback: Option<extern "C" fn(Reset, *mut c_void)>,
    voltage_setpoint_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_voltage_setpoint_callback: Option<extern "C" fn(u16, *mut c_void)>,
    power_setpoint_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_power_setpoint_callback: Option<extern "C" fn(u16, *mut c_void)>,
    current_setpoint_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_current_setpoint_callback: Option<extern "C" fn(u16, *mut c_void)>,
    en50530_mode_callback: Option<extern "C" fn(*const c_void) -> En50530>,
    set_en50530_mode_callback: Option<extern "C" fn(En50530, *mut c_void)>,
    en50530_mpp_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_en50530_mpp_voltage_callback: Option<extern "C" fn(u16, *mut c_void)>,
    en50530_mpp_power_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_en50530_mpp_power_callback: Option<extern "C" fn(u16, *mut c_void)>,
    irradiance_setpoint_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_irradiance_setpoint_callback: Option<extern "C" fn(u16, *mut c_void)>,
    voltage_slew_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_voltage_slew_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    power_slew_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_power_slew_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    current_slew_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_current_slew_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    enable_profile_callback: Option<extern "C" fn(*const c_void) -> EnaProf>,
    set_enable_profile_callback: Option<extern "C" fn(EnaProf, *mut c_void)>,
    profile_adoption_request_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_profile_adoption_request_callback: Option<extern "C" fn(u16, *mut c_void)>,
    adopt_profile_result_callback: extern "C" fn(*const c_void) -> AdptProfRslt,
    measured_voltage_callback: Option<extern "C" fn(*const c_void) -> i32>,
    measured_power_callback: Option<extern "C" fn(*const c_void) -> i32>,
    measured_current_callback: Option<extern "C" fn(*const c_void) -> i32>,
    errors_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    number_of_points_callback: extern "C" fn(*const c_void) -> u16,
    stored_profile_count_callback: extern "C" fn(*const c_void) -> u16,
    power_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_power_scale_factor_callback: extern "C" fn(u16, *mut c_void),
    voltage_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_voltage_scale_factor_callback: extern "C" fn(u16, *mut c_void),
    current_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_current_scale_factor_callback: extern "C" fn(u16, *mut c_void),
    irradiance_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_irradiance_scale_factor_callback: extern "C" fn(u16, *mut c_void),
    time_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_time_scale_factor_callback: extern "C" fn(u16, *mut c_void),
    voltage_slew_rate_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_voltage_slew_rate_scale_factor_callback: extern "C" fn(u16, *mut c_void),
    power_slew_rate_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_power_slew_rate_scale_factor_callback: extern "C" fn(u16, *mut c_void),
    current_slew_rate_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_current_slew_rate_scale_factor_callback: extern "C" fn(u16, *mut c_void),
    percent_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    set_percent_scale_factor_callback: extern "C" fn(u16, *mut c_void),
}

impl ModelAdapter for Model64410CallbackAdapter {
    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    fn maximum_voltage(&self) -> Option<u16> {
        self.maximum_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    fn set_maximum_voltage(&mut self, value: u16) {
        if let Some(callback) = self.set_maximum_voltage_callback {
            (callback)(value, self.context);
        };
    }

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn maximum_power(&self) -> Option<u16> {
        self.maximum_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn set_maximum_power(&mut self, value: u16) {
        if let Some(callback) = self.set_maximum_power_callback {
            (callback)(value, self.context);
        };
    }

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn maximum_current(&self) -> Option<u16> {
        self.maximum_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn set_maximum_current(&mut self, value: u16) {
        if let Some(callback) = self.set_maximum_current_callback {
            (callback)(value, self.context);
        };
    }

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn cv_or_cc_mode(&self) -> Option<Mode> {
        self.cv_or_cc_mode_callback
            .map(|callback| (callback)(self.context))
    }

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn set_cv_or_cc_mode(&mut self, value: Mode) {
        if let Some(callback) = self.set_cv_or_cc_mode_callback {
            (callback)(value, self.context);
        };
    }

    /// Power On/Off
    ///
    /// Power On/Off
    fn power_on_off(&self) -> Option<Ena> {
        self.power_on_off_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power On/Off
    ///
    /// Power On/Off
    fn set_power_on_off(&mut self, value: Ena) {
        if let Some(callback) = self.set_power_on_off_callback {
            (callback)(value, self.context);
        };
    }

    /// Reset Device
    ///
    /// Reset Device
    fn reset_device(&self) -> Option<Reset> {
        self.reset_device_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reset Device
    ///
    /// Reset Device
    fn set_reset_device(&mut self, value: Reset) {
        if let Some(callback) = self.set_reset_device_callback {
            (callback)(value, self.context);
        };
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn voltage_setpoint(&self) -> Option<u16> {
        self.voltage_setpoint_callback
            .map(|callback| (callback)(self.context))
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn set_voltage_setpoint(&mut self, value: u16) {
        if let Some(callback) = self.set_voltage_setpoint_callback {
            (callback)(value, self.context);
        };
    }

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn power_setpoint(&self) -> Option<u16> {
        self.power_setpoint_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn set_power_setpoint(&mut self, value: u16) {
        if let Some(callback) = self.set_power_setpoint_callback {
            (callback)(value, self.context);
        };
    }

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn current_setpoint(&self) -> Option<u16> {
        self.current_setpoint_callback
            .map(|callback| (callback)(self.context))
    }

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn set_current_setpoint(&mut self, value: u16) {
        if let Some(callback) = self.set_current_setpoint_callback {
            (callback)(value, self.context);
        };
    }

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn en50530_mode(&self) -> Option<En50530> {
        self.en50530_mode_callback
            .map(|callback| (callback)(self.context))
    }

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn set_en50530_mode(&mut self, value: En50530) {
        if let Some(callback) = self.set_en50530_mode_callback {
            (callback)(value, self.context);
        };
    }

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn en50530_mpp_voltage(&self) -> Option<u16> {
        self.en50530_mpp_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn set_en50530_mpp_voltage(&mut self, value: u16) {
        if let Some(callback) = self.set_en50530_mpp_voltage_callback {
            (callback)(value, self.context);
        };
    }

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn en50530_mpp_power(&self) -> Option<u16> {
        self.en50530_mpp_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn set_en50530_mpp_power(&mut self, value: u16) {
        if let Some(callback) = self.set_en50530_mpp_power_callback {
            (callback)(value, self.context);
        };
    }

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn irradiance_setpoint(&self) -> Option<u16> {
        self.irradiance_setpoint_callback
            .map(|callback| (callback)(self.context))
    }

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn set_irradiance_setpoint(&mut self, value: u16) {
        if let Some(callback) = self.set_irradiance_setpoint_callback {
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

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn power_slew_rate(&self) -> Option<u16> {
        self.power_slew_rate_callback
            .map(|callback| (callback)(self.context))
    }

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn set_power_slew_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_power_slew_rate_callback {
            (callback)(value, self.context);
        };
    }

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn current_slew_rate(&self) -> Option<u16> {
        self.current_slew_rate_callback
            .map(|callback| (callback)(self.context))
    }

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn set_current_slew_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_current_slew_rate_callback {
            (callback)(value, self.context);
        };
    }

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn enable_profile(&self) -> Option<EnaProf> {
        self.enable_profile_callback
            .map(|callback| (callback)(self.context))
    }

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn set_enable_profile(&mut self, value: EnaProf) {
        if let Some(callback) = self.set_enable_profile_callback {
            (callback)(value, self.context);
        };
    }

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn profile_adoption_request(&self) -> Option<u16> {
        self.profile_adoption_request_callback
            .map(|callback| (callback)(self.context))
    }

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn set_profile_adoption_request(&mut self, value: u16) {
        if let Some(callback) = self.set_profile_adoption_request_callback {
            (callback)(value, self.context);
        };
    }

    /// Adopt Profile Result
    ///
    /// Result of last adopt profile operation.
    fn adopt_profile_result(&self) -> AdptProfRslt {
        (self.adopt_profile_result_callback)(self.context)
    }

    /// Measured Voltage
    ///
    /// Measured Voltage
    fn measured_voltage(&self) -> Option<i32> {
        self.measured_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Measured Power
    ///
    /// Measured Power
    fn measured_power(&self) -> Option<i32> {
        self.measured_power_callback
            .map(|callback| (callback)(self.context))
    }

    /// Measured Current
    ///
    /// Measured Current
    fn measured_current(&self) -> Option<i32> {
        self.measured_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Errors
    ///
    /// Error States
    fn errors(&self) -> Option<&CStr> {
        self.errors_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Number Of Points
    ///
    /// Number of profile points supported.
    fn number_of_points(&self) -> u16 {
        (self.number_of_points_callback)(self.context)
    }

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn stored_profile_count(&self) -> u16 {
        (self.stored_profile_count_callback)(self.context)
    }

    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    fn power_scale_factor(&self) -> u16 {
        (self.power_scale_factor_callback)(self.context)
    }

    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    fn set_power_scale_factor(&mut self, value: u16) {
        (self.set_power_scale_factor_callback)(value, self.context);
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        (self.voltage_scale_factor_callback)(self.context)
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn set_voltage_scale_factor(&mut self, value: u16) {
        (self.set_voltage_scale_factor_callback)(value, self.context);
    }

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn current_scale_factor(&self) -> u16 {
        (self.current_scale_factor_callback)(self.context)
    }

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn set_current_scale_factor(&mut self, value: u16) {
        (self.set_current_scale_factor_callback)(value, self.context);
    }

    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    fn irradiance_scale_factor(&self) -> u16 {
        (self.irradiance_scale_factor_callback)(self.context)
    }

    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    fn set_irradiance_scale_factor(&mut self, value: u16) {
        (self.set_irradiance_scale_factor_callback)(value, self.context);
    }

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn time_scale_factor(&self) -> u16 {
        (self.time_scale_factor_callback)(self.context)
    }

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn set_time_scale_factor(&mut self, value: u16) {
        (self.set_time_scale_factor_callback)(value, self.context);
    }

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn voltage_slew_rate_scale_factor(&self) -> u16 {
        (self.voltage_slew_rate_scale_factor_callback)(self.context)
    }

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn set_voltage_slew_rate_scale_factor(&mut self, value: u16) {
        (self.set_voltage_slew_rate_scale_factor_callback)(value, self.context);
    }

    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    fn power_slew_rate_scale_factor(&self) -> u16 {
        (self.power_slew_rate_scale_factor_callback)(self.context)
    }

    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    fn set_power_slew_rate_scale_factor(&mut self, value: u16) {
        (self.set_power_slew_rate_scale_factor_callback)(value, self.context);
    }

    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    fn current_slew_rate_scale_factor(&self) -> u16 {
        (self.current_slew_rate_scale_factor_callback)(self.context)
    }

    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    fn set_current_slew_rate_scale_factor(&mut self, value: u16) {
        (self.set_current_slew_rate_scale_factor_callback)(value, self.context);
    }

    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    fn percent_scale_factor(&self) -> u16 {
        (self.percent_scale_factor_callback)(self.context)
    }

    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    fn set_percent_scale_factor(&mut self, value: u16) {
        (self.set_percent_scale_factor_callback)(value, self.context);
    }
}

#[repr(C)]
pub struct Model64410StatefulAdapter {
    maximum_voltage: u16,
    maximum_power: u16,
    maximum_current: u16,
    cv_or_cc_mode: Mode,
    power_on_off: Ena,
    reset_device: Reset,
    voltage_setpoint: u16,
    power_setpoint: u16,
    current_setpoint: u16,
    en50530_mode: En50530,
    en50530_mpp_voltage: u16,
    en50530_mpp_power: u16,
    irradiance_setpoint: u16,
    voltage_slew_rate: u16,
    power_slew_rate: u16,
    current_slew_rate: u16,
    enable_profile: EnaProf,
    profile_adoption_request: u16,
    adopt_profile_result: AdptProfRslt,
    measured_voltage: i32,
    measured_power: i32,
    measured_current: i32,
    errors: [c_char; 64],
    number_of_points: u16,
    stored_profile_count: u16,
    power_scale_factor: u16,
    voltage_scale_factor: u16,
    current_scale_factor: u16,
    irradiance_scale_factor: u16,
    time_scale_factor: u16,
    voltage_slew_rate_scale_factor: u16,
    power_slew_rate_scale_factor: u16,
    current_slew_rate_scale_factor: u16,
    percent_scale_factor: u16,
}

impl ModelAdapter for Model64410StatefulAdapter {
    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    fn maximum_voltage(&self) -> Option<u16> {
        Some(self.maximum_voltage)
    }

    /// Maximum Voltage
    ///
    /// Upper Voltage Protection Limit
    fn set_maximum_voltage(&mut self, value: u16) {
        self.maximum_voltage = value;
    }

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn maximum_power(&self) -> Option<u16> {
        Some(self.maximum_power)
    }

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn set_maximum_power(&mut self, value: u16) {
        self.maximum_power = value;
    }

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn maximum_current(&self) -> Option<u16> {
        Some(self.maximum_current)
    }

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn set_maximum_current(&mut self, value: u16) {
        self.maximum_current = value;
    }

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn cv_or_cc_mode(&self) -> Option<Mode> {
        Some(self.cv_or_cc_mode)
    }

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn set_cv_or_cc_mode(&mut self, value: Mode) {
        self.cv_or_cc_mode = value;
    }

    /// Power On/Off
    ///
    /// Power On/Off
    fn power_on_off(&self) -> Option<Ena> {
        Some(self.power_on_off)
    }

    /// Power On/Off
    ///
    /// Power On/Off
    fn set_power_on_off(&mut self, value: Ena) {
        self.power_on_off = value;
    }

    /// Reset Device
    ///
    /// Reset Device
    fn reset_device(&self) -> Option<Reset> {
        Some(self.reset_device)
    }

    /// Reset Device
    ///
    /// Reset Device
    fn set_reset_device(&mut self, value: Reset) {
        self.reset_device = value;
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn voltage_setpoint(&self) -> Option<u16> {
        Some(self.voltage_setpoint)
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn set_voltage_setpoint(&mut self, value: u16) {
        self.voltage_setpoint = value;
    }

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn power_setpoint(&self) -> Option<u16> {
        Some(self.power_setpoint)
    }

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn set_power_setpoint(&mut self, value: u16) {
        self.power_setpoint = value;
    }

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn current_setpoint(&self) -> Option<u16> {
        Some(self.current_setpoint)
    }

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn set_current_setpoint(&mut self, value: u16) {
        self.current_setpoint = value;
    }

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn en50530_mode(&self) -> Option<En50530> {
        Some(self.en50530_mode)
    }

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn set_en50530_mode(&mut self, value: En50530) {
        self.en50530_mode = value;
    }

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn en50530_mpp_voltage(&self) -> Option<u16> {
        Some(self.en50530_mpp_voltage)
    }

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn set_en50530_mpp_voltage(&mut self, value: u16) {
        self.en50530_mpp_voltage = value;
    }

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn en50530_mpp_power(&self) -> Option<u16> {
        Some(self.en50530_mpp_power)
    }

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn set_en50530_mpp_power(&mut self, value: u16) {
        self.en50530_mpp_power = value;
    }

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn irradiance_setpoint(&self) -> Option<u16> {
        Some(self.irradiance_setpoint)
    }

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn set_irradiance_setpoint(&mut self, value: u16) {
        self.irradiance_setpoint = value;
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

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn power_slew_rate(&self) -> Option<u16> {
        Some(self.power_slew_rate)
    }

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn set_power_slew_rate(&mut self, value: u16) {
        self.power_slew_rate = value;
    }

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn current_slew_rate(&self) -> Option<u16> {
        Some(self.current_slew_rate)
    }

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn set_current_slew_rate(&mut self, value: u16) {
        self.current_slew_rate = value;
    }

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn enable_profile(&self) -> Option<EnaProf> {
        Some(self.enable_profile)
    }

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn set_enable_profile(&mut self, value: EnaProf) {
        self.enable_profile = value;
    }

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn profile_adoption_request(&self) -> Option<u16> {
        Some(self.profile_adoption_request)
    }

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn set_profile_adoption_request(&mut self, value: u16) {
        self.profile_adoption_request = value;
    }

    /// Adopt Profile Result
    ///
    /// Result of last adopt profile operation.
    fn adopt_profile_result(&self) -> AdptProfRslt {
        self.adopt_profile_result
    }

    /// Measured Voltage
    ///
    /// Measured Voltage
    fn measured_voltage(&self) -> Option<i32> {
        Some(self.measured_voltage)
    }

    /// Measured Power
    ///
    /// Measured Power
    fn measured_power(&self) -> Option<i32> {
        Some(self.measured_power)
    }

    /// Measured Current
    ///
    /// Measured Current
    fn measured_current(&self) -> Option<i32> {
        Some(self.measured_current)
    }

    /// Errors
    ///
    /// Error States
    fn errors(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.errors.as_ptr()) })
    }

    /// Number Of Points
    ///
    /// Number of profile points supported.
    fn number_of_points(&self) -> u16 {
        self.number_of_points
    }

    /// Stored Profile Count
    ///
    /// Number of stored profiles supported.
    fn stored_profile_count(&self) -> u16 {
        self.stored_profile_count
    }

    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    fn power_scale_factor(&self) -> u16 {
        self.power_scale_factor
    }

    /// Power Scale Factor
    ///
    /// Scale factor for power points.
    fn set_power_scale_factor(&mut self, value: u16) {
        self.power_scale_factor = value;
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn voltage_scale_factor(&self) -> u16 {
        self.voltage_scale_factor
    }

    /// Voltage Scale Factor
    ///
    /// Scale factor for voltage points.
    fn set_voltage_scale_factor(&mut self, value: u16) {
        self.voltage_scale_factor = value;
    }

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn current_scale_factor(&self) -> u16 {
        self.current_scale_factor
    }

    /// Current Scale Factor
    ///
    /// Scale factor for current points.
    fn set_current_scale_factor(&mut self, value: u16) {
        self.current_scale_factor = value;
    }

    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    fn irradiance_scale_factor(&self) -> u16 {
        self.irradiance_scale_factor
    }

    /// Irradiance Scale Factor
    ///
    /// Scale factor for irradiance.
    fn set_irradiance_scale_factor(&mut self, value: u16) {
        self.irradiance_scale_factor = value;
    }

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn time_scale_factor(&self) -> u16 {
        self.time_scale_factor
    }

    /// Time Scale Factor
    ///
    /// Scale factor for time points.
    fn set_time_scale_factor(&mut self, value: u16) {
        self.time_scale_factor = value;
    }

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn voltage_slew_rate_scale_factor(&self) -> u16 {
        self.voltage_slew_rate_scale_factor
    }

    /// Voltage Slew Rate Scale Factor
    ///
    /// Scale factor for voltage slew rate.
    fn set_voltage_slew_rate_scale_factor(&mut self, value: u16) {
        self.voltage_slew_rate_scale_factor = value;
    }

    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    fn power_slew_rate_scale_factor(&self) -> u16 {
        self.power_slew_rate_scale_factor
    }

    /// Power Slew Rate Scale Factor
    ///
    /// Scale factor for power slew rate.
    fn set_power_slew_rate_scale_factor(&mut self, value: u16) {
        self.power_slew_rate_scale_factor = value;
    }

    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    fn current_slew_rate_scale_factor(&self) -> u16 {
        self.current_slew_rate_scale_factor
    }

    /// Current Slew Rate Scale Factor
    ///
    /// Scale factor for current slew rate.
    fn set_current_slew_rate_scale_factor(&mut self, value: u16) {
        self.current_slew_rate_scale_factor = value;
    }

    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    fn percent_scale_factor(&self) -> u16 {
        self.percent_scale_factor
    }

    /// Percent Scale Factor
    ///
    /// Scale factor for percentages.
    fn set_percent_scale_factor(&mut self, value: u16) {
        self.percent_scale_factor = value;
    }
}
