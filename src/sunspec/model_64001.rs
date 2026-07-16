use heapless::String;

pub struct Model64001 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Command Code
    cmd: Option<Cmd>,
    /// Hardware Revision
    hw_rev: Option<u16>,
    /// RS FW Revision
    rsfw_rev: Option<u16>,
    /// OS FW Revision
    osfw_rev: Option<u16>,
    /// Product Revision
    prod_rev: Option<String<4>>,
    /// Boot Count
    boots: Option<u16>,
    /// DIP Switches
    switch: Option<u16>,
    /// Num Detected Sensors
    sensors: Option<u16>,
    /// Num Communicating Sensors
    talking: Option<u16>,
    /// System Status
    status: Option<u16>,
    /// System Configuration
    config: Option<u16>,
    /// LED Blink Threshold
    le_dblink: Option<u16>,
    /// LED On Threshold
    le_don: Option<u16>,
    reserved: Option<u16>,
    /// Location String
    loc: Option<String<32>>,
    /// Sensor 1 Unit ID
    s1id: Option<S1id>,
    /// Sensor 1 Address
    s1_addr: Option<u16>,
    /// Sensor 1 OS Version
    s1os_ver: Option<u16>,
    /// Sensor 1 Product Version
    s1_ver: Option<String<4>>,
    /// Sensor 1 Serial Num
    s1_serial: Option<String<10>>,
    /// Sensor 2 Unit ID
    s2id: Option<S2id>,
    /// Sensor 2 Address
    s2_addr: Option<u16>,
    /// Sensor 2 OS Version
    s2os_ver: Option<u16>,
    /// Sensor 2 Product Version
    s2_ver: Option<String<4>>,
    /// Sensor 2 Serial Num
    s2_serial: Option<String<10>>,
    /// Sensor 3 Unit ID
    s3id: Option<S3id>,
    /// Sensor 3 Address
    s3_addr: Option<u16>,
    /// Sensor 3 OS Version
    s3os_ver: Option<u16>,
    /// Sensor 3 Product Version
    s3_ver: Option<String<4>>,
    /// Sensor 3 Serial Num
    s3_serial: Option<String<10>>,
    /// Sensor 4 Unit ID
    s4id: Option<S4id>,
    /// Sensor 4 Address
    s4_addr: Option<u16>,
    /// Sensor 4 OS Version
    s4os_ver: Option<u16>,
    /// Sensor 4 Product Version
    s4_ver: Option<String<4>>,
    /// Sensor 4 Serial Num
    s4_serial: Option<String<10>>,
}

trait Model64001Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Command Code
    fn cmd(&self) -> Option<Cmd> {
        None
    }

    /// Command Code
    fn set_cmd(&mut self, value: Cmd) {}

    /// Hardware Revision
    fn hw_rev(&self) -> Option<u16> {
        None
    }

    /// RS FW Revision
    fn rsfw_rev(&self) -> Option<u16> {
        None
    }

    /// OS FW Revision
    fn osfw_rev(&self) -> Option<u16> {
        None
    }

    /// Product Revision
    fn prod_rev(&self) -> Option<String<4>> {
        None
    }

    /// Boot Count
    fn boots(&self) -> Option<u16> {
        None
    }

    /// DIP Switches
    fn switch(&self) -> Option<u16> {
        None
    }

    /// Num Detected Sensors
    fn sensors(&self) -> Option<u16> {
        None
    }

    /// Num Communicating Sensors
    fn talking(&self) -> Option<u16> {
        None
    }

    /// System Status
    fn status(&self) -> Option<u16> {
        None
    }

    /// System Configuration
    fn config(&self) -> Option<u16> {
        None
    }

    /// LED Blink Threshold
    fn le_dblink(&self) -> Option<u16> {
        None
    }

    /// LED On Threshold
    fn le_don(&self) -> Option<u16> {
        None
    }

    fn reserved(&self) -> Option<u16> {
        None
    }

    /// Location String
    fn loc(&self) -> Option<String<32>> {
        None
    }

    /// Sensor 1 Unit ID
    fn s1id(&self) -> Option<S1id> {
        None
    }

    /// Sensor 1 Address
    fn s1_addr(&self) -> Option<u16> {
        None
    }

    /// Sensor 1 OS Version
    fn s1os_ver(&self) -> Option<u16> {
        None
    }

    /// Sensor 1 Product Version
    fn s1_ver(&self) -> Option<String<4>> {
        None
    }

    /// Sensor 1 Serial Num
    fn s1_serial(&self) -> Option<String<10>> {
        None
    }

    /// Sensor 2 Unit ID
    fn s2id(&self) -> Option<S2id> {
        None
    }

    /// Sensor 2 Address
    fn s2_addr(&self) -> Option<u16> {
        None
    }

    /// Sensor 2 OS Version
    fn s2os_ver(&self) -> Option<u16> {
        None
    }

    /// Sensor 2 Product Version
    fn s2_ver(&self) -> Option<String<4>> {
        None
    }

    /// Sensor 2 Serial Num
    fn s2_serial(&self) -> Option<String<10>> {
        None
    }

    /// Sensor 3 Unit ID
    fn s3id(&self) -> Option<S3id> {
        None
    }

    /// Sensor 3 Address
    fn s3_addr(&self) -> Option<u16> {
        None
    }

    /// Sensor 3 OS Version
    fn s3os_ver(&self) -> Option<u16> {
        None
    }

    /// Sensor 3 Product Version
    fn s3_ver(&self) -> Option<String<4>> {
        None
    }

    /// Sensor 3 Serial Num
    fn s3_serial(&self) -> Option<String<10>> {
        None
    }

    /// Sensor 4 Unit ID
    fn s4id(&self) -> Option<S4id> {
        None
    }

    /// Sensor 4 Address
    fn s4_addr(&self) -> Option<u16> {
        None
    }

    /// Sensor 4 OS Version
    fn s4os_ver(&self) -> Option<u16> {
        None
    }

    /// Sensor 4 Product Version
    fn s4_ver(&self) -> Option<String<4>> {
        None
    }

    /// Sensor 4 Serial Num
    fn s4_serial(&self) -> Option<String<10>> {
        None
    }
}

pub enum Cmd {}

pub enum S1id {}

pub enum S2id {}

pub enum S3id {}

pub enum S4id {}
