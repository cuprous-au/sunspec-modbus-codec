use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 52;

pub static POINTS: [ReadablePoint; 45] = [
    ReadablePoint {
        reference: PointReference::Static { value: 102 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 50 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::Amps },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::AmpsPhaseA,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::AmpsPhaseB,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::AmpsPhaseC,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::PhaseVoltageAb,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::PhaseVoltageBc,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::PhaseVoltageCa,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::PhaseVoltageAn,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::PhaseVoltageBn,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::PhaseVoltageCn,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::Watts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::WSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::Hz },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::HzSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::Va },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::VaSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::VAr },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::VArSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::Pf },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::PfSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::WattHours,
        },
        size: 2,
        data_type: PointType::Acc32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 { point: Point::WhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::DcAmps,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::DcaSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::DcVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::DcvSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::DcWatts,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::DcwSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::CabinetTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::HeatSinkTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::TransformerTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::OtherTemperature,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::TmpSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::OperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::VendorOperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::Event1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::EventBitfield2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::VendorEventBitfield1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::VendorEventBitfield2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::VendorEventBitfield3,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model102 {
            point: Point::VendorEventBitfield4,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::Amps => serialisation::write_u16(model.amps(), buffer),
        Point::AmpsPhaseA => serialisation::write_u16(model.amps_phase_a(), buffer),
        Point::AmpsPhaseB => serialisation::write_u16(model.amps_phase_b(), buffer),
        Point::AmpsPhaseC => {
            if let Some(value) = model.amps_phase_c() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ASf => serialisation::write_u16(model.a_sf(), buffer),
        Point::PhaseVoltageAb => {
            if let Some(value) = model.phase_voltage_ab() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PhaseVoltageBc => {
            if let Some(value) = model.phase_voltage_bc() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PhaseVoltageCa => {
            if let Some(value) = model.phase_voltage_ca() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::PhaseVoltageAn => serialisation::write_u16(model.phase_voltage_an(), buffer),
        Point::PhaseVoltageBn => serialisation::write_u16(model.phase_voltage_bn(), buffer),
        Point::PhaseVoltageCn => {
            if let Some(value) = model.phase_voltage_cn() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VSf => serialisation::write_u16(model.v_sf(), buffer),
        Point::Watts => serialisation::write_i16(model.watts(), buffer),
        Point::WSf => serialisation::write_u16(model.w_sf(), buffer),
        Point::Hz => serialisation::write_u16(model.hz(), buffer),
        Point::HzSf => serialisation::write_u16(model.hz_sf(), buffer),
        Point::Va => {
            if let Some(value) = model.va() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VaSf => {
            if let Some(value) = model.va_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VAr => {
            if let Some(value) = model.v_ar() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::VArSf => {
            if let Some(value) = model.v_ar_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Pf => {
            if let Some(value) = model.pf() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::PfSf => {
            if let Some(value) = model.pf_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WattHours => serialisation::write_u32(model.watt_hours(), buffer, offset, limit),
        Point::WhSf => serialisation::write_u16(model.wh_sf(), buffer),
        Point::DcAmps => {
            if let Some(value) = model.dc_amps() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DcaSf => {
            if let Some(value) = model.dca_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DcVoltage => {
            if let Some(value) = model.dc_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DcvSf => {
            if let Some(value) = model.dcv_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DcWatts => {
            if let Some(value) = model.dc_watts() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::DcwSf => {
            if let Some(value) = model.dcw_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::CabinetTemperature => serialisation::write_i16(model.cabinet_temperature(), buffer),
        Point::HeatSinkTemperature => {
            if let Some(value) = model.heat_sink_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::TransformerTemperature => {
            if let Some(value) = model.transformer_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::OtherTemperature => {
            if let Some(value) = model.other_temperature() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::TmpSf => serialisation::write_u16(model.tmp_sf(), buffer),
        Point::OperatingState => serialisation::write_u16(model.operating_state() as u16, buffer),
        Point::VendorOperatingState => {
            if let Some(value) = model.vendor_operating_state() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::Event1 => serialisation::write_u32(model.event1(), buffer, offset, limit),
        Point::EventBitfield2 => {
            serialisation::write_u32(model.event_bitfield_2(), buffer, offset, limit)
        }
        Point::VendorEventBitfield1 => {
            if let Some(value) = model.vendor_event_bitfield_1() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::VendorEventBitfield2 => {
            if let Some(value) = model.vendor_event_bitfield_2() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::VendorEventBitfield3 => {
            if let Some(value) = model.vendor_event_bitfield_3() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::VendorEventBitfield4 => {
            if let Some(value) = model.vendor_event_bitfield_4() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Amps
    ///
    /// AC Current
    ///
    /// Sum of active phases
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
    ///
    /// Connected Phase
    fn amps_phase_b(&self) -> u16;

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
    fn phase_voltage_bn(&self) -> u16;

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
    fn vendor_operating_state(&self) -> Option<StVnd> {
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

pub enum StVnd {}

#[repr(C)]
pub struct Model102CallbackAdapter {
    amps_callback: extern "C" fn() -> u16,
    amps_phase_a_callback: extern "C" fn() -> u16,
    amps_phase_b_callback: extern "C" fn() -> u16,
    amps_phase_c_callback: Option<extern "C" fn() -> u16>,
    a_sf_callback: extern "C" fn() -> u16,
    phase_voltage_ab_callback: Option<extern "C" fn() -> u16>,
    phase_voltage_bc_callback: Option<extern "C" fn() -> u16>,
    phase_voltage_ca_callback: Option<extern "C" fn() -> u16>,
    phase_voltage_an_callback: extern "C" fn() -> u16,
    phase_voltage_bn_callback: extern "C" fn() -> u16,
    phase_voltage_cn_callback: Option<extern "C" fn() -> u16>,
    v_sf_callback: extern "C" fn() -> u16,
    watts_callback: extern "C" fn() -> i16,
    w_sf_callback: extern "C" fn() -> u16,
    hz_callback: extern "C" fn() -> u16,
    hz_sf_callback: extern "C" fn() -> u16,
    va_callback: Option<extern "C" fn() -> i16>,
    va_sf_callback: Option<extern "C" fn() -> u16>,
    v_ar_callback: Option<extern "C" fn() -> i16>,
    v_ar_sf_callback: Option<extern "C" fn() -> u16>,
    pf_callback: Option<extern "C" fn() -> i16>,
    pf_sf_callback: Option<extern "C" fn() -> u16>,
    watt_hours_callback: extern "C" fn() -> u32,
    wh_sf_callback: extern "C" fn() -> u16,
    dc_amps_callback: Option<extern "C" fn() -> u16>,
    dca_sf_callback: Option<extern "C" fn() -> u16>,
    dc_voltage_callback: Option<extern "C" fn() -> u16>,
    dcv_sf_callback: Option<extern "C" fn() -> u16>,
    dc_watts_callback: Option<extern "C" fn() -> i16>,
    dcw_sf_callback: Option<extern "C" fn() -> u16>,
    cabinet_temperature_callback: extern "C" fn() -> i16,
    heat_sink_temperature_callback: Option<extern "C" fn() -> i16>,
    transformer_temperature_callback: Option<extern "C" fn() -> i16>,
    other_temperature_callback: Option<extern "C" fn() -> i16>,
    tmp_sf_callback: extern "C" fn() -> u16,
    operating_state_callback: extern "C" fn() -> St,
    vendor_operating_state_callback: Option<extern "C" fn() -> StVnd>,
    event1_callback: extern "C" fn() -> u32,
    event_bitfield_2_callback: extern "C" fn() -> u32,
    vendor_event_bitfield_1_callback: Option<extern "C" fn() -> u32>,
    vendor_event_bitfield_2_callback: Option<extern "C" fn() -> u32>,
    vendor_event_bitfield_3_callback: Option<extern "C" fn() -> u32>,
    vendor_event_bitfield_4_callback: Option<extern "C" fn() -> u32>,
}

impl ModelAdapter for Model102CallbackAdapter {
    /// Amps
    ///
    /// AC Current
    ///
    /// Sum of active phases
    fn amps(&self) -> u16 {
        (self.amps_callback)()
    }

    /// Amps PhaseA
    ///
    /// Phase A Current
    ///
    /// Connected Phase
    fn amps_phase_a(&self) -> u16 {
        (self.amps_phase_a_callback)()
    }

    /// Amps PhaseB
    ///
    /// Phase B Current
    ///
    /// Connected Phase
    fn amps_phase_b(&self) -> u16 {
        (self.amps_phase_b_callback)()
    }

    /// Amps PhaseC
    ///
    /// Phase C Current
    fn amps_phase_c(&self) -> Option<u16> {
        self.amps_phase_c_callback.map(|callback| (callback)())
    }

    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)()
    }

    /// Phase Voltage AB
    ///
    /// Phase Voltage AB
    fn phase_voltage_ab(&self) -> Option<u16> {
        self.phase_voltage_ab_callback.map(|callback| (callback)())
    }

    /// Phase Voltage BC
    ///
    /// Phase Voltage BC
    fn phase_voltage_bc(&self) -> Option<u16> {
        self.phase_voltage_bc_callback.map(|callback| (callback)())
    }

    /// Phase Voltage CA
    ///
    /// Phase Voltage CA
    fn phase_voltage_ca(&self) -> Option<u16> {
        self.phase_voltage_ca_callback.map(|callback| (callback)())
    }

    /// Phase Voltage AN
    ///
    /// Phase Voltage AN
    fn phase_voltage_an(&self) -> u16 {
        (self.phase_voltage_an_callback)()
    }

    /// Phase Voltage BN
    ///
    /// Phase Voltage BN
    fn phase_voltage_bn(&self) -> u16 {
        (self.phase_voltage_bn_callback)()
    }

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> Option<u16> {
        self.phase_voltage_cn_callback.map(|callback| (callback)())
    }

    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)()
    }

    /// Watts
    ///
    /// AC Power
    fn watts(&self) -> i16 {
        (self.watts_callback)()
    }

    fn w_sf(&self) -> u16 {
        (self.w_sf_callback)()
    }

    /// Hz
    ///
    /// Line Frequency
    fn hz(&self) -> u16 {
        (self.hz_callback)()
    }

    fn hz_sf(&self) -> u16 {
        (self.hz_sf_callback)()
    }

    /// VA
    ///
    /// AC Apparent Power
    fn va(&self) -> Option<i16> {
        self.va_callback.map(|callback| (callback)())
    }

    fn va_sf(&self) -> Option<u16> {
        self.va_sf_callback.map(|callback| (callback)())
    }

    /// VAr
    ///
    /// AC Reactive Power
    fn v_ar(&self) -> Option<i16> {
        self.v_ar_callback.map(|callback| (callback)())
    }

    fn v_ar_sf(&self) -> Option<u16> {
        self.v_ar_sf_callback.map(|callback| (callback)())
    }

    /// PF
    ///
    /// AC Power Factor
    fn pf(&self) -> Option<i16> {
        self.pf_callback.map(|callback| (callback)())
    }

    fn pf_sf(&self) -> Option<u16> {
        self.pf_sf_callback.map(|callback| (callback)())
    }

    /// WattHours
    ///
    /// AC Energy
    fn watt_hours(&self) -> u32 {
        (self.watt_hours_callback)()
    }

    fn wh_sf(&self) -> u16 {
        (self.wh_sf_callback)()
    }

    /// DC Amps
    ///
    /// DC Current
    fn dc_amps(&self) -> Option<u16> {
        self.dc_amps_callback.map(|callback| (callback)())
    }

    fn dca_sf(&self) -> Option<u16> {
        self.dca_sf_callback.map(|callback| (callback)())
    }

    /// DC Voltage
    ///
    /// DC Voltage
    fn dc_voltage(&self) -> Option<u16> {
        self.dc_voltage_callback.map(|callback| (callback)())
    }

    fn dcv_sf(&self) -> Option<u16> {
        self.dcv_sf_callback.map(|callback| (callback)())
    }

    /// DC Watts
    ///
    /// DC Power
    fn dc_watts(&self) -> Option<i16> {
        self.dc_watts_callback.map(|callback| (callback)())
    }

    fn dcw_sf(&self) -> Option<u16> {
        self.dcw_sf_callback.map(|callback| (callback)())
    }

    /// Cabinet Temperature
    ///
    /// Cabinet Temperature
    fn cabinet_temperature(&self) -> i16 {
        (self.cabinet_temperature_callback)()
    }

    /// Heat Sink Temperature
    ///
    /// Heat Sink Temperature
    fn heat_sink_temperature(&self) -> Option<i16> {
        self.heat_sink_temperature_callback
            .map(|callback| (callback)())
    }

    /// Transformer Temperature
    ///
    /// Transformer Temperature
    fn transformer_temperature(&self) -> Option<i16> {
        self.transformer_temperature_callback
            .map(|callback| (callback)())
    }

    /// Other Temperature
    ///
    /// Other Temperature
    fn other_temperature(&self) -> Option<i16> {
        self.other_temperature_callback.map(|callback| (callback)())
    }

    fn tmp_sf(&self) -> u16 {
        (self.tmp_sf_callback)()
    }

    /// Operating State
    ///
    /// Operating state
    fn operating_state(&self) -> St {
        (self.operating_state_callback)()
    }

    /// Vendor Operating State
    ///
    /// Vendor specific operating state code
    fn vendor_operating_state(&self) -> Option<StVnd> {
        self.vendor_operating_state_callback
            .map(|callback| (callback)())
    }

    /// Event1
    ///
    /// Event fields
    fn event1(&self) -> u32 {
        (self.event1_callback)()
    }

    /// Event Bitfield 2
    ///
    /// Reserved for future use
    fn event_bitfield_2(&self) -> u32 {
        (self.event_bitfield_2_callback)()
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_1(&self) -> Option<u32> {
        self.vendor_event_bitfield_1_callback
            .map(|callback| (callback)())
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_2(&self) -> Option<u32> {
        self.vendor_event_bitfield_2_callback
            .map(|callback| (callback)())
    }

    /// Vendor Event Bitfield 3
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_3(&self) -> Option<u32> {
        self.vendor_event_bitfield_3_callback
            .map(|callback| (callback)())
    }

    /// Vendor Event Bitfield 4
    ///
    /// Vendor defined events
    fn vendor_event_bitfield_4(&self) -> Option<u32> {
        self.vendor_event_bitfield_4_callback
            .map(|callback| (callback)())
    }
}
