use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::c_void;

pub const SIZE: u16 = 25;

pub static POINTS: [ReadablePoint; 25] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64111 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64111 {
            point: Point::ModelLength,
        },
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
    ModelLength,
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

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    25
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
        Point::PortNumber => {
            buffer::write_u16(model.port_number(), buffer);
        }
        Point::VSf => {
            buffer::write_u16(model.v_sf(), buffer);
        }
        Point::ASf => {
            buffer::write_u16(model.a_sf(), buffer);
        }
        Point::PSf => {
            buffer::write_u16(model.p_sf(), buffer);
        }
        Point::AhSf => {
            buffer::write_u16(model.ah_sf(), buffer);
        }
        Point::KwhSf => {
            buffer::write_u16(model.kwh_sf(), buffer);
        }
        Point::BatteryVoltage => {
            buffer::write_u16(model.battery_voltage(), buffer);
        }
        Point::ArrayVoltage => {
            buffer::write_u16(model.array_voltage(), buffer);
        }
        Point::OutputCurrent => {
            buffer::write_u16(model.output_current(), buffer);
        }
        Point::ArrayCurrent => {
            buffer::write_u16(model.array_current(), buffer);
        }
        Point::OperatingState => {
            buffer::write_u16(model.operating_state() as u16, buffer);
        }
        Point::OutputWattage => {
            buffer::write_u16(model.output_wattage(), buffer);
        }
        Point::TodaySMinimumBatteryVoltage => {
            buffer::write_u16(model.today_s_minimum_battery_voltage(), buffer);
        }
        Point::TodaySMaximumBatteryVoltage => {
            buffer::write_u16(model.today_s_maximum_battery_voltage(), buffer);
        }
        Point::Voc => {
            buffer::write_u16(model.voc(), buffer);
        }
        Point::TodaySMaximumVoc => {
            buffer::write_u16(model.today_s_maximum_voc(), buffer);
        }
        Point::TodaySKWh => {
            buffer::write_u16(model.today_s_k_wh(), buffer);
        }
        Point::TodaySAh => {
            buffer::write_u16(model.today_s_ah(), buffer);
        }
        Point::LifetimeKWh => {
            buffer::write_u16(model.lifetime_k_wh(), buffer);
        }
        Point::LifetimeKAh => {
            buffer::write_u16(model.lifetime_k_ah(), buffer);
        }
        Point::LifetimeMaximumOutputWattage => {
            buffer::write_u16(model.lifetime_maximum_output_wattage(), buffer);
        }
        Point::LifetimeMaximumBatteryVoltage => {
            buffer::write_u16(model.lifetime_maximum_battery_voltage(), buffer);
        }
        Point::LifetimeMaximumVocVoltage => {
            buffer::write_u16(model.lifetime_maximum_voc_voltage(), buffer);
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

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum ChargerSt {
    Off = 0,
    Float = 1,
    Bulk = 2,
    Absorb = 3,
    Eq = 4,
}

#[repr(C)]
pub struct Model64111CallbackAdapter {
    context: *mut c_void,
    port_number_callback: extern "C" fn(*const c_void) -> u16,
    v_sf_callback: extern "C" fn(*const c_void) -> u16,
    a_sf_callback: extern "C" fn(*const c_void) -> u16,
    p_sf_callback: extern "C" fn(*const c_void) -> u16,
    ah_sf_callback: extern "C" fn(*const c_void) -> u16,
    kwh_sf_callback: extern "C" fn(*const c_void) -> u16,
    battery_voltage_callback: extern "C" fn(*const c_void) -> u16,
    array_voltage_callback: extern "C" fn(*const c_void) -> u16,
    output_current_callback: extern "C" fn(*const c_void) -> u16,
    array_current_callback: extern "C" fn(*const c_void) -> u16,
    operating_state_callback: extern "C" fn(*const c_void) -> ChargerSt,
    output_wattage_callback: extern "C" fn(*const c_void) -> u16,
    today_s_minimum_battery_voltage_callback: extern "C" fn(*const c_void) -> u16,
    today_s_maximum_battery_voltage_callback: extern "C" fn(*const c_void) -> u16,
    voc_callback: extern "C" fn(*const c_void) -> u16,
    today_s_maximum_voc_callback: extern "C" fn(*const c_void) -> u16,
    today_s_k_wh_callback: extern "C" fn(*const c_void) -> u16,
    today_s_ah_callback: extern "C" fn(*const c_void) -> u16,
    lifetime_k_wh_callback: extern "C" fn(*const c_void) -> u16,
    lifetime_k_ah_callback: extern "C" fn(*const c_void) -> u16,
    lifetime_maximum_output_wattage_callback: extern "C" fn(*const c_void) -> u16,
    lifetime_maximum_battery_voltage_callback: extern "C" fn(*const c_void) -> u16,
    lifetime_maximum_voc_voltage_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model64111CallbackAdapter {
    /// Port Number
    fn port_number(&self) -> u16 {
        (self.port_number_callback)(self.context)
    }

    fn v_sf(&self) -> u16 {
        (self.v_sf_callback)(self.context)
    }

    fn a_sf(&self) -> u16 {
        (self.a_sf_callback)(self.context)
    }

    fn p_sf(&self) -> u16 {
        (self.p_sf_callback)(self.context)
    }

    fn ah_sf(&self) -> u16 {
        (self.ah_sf_callback)(self.context)
    }

    fn kwh_sf(&self) -> u16 {
        (self.kwh_sf_callback)(self.context)
    }

    /// Battery Voltage
    fn battery_voltage(&self) -> u16 {
        (self.battery_voltage_callback)(self.context)
    }

    /// Array Voltage
    fn array_voltage(&self) -> u16 {
        (self.array_voltage_callback)(self.context)
    }

    /// Output Current
    fn output_current(&self) -> u16 {
        (self.output_current_callback)(self.context)
    }

    /// Array Current
    fn array_current(&self) -> u16 {
        (self.array_current_callback)(self.context)
    }

    /// Operating State
    fn operating_state(&self) -> ChargerSt {
        (self.operating_state_callback)(self.context)
    }

    /// Output Wattage
    fn output_wattage(&self) -> u16 {
        (self.output_wattage_callback)(self.context)
    }

    /// Today's Minimum Battery Voltage
    fn today_s_minimum_battery_voltage(&self) -> u16 {
        (self.today_s_minimum_battery_voltage_callback)(self.context)
    }

    /// Today's Maximum Battery Voltage
    fn today_s_maximum_battery_voltage(&self) -> u16 {
        (self.today_s_maximum_battery_voltage_callback)(self.context)
    }

    /// VOC
    fn voc(&self) -> u16 {
        (self.voc_callback)(self.context)
    }

    /// Today's Maximum VOC
    fn today_s_maximum_voc(&self) -> u16 {
        (self.today_s_maximum_voc_callback)(self.context)
    }

    /// Today's kWh
    fn today_s_k_wh(&self) -> u16 {
        (self.today_s_k_wh_callback)(self.context)
    }

    /// Today's AH
    fn today_s_ah(&self) -> u16 {
        (self.today_s_ah_callback)(self.context)
    }

    /// Lifetime kWh
    fn lifetime_k_wh(&self) -> u16 {
        (self.lifetime_k_wh_callback)(self.context)
    }

    /// Lifetime kAH
    fn lifetime_k_ah(&self) -> u16 {
        (self.lifetime_k_ah_callback)(self.context)
    }

    /// Lifetime Maximum Output Wattage
    fn lifetime_maximum_output_wattage(&self) -> u16 {
        (self.lifetime_maximum_output_wattage_callback)(self.context)
    }

    /// Lifetime Maximum Battery Voltage
    fn lifetime_maximum_battery_voltage(&self) -> u16 {
        (self.lifetime_maximum_battery_voltage_callback)(self.context)
    }

    /// Lifetime Maximum VOC Voltage
    fn lifetime_maximum_voc_voltage(&self) -> u16 {
        (self.lifetime_maximum_voc_voltage_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model64111StatefulAdapter {
    port_number: u16,
    v_sf: u16,
    a_sf: u16,
    p_sf: u16,
    ah_sf: u16,
    kwh_sf: u16,
    battery_voltage: u16,
    array_voltage: u16,
    output_current: u16,
    array_current: u16,
    operating_state: ChargerSt,
    output_wattage: u16,
    today_s_minimum_battery_voltage: u16,
    today_s_maximum_battery_voltage: u16,
    voc: u16,
    today_s_maximum_voc: u16,
    today_s_k_wh: u16,
    today_s_ah: u16,
    lifetime_k_wh: u16,
    lifetime_k_ah: u16,
    lifetime_maximum_output_wattage: u16,
    lifetime_maximum_battery_voltage: u16,
    lifetime_maximum_voc_voltage: u16,
}

impl ModelAdapter for Model64111StatefulAdapter {
    /// Port Number
    fn port_number(&self) -> u16 {
        self.port_number
    }

    fn v_sf(&self) -> u16 {
        self.v_sf
    }

    fn a_sf(&self) -> u16 {
        self.a_sf
    }

    fn p_sf(&self) -> u16 {
        self.p_sf
    }

    fn ah_sf(&self) -> u16 {
        self.ah_sf
    }

    fn kwh_sf(&self) -> u16 {
        self.kwh_sf
    }

    /// Battery Voltage
    fn battery_voltage(&self) -> u16 {
        self.battery_voltage
    }

    /// Array Voltage
    fn array_voltage(&self) -> u16 {
        self.array_voltage
    }

    /// Output Current
    fn output_current(&self) -> u16 {
        self.output_current
    }

    /// Array Current
    fn array_current(&self) -> u16 {
        self.array_current
    }

    /// Operating State
    fn operating_state(&self) -> ChargerSt {
        self.operating_state
    }

    /// Output Wattage
    fn output_wattage(&self) -> u16 {
        self.output_wattage
    }

    /// Today's Minimum Battery Voltage
    fn today_s_minimum_battery_voltage(&self) -> u16 {
        self.today_s_minimum_battery_voltage
    }

    /// Today's Maximum Battery Voltage
    fn today_s_maximum_battery_voltage(&self) -> u16 {
        self.today_s_maximum_battery_voltage
    }

    /// VOC
    fn voc(&self) -> u16 {
        self.voc
    }

    /// Today's Maximum VOC
    fn today_s_maximum_voc(&self) -> u16 {
        self.today_s_maximum_voc
    }

    /// Today's kWh
    fn today_s_k_wh(&self) -> u16 {
        self.today_s_k_wh
    }

    /// Today's AH
    fn today_s_ah(&self) -> u16 {
        self.today_s_ah
    }

    /// Lifetime kWh
    fn lifetime_k_wh(&self) -> u16 {
        self.lifetime_k_wh
    }

    /// Lifetime kAH
    fn lifetime_k_ah(&self) -> u16 {
        self.lifetime_k_ah
    }

    /// Lifetime Maximum Output Wattage
    fn lifetime_maximum_output_wattage(&self) -> u16 {
        self.lifetime_maximum_output_wattage
    }

    /// Lifetime Maximum Battery Voltage
    fn lifetime_maximum_battery_voltage(&self) -> u16 {
        self.lifetime_maximum_battery_voltage
    }

    /// Lifetime Maximum VOC Voltage
    fn lifetime_maximum_voc_voltage(&self) -> u16 {
        self.lifetime_maximum_voc_voltage
    }
}
