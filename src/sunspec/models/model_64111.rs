use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 25;

pub static POINTS: [ReadablePoint; 25] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64111 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 23 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::PortNumber,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 { point: Point::VSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 { point: Point::ASf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 { point: Point::PSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 { point: Point::AhSf },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::KwhSf,
        },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::BatteryVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::ArrayVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::OutputCurrent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::ArrayCurrent,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::OperatingState,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::OutputWattage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::TodaySMinimumBatteryVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::TodaySMaximumBatteryVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 { point: Point::Voc },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::TodaySMaximumVoc,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::TodaySKWh,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::TodaySAh,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::LifetimeKWh,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::LifetimeKAh,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::LifetimeMaximumOutputWattage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::LifetimeMaximumBatteryVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::LifetimeMaximumVocVoltage,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    PortNumber,
    VSf,
    ASf,
    PSf,
    AhSf,
    KwhSf,
    BatteryVoltage,
    ArrayVoltage,
    OutputCurrent,
    ArrayCurrent,
    OperatingState,
    OutputWattage,
    TodaySMinimumBatteryVoltage,
    TodaySMaximumBatteryVoltage,
    Voc,
    TodaySMaximumVoc,
    TodaySKWh,
    TodaySAh,
    LifetimeKWh,
    LifetimeKAh,
    LifetimeMaximumOutputWattage,
    LifetimeMaximumBatteryVoltage,
    LifetimeMaximumVocVoltage,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::PortNumber => serialisation::write_u16(model.port_number(), buffer),
        Point::VSf => serialisation::write_u16(model.v_sf(), buffer),
        Point::ASf => serialisation::write_u16(model.a_sf(), buffer),
        Point::PSf => serialisation::write_u16(model.p_sf(), buffer),
        Point::AhSf => serialisation::write_u16(model.ah_sf(), buffer),
        Point::KwhSf => serialisation::write_u16(model.kwh_sf(), buffer),
        Point::BatteryVoltage => serialisation::write_u16(model.battery_voltage(), buffer),
        Point::ArrayVoltage => serialisation::write_u16(model.array_voltage(), buffer),
        Point::OutputCurrent => serialisation::write_u16(model.output_current(), buffer),
        Point::ArrayCurrent => serialisation::write_u16(model.array_current(), buffer),
        Point::OperatingState => serialisation::write_u16(model.operating_state() as u16, buffer),
        Point::OutputWattage => serialisation::write_u16(model.output_wattage(), buffer),
        Point::TodaySMinimumBatteryVoltage => {
            serialisation::write_u16(model.today_s_minimum_battery_voltage(), buffer)
        }
        Point::TodaySMaximumBatteryVoltage => {
            serialisation::write_u16(model.today_s_maximum_battery_voltage(), buffer)
        }
        Point::Voc => serialisation::write_u16(model.voc(), buffer),
        Point::TodaySMaximumVoc => serialisation::write_u16(model.today_s_maximum_voc(), buffer),
        Point::TodaySKWh => serialisation::write_u16(model.today_s_k_wh(), buffer),
        Point::TodaySAh => serialisation::write_u16(model.today_s_ah(), buffer),
        Point::LifetimeKWh => serialisation::write_u16(model.lifetime_k_wh(), buffer),
        Point::LifetimeKAh => serialisation::write_u16(model.lifetime_k_ah(), buffer),
        Point::LifetimeMaximumOutputWattage => {
            serialisation::write_u16(model.lifetime_maximum_output_wattage(), buffer)
        }
        Point::LifetimeMaximumBatteryVoltage => {
            serialisation::write_u16(model.lifetime_maximum_battery_voltage(), buffer)
        }
        Point::LifetimeMaximumVocVoltage => {
            serialisation::write_u16(model.lifetime_maximum_voc_voltage(), buffer)
        }
    }
}

pub trait ModelAdapter {
    /// Port Number
    fn port_number(&self) -> u16;

    fn v_sf(&self) -> u16;

    fn a_sf(&self) -> u16;

    fn p_sf(&self) -> u16;

    fn ah_sf(&self) -> u16;

    fn kwh_sf(&self) -> u16;

    /// Battery Voltage
    fn battery_voltage(&self) -> u16;

    /// Array Voltage
    fn array_voltage(&self) -> u16;

    /// Output Current
    fn output_current(&self) -> u16;

    /// Array Current
    fn array_current(&self) -> u16;

    /// Operating State
    fn operating_state(&self) -> ChargerSt;

    /// Output Wattage
    fn output_wattage(&self) -> u16;

    /// Today's Minimum Battery Voltage
    fn today_s_minimum_battery_voltage(&self) -> u16;

    /// Today's Maximum Battery Voltage
    fn today_s_maximum_battery_voltage(&self) -> u16;

    /// VOC
    fn voc(&self) -> u16;

    /// Today's Maximum VOC
    fn today_s_maximum_voc(&self) -> u16;

    /// Today's kWh
    fn today_s_k_wh(&self) -> u16;

    /// Today's AH
    fn today_s_ah(&self) -> u16;

    /// Lifetime kWh
    fn lifetime_k_wh(&self) -> u16;

