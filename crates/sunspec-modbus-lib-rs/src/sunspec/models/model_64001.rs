use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::{CStr, c_char, c_void};

pub const SIZE: u16 = 73;

static POINTS: [PointDetails<()>; 37] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::CommandCode,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::HardwareRevision,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::RsFwRevision,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::OsFwRevision,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::ProductRevision,
        size: 2,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::BootCount,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::DipSwitches,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::NumDetectedSensors,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::NumCommunicatingSensors,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::SystemStatus,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::SystemConfiguration,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::LedBlinkThreshold,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::LedOnThreshold,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::Reserved,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::LocationString,
        size: 16,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::Sensor1UnitId,
        size: 1,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::Sensor1Address,
        size: 1,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::Sensor1OsVersion,
        size: 1,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::Sensor1ProductVersion,
        size: 2,
        start_address: 36,
    },
    PointDetails {
        point: |()| Point::Sensor1SerialNum,
        size: 5,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::Sensor2UnitId,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::Sensor2Address,
        size: 1,
        start_address: 44,
    },
    PointDetails {
        point: |()| Point::Sensor2OsVersion,
        size: 1,
        start_address: 45,
    },
    PointDetails {
        point: |()| Point::Sensor2ProductVersion,
        size: 2,
        start_address: 46,
    },
    PointDetails {
        point: |()| Point::Sensor2SerialNum,
        size: 5,
        start_address: 48,
    },
    PointDetails {
        point: |()| Point::Sensor3UnitId,
        size: 1,
        start_address: 53,
    },
    PointDetails {
        point: |()| Point::Sensor3Address,
        size: 1,
        start_address: 54,
    },
    PointDetails {
        point: |()| Point::Sensor3OsVersion,
        size: 1,
        start_address: 55,
    },
    PointDetails {
        point: |()| Point::Sensor3ProductVersion,
        size: 2,
        start_address: 56,
    },
    PointDetails {
        point: |()| Point::Sensor3SerialNum,
        size: 5,
        start_address: 58,
    },
    PointDetails {
        point: |()| Point::Sensor4UnitId,
        size: 1,
        start_address: 63,
    },
    PointDetails {
        point: |()| Point::Sensor4Address,
        size: 1,
        start_address: 64,
    },
    PointDetails {
        point: |()| Point::Sensor4OsVersion,
        size: 1,
        start_address: 65,
    },
    PointDetails {
        point: |()| Point::Sensor4ProductVersion,
        size: 2,
        start_address: 66,
    },
    PointDetails {
        point: |()| Point::Sensor4SerialNum,
        size: 5,
        start_address: 68,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
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

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    73
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(64001, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::CommandCode => {
            if let Some(value) = model.command_code() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::HardwareRevision => {
            if let Some(value) = model.hardware_revision() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::RsFwRevision => {
            if let Some(value) = model.rs_fw_revision() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::OsFwRevision => {
            if let Some(value) = model.os_fw_revision() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ProductRevision => {
            if let Some(value) = model.product_revision() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::BootCount => {
            if let Some(value) = model.boot_count() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::DipSwitches => {
            if let Some(value) = model.dip_switches() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NumDetectedSensors => {
            if let Some(value) = model.num_detected_sensors() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NumCommunicatingSensors => {
            if let Some(value) = model.num_communicating_sensors() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SystemStatus => {
            if let Some(value) = model.system_status() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SystemConfiguration => {
            if let Some(value) = model.system_configuration() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::LedBlinkThreshold => {
            if let Some(value) = model.led_blink_threshold() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::LedOnThreshold => {
            if let Some(value) = model.led_on_threshold() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Reserved => {
            if let Some(value) = model.reserved() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::LocationString => {
            if let Some(value) = model.location_string() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 16);
            }
        }
        Point::Sensor1UnitId => {
            if let Some(value) = model.sensor_1_unit_id() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor1Address => {
            if let Some(value) = model.sensor_1_address() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor1OsVersion => {
            if let Some(value) = model.sensor_1_os_version() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor1ProductVersion => {
            if let Some(value) = model.sensor_1_product_version() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Sensor1SerialNum => {
            if let Some(value) = model.sensor_1_serial_num() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 5);
            }
        }
        Point::Sensor2UnitId => {
            if let Some(value) = model.sensor_2_unit_id() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor2Address => {
            if let Some(value) = model.sensor_2_address() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor2OsVersion => {
            if let Some(value) = model.sensor_2_os_version() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor2ProductVersion => {
            if let Some(value) = model.sensor_2_product_version() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Sensor2SerialNum => {
            if let Some(value) = model.sensor_2_serial_num() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 5);
            }
        }
        Point::Sensor3UnitId => {
            if let Some(value) = model.sensor_3_unit_id() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor3Address => {
            if let Some(value) = model.sensor_3_address() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor3OsVersion => {
            if let Some(value) = model.sensor_3_os_version() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor3ProductVersion => {
            if let Some(value) = model.sensor_3_product_version() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Sensor3SerialNum => {
            if let Some(value) = model.sensor_3_serial_num() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 5);
            }
        }
        Point::Sensor4UnitId => {
            if let Some(value) = model.sensor_4_unit_id() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor4Address => {
            if let Some(value) = model.sensor_4_address() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor4OsVersion => {
            if let Some(value) = model.sensor_4_os_version() {
                buffer::write_u16(value, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::Sensor4ProductVersion => {
            if let Some(value) = model.sensor_4_product_version() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 2);
            }
        }
        Point::Sensor4SerialNum => {
            if let Some(value) = model.sensor_4_serial_num() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, 5);
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
    context: *mut c_void,
    command_code_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_command_code_callback: Option<extern "C" fn(u16, *mut c_void)>,
    hardware_revision_callback: Option<extern "C" fn(*const c_void) -> u16>,
    rs_fw_revision_callback: Option<extern "C" fn(*const c_void) -> u16>,
    os_fw_revision_callback: Option<extern "C" fn(*const c_void) -> u16>,
    product_revision_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    boot_count_callback: Option<extern "C" fn(*const c_void) -> u16>,
    dip_switches_callback: Option<extern "C" fn(*const c_void) -> u16>,
    num_detected_sensors_callback: Option<extern "C" fn(*const c_void) -> u16>,
    num_communicating_sensors_callback: Option<extern "C" fn(*const c_void) -> u16>,
    system_status_callback: Option<extern "C" fn(*const c_void) -> u16>,
    system_configuration_callback: Option<extern "C" fn(*const c_void) -> u16>,
    led_blink_threshold_callback: Option<extern "C" fn(*const c_void) -> u16>,
    led_on_threshold_callback: Option<extern "C" fn(*const c_void) -> u16>,
    reserved_callback: Option<extern "C" fn(*const c_void) -> u16>,
    location_string_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sensor_1_unit_id_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_1_address_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_1_os_version_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_1_product_version_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sensor_1_serial_num_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sensor_2_unit_id_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_2_address_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_2_os_version_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_2_product_version_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sensor_2_serial_num_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sensor_3_unit_id_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_3_address_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_3_os_version_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_3_product_version_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sensor_3_serial_num_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sensor_4_unit_id_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_4_address_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_4_os_version_callback: Option<extern "C" fn(*const c_void) -> u16>,
    sensor_4_product_version_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    sensor_4_serial_num_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
}

impl ModelAdapter for Model64001CallbackAdapter {
    /// Command Code
    fn command_code(&self) -> Option<u16> {
        self.command_code_callback
            .map(|callback| (callback)(self.context))
    }

    /// Command Code
    fn set_command_code(&mut self, value: u16) {
        if let Some(callback) = self.set_command_code_callback {
            (callback)(value, self.context);
        };
    }

    /// Hardware Revision
    fn hardware_revision(&self) -> Option<u16> {
        self.hardware_revision_callback
            .map(|callback| (callback)(self.context))
    }

    /// RS FW Revision
    fn rs_fw_revision(&self) -> Option<u16> {
        self.rs_fw_revision_callback
            .map(|callback| (callback)(self.context))
    }

    /// OS FW Revision
    fn os_fw_revision(&self) -> Option<u16> {
        self.os_fw_revision_callback
            .map(|callback| (callback)(self.context))
    }

    /// Product Revision
    fn product_revision(&self) -> Option<&CStr> {
        self.product_revision_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Boot Count
    fn boot_count(&self) -> Option<u16> {
        self.boot_count_callback
            .map(|callback| (callback)(self.context))
    }

    /// DIP Switches
    fn dip_switches(&self) -> Option<u16> {
        self.dip_switches_callback
            .map(|callback| (callback)(self.context))
    }

    /// Num Detected Sensors
    fn num_detected_sensors(&self) -> Option<u16> {
        self.num_detected_sensors_callback
            .map(|callback| (callback)(self.context))
    }

    /// Num Communicating Sensors
    fn num_communicating_sensors(&self) -> Option<u16> {
        self.num_communicating_sensors_callback
            .map(|callback| (callback)(self.context))
    }

    /// System Status
    fn system_status(&self) -> Option<u16> {
        self.system_status_callback
            .map(|callback| (callback)(self.context))
    }

    /// System Configuration
    fn system_configuration(&self) -> Option<u16> {
        self.system_configuration_callback
            .map(|callback| (callback)(self.context))
    }

    /// LED Blink Threshold
    fn led_blink_threshold(&self) -> Option<u16> {
        self.led_blink_threshold_callback
            .map(|callback| (callback)(self.context))
    }

    /// LED On Threshold
    fn led_on_threshold(&self) -> Option<u16> {
        self.led_on_threshold_callback
            .map(|callback| (callback)(self.context))
    }

    fn reserved(&self) -> Option<u16> {
        self.reserved_callback
            .map(|callback| (callback)(self.context))
    }

    /// Location String
    fn location_string(&self) -> Option<&CStr> {
        self.location_string_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Sensor 1 Unit ID
    fn sensor_1_unit_id(&self) -> Option<u16> {
        self.sensor_1_unit_id_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 1 Address
    fn sensor_1_address(&self) -> Option<u16> {
        self.sensor_1_address_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 1 OS Version
    fn sensor_1_os_version(&self) -> Option<u16> {
        self.sensor_1_os_version_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 1 Product Version
    fn sensor_1_product_version(&self) -> Option<&CStr> {
        self.sensor_1_product_version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Sensor 1 Serial Num
    fn sensor_1_serial_num(&self) -> Option<&CStr> {
        self.sensor_1_serial_num_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Sensor 2 Unit ID
    fn sensor_2_unit_id(&self) -> Option<u16> {
        self.sensor_2_unit_id_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 2 Address
    fn sensor_2_address(&self) -> Option<u16> {
        self.sensor_2_address_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 2 OS Version
    fn sensor_2_os_version(&self) -> Option<u16> {
        self.sensor_2_os_version_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 2 Product Version
    fn sensor_2_product_version(&self) -> Option<&CStr> {
        self.sensor_2_product_version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Sensor 2 Serial Num
    fn sensor_2_serial_num(&self) -> Option<&CStr> {
        self.sensor_2_serial_num_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Sensor 3 Unit ID
    fn sensor_3_unit_id(&self) -> Option<u16> {
        self.sensor_3_unit_id_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 3 Address
    fn sensor_3_address(&self) -> Option<u16> {
        self.sensor_3_address_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 3 OS Version
    fn sensor_3_os_version(&self) -> Option<u16> {
        self.sensor_3_os_version_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 3 Product Version
    fn sensor_3_product_version(&self) -> Option<&CStr> {
        self.sensor_3_product_version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Sensor 3 Serial Num
    fn sensor_3_serial_num(&self) -> Option<&CStr> {
        self.sensor_3_serial_num_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Sensor 4 Unit ID
    fn sensor_4_unit_id(&self) -> Option<u16> {
        self.sensor_4_unit_id_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 4 Address
    fn sensor_4_address(&self) -> Option<u16> {
        self.sensor_4_address_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 4 OS Version
    fn sensor_4_os_version(&self) -> Option<u16> {
        self.sensor_4_os_version_callback
            .map(|callback| (callback)(self.context))
    }

    /// Sensor 4 Product Version
    fn sensor_4_product_version(&self) -> Option<&CStr> {
        self.sensor_4_product_version_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Sensor 4 Serial Num
    fn sensor_4_serial_num(&self) -> Option<&CStr> {
        self.sensor_4_serial_num_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }
}

#[repr(C)]
pub struct Model64001StatefulAdapter {
    command_code: u16,
    hardware_revision: u16,
    rs_fw_revision: u16,
    os_fw_revision: u16,
    product_revision: [c_char; 4],
    boot_count: u16,
    dip_switches: u16,
    num_detected_sensors: u16,
    num_communicating_sensors: u16,
    system_status: u16,
    system_configuration: u16,
    led_blink_threshold: u16,
    led_on_threshold: u16,
    reserved: u16,
    location_string: [c_char; 32],
    sensor_1_unit_id: u16,
    sensor_1_address: u16,
    sensor_1_os_version: u16,
    sensor_1_product_version: [c_char; 4],
    sensor_1_serial_num: [c_char; 10],
    sensor_2_unit_id: u16,
    sensor_2_address: u16,
    sensor_2_os_version: u16,
    sensor_2_product_version: [c_char; 4],
    sensor_2_serial_num: [c_char; 10],
    sensor_3_unit_id: u16,
    sensor_3_address: u16,
    sensor_3_os_version: u16,
    sensor_3_product_version: [c_char; 4],
    sensor_3_serial_num: [c_char; 10],
    sensor_4_unit_id: u16,
    sensor_4_address: u16,
    sensor_4_os_version: u16,
    sensor_4_product_version: [c_char; 4],
    sensor_4_serial_num: [c_char; 10],
}

impl ModelAdapter for Model64001StatefulAdapter {
    /// Command Code
    fn command_code(&self) -> Option<u16> {
        Some(self.command_code)
    }

    /// Command Code
    fn set_command_code(&mut self, value: u16) {
        self.command_code = value;
    }

    /// Hardware Revision
    fn hardware_revision(&self) -> Option<u16> {
        Some(self.hardware_revision)
    }

    /// RS FW Revision
    fn rs_fw_revision(&self) -> Option<u16> {
        Some(self.rs_fw_revision)
    }

    /// OS FW Revision
    fn os_fw_revision(&self) -> Option<u16> {
        Some(self.os_fw_revision)
    }

    /// Product Revision
    fn product_revision(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.product_revision.as_ptr()) })
    }

    /// Boot Count
    fn boot_count(&self) -> Option<u16> {
        Some(self.boot_count)
    }

    /// DIP Switches
    fn dip_switches(&self) -> Option<u16> {
        Some(self.dip_switches)
    }

    /// Num Detected Sensors
    fn num_detected_sensors(&self) -> Option<u16> {
        Some(self.num_detected_sensors)
    }

    /// Num Communicating Sensors
    fn num_communicating_sensors(&self) -> Option<u16> {
        Some(self.num_communicating_sensors)
    }

    /// System Status
    fn system_status(&self) -> Option<u16> {
        Some(self.system_status)
    }

    /// System Configuration
    fn system_configuration(&self) -> Option<u16> {
        Some(self.system_configuration)
    }

    /// LED Blink Threshold
    fn led_blink_threshold(&self) -> Option<u16> {
        Some(self.led_blink_threshold)
    }

    /// LED On Threshold
    fn led_on_threshold(&self) -> Option<u16> {
        Some(self.led_on_threshold)
    }

    fn reserved(&self) -> Option<u16> {
        Some(self.reserved)
    }

    /// Location String
    fn location_string(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.location_string.as_ptr()) })
    }

    /// Sensor 1 Unit ID
    fn sensor_1_unit_id(&self) -> Option<u16> {
        Some(self.sensor_1_unit_id)
    }

    /// Sensor 1 Address
    fn sensor_1_address(&self) -> Option<u16> {
        Some(self.sensor_1_address)
    }

    /// Sensor 1 OS Version
    fn sensor_1_os_version(&self) -> Option<u16> {
        Some(self.sensor_1_os_version)
    }

    /// Sensor 1 Product Version
    fn sensor_1_product_version(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.sensor_1_product_version.as_ptr()) })
    }

    /// Sensor 1 Serial Num
    fn sensor_1_serial_num(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.sensor_1_serial_num.as_ptr()) })
    }

    /// Sensor 2 Unit ID
    fn sensor_2_unit_id(&self) -> Option<u16> {
        Some(self.sensor_2_unit_id)
    }

    /// Sensor 2 Address
    fn sensor_2_address(&self) -> Option<u16> {
        Some(self.sensor_2_address)
    }

    /// Sensor 2 OS Version
    fn sensor_2_os_version(&self) -> Option<u16> {
        Some(self.sensor_2_os_version)
    }

    /// Sensor 2 Product Version
    fn sensor_2_product_version(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.sensor_2_product_version.as_ptr()) })
    }

    /// Sensor 2 Serial Num
    fn sensor_2_serial_num(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.sensor_2_serial_num.as_ptr()) })
    }

    /// Sensor 3 Unit ID
    fn sensor_3_unit_id(&self) -> Option<u16> {
        Some(self.sensor_3_unit_id)
    }

    /// Sensor 3 Address
    fn sensor_3_address(&self) -> Option<u16> {
        Some(self.sensor_3_address)
    }

    /// Sensor 3 OS Version
    fn sensor_3_os_version(&self) -> Option<u16> {
        Some(self.sensor_3_os_version)
    }

    /// Sensor 3 Product Version
    fn sensor_3_product_version(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.sensor_3_product_version.as_ptr()) })
    }

    /// Sensor 3 Serial Num
    fn sensor_3_serial_num(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.sensor_3_serial_num.as_ptr()) })
    }

    /// Sensor 4 Unit ID
    fn sensor_4_unit_id(&self) -> Option<u16> {
        Some(self.sensor_4_unit_id)
    }

    /// Sensor 4 Address
    fn sensor_4_address(&self) -> Option<u16> {
        Some(self.sensor_4_address)
    }

    /// Sensor 4 OS Version
    fn sensor_4_os_version(&self) -> Option<u16> {
        Some(self.sensor_4_os_version)
    }

    /// Sensor 4 Product Version
    fn sensor_4_product_version(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.sensor_4_product_version.as_ptr()) })
    }

    /// Sensor 4 Serial Num
    fn sensor_4_serial_num(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.sensor_4_serial_num.as_ptr()) })
    }
}
