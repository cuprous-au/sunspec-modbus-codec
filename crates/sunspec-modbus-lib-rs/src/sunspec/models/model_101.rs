use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 52;

pub static POINTS: [ReadablePoint; 45] = [
    ReadablePoint {
        reference: PointReference::Static { value: 101 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::Amps },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::AmpsPhaseA,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::AmpsPhaseB,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::AmpsPhaseC,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::PhaseVoltageAb,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::PhaseVoltageBc,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::PhaseVoltageCa,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::PhaseVoltageAn,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::PhaseVoltageBn,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::PhaseVoltageCn,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::Watts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::WSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::Hz },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::HzSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::Va },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::VaSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::VAr },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::VArSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::Pf },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::PfSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::WattHours,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 { point: Point::WhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::DcAmps,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::DcaSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::DcVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::DcvSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::DcWatts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::DcwSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::CabinetTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::HeatSinkTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::TransformerTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::OtherTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::TmpSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::OperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::VendorOperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::Event1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::EventBitfield2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::VendorEventBitfield1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::VendorEventBitfield2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
            point: Point::VendorEventBitfield3,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model101 {
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
    ASf,
    PhaseVoltageAb,
    PhaseVoltageBc,
    PhaseVoltageCa,
    PhaseVoltageAn,
    PhaseVoltageBn,
    PhaseVoltageCn,
    VSf,
    Watts,
    WSf,
    Hz,
    HzSf,
    Va,
    VaSf,
    VAr,
    VArSf,
    Pf,
    PfSf,
    WattHours,
    WhSf,
    DcAmps,
    DcaSf,
    DcVoltage,
    DcvSf,
    DcWatts,
    DcwSf,
    CabinetTemperature,
    HeatSinkTemperature,
    TransformerTemperature,
    OtherTemperature,
    TmpSf,
    OperatingState,
    VendorOperatingState,
    Event1,
    EventBitfield2,
    VendorEventBitfield1,
    VendorEventBitfield2,
    VendorEventBitfield3,
    VendorEventBitfield4,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    52
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
        Point::Amps => {
            buffer::write_u16(model.amps(), buffer);
        }
        Point::AmpsPhaseA => {
            buffer::write_u16(model.amps_phase_a(), buffer);
        }
        Point::AmpsPhaseB => {
            if let Some(value) = model.amps_phase_b() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AmpsPhaseC => {
            if let Some(value) = model.amps_phase_c() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ASf => {
            buffer::write_u16(model.a_sf(), buffer);
        }
        Point::PhaseVoltageAb => {
            if let Some(value) = model.phase_voltage_ab() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageBc => {
            if let Some(value) = model.phase_voltage_bc() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageCa => {
            if let Some(value) = model.phase_voltage_ca() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageAn => {
            buffer::write_u16(model.phase_voltage_an(), buffer);
        }
        Point::PhaseVoltageBn => {
            if let Some(value) = model.phase_voltage_bn() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PhaseVoltageCn => {
            if let Some(value) = model.phase_voltage_cn() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VSf => {
            buffer::write_u16(model.v_sf(), buffer);
        }
        Point::Watts => {
            buffer::write_i16(model.watts(), buffer);
        }
        Point::WSf => {
            buffer::write_u16(model.w_sf(), buffer);
        }
        Point::Hz => {
            buffer::write_u16(model.hz(), buffer);
        }
        Point::HzSf => {
            buffer::write_u16(model.hz_sf(), buffer);
        }
        Point::Va => {
            if let Some(value) = model.va() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VaSf => {
            if let Some(value) = model.va_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VAr => {
            if let Some(value) = model.v_ar() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VArSf => {
            if let Some(value) = model.v_ar_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Pf => {
            if let Some(value) = model.pf() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::PfSf => {
            if let Some(value) = model.pf_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WattHours => {
            buffer::write_u32(model.watt_hours(), buffer, offset, limit);
        }
        Point::WhSf => {
            buffer::write_u16(model.wh_sf(), buffer);
        }
        Point::DcAmps => {
            if let Some(value) = model.dc_amps() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcaSf => {
            if let Some(value) = model.dca_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcVoltage => {
            if let Some(value) = model.dc_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcvSf => {
            if let Some(value) = model.dcv_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcWatts => {
            if let Some(value) = model.dc_watts() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::DcwSf => {
            if let Some(value) = model.dcw_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CabinetTemperature => {
            buffer::write_i16(model.cabinet_temperature(), buffer);
        }
        Point::HeatSinkTemperature => {
            if let Some(value) = model.heat_sink_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TransformerTemperature => {
            if let Some(value) = model.transformer_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::OtherTemperature => {
            if let Some(value) = model.other_temperature() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TmpSf => {
            buffer::write_u16(model.tmp_sf(), buffer);
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
    fn amps(&self) -> u16;

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn amps_phase_a(&self) -> u16;

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<u16> {
        None
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<u16> {
        None
    }

    fn a_sf(&self) -> u16;

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> u16;

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<u16> {
        None
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<u16> {
        None
    }

    fn v_sf(&self) -> u16;

    /// Watts
    ///
    /// AC Power
    fn watts(&self) -> i16;

    fn w_sf(&self) -> u16;

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> u16;

    fn hz_sf(&self) -> u16;

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        None
    }

    fn va_sf(&self) -> Option<u16> {
        None
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<i16> {
        None
    }

    fn v_ar_sf(&self) -> Option<u16> {
        None
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<i16> {
        None
    }

    fn pf_sf(&self) -> Option<u16> {
        None
    }

    /// WattHours
    ///
    /// AC Energy
    fn watt_hours(&self) -> u32;

    fn wh_sf(&self) -> u16;

    /// DC Amps
    ///
    /// DC Current
    fn dc_amps(&self) -> Option<u16> {
        None
    }

    fn dca_sf(&self) -> Option<u16> {
        None
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dc_voltage(&self) -> Option<u16> {
        None
    }

    fn dcv_sf(&self) -> Option<u16> {
        None
    }

    /// DC Watts
    ///
    /// DC Power
    fn dc_watts(&self) -> Option<i16> {
        None
    }

    fn dcw_sf(&self) -> Option<u16> {
        None
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn cabinet_temperature(&self) -> i16;

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn heat_sink_temperature(&self) -> Option<i16> {
        None
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn transformer_temperature(&self) -> Option<i16> {
        None
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn other_temperature(&self) -> Option<i16> {
        None
    }

    fn tmp_sf(&self) -> u16;

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
    Off = 1,
    Sleeping = 2,
    Starting = 3,
    Mppt = 4,
    Throttled = 5,
    ShuttingDown = 6,
    Fault = 7,
    Standby = 8,
}

#[repr(C)]
pub struct Model101CallbackAdapter {
    context: *mut c_void,
    amps_callback: extern "C" fn(*const c_void) -> u16,
    amps_phase_a_callback: extern "C" fn(*const c_void) -> u16,
    amps_phase_b_callback: Option<extern "C" fn(*const c_void) -> u16>,
    amps_phase_c_callback: Option<extern "C" fn(*const c_void) -> u16>,
    a_sf_callback: extern "C" fn(*const c_void) -> u16,
    phase_voltage_ab_callback: Option<extern "C" fn(*const c_void) -> u16>,
    phase_voltage_bc_callback: Option<extern "C" fn(*const c_void) -> u16>,
    phase_voltage_ca_callback: Option<extern "C" fn(*const c_void) -> u16>,
    phase_voltage_an_callback: extern "C" fn(*const c_void) -> u16,
    phase_voltage_bn_callback: Option<extern "C" fn(*const c_void) -> u16>,
    phase_voltage_cn_callback: Option<extern "C" fn(*const c_void) -> u16>,
    v_sf_callback: extern "C" fn(*const c_void) -> u16,
    watts_callback: extern "C" fn(*const c_void) -> i16,
    w_sf_callback: extern "C" fn(*const c_void) -> u16,
    hz_callback: extern "C" fn(*const c_void) -> u16,
    hz_sf_callback: extern "C" fn(*const c_void) -> u16,
    va_callback: Option<extern "C" fn(*const c_void) -> i16>,
    va_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    v_ar_callback: Option<extern "C" fn(*const c_void) -> i16>,
    v_ar_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    pf_callback: Option<extern "C" fn(*const c_void) -> i16>,
    pf_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    watt_hours_callback: extern "C" fn(*const c_void) -> u32,
    wh_sf_callback: extern "C" fn(*const c_void) -> u16,
    dc_amps_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dca_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dc_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dcv_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dc_watts_callback: Option<extern "C" fn(*const c_void) -> i16>,
    dcw_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    cabinet_temperature_callback: extern "C" fn(*const c_void) -> i16,
    heat_sink_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    transformer_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    other_temperature_callback: Option<extern "C" fn(*const c_void) -> i16>,
    tmp_sf_callback: extern "C" fn(*const c_void) -> u16,
    operating_state_callback: extern "C" fn(*const c_void) -> St,
    vendor_operating_state_callback: Option<extern "C" fn(*const c_void) -> u16>,
    event1_callback: extern "C" fn(*const c_void) -> u32,
    event_bitfield_2_callback: extern "C" fn(*const c_void) -> u32,
    vendor_event_bitfield_1_callback: Option<extern "C" fn(*const c_void) -> u32>,
    vendor_event_bitfield_2_callback: Option<extern "C" fn(*const c_void) -> u32>,
    vendor_event_bitfield_3_callback: Option<extern "C" fn(*const c_void) -> u32>,
    vendor_event_bitfield_4_callback: Option<extern "C" fn(*const c_void) -> u32>,
}

impl ModelAdapter for Model101CallbackAdapter {
    /// Amps
    ///
    /// AC Current
    fn amps(&self) -> u16 {
        (self.amps_callback)(self.context)
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn amps_phase_a(&self) -> u16 {
        (self.amps_phase_a_callback)(self.context)
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<u16> {
        self.amps_phase_b_callback
            .map(|callback| (callback)(self.context))
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<u16> {
        self.amps_phase_c_callback
            .map(|callback| (callback)(self.context))
    }

    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)(self.context)
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<u16> {
        self.phase_voltage_ab_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<u16> {
        self.phase_voltage_bc_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<u16> {
        self.phase_voltage_ca_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> u16 {
        (self.phase_voltage_an_callback)(self.context)
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<u16> {
        self.phase_voltage_bn_callback
            .map(|callback| (callback)(self.context))
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<u16> {
        self.phase_voltage_cn_callback
            .map(|callback| (callback)(self.context))
    }

    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)(self.context)
    }

    /// Watts
    ///
    /// AC Power
    fn watts(&self) -> i16 {
        (self.watts_callback)(self.context)
    }

    fn w_sf(&self) -> u16 {
        (self.w_sf_callback)(self.context)
    }

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> u16 {
        (self.hz_callback)(self.context)
    }

    fn hz_sf(&self) -> u16 {
        (self.hz_sf_callback)(self.context)
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        self.va_callback.map(|callback| (callback)(self.context))
    }

    fn va_sf(&self) -> Option<u16> {
        self.va_sf_callback.map(|callback| (callback)(self.context))
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<i16> {
        self.v_ar_callback.map(|callback| (callback)(self.context))
    }

    fn v_ar_sf(&self) -> Option<u16> {
        self.v_ar_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<i16> {
        self.pf_callback.map(|callback| (callback)(self.context))
    }

    fn pf_sf(&self) -> Option<u16> {
        self.pf_sf_callback.map(|callback| (callback)(self.context))
    }

    /// WattHours
    ///
    /// AC Energy
    fn watt_hours(&self) -> u32 {
        (self.watt_hours_callback)(self.context)
    }

    fn wh_sf(&self) -> u16 {
        (self.wh_sf_callback)(self.context)
    }

    /// DC Amps
    ///
    /// DC Current
    fn dc_amps(&self) -> Option<u16> {
        self.dc_amps_callback
            .map(|callback| (callback)(self.context))
    }

    fn dca_sf(&self) -> Option<u16> {
        self.dca_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dc_voltage(&self) -> Option<u16> {
        self.dc_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    fn dcv_sf(&self) -> Option<u16> {
        self.dcv_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// DC Watts
    ///
    /// DC Power
    fn dc_watts(&self) -> Option<i16> {
        self.dc_watts_callback
            .map(|callback| (callback)(self.context))
    }

    fn dcw_sf(&self) -> Option<u16> {
        self.dcw_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn cabinet_temperature(&self) -> i16 {
        (self.cabinet_temperature_callback)(self.context)
    }

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn heat_sink_temperature(&self) -> Option<i16> {
        self.heat_sink_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn transformer_temperature(&self) -> Option<i16> {
        self.transformer_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn other_temperature(&self) -> Option<i16> {
        self.other_temperature_callback
            .map(|callback| (callback)(self.context))
    }

    fn tmp_sf(&self) -> u16 {
        (self.tmp_sf_callback)(self.context)
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
pub struct Model101StatefulAdapter {
    amps: u16,
    amps_phase_a: u16,
    amps_phase_b: u16,
    amps_phase_c: u16,
    a_sf: u16,
    phase_voltage_ab: u16,
    phase_voltage_bc: u16,
    phase_voltage_ca: u16,
    phase_voltage_an: u16,
    phase_voltage_bn: u16,
    phase_voltage_cn: u16,
    v_sf: u16,
    watts: i16,
    w_sf: u16,
    hz: u16,
    hz_sf: u16,
    va: i16,
    va_sf: u16,
    v_ar: i16,
    v_ar_sf: u16,
    pf: i16,
    pf_sf: u16,
    watt_hours: u32,
    wh_sf: u16,
    dc_amps: u16,
    dca_sf: u16,
    dc_voltage: u16,
    dcv_sf: u16,
    dc_watts: i16,
    dcw_sf: u16,
    cabinet_temperature: i16,
    heat_sink_temperature: i16,
    transformer_temperature: i16,
    other_temperature: i16,
    tmp_sf: u16,
    operating_state: St,
    vendor_operating_state: u16,
    event1: u32,
    event_bitfield_2: u32,
    vendor_event_bitfield_1: u32,
    vendor_event_bitfield_2: u32,
    vendor_event_bitfield_3: u32,
    vendor_event_bitfield_4: u32,
}

impl ModelAdapter for Model101StatefulAdapter {
    /// Amps
    ///
    /// AC Current
    fn amps(&self) -> u16 {
        self.amps
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn amps_phase_a(&self) -> u16 {
        self.amps_phase_a
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    fn amps_phase_b(&self) -> Option<u16> {
        Some(self.amps_phase_b)
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<u16> {
        Some(self.amps_phase_c)
    }

    fn a_sf(&self) -> u16 {
        self.a_sf
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<u16> {
        Some(self.phase_voltage_ab)
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<u16> {
        Some(self.phase_voltage_bc)
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<u16> {
        Some(self.phase_voltage_ca)
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> u16 {
        self.phase_voltage_an
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> Option<u16> {
        Some(self.phase_voltage_bn)
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<u16> {
        Some(self.phase_voltage_cn)
    }

    fn v_sf(&self) -> u16 {
        self.v_sf
    }

    /// Watts
    ///
    /// AC Power
    fn watts(&self) -> i16 {
        self.watts
    }

    fn w_sf(&self) -> u16 {
        self.w_sf
    }

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> u16 {
        self.hz
    }

    fn hz_sf(&self) -> u16 {
        self.hz_sf
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        Some(self.va)
    }

    fn va_sf(&self) -> Option<u16> {
        Some(self.va_sf)
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<i16> {
        Some(self.v_ar)
    }

    fn v_ar_sf(&self) -> Option<u16> {
        Some(self.v_ar_sf)
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<i16> {
        Some(self.pf)
    }

    fn pf_sf(&self) -> Option<u16> {
        Some(self.pf_sf)
    }

    /// WattHours
    ///
    /// AC Energy
    fn watt_hours(&self) -> u32 {
        self.watt_hours
    }

    fn wh_sf(&self) -> u16 {
        self.wh_sf
    }

    /// DC Amps
    ///
    /// DC Current
    fn dc_amps(&self) -> Option<u16> {
        Some(self.dc_amps)
    }

    fn dca_sf(&self) -> Option<u16> {
        Some(self.dca_sf)
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dc_voltage(&self) -> Option<u16> {
        Some(self.dc_voltage)
    }

    fn dcv_sf(&self) -> Option<u16> {
        Some(self.dcv_sf)
    }

    /// DC Watts
    ///
    /// DC Power
    fn dc_watts(&self) -> Option<i16> {
        Some(self.dc_watts)
    }

    fn dcw_sf(&self) -> Option<u16> {
        Some(self.dcw_sf)
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn cabinet_temperature(&self) -> i16 {
        self.cabinet_temperature
    }

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn heat_sink_temperature(&self) -> Option<i16> {
        Some(self.heat_sink_temperature)
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn transformer_temperature(&self) -> Option<i16> {
        Some(self.transformer_temperature)
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn other_temperature(&self) -> Option<i16> {
        Some(self.other_temperature)
    }

    fn tmp_sf(&self) -> u16 {
        self.tmp_sf
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