    /// Lifetime kAH
    fn lifetime_k_ah(&self) -> u16;

    /// Lifetime Maximum Output Wattage
    fn lifetime_maximum_output_wattage(&self) -> u16;

    /// Lifetime Maximum Battery Voltage
    fn lifetime_maximum_battery_voltage(&self) -> u16;

    /// Lifetime Maximum VOC Voltage
    fn lifetime_maximum_voc_voltage(&self) -> u16;
}

pub enum ChargerSt {
    Off = 0,
    Float = 1,
    Bulk = 2,
    Absorb = 3,
    Eq = 4,
}

#[repr(C)]
pub struct Model64111CallbackAdapter {
    port_number_callback: extern "C" fn() -> u16,
    v_sf_callback: extern "C" fn() -> u16,
    a_sf_callback: extern "C" fn() -> u16,
    p_sf_callback: extern "C" fn() -> u16,
    ah_sf_callback: extern "C" fn() -> u16,
    kwh_sf_callback: extern "C" fn() -> u16,
    battery_voltage_callback: extern "C" fn() -> u16,
    array_voltage_callback: extern "C" fn() -> u16,
    output_current_callback: extern "C" fn() -> u16,
    array_current_callback: extern "C" fn() -> u16,
    operating_state_callback: extern "C" fn() -> ChargerSt,
    output_wattage_callback: extern "C" fn() -> u16,
    today_s_minimum_battery_voltage_callback: extern "C" fn() -> u16,
    today_s_maximum_battery_voltage_callback: extern "C" fn() -> u16,
    voc_callback: extern "C" fn() -> u16,
    today_s_maximum_voc_callback: extern "C" fn() -> u16,
    today_s_k_wh_callback: extern "C" fn() -> u16,
    today_s_ah_callback: extern "C" fn() -> u16,
    lifetime_k_wh_callback: extern "C" fn() -> u16,
    lifetime_k_ah_callback: extern "C" fn() -> u16,
    lifetime_maximum_output_wattage_callback: extern "C" fn() -> u16,
    lifetime_maximum_battery_voltage_callback: extern "C" fn() -> u16,
    lifetime_maximum_voc_voltage_callback: extern "C" fn() -> u16,
}

impl ModelAdapter for Model64111CallbackAdapter {
    /// Port Number
    fn port_number(&self) -> u16 {
        (self.port_number_callback)()
    }

    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)()
    }

    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)()
    }

    fn p_sf(&self) -> u16 {
        (self.p_sf_callback)()
    }

    fn ah_sf(&self) -> u16 {
        (self.ah_sf_callback)()
    }

    fn kwh_sf(&self) -> u16 {
        (self.kwh_sf_callback)()
    }

    /// Battery Voltage
    fn battery_voltage(&self) -> u16 {
        (self.battery_voltage_callback)()
    }

    /// Array Voltage
    fn array_voltage(&self) -> u16 {
        (self.array_voltage_callback)()
    }

    /// Output Current
    fn output_current(&self) -> u16 {
        (self.output_current_callback)()
    }

    /// Array Current
    fn array_current(&self) -> u16 {
        (self.array_current_callback)()
    }

    /// Operating State
    fn operating_state(&self) -> ChargerSt {
        (self.operating_state_callback)()
    }

    /// Output Wattage
    fn output_wattage(&self) -> u16 {
        (self.output_wattage_callback)()
    }

    /// Today's Minimum Battery Voltage
    fn today_s_minimum_battery_voltage(&self) -> u16 {
        (self.today_s_minimum_battery_voltage_callback)()
    }

    /// Today's Maximum Battery Voltage
    fn today_s_maximum_battery_voltage(&self) -> u16 {
        (self.today_s_maximum_battery_voltage_callback)()
    }

    /// VOC
    fn voc(&self) -> u16 {
        (self.voc_callback)()
    }

    /// Today's Maximum VOC
    fn today_s_maximum_voc(&self) -> u16 {
        (self.today_s_maximum_voc_callback)()
    }

    /// Today's kWh
    fn today_s_k_wh(&self) -> u16 {
        (self.today_s_k_wh_callback)()
    }

    /// Today's AH
    fn today_s_ah(&self) -> u16 {
        (self.today_s_ah_callback)()
    }

    /// Lifetime kWh
    fn lifetime_k_wh(&self) -> u16 {
        (self.lifetime_k_wh_callback)()
    }

    /// Lifetime kAH
    fn lifetime_k_ah(&self) -> u16 {
        (self.lifetime_k_ah_callback)()
    }

    /// Lifetime Maximum Output Wattage
    fn lifetime_maximum_output_wattage(&self) -> u16 {
        (self.lifetime_maximum_output_wattage_callback)()
    }

    /// Lifetime Maximum Battery Voltage
    fn lifetime_maximum_battery_voltage(&self) -> u16 {
        (self.lifetime_maximum_battery_voltage_callback)()
    }

    /// Lifetime Maximum VOC Voltage
    fn lifetime_maximum_voc_voltage(&self) -> u16 {
        (self.lifetime_maximum_voc_voltage_callback)()
    }
}
