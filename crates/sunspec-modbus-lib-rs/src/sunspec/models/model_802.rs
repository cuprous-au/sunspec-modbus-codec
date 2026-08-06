use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 64;

pub static POINTS: [ReadablePoint; 58] = [
    ReadablePoint {
        reference: PointReference::Static { value: 802 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::ModelLength,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::NameplateChargeCapacity,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::NameplateEnergyCapacity,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::NameplateMaxChargeRate,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::NameplateMaxDischargeRate,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::SelfDischargeRate,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::NameplateMaxSoC,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::NameplateMinSoC,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MaxReservePercent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MinReservePercent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::StateOfCharge,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::DepthOfDischarge,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::StateOfHealth,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::CycleCount,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::ChargeStatus,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::ControlMode,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::BatteryHeartbeat,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::ControllerHeartbeat,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::AlarmReset,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::BatteryType,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::StateOfTheBatteryBank,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::VendorBatteryBankState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::WarrantyDate,
        },
        size: 2,
        data_type: PointType::Uint32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::BatteryEvent1Bitfield,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::BatteryEvent2Bitfield,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::VendorEventBitfield1,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::VendorEventBitfield2,
        },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::ExternalBatteryVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MaxBatteryVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MinBatteryVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MaxCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MaxCellVoltageString,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MaxCellVoltageModule,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MinCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MinCellVoltageString,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MinCellVoltageModule,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::AverageCellVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::TotalDcCurrent,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MaxChargeCurrent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::MaxDischargeCurrent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::TotalPower,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::InverterStateRequest,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::BatteryPowerRequest,
        },
        size: 1,
        data_type: PointType::Int16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::Operation,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::InverterState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::AhRtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::WhRtgSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::WChaDisChaMaxSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::DisChaRteSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::SoCSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::DoDSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::SoHSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::CellVSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::AMaxSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model802 { point: Point::WSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelLength,
    NameplateChargeCapacity,
    NameplateEnergyCapacity,
    NameplateMaxChargeRate,
    NameplateMaxDischargeRate,
    SelfDischargeRate,
    NameplateMaxSoC,
    NameplateMinSoC,
    MaxReservePercent,
    MinReservePercent,
    StateOfCharge,
    DepthOfDischarge,
    StateOfHealth,
    CycleCount,
    ChargeStatus,
    ControlMode,
    BatteryHeartbeat,
    ControllerHeartbeat,
    AlarmReset,
    BatteryType,
    StateOfTheBatteryBank,
    VendorBatteryBankState,
    WarrantyDate,
    BatteryEvent1Bitfield,
    BatteryEvent2Bitfield,
    VendorEventBitfield1,
    VendorEventBitfield2,
    ExternalBatteryVoltage,
    MaxBatteryVoltage,
    MinBatteryVoltage,
    MaxCellVoltage,
    MaxCellVoltageString,
    MaxCellVoltageModule,
    MinCellVoltage,
    MinCellVoltageString,
    MinCellVoltageModule,
    AverageCellVoltage,
    TotalDcCurrent,
    MaxChargeCurrent,
    MaxDischargeCurrent,
    TotalPower,
    InverterStateRequest,
    BatteryPowerRequest,
    Operation,
    InverterState,
    AhRtgSf,
    WhRtgSf,
    WChaDisChaMaxSf,
    DisChaRteSf,
    SoCSf,
    DoDSf,
    SoHSf,
    VSf,
    CellVSf,
    ASf,
    AMaxSf,
    WSf,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    64
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
        Point::NameplateChargeCapacity => {
            buffer::write_u16(model.nameplate_charge_capacity(), buffer);
        }
        Point::NameplateEnergyCapacity => {
            buffer::write_u16(model.nameplate_energy_capacity(), buffer);
        }
        Point::NameplateMaxChargeRate => {
            buffer::write_u16(model.nameplate_max_charge_rate(), buffer);
        }
        Point::NameplateMaxDischargeRate => {
            buffer::write_u16(model.nameplate_max_discharge_rate(), buffer);
        }
        Point::SelfDischargeRate => {
            if let Some(value) = model.self_discharge_rate() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NameplateMaxSoC => {
            if let Some(value) = model.nameplate_max_so_c() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::NameplateMinSoC => {
            if let Some(value) = model.nameplate_min_so_c() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxReservePercent => {
            if let Some(value) = model.max_reserve_percent() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinReservePercent => {
            if let Some(value) = model.min_reserve_percent() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StateOfCharge => {
            buffer::write_u16(model.state_of_charge(), buffer);
        }
        Point::DepthOfDischarge => {
            if let Some(value) = model.depth_of_discharge() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::StateOfHealth => {
            if let Some(value) = model.state_of_health() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::CycleCount => {
            if let Some(value) = model.cycle_count() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ChargeStatus => {
            if let Some(value) = model.charge_status() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ControlMode => {
            buffer::write_u16(model.control_mode() as u16, buffer);
        }
        Point::BatteryHeartbeat => {
            if let Some(value) = model.battery_heartbeat() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::ControllerHeartbeat => {
            if let Some(value) = model.controller_heartbeat() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AlarmReset => {
            buffer::write_u16(model.alarm_reset(), buffer);
        }
        Point::BatteryType => {
            buffer::write_u16(model.battery_type() as u16, buffer);
        }
        Point::StateOfTheBatteryBank => {
            buffer::write_u16(model.state_of_the_battery_bank() as u16, buffer);
        }
        Point::VendorBatteryBankState => {
            if let Some(value) = model.vendor_battery_bank_state() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::WarrantyDate => {
            if let Some(value) = model.warranty_date() {
                buffer::write_u32(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::BatteryEvent1Bitfield => {
            buffer::write_u32(model.battery_event_1_bitfield(), buffer, offset, limit);
        }
        Point::BatteryEvent2Bitfield => {
            buffer::write_u32(model.battery_event_2_bitfield(), buffer, offset, limit);
        }
        Point::VendorEventBitfield1 => {
            buffer::write_u32(model.vendor_event_bitfield_1(), buffer, offset, limit);
        }
        Point::VendorEventBitfield2 => {
            buffer::write_u32(model.vendor_event_bitfield_2(), buffer, offset, limit);
        }
        Point::ExternalBatteryVoltage => {
            buffer::write_u16(model.external_battery_voltage(), buffer);
        }
        Point::MaxBatteryVoltage => {
            if let Some(value) = model.max_battery_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinBatteryVoltage => {
            if let Some(value) = model.min_battery_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxCellVoltage => {
            if let Some(value) = model.max_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxCellVoltageString => {
            if let Some(value) = model.max_cell_voltage_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxCellVoltageModule => {
            if let Some(value) = model.max_cell_voltage_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellVoltage => {
            if let Some(value) = model.min_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellVoltageString => {
            if let Some(value) = model.min_cell_voltage_string() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MinCellVoltageModule => {
            if let Some(value) = model.min_cell_voltage_module() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::AverageCellVoltage => {
            if let Some(value) = model.average_cell_voltage() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalDcCurrent => {
            buffer::write_i16(model.total_dc_current(), buffer);
        }
        Point::MaxChargeCurrent => {
            if let Some(value) = model.max_charge_current() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::MaxDischargeCurrent => {
            if let Some(value) = model.max_discharge_current() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::TotalPower => {
            buffer::write_i16(model.total_power(), buffer);
        }
        Point::InverterStateRequest => {
            if let Some(value) = model.inverter_state_request() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::BatteryPowerRequest => {
            if let Some(value) = model.battery_power_request() {
                buffer::write_i16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Operation => {
            buffer::write_u16(model.operation() as u16, buffer);
        }
        Point::InverterState => {
            buffer::write_u16(model.inverter_state() as u16, buffer);
        }
        Point::AhRtgSf => {
            buffer::write_u16(model.ah_rtg_sf(), buffer);
        }
        Point::WhRtgSf => {
            buffer::write_u16(model.wh_rtg_sf(), buffer);
        }
        Point::WChaDisChaMaxSf => {
            buffer::write_u16(model.w_cha_dis_cha_max_sf(), buffer);
        }
        Point::DisChaRteSf => {
            if let Some(value) = model.dis_cha_rte_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SoCSf => {
            buffer::write_u16(model.so_c_sf(), buffer);
        }
        Point::DoDSf => {
            if let Some(value) = model.do_d_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SoHSf => {
            if let Some(value) = model.so_h_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::VSf => {
            buffer::write_u16(model.v_sf(), buffer);
        }
        Point::CellVSf => {
            buffer::write_u16(model.cell_v_sf(), buffer);
        }
        Point::ASf => {
            buffer::write_u16(model.a_sf(), buffer);
        }
        Point::AMaxSf => {
            buffer::write_u16(model.a_max_sf(), buffer);
        }
        Point::WSf => {
            if let Some(value) = model.w_sf() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Nameplate Charge Capacity
    ///
    /// Nameplate charge capacity in amp-hours.
    fn nameplate_charge_capacity(&self) -> u16;

    /// Nameplate Energy Capacity
    ///
    /// Nameplate energy capacity in DC watt-hours.
    fn nameplate_energy_capacity(&self) -> u16;

    /// Nameplate Max Charge Rate
    ///
    /// Maximum rate of energy transfer into the storage device in DC watts.
    fn nameplate_max_charge_rate(&self) -> u16;

    /// Nameplate Max Discharge Rate
    ///
    /// Maximum rate of energy transfer out of the storage device in DC watts.
    fn nameplate_max_discharge_rate(&self) -> u16;

    /// Self Discharge Rate
    ///
    /// Self discharge rate. Percentage of capacity (WHRtg) discharged per day.
    fn self_discharge_rate(&self) -> Option<u16> {
        None
    }

    /// Nameplate Max SoC
    ///
    /// Manufacturer maximum state of charge, expressed as a percentage.
    fn nameplate_max_so_c(&self) -> Option<u16> {
        None
    }

    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    fn nameplate_min_so_c(&self) -> Option<u16> {
        None
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn max_reserve_percent(&self) -> Option<u16> {
        None
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn set_max_reserve_percent(&mut self, value: u16) {}

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn min_reserve_percent(&self) -> Option<u16> {
        None
    }

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_min_reserve_percent(&mut self, value: u16) {}

    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn state_of_charge(&self) -> u16;

    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Measurement.
    fn depth_of_discharge(&self) -> Option<u16> {
        None
    }

    /// State of Health
    ///
    /// Percentage of battery life remaining.
    fn state_of_health(&self) -> Option<u16> {
        None
    }

    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    fn cycle_count(&self) -> Option<u32> {
        None
    }

    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    fn charge_status(&self) -> Option<ChaSt> {
        None
    }

    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Maps to DRCC.LocRemCtl in IEC 61850.
    fn control_mode(&self) -> LocRemCtl;

    /// Battery Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn battery_heartbeat(&self) -> Option<u16> {
        None
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn controller_heartbeat(&self) -> Option<u16> {
        None
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn set_controller_heartbeat(&mut self, value: u16) {}

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn alarm_reset(&self) -> u16;

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn set_alarm_reset(&mut self, value: u16);

    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Maps to DBAT.BatTyp in 61850.
    fn battery_type(&self) -> Typ;

    /// State of the Battery Bank
    ///
    /// State of the battery bank. Enumeration.
    ///
    /// Must be reconciled with State in IEC 61850.
    fn state_of_the_battery_bank(&self) -> State;

    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state. Enumeration.
    fn vendor_battery_bank_state(&self) -> Option<u16> {
        None
    }

    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Number of days since 1/1/2000.
    fn warranty_date(&self) -> Option<u32> {
        None
    }

    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.
    fn battery_event_1_bitfield(&self) -> u32;

    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.
    ///
    /// Reserved for future use.
    fn battery_event_2_bitfield(&self) -> u32;

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> u32;

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> u32;

    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Maps to ZBAT.V in IEC 61850.
    fn external_battery_voltage(&self) -> u16;

    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn max_battery_voltage(&self) -> Option<u16> {
        None
    }

    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn min_battery_voltage(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage String
    ///
    /// String containing the cell with maximum voltage.
    fn max_cell_voltage_string(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage String
    ///
    /// String containing the cell with minimum voltage.
    fn min_cell_voltage_string(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average cell voltage for all cells in the bank.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> Option<u16> {
        None
    }

    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Measurement.
    fn total_dc_current(&self) -> i16;

    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn max_charge_current(&self) -> Option<u16> {
        None
    }

    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn max_discharge_current(&self) -> Option<u16> {
        None
    }

    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// DC Measurement.
    fn total_power(&self) -> i16;

    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter. Enumeration.
    ///
    /// Used in special states such as manual battery charging.
    fn inverter_state_request(&self) -> Option<ReqInvState> {
        None
    }

    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Used in special states such as string balancing.
    fn battery_power_request(&self) -> Option<i16> {
        None
    }

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn operation(&self) -> SetOp;

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn set_operation(&mut self, value: SetOp);

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn inverter_state(&self) -> SetInvState;

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_inverter_state(&mut self, value: SetInvState);

    /// Scale factor for charge capacity.
    fn ah_rtg_sf(&self) -> u16;

    /// Scale factor for energy capacity.
    fn wh_rtg_sf(&self) -> u16;

    /// Scale factor for maximum charge and discharge rate.
    fn w_cha_dis_cha_max_sf(&self) -> u16;

    /// Scale factor for self discharge rate.
    fn dis_cha_rte_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for state of charge values.
    fn so_c_sf(&self) -> u16;

    /// Scale factor for depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for state of health.
    fn so_h_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for DC bus voltage.
    fn v_sf(&self) -> u16;

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for DC current.
    fn a_sf(&self) -> u16;

    /// Scale factor for instantaneous DC charge/discharge current.
    fn a_max_sf(&self) -> u16;

    /// Scale factor for AC power request.
    fn w_sf(&self) -> Option<u16> {
        None
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ChaSt {
    Off = 1,
    Empty = 2,
    Discharging = 3,
    Charging = 4,
    Full = 5,
    Holding = 6,
    Testing = 7,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum LocRemCtl {
    /// Value of 0 matches LocRemCtl in IEC 61850.
    Remote = 0,
    /// Value of 1 matches LocRemCtl in IEC 61850.
    Local = 1,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ReqInvState {
    NoRequest = 0,
    /// Battery is notified of inverter state change through SetInvState.
    Start = 1,
    /// Battery is notified of inverter state change through SetInvState.
    Stop = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum SetInvState {
    InverterStopped = 1,
    InverterStandby = 2,
    InverterStarted = 3,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum SetOp {
    Connect = 1,
    Disconnect = 2,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum State {
    Disconnected = 1,
    Initializing = 2,
    Connected = 3,
    Standby = 4,
    SocProtection = 5,
    Suspending = 6,
    Fault = 99,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Typ {
    NotApplicableUnknown = 0,
    LeadAcid = 1,
    NickelMetalHydrate = 2,
    NickelCadmium = 3,
    LithiumIon = 4,
    CarbonZinc = 5,
    ZincChloride = 6,
    Alkaline = 7,
    RechargeableAlkaline = 8,
    SodiumSulfur = 9,
    Flow = 10,
    Other = 99,
}

#[repr(C)]
pub struct Model802CallbackAdapter {
    context: *mut c_void,
    nameplate_charge_capacity_callback: extern "C" fn(*const c_void) -> u16,
    nameplate_energy_capacity_callback: extern "C" fn(*const c_void) -> u16,
    nameplate_max_charge_rate_callback: extern "C" fn(*const c_void) -> u16,
    nameplate_max_discharge_rate_callback: extern "C" fn(*const c_void) -> u16,
    self_discharge_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    nameplate_max_so_c_callback: Option<extern "C" fn(*const c_void) -> u16>,
    nameplate_min_so_c_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_reserve_percent_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_max_reserve_percent_callback: Option<extern "C" fn(u16, *mut c_void)>,
    min_reserve_percent_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_min_reserve_percent_callback: Option<extern "C" fn(u16, *mut c_void)>,
    state_of_charge_callback: extern "C" fn(*const c_void) -> u16,
    depth_of_discharge_callback: Option<extern "C" fn(*const c_void) -> u16>,
    state_of_health_callback: Option<extern "C" fn(*const c_void) -> u16>,
    cycle_count_callback: Option<extern "C" fn(*const c_void) -> u32>,
    charge_status_callback: Option<extern "C" fn(*const c_void) -> ChaSt>,
    control_mode_callback: extern "C" fn(*const c_void) -> LocRemCtl,
    battery_heartbeat_callback: Option<extern "C" fn(*const c_void) -> u16>,
    controller_heartbeat_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_controller_heartbeat_callback: Option<extern "C" fn(u16, *mut c_void)>,
    alarm_reset_callback: extern "C" fn(*const c_void) -> u16,
    set_alarm_reset_callback: extern "C" fn(u16, *mut c_void),
    battery_type_callback: extern "C" fn(*const c_void) -> Typ,
    state_of_the_battery_bank_callback: extern "C" fn(*const c_void) -> State,
    vendor_battery_bank_state_callback: Option<extern "C" fn(*const c_void) -> u16>,
    warranty_date_callback: Option<extern "C" fn(*const c_void) -> u32>,
    battery_event_1_bitfield_callback: extern "C" fn(*const c_void) -> u32,
    battery_event_2_bitfield_callback: extern "C" fn(*const c_void) -> u32,
    vendor_event_bitfield_1_callback: extern "C" fn(*const c_void) -> u32,
    vendor_event_bitfield_2_callback: extern "C" fn(*const c_void) -> u32,
    external_battery_voltage_callback: extern "C" fn(*const c_void) -> u16,
    max_battery_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_battery_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_cell_voltage_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_cell_voltage_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_voltage_string_callback: Option<extern "C" fn(*const c_void) -> u16>,
    min_cell_voltage_module_callback: Option<extern "C" fn(*const c_void) -> u16>,
    average_cell_voltage_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_dc_current_callback: extern "C" fn(*const c_void) -> i16,
    max_charge_current_callback: Option<extern "C" fn(*const c_void) -> u16>,
    max_discharge_current_callback: Option<extern "C" fn(*const c_void) -> u16>,
    total_power_callback: extern "C" fn(*const c_void) -> i16,
    inverter_state_request_callback: Option<extern "C" fn(*const c_void) -> ReqInvState>,
    battery_power_request_callback: Option<extern "C" fn(*const c_void) -> i16>,
    operation_callback: extern "C" fn(*const c_void) -> SetOp,
    set_operation_callback: extern "C" fn(SetOp, *mut c_void),
    inverter_state_callback: extern "C" fn(*const c_void) -> SetInvState,
    set_inverter_state_callback: extern "C" fn(SetInvState, *mut c_void),
    ah_rtg_sf_callback: extern "C" fn(*const c_void) -> u16,
    wh_rtg_sf_callback: extern "C" fn(*const c_void) -> u16,
    w_cha_dis_cha_max_sf_callback: extern "C" fn(*const c_void) -> u16,
    dis_cha_rte_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    so_c_sf_callback: extern "C" fn(*const c_void) -> u16,
    do_d_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    so_h_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
    v_sf_callback: extern "C" fn(*const c_void) -> u16,
    cell_v_sf_callback: extern "C" fn(*const c_void) -> u16,
    a_sf_callback: extern "C" fn(*const c_void) -> u16,
    a_max_sf_callback: extern "C" fn(*const c_void) -> u16,
    w_sf_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model802CallbackAdapter {
    /// Nameplate Charge Capacity
    ///
    /// Nameplate charge capacity in amp-hours.
    fn nameplate_charge_capacity(&self) -> u16 {
        (self.nameplate_charge_capacity_callback)(self.context)
    }

    /// Nameplate Energy Capacity
    ///
    /// Nameplate energy capacity in DC watt-hours.
    fn nameplate_energy_capacity(&self) -> u16 {
        (self.nameplate_energy_capacity_callback)(self.context)
    }

    /// Nameplate Max Charge Rate
    ///
    /// Maximum rate of energy transfer into the storage device in DC watts.
    fn nameplate_max_charge_rate(&self) -> u16 {
        (self.nameplate_max_charge_rate_callback)(self.context)
    }

    /// Nameplate Max Discharge Rate
    ///
    /// Maximum rate of energy transfer out of the storage device in DC watts.
    fn nameplate_max_discharge_rate(&self) -> u16 {
        (self.nameplate_max_discharge_rate_callback)(self.context)
    }

    /// Self Discharge Rate
    ///
    /// Self discharge rate. Percentage of capacity (WHRtg) discharged per day.
    fn self_discharge_rate(&self) -> Option<u16> {
        self.self_discharge_rate_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Max SoC
    ///
    /// Manufacturer maximum state of charge, expressed as a percentage.
    fn nameplate_max_so_c(&self) -> Option<u16> {
        self.nameplate_max_so_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    fn nameplate_min_so_c(&self) -> Option<u16> {
        self.nameplate_min_so_c_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn max_reserve_percent(&self) -> Option<u16> {
        self.max_reserve_percent_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn set_max_reserve_percent(&mut self, value: u16) {
        if let Some(callback) = self.set_max_reserve_percent_callback {
            (callback)(value, self.context);
        };
    }

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn min_reserve_percent(&self) -> Option<u16> {
        self.min_reserve_percent_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_min_reserve_percent(&mut self, value: u16) {
        if let Some(callback) = self.set_min_reserve_percent_callback {
            (callback)(value, self.context);
        };
    }

    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn state_of_charge(&self) -> u16 {
        (self.state_of_charge_callback)(self.context)
    }

    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Measurement.
    fn depth_of_discharge(&self) -> Option<u16> {
        self.depth_of_discharge_callback
            .map(|callback| (callback)(self.context))
    }

    /// State of Health
    ///
    /// Percentage of battery life remaining.
    fn state_of_health(&self) -> Option<u16> {
        self.state_of_health_callback
            .map(|callback| (callback)(self.context))
    }

    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    fn cycle_count(&self) -> Option<u32> {
        self.cycle_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    fn charge_status(&self) -> Option<ChaSt> {
        self.charge_status_callback
            .map(|callback| (callback)(self.context))
    }

    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Maps to DRCC.LocRemCtl in IEC 61850.
    fn control_mode(&self) -> LocRemCtl {
        (self.control_mode_callback)(self.context)
    }

    /// Battery Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn battery_heartbeat(&self) -> Option<u16> {
        self.battery_heartbeat_callback
            .map(|callback| (callback)(self.context))
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn controller_heartbeat(&self) -> Option<u16> {
        self.controller_heartbeat_callback
            .map(|callback| (callback)(self.context))
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn set_controller_heartbeat(&mut self, value: u16) {
        if let Some(callback) = self.set_controller_heartbeat_callback {
            (callback)(value, self.context);
        };
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn alarm_reset(&self) -> u16 {
        (self.alarm_reset_callback)(self.context)
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn set_alarm_reset(&mut self, value: u16) {
        (self.set_alarm_reset_callback)(value, self.context);
    }

    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Maps to DBAT.BatTyp in 61850.
    fn battery_type(&self) -> Typ {
        (self.battery_type_callback)(self.context)
    }

    /// State of the Battery Bank
    ///
    /// State of the battery bank. Enumeration.
    ///
    /// Must be reconciled with State in IEC 61850.
    fn state_of_the_battery_bank(&self) -> State {
        (self.state_of_the_battery_bank_callback)(self.context)
    }

    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state. Enumeration.
    fn vendor_battery_bank_state(&self) -> Option<u16> {
        self.vendor_battery_bank_state_callback
            .map(|callback| (callback)(self.context))
    }

    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Number of days since 1/1/2000.
    fn warranty_date(&self) -> Option<u32> {
        self.warranty_date_callback
            .map(|callback| (callback)(self.context))
    }

    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.
    fn battery_event_1_bitfield(&self) -> u32 {
        (self.battery_event_1_bitfield_callback)(self.context)
    }

    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.
    ///
    /// Reserved for future use.
    fn battery_event_2_bitfield(&self) -> u32 {
        (self.battery_event_2_bitfield_callback)(self.context)
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> u32 {
        (self.vendor_event_bitfield_1_callback)(self.context)
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> u32 {
        (self.vendor_event_bitfield_2_callback)(self.context)
    }

    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Maps to ZBAT.V in IEC 61850.
    fn external_battery_voltage(&self) -> u16 {
        (self.external_battery_voltage_callback)(self.context)
    }

    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn max_battery_voltage(&self) -> Option<u16> {
        self.max_battery_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn min_battery_voltage(&self) -> Option<u16> {
        self.min_battery_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> Option<u16> {
        self.max_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Cell Voltage String
    ///
    /// String containing the cell with maximum voltage.
    fn max_cell_voltage_string(&self) -> Option<u16> {
        self.max_cell_voltage_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        self.max_cell_voltage_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> Option<u16> {
        self.min_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage String
    ///
    /// String containing the cell with minimum voltage.
    fn min_cell_voltage_string(&self) -> Option<u16> {
        self.min_cell_voltage_string_callback
            .map(|callback| (callback)(self.context))
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        self.min_cell_voltage_module_callback
            .map(|callback| (callback)(self.context))
    }

    /// Average Cell Voltage
    ///
    /// Average cell voltage for all cells in the bank.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> Option<u16> {
        self.average_cell_voltage_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Measurement.
    fn total_dc_current(&self) -> i16 {
        (self.total_dc_current_callback)(self.context)
    }

    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn max_charge_current(&self) -> Option<u16> {
        self.max_charge_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn max_discharge_current(&self) -> Option<u16> {
        self.max_discharge_current_callback
            .map(|callback| (callback)(self.context))
    }

    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// DC Measurement.
    fn total_power(&self) -> i16 {
        (self.total_power_callback)(self.context)
    }

    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter. Enumeration.
    ///
    /// Used in special states such as manual battery charging.
    fn inverter_state_request(&self) -> Option<ReqInvState> {
        self.inverter_state_request_callback
            .map(|callback| (callback)(self.context))
    }

    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Used in special states such as string balancing.
    fn battery_power_request(&self) -> Option<i16> {
        self.battery_power_request_callback
            .map(|callback| (callback)(self.context))
    }

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn operation(&self) -> SetOp {
        (self.operation_callback)(self.context)
    }

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn set_operation(&mut self, value: SetOp) {
        (self.set_operation_callback)(value, self.context);
    }

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn inverter_state(&self) -> SetInvState {
        (self.inverter_state_callback)(self.context)
    }

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_inverter_state(&mut self, value: SetInvState) {
        (self.set_inverter_state_callback)(value, self.context);
    }

    /// Scale factor for charge capacity.
    fn ah_rtg_sf(&self) -> u16 {
        (self.ah_rtg_sf_callback)(self.context)
    }

    /// Scale factor for energy capacity.
    fn wh_rtg_sf(&self) -> u16 {
        (self.wh_rtg_sf_callback)(self.context)
    }

    /// Scale factor for maximum charge and discharge rate.
    fn w_cha_dis_cha_max_sf(&self) -> u16 {
        (self.w_cha_dis_cha_max_sf_callback)(self.context)
    }

    /// Scale factor for self discharge rate.
    fn dis_cha_rte_sf(&self) -> Option<u16> {
        self.dis_cha_rte_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for state of charge values.
    fn so_c_sf(&self) -> u16 {
        (self.so_c_sf_callback)(self.context)
    }

    /// Scale factor for depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        self.do_d_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for state of health.
    fn so_h_sf(&self) -> Option<u16> {
        self.so_h_sf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Scale factor for DC bus voltage.
    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)(self.context)
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        (self.cell_v_sf_callback)(self.context)
    }

    /// Scale factor for DC current.
    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)(self.context)
    }

    /// Scale factor for instantaneous DC charge/discharge current.
    fn a_max_sf(&self) -> u16 {
        (self.a_max_sf_callback)(self.context)
    }

    /// Scale factor for AC power request.
    fn w_sf(&self) -> Option<u16> {
        self.w_sf_callback.map(|callback| (callback)(self.context))
    }
}

#[repr(C)]
pub struct Model802StatefulAdapter {
    nameplate_charge_capacity: u16,
    nameplate_energy_capacity: u16,
    nameplate_max_charge_rate: u16,
    nameplate_max_discharge_rate: u16,
    self_discharge_rate: u16,
    nameplate_max_so_c: u16,
    nameplate_min_so_c: u16,
    max_reserve_percent: u16,
    min_reserve_percent: u16,
    state_of_charge: u16,
    depth_of_discharge: u16,
    state_of_health: u16,
    cycle_count: u32,
    charge_status: ChaSt,
    control_mode: LocRemCtl,
    battery_heartbeat: u16,
    controller_heartbeat: u16,
    alarm_reset: u16,
    battery_type: Typ,
    state_of_the_battery_bank: State,
    vendor_battery_bank_state: u16,
    warranty_date: u32,
    battery_event_1_bitfield: u32,
    battery_event_2_bitfield: u32,
    vendor_event_bitfield_1: u32,
    vendor_event_bitfield_2: u32,
    external_battery_voltage: u16,
    max_battery_voltage: u16,
    min_battery_voltage: u16,
    max_cell_voltage: u16,
    max_cell_voltage_string: u16,
    max_cell_voltage_module: u16,
    min_cell_voltage: u16,
    min_cell_voltage_string: u16,
    min_cell_voltage_module: u16,
    average_cell_voltage: u16,
    total_dc_current: i16,
    max_charge_current: u16,
    max_discharge_current: u16,
    total_power: i16,
    inverter_state_request: ReqInvState,
    battery_power_request: i16,
    operation: SetOp,
    inverter_state: SetInvState,
    ah_rtg_sf: u16,
    wh_rtg_sf: u16,
    w_cha_dis_cha_max_sf: u16,
    dis_cha_rte_sf: u16,
    so_c_sf: u16,
    do_d_sf: u16,
    so_h_sf: u16,
    v_sf: u16,
    cell_v_sf: u16,
    a_sf: u16,
    a_max_sf: u16,
    w_sf: u16,
}

impl ModelAdapter for Model802StatefulAdapter {
    /// Nameplate Charge Capacity
    ///
    /// Nameplate charge capacity in amp-hours.
    fn nameplate_charge_capacity(&self) -> u16 {
        self.nameplate_charge_capacity
    }

    /// Nameplate Energy Capacity
    ///
    /// Nameplate energy capacity in DC watt-hours.
    fn nameplate_energy_capacity(&self) -> u16 {
        self.nameplate_energy_capacity
    }

    /// Nameplate Max Charge Rate
    ///
    /// Maximum rate of energy transfer into the storage device in DC watts.
    fn nameplate_max_charge_rate(&self) -> u16 {
        self.nameplate_max_charge_rate
    }

    /// Nameplate Max Discharge Rate
    ///
    /// Maximum rate of energy transfer out of the storage device in DC watts.
    fn nameplate_max_discharge_rate(&self) -> u16 {
        self.nameplate_max_discharge_rate
    }

    /// Self Discharge Rate
    ///
    /// Self discharge rate. Percentage of capacity (WHRtg) discharged per day.
    fn self_discharge_rate(&self) -> Option<u16> {
        Some(self.self_discharge_rate)
    }

    /// Nameplate Max SoC
    ///
    /// Manufacturer maximum state of charge, expressed as a percentage.
    fn nameplate_max_so_c(&self) -> Option<u16> {
        Some(self.nameplate_max_so_c)
    }

    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    fn nameplate_min_so_c(&self) -> Option<u16> {
        Some(self.nameplate_min_so_c)
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn max_reserve_percent(&self) -> Option<u16> {
        Some(self.max_reserve_percent)
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn set_max_reserve_percent(&mut self, value: u16) {
        self.max_reserve_percent = value;
    }

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn min_reserve_percent(&self) -> Option<u16> {
        Some(self.min_reserve_percent)
    }

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_min_reserve_percent(&mut self, value: u16) {
        self.min_reserve_percent = value;
    }

    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn state_of_charge(&self) -> u16 {
        self.state_of_charge
    }

    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Measurement.
    fn depth_of_discharge(&self) -> Option<u16> {
        Some(self.depth_of_discharge)
    }

    /// State of Health
    ///
    /// Percentage of battery life remaining.
    fn state_of_health(&self) -> Option<u16> {
        Some(self.state_of_health)
    }

    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    fn cycle_count(&self) -> Option<u32> {
        Some(self.cycle_count)
    }

    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    fn charge_status(&self) -> Option<ChaSt> {
        Some(self.charge_status)
    }

    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Maps to DRCC.LocRemCtl in IEC 61850.
    fn control_mode(&self) -> LocRemCtl {
        self.control_mode
    }

    /// Battery Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn battery_heartbeat(&self) -> Option<u16> {
        Some(self.battery_heartbeat)
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn controller_heartbeat(&self) -> Option<u16> {
        Some(self.controller_heartbeat)
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn set_controller_heartbeat(&mut self, value: u16) {
        self.controller_heartbeat = value;
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn alarm_reset(&self) -> u16 {
        self.alarm_reset
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn set_alarm_reset(&mut self, value: u16) {
        self.alarm_reset = value;
    }

    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Maps to DBAT.BatTyp in 61850.
    fn battery_type(&self) -> Typ {
        self.battery_type
    }

    /// State of the Battery Bank
    ///
    /// State of the battery bank. Enumeration.
    ///
    /// Must be reconciled with State in IEC 61850.
    fn state_of_the_battery_bank(&self) -> State {
        self.state_of_the_battery_bank
    }

    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state. Enumeration.
    fn vendor_battery_bank_state(&self) -> Option<u16> {
        Some(self.vendor_battery_bank_state)
    }

    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Number of days since 1/1/2000.
    fn warranty_date(&self) -> Option<u32> {
        Some(self.warranty_date)
    }

    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.
    fn battery_event_1_bitfield(&self) -> u32 {
        self.battery_event_1_bitfield
    }

    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.
    ///
    /// Reserved for future use.
    fn battery_event_2_bitfield(&self) -> u32 {
        self.battery_event_2_bitfield
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> u32 {
        self.vendor_event_bitfield_1
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> u32 {
        self.vendor_event_bitfield_2
    }

    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Maps to ZBAT.V in IEC 61850.
    fn external_battery_voltage(&self) -> u16 {
        self.external_battery_voltage
    }

    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn max_battery_voltage(&self) -> Option<u16> {
        Some(self.max_battery_voltage)
    }

    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn min_battery_voltage(&self) -> Option<u16> {
        Some(self.min_battery_voltage)
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> Option<u16> {
        Some(self.max_cell_voltage)
    }

    /// Max Cell Voltage String
    ///
    /// String containing the cell with maximum voltage.
    fn max_cell_voltage_string(&self) -> Option<u16> {
        Some(self.max_cell_voltage_string)
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        Some(self.max_cell_voltage_module)
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> Option<u16> {
        Some(self.min_cell_voltage)
    }

    /// Min Cell Voltage String
    ///
    /// String containing the cell with minimum voltage.
    fn min_cell_voltage_string(&self) -> Option<u16> {
        Some(self.min_cell_voltage_string)
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        Some(self.min_cell_voltage_module)
    }

    /// Average Cell Voltage
    ///
    /// Average cell voltage for all cells in the bank.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> Option<u16> {
        Some(self.average_cell_voltage)
    }

    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Measurement.
    fn total_dc_current(&self) -> i16 {
        self.total_dc_current
    }

    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn max_charge_current(&self) -> Option<u16> {
        Some(self.max_charge_current)
    }

    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn max_discharge_current(&self) -> Option<u16> {
        Some(self.max_discharge_current)
    }

    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// DC Measurement.
    fn total_power(&self) -> i16 {
        self.total_power
    }

    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter. Enumeration.
    ///
    /// Used in special states such as manual battery charging.
    fn inverter_state_request(&self) -> Option<ReqInvState> {
        Some(self.inverter_state_request)
    }

    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Used in special states such as string balancing.
    fn battery_power_request(&self) -> Option<i16> {
        Some(self.battery_power_request)
    }

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn operation(&self) -> SetOp {
        self.operation
    }

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn set_operation(&mut self, value: SetOp) {
        self.operation = value;
    }

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn inverter_state(&self) -> SetInvState {
        self.inverter_state
    }

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_inverter_state(&mut self, value: SetInvState) {
        self.inverter_state = value;
    }

    /// Scale factor for charge capacity.
    fn ah_rtg_sf(&self) -> u16 {
        self.ah_rtg_sf
    }

    /// Scale factor for energy capacity.
    fn wh_rtg_sf(&self) -> u16 {
        self.wh_rtg_sf
    }

    /// Scale factor for maximum charge and discharge rate.
    fn w_cha_dis_cha_max_sf(&self) -> u16 {
        self.w_cha_dis_cha_max_sf
    }

    /// Scale factor for self discharge rate.
    fn dis_cha_rte_sf(&self) -> Option<u16> {
        Some(self.dis_cha_rte_sf)
    }

    /// Scale factor for state of charge values.
    fn so_c_sf(&self) -> u16 {
        self.so_c_sf
    }

    /// Scale factor for depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        Some(self.do_d_sf)
    }

    /// Scale factor for state of health.
    fn so_h_sf(&self) -> Option<u16> {
        Some(self.so_h_sf)
    }

    /// Scale factor for DC bus voltage.
    fn v_sf(&self) -> u16 {
        self.v_sf
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        self.cell_v_sf
    }

    /// Scale factor for DC current.
    fn a_sf(&self) -> u16 {
        self.a_sf
    }

    /// Scale factor for instantaneous DC charge/discharge current.
    fn a_max_sf(&self) -> u16 {
        self.a_max_sf
    }

    /// Scale factor for AC power request.
    fn w_sf(&self) -> Option<u16> {
        Some(self.w_sf)
    }
}
