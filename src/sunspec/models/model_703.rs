use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

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
        reference: PointReference::Model703 {
            point: Point::PermitEnterService,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::EnterServiceVoltageHigh,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::EnterServiceVoltageLow,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::EnterServiceFrequencyHigh,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::EnterServiceFrequencyLow,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::EnterServiceDelayTime,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::EnterServiceRandomDelay,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::EnterServiceRampTime,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::EnterServiceDelayRemaining,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::VoltageScaleFactor,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model703 {
            point: Point::FrequencyScaleFactor,
        },
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::PermitEnterService => {
            if let Some(value) = model.permit_enter_service() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::EnterServiceVoltageHigh => {
            if let Some(value) = model.enter_service_voltage_high() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::EnterServiceVoltageLow => {
            if let Some(value) = model.enter_service_voltage_low() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::EnterServiceFrequencyHigh => {
            if let Some(value) = model.enter_service_frequency_high() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::EnterServiceFrequencyLow => {
            if let Some(value) = model.enter_service_frequency_low() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::EnterServiceDelayTime => {
            if let Some(value) = model.enter_service_delay_time() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::EnterServiceRandomDelay => {
            if let Some(value) = model.enter_service_random_delay() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::EnterServiceRampTime => {
            if let Some(value) = model.enter_service_ramp_time() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::EnterServiceDelayRemaining => {
            if let Some(value) = model.enter_service_delay_remaining() {
                serialisation::write_u32(value, buffer, offset, limit);
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
    fn set_permit_enter_service(&mut self, value: Es) {}

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn enter_service_voltage_high(&self) -> Option<u16> {
        None
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn set_enter_service_voltage_high(&mut self, value: u16) {}

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn enter_service_voltage_low(&self) -> Option<u16> {
        None
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn set_enter_service_voltage_low(&mut self, value: u16) {}

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn enter_service_frequency_high(&self) -> Option<u32> {
        None
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn set_enter_service_frequency_high(&mut self, value: u32) {}

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn enter_service_frequency_low(&self) -> Option<u32> {
        None
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn set_enter_service_frequency_low(&mut self, value: u32) {}

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn enter_service_delay_time(&self) -> Option<u32> {
        None
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn set_enter_service_delay_time(&mut self, value: u32) {}

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn enter_service_random_delay(&self) -> Option<u32> {
        None
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn set_enter_service_random_delay(&mut self, value: u32) {}

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn enter_service_ramp_time(&self) -> Option<u32> {
        None
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn set_enter_service_ramp_time(&mut self, value: u32) {}

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

#[repr(u16)]
pub enum Es {
    Disabled = 0,
    Enabled = 1,
}

#[repr(C)]
pub struct Model703CallbackAdapter {
    permit_enter_service_callback: Option<extern "C" fn() -> Es>,
    set_permit_enter_service_callback: Option<extern "C" fn(Es)>,
    enter_service_voltage_high_callback: Option<extern "C" fn() -> u16>,
    set_enter_service_voltage_high_callback: Option<extern "C" fn(u16)>,
    enter_service_voltage_low_callback: Option<extern "C" fn() -> u16>,
    set_enter_service_voltage_low_callback: Option<extern "C" fn(u16)>,
    enter_service_frequency_high_callback: Option<extern "C" fn() -> u32>,
    set_enter_service_frequency_high_callback: Option<extern "C" fn(u32)>,
    enter_service_frequency_low_callback: Option<extern "C" fn() -> u32>,
    set_enter_service_frequency_low_callback: Option<extern "C" fn(u32)>,
    enter_service_delay_time_callback: Option<extern "C" fn() -> u32>,
    set_enter_service_delay_time_callback: Option<extern "C" fn(u32)>,
    enter_service_random_delay_callback: Option<extern "C" fn() -> u32>,
    set_enter_service_random_delay_callback: Option<extern "C" fn(u32)>,
    enter_service_ramp_time_callback: Option<extern "C" fn() -> u32>,
    set_enter_service_ramp_time_callback: Option<extern "C" fn(u32)>,
    enter_service_delay_remaining_callback: Option<extern "C" fn() -> u32>,
    voltage_scale_factor_callback: Option<extern "C" fn() -> u16>,
    frequency_scale_factor_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model703CallbackAdapter {
    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn permit_enter_service(&self) -> Option<Es> {
        self.permit_enter_service_callback
            .map(|callback| (callback)())
    }

    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn set_permit_enter_service(&mut self, value: Es) {
        if let Some(callback) = self.set_permit_enter_service_callback {
            (callback)(value);
        };
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn enter_service_voltage_high(&self) -> Option<u16> {
        self.enter_service_voltage_high_callback
            .map(|callback| (callback)())
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn set_enter_service_voltage_high(&mut self, value: u16) {
        if let Some(callback) = self.set_enter_service_voltage_high_callback {
            (callback)(value);
        };
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn enter_service_voltage_low(&self) -> Option<u16> {
        self.enter_service_voltage_low_callback
            .map(|callback| (callback)())
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn set_enter_service_voltage_low(&mut self, value: u16) {
        if let Some(callback) = self.set_enter_service_voltage_low_callback {
            (callback)(value);
        };
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn enter_service_frequency_high(&self) -> Option<u32> {
        self.enter_service_frequency_high_callback
            .map(|callback| (callback)())
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn set_enter_service_frequency_high(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_frequency_high_callback {
            (callback)(value);
        };
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn enter_service_frequency_low(&self) -> Option<u32> {
        self.enter_service_frequency_low_callback
            .map(|callback| (callback)())
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn set_enter_service_frequency_low(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_frequency_low_callback {
            (callback)(value);
        };
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn enter_service_delay_time(&self) -> Option<u32> {
        self.enter_service_delay_time_callback
            .map(|callback| (callback)())
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn set_enter_service_delay_time(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_delay_time_callback {
            (callback)(value);
        };
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn enter_service_random_delay(&self) -> Option<u32> {
        self.enter_service_random_delay_callback
            .map(|callback| (callback)())
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn set_enter_service_random_delay(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_random_delay_callback {
            (callback)(value);
        };
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn enter_service_ramp_time(&self) -> Option<u32> {
        self.enter_service_ramp_time_callback
            .map(|callback| (callback)())
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn set_enter_service_ramp_time(&mut self, value: u32) {
        if let Some(callback) = self.set_enter_service_ramp_time_callback {
            (callback)(value);
        };
    }

    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    fn enter_service_delay_remaining(&self) -> Option<u32> {
        self.enter_service_delay_remaining_callback
            .map(|callback| (callback)())
    }

    /// Voltage Scale Factor
    ///
    /// Voltage percentage scale factor.
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
}
