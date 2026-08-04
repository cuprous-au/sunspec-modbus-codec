use core::ffi::c_void;
use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 19;

pub static POINTS: [ReadablePoint; 13] = [
    ReadablePoint {
        reference: PointReference::Static { value: 703 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 17 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::PermitEnterService },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::EnterServiceVoltageHigh },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::EnterServiceVoltageLow },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::EnterServiceFrequencyHigh },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::EnterServiceFrequencyLow },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::EnterServiceDelayTime },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::EnterServiceRandomDelay },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::EnterServiceRampTime },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::EnterServiceDelayRemaining },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::VoltageScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model703 { point: Point::FrequencyScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    PermitEnterService,
    EnterServiceVoltageHigh,
    EnterServiceVoltageLow,
    EnterServiceFrequencyHigh,
    EnterServiceFrequencyLow,
    EnterServiceDelayTime,
    EnterServiceRandomDelay,
    EnterServiceRampTime,
    EnterServiceDelayRemaining,
    VoltageScaleFactor,
    FrequencyScaleFactor,
}

pub fn write_point<'a>(model: &dyn ModelAdapter, point: &Point, buffer: ModbusBuffer<'a>, offset: u16, limit: u16) {
    match point {
        Point::PermitEnterService => {
            if let Some(value) = model.permit_enter_service() {
                buffer::write_u16(value as u16, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnterServiceVoltageHigh => {
            if let Some(value) = model.enter_service_voltage_high() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnterServiceVoltageLow => {
            if let Some(value) = model.enter_service_voltage_low() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnterServiceFrequencyHigh => {
            if let Some(value) = model.enter_service_frequency_high() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnterServiceFrequencyLow => {
            if let Some(value) = model.enter_service_frequency_low() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnterServiceDelayTime => {
            if let Some(value) = model.enter_service_delay_time() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnterServiceRandomDelay => {
            if let Some(value) = model.enter_service_random_delay() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnterServiceRampTime => {
            if let Some(value) = model.enter_service_ramp_time() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::EnterServiceDelayRemaining => {
            if let Some(value) = model.enter_service_delay_remaining() {
                buffer::write_u32(value, buffer, offset, limit);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VoltageScaleFactor => {
            if let Some(value) = model.voltage_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
        Point::FrequencyScaleFactor => {
            if let Some(value) = model.frequency_scale_factor() {
                buffer::write_u16(value, buffer);
            }
            else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn permit_enter_service(&self) -> Option<Es> {
        None
    }

    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn set_permit_enter_service(&mut self, value: Es) {
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn enter_service_voltage_high(&self) -> Option<u16> {
        None
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn set_enter_service_voltage_high(&mut self, value: u16) {
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn enter_service_voltage_low(&self) -> Option<u16> {
        None
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn set_enter_service_voltage_low(&mut self, value: u16) {
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn enter_service_frequency_high(&self) -> Option<u32> {
        None
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn set_enter_service_frequency_high(&mut self, value: u32) {
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn enter_service_frequency_low(&self) -> Option<u32> {
        None
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn set_enter_service_frequency_low(&mut self, value: u32) {
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn enter_service_delay_time(&self) -> Option<u32> {
        None
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn set_enter_service_delay_time(&mut self, value: u32) {
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn enter_service_random_delay(&self) -> Option<u32> {
        None
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn set_enter_service_random_delay(&mut self, value: u32) {
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn enter_service_ramp_time(&self) -> Option<u32> {
        None
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn set_enter_service_ramp_time(&mut self, value: u32) {
    }

    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    fn enter_service_delay_remaining(&self) -> Option<u32> {
        None
    }

    /// Voltage Scale Factor
    ///
    /// Voltage percentage scale factor.
    fn voltage_scale_factor(&self) -> Option<u16> {
        None
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn frequency_scale_factor(&self) -> Option<u16> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Es {
    Disabled = 0,
    Enabled = 1,
}

#[repr(C)]
pub struct Model703CallbackAdapter {
    context: *mut c_void,
    permit_enter_service_callback: Option<extern "C" fn(*const c_void) -> Es>,
    set_permit_enter_service_callback: Option<extern "C" fn(Es, *mut c_void)>,
    enter_service_voltage_high_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_enter_service_voltage_high_callback: Option<extern "C" fn(u16, *mut c_void)>,
    enter_service_voltage_low_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_enter_service_voltage_low_callback: Option<extern "C" fn(u16, *mut c_void)>,
    enter_service_frequency_high_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_enter_service_frequency_high_callback: Option<extern "C" fn(u32, *mut c_void)>,
    enter_service_frequency_low_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_enter_service_frequency_low_callback: Option<extern "C" fn(u32, *mut c_void)>,
    enter_service_delay_time_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_enter_service_delay_time_callback: Option<extern "C" fn(u32, *mut c_void)>,
    enter_service_random_delay_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_enter_service_random_delay_callback: Option<extern "C" fn(u32, *mut c_void)>,
    enter_service_ramp_time_callback: Option<extern "C" fn(*const c_void) -> u32>,
    set_enter_service_ramp_time_callback: Option<extern "C" fn(u32, *mut c_void)>,
    enter_service_delay_remaining_callback: Option<extern "C" fn(*const c_void) -> u32>,
    voltage_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
    frequency_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model703CallbackAdapter {
    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn permit_enter_service(&self) -> Option<Es> {
        self.permit_enter_service_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn set_permit_enter_service(&mut self, value: Es) {
        if let Some(callback) = self.set_permit_enter_service_callback {
        (callback)(value, self.context);
        };
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn enter_service_voltage_high(&self) -> Option<u16> {
        self.enter_service_voltage_high_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn set_enter_service_voltage_high(&mut self, value: u16) {
        if let Some(callback) = self.set_enter_service_voltage_high_callback {
        (callback)(value, self.context);
        };
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn enter_service_voltage_low(&self) -> Option<u16> {
        self.enter_service_voltage_low_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn set_enter_service_voltage_low(&mut self, value: u16) {
        if let Some(callback) = self.set_enter_service_voltage_low_callback {
        (callback)(value, self.context);
        };
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn enter_service_frequency_high(&self) -> Option<u32> {
        self.enter_service_frequency_high_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn set_enter_service_frequency_high(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_frequency_high_callback {
        (callback)(value, self.context);
        };
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn enter_service_frequency_low(&self) -> Option<u32> {
        self.enter_service_frequency_low_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn set_enter_service_frequency_low(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_frequency_low_callback {
        (callback)(value, self.context);
        };
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn enter_service_delay_time(&self) -> Option<u32> {
        self.enter_service_delay_time_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn set_enter_service_delay_time(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_delay_time_callback {
        (callback)(value, self.context);
        };
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn enter_service_random_delay(&self) -> Option<u32> {
        self.enter_service_random_delay_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn set_enter_service_random_delay(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_random_delay_callback {
        (callback)(value, self.context);
        };
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn enter_service_ramp_time(&self) -> Option<u32> {
        self.enter_service_ramp_time_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn set_enter_service_ramp_time(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_ramp_time_callback {
        (callback)(value, self.context);
        };
    }

    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    fn enter_service_delay_remaining(&self) -> Option<u32> {
        self.enter_service_delay_remaining_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Voltage Scale Factor
    ///
    /// Voltage percentage scale factor.
    fn voltage_scale_factor(&self) -> Option<u16> {
        self.voltage_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn frequency_scale_factor(&self) -> Option<u16> {
        self.frequency_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model703StatefulAdapter {
    permit_enter_service: Es,
    enter_service_voltage_high: u16,
    enter_service_voltage_low: u16,
    enter_service_frequency_high: u32,
    enter_service_frequency_low: u32,
    enter_service_delay_time: u32,
    enter_service_random_delay: u32,
    enter_service_ramp_time: u32,
    enter_service_delay_remaining: u32,
    voltage_scale_factor: u16,
    frequency_scale_factor: u16,
}

impl ModelAdapter for Model703StatefulAdapter {
    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn permit_enter_service(&self) -> Option<Es> {
        Some(
        self.permit_enter_service
        )
    }

    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn set_permit_enter_service(&mut self, value: Es) {
        self.permit_enter_service = value;
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn enter_service_voltage_high(&self) -> Option<u16> {
        Some(
        self.enter_service_voltage_high
        )
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn set_enter_service_voltage_high(&mut self, value: u16) {
        self.enter_service_voltage_high = value;
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn enter_service_voltage_low(&self) -> Option<u16> {
        Some(
        self.enter_service_voltage_low
        )
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn set_enter_service_voltage_low(&mut self, value: u16) {
        self.enter_service_voltage_low = value;
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn enter_service_frequency_high(&self) -> Option<u32> {
        Some(
        self.enter_service_frequency_high
        )
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn set_enter_service_frequency_high(&mut self, value: u32) {
        self.enter_service_frequency_high = value;
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn enter_service_frequency_low(&self) -> Option<u32> {
        Some(
        self.enter_service_frequency_low
        )
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn set_enter_service_frequency_low(&mut self, value: u32) {
        self.enter_service_frequency_low = value;
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn enter_service_delay_time(&self) -> Option<u32> {
        Some(
        self.enter_service_delay_time
        )
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn set_enter_service_delay_time(&mut self, value: u32) {
        self.enter_service_delay_time = value;
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn enter_service_random_delay(&self) -> Option<u32> {
        Some(
        self.enter_service_random_delay
        )
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn set_enter_service_random_delay(&mut self, value: u32) {
        self.enter_service_random_delay = value;
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn enter_service_ramp_time(&self) -> Option<u32> {
        Some(
        self.enter_service_ramp_time
        )
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn set_enter_service_ramp_time(&mut self, value: u32) {
        self.enter_service_ramp_time = value;
    }

    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    fn enter_service_delay_remaining(&self) -> Option<u32> {
        Some(
        self.enter_service_delay_remaining
        )
    }

    /// Voltage Scale Factor
    ///
    /// Voltage percentage scale factor.
    fn voltage_scale_factor(&self) -> Option<u16> {
        Some(
        self.voltage_scale_factor
        )
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn frequency_scale_factor(&self) -> Option<u16> {
        Some(
        self.frequency_scale_factor
        )
    }
}