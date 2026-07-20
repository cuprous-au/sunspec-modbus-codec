use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 66;

pub static POINTS: [ReadablePoint; 66] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64112 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 64 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::PortNumber },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::CSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::HSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::PSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::KwhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::Faults },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::Absorb },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AbsorbTime },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AbsorbEnd },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::Rebulk },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::Float },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::MaximumCharge },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::Equalize },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::EqualizeTime },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AutoEqualizeInterval },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::MpptMode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::SweepWidth },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::SweepMaximum },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::UPickPwmDutyCycle },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::GridTieMode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::TempCompMode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::TempCompLowerLimit },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::TempCompUpperLimit },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AutoRestartMode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::WakeupVocChange },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::SnoozeMode },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::WakeupInterval },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxOutputMode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxOutputControl },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxOutputState },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxOutputPolarity },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxLowBatteryDisconnect },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxLowBatteryReconnect },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxLowBatteryDisconnectDelay },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxVentFan },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxPvTrigger },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxPvTriggerHoldTime },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxNightLightThreshold },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxNightLightOnTime },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxNightLightOnHysteresis },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxNightLightOffHysteresis },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxErrorOutputLowBattery },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxDivertHoldTime },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxDivertDelayTime },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxDivertRelative },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::AuxDivertHysteresis },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::FmCcMajorFirmwareNumber },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::FmCcMidFirmwareNumber },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::FmCcMinorFirmwareNumber },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::SetDataLogDayOffset },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::CurrentDataLogDayOffset },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyAh },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyKWh },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyMaximumOutputA },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyMaximumOutputW },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyAbsorbTime },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyFloatTime },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyMinimumBattery },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyMaximumBattery },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogDailyMaximumInput },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogClear },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64112 { point: Point::DataLogClearComplement },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    PortNumber,
    VSf,
    CSf,
    HSf,
    PSf,
    AhSf,
    KwhSf,
    Faults,
    Absorb,
    AbsorbTime,
    AbsorbEnd,
    Rebulk,
    Float,
    MaximumCharge,
    Equalize,
    EqualizeTime,
    AutoEqualizeInterval,
    MpptMode,
    SweepWidth,
    SweepMaximum,
    UPickPwmDutyCycle,
    GridTieMode,
    TempCompMode,
    TempCompLowerLimit,
    TempCompUpperLimit,
    AutoRestartMode,
    WakeupVocChange,
    SnoozeMode,
    WakeupInterval,
    AuxOutputMode,
    AuxOutputControl,
    AuxOutputState,
    AuxOutputPolarity,
    AuxLowBatteryDisconnect,
    AuxLowBatteryReconnect,
    AuxLowBatteryDisconnectDelay,
    AuxVentFan,
    AuxPvTrigger,
    AuxPvTriggerHoldTime,
    AuxNightLightThreshold,
    AuxNightLightOnTime,
    AuxNightLightOnHysteresis,
    AuxNightLightOffHysteresis,
    AuxErrorOutputLowBattery,
    AuxDivertHoldTime,
    AuxDivertDelayTime,
    AuxDivertRelative,
    AuxDivertHysteresis,
    FmCcMajorFirmwareNumber,
    FmCcMidFirmwareNumber,
    FmCcMinorFirmwareNumber,
    SetDataLogDayOffset,
    CurrentDataLogDayOffset,
    DataLogDailyAh,
    DataLogDailyKWh,
    DataLogDailyMaximumOutputA,
    DataLogDailyMaximumOutputW,
    DataLogDailyAbsorbTime,
    DataLogDailyFloatTime,
    DataLogDailyMinimumBattery,
    DataLogDailyMaximumBattery,
    DataLogDailyMaximumInput,
    DataLogClear,
    DataLogClearComplement,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::PortNumber => serialisation::write_u16(model.port_number(), buffer),
        Point::VSf => serialisation::write_u16(model.v_sf(), buffer),
        Point::CSf => serialisation::write_u16(model.c_sf(), buffer),
        Point::HSf => serialisation::write_u16(model.h_sf(), buffer),
        Point::PSf => serialisation::write_u16(model.p_sf(), buffer),
        Point::AhSf => serialisation::write_u16(model.ah_sf(), buffer),
        Point::KwhSf => serialisation::write_u16(model.kwh_sf(), buffer),
        Point::Faults => serialisation::write_u16(model.faults(), buffer),
        Point::Absorb => serialisation::write_u16(model.absorb(), buffer),
        Point::AbsorbTime => serialisation::write_u16(model.absorb_time(), buffer),
        Point::AbsorbEnd => serialisation::write_u16(model.absorb_end(), buffer),
        Point::Rebulk => serialisation::write_u16(model.rebulk(), buffer),
        Point::Float => serialisation::write_u16(model.float(), buffer),
        Point::MaximumCharge => serialisation::write_u16(model.maximum_charge(), buffer),
        Point::Equalize => serialisation::write_u16(model.equalize(), buffer),
        Point::EqualizeTime => serialisation::write_u16(model.equalize_time(), buffer),
        Point::AutoEqualizeInterval => serialisation::write_u16(model.auto_equalize_interval(), buffer),
        Point::MpptMode => serialisation::write_u16(model.mppt_mode() as u16, buffer),
        Point::SweepWidth => serialisation::write_u16(model.sweep_width() as u16, buffer),
        Point::SweepMaximum => serialisation::write_u16(model.sweep_maximum() as u16, buffer),
        Point::UPickPwmDutyCycle => serialisation::write_u16(model.u_pick_pwm_duty_cycle(), buffer),
        Point::GridTieMode => serialisation::write_u16(model.grid_tie_mode() as u16, buffer),
        Point::TempCompMode => serialisation::write_u16(model.temp_comp_mode() as u16, buffer),
        Point::TempCompLowerLimit => serialisation::write_u16(model.temp_comp_lower_limit(), buffer),
        Point::TempCompUpperLimit => serialisation::write_u16(model.temp_comp_upper_limit(), buffer),
        Point::AutoRestartMode => serialisation::write_u16(model.auto_restart_mode() as u16, buffer),
        Point::WakeupVocChange => serialisation::write_u16(model.wakeup_voc_change(), buffer),
        Point::SnoozeMode => serialisation::write_u16(model.snooze_mode(), buffer),
        Point::WakeupInterval => serialisation::write_u16(model.wakeup_interval(), buffer),
        Point::AuxOutputMode => serialisation::write_u16(model.aux_output_mode() as u16, buffer),
        Point::AuxOutputControl => serialisation::write_u16(model.aux_output_control() as u16, buffer),
        Point::AuxOutputState => serialisation::write_u16(model.aux_output_state() as u16, buffer),
        Point::AuxOutputPolarity => serialisation::write_u16(model.aux_output_polarity() as u16, buffer),
        Point::AuxLowBatteryDisconnect => serialisation::write_u16(model.aux_low_battery_disconnect(), buffer),
        Point::AuxLowBatteryReconnect => serialisation::write_u16(model.aux_low_battery_reconnect(), buffer),
        Point::AuxLowBatteryDisconnectDelay => serialisation::write_u16(model.aux_low_battery_disconnect_delay(), buffer),
        Point::AuxVentFan => serialisation::write_u16(model.aux_vent_fan(), buffer),
        Point::AuxPvTrigger => serialisation::write_u16(model.aux_pv_trigger(), buffer),
        Point::AuxPvTriggerHoldTime => serialisation::write_u16(model.aux_pv_trigger_hold_time(), buffer),
        Point::AuxNightLightThreshold => serialisation::write_u16(model.aux_night_light_threshold(), buffer),
        Point::AuxNightLightOnTime => serialisation::write_u16(model.aux_night_light_on_time(), buffer),
        Point::AuxNightLightOnHysteresis => serialisation::write_u16(model.aux_night_light_on_hysteresis(), buffer),
        Point::AuxNightLightOffHysteresis => serialisation::write_u16(model.aux_night_light_off_hysteresis(), buffer),
        Point::AuxErrorOutputLowBattery => serialisation::write_u16(model.aux_error_output_low_battery(), buffer),
        Point::AuxDivertHoldTime => serialisation::write_u16(model.aux_divert_hold_time(), buffer),
        Point::AuxDivertDelayTime => serialisation::write_u16(model.aux_divert_delay_time(), buffer),
        Point::AuxDivertRelative => serialisation::write_u16(model.aux_divert_relative(), buffer),
        Point::AuxDivertHysteresis => serialisation::write_u16(model.aux_divert_hysteresis(), buffer),
        Point::FmCcMajorFirmwareNumber => serialisation::write_u16(model.fm_cc_major_firmware_number(), buffer),
        Point::FmCcMidFirmwareNumber => serialisation::write_u16(model.fm_cc_mid_firmware_number(), buffer),
        Point::FmCcMinorFirmwareNumber => serialisation::write_u16(model.fm_cc_minor_firmware_number(), buffer),
        Point::SetDataLogDayOffset => serialisation::write_u16(model.set_data_log_day_offset(), buffer),
        Point::CurrentDataLogDayOffset => serialisation::write_u16(model.current_data_log_day_offset(), buffer),
        Point::DataLogDailyAh => serialisation::write_u16(model.data_log_daily_ah(), buffer),
        Point::DataLogDailyKWh => serialisation::write_u16(model.data_log_daily_k_wh(), buffer),
        Point::DataLogDailyMaximumOutputA => serialisation::write_u16(model.data_log_daily_maximum_output_a(), buffer),
        Point::DataLogDailyMaximumOutputW => serialisation::write_u16(model.data_log_daily_maximum_output_w(), buffer),
        Point::DataLogDailyAbsorbTime => serialisation::write_u16(model.data_log_daily_absorb_time(), buffer),
        Point::DataLogDailyFloatTime => serialisation::write_u16(model.data_log_daily_float_time(), buffer),
        Point::DataLogDailyMinimumBattery => serialisation::write_u16(model.data_log_daily_minimum_battery(), buffer),
        Point::DataLogDailyMaximumBattery => serialisation::write_u16(model.data_log_daily_maximum_battery(), buffer),
        Point::DataLogDailyMaximumInput => serialisation::write_u16(model.data_log_daily_maximum_input(), buffer),
        Point::DataLogClear => serialisation::write_u16(model.data_log_clear(), buffer),
        Point::DataLogClearComplement => serialisation::write_u16(model.data_log_clear_complement(), buffer),
    }
}

