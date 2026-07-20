use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 62;

pub static POINTS: [ReadablePoint; 33] = [
    ReadablePoint {
        reference: PointReference::Static { value: 113 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 60 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::Amps },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::AmpsPhaseA },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::AmpsPhaseB },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::AmpsPhaseC },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::PhaseVoltageAb },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::PhaseVoltageBc },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::PhaseVoltageCa },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::PhaseVoltageAn },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::PhaseVoltageBn },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::PhaseVoltageCn },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::Watts },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::Hz },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::Va },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::VAr },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::Pf },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::WattHours },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::DcAmps },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::DcVoltage },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::DcWatts },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::CabinetTemperature },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::HeatSinkTemperature },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::TransformerTemperature },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::OtherTemperature },
        size: 2,
        data_type: PointType::Float32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::OperatingState },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::VendorOperatingState },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::Event1 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::EventBitfield2 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::VendorEventBitfield1 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::VendorEventBitfield2 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::VendorEventBitfield3 },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model113 { point: Point::VendorEventBitfield4 },
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::Amps => serialisation::write_f32(model.amps(), buffer, offset, limit),
        Point::AmpsPhaseA => serialisation::write_f32(model.amps_phase_a(), buffer, offset, limit),
        Point::AmpsPhaseB => serialisation::write_f32(model.amps_phase_b(), buffer, offset, limit),
        Point::AmpsPhaseC => serialisation::write_f32(model.amps_phase_c(), buffer, offset, limit),
        Point::PhaseVoltageAb => if let Some(value) = model.phase_voltage_ab() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::PhaseVoltageBc => if let Some(value) = model.phase_voltage_bc() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::PhaseVoltageCa => if let Some(value) = model.phase_voltage_ca() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::PhaseVoltageAn => serialisation::write_f32(model.phase_voltage_an(), buffer, offset, limit),
        Point::PhaseVoltageBn => serialisation::write_f32(model.phase_voltage_bn(), buffer, offset, limit),
        Point::PhaseVoltageCn => serialisation::write_f32(model.phase_voltage_cn(), buffer, offset, limit),
        Point::Watts => serialisation::write_f32(model.watts(), buffer, offset, limit),
        Point::Hz => serialisation::write_f32(model.hz(), buffer, offset, limit),
        Point::Va => if let Some(value) = model.va() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::VAr => if let Some(value) = model.v_ar() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::Pf => if let Some(value) = model.pf() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::WattHours => serialisation::write_f32(model.watt_hours(), buffer, offset, limit),
        Point::DcAmps => if let Some(value) = model.dc_amps() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::DcVoltage => if let Some(value) = model.dc_voltage() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::DcWatts => if let Some(value) = model.dc_watts() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::CabinetTemperature => serialisation::write_f32(model.cabinet_temperature(), buffer, offset, limit),
        Point::HeatSinkTemperature => if let Some(value) = model.heat_sink_temperature() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::TransformerTemperature => if let Some(value) = model.transformer_temperature() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::OtherTemperature => if let Some(value) = model.other_temperature() { serialisation::write_f32(value, buffer, offset, limit); },
        Point::OperatingState => serialisation::write_u16(model.operating_state() as u16, buffer),
        Point::VendorOperatingState => if let Some(value) = model.vendor_operating_state() { serialisation::write_u16(value as u16, buffer); },
        Point::Event1 => serialisation::write_u32(model.event1(), buffer, offset, limit),
        Point::EventBitfield2 => serialisation::write_u32(model.event_bitfield_2(), buffer, offset, limit),
        Point::VendorEventBitfield1 => if let Some(value) = model.vendor_event_bitfield_1() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::VendorEventBitfield2 => if let Some(value) = model.vendor_event_bitfield_2() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::VendorEventBitfield3 => if let Some(value) = model.vendor_event_bitfield_3() { serialisation::write_u32(value, buffer, offset, limit); },
        Point::VendorEventBitfield4 => if let Some(value) = model.vendor_event_bitfield_4() { serialisation::write_u32(value, buffer, offset, limit); },
    }
}

pub trait ModelAdapter {
    /// Amps
    ///
    /// AC Current
    ///
    /// Sum of active phases
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
    ///
    /// Connected Phase
    fn amps_phase_b(&self) -> f32;

    /// Amps PhaseC
    ///
    /// Phase C Current
    ///
    /// Connected Phase
    fn amps_phase_c(&self) -> f32;

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
    fn phase_voltage_bn(&self) -> f32;

    /// Phase Voltage CN
    ///
    /// Phase Voltage CN
    fn phase_voltage_cn(&self) -> f32;

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

pub enum StVnd {
}