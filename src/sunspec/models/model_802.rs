use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 64;

pub static POINTS: [ReadablePoint; 58] = [
    ReadablePoint {
        reference: PointReference::Static { value: 802 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 62 },
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
            point: Point::SetOperation,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model802 {
            point: Point::SetInverterState,
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
    SetOperation,
    SetInverterState,
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

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::NameplateChargeCapacity => {
            serialisation::write_u16(model.nameplate_charge_capacity(), buffer)
        }
        Point::NameplateEnergyCapacity => {
            serialisation::write_u16(model.nameplate_energy_capacity(), buffer)
        }
        Point::NameplateMaxChargeRate => {
            serialisation::write_u16(model.nameplate_max_charge_rate(), buffer)
        }
        Point::NameplateMaxDischargeRate => {
            serialisation::write_u16(model.nameplate_max_discharge_rate(), buffer)
        }
        Point::SelfDischargeRate => {
            if let Some(value) = model.self_discharge_rate() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::NameplateMaxSoC => {
            if let Some(value) = model.nameplate_max_so_c() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::NameplateMinSoC => {
            if let Some(value) = model.nameplate_min_so_c() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaxReservePercent => {
            if let Some(value) = model.max_reserve_percent() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MinReservePercent => {
            if let Some(value) = model.min_reserve_percent() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::StateOfCharge => serialisation::write_u16(model.state_of_charge(), buffer),
        Point::DepthOfDischarge => {
            if let Some(value) = model.depth_of_discharge() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::StateOfHealth => {
            if let Some(value) = model.state_of_health() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::CycleCount => {
            if let Some(value) = model.cycle_count() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::ChargeStatus => {
            if let Some(value) = model.charge_status() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::ControlMode => serialisation::write_u16(model.control_mode() as u16, buffer),
        Point::BatteryHeartbeat => {
            if let Some(value) = model.battery_heartbeat() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ControllerHeartbeat => {
            if let Some(value) = model.controller_heartbeat() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::AlarmReset => serialisation::write_u16(model.alarm_reset(), buffer),
        Point::BatteryType => serialisation::write_u16(model.battery_type() as u16, buffer),
        Point::StateOfTheBatteryBank => {
            serialisation::write_u16(model.state_of_the_battery_bank() as u16, buffer)
        }
        Point::VendorBatteryBankState => {
            if let Some(value) = model.vendor_battery_bank_state() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::WarrantyDate => {
            if let Some(value) = model.warranty_date() {
                serialisation::write_u32(value, buffer, offset, limit);
            }
        }
        Point::BatteryEvent1Bitfield => {
            serialisation::write_u32(model.battery_event_1_bitfield(), buffer, offset, limit)
        }
        Point::BatteryEvent2Bitfield => {
            serialisation::write_u32(model.battery_event_2_bitfield(), buffer, offset, limit)
        }
        Point::VendorEventBitfield1 => {
            serialisation::write_u32(model.vendor_event_bitfield_1(), buffer, offset, limit)
        }
        Point::VendorEventBitfield2 => {
            serialisation::write_u32(model.vendor_event_bitfield_2(), buffer, offset, limit)
        }
        Point::ExternalBatteryVoltage => {
            serialisation::write_u16(model.external_battery_voltage(), buffer)
        }
        Point::MaxBatteryVoltage => {
            if let Some(value) = model.max_battery_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MinBatteryVoltage => {
            if let Some(value) = model.min_battery_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaxCellVoltage => {
            if let Some(value) = model.max_cell_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaxCellVoltageString => {
            if let Some(value) = model.max_cell_voltage_string() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaxCellVoltageModule => {
            if let Some(value) = model.max_cell_voltage_module() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MinCellVoltage => {
            if let Some(value) = model.min_cell_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MinCellVoltageString => {
            if let Some(value) = model.min_cell_voltage_string() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MinCellVoltageModule => {
            if let Some(value) = model.min_cell_voltage_module() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::AverageCellVoltage => {
            if let Some(value) = model.average_cell_voltage() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TotalDcCurrent => serialisation::write_i16(model.total_dc_current(), buffer),
        Point::MaxChargeCurrent => {
            if let Some(value) = model.max_charge_current() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::MaxDischargeCurrent => {
            if let Some(value) = model.max_discharge_current() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::TotalPower => serialisation::write_i16(model.total_power(), buffer),
        Point::InverterStateRequest => {
            if let Some(value) = model.inverter_state_request() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::BatteryPowerRequest => {
            if let Some(value) = model.battery_power_request() {
                serialisation::write_i16(value, buffer);
            }
        }
        Point::SetOperation => serialisation::write_u16(model.set_operation() as u16, buffer),
        Point::SetInverterState => {
            serialisation::write_u16(model.set_inverter_state() as u16, buffer)
        }
        Point::AhRtgSf => serialisation::write_u16(model.ah_rtg_sf(), buffer),
        Point::WhRtgSf => serialisation::write_u16(model.wh_rtg_sf(), buffer),
        Point::WChaDisChaMaxSf => serialisation::write_u16(model.w_cha_dis_cha_max_sf(), buffer),
        Point::DisChaRteSf => {
            if let Some(value) = model.dis_cha_rte_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::SoCSf => serialisation::write_u16(model.so_c_sf(), buffer),
        Point::DoDSf => {
            if let Some(value) = model.do_d_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::SoHSf => {
            if let Some(value) = model.so_h_sf() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::VSf => serialisation::write_u16(model.v_sf(), buffer),
        Point::CellVSf => serialisation::write_u16(model.cell_v_sf(), buffer),
        Point::ASf => serialisation::write_u16(model.a_sf(), buffer),
        Point::AMaxSf => serialisation::write_u16(model.a_max_sf(), buffer),
        Point::WSf => {
            if let Some(value) = model.w_sf() {
                serialisation::write_u16(value, buffer);
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
    fn set_operation(&self) -> SetOp;

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn set_set_operation(&mut self, value: SetOp);

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_inverter_state(&self) -> SetInvState;

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_set_inverter_state(&mut self, value: SetInvState);

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

#[repr(u16)]
pub enum LocRemCtl {
    /// Value of 0 matches LocRemCtl in IEC 61850.
    Remote = 0,
    /// Value of 1 matches LocRemCtl in IEC 61850.
    Local = 1,
}

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

#[repr(u16)]
pub enum ReqInvState {
    NoRequest = 0,
    /// Battery is notified of inverter state change through SetInvState.
    Start = 1,
    /// Battery is notified of inverter state change through SetInvState.
    Stop = 2,
}

#[repr(u16)]
pub enum SetOp {
    Connect = 1,
    Disconnect = 2,
}

#[repr(u16)]
pub enum SetInvState {
    InverterStopped = 1,
    InverterStandby = 2,
    InverterStarted = 3,
}

#[repr(C)]
pub struct Model802CallbackAdapter {
    nameplate_charge_capacity_callback: extern "C" fn() -> u16,
    nameplate_energy_capacity_callback: extern "C" fn() -> u16,
    nameplate_max_charge_rate_callback: extern "C" fn() -> u16,
    nameplate_max_discharge_rate_callback: extern "C" fn() -> u16,
    self_discharge_rate_callback: Option<extern "C" fn() -> u16>,
    nameplate_max_so_c_callback: Option<extern "C" fn() -> u16>,
    nameplate_min_so_c_callback: Option<extern "C" fn() -> u16>,
    max_reserve_percent_callback: Option<extern "C" fn() -> u16>,
    set_max_reserve_percent_callback: Option<extern "C" fn(u16)>,
    min_reserve_percent_callback: Option<extern "C" fn() -> u16>,
    set_min_reserve_percent_callback: Option<extern "C" fn(u16)>,
    state_of_charge_callback: extern "C" fn() -> u16,
    depth_of_discharge_callback: Option<extern "C" fn() -> u16>,
    state_of_health_callback: Option<extern "C" fn() -> u16>,
    cycle_count_callback: Option<extern "C" fn() -> u32>,
    charge_status_callback: Option<extern "C" fn() -> ChaSt>,
    control_mode_callback: extern "C" fn() -> LocRemCtl,
    battery_heartbeat_callback: Option<extern "C" fn() -> u16>,
    controller_heartbeat_callback: Option<extern "C" fn() -> u16>,
    set_controller_heartbeat_callback: Option<extern "C" fn(u16)>,
    alarm_reset_callback: extern "C" fn() -> u16,
    set_alarm_reset_callback: extern "C" fn(u16),
    battery_type_callback: extern "C" fn() -> Typ,
    state_of_the_battery_bank_callback: extern "C" fn() -> State,
    vendor_battery_bank_state_callback: Option<extern "C" fn() -> u16>,
    warranty_date_callback: Option<extern "C" fn() -> u32>,
    battery_event_1_bitfield_callback: extern "C" fn() -> u32,
    battery_event_2_bitfield_callback: extern "C" fn() -> u32,
    vendor_event_bitfield_1_callback: extern "C" fn() -> u32,
    vendor_event_bitfield_2_callback: extern "C" fn() -> u32,
    external_battery_voltage_callback: extern "C" fn() -> u16,
    max_battery_voltage_callback: Option<extern "C" fn() -> u16>,
    min_battery_voltage_callback: Option<extern "C" fn() -> u16>,
    max_cell_voltage_callback: Option<extern "C" fn() -> u16>,
    max_cell_voltage_string_callback: Option<extern "C" fn() -> u16>,
    max_cell_voltage_module_callback: Option<extern "C" fn() -> u16>,
    min_cell_voltage_callback: Option<extern "C" fn() -> u16>,
    min_cell_voltage_string_callback: Option<extern "C" fn() -> u16>,
    min_cell_voltage_module_callback: Option<extern "C" fn() -> u16>,
    average_cell_voltage_callback: Option<extern "C" fn() -> u16>,
    total_dc_current_callback: extern "C" fn() -> i16,
    max_charge_current_callback: Option<extern "C" fn() -> u16>,
    max_discharge_current_callback: Option<extern "C" fn() -> u16>,
    total_power_callback: extern "C" fn() -> i16,
    inverter_state_request_callback: Option<extern "C" fn() -> ReqInvState>,
    battery_power_request_callback: Option<extern "C" fn() -> i16>,
    set_operation_callback: extern "C" fn() -> SetOp,
    set_set_operation_callback: extern "C" fn(SetOp),
    set_inverter_state_callback: extern "C" fn() -> SetInvState,
    set_set_inverter_state_callback: extern "C" fn(SetInvState),
    ah_rtg_sf_callback: extern "C" fn() -> u16,
    wh_rtg_sf_callback: extern "C" fn() -> u16,
    w_cha_dis_cha_max_sf_callback: extern "C" fn() -> u16,
    dis_cha_rte_sf_callback: Option<extern "C" fn() -> u16>,
    so_c_sf_callback: extern "C" fn() -> u16,
    do_d_sf_callback: Option<extern "C" fn() -> u16>,
    so_h_sf_callback: Option<extern "C" fn() -> u16>,
    v_sf_callback: extern "C" fn() -> u16,
    cell_v_sf_callback: extern "C" fn() -> u16,
    a_sf_callback: extern "C" fn() -> u16,
    a_max_sf_callback: extern "C" fn() -> u16,
    w_sf_callback: Option<extern "C" fn() -> u16>,
}

impl ModelAdapter for Model802CallbackAdapter {
    /// Nameplate Charge Capacity
    ///
    /// Nameplate charge capacity in amp-hours.
    fn nameplate_charge_capacity(&self) -> u16 {
        (self.nameplate_charge_capacity_callback)()
    }

    /// Nameplate Energy Capacity
    ///
    /// Nameplate energy capacity in DC watt-hours.
    fn nameplate_energy_capacity(&self) -> u16 {
        (self.nameplate_energy_capacity_callback)()
    }

    /// Nameplate Max Charge Rate
    ///
    /// Maximum rate of energy transfer into the storage device in DC watts.
    fn nameplate_max_charge_rate(&self) -> u16 {
        (self.nameplate_max_charge_rate_callback)()
    }

    /// Nameplate Max Discharge Rate
    ///
    /// Maximum rate of energy transfer out of the storage device in DC watts.
    fn nameplate_max_discharge_rate(&self) -> u16 {
        (self.nameplate_max_discharge_rate_callback)()
    }

    /// Self Discharge Rate
    ///
    /// Self discharge rate. Percentage of capacity (WHRtg) discharged per day.
    fn self_discharge_rate(&self) -> Option<u16> {
        self.self_discharge_rate_callback
            .map(|callback| (callback)())
    }

    /// Nameplate Max SoC
    ///
    /// Manufacturer maximum state of charge, expressed as a percentage.
    fn nameplate_max_so_c(&self) -> Option<u16> {
        self.nameplate_max_so_c_callback
            .map(|callback| (callback)())
    }

    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    fn nameplate_min_so_c(&self) -> Option<u16> {
        self.nameplate_min_so_c_callback
            .map(|callback| (callback)())
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn max_reserve_percent(&self) -> Option<u16> {
        self.max_reserve_percent_callback
            .map(|callback| (callback)())
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn set_max_reserve_percent(&mut self, value: u16) {
        if let Some(callback) = self.set_max_reserve_percent_callback {
            (callback)(value);
        };
    }

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn min_reserve_percent(&self) -> Option<u16> {
        self.min_reserve_percent_callback
            .map(|callback| (callback)())
    }

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_min_reserve_percent(&mut self, value: u16) {
        if let Some(callback) = self.set_min_reserve_percent_callback {
            (callback)(value);
        };
    }

    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn state_of_charge(&self) -> u16 {
        (self.state_of_charge_callback)()
    }

    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Measurement.
    fn depth_of_discharge(&self) -> Option<u16> {
        self.depth_of_discharge_callback
            .map(|callback| (callback)())
    }

    /// State of Health
    ///
    /// Percentage of battery life remaining.
    fn state_of_health(&self) -> Option<u16> {
        self.state_of_health_callback.map(|callback| (callback)())
    }

    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    fn cycle_count(&self) -> Option<u32> {
        self.cycle_count_callback.map(|callback| (callback)())
    }

    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    fn charge_status(&self) -> Option<ChaSt> {
        self.charge_status_callback.map(|callback| (callback)())
    }

    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Maps to DRCC.LocRemCtl in IEC 61850.
    fn control_mode(&self) -> LocRemCtl {
        (self.control_mode_callback)()
    }

    /// Battery Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn battery_heartbeat(&self) -> Option<u16> {
        self.battery_heartbeat_callback.map(|callback| (callback)())
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn controller_heartbeat(&self) -> Option<u16> {
        self.controller_heartbeat_callback
            .map(|callback| (callback)())
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn set_controller_heartbeat(&mut self, value: u16) {
        if let Some(callback) = self.set_controller_heartbeat_callback {
            (callback)(value);
        };
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn alarm_reset(&self) -> u16 {
        (self.alarm_reset_callback)()
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn set_alarm_reset(&mut self, value: u16) {
        (self.set_alarm_reset_callback)(value);
    }

    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Maps to DBAT.BatTyp in 61850.
    fn battery_type(&self) -> Typ {
        (self.battery_type_callback)()
    }

    /// State of the Battery Bank
    ///
    /// State of the battery bank. Enumeration.
    ///
    /// Must be reconciled with State in IEC 61850.
    fn state_of_the_battery_bank(&self) -> State {
        (self.state_of_the_battery_bank_callback)()
    }

    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state. Enumeration.
    fn vendor_battery_bank_state(&self) -> Option<u16> {
        self.vendor_battery_bank_state_callback
            .map(|callback| (callback)())
    }

    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Number of days since 1/1/2000.
    fn warranty_date(&self) -> Option<u32> {
        self.warranty_date_callback.map(|callback| (callback)())
    }

    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.
    fn battery_event_1_bitfield(&self) -> u32 {
        (self.battery_event_1_bitfield_callback)()
    }

    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.
    ///
    /// Reserved for future use.
    fn battery_event_2_bitfield(&self) -> u32 {
        (self.battery_event_2_bitfield_callback)()
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_1(&self) -> u32 {
        (self.vendor_event_bitfield_1_callback)()
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn vendor_event_bitfield_2(&self) -> u32 {
        (self.vendor_event_bitfield_2_callback)()
    }

    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Maps to ZBAT.V in IEC 61850.
    fn external_battery_voltage(&self) -> u16 {
        (self.external_battery_voltage_callback)()
    }

    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn max_battery_voltage(&self) -> Option<u16> {
        self.max_battery_voltage_callback
            .map(|callback| (callback)())
    }

    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn min_battery_voltage(&self) -> Option<u16> {
        self.min_battery_voltage_callback
            .map(|callback| (callback)())
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn max_cell_voltage(&self) -> Option<u16> {
        self.max_cell_voltage_callback.map(|callback| (callback)())
    }

    /// Max Cell Voltage String
    ///
    /// String containing the cell with maximum voltage.
    fn max_cell_voltage_string(&self) -> Option<u16> {
        self.max_cell_voltage_string_callback
            .map(|callback| (callback)())
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum voltage.
    fn max_cell_voltage_module(&self) -> Option<u16> {
        self.max_cell_voltage_module_callback
            .map(|callback| (callback)())
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn min_cell_voltage(&self) -> Option<u16> {
        self.min_cell_voltage_callback.map(|callback| (callback)())
    }

    /// Min Cell Voltage String
    ///
    /// String containing the cell with minimum voltage.
    fn min_cell_voltage_string(&self) -> Option<u16> {
        self.min_cell_voltage_string_callback
            .map(|callback| (callback)())
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum voltage.
    fn min_cell_voltage_module(&self) -> Option<u16> {
        self.min_cell_voltage_module_callback
            .map(|callback| (callback)())
    }

    /// Average Cell Voltage
    ///
    /// Average cell voltage for all cells in the bank.
    ///
    /// Calculation based on measurements.
    fn average_cell_voltage(&self) -> Option<u16> {
        self.average_cell_voltage_callback
            .map(|callback| (callback)())
    }

    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Measurement.
    fn total_dc_current(&self) -> i16 {
        (self.total_dc_current_callback)()
    }

    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn max_charge_current(&self) -> Option<u16> {
        self.max_charge_current_callback
            .map(|callback| (callback)())
    }

    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn max_discharge_current(&self) -> Option<u16> {
        self.max_discharge_current_callback
            .map(|callback| (callback)())
    }

    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// DC Measurement.
    fn total_power(&self) -> i16 {
        (self.total_power_callback)()
    }

    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter. Enumeration.
    ///
    /// Used in special states such as manual battery charging.
    fn inverter_state_request(&self) -> Option<ReqInvState> {
        self.inverter_state_request_callback
            .map(|callback| (callback)())
    }

    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Used in special states such as string balancing.
    fn battery_power_request(&self) -> Option<i16> {
        self.battery_power_request_callback
            .map(|callback| (callback)())
    }

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn set_operation(&self) -> SetOp {
        (self.set_operation_callback)()
    }

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn set_set_operation(&mut self, value: SetOp) {
        (self.set_set_operation_callback)(value);
    }

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_inverter_state(&self) -> SetInvState {
        (self.set_inverter_state_callback)()
    }

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_set_inverter_state(&mut self, value: SetInvState) {
        (self.set_set_inverter_state_callback)(value);
    }

    /// Scale factor for charge capacity.
    fn ah_rtg_sf(&self) -> u16 {
        (self.ah_rtg_sf_callback)()
    }

    /// Scale factor for energy capacity.
    fn wh_rtg_sf(&self) -> u16 {
        (self.wh_rtg_sf_callback)()
    }

    /// Scale factor for maximum charge and discharge rate.
    fn w_cha_dis_cha_max_sf(&self) -> u16 {
        (self.w_cha_dis_cha_max_sf_callback)()
    }

    /// Scale factor for self discharge rate.
    fn dis_cha_rte_sf(&self) -> Option<u16> {
        self.dis_cha_rte_sf_callback.map(|callback| (callback)())
    }

    /// Scale factor for state of charge values.
    fn so_c_sf(&self) -> u16 {
        (self.so_c_sf_callback)()
    }

    /// Scale factor for depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        self.do_d_sf_callback.map(|callback| (callback)())
    }

    /// Scale factor for state of health.
    fn so_h_sf(&self) -> Option<u16> {
        self.so_h_sf_callback.map(|callback| (callback)())
    }

    /// Scale factor for DC bus voltage.
    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)()
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16 {
        (self.cell_v_sf_callback)()
    }

    /// Scale factor for DC current.
    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)()
    }

    /// Scale factor for instantaneous DC charge/discharge current.
    fn a_max_sf(&self) -> u16 {
        (self.a_max_sf_callback)()
    }

    /// Scale factor for AC power request.
    fn w_sf(&self) -> Option<u16> {
        self.w_sf_callback.map(|callback| (callback)())
    }
}