pub trait ModelAdapter {
    /// Port Number
    fn port_number(&self) -> u16;

    fn v_sf(&self) -> u16;

    fn c_sf(&self) -> u16;

    fn h_sf(&self) -> u16;

    fn p_sf(&self) -> u16;

    fn ah_sf(&self) -> u16;

    fn kwh_sf(&self) -> u16;

    /// Faults
    fn faults(&self) -> u16;

    /// Absorb
    fn absorb(&self) -> u16;

    /// Absorb Time
    fn absorb_time(&self) -> u16;

    /// Absorb End
    fn absorb_end(&self) -> u16;

    /// Rebulk
    fn rebulk(&self) -> u16;

    /// Float
    fn float(&self) -> u16;

    /// Maximum Charge
    fn maximum_charge(&self) -> u16;

    /// Equalize
    fn equalize(&self) -> u16;

    /// Equalize Time
    fn equalize_time(&self) -> u16;

    /// Auto Equalize Interval
    fn auto_equalize_interval(&self) -> u16;

    /// MPPT mode
    fn mppt_mode(&self) -> CcConfigMpptMode;

    /// Sweep Width
    fn sweep_width(&self) -> CcConfigSweepWidth;

    /// Sweep Maximum
    fn sweep_maximum(&self) -> CcConfigSweepMax;

