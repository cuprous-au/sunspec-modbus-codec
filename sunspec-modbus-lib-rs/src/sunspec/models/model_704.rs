use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
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
        reference: PointReference::Model704 { point: Point::ActivePowerEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ActivePowerMode },
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
        reference: PointReference::Model704 { point: Point::ReactivePowerEnable },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model704 { point: Point::ReactivePowerMode },
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
    ActivePowerEnable,
    ActivePowerMode,
    ActivePowerSetpointW,
    ReversionActivePowerW,
    ActivePowerSetpointPct,
    ReversionActivePowerPct,
    ReversionActivePowerEnable,
    ActivePowerReversionTime,
    ActivePowerRevTimeRem,
    ReactivePowerEnable,
    ReactivePowerMode,
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

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::PowerFactorEnableWInjEnable => {
            if let Some(value) = model.power_factor_enable_w_inj_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerFactorReversionEnableWInj => {
            if let Some(value) = model.power_factor_reversion_enable_w_inj() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfReversionTimeWInj => {
            if let Some(value) = model.pf_reversion_time_w_inj() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfReversionTimeRemWInj => {
            if let Some(value) = model.pf_reversion_time_rem_w_inj() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerFactorEnableWAbsEnable => {
            if let Some(value) = model.power_factor_enable_w_abs_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerFactorReversionEnableWAbs => {
            if let Some(value) = model.power_factor_reversion_enable_w_abs() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfReversionTimeWAbs => {
            if let Some(value) = model.pf_reversion_time_w_abs() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfReversionTimeRemWAbs => {
            if let Some(value) = model.pf_reversion_time_rem_w_abs() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LimitMaxPowerPctEnable => {
            if let Some(value) = model.limit_max_power_pct_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LimitMaxPowerPctSetpoint => {
            if let Some(value) = model.limit_max_power_pct_setpoint() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionLimitMaxPowerPct => {
            if let Some(value) = model.reversion_limit_max_power_pct() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionLimitMaxPowerPctEnable => {
            if let Some(value) = model.reversion_limit_max_power_pct_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LimitMaxPowerPctReversionTime => {
            if let Some(value) = model.limit_max_power_pct_reversion_time() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LimitMaxPowerPctRevTimeRem => {
            if let Some(value) = model.limit_max_power_pct_rev_time_rem() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerEnable => {
            if let Some(value) = model.active_power_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerMode => {
            if let Some(value) = model.active_power_mode() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerSetpointW => {
            if let Some(value) = model.active_power_setpoint_w() {
                buffer::write_i32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionActivePowerW => {
            if let Some(value) = model.reversion_active_power_w() {
                buffer::write_i32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerSetpointPct => {
            if let Some(value) = model.active_power_setpoint_pct() {
                buffer::write_i16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionActivePowerPct => {
            if let Some(value) = model.reversion_active_power_pct() {
                buffer::write_i16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionActivePowerEnable => {
            if let Some(value) = model.reversion_active_power_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerReversionTime => {
            if let Some(value) = model.active_power_reversion_time() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerRevTimeRem => {
            if let Some(value) = model.active_power_rev_time_rem() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerEnable => {
            if let Some(value) = model.reactive_power_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerMode => {
            if let Some(value) = model.reactive_power_mode() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerPriority => {
            if let Some(value) = model.reactive_power_priority() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerSetpointVars => {
            if let Some(value) = model.reactive_power_setpoint_vars() {
                buffer::write_i32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionReactivePowerVars => {
            if let Some(value) = model.reversion_reactive_power_vars() {
                buffer::write_i32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerSetpointPct => {
            if let Some(value) = model.reactive_power_setpoint_pct() {
                buffer::write_i16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionReactivePowerPct => {
            if let Some(value) = model.reversion_reactive_power_pct() {
                buffer::write_i16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionReactivePowerEnable => {
            if let Some(value) = model.reversion_reactive_power_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerReversionTime => {
            if let Some(value) = model.reactive_power_reversion_time() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerRevTimeRem => {
            if let Some(value) = model.reactive_power_rev_time_rem() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NormalRampRate => {
            if let Some(value) = model.normal_ramp_rate() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NormalRampRateReference => {
            if let Some(value) = model.normal_ramp_rate_reference() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerRampRate => {
            if let Some(value) = model.reactive_power_ramp_rate() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AntiIslandingEnable => {
            if let Some(value) = model.anti_islanding_enable() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PowerFactorScaleFactor => {
            if let Some(value) = model.power_factor_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::LimitMaxPowerScaleFactor => {
            if let Some(value) = model.limit_max_power_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerScaleFactor => {
            if let Some(value) = model.active_power_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ActivePowerPctScaleFactor => {
            if let Some(value) = model.active_power_pct_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerScaleFactor => {
            if let Some(value) = model.reactive_power_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReactivePowerPctScaleFactor => {
            if let Some(value) = model.reactive_power_pct_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
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
    fn active_power_enable(&self) -> Option<WSetEna> {
        None
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn set_active_power_enable(&mut self, value: WSetEna) {
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn active_power_mode(&self) -> Option<WSetMod> {
        None
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn set_active_power_mode(&mut self, value: WSetMod) {
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
    fn reactive_power_enable(&self) -> Option<VarSetEna> {
        None
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn set_reactive_power_enable(&mut self, value: VarSetEna) {
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn reactive_power_mode(&self) -> Option<VarSetMod> {
        None
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn set_reactive_power_mode(&mut self, value: VarSetMod) {
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[repr(C)]
pub struct Model704CallbackAdapter {
    context: *mut c_void,
    power_factor_enable_w_inj_enable_callback: Option<extern "C" fn(*const c_void) -> PfwInjEna>,
    set_power_factor_enable_w_inj_enable_callback: Option<extern "C" fn(PfwInjEna, *mut c_void)>,
    power_factor_reversion_enable_w_inj_callback: Option<extern "C" fn(*const c_void) -> PfwInjEnaRvrt>,
    set_power_factor_reversion_enable_w_inj_callback: Option<extern "C" fn(PfwInjEnaRvrt, *mut c_void)>,
    pf_reversion_time_w_inj_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_pf_reversion_time_w_inj_callback: Option<extern "C" fn(u32, *mut c_void)>,
    pf_reversion_time_rem_w_inj_callback: Option<extern "C" fn(*const c_void) -> u32>,
    power_factor_enable_w_abs_enable_callback: Option<extern "C" fn(*const c_void) -> PfwAbsEna>,
    set_power_factor_enable_w_abs_enable_callback: Option<extern "C" fn(PfwAbsEna, *mut c_void)>,
    power_factor_reversion_enable_w_abs_callback: Option<extern "C" fn(*const c_void) -> PfwAbsEnaRvrt>,
    set_power_factor_reversion_enable_w_abs_callback: Option<extern "C" fn(PfwAbsEnaRvrt, *mut c_void)>,
    pf_reversion_time_w_abs_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_pf_reversion_time_w_abs_callback: Option<extern "C" fn(u32, *mut c_void)>,
    pf_reversion_time_rem_w_abs_callback: Option<extern "C" fn(*const c_void) -> u32>,
    limit_max_power_pct_enable_callback: Option<extern "C" fn(*const c_void) -> WMaxLimPctEna>,
    set_limit_max_power_pct_enable_callback: Option<extern "C" fn(WMaxLimPctEna, *mut c_void)>,
    limit_max_power_pct_setpoint_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_limit_max_power_pct_setpoint_callback: Option<extern "C" fn(u16, *mut c_void)>,
    reversion_limit_max_power_pct_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_reversion_limit_max_power_pct_callback: Option<extern "C" fn(u16, *mut c_void)>,
    reversion_limit_max_power_pct_enable_callback: Option<extern "C" fn(*const c_void) -> WMaxLimPctEnaRvrt>,
    set_reversion_limit_max_power_pct_enable_callback: Option<extern "C" fn(WMaxLimPctEnaRvrt, *mut c_void)>,
    limit_max_power_pct_reversion_time_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_limit_max_power_pct_reversion_time_callback: Option<extern "C" fn(u32, *mut c_void)>,
    limit_max_power_pct_rev_time_rem_callback: Option<extern "C" fn(*const c_void) -> u32>,
    active_power_enable_callback: Option<extern "C" fn(*const c_void) -> WSetEna>,
    set_active_power_enable_callback: Option<extern "C" fn(WSetEna, *mut c_void)>,
    active_power_mode_callback: Option<extern "C" fn(*const c_void) -> WSetMod>,
    set_active_power_mode_callback: Option<extern "C" fn(WSetMod, *mut c_void)>,
    active_power_setpoint_w_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_active_power_setpoint_w_callback: Option<extern "C" fn(i32, *mut c_void)>,
    reversion_active_power_w_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_reversion_active_power_w_callback: Option<extern "C" fn(i32, *mut c_void)>,
    active_power_setpoint_pct_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_active_power_setpoint_pct_callback: Option<extern "C" fn(i16, *mut c_void)>,
    reversion_active_power_pct_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_reversion_active_power_pct_callback: Option<extern "C" fn(i16, *mut c_void)>,
    reversion_active_power_enable_callback: Option<extern "C" fn(*const c_void) -> WSetEnaRvrt>,
    set_reversion_active_power_enable_callback: Option<extern "C" fn(WSetEnaRvrt, *mut c_void)>,
    active_power_reversion_time_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_active_power_reversion_time_callback: Option<extern "C" fn(u32, *mut c_void)>,
    active_power_rev_time_rem_callback: Option<extern "C" fn(*const c_void) -> u32>,
    reactive_power_enable_callback: Option<extern "C" fn(*const c_void) -> VarSetEna>,
    set_reactive_power_enable_callback: Option<extern "C" fn(VarSetEna, *mut c_void)>,
    reactive_power_mode_callback: Option<extern "C" fn(*const c_void) -> VarSetMod>,
    set_reactive_power_mode_callback: Option<extern "C" fn(VarSetMod, *mut c_void)>,
    reactive_power_priority_callback: Option<extern "C" fn(*const c_void) -> VarSetPri>,
    set_reactive_power_priority_callback: Option<extern "C" fn(VarSetPri, *mut c_void)>,
    reactive_power_setpoint_vars_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_reactive_power_setpoint_vars_callback: Option<extern "C" fn(i32, *mut c_void)>,
    reversion_reactive_power_vars_callback: Option<extern "C" fn(*const c_void) -> i32>,
    set_reversion_reactive_power_vars_callback: Option<extern "C" fn(i32, *mut c_void)>,
    reactive_power_setpoint_pct_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_reactive_power_setpoint_pct_callback: Option<extern "C" fn(i16, *mut c_void)>,
    reversion_reactive_power_pct_callback: Option<extern "C" fn(*const c_void) -> i16>,
    set_reversion_reactive_power_pct_callback: Option<extern "C" fn(i16, *mut c_void)>,
    reversion_reactive_power_enable_callback: Option<extern "C" fn(*const c_void) -> VarSetEnaRvrt>,
    set_reversion_reactive_power_enable_callback: Option<extern "C" fn(VarSetEnaRvrt, *mut c_void)>,
    reactive_power_reversion_time_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_reactive_power_reversion_time_callback: Option<extern "C" fn(u32, *mut c_void)>,
    reactive_power_rev_time_rem_callback: Option<extern "C" fn(*const c_void) -> u32>,
    normal_ramp_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_normal_ramp_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    normal_ramp_rate_reference_callback: Option<extern "C" fn(*const c_void) -> WRmpRef>,
    set_normal_ramp_rate_reference_callback: Option<extern "C" fn(WRmpRef, *mut c_void)>,
    reactive_power_ramp_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_reactive_power_ramp_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    anti_islanding_enable_callback: Option<extern "C" fn(*const c_void) -> AntiIslEna>,
    set_anti_islanding_enable_callback: Option<extern "C" fn(AntiIslEna, *mut c_void)>,
    power_factor_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    limit_max_power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    active_power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    active_power_pct_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    reactive_power_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    reactive_power_pct_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model704CallbackAdapter {
    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    fn power_factor_enable_w_inj_enable(&self) -> Option<PfwInjEna> {
        self.power_factor_enable_w_inj_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    fn set_power_factor_enable_w_inj_enable(&mut self, value: PfwInjEna) {
        if let Some(callback) = self.set_power_factor_enable_w_inj_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    fn power_factor_reversion_enable_w_inj(&self) -> Option<PfwInjEnaRvrt> {
        self.power_factor_reversion_enable_w_inj_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    fn set_power_factor_reversion_enable_w_inj(&mut self, value: PfwInjEnaRvrt) {
        if let Some(callback) = self.set_power_factor_reversion_enable_w_inj_callback {
        (callback)(value, self.context);
        };
    }

    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    fn pf_reversion_time_w_inj(&self) -> Option<u32> {
        self.pf_reversion_time_w_inj_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    fn set_pf_reversion_time_w_inj(&mut self, value: u32) {
        if let Some(callback) = self.set_pf_reversion_time_w_inj_callback {
        (callback)(value, self.context);
        };
    }

    /// PF Reversion Time Rem (W Inj)
    ///
    /// Power factor reversion time remaining when injecting active power.
    fn pf_reversion_time_rem_w_inj(&self) -> Option<u32> {
        self.pf_reversion_time_rem_w_inj_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    fn power_factor_enable_w_abs_enable(&self) -> Option<PfwAbsEna> {
        self.power_factor_enable_w_abs_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    fn set_power_factor_enable_w_abs_enable(&mut self, value: PfwAbsEna) {
        if let Some(callback) = self.set_power_factor_enable_w_abs_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    fn power_factor_reversion_enable_w_abs(&self) -> Option<PfwAbsEnaRvrt> {
        self.power_factor_reversion_enable_w_abs_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    fn set_power_factor_reversion_enable_w_abs(&mut self, value: PfwAbsEnaRvrt) {
        if let Some(callback) = self.set_power_factor_reversion_enable_w_abs_callback {
        (callback)(value, self.context);
        };
    }

    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    fn pf_reversion_time_w_abs(&self) -> Option<u32> {
        self.pf_reversion_time_w_abs_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    fn set_pf_reversion_time_w_abs(&mut self, value: u32) {
        if let Some(callback) = self.set_pf_reversion_time_w_abs_callback {
        (callback)(value, self.context);
        };
    }

    /// PF Reversion Time Rem (W Abs)
    ///
    /// Power factor reversion time remaining when absorbing active power.
    fn pf_reversion_time_rem_w_abs(&self) -> Option<u32> {
        self.pf_reversion_time_rem_w_abs_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    fn limit_max_power_pct_enable(&self) -> Option<WMaxLimPctEna> {
        self.limit_max_power_pct_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    fn set_limit_max_power_pct_enable(&mut self, value: WMaxLimPctEna) {
        if let Some(callback) = self.set_limit_max_power_pct_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    fn limit_max_power_pct_setpoint(&self) -> Option<u16> {
        self.limit_max_power_pct_setpoint_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    fn set_limit_max_power_pct_setpoint(&mut self, value: u16) {
        if let Some(callback) = self.set_limit_max_power_pct_setpoint_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    fn reversion_limit_max_power_pct(&self) -> Option<u16> {
        self.reversion_limit_max_power_pct_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    fn set_reversion_limit_max_power_pct(&mut self, value: u16) {
        if let Some(callback) = self.set_reversion_limit_max_power_pct_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    fn reversion_limit_max_power_pct_enable(&self) -> Option<WMaxLimPctEnaRvrt> {
        self.reversion_limit_max_power_pct_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    fn set_reversion_limit_max_power_pct_enable(&mut self, value: WMaxLimPctEnaRvrt) {
        if let Some(callback) = self.set_reversion_limit_max_power_pct_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    fn limit_max_power_pct_reversion_time(&self) -> Option<u32> {
        self.limit_max_power_pct_reversion_time_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    fn set_limit_max_power_pct_reversion_time(&mut self, value: u32) {
        if let Some(callback) = self.set_limit_max_power_pct_reversion_time_callback {
        (callback)(value, self.context);
        };
    }

    /// Limit Max Power Pct Rev Time Rem
    ///
    /// Limit maximum active power percent reversion time remaining.
    fn limit_max_power_pct_rev_time_rem(&self) -> Option<u32> {
        self.limit_max_power_pct_rev_time_rem_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn active_power_enable(&self) -> Option<WSetEna> {
        self.active_power_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn set_active_power_enable(&mut self, value: WSetEna) {
        if let Some(callback) = self.set_active_power_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn active_power_mode(&self) -> Option<WSetMod> {
        self.active_power_mode_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn set_active_power_mode(&mut self, value: WSetMod) {
        if let Some(callback) = self.set_active_power_mode_callback {
        (callback)(value, self.context);
        };
    }

    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    fn active_power_setpoint_w(&self) -> Option<i32> {
        self.active_power_setpoint_w_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    fn set_active_power_setpoint_w(&mut self, value: i32) {
        if let Some(callback) = self.set_active_power_setpoint_w_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    fn reversion_active_power_w(&self) -> Option<i32> {
        self.reversion_active_power_w_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    fn set_reversion_active_power_w(&mut self, value: i32) {
        if let Some(callback) = self.set_reversion_active_power_w_callback {
        (callback)(value, self.context);
        };
    }

    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    fn active_power_setpoint_pct(&self) -> Option<i16> {
        self.active_power_setpoint_pct_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    fn set_active_power_setpoint_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_active_power_setpoint_pct_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    fn reversion_active_power_pct(&self) -> Option<i16> {
        self.reversion_active_power_pct_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    fn set_reversion_active_power_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_reversion_active_power_pct_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    fn reversion_active_power_enable(&self) -> Option<WSetEnaRvrt> {
        self.reversion_active_power_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    fn set_reversion_active_power_enable(&mut self, value: WSetEnaRvrt) {
        if let Some(callback) = self.set_reversion_active_power_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    fn active_power_reversion_time(&self) -> Option<u32> {
        self.active_power_reversion_time_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    fn set_active_power_reversion_time(&mut self, value: u32) {
        if let Some(callback) = self.set_active_power_reversion_time_callback {
        (callback)(value, self.context);
        };
    }

    /// Active Power Rev Time Rem
    ///
    /// Set active power reversion time remaining.
    fn active_power_rev_time_rem(&self) -> Option<u32> {
        self.active_power_rev_time_rem_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn reactive_power_enable(&self) -> Option<VarSetEna> {
        self.reactive_power_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn set_reactive_power_enable(&mut self, value: VarSetEna) {
        if let Some(callback) = self.set_reactive_power_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn reactive_power_mode(&self) -> Option<VarSetMod> {
        self.reactive_power_mode_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn set_reactive_power_mode(&mut self, value: VarSetMod) {
        if let Some(callback) = self.set_reactive_power_mode_callback {
        (callback)(value, self.context);
        };
    }

    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    fn reactive_power_priority(&self) -> Option<VarSetPri> {
        self.reactive_power_priority_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    fn set_reactive_power_priority(&mut self, value: VarSetPri) {
        if let Some(callback) = self.set_reactive_power_priority_callback {
        (callback)(value, self.context);
        };
    }

    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    fn reactive_power_setpoint_vars(&self) -> Option<i32> {
        self.reactive_power_setpoint_vars_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    fn set_reactive_power_setpoint_vars(&mut self, value: i32) {
        if let Some(callback) = self.set_reactive_power_setpoint_vars_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    fn reversion_reactive_power_vars(&self) -> Option<i32> {
        self.reversion_reactive_power_vars_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    fn set_reversion_reactive_power_vars(&mut self, value: i32) {
        if let Some(callback) = self.set_reversion_reactive_power_vars_callback {
        (callback)(value, self.context);
        };
    }

    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    fn reactive_power_setpoint_pct(&self) -> Option<i16> {
        self.reactive_power_setpoint_pct_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    fn set_reactive_power_setpoint_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_reactive_power_setpoint_pct_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    fn reversion_reactive_power_pct(&self) -> Option<i16> {
        self.reversion_reactive_power_pct_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    fn set_reversion_reactive_power_pct(&mut self, value: i16) {
        if let Some(callback) = self.set_reversion_reactive_power_pct_callback {
        (callback)(value, self.context);
        };
    }

    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    fn reversion_reactive_power_enable(&self) -> Option<VarSetEnaRvrt> {
        self.reversion_reactive_power_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    fn set_reversion_reactive_power_enable(&mut self, value: VarSetEnaRvrt) {
        if let Some(callback) = self.set_reversion_reactive_power_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    fn reactive_power_reversion_time(&self) -> Option<u32> {
        self.reactive_power_reversion_time_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    fn set_reactive_power_reversion_time(&mut self, value: u32) {
        if let Some(callback) = self.set_reactive_power_reversion_time_callback {
        (callback)(value, self.context);
        };
    }

    /// Reactive Power Rev Time Rem
    ///
    /// Set reactive power reversion time remaining.
    fn reactive_power_rev_time_rem(&self) -> Option<u32> {
        self.reactive_power_rev_time_rem_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    fn normal_ramp_rate(&self) -> Option<u16> {
        self.normal_ramp_rate_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    fn set_normal_ramp_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_normal_ramp_rate_callback {
        (callback)(value, self.context);
        };
    }

    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    fn normal_ramp_rate_reference(&self) -> Option<WRmpRef> {
        self.normal_ramp_rate_reference_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    fn set_normal_ramp_rate_reference(&mut self, value: WRmpRef) {
        if let Some(callback) = self.set_normal_ramp_rate_reference_callback {
        (callback)(value, self.context);
        };
    }

    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    fn reactive_power_ramp_rate(&self) -> Option<u16> {
        self.reactive_power_ramp_rate_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    fn set_reactive_power_ramp_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_reactive_power_ramp_rate_callback {
        (callback)(value, self.context);
        };
    }

    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    fn anti_islanding_enable(&self) -> Option<AntiIslEna> {
        self.anti_islanding_enable_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    fn set_anti_islanding_enable(&mut self, value: AntiIslEna) {
        if let Some(callback) = self.set_anti_islanding_enable_callback {
        (callback)(value, self.context);
        };
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn power_factor_scale_factor(&self) -> Option<u16> {
        self.power_factor_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Limit Max Power Scale Factor
    ///
    /// Limit maximum power scale factor.
    fn limit_max_power_scale_factor(&self) -> Option<u16> {
        self.limit_max_power_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn active_power_scale_factor(&self) -> Option<u16> {
        self.active_power_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Active Power Pct Scale Factor
    ///
    /// Active power pct scale factor.
    fn active_power_pct_scale_factor(&self) -> Option<u16> {
        self.active_power_pct_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn reactive_power_scale_factor(&self) -> Option<u16> {
        self.reactive_power_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Reactive Power Pct Scale Factor
    ///
    /// Reactive power pct scale factor.
    fn reactive_power_pct_scale_factor(&self) -> Option<u16> {
        self.reactive_power_pct_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model704StatefulAdapter {
    power_factor_enable_w_inj_enable: PfwInjEna,
    power_factor_reversion_enable_w_inj: PfwInjEnaRvrt,
    pf_reversion_time_w_inj: u32,
    pf_reversion_time_rem_w_inj: u32,
    power_factor_enable_w_abs_enable: PfwAbsEna,
    power_factor_reversion_enable_w_abs: PfwAbsEnaRvrt,
    pf_reversion_time_w_abs: u32,
    pf_reversion_time_rem_w_abs: u32,
    limit_max_power_pct_enable: WMaxLimPctEna,
    limit_max_power_pct_setpoint: u16,
    reversion_limit_max_power_pct: u16,
    reversion_limit_max_power_pct_enable: WMaxLimPctEnaRvrt,
    limit_max_power_pct_reversion_time: u32,
    limit_max_power_pct_rev_time_rem: u32,
    active_power_enable: WSetEna,
    active_power_mode: WSetMod,
    active_power_setpoint_w: i32,
    reversion_active_power_w: i32,
    active_power_setpoint_pct: i16,
    reversion_active_power_pct: i16,
    reversion_active_power_enable: WSetEnaRvrt,
    active_power_reversion_time: u32,
    active_power_rev_time_rem: u32,
    reactive_power_enable: VarSetEna,
    reactive_power_mode: VarSetMod,
    reactive_power_priority: VarSetPri,
    reactive_power_setpoint_vars: i32,
    reversion_reactive_power_vars: i32,
    reactive_power_setpoint_pct: i16,
    reversion_reactive_power_pct: i16,
    reversion_reactive_power_enable: VarSetEnaRvrt,
    reactive_power_reversion_time: u32,
    reactive_power_rev_time_rem: u32,
    normal_ramp_rate: u16,
    normal_ramp_rate_reference: WRmpRef,
    reactive_power_ramp_rate: u16,
    anti_islanding_enable: AntiIslEna,
    power_factor_scale_factor: u16,
    limit_max_power_scale_factor: u16,
    active_power_scale_factor: u16,
    active_power_pct_scale_factor: u16,
    reactive_power_scale_factor: u16,
    reactive_power_pct_scale_factor: u16,
}

impl ModelAdapter for Model704StatefulAdapter {
    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    fn power_factor_enable_w_inj_enable(&self) -> Option<PfwInjEna> {
        Some(
        self.power_factor_enable_w_inj_enable
        )
    }

    /// Power Factor Enable (W Inj) Enable
    ///
    /// Power factor enable when injecting active power.
    fn set_power_factor_enable_w_inj_enable(&mut self, value: PfwInjEna) {
        self.power_factor_enable_w_inj_enable = value;
    }

    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    fn power_factor_reversion_enable_w_inj(&self) -> Option<PfwInjEnaRvrt> {
        Some(
        self.power_factor_reversion_enable_w_inj
        )
    }

    /// Power Factor Reversion Enable (W Inj)
    ///
    /// Power factor reversion timer when injecting active power enable.
    fn set_power_factor_reversion_enable_w_inj(&mut self, value: PfwInjEnaRvrt) {
        self.power_factor_reversion_enable_w_inj = value;
    }

    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    fn pf_reversion_time_w_inj(&self) -> Option<u32> {
        Some(
        self.pf_reversion_time_w_inj
        )
    }

    /// PF Reversion Time (W Inj)
    ///
    /// Power factor reversion timer when injecting active power.
    fn set_pf_reversion_time_w_inj(&mut self, value: u32) {
        self.pf_reversion_time_w_inj = value;
    }

    /// PF Reversion Time Rem (W Inj)
    ///
    /// Power factor reversion time remaining when injecting active power.
    fn pf_reversion_time_rem_w_inj(&self) -> Option<u32> {
        Some(
        self.pf_reversion_time_rem_w_inj
        )
    }

    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    fn power_factor_enable_w_abs_enable(&self) -> Option<PfwAbsEna> {
        Some(
        self.power_factor_enable_w_abs_enable
        )
    }

    /// Power Factor Enable (W Abs) Enable
    ///
    /// Power factor enable when absorbing active power.
    fn set_power_factor_enable_w_abs_enable(&mut self, value: PfwAbsEna) {
        self.power_factor_enable_w_abs_enable = value;
    }

    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    fn power_factor_reversion_enable_w_abs(&self) -> Option<PfwAbsEnaRvrt> {
        Some(
        self.power_factor_reversion_enable_w_abs
        )
    }

    /// Power Factor Reversion Enable (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power enable.
    fn set_power_factor_reversion_enable_w_abs(&mut self, value: PfwAbsEnaRvrt) {
        self.power_factor_reversion_enable_w_abs = value;
    }

    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    fn pf_reversion_time_w_abs(&self) -> Option<u32> {
        Some(
        self.pf_reversion_time_w_abs
        )
    }

    /// PF Reversion Time (W Abs)
    ///
    /// Power factor reversion timer when absorbing active power.
    fn set_pf_reversion_time_w_abs(&mut self, value: u32) {
        self.pf_reversion_time_w_abs = value;
    }

    /// PF Reversion Time Rem (W Abs)
    ///
    /// Power factor reversion time remaining when absorbing active power.
    fn pf_reversion_time_rem_w_abs(&self) -> Option<u32> {
        Some(
        self.pf_reversion_time_rem_w_abs
        )
    }

    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    fn limit_max_power_pct_enable(&self) -> Option<WMaxLimPctEna> {
        Some(
        self.limit_max_power_pct_enable
        )
    }

    /// Limit Max Power Pct Enable
    ///
    /// Limit maximum active power percent enable.
    fn set_limit_max_power_pct_enable(&mut self, value: WMaxLimPctEna) {
        self.limit_max_power_pct_enable = value;
    }

    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    fn limit_max_power_pct_setpoint(&self) -> Option<u16> {
        Some(
        self.limit_max_power_pct_setpoint
        )
    }

    /// Limit Max Power Pct Setpoint
    ///
    /// Limit maximum active power percent value.
    fn set_limit_max_power_pct_setpoint(&mut self, value: u16) {
        self.limit_max_power_pct_setpoint = value;
    }

    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    fn reversion_limit_max_power_pct(&self) -> Option<u16> {
        Some(
        self.reversion_limit_max_power_pct
        )
    }

    /// Reversion Limit Max Power Pct
    ///
    /// Reversion limit maximum active power percent value.
    fn set_reversion_limit_max_power_pct(&mut self, value: u16) {
        self.reversion_limit_max_power_pct = value;
    }

    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    fn reversion_limit_max_power_pct_enable(&self) -> Option<WMaxLimPctEnaRvrt> {
        Some(
        self.reversion_limit_max_power_pct_enable
        )
    }

    /// Reversion Limit Max Power Pct Enable
    ///
    /// Reversion limit maximum active power percent value enable.
    fn set_reversion_limit_max_power_pct_enable(&mut self, value: WMaxLimPctEnaRvrt) {
        self.reversion_limit_max_power_pct_enable = value;
    }

    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    fn limit_max_power_pct_reversion_time(&self) -> Option<u32> {
        Some(
        self.limit_max_power_pct_reversion_time
        )
    }

    /// Limit Max Power Pct Reversion Time
    ///
    /// Limit maximum active power percent reversion time.
    fn set_limit_max_power_pct_reversion_time(&mut self, value: u32) {
        self.limit_max_power_pct_reversion_time = value;
    }

    /// Limit Max Power Pct Rev Time Rem
    ///
    /// Limit maximum active power percent reversion time remaining.
    fn limit_max_power_pct_rev_time_rem(&self) -> Option<u32> {
        Some(
        self.limit_max_power_pct_rev_time_rem
        )
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn active_power_enable(&self) -> Option<WSetEna> {
        Some(
        self.active_power_enable
        )
    }

    /// Set Active Power Enable
    ///
    /// Set active power enable.
    fn set_active_power_enable(&mut self, value: WSetEna) {
        self.active_power_enable = value;
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn active_power_mode(&self) -> Option<WSetMod> {
        Some(
        self.active_power_mode
        )
    }

    /// Set Active Power Mode
    ///
    /// Set active power mode.
    fn set_active_power_mode(&mut self, value: WSetMod) {
        self.active_power_mode = value;
    }

    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    fn active_power_setpoint_w(&self) -> Option<i32> {
        Some(
        self.active_power_setpoint_w
        )
    }

    /// Active Power Setpoint (W)
    ///
    /// Active power setting value in watts.
    fn set_active_power_setpoint_w(&mut self, value: i32) {
        self.active_power_setpoint_w = value;
    }

    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    fn reversion_active_power_w(&self) -> Option<i32> {
        Some(
        self.reversion_active_power_w
        )
    }

    /// Reversion Active Power (W)
    ///
    /// Reversion active power setting value in watts.
    fn set_reversion_active_power_w(&mut self, value: i32) {
        self.reversion_active_power_w = value;
    }

    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    fn active_power_setpoint_pct(&self) -> Option<i16> {
        Some(
        self.active_power_setpoint_pct
        )
    }

    /// Active Power Setpoint (Pct)
    ///
    /// Active power setting value as percent.
    fn set_active_power_setpoint_pct(&mut self, value: i16) {
        self.active_power_setpoint_pct = value;
    }

    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    fn reversion_active_power_pct(&self) -> Option<i16> {
        Some(
        self.reversion_active_power_pct
        )
    }

    /// Reversion Active Power (Pct)
    ///
    /// Reversion active power setting value as percent.
    fn set_reversion_active_power_pct(&mut self, value: i16) {
        self.reversion_active_power_pct = value;
    }

    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    fn reversion_active_power_enable(&self) -> Option<WSetEnaRvrt> {
        Some(
        self.reversion_active_power_enable
        )
    }

    /// Reversion Active Power Enable
    ///
    /// Reversion active power function enable.
    fn set_reversion_active_power_enable(&mut self, value: WSetEnaRvrt) {
        self.reversion_active_power_enable = value;
    }

    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    fn active_power_reversion_time(&self) -> Option<u32> {
        Some(
        self.active_power_reversion_time
        )
    }

    /// Active Power Reversion Time
    ///
    /// Set active power reversion time.
    fn set_active_power_reversion_time(&mut self, value: u32) {
        self.active_power_reversion_time = value;
    }

    /// Active Power Rev Time Rem
    ///
    /// Set active power reversion time remaining.
    fn active_power_rev_time_rem(&self) -> Option<u32> {
        Some(
        self.active_power_rev_time_rem
        )
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn reactive_power_enable(&self) -> Option<VarSetEna> {
        Some(
        self.reactive_power_enable
        )
    }

    /// Set Reactive Power Enable
    ///
    /// Set reactive power enable.
    fn set_reactive_power_enable(&mut self, value: VarSetEna) {
        self.reactive_power_enable = value;
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn reactive_power_mode(&self) -> Option<VarSetMod> {
        Some(
        self.reactive_power_mode
        )
    }

    /// Set Reactive Power Mode
    ///
    /// Set reactive power mode.
    fn set_reactive_power_mode(&mut self, value: VarSetMod) {
        self.reactive_power_mode = value;
    }

    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    fn reactive_power_priority(&self) -> Option<VarSetPri> {
        Some(
        self.reactive_power_priority
        )
    }

    /// Reactive Power Priority
    ///
    /// Reactive power priority.
    fn set_reactive_power_priority(&mut self, value: VarSetPri) {
        self.reactive_power_priority = value;
    }

    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    fn reactive_power_setpoint_vars(&self) -> Option<i32> {
        Some(
        self.reactive_power_setpoint_vars
        )
    }

    /// Reactive Power Setpoint (Vars)
    ///
    /// Reactive power setting value in vars.
    fn set_reactive_power_setpoint_vars(&mut self, value: i32) {
        self.reactive_power_setpoint_vars = value;
    }

    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    fn reversion_reactive_power_vars(&self) -> Option<i32> {
        Some(
        self.reversion_reactive_power_vars
        )
    }

    /// Reversion Reactive Power (Vars)
    ///
    /// Reversion reactive power setting value in vars.
    fn set_reversion_reactive_power_vars(&mut self, value: i32) {
        self.reversion_reactive_power_vars = value;
    }

    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    fn reactive_power_setpoint_pct(&self) -> Option<i16> {
        Some(
        self.reactive_power_setpoint_pct
        )
    }

    /// Reactive Power Setpoint (Pct)
    ///
    /// Reactive power setting value as percent.
    fn set_reactive_power_setpoint_pct(&mut self, value: i16) {
        self.reactive_power_setpoint_pct = value;
    }

    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    fn reversion_reactive_power_pct(&self) -> Option<i16> {
        Some(
        self.reversion_reactive_power_pct
        )
    }

    /// Reversion Reactive Power (Pct)
    ///
    /// Reversion reactive power setting value as percent.
    fn set_reversion_reactive_power_pct(&mut self, value: i16) {
        self.reversion_reactive_power_pct = value;
    }

    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    fn reversion_reactive_power_enable(&self) -> Option<VarSetEnaRvrt> {
        Some(
        self.reversion_reactive_power_enable
        )
    }

    /// Reversion Reactive Power Enable
    ///
    /// Reversion reactive power function enable.
    fn set_reversion_reactive_power_enable(&mut self, value: VarSetEnaRvrt) {
        self.reversion_reactive_power_enable = value;
    }

    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    fn reactive_power_reversion_time(&self) -> Option<u32> {
        Some(
        self.reactive_power_reversion_time
        )
    }

    /// Reactive Power Reversion Time
    ///
    /// Set reactive power reversion time.
    fn set_reactive_power_reversion_time(&mut self, value: u32) {
        self.reactive_power_reversion_time = value;
    }

    /// Reactive Power Rev Time Rem
    ///
    /// Set reactive power reversion time remaining.
    fn reactive_power_rev_time_rem(&self) -> Option<u32> {
        Some(
        self.reactive_power_rev_time_rem
        )
    }

    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    fn normal_ramp_rate(&self) -> Option<u16> {
        Some(
        self.normal_ramp_rate
        )
    }

    /// Normal Ramp Rate
    ///
    /// Ramp rate for increases in active power during normal generation.
    fn set_normal_ramp_rate(&mut self, value: u16) {
        self.normal_ramp_rate = value;
    }

    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    fn normal_ramp_rate_reference(&self) -> Option<WRmpRef> {
        Some(
        self.normal_ramp_rate_reference
        )
    }

    /// Normal Ramp Rate Reference
    ///
    /// Ramp rate reference unit for increases in active power or current during normal generation.
    fn set_normal_ramp_rate_reference(&mut self, value: WRmpRef) {
        self.normal_ramp_rate_reference = value;
    }

    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    fn reactive_power_ramp_rate(&self) -> Option<u16> {
        Some(
        self.reactive_power_ramp_rate
        )
    }

    /// Reactive Power Ramp Rate
    ///
    /// Ramp rate based on max reactive power per second.
    fn set_reactive_power_ramp_rate(&mut self, value: u16) {
        self.reactive_power_ramp_rate = value;
    }

    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    fn anti_islanding_enable(&self) -> Option<AntiIslEna> {
        Some(
        self.anti_islanding_enable
        )
    }

    /// Anti-Islanding Enable
    ///
    /// Anti-islanding enable.
    fn set_anti_islanding_enable(&mut self, value: AntiIslEna) {
        self.anti_islanding_enable = value;
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn power_factor_scale_factor(&self) -> Option<u16> {
        Some(
        self.power_factor_scale_factor
        )
    }

    /// Limit Max Power Scale Factor
    ///
    /// Limit maximum power scale factor.
    fn limit_max_power_scale_factor(&self) -> Option<u16> {
        Some(
        self.limit_max_power_scale_factor
        )
    }

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn active_power_scale_factor(&self) -> Option<u16> {
        Some(
        self.active_power_scale_factor
        )
    }

    /// Active Power Pct Scale Factor
    ///
    /// Active power pct scale factor.
    fn active_power_pct_scale_factor(&self) -> Option<u16> {
        Some(
        self.active_power_pct_scale_factor
        )
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn reactive_power_scale_factor(&self) -> Option<u16> {
        Some(
        self.reactive_power_scale_factor
        )
    }

    /// Reactive Power Pct Scale Factor
    ///
    /// Reactive power pct scale factor.
    fn reactive_power_pct_scale_factor(&self) -> Option<u16> {
        Some(
        self.reactive_power_pct_scale_factor
        )
    }
}