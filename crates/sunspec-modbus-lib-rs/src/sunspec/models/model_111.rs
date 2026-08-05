use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 62;

pub static POINTS: [ReadablePoint; 33] = [
    ReadablePoint {
        reference: PointReference::Static { value: 111 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 { point: Point::Amps },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::AmpsPhaseA,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::AmpsPhaseB,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::AmpsPhaseC,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::PhaseVoltageAb,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::PhaseVoltageBc,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::PhaseVoltageCa,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::PhaseVoltageAn,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::PhaseVoltageBn,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::PhaseVoltageCn,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::Watts,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 { point: Point::Hz },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 { point: Point::Va },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 { point: Point::VAr },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 { point: Point::Pf },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::WattHours,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::DcAmps,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::DcVoltage,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::DcWatts,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::CabinetTemperature,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::HeatSinkTemperature,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::TransformerTemperature,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::OtherTemperature,
        },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::OperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::VendorOperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::Event1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::EventBitfield2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::VendorEventBitfield1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::VendorEventBitfield2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::VendorEventBitfield3,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model111 {
            point: Point::VendorEventBitfield4,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    Amps,
    AmpsPhaseA,
    AmpsPhaseB,
    AmpsPhaseC,
    PhaseVoltageAb,
    PhaseVoltageBc,
    PhaseVoltageCa,
    PhaseVoltageAn,
    PhaseVoltageBn,
    PhaseVoltageCn,
    Watts,
    Hz,
    Va,
    VAr,
    Pf,
    WattHours,
    DcAmps,
    DcVoltage,
    DcWatts,
    CabinetTemperature,
    HeatSinkTemperature,
    TransformerTemperature,
    OtherTemperature,
    OperatingState,
    VendorOperatingState,
    Event1,
    EventBitfield2,
    VendorEventBitfield1,
    VendorEventBitfield2,
    VendorEventBitfield3,
    VendorEventBitfield4,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    62
}

pub fn write_point<'a, 'b>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::Amps => {
            buffer::write_f32(model.amps(), buffer, offset, limit);
        }
        Point::AmpsPhaseA => {
            buffer::write_f32(model.amps_phase_a(), buffer, offset, limit);
        }
        Point::AmpsPhaseB => {
            if let Some(value) = model.amps_phase_b() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AmpsPhaseC => {
            if let Some(value) = model.amps_phase_c() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageAb => {
            if let Some(value) = model.phase_voltage_ab() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageBc => {
            if let Some(value) = model.phase_voltage_bc() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageCa => {
            if let Some(value) = model.phase_voltage_ca() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageAn => {
            buffer::write_f32(model.phase_voltage_an(), buffer, offset, limit);
        }
        Point::PhaseVoltageBn => {
            if let Some(value) = model.phase_voltage_bn() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageCn => {
            if let Some(value) = model.phase_voltage_cn() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Watts => {
            buffer::write_f32(model.watts(), buffer, offset, limit);
        }
        Point::Hz => {
            buffer::write_f32(model.hz(), buffer, offset, limit);
        }
        Point::Va => {
            if let Some(value) = model.va() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VAr => {
            if let Some(value) = model.v_ar() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Pf => {
            if let Some(value) = model.pf() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WattHours => {
            buffer::write_f32(model.watt_hours(), buffer, offset, limit);
        }
        Point::DcAmps => {
            if let Some(value) = model.dc_amps() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcVoltage => {
            if let Some(value) = model.dc_voltage() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcWatts => {
            if let Some(value) = model.dc_watts() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CabinetTemperature => {
            buffer::write_f32(model.cabinet_temperature(), buffer, offset, limit);
        }
        Point::HeatSinkTemperature => {
            if let Some(value) = model.heat_sink_temperature() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TransformerTemperature => {
            if let Some(value) = model.transformer_temperature() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OtherTemperature => {
            if let Some(value) = model.other_temperature() {
                buffer::write_f32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OperatingState => {
            buffer::write_u16(model.operating_state() as u16, buffer);
        }
        Point::VendorOperatingState => {
            if let Some(value) = model.vendor_operating_state() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Event1 => {
            buffer::write_u32(model.event1(), buffer, offset, limit);
        }
        Point::EventBitfield2 => {
            buffer::write_u32(model.event_bitfield_2(), buffer, offset, limit);
        }
        Point::VendorEventBitfield1 => {
            if let Some(value) = model.vendor_event_bitfield_1() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VendorEventBitfield2 => {
            if let Some(value) = model.vendor_event_bitfield_2() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VendorEventBitfield3 => {
            if let Some(value) = model.vendor_event_bitfield_3() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VendorEventBitfield4 => {
            if let Some(value) = model.vendor_event_bitfield_4() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Amps
    ///
    /// AC Current
    fn amps(&self) -> f32;

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn amps_phase_a(&self) -> f32;

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<f32> {
        None
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> f32;

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<f32> {
        None
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<f32> {
        None
    }

    /// Watts
    ///
    /// AC Power
    fn watts(&self) -> f32;

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> f32;

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        None
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<f32> {
        None
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<f32> {
        None
    }

    /// WattHours
    ///
    /// AC Energy
    fn watt_hours(&self) -> f32;

    /// DC Amps
    ///
    /// DC Current
    fn dc_amps(&self) -> Option<f32> {
        None
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dc_voltage(&self) -> Option<f32> {
        None
    }

    /// DC Watts
    ///
    /// DC Power
    fn dc_watts(&self) -> Option<f32> {
        None
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn cabinet_temperature(&self) -> f32;

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn heat_sink_temperature(&self) -> Option<f32> {
        None
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn transformer_temperature(&self) -> Option<f32> {
        None
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn other_temperature(&self) -> Option<f32> {
        None
    }

    /// Operating State
    ///
    /// Operating state
    fn operating_state(&self) -> St;

    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    fn vendor_operating_state(&self) -> Option<u16> {
        None
    }

    /// Event1
    ///
    /// Event fields
    fn event1(&self) -> u32;

    /// Event Bitfield 2
    ///
    /// Reserved for future use
    fn event_bitfield_2(&self) -> u32;

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_1(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_2(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_3(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_4(&self) -> Option<u32> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum St {
    GgOff = 1,
    GgSleeping = 2,
    GgStarting = 3,
    GgMppt = 4,
    GgThrottled = 5,
    GgShuttingDown = 6,
    GgFault = 7,
    GgStandby = 8,
}

#[repr(C)]
pub struct Model111CallbackAdapter {
    context: *mut c_void,
    amps_callback: extern "C" fn(*const c_void) -> f32,
    amps_phase_a_callback: extern "C" fn(*const c_void) -> f32,
    amps_phase_b_callback: Option<extern "C" fn(*const c_void) -> f32>,
    amps_phase_c_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_ab_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_bc_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_ca_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_an_callback: extern "C" fn(*const c_void) -> f32,
    phase_voltage_bn_callback: Option<extern "C" fn(*const c_void) -> f32>,
    phase_voltage_cn_callback: Option<extern "C" fn(*const c_void) -> f32>,
    watts_callback: extern "C" fn(*const c_void) -> f32,
    hz_callback: extern "C" fn(*const c_void) -> f32,
    va_callback: Option<extern "C" fn(*const c_void) -> f32>,
    v_ar_callback: Option<extern "C" fn(*const c_void) -> f32>,
    pf_callback: Option<extern "C" fn(*const c_void) -> f32>,
    watt_hours_callback: extern "C" fn(*const c_void) -> f32,
    dc_amps_callback: Option<extern "C" fn(*const c_void) -> f32>,
    dc_voltage_callback: Option<extern "C" fn(*const c_void) -> f32>,
    dc_watts_callback: Option<extern "C" fn(*const c_void) -> f32>,
    cabinet_temperature_callback: extern "C" fn(*const c_void) -> f32,
    heat_sink_temperature_callback: Option<extern "C" fn(*const c_void) -> f32>,
    transformer_temperature_callback: Option<extern "C" fn(*const c_void) -> f32>,
    other_temperature_callback: Option<extern "C" fn(*const c_void) -> f32>,
    operating_state_callback: extern "C" fn(*const c_void) -> St,
    vendor_operating_state_callback: Option<extern "C" fn(*const c_void) -> u16>,
    event1_callback: extern "C" fn(*const c_void) -> u32,
    event_bitfield_2_callback: extern "C" fn(*const c_void) -> u32,
    vendor_event_bitfield_1_callback: Option<extern "C" fn(*const c_void) -> u32>,
    vendor_event_bitfield_2_callback: Option<extern "C" fn(*const c_void) -> u32>,
    vendor_event_bitfield_3_callback: Option<extern "C" fn(*const c_void) -> u32>,
    vendor_event_bitfield_4_callback: Option<extern "C" fn(*const c_void) -> u32>,
}

impl ModelAdapter for Model111CallbackAdapter {
    /// Amps
    ///
    /// AC Current
    fn amps(&self) -> f32 {
        (self.amps_callback)(self.context)
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn amps_phase_a(&self) -> f32 {
        (self.amps_phase_a_callback)(self.context)
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<f32> {
        self.amps_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<f32> {
        self.amps_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<f32> {
        self.phase_voltage_ab_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<f32> {
        self.phase_voltage_bc_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<f32> {
        self.phase_voltage_ca_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> f32 {
        (self.phase_voltage_an_callback)(self.context)
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<f32> {
        self.phase_voltage_bn_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<f32> {
        self.phase_voltage_cn_callback
            .map(|callback| (callback)(self.context))
    }

    /// Watts
    ///
    /// AC Power
    fn watts(&self) -> f32 {
        (self.watts_callback)(self.context)
    }

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> f32 {
        (self.hz_callback)(self.context)
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        self.va_callback.map(|callback| (callback)(self.context))
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<f32> {
        self.v_ar_callback.map(|callback| (callback)(self.context))
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<f32> {
        self.pf_callback.map(|callback| (callback)(self.context))
    }

    /// WattHours
    ///
    /// AC Energy
    fn watt_hours(&self) -> f32 {
        (self.watt_hours_callback)(self.context)
    }

    /// DC Amps
    ///
    /// DC Current
    fn dc_amps(&self) -> Option<f32> {
        self.dc_amps_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dc_voltage(&self) -> Option<f32> {
        self.dc_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Watts
    ///
    /// DC Power
    fn dc_watts(&self) -> Option<f32> {
        self.dc_watts_callback
            .map(|callback| (callback)(self.context))
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn cabinet_temperature(&self) -> f32 {
        (self.cabinet_temperature_callback)(self.context)
    }

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn heat_sink_temperature(&self) -> Option<f32> {
        self.heat_sink_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn transformer_temperature(&self) -> Option<f32> {
        self.transformer_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn other_temperature(&self) -> Option<f32> {
        self.other_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Operating State
    ///
    /// Operating state
    fn operating_state(&self) -> St {
        (self.operating_state_callback)(self.context)
    }

    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    fn vendor_operating_state(&self) -> Option<u16> {
        self.vendor_operating_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Event1
    ///
    /// Event fields
    fn event1(&self) -> u32 {
        (self.event1_callback)(self.context)
    }

    /// Event Bitfield 2
    ///
    /// Reserved for future use
    fn event_bitfield_2(&self) -> u32 {
        (self.event_bitfield_2_callback)(self.context)
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_1(&self) -> Option<u32> {
        self.vendor_event_bitfield_1_callback
            .map(|callback| (callback)(self.context))
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_2(&self) -> Option<u32> {
        self.vendor_event_bitfield_2_callback
            .map(|callback| (callback)(self.context))
    }

    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_3(&self) -> Option<u32> {
        self.vendor_event_bitfield_3_callback
            .map(|callback| (callback)(self.context))
    }

    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_4(&self) -> Option<u32> {
        self.vendor_event_bitfield_4_callback
            .map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model111StatefulAdapter {
    amps: f32,
    amps_phase_a: f32,
    amps_phase_b: f32,
    amps_phase_c: f32,
    phase_voltage_ab: f32,
    phase_voltage_bc: f32,
    phase_voltage_ca: f32,
    phase_voltage_an: f32,
    phase_voltage_bn: f32,
    phase_voltage_cn: f32,
    watts: f32,
    hz: f32,
    va: f32,
    v_ar: f32,
    pf: f32,
    watt_hours: f32,
    dc_amps: f32,
    dc_voltage: f32,
    dc_watts: f32,
    cabinet_temperature: f32,
    heat_sink_temperature: f32,
    transformer_temperature: f32,
    other_temperature: f32,
    operating_state: St,
    vendor_operating_state: u16,
    event1: u32,
    event_bitfield_2: u32,
    vendor_event_bitfield_1: u32,
    vendor_event_bitfield_2: u32,
    vendor_event_bitfield_3: u32,
    vendor_event_bitfield_4: u32,
}

impl ModelAdapter for Model111StatefulAdapter {
    /// Amps
    ///
    /// AC Current
    fn amps(&self) -> f32 {
        self.amps
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn amps_phase_a(&self) -> f32 {
        self.amps_phase_a
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<f32> {
        Some(self.amps_phase_b)
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<f32> {
        Some(self.amps_phase_c)
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<f32> {
        Some(self.phase_voltage_ab)
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<f32> {
        Some(self.phase_voltage_bc)
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<f32> {
        Some(self.phase_voltage_ca)
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> f32 {
        self.phase_voltage_an
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<f32> {
        Some(self.phase_voltage_bn)
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<f32> {
        Some(self.phase_voltage_cn)
    }

    /// Watts
    ///
    /// AC Power
    fn watts(&self) -> f32 {
        self.watts
    }

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> f32 {
        self.hz
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<f32> {
        Some(self.va)
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<f32> {
        Some(self.v_ar)
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<f32> {
        Some(self.pf)
    }

    /// WattHours
    ///
    /// AC Energy
    fn watt_hours(&self) -> f32 {
        self.watt_hours
    }

    /// DC Amps
    ///
    /// DC Current
    fn dc_amps(&self) -> Option<f32> {
        Some(self.dc_amps)
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dc_voltage(&self) -> Option<f32> {
        Some(self.dc_voltage)
    }

    /// DC Watts
    ///
    /// DC Power
    fn dc_watts(&self) -> Option<f32> {
        Some(self.dc_watts)
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn cabinet_temperature(&self) -> f32 {
        self.cabinet_temperature
    }

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn heat_sink_temperature(&self) -> Option<f32> {
        Some(self.heat_sink_temperature)
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn transformer_temperature(&self) -> Option<f32> {
        Some(self.transformer_temperature)
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn other_temperature(&self) -> Option<f32> {
        Some(self.other_temperature)
    }

    /// Operating State
    ///
    /// Operating state
    fn operating_state(&self) -> St {
        self.operating_state
    }

    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    fn vendor_operating_state(&self) -> Option<u16> {
        Some(self.vendor_operating_state)
    }

    /// Event1
    ///
    /// Event fields
    fn event1(&self) -> u32 {
        self.event1
    }

    /// Event Bitfield 2
    ///
    /// Reserved for future use
    fn event_bitfield_2(&self) -> u32 {
        self.event_bitfield_2
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_1(&self) -> Option<u32> {
        Some(self.vendor_event_bitfield_1)
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_2(&self) -> Option<u32> {
        Some(self.vendor_event_bitfield_2)
    }

    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_3(&self) -> Option<u32> {
        Some(self.vendor_event_bitfield_3)
    }

    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_4(&self) -> Option<u32> {
        Some(self.vendor_event_bitfield_4)
    }
}
