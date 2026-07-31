use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 73;

pub static POINTS: [ReadablePoint; 37] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64001 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 71 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::CommandCode,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::HardwareRevision,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::RsFwRevision,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::OsFwRevision,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::ProductRevision,
        },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::BootCount,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::DipSwitches,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::NumDetectedSensors,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::NumCommunicatingSensors,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::SystemStatus,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::SystemConfiguration,
        },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::LedBlinkThreshold,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::LedOnThreshold,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Reserved,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::LocationString,
        },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor1UnitId,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor1Address,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor1OsVersion,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor1ProductVersion,
        },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor1SerialNum,
        },
        size: 5,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor2UnitId,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor2Address,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor2OsVersion,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor2ProductVersion,
        },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor2SerialNum,
        },
        size: 5,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor3UnitId,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor3Address,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor3OsVersion,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor3ProductVersion,
        },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor3SerialNum,
        },
        size: 5,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor4UnitId,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor4Address,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor4OsVersion,
        },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor4ProductVersion,
        },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 {
            point: Point::Sensor4SerialNum,
        },
        size: 5,
        data_type: PointType::String,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    CommandCode,
    HardwareRevision,
    RsFwRevision,
    OsFwRevision,
    ProductRevision,
    BootCount,
    DipSwitches,
    NumDetectedSensors,
    NumCommunicatingSensors,
    SystemStatus,
    SystemConfiguration,
    LedBlinkThreshold,
    LedOnThreshold,
    Reserved,
    LocationString,
    Sensor1UnitId,
    Sensor1Address,
    Sensor1OsVersion,
    Sensor1ProductVersion,
    Sensor1SerialNum,
    Sensor2UnitId,
    Sensor2Address,
    Sensor2OsVersion,
    Sensor2ProductVersion,
    Sensor2SerialNum,
    Sensor3UnitId,
    Sensor3Address,
    Sensor3OsVersion,
    Sensor3ProductVersion,
    Sensor3SerialNum,
    Sensor4UnitId,
    Sensor4Address,
    Sensor4OsVersion,
    Sensor4ProductVersion,
    Sensor4SerialNum,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::CommandCode => {
            if let Some(value) = model.command_code() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::HardwareRevision => {
            if let Some(value) = model.hardware_revision() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::RsFwRevision => {
            if let Some(value) = model.rs_fw_revision() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::OsFwRevision => {
            if let Some(value) = model.os_fw_revision() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::ProductRevision => {
            if let Some(value) = model.product_revision() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::BootCount => {
            if let Some(value) = model.boot_count() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::DipSwitches => {
            if let Some(value) = model.dip_switches() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::NumDetectedSensors => {
            if let Some(value) = model.num_detected_sensors() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::NumCommunicatingSensors => {
            if let Some(value) = model.num_communicating_sensors() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::SystemStatus => {
            if let Some(value) = model.system_status() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::SystemConfiguration => {
            if let Some(value) = model.system_configuration() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::LedBlinkThreshold => {
            if let Some(value) = model.led_blink_threshold() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::LedOnThreshold => {
            if let Some(value) = model.led_on_threshold() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Reserved => {
            if let Some(value) = model.reserved() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::LocationString => {
            if let Some(value) = model.location_string() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Sensor1UnitId => {
            if let Some(value) = model.sensor_1_unit_id() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor1Address => {
            if let Some(value) = model.sensor_1_address() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor1OsVersion => {
            if let Some(value) = model.sensor_1_os_version() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor1ProductVersion => {
            if let Some(value) = model.sensor_1_product_version() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Sensor1SerialNum => {
            if let Some(value) = model.sensor_1_serial_num() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Sensor2UnitId => {
            if let Some(value) = model.sensor_2_unit_id() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor2Address => {
            if let Some(value) = model.sensor_2_address() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor2OsVersion => {
            if let Some(value) = model.sensor_2_os_version() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor2ProductVersion => {
            if let Some(value) = model.sensor_2_product_version() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Sensor2SerialNum => {
            if let Some(value) = model.sensor_2_serial_num() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Sensor3UnitId => {
            if let Some(value) = model.sensor_3_unit_id() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor3Address => {
            if let Some(value) = model.sensor_3_address() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor3OsVersion => {
            if let Some(value) = model.sensor_3_os_version() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor3ProductVersion => {
            if let Some(value) = model.sensor_3_product_version() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Sensor3SerialNum => {
            if let Some(value) = model.sensor_3_serial_num() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Sensor4UnitId => {
            if let Some(value) = model.sensor_4_unit_id() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor4Address => {
            if let Some(value) = model.sensor_4_address() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor4OsVersion => {
            if let Some(value) = model.sensor_4_os_version() {
                serialisation::write_u16(value, buffer);
            }
        }
        Point::Sensor4ProductVersion => {
            if let Some(value) = model.sensor_4_product_version() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::Sensor4SerialNum => {
            if let Some(value) = model.sensor_4_serial_num() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
    }
}

pub trait ModelAdapter {
    /// Command Code
    fn command_code(&self) -> Option<u16> {
        None
    }

    /// Command Code
    fn set_command_code(&mut self, value: u16) {}

    /// Hardware Revision
    fn hardware_revision(&self) -> Option<u16> {
        None
    }

    /// RS FW Revision
    fn rs_fw_revision(&self) -> Option<u16> {
        None
    }

    /// OS FW Revision
    fn os_fw_revision(&self) -> Option<u16> {
        None
    }

    /// Product Revision
    fn product_revision(&self) -> Option<&CStr> {
        None
    }

    /// Boot Count
    fn boot_count(&self) -> Option<u16> {
        None
    }

    /// DIP Switches
    fn dip_switches(&self) -> Option<u16> {
        None
    }

    /// Num Detected Sensors
    fn num_detected_sensors(&self) -> Option<u16> {
        None
    }

    /// Num Communicating Sensors
    fn num_communicating_sensors(&self) -> Option<u16> {
        None
    }

    /// System Status
    fn system_status(&self) -> Option<u16> {
        None
    }

    /// System Configuration
    fn system_configuration(&self) -> Option<u16> {
        None
    }

    /// LED Blink Threshold
    fn led_blink_threshold(&self) -> Option<u16> {
        None
    }

    /// LED On Threshold
    fn led_on_threshold(&self) -> Option<u16> {
        None
    }

    fn reserved(&self) -> Option<u16> {
        None
    }

    /// Location String
    fn location_string(&self) -> Option<&CStr> {
        None
    }

    /// Sensor 1 Unit ID
    fn sensor_1_unit_id(&self) -> Option<u16> {
        None
    }

    /// Sensor 1 Address
    fn sensor_1_address(&self) -> Option<u16> {
        None
    }

    /// Sensor 1 OS Version
    fn sensor_1_os_version(&self) -> Option<u16> {
        None
    }

    /// Sensor 1 Product Version
    fn sensor_1_product_version(&self) -> Option<&CStr> {
        None
    }

    /// Sensor 1 Serial Num
    fn sensor_1_serial_num(&self) -> Option<&CStr> {
        None
    }

    /// Sensor 2 Unit ID
    fn sensor_2_unit_id(&self) -> Option<u16> {
        None
    }

    /// Sensor 2 Address
    fn sensor_2_address(&self) -> Option<u16> {
        None
    }

    /// Sensor 2 OS Version
    fn sensor_2_os_version(&self) -> Option<u16> {
        None
    }

    /// Sensor 2 Product Version
    fn sensor_2_product_version(&self) -> Option<&CStr> {
        None
    }

    /// Sensor 2 Serial Num
    fn sensor_2_serial_num(&self) -> Option<&CStr> {
        None
    }

    /// Sensor 3 Unit ID
    fn sensor_3_unit_id(&self) -> Option<u16> {
        None
    }

    /// Sensor 3 Address
    fn sensor_3_address(&self) -> Option<u16> {
        None
    }

    /// Sensor 3 OS Version
    fn sensor_3_os_version(&self) -> Option<u16> {
        None
    }

    /// Sensor 3 Product Version
    fn sensor_3_product_version(&self) -> Option<&CStr> {
        None
    }

    /// Sensor 3 Serial Num
    fn sensor_3_serial_num(&self) -> Option<&CStr> {
        None
    }

    /// Sensor 4 Unit ID
    fn sensor_4_unit_id(&self) -> Option<u16> {
        None
    }

    /// Sensor 4 Address
    fn sensor_4_address(&self) -> Option<u16> {
        None
    }

    /// Sensor 4 OS Version
    fn sensor_4_os_version(&self) -> Option<u16> {
        None
    }

    /// Sensor 4 Product Version
    fn sensor_4_product_version(&self) -> Option<&CStr> {
        None
    }

    /// Sensor 4 Serial Num
    fn sensor_4_serial_num(&self) -> Option<&CStr> {
        None
    }
}

#[repr(C)]
pub struct Model64001CallbackAdapter {
    command_code_callback: Option<extern "C" fn() -> u16>,
    set_command_code_callback: Option<extern "C" fn(u16)>,
    hardware_revision_callback: Option<extern "C" fn() -> u16>,
    rs_fw_revision_callback: Option<extern "C" fn() -> u16>,
    os_fw_revision_callback: Option<extern "C" fn() -> u16>,
    product_revision_callback: Option<extern "C" fn() -> *const c_char>,
    boot_count_callback: Option<extern "C" fn() -> u16>,
    dip_switches_callback: Option<extern "C" fn() -> u16>,
    num_detected_sensors_callback: Option<extern "C" fn() -> u16>,
    num_communicating_sensors_callback: Option<extern "C" fn() -> u16>,
    system_status_callback: Option<extern "C" fn() -> u16>,
    system_configuration_callback: Option<extern "C" fn() -> u16>,
    led_blink_threshold_callback: Option<extern "C" fn() -> u16>,
    led_on_threshold_callback: Option<extern "C" fn() -> u16>,
    reserved_callback: Option<extern "C" fn() -> u16>,
    location_string_callback: Option<extern "C" fn() -> *const c_char>,
    sensor_1_unit_id_callback: Option<extern "C" fn() -> u16>,
    sensor_1_address_callback: Option<extern "C" fn() -> u16>,
    sensor_1_os_version_callback: Option<extern "C" fn() -> u16>,
    sensor_1_product_version_callback: Option<extern "C" fn() -> *const c_char>,
    sensor_1_serial_num_callback: Option<extern "C" fn() -> *const c_char>,
    sensor_2_unit_id_callback: Option<extern "C" fn() -> u16>,
    sensor_2_address_callback: Option<extern "C" fn() -> u16>,
    sensor_2_os_version_callback: Option<extern "C" fn() -> u16>,
    sensor_2_product_version_callback: Option<extern "C" fn() -> *const c_char>,
    sensor_2_serial_num_callback: Option<extern "C" fn() -> *const c_char>,
    sensor_3_unit_id_callback: Option<extern "C" fn() -> u16>,
    sensor_3_address_callback: Option<extern "C" fn() -> u16>,
    sensor_3_os_version_callback: Option<extern "C" fn() -> u16>,
    sensor_3_product_version_callback: Option<extern "C" fn() -> *const c_char>,
    sensor_3_serial_num_callback: Option<extern "C" fn() -> *const c_char>,
    sensor_4_unit_id_callback: Option<extern "C" fn() -> u16>,
    sensor_4_address_callback: Option<extern "C" fn() -> u16>,
    sensor_4_os_version_callback: Option<extern "C" fn() -> u16>,
    sensor_4_product_version_callback: Option<extern "C" fn() -> *const c_char>,
    sensor_4_serial_num_callback: Option<extern "C" fn() -> *const c_char>,
}

impl ModelAdapter for Model64001CallbackAdapter {
    /// Command Code
    fn command_code(&self) -> Option<u16> {
        self.command_code_callback.map(|callback| (callback)())
    }

    /// Command Code
    fn set_command_code(&mut self, value: u16) {
        if let Some(callback) = self.set_command_code_callback {
            (callback)(value);
        };
    }

    /// Hardware Revision
    fn hardware_revision(&self) -> Option<u16> {
        self.hardware_revision_callback.map(|callback| (callback)())
    }

    /// RS FW Revision
    fn rs_fw_revision(&self) -> Option<u16> {
        self.rs_fw_revision_callback.map(|callback| (callback)())
    }

    /// OS FW Revision
    fn os_fw_revision(&self) -> Option<u16> {
        self.os_fw_revision_callback.map(|callback| (callback)())
    }

    /// Product Revision
    fn product_revision(&self) -> Option<&CStr> {
        self.product_revision_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Boot Count
    fn boot_count(&self) -> Option<u16> {
        self.boot_count_callback.map(|callback| (callback)())
    }

    /// DIP Switches
    fn dip_switches(&self) -> Option<u16> {
        self.dip_switches_callback.map(|callback| (callback)())
    }

    /// Num Detected Sensors
    fn num_detected_sensors(&self) -> Option<u16> {
        self.num_detected_sensors_callback
            .map(|callback| (callback)())
    }

    /// Num Communicating Sensors
    fn num_communicating_sensors(&self) -> Option<u16> {
        self.num_communicating_sensors_callback
            .map(|callback| (callback)())
    }

    /// System Status
    fn system_status(&self) -> Option<u16> {
        self.system_status_callback.map(|callback| (callback)())
    }

    /// System Configuration
    fn system_configuration(&self) -> Option<u16> {
        self.system_configuration_callback
            .map(|callback| (callback)())
    }

    /// LED Blink Threshold
    fn led_blink_threshold(&self) -> Option<u16> {
        self.led_blink_threshold_callback
            .map(|callback| (callback)())
    }

    /// LED On Threshold
    fn led_on_threshold(&self) -> Option<u16> {
        self.led_on_threshold_callback.map(|callback| (callback)())
    }

    fn reserved(&self) -> Option<u16> {
        self.reserved_callback.map(|callback| (callback)())
    }

    /// Location String
    fn location_string(&self) -> Option<&CStr> {
        self.location_string_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Sensor 1 Unit ID
    fn sensor_1_unit_id(&self) -> Option<u16> {
        self.sensor_1_unit_id_callback.map(|callback| (callback)())
    }

    /// Sensor 1 Address
    fn sensor_1_address(&self) -> Option<u16> {
        self.sensor_1_address_callback.map(|callback| (callback)())
    }

    /// Sensor 1 OS Version
    fn sensor_1_os_version(&self) -> Option<u16> {
        self.sensor_1_os_version_callback
            .map(|callback| (callback)())
    }

    /// Sensor 1 Product Version
    fn sensor_1_product_version(&self) -> Option<&CStr> {
        self.sensor_1_product_version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Sensor 1 Serial Num
    fn sensor_1_serial_num(&self) -> Option<&CStr> {
        self.sensor_1_serial_num_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Sensor 2 Unit ID
    fn sensor_2_unit_id(&self) -> Option<u16> {
        self.sensor_2_unit_id_callback.map(|callback| (callback)())
    }

    /// Sensor 2 Address
    fn sensor_2_address(&self) -> Option<u16> {
        self.sensor_2_address_callback.map(|callback| (callback)())
    }

    /// Sensor 2 OS Version
    fn sensor_2_os_version(&self) -> Option<u16> {
        self.sensor_2_os_version_callback
            .map(|callback| (callback)())
    }

    /// Sensor 2 Product Version
    fn sensor_2_product_version(&self) -> Option<&CStr> {
        self.sensor_2_product_version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Sensor 2 Serial Num
    fn sensor_2_serial_num(&self) -> Option<&CStr> {
        self.sensor_2_serial_num_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Sensor 3 Unit ID
    fn sensor_3_unit_id(&self) -> Option<u16> {
        self.sensor_3_unit_id_callback.map(|callback| (callback)())
    }

    /// Sensor 3 Address
    fn sensor_3_address(&self) -> Option<u16> {
        self.sensor_3_address_callback.map(|callback| (callback)())
    }

    /// Sensor 3 OS Version
    fn sensor_3_os_version(&self) -> Option<u16> {
        self.sensor_3_os_version_callback
            .map(|callback| (callback)())
    }

    /// Sensor 3 Product Version
    fn sensor_3_product_version(&self) -> Option<&CStr> {
        self.sensor_3_product_version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Sensor 3 Serial Num
    fn sensor_3_serial_num(&self) -> Option<&CStr> {
        self.sensor_3_serial_num_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Sensor 4 Unit ID
    fn sensor_4_unit_id(&self) -> Option<u16> {
        self.sensor_4_unit_id_callback.map(|callback| (callback)())
    }

    /// Sensor 4 Address
    fn sensor_4_address(&self) -> Option<u16> {
        self.sensor_4_address_callback.map(|callback| (callback)())
    }

    /// Sensor 4 OS Version
    fn sensor_4_os_version(&self) -> Option<u16> {
        self.sensor_4_os_version_callback
            .map(|callback| (callback)())
    }

    /// Sensor 4 Product Version
    fn sensor_4_product_version(&self) -> Option<&CStr> {
        self.sensor_4_product_version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Sensor 4 Serial Num
    fn sensor_4_serial_num(&self) -> Option<&CStr> {
        self.sensor_4_serial_num_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }
}
