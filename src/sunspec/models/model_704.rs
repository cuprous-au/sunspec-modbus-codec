use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 59;

pub static POINTS: [ReadablePoint; 45] = [
    ReadablePoint {
        reference: PointReference::Static { value: 704 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 65 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PowerFactorEnableWInjEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PowerFactorReversionEnableWInj },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PfReversionTimeWInj },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PfReversionTimeRemWInj },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PowerFactorEnableWAbsEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PowerFactorReversionEnableWAbs },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PfReversionTimeWAbs },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PfReversionTimeRemWAbs },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::LimitMaxPowerPctEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::LimitMaxPowerPctSetpoint },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReversionLimitMaxPowerPct },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReversionLimitMaxPowerPctEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::LimitMaxPowerPctReversionTime },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::LimitMaxPowerPctRevTimeRem },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::SetActivePowerEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::SetActivePowerMode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ActivePowerSetpointW },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReversionActivePowerW },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ActivePowerSetpointPct },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReversionActivePowerPct },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReversionActivePowerEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ActivePowerReversionTime },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ActivePowerRevTimeRem },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::SetReactivePowerEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::SetReactivePowerMode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerPriority },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerSetpointVars },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReversionReactivePowerVars },
        size: 2,
        data_type: PointType::Int32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerSetpointPct },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReversionReactivePowerPct },
        size: 1,
        data_type: PointType::Int16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReversionReactivePowerEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerReversionTime },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerRevTimeRem },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::NormalRampRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::NormalRampRateReference },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerRampRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::AntiIslandingEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::PowerFactorScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::LimitMaxPowerScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ActivePowerScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ActivePowerPctScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerPctScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    PowerFactorEnableWInjEnable,
    PowerFactorReversionEnableWInj,
    PfReversionTimeWInj,
    PfReversionTimeRemWInj,
    PowerFactorEnableWAbsEnable,
    PowerFactorReversionEnableWAbs,
    PfReversionTimeWAbs,
    PfReversionTimeRemWAbs,
    LimitMaxPowerPctEnable,
    LimitMaxPowerPctSetpoint,
    ReversionLimitMaxPowerPct,
    ReversionLimitMaxPowerPctEnable,
    LimitMaxPowerPctReversionTime,
    LimitMaxPowerPctRevTimeRem,
    SetActivePowerEnable,
    SetActivePowerMode,
    ActivePowerSetpointW,
    ReversionActivePowerW,
    ActivePowerSetpointPct,
    ReversionActivePowerPct,
    ReversionActivePowerEnable,
    ActivePowerReversionTime,
    ActivePowerRevTimeRem,
    SetReactivePowerEnable,
    SetReactivePowerMode,
    ReactivePowerPriority,
    ReactivePowerSetpointVars,
    ReversionReactivePowerVars,
    ReactivePowerSetpointPct,
    ReversionReactivePowerPct,
    ReversionReactivePowerEnable,
    ReactivePowerReversionTime,
    ReactivePowerRevTimeRem,
    NormalRampRate,
    NormalRampRateReference,
    ReactivePowerRampRate,
    AntiIslandingEnable,
    PowerFactorScaleFactor,
    LimitMaxPowerScaleFactor,
    ActivePowerScaleFactor,
    ActivePowerPctScaleFactor,
    ReactivePowerScaleFactor,
    ReactivePowerPctScaleFactor,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::PowerFactorEnableWInjEnable => if let Some(value) = model.power_factor_enable_w_inj_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::PowerFactorReversionEnableWInj => if let Some(value) = model.power_factor_reversion_enable_w_inj() { serialisation::write_u16(value as u16, buffer); },
        Point::PfReversionTimeWInj => if let Some(value) = model.pf_reversion_time_w_inj() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::PfReversionTimeRemWInj => if let Some(value) = model.pf_reversion_time_rem_w_inj() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::PowerFactorEnableWAbsEnable => if let Some(value) = model.power_factor_enable_w_abs_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::PowerFactorReversionEnableWAbs => if let Some(value) = model.power_factor_reversion_enable_w_abs() { serialisation::write_u16(value as u16, buffer); },
        Point::PfReversionTimeWAbs => if let Some(value) = model.pf_reversion_time_w_abs() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::PfReversionTimeRemWAbs => if let Some(value) = model.pf_reversion_time_rem_w_abs() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::LimitMaxPowerPctEnable => if let Some(value) = model.limit_max_power_pct_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::LimitMaxPowerPctSetpoint => if let Some(value) = model.limit_max_power_pct_setpoint() { serialisation::write_u16(value, buffer); },
        Point::ReversionLimitMaxPowerPct => if let Some(value) = model.reversion_limit_max_power_pct() { serialisation::write_u16(value, buffer); },
        Point::ReversionLimitMaxPowerPctEnable => if let Some(value) = model.reversion_limit_max_power_pct_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::LimitMaxPowerPctReversionTime => if let Some(value) = model.limit_max_power_pct_reversion_time() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::LimitMaxPowerPctRevTimeRem => if let Some(value) = model.limit_max_power_pct_rev_time_rem() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::SetActivePowerEnable => if let Some(value) = model.set_active_power_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::SetActivePowerMode => if let Some(value) = model.set_active_power_mode() { serialisation::write_u16(value as u16, buffer); },
        Point::ActivePowerSetpointW => if let Some(value) = model.active_power_setpoint_w() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::ReversionActivePowerW => if let Some(value) = model.reversion_active_power_w() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::ActivePowerSetpointPct => if let Some(value) = model.active_power_setpoint_pct() { serialisation::write_i16(value, buffer); },
        Point::ReversionActivePowerPct => if let Some(value) = model.reversion_active_power_pct() { serialisation::write_i16(value, buffer); },
        Point::ReversionActivePowerEnable => if let Some(value) = model.reversion_active_power_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::ActivePowerReversionTime => if let Some(value) = model.active_power_reversion_time() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::ActivePowerRevTimeRem => if let Some(value) = model.active_power_rev_time_rem() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::SetReactivePowerEnable => if let Some(value) = model.set_reactive_power_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::SetReactivePowerMode => if let Some(value) = model.set_reactive_power_mode() { serialisation::write_u16(value as u16, buffer); },
        Point::ReactivePowerPriority => if let Some(value) = model.reactive_power_priority() { serialisation::write_u16(value as u16, buffer); },
        Point::ReactivePowerSetpointVars => if let Some(value) = model.reactive_power_setpoint_vars() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::ReversionReactivePowerVars => if let Some(value) = model.reversion_reactive_power_vars() { serialisation::write_i32(value, buffer, offset, limit); },
        Point::ReactivePowerSetpointPct => if let Some(value) = model.reactive_power_setpoint_pct() { serialisation::write_i16(value, buffer); },
        Point::ReversionReactivePowerPct => if let Some(value) = model.reversion_reactive_power_pct() { serialisation::write_i16(value, buffer); },
        Point::ReversionReactivePowerEnable => if let Some(value) = model.reversion_reactive_power_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::ReactivePowerReversionTime => if let Some(value) = model.reactive_power_reversion_time() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::ReactivePowerRevTimeRem => if let Some(value) = model.reactive_power_rev_time_rem() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::NormalRampRate => if let Some(value) = model.normal_ramp_rate() { serialisation::write_u16(value, buffer); },
        Point::NormalRampRateReference => if let Some(value) = model.normal_ramp_rate_reference() { serialisation::write_u16(value as u16, buffer); },
        Point::ReactivePowerRampRate => if let Some(value) = model.reactive_power_ramp_rate() { serialisation::write_u16(value, buffer); },
        Point::AntiIslandingEnable => if let Some(value) = model.anti_islanding_enable() { serialisation::write_u16(value as u16, buffer); },
        Point::PowerFactorScaleFactor => if let Some(value) = model.power_factor_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::LimitMaxPowerScaleFactor => if let Some(value) = model.limit_max_power_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::ActivePowerScaleFactor => if let Some(value) = model.active_power_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::ActivePowerPctScaleFactor => if let Some(value) = model.active_power_pct_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::ReactivePowerScaleFactor => if let Some(value) = model.reactive_power_scale_factor() { serialisation::write_u16(value, buffer); },
        Point::ReactivePowerPctScaleFactor => if let Some(value) = model.reactive_power_pct_scale_factor() { serialisation::write_u16(value, buffer); },
    }
}

