use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 14;

static POINTS: [PointDetails<()>; 12] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::DerFrequencyDroopModuleEnable,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::ActiveControlRequest,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::SetActiveControlResult,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::StoredControlCount,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::ReversionTimeout,
        size: 2,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::ReversionTimeLeft,
        size: 2,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::ReversionControl,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::DeadbandScaleFactor,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::FrequencyChangeScaleFactor,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::OpenLoopScaleFactor,
        size: 1,
        start_address: 13,
    },
];

static CTL_POINTS: [PointDetails<u16>; 7] = [
    PointDetails {
        point: |ctl_index| Point::CtlOverFrequencyDeadband { ctl_index },
        size: 2,
        start_address: 0,
    },
    PointDetails {
        point: |ctl_index| Point::CtlUnderFrequencyDeadband { ctl_index },
        size: 2,
        start_address: 2,
    },
    PointDetails {
        point: |ctl_index| Point::CtlOverFrequencyChangeRatio { ctl_index },
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |ctl_index| Point::CtlUnderFrequencyChangeRatio { ctl_index },
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |ctl_index| Point::CtlOpenLoopResponseTime { ctl_index },
        size: 2,
        start_address: 6,
    },
    PointDetails {
        point: |ctl_index| Point::CtlMinimumActivePower { ctl_index },
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |ctl_index| Point::CtlControlAccess { ctl_index },
        size: 1,
        start_address: 9,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
    DerFrequencyDroopModuleEnable,
    ActiveControlRequest,
    SetActiveControlResult,
    StoredControlCount,
    ReversionTimeout,
    ReversionTimeLeft,
    ReversionControl,
    DeadbandScaleFactor,
    FrequencyChangeScaleFactor,
    OpenLoopScaleFactor,
    CtlOverFrequencyDeadband { ctl_index: u16 },
    CtlUnderFrequencyDeadband { ctl_index: u16 },
    CtlOverFrequencyChangeRatio { ctl_index: u16 },
    CtlUnderFrequencyChangeRatio { ctl_index: u16 },
    CtlOpenLoopResponseTime { ctl_index: u16 },
    CtlMinimumActivePower { ctl_index: u16 },
    CtlControlAccess { ctl_index: u16 },
}

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    14 + model.stored_control_count() * (10)
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    let ctl_count = model.stored_control_count();
    let ctl_size = 10;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .chain((0..ctl_count).flat_map(move |ctl_index| {
            let ctl_address = 14 + ctl_index * ctl_size;
            CTL_POINTS
                .iter()
                .map(move |p| (ctl_address + p.start_address, p.size, (p.point)(ctl_index)))
        }))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(711, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::DerFrequencyDroopModuleEnable => {
            buffer::write_u16(model.der_frequency_droop_module_enable() as u16, buffer);
        }
        Point::ActiveControlRequest => {
            buffer::write_u16(model.active_control_request(), buffer);
        }
        Point::SetActiveControlResult => {
            buffer::write_u16(model.set_active_control_result() as u16, buffer);
        }
        Point::StoredControlCount => {
            buffer::write_u16(model.stored_control_count(), buffer);
        }
        Point::ReversionTimeout => {
            if let Some(value) = model.reversion_timeout() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionTimeLeft => {
            if let Some(value) = model.reversion_time_left() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ReversionControl => {
            if let Some(value) = model.reversion_control() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DeadbandScaleFactor => {
            buffer::write_u16(model.deadband_scale_factor(), buffer);
        }
        Point::FrequencyChangeScaleFactor => {
            buffer::write_u16(model.frequency_change_scale_factor(), buffer);
        }
        Point::OpenLoopScaleFactor => {
            buffer::write_u16(model.open_loop_scale_factor(), buffer);
        }
        Point::CtlOverFrequencyDeadband { ctl_index } => {
            buffer::write_u32(
                model.ctl_over_frequency_deadband(*ctl_index),
                buffer,
                offset,
                limit,
            );
        }
        Point::CtlUnderFrequencyDeadband { ctl_index } => {
            buffer::write_u32(
                model.ctl_under_frequency_deadband(*ctl_index),
                buffer,
                offset,
                limit,
            );
        }
        Point::CtlOverFrequencyChangeRatio { ctl_index } => {
            buffer::write_u16(model.ctl_over_frequency_change_ratio(*ctl_index), buffer);
        }
        Point::CtlUnderFrequencyChangeRatio { ctl_index } => {
            buffer::write_u16(model.ctl_under_frequency_change_ratio(*ctl_index), buffer);
        }
        Point::CtlOpenLoopResponseTime { ctl_index } => {
            buffer::write_u32(
                model.ctl_open_loop_response_time(*ctl_index),
                buffer,
                offset,
                limit,
            );
        }
        Point::CtlMinimumActivePower { ctl_index } => {
            if let Some(value) = model.ctl_minimum_active_power(*ctl_index) {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CtlControlAccess { ctl_index } => {
            buffer::write_u16(model.ctl_control_access(*ctl_index) as u16, buffer);
        }
    }
}

pub trait ModelAdapter {
    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn der_frequency_droop_module_enable(&self) -> Ena;

    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn set_der_frequency_droop_module_enable(&mut self, value: Ena);

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn active_control_request(&self) -> u16;

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn set_active_control_request(&mut self, value: u16);

    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    fn set_active_control_result(&self) -> AdptCtlRslt;

    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    fn stored_control_count(&self) -> u16;

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn reversion_timeout(&self) -> Option<u32> {
        None
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {}

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
        None
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn reversion_control(&self) -> Option<u16> {
        None
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn set_reversion_control(&mut self, value: u16) {}

    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    fn deadband_scale_factor(&self) -> u16;

    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    fn frequency_change_scale_factor(&self) -> u16;

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16;

    /// Over-Frequency Deadband
    ///
    /// The deadband value for over-frequency conditions in Hz.
    fn ctl_over_frequency_deadband(&self, ctl_index: u16) -> u32;

    /// Over-Frequency Deadband
    ///
    /// The deadband value for over-frequency conditions in Hz.
    fn set_ctl_over_frequency_deadband(&mut self, value: u32, ctl_index: u16);

    /// Under-Frequency Deadband
    ///
    /// The deadband value for under-frequency conditions in Hz.
    fn ctl_under_frequency_deadband(&self, ctl_index: u16) -> u32;

    /// Under-Frequency Deadband
    ///
    /// The deadband value for under-frequency conditions in Hz.
    fn set_ctl_under_frequency_deadband(&mut self, value: u32, ctl_index: u16);

    /// Over-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change.
    fn ctl_over_frequency_change_ratio(&self, ctl_index: u16) -> u16;

    /// Over-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change.
    fn set_ctl_over_frequency_change_ratio(&mut self, value: u16, ctl_index: u16);

    /// Under-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change.
    fn ctl_under_frequency_change_ratio(&self, ctl_index: u16) -> u16;

    /// Under-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change.
    fn set_ctl_under_frequency_change_ratio(&mut self, value: u16, ctl_index: u16);

    /// Open-Loop Response Time
    ///
    /// The open-loop response time in seconds.
    fn ctl_open_loop_response_time(&self, ctl_index: u16) -> u32;

    /// Open-Loop Response Time
    ///
    /// The open-loop response time in seconds.
    fn set_ctl_open_loop_response_time(&mut self, value: u32, ctl_index: u16);

    /// Minimum Active Power
    ///
    /// The minimum active power output due to DER prime mover constraints, in percent of the DER active power rating. The valid range is -100 to 100. This setting applies only to the frequency droop control.
    fn ctl_minimum_active_power(&self, ctl_index: u16) -> Option<i16> {
        None
    }

    /// Minimum Active Power
    ///
    /// The minimum active power output due to DER prime mover constraints, in percent of the DER active power rating. The valid range is -100 to 100. This setting applies only to the frequency droop control.
    fn set_ctl_minimum_active_power(&mut self, value: i16, ctl_index: u16) {}

    /// Control Access
    ///
    /// Control read-write access.
    fn ctl_control_access(&self, ctl_index: u16) -> ReadOnly;
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum AdptCtlRslt {
    /// Update In Progress
    ///
    /// Control update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Control update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Control update failed.
    Failed = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Ena {
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
pub enum ReadOnly {
    /// Read-Write Access
    ///
    /// Control has read-write access.
    Rw = 0,
    /// Read-Only Access
    ///
    /// Control has read-only access.
    R = 1,
}

#[repr(C)]
pub struct Model711CallbackAdapter {
    context: *mut c_void,
    der_frequency_droop_module_enable_callback: extern "C" fn(*const c_void) -> Ena,
    set_der_frequency_droop_module_enable_callback: extern "C" fn(Ena, *mut c_void),
    active_control_request_callback: extern "C" fn(*const c_void) -> u16,
    set_active_control_request_callback: extern "C" fn(u16, *mut c_void),
    set_active_control_result_callback: extern "C" fn(*const c_void) -> AdptCtlRslt,
    stored_control_count_callback: extern "C" fn(*const c_void) -> u16,
    reversion_timeout_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_reversion_timeout_callback: Option<extern "C" fn(u32, *mut c_void)>,
    reversion_time_left_callback: Option<extern "C" fn(*const c_void) -> u32>,
    reversion_control_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_reversion_control_callback: Option<extern "C" fn(u16, *mut c_void)>,
    deadband_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    frequency_change_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    open_loop_scale_factor_callback: extern "C" fn(*const c_void) -> u16,
    ctl_over_frequency_deadband_callback: extern "C" fn(*const c_void, u16) -> u32,
    set_ctl_over_frequency_deadband_callback: extern "C" fn(u32, *mut c_void, u16),
    ctl_under_frequency_deadband_callback: extern "C" fn(*const c_void, u16) -> u32,
    set_ctl_under_frequency_deadband_callback: extern "C" fn(u32, *mut c_void, u16),
    ctl_over_frequency_change_ratio_callback: extern "C" fn(*const c_void, u16) -> u16,
    set_ctl_over_frequency_change_ratio_callback: extern "C" fn(u16, *mut c_void, u16),
    ctl_under_frequency_change_ratio_callback: extern "C" fn(*const c_void, u16) -> u16,
    set_ctl_under_frequency_change_ratio_callback: extern "C" fn(u16, *mut c_void, u16),
    ctl_open_loop_response_time_callback: extern "C" fn(*const c_void, u16) -> u32,
    set_ctl_open_loop_response_time_callback: extern "C" fn(u32, *mut c_void, u16),
    ctl_minimum_active_power_callback: Option<extern "C" fn(*const c_void, u16) -> i16>,
    set_ctl_minimum_active_power_callback: Option<extern "C" fn(i16, *mut c_void, u16)>,
    ctl_control_access_callback: extern "C" fn(*const c_void, u16) -> ReadOnly,
}

impl ModelAdapter for Model711CallbackAdapter {
    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn der_frequency_droop_module_enable(&self) -> Ena {
        (self.der_frequency_droop_module_enable_callback)(self.context)
    }

    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn set_der_frequency_droop_module_enable(&mut self, value: Ena) {
        (self.set_der_frequency_droop_module_enable_callback)(value, self.context);
    }

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn active_control_request(&self) -> u16 {
        (self.active_control_request_callback)(self.context)
    }

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn set_active_control_request(&mut self, value: u16) {
        (self.set_active_control_request_callback)(value, self.context);
    }

    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    fn set_active_control_result(&self) -> AdptCtlRslt {
        (self.set_active_control_result_callback)(self.context)
    }

    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    fn stored_control_count(&self) -> u16 {
        (self.stored_control_count_callback)(self.context)
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn reversion_timeout(&self) -> Option<u32> {
        self.reversion_timeout_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {
        if let Some(callback) = self.set_reversion_timeout_callback {
            (callback)(value, self.context);
        };
    }

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
        self.reversion_time_left_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn reversion_control(&self) -> Option<u16> {
        self.reversion_control_callback
            .map(|callback| (callback)(self.context))
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn set_reversion_control(&mut self, value: u16) {
        if let Some(callback) = self.set_reversion_control_callback {
            (callback)(value, self.context);
        };
    }

    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    fn deadband_scale_factor(&self) -> u16 {
        (self.deadband_scale_factor_callback)(self.context)
    }

    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    fn frequency_change_scale_factor(&self) -> u16 {
        (self.frequency_change_scale_factor_callback)(self.context)
    }

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16 {
        (self.open_loop_scale_factor_callback)(self.context)
    }

    /// Over-Frequency Deadband
    ///
    /// The deadband value for over-frequency conditions in Hz.
    fn ctl_over_frequency_deadband(&self, ctl_index: u16) -> u32 {
        (self.ctl_over_frequency_deadband_callback)(self.context, ctl_index)
    }

    /// Over-Frequency Deadband
    ///
    /// The deadband value for over-frequency conditions in Hz.
    fn set_ctl_over_frequency_deadband(&mut self, value: u32, ctl_index: u16) {
        (self.set_ctl_over_frequency_deadband_callback)(value, self.context, ctl_index);
    }

    /// Under-Frequency Deadband
    ///
    /// The deadband value for under-frequency conditions in Hz.
    fn ctl_under_frequency_deadband(&self, ctl_index: u16) -> u32 {
        (self.ctl_under_frequency_deadband_callback)(self.context, ctl_index)
    }

    /// Under-Frequency Deadband
    ///
    /// The deadband value for under-frequency conditions in Hz.
    fn set_ctl_under_frequency_deadband(&mut self, value: u32, ctl_index: u16) {
        (self.set_ctl_under_frequency_deadband_callback)(value, self.context, ctl_index);
    }

    /// Over-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change.
    fn ctl_over_frequency_change_ratio(&self, ctl_index: u16) -> u16 {
        (self.ctl_over_frequency_change_ratio_callback)(self.context, ctl_index)
    }

    /// Over-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change.
    fn set_ctl_over_frequency_change_ratio(&mut self, value: u16, ctl_index: u16) {
        (self.set_ctl_over_frequency_change_ratio_callback)(value, self.context, ctl_index);
    }

    /// Under-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change.
    fn ctl_under_frequency_change_ratio(&self, ctl_index: u16) -> u16 {
        (self.ctl_under_frequency_change_ratio_callback)(self.context, ctl_index)
    }

    /// Under-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change.
    fn set_ctl_under_frequency_change_ratio(&mut self, value: u16, ctl_index: u16) {
        (self.set_ctl_under_frequency_change_ratio_callback)(value, self.context, ctl_index);
    }

    /// Open-Loop Response Time
    ///
    /// The open-loop response time in seconds.
    fn ctl_open_loop_response_time(&self, ctl_index: u16) -> u32 {
        (self.ctl_open_loop_response_time_callback)(self.context, ctl_index)
    }

    /// Open-Loop Response Time
    ///
    /// The open-loop response time in seconds.
    fn set_ctl_open_loop_response_time(&mut self, value: u32, ctl_index: u16) {
        (self.set_ctl_open_loop_response_time_callback)(value, self.context, ctl_index);
    }

    /// Minimum Active Power
    ///
    /// The minimum active power output due to DER prime mover constraints, in percent of the DER active power rating. The valid range is -100 to 100. This setting applies only to the frequency droop control.
    fn ctl_minimum_active_power(&self, ctl_index: u16) -> Option<i16> {
        self.ctl_minimum_active_power_callback
            .map(|callback| (callback)(self.context, ctl_index))
    }

    /// Minimum Active Power
    ///
    /// The minimum active power output due to DER prime mover constraints, in percent of the DER active power rating. The valid range is -100 to 100. This setting applies only to the frequency droop control.
    fn set_ctl_minimum_active_power(&mut self, value: i16, ctl_index: u16) {
        if let Some(callback) = self.set_ctl_minimum_active_power_callback {
            (callback)(value, self.context, ctl_index);
        };
    }

    /// Control Access
    ///
    /// Control read-write access.
    fn ctl_control_access(&self, ctl_index: u16) -> ReadOnly {
        (self.ctl_control_access_callback)(self.context, ctl_index)
    }
}

#[repr(C)]
pub struct Model711StatefulAdapter<const STORED_CONTROL_COUNT: usize> {
    der_frequency_droop_module_enable: Ena,
    active_control_request: u16,
    set_active_control_result: AdptCtlRslt,
    stored_control_count: u16,
    reversion_timeout: u32,
    reversion_time_left: u32,
    reversion_control: u16,
    deadband_scale_factor: u16,
    frequency_change_scale_factor: u16,
    open_loop_scale_factor: u16,
    stored_controls: [Model711StoredControls; STORED_CONTROL_COUNT],
}

#[repr(C)]
pub struct Model711StoredControls {
    ctl_over_frequency_deadband: u32,
    ctl_under_frequency_deadband: u32,
    ctl_over_frequency_change_ratio: u16,
    ctl_under_frequency_change_ratio: u16,
    ctl_open_loop_response_time: u32,
    ctl_minimum_active_power: i16,
    ctl_control_access: ReadOnly,
}

impl<const STORED_CONTROL_COUNT: usize> ModelAdapter
    for Model711StatefulAdapter<STORED_CONTROL_COUNT>
{
    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn der_frequency_droop_module_enable(&self) -> Ena {
        self.der_frequency_droop_module_enable
    }

    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn set_der_frequency_droop_module_enable(&mut self, value: Ena) {
        self.der_frequency_droop_module_enable = value;
    }

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn active_control_request(&self) -> u16 {
        self.active_control_request
    }

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn set_active_control_request(&mut self, value: u16) {
        self.active_control_request = value;
    }

    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    fn set_active_control_result(&self) -> AdptCtlRslt {
        self.set_active_control_result
    }

    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    fn stored_control_count(&self) -> u16 {
        self.stored_control_count
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn reversion_timeout(&self) -> Option<u32> {
        Some(self.reversion_timeout)
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_reversion_timeout(&mut self, value: u32) {
        self.reversion_timeout = value;
    }

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn reversion_time_left(&self) -> Option<u32> {
        Some(self.reversion_time_left)
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn reversion_control(&self) -> Option<u16> {
        Some(self.reversion_control)
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn set_reversion_control(&mut self, value: u16) {
        self.reversion_control = value;
    }

    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    fn deadband_scale_factor(&self) -> u16 {
        self.deadband_scale_factor
    }

    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    fn frequency_change_scale_factor(&self) -> u16 {
        self.frequency_change_scale_factor
    }

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn open_loop_scale_factor(&self) -> u16 {
        self.open_loop_scale_factor
    }

    /// Over-Frequency Deadband
    ///
    /// The deadband value for over-frequency conditions in Hz.
    fn ctl_over_frequency_deadband(&self, ctl_index: u16) -> u32 {
        self.stored_controls[ctl_index as usize].ctl_over_frequency_deadband
    }

    /// Over-Frequency Deadband
    ///
    /// The deadband value for over-frequency conditions in Hz.
    fn set_ctl_over_frequency_deadband(&mut self, value: u32, ctl_index: u16) {
        self.stored_controls[ctl_index as usize].ctl_over_frequency_deadband = value;
    }

    /// Under-Frequency Deadband
    ///
    /// The deadband value for under-frequency conditions in Hz.
    fn ctl_under_frequency_deadband(&self, ctl_index: u16) -> u32 {
        self.stored_controls[ctl_index as usize].ctl_under_frequency_deadband
    }

    /// Under-Frequency Deadband
    ///
    /// The deadband value for under-frequency conditions in Hz.
    fn set_ctl_under_frequency_deadband(&mut self, value: u32, ctl_index: u16) {
        self.stored_controls[ctl_index as usize].ctl_under_frequency_deadband = value;
    }

    /// Over-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change.
    fn ctl_over_frequency_change_ratio(&self, ctl_index: u16) -> u16 {
        self.stored_controls[ctl_index as usize].ctl_over_frequency_change_ratio
    }

    /// Over-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for over-frequency conditions corresponding to 1 per-unit power output change.
    fn set_ctl_over_frequency_change_ratio(&mut self, value: u16, ctl_index: u16) {
        self.stored_controls[ctl_index as usize].ctl_over_frequency_change_ratio = value;
    }

    /// Under-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change.
    fn ctl_under_frequency_change_ratio(&self, ctl_index: u16) -> u16 {
        self.stored_controls[ctl_index as usize].ctl_under_frequency_change_ratio
    }

    /// Under-Frequency Change Ratio
    ///
    /// Frequency droop per-unit frequency change for under-frequency conditions corresponding to 1 per-unit power output change.
    fn set_ctl_under_frequency_change_ratio(&mut self, value: u16, ctl_index: u16) {
        self.stored_controls[ctl_index as usize].ctl_under_frequency_change_ratio = value;
    }

    /// Open-Loop Response Time
    ///
    /// The open-loop response time in seconds.
    fn ctl_open_loop_response_time(&self, ctl_index: u16) -> u32 {
        self.stored_controls[ctl_index as usize].ctl_open_loop_response_time
    }

    /// Open-Loop Response Time
    ///
    /// The open-loop response time in seconds.
    fn set_ctl_open_loop_response_time(&mut self, value: u32, ctl_index: u16) {
        self.stored_controls[ctl_index as usize].ctl_open_loop_response_time = value;
    }

    /// Minimum Active Power
    ///
    /// The minimum active power output due to DER prime mover constraints, in percent of the DER active power rating. The valid range is -100 to 100. This setting applies only to the frequency droop control.
    fn ctl_minimum_active_power(&self, ctl_index: u16) -> Option<i16> {
        Some(self.stored_controls[ctl_index as usize].ctl_minimum_active_power)
    }

    /// Minimum Active Power
    ///
    /// The minimum active power output due to DER prime mover constraints, in percent of the DER active power rating. The valid range is -100 to 100. This setting applies only to the frequency droop control.
    fn set_ctl_minimum_active_power(&mut self, value: i16, ctl_index: u16) {
        self.stored_controls[ctl_index as usize].ctl_minimum_active_power = value;
    }

    /// Control Access
    ///
    /// Control read-write access.
    fn ctl_control_access(&self, ctl_index: u16) -> ReadOnly {
        self.stored_controls[ctl_index as usize].ctl_control_access
    }
}