    /// U-Pick PWM Duty Cycle
    fn u_pick_pwm_duty_cycle(&self) -> u16;

    /// Grid Tie Mode
    fn grid_tie_mode(&self) -> CcConfigGridTie;

    /// Temp Comp Mode
    fn temp_comp_mode(&self) -> CcConfigTempComp;

    /// Temp Comp Lower Limit
    fn temp_comp_lower_limit(&self) -> u16;

    /// Temp Comp Upper Limit
    fn temp_comp_upper_limit(&self) -> u16;

    /// Auto Restart Mode
    fn auto_restart_mode(&self) -> CcConfigAutoRestart;

    /// Wakeup VOC Change
    fn wakeup_voc_change(&self) -> u16;

    /// Snooze Mode
    fn snooze_mode(&self) -> u16;

    /// Wakeup Interval
    fn wakeup_interval(&self) -> u16;

    /// AUX Output Mode
    fn aux_output_mode(&self) -> CcConfigAuxMode;

    /// AUX Output Control
    fn aux_output_control(&self) -> CcConfigAuxControl;

    /// AUX Output State
    fn aux_output_state(&self) -> CcConfigAuxState;

    /// AUX Output Polarity
    fn aux_output_polarity(&self) -> CcConfigAuxPolarity;

    /// AUX Low Battery Disconnect
    fn aux_low_battery_disconnect(&self) -> u16;