pub trait ModelAdapter {
    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    fn power_factor_enable_w_inj_enable(&self) -> Option<PfwInjEna> {
        None
    }

    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    fn set_power_factor_enable_w_inj_enable(&mut self, value: PfwInjEna) {
    }

    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    fn power_factor_reversion_enable_w_inj(&self) -> Option<PfwInjEnaRvrt> {
        None
    }

    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    fn set_power_factor_reversion_enable_w_inj(&mut self, value: PfwInjEnaRvrt) {
    }

    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    fn pf_reversion_time_w_inj(&self) -> Option<u32> {
        None
    }

    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    fn set_pf_reversion_time_w_inj(&mut self, value: u32) {
    }

    /// PF Reversion Time Rem (W Inj)
    ///
    /// Power factor reversion time remaining when injecting active power.
    fn pf_reversion_time_rem_w_inj(&self) -> Option<u32> {
        None
    }

    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    fn power_factor_enable_w_abs_enable(&self) -> Option<PfwAbsEna> {
        None
    }

    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    fn set_power_factor_enable_w_abs_enable(&mut self, value: PfwAbsEna) {
    }

    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    fn power_factor_reversion_enable_w_abs(&self) -> Option<PfwAbsEnaRvrt> {
        None
    }

    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    fn set_power_factor_reversion_enable_w_abs(&mut self, value: PfwAbsEnaRvrt) {
    }

    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    fn pf_reversion_time_w_abs(&self) -> Option<u32> {
        None
    }

    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    fn set_pf_reversion_time_w_abs(&mut self, value: u32) {
    }

