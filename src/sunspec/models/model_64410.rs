use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 70;

pub static POINTS: [ReadablePoint; 36] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64410 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 68 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::MaximumVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::MaximumPower },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::MaximumCurrent },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::CvOrCcMode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::PowerOnOff },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::ResetDevice },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::VoltageSetpoint },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::PowerSetpoint },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::CurrentSetpoint },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::En50530Mode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::En50530MppVoltage },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::En50530MppPower },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::IrradianceSetpoint },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::VoltageSlewRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::PowerSlewRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::CurrentSlewRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::EnableProfile },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::ProfileAdoptionRequest },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::AdoptProfileResult },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::MeasuredVoltage },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::MeasuredPower },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::MeasuredCurrent },
        size: 2,
        data_type: PointType::Int32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::Errors },
        size: 32,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::NumberOfPoints },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::StoredProfileCount },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::PowerScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::VoltageScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::CurrentScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::IrradianceScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::TimeScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::VoltageSlewRateScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::PowerSlewRateScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::CurrentSlewRateScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64410 { point: Point::PercentScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::MaximumVoltage => if let Some(value) = model.maximum_voltage() { serialisation::write_u16(value, buffer); },
        Point::MaximumPower => if let Some(value) = model.maximum_power() { serialisation::write_u16(value, buffer); },
        Point::MaximumCurrent => if let Some(value) = model.maximum_current() { serialisation::write_u16(value, buffer); },
        Point::CvOrCcMode => if let Some(value) = model.cv_or_cc_mode() { serialisation::write_u16(value as u16, buffer); },
        Point::PowerOnOff => if let Some(value) = model.power_on_off() { serialisation::write_u16(value as u16, buffer); },
        Point::ResetDevice => if let Some(value) = model.reset_device() { serialisation::write_u16(value as u16, buffer); },
        Point::VoltageSetpoint => if let Some(value) = model.voltage_setpoint() { serialisation::write_u16(value, buffer); },
        Point::PowerSetpoint => if let Some(value) = model.power_setpoint() { serialisation::write_u16(value, buffer); },
        Point::CurrentSetpoint => if let Some(value) = model.current_setpoint() { serialisation::write_u16(value, buffer); },
        Point::En50530Mode => if let Some(value) = model.en50530_mode() { serialisation::write_u16(value as u16, buffer); },
        Point::En50530MppVoltage => if let Some(value) = model.en50530_mpp_voltage() { serialisation::write_u16(value, buffer); },
        Point::En50530MppPower => if let Some(value) = model.en50530_mpp_power() { serialisation::write_u16(value, buffer); },
        Point::IrradianceSetpoint => if let Some(value) = model.irradiance_setpoint() { serialisation::write_u16(value, buffer); },
        Point::VoltageSlewRate => if let Some(value) = model.voltage_slew_rate() { serialisation::write_u16(value, buffer); },
        Point::PowerSlewRate => if let Some(value) = model.power_slew_rate() { serialisation::write_u16(value, buffer); },
        Point::CurrentSlewRate => if let Some(value) = model.current_slew_rate() { serialisation::write_u16(value, buffer); },
        Point::EnableProfile => if let Some(value) = model.enable_profile() { serialisation::write_u16(value as u16, buffer); },
        Point::ProfileAdoptionRequest => if let Some(value) = model.profile_adoption_request() { serialisation::write_u16(value, buffer); },
        Point::AdoptProfileResult => serialisation::write_u16(model.adopt_profile_result() as u16, buffer),
        Point::MeasuredVoltage => if let Some(value) = model.measured_voltage() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::MeasuredPower => if let Some(value) = model.measured_power() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::MeasuredCurrent => if let Some(value) = model.measured_current() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::Errors => if let Some(value) = model.errors() { serialisation::write_string(value, buffer, offset, limit); },
        Point::NumberOfPoints => serialisation::write_u16(model.number_of_points(), buffer),
        Point::StoredProfileCount => serialisation::write_u16(model.stored_profile_count(), buffer),
        Point::PowerScaleFactor => serialisation::write_u16(model.power_scale_factor(), buffer),
        Point::VoltageScaleFactor => serialisation::write_u16(model.voltage_scale_factor(), buffer),
        Point::CurrentScaleFactor => serialisation::write_u16(model.current_scale_factor(), buffer),
        Point::IrradianceScaleFactor => serialisation::write_u16(model.irradiance_scale_factor(), buffer),
        Point::TimeScaleFactor => serialisation::write_u16(model.time_scale_factor(), buffer),
        Point::VoltageSlewRateScaleFactor => serialisation::write_u16(model.voltage_slew_rate_scale_factor(), buffer),
        Point::PowerSlewRateScaleFactor => serialisation::write_u16(model.power_slew_rate_scale_factor(), buffer),
        Point::CurrentSlewRateScaleFactor => serialisation::write_u16(model.current_slew_rate_scale_factor(), buffer),
        Point::PercentScaleFactor => serialisation::write_u16(model.percent_scale_factor(), buffer),
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
    fn set_maximum_voltage(&mut self, value: u16) {
    }

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn maximum_power(&self) -> Option<u16> {
        None
    }

    /// Maximum Power
    ///
    /// Upper Power Protection Limit
    fn set_maximum_power(&mut self, value: u16) {
    }

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn maximum_current(&self) -> Option<u16> {
        None
    }

    /// Maximum Current
    ///
    /// Upper Current Protection Limit
    fn set_maximum_current(&mut self, value: u16) {
    }

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn cv_or_cc_mode(&self) -> Option<Mode> {
        None
    }

    /// CV or CC Mode
    ///
    /// Constant Voltage (CV) or Constant Current (CC) Mode
    fn set_cv_or_cc_mode(&mut self, value: Mode) {
    }

    /// Power On/Off
    ///
    /// Power On/Off
    fn power_on_off(&self) -> Option<Ena> {
        None
    }

    /// Power On/Off
    ///
    /// Power On/Off
    fn set_power_on_off(&mut self, value: Ena) {
    }

    /// Reset Device
    ///
    /// Reset Device
    fn reset_device(&self) -> Option<Reset> {
        None
    }

    /// Reset Device
    ///
    /// Reset Device
    fn set_reset_device(&mut self, value: Reset) {
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn voltage_setpoint(&self) -> Option<u16> {
        None
    }

    /// Voltage Setpoint
    ///
    /// Voltage Setpoint
    fn set_voltage_setpoint(&mut self, value: u16) {
    }

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn power_setpoint(&self) -> Option<u16> {
        None
    }

    /// Power Setpoint
    ///
    /// Power Setpoint
    fn set_power_setpoint(&mut self, value: u16) {
    }

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn current_setpoint(&self) -> Option<u16> {
        None
    }

    /// Current Setpoint
    ///
    /// Current Setpoint
    fn set_current_setpoint(&mut self, value: u16) {
    }

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn en50530_mode(&self) -> Option<En50530> {
        None
    }

    /// EN50530 Mode
    ///
    /// EN50530 Mode - Enable or disable EN50530 profile mode
    fn set_en50530_mode(&mut self, value: En50530) {
    }

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn en50530_mpp_voltage(&self) -> Option<u16> {
        None
    }

    /// EN50530 MPP Voltage
    ///
    /// EN50530 MPP Voltage
    fn set_en50530_mpp_voltage(&mut self, value: u16) {
    }

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn en50530_mpp_power(&self) -> Option<u16> {
        None
    }

    /// EN50530 MPP Power
    ///
    /// EN50530 MPP Power
    fn set_en50530_mpp_power(&mut self, value: u16) {
    }

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn irradiance_setpoint(&self) -> Option<u16> {
        None
    }

    /// Irradiance Setpoint
    ///
    /// Irradiance Setpoint
    fn set_irradiance_setpoint(&mut self, value: u16) {
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn voltage_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Voltage Slew Rate
    ///
    /// Voltage Slew Rate
    fn set_voltage_slew_rate(&mut self, value: u16) {
    }

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn power_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Power Slew Rate
    ///
    /// Power Slew Rate
    fn set_power_slew_rate(&mut self, value: u16) {
    }

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn current_slew_rate(&self) -> Option<u16> {
        None
    }

    /// Current Slew Rate
    ///
    /// Current Slew Rate
    fn set_current_slew_rate(&mut self, value: u16) {
    }

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn enable_profile(&self) -> Option<EnaProf> {
        None
    }

    /// Enable Profile
    ///
    /// Start/Stop the Profile
    fn set_enable_profile(&mut self, value: EnaProf) {
    }

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn profile_adoption_request(&self) -> Option<u16> {
        None
    }

    /// Profile Adoption Request
    ///
    /// Index of profile points to adopt. First curve index is 1.
    fn set_profile_adoption_request(&mut self, value: u16) {
    }

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
    fn errors(&self) -> Option<String<64>> {
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