    /// AUX Low Battery Reconnect
    fn aux_low_battery_reconnect(&self) -> u16;

    /// AUX Low Battery Disconnect Delay
    fn aux_low_battery_disconnect_delay(&self) -> u16;

    /// AUX Vent Fan
    fn aux_vent_fan(&self) -> u16;

    /// AUX PV Trigger
    fn aux_pv_trigger(&self) -> u16;

    /// AUX PV Trigger Hold Time
    fn aux_pv_trigger_hold_time(&self) -> u16;

    /// AUX Night Light Threshold
    fn aux_night_light_threshold(&self) -> u16;

    /// AUX Night Light On Time
    fn aux_night_light_on_time(&self) -> u16;

    /// AUX Night Light On Hysteresis
    fn aux_night_light_on_hysteresis(&self) -> u16;

    /// AUX Night Light Off Hysteresis
    fn aux_night_light_off_hysteresis(&self) -> u16;

    /// AUX Error Output Low Battery
    fn aux_error_output_low_battery(&self) -> u16;

    /// AUX Divert Hold Time
    fn aux_divert_hold_time(&self) -> u16;

    /// AUX Divert Delay Time
    fn aux_divert_delay_time(&self) -> u16;

    /// AUX Divert Relative
    fn aux_divert_relative(&self) -> u16;

    /// AUX Divert Hysteresis
    fn aux_divert_hysteresis(&self) -> u16;

    /// FM CC Major Firmware Number
    fn fm_cc_major_firmware_number(&self) -> u16;

    /// FM CC Mid Firmware Number
    fn fm_cc_mid_firmware_number(&self) -> u16;

    /// FM CC Minor Firmware Number
    fn fm_cc_minor_firmware_number(&self) -> u16;

    /// Set Data Log Day Offset
    fn set_data_log_day_offset(&self) -> u16;

    /// Current Data Log Day Offset
    fn current_data_log_day_offset(&self) -> u16;

    /// Data Log Daily (Ah)
    fn data_log_daily_ah(&self) -> u16;

    /// Data Log Daily (kWh)
    fn data_log_daily_k_wh(&self) -> u16;

    /// Data Log Daily Maximum Output (A)
    fn data_log_daily_maximum_output_a(&self) -> u16;

    /// Data Log Daily Maximum Output (W)
    fn data_log_daily_maximum_output_w(&self) -> u16;

    /// Data Log Daily Absorb Time
    fn data_log_daily_absorb_time(&self) -> u16;

    /// Data Log Daily Float Time
    fn data_log_daily_float_time(&self) -> u16;

    /// Data Log Daily Minimum Battery
    fn data_log_daily_minimum_battery(&self) -> u16;

    /// Data Log Daily Maximum Battery
    fn data_log_daily_maximum_battery(&self) -> u16;

    /// Data Log Daily Maximum Input
    fn data_log_daily_maximum_input(&self) -> u16;

    /// Data Log Clear
    fn data_log_clear(&self) -> u16;

    /// Data Log Clear Complement
    fn data_log_clear_complement(&self) -> u16;
}

pub enum CcConfigMpptMode {
    Auto = 0,
    UPick = 1,
    Wind = 2,
}

pub enum CcConfigSweepWidth {
    Half = 0,
    Full = 1,
}

pub enum CcConfigSweepMax {
    EightyPercent = 0,
    EightyFivePercent = 1,
    NintyPercent = 2,
    NintyNinePercent = 3,
}

pub enum CcConfigGridTie {
    Disabled = 0,
    Enabled = 1,
}

pub enum CcConfigTempComp {
    Wide = 0,
    Limited = 1,
}

pub enum CcConfigAutoRestart {
    Off = 0,
    Every90Minutes = 1,
    Every90MinutesIfAbsorbOrFloat = 2,
}

pub enum CcConfigAuxMode {
    Float = 0,
    DiversionRelay = 1,
    DiversionSolidSt = 2,
    LowBattDisconnect = 3,
    Remote = 4,
    VentFan = 5,
    PvTrigger = 6,
    ErrorOutput = 7,
    NightLight = 8,
}

pub enum CcConfigAuxControl {
    Off = 0,
    Auto = 1,
    On = 2,
}

pub enum CcConfigAuxState {
    Disabled = 0,
    Enabled = 1,
}

pub enum CcConfigAuxPolarity {
    Low = 0,
    High = 1,
}