    /// PF Reversion Time Rem (W Abs)
    ///
    /// Power factor reversion time remaining when absorbing active power.
    fn pf_reversion_time_rem_w_abs(&self) -> Option<u32> {
        None
    }

    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    fn limit_max_power_pct_enable(&self) -> Option<WMaxLimPctEna> {
        None
    }

    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    fn set_limit_max_power_pct_enable(&mut self, value: WMaxLimPctEna) {
    }

    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    fn limit_max_power_pct_setpoint(&self) -> Option<u16> {
        None
    }

    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    fn set_limit_max_power_pct_setpoint(&mut self, value: u16) {
    }

    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    fn reversion_limit_max_power_pct(&self) -> Option<u16> {
        None
    }

    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    fn set_reversion_limit_max_power_pct(&mut self, value: u16) {
    }

    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    fn reversion_limit_max_power_pct_enable(&self) -> Option<WMaxLimPctEnaRvrt> {
        None
    }

    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    fn set_reversion_limit_max_power_pct_enable(&mut self, value: WMaxLimPctEnaRvrt) {
    }

    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    fn limit_max_power_pct_reversion_time(&self) -> Option<u32> {
        None
    }

    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    fn set_limit_max_power_pct_reversion_time(&mut self, value: u32) {
    }

    /// Limit Max Power Pct Rev Time Rem
    ///
    /// Limit maximum active power percent reversion time remaining.
    fn limit_max_power_pct_rev_time_rem(&self) -> Option<u32> {
        None
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn set_active_power_enable(&self) -> Option<WSetEna> {
        None
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn set_set_active_power_enable(&mut self, value: WSetEna) {
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn set_active_power_mode(&self) -> Option<WSetMod> {
        None
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn set_set_active_power_mode(&mut self, value: WSetMod) {
    }

    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    fn active_power_setpoint_w(&self) -> Option<i32> {
        None
    }

    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    fn set_active_power_setpoint_w(&mut self, value: i32) {
    }

    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    fn reversion_active_power_w(&self) -> Option<i32> {
        None
    }

    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    fn set_reversion_active_power_w(&mut self, value: i32) {
    }

    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    fn active_power_setpoint_pct(&self) -> Option<i16> {
        None
    }

    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    fn set_active_power_setpoint_pct(&mut self, value: i16) {
    }

    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    fn reversion_active_power_pct(&self) -> Option<i16> {
        None
    }

    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    fn set_reversion_active_power_pct(&mut self, value: i16) {
    }

    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    fn reversion_active_power_enable(&self) -> Option<WSetEnaRvrt> {
        None
    }

    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    fn set_reversion_active_power_enable(&mut self, value: WSetEnaRvrt) {
    }

    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    fn active_power_reversion_time(&self) -> Option<u32> {
        None
    }

    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    fn set_active_power_reversion_time(&mut self, value: u32) {
    }

    /// Active Power Rev Time Rem
    ///
    /// Set active power reversion time remaining.
    fn active_power_rev_time_rem(&self) -> Option<u32> {
        None
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn set_reactive_power_enable(&self) -> Option<VarSetEna> {
        None
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn set_set_reactive_power_enable(&mut self, value: VarSetEna) {
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn set_reactive_power_mode(&self) -> Option<VarSetMod> {
        None
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn set_set_reactive_power_mode(&mut self, value: VarSetMod) {
    }

    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    fn reactive_power_priority(&self) -> Option<VarSetPri> {
        None
    }

    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    fn set_reactive_power_priority(&mut self, value: VarSetPri) {
    }

    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    fn reactive_power_setpoint_vars(&self) -> Option<i32> {
        None
    }

    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    fn set_reactive_power_setpoint_vars(&mut self, value: i32) {
    }

    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    fn reversion_reactive_power_vars(&self) -> Option<i32> {
        None
    }

    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    fn set_reversion_reactive_power_vars(&mut self, value: i32) {
    }

    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    fn reactive_power_setpoint_pct(&self) -> Option<i16> {
        None
    }

    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    fn set_reactive_power_setpoint_pct(&mut self, value: i16) {
    }

    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    fn reversion_reactive_power_pct(&self) -> Option<i16> {
        None
    }

    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    fn set_reversion_reactive_power_pct(&mut self, value: i16) {
    }

    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    fn reversion_reactive_power_enable(&self) -> Option<VarSetEnaRvrt> {
        None
    }

    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    fn set_reversion_reactive_power_enable(&mut self, value: VarSetEnaRvrt) {
    }

    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    fn reactive_power_reversion_time(&self) -> Option<u32> {
        None
    }

    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    fn set_reactive_power_reversion_time(&mut self, value: u32) {
    }

    /// Reactive Power Rev Time Rem
    ///
    /// Set reactive power reversion time remaining.
    fn reactive_power_rev_time_rem(&self) -> Option<u32> {
        None
    }

    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    fn normal_ramp_rate(&self) -> Option<u16> {
        None
    }

    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    fn set_normal_ramp_rate(&mut self, value: u16) {
    }

    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    fn normal_ramp_rate_reference(&self) -> Option<WRmpRef> {
        None
    }

    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    fn set_normal_ramp_rate_reference(&mut self, value: WRmpRef) {
    }

    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    fn reactive_power_ramp_rate(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    fn set_reactive_power_ramp_rate(&mut self, value: u16) {
    }

    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    fn anti_islanding_enable(&self) -> Option<AntiIslEna> {
        None
    }

    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    fn set_anti_islanding_enable(&mut self, value: AntiIslEna) {
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn power_factor_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Limit Max Power Scale Factor
    ///
    /// Limit maximum power scale factor.
    fn limit_max_power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn active_power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Active Power Pct Scale Factor
    ///
    /// Active power pct scale factor.
    fn active_power_pct_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn reactive_power_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Pct Scale Factor
    ///
    /// Reactive power pct scale factor.
    fn reactive_power_pct_scale_factor(&self) -> Option<u16> {
        None
    }
}

pub enum PfwInjEna {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum PfwInjEnaRvrt {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum PfwAbsEna {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum PfwAbsEnaRvrt {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum WMaxLimPctEna {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum WMaxLimPctEnaRvrt {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum WSetEna {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum WSetMod {
    /// Active Power As Max Percent
    /// 
    /// Active power setting is percentage of maximum active power.
    WMaxPct = 0,
    /// Active Power As Watts
    /// 
    /// Active power setting is in watts.
    Watts = 1,
}

pub enum WSetEnaRvrt {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum VarSetEna {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum VarSetMod {
    /// Reactive Power As Watt Max Pct
    /// 
    /// Reactive power setting is percent of maximum active power.
    WMaxPct = 0,
    /// Reactive Power As Var Max Pct
    /// 
    /// Reactive power setting is percent of maximum reactive power.
    VarMaxPct = 1,
    /// Reactive Power As Var Avail Pct
    /// 
    /// Reactive power setting is percent of available reactive  power.
    VarAvailPct = 2,
    /// Reactive Power As VA Max Pct
    /// 
    /// Reactive power setting is percent of maximum apparent power.
    VaMaxPct = 3,
    /// Reactive Power As Vars
    /// 
    /// Reactive power is in vars.
    Vars = 4,
}

pub enum VarSetPri {
    /// Active Power Priority
    /// 
    /// Active power priority.
    Active = 0,
    /// Reactive Power Priority
    /// 
    /// Reactive power priority.
    Reactive = 1,
    /// Vendor Power Priority
    /// 
    /// Power priority is vendor specific mode.
    Vendor = 2,
}

pub enum VarSetEnaRvrt {
    /// Disabled
    /// 
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Function is enabled.
    Enabled = 1,
}

pub enum WRmpRef {
    /// Max Current Ramp
    /// 
    /// Ramp based on percent of max current per second.
    AMax = 0,
    /// Max Active Power Ramp
    /// 
    /// Ramp based on percent of max active power per second.
    WMax = 1,
}

pub enum AntiIslEna {
    /// Disabled
    /// 
    /// Anti-islanding is disabled.
    Disabled = 0,
    /// Enabled
    /// 
    /// Anti-islanding is enabled.
    Enabled = 1,
}