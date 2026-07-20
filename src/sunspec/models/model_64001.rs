use heapless::String;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

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
        reference: PointReference::Model64001 { point: Point::CommandCode },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::HardwareRevision },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::RsFwRevision },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::OsFwRevision },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::ProductRevision },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::BootCount },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::DipSwitches },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::NumDetectedSensors },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::NumCommunicatingSensors },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::SystemStatus },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::SystemConfiguration },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::LedBlinkThreshold },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::LedOnThreshold },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Reserved },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::LocationString },
        size: 16,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor1UnitId },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor1Address },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor1OsVersion },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor1ProductVersion },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor1SerialNum },
        size: 5,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor2UnitId },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor2Address },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor2OsVersion },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor2ProductVersion },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor2SerialNum },
        size: 5,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor3UnitId },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor3Address },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor3OsVersion },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor3ProductVersion },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor3SerialNum },
        size: 5,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor4UnitId },
        size: 1,
        data_type: PointType::Enum16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor4Address },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor4OsVersion },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor4ProductVersion },
        size: 2,
        data_type: PointType::String,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64001 { point: Point::Sensor4SerialNum },
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

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::CommandCode => if let Some(value) = model.command_code() { serialisation::write_u16(value as u16, buffer); },
        Point::HardwareRevision => if let Some(value) = model.hardware_revision() { serialisation::write_u16(value, buffer); },
        Point::RsFwRevision => if let Some(value) = model.rs_fw_revision() { serialisation::write_u16(value, buffer); },
        Point::OsFwRevision => if let Some(value) = model.os_fw_revision() { serialisation::write_u16(value, buffer); },
        Point::ProductRevision => if let Some(value) = model.product_revision() { serialisation::write_string(value, buffer, offset, limit); },
        Point::BootCount => if let Some(value) = model.boot_count() { serialisation::write_u16(value, buffer); },
        Point::DipSwitches => if let Some(value) = model.dip_switches() { serialisation::write_u16(value, buffer); },
        Point::NumDetectedSensors => if let Some(value) = model.num_detected_sensors() { serialisation::write_u16(value, buffer); },
        Point::NumCommunicatingSensors => if let Some(value) = model.num_communicating_sensors() { serialisation::write_u16(value, buffer); },
        Point::SystemStatus => if let Some(value) = model.system_status() { serialisation::write_u16(value, buffer); },
        Point::SystemConfiguration => if let Some(value) = model.system_configuration() { serialisation::write_u16(value, buffer); },
        Point::LedBlinkThreshold => if let Some(value) = model.led_blink_threshold() { serialisation::write_u16(value, buffer); },
        Point::LedOnThreshold => if let Some(value) = model.led_on_threshold() { serialisation::write_u16(value, buffer); },
        Point::Reserved => if let Some(value) = model.reserved() { serialisation::write_u16(value, buffer); },
        Point::LocationString => if let Some(value) = model.location_string() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sensor1UnitId => if let Some(value) = model.sensor_1_unit_id() { serialisation::write_u16(value as u16, buffer); },
        Point::Sensor1Address => if let Some(value) = model.sensor_1_address() { serialisation::write_u16(value, buffer); },
        Point::Sensor1OsVersion => if let Some(value) = model.sensor_1_os_version() { serialisation::write_u16(value, buffer); },
        Point::Sensor1ProductVersion => if let Some(value) = model.sensor_1_product_version() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sensor1SerialNum => if let Some(value) = model.sensor_1_serial_num() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sensor2UnitId => if let Some(value) = model.sensor_2_unit_id() { serialisation::write_u16(value as u16, buffer); },
        Point::Sensor2Address => if let Some(value) = model.sensor_2_address() { serialisation::write_u16(value, buffer); },
        Point::Sensor2OsVersion => if let Some(value) = model.sensor_2_os_version() { serialisation::write_u16(value, buffer); },
        Point::Sensor2ProductVersion => if let Some(value) = model.sensor_2_product_version() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sensor2SerialNum => if let Some(value) = model.sensor_2_serial_num() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sensor3UnitId => if let Some(value) = model.sensor_3_unit_id() { serialisation::write_u16(value as u16, buffer); },
        Point::Sensor3Address => if let Some(value) = model.sensor_3_address() { serialisation::write_u16(value, buffer); },
        Point::Sensor3OsVersion => if let Some(value) = model.sensor_3_os_version() { serialisation::write_u16(value, buffer); },
        Point::Sensor3ProductVersion => if let Some(value) = model.sensor_3_product_version() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sensor3SerialNum => if let Some(value) = model.sensor_3_serial_num() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sensor4UnitId => if let Some(value) = model.sensor_4_unit_id() { serialisation::write_u16(value as u16, buffer); },
        Point::Sensor4Address => if let Some(value) = model.sensor_4_address() { serialisation::write_u16(value, buffer); },
        Point::Sensor4OsVersion => if let Some(value) = model.sensor_4_os_version() { serialisation::write_u16(value, buffer); },
        Point::Sensor4ProductVersion => if let Some(value) = model.sensor_4_product_version() { serialisation::write_string(value, buffer, offset, limit); },
        Point::Sensor4SerialNum => if let Some(value) = model.sensor_4_serial_num() { serialisation::write_string(value, buffer, offset, limit); },
    }
}

pub trait ModelAdapter {
    /// Command Code
    fn command_code(&self) -> Option<Cmd> {
        None
    }

    /// Command Code
    fn set_command_code(&mut self, value: Cmd) {
    }

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
    fn product_revision(&self) -> Option<String<4>> {
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
    fn location_string(&self) -> Option<String<32>> {
        None
    }

    /// Sensor 1 Unit ID
    fn sensor_1_unit_id(&self) -> Option<S1id> {
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
    fn sensor_1_product_version(&self) -> Option<String<4>> {
        None
    }

    /// Sensor 1 Serial Num
    fn sensor_1_serial_num(&self) -> Option<String<10>> {
        None
    }

    /// Sensor 2 Unit ID
    fn sensor_2_unit_id(&self) -> Option<S2id> {
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
    fn sensor_2_product_version(&self) -> Option<String<4>> {
        None
    }

    /// Sensor 2 Serial Num
    fn sensor_2_serial_num(&self) -> Option<String<10>> {
        None
    }

    /// Sensor 3 Unit ID
    fn sensor_3_unit_id(&self) -> Option<S3id> {
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
    fn sensor_3_product_version(&self) -> Option<String<4>> {
        None
    }

    /// Sensor 3 Serial Num
    fn sensor_3_serial_num(&self) -> Option<String<10>> {
        None
    }

    /// Sensor 4 Unit ID
    fn sensor_4_unit_id(&self) -> Option<S4id> {
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
    fn sensor_4_product_version(&self) -> Option<String<4>> {
        None
    }

    /// Sensor 4 Serial Num
    fn sensor_4_serial_num(&self) -> Option<String<10>> {
        None
    }
}

pub enum Cmd {
}

pub enum S1id {
}

pub enum S2id {
}

pub enum S3id {
}

pub enum S4id {
}