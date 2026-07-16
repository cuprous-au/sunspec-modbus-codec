pub type Model502 = SolarModule;

/// A solar module model supporting DC-DC converter
pub struct SolarModule {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Current scale factor
    a_sf: Option<u16>,
    /// Voltage scale factor
    v_sf: Option<u16>,
    /// Power scale factor
    w_sf: Option<u16>,
    /// Energy scale factor
    wh_sf: Option<u16>,
    /// Status
    ///
    /// Module Status Code
    stat: Stat,
    /// Vendor Status
    ///
    /// Module Vendor Status Code
    stat_vend: Option<StatVend>,
    /// Events
    ///
    /// Module Event Flags
    evt: u32,
    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    evt_vend: Option<u32>,
    /// Control
    ///
    /// Module Control
    ctl: Option<Ctl>,
    /// Vendor Control
    ///
    /// Vendor Module Control
    ctl_vend: Option<CtlVend>,
    /// Control Value
    ///
    /// Module Control Value
    ctl_val: Option<i32>,
    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    tms: Option<u32>,
    /// Output Current
    ///
    /// Output Current
    out_a: Option<i16>,
    /// Output Voltage
    ///
    /// Output Voltage
    out_v: Option<i16>,
    /// Output Energy
    ///
    /// Output Energy
    out_wh: Option<u32>,
    /// Output Power
    ///
    /// Output Power
    out_pw: Option<i16>,
    /// Temp
    ///
    /// Module Temperature
    tmp: Option<i16>,
    /// Input Current
    ///
    /// Input Current
    in_a: Option<i16>,
    /// Input Voltage
    ///
    /// Input Voltage
    in_v: Option<i16>,
    /// Input Energy
    ///
    /// Input Energy
    in_wh: Option<u32>,
    /// Input Power
    ///
    /// Input Power
    in_w: Option<i16>,
}

trait SolarModuleTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Current scale factor
    fn a_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage scale factor
    fn v_sf(&self) -> Option<u16> {
        None
    }

    /// Power scale factor
    fn w_sf(&self) -> Option<u16> {
        None
    }

    /// Energy scale factor
    fn wh_sf(&self) -> Option<u16> {
        None
    }

    /// Status
    ///
    /// Module Status Code
    fn stat(&self) -> Stat;

    /// Vendor Status
    ///
    /// Module Vendor Status Code
    fn stat_vend(&self) -> Option<StatVend> {
        None
    }

    /// Events
    ///
    /// Module Event Flags
    fn evt(&self) -> u32;

    /// Vendor Module Event Flags
    ///
    /// Vendor specific flags
    fn evt_vend(&self) -> Option<u32> {
        None
    }

    /// Control
    ///
    /// Module Control
    fn ctl(&self) -> Option<Ctl> {
        None
    }

    /// Control
    ///
    /// Module Control
    fn set_ctl(&mut self, value: Ctl) {}

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn ctl_vend(&self) -> Option<CtlVend> {
        None
    }

    /// Vendor Control
    ///
    /// Vendor Module Control
    fn set_ctl_vend(&mut self, value: CtlVend) {}

    /// Control Value
    ///
    /// Module Control Value
    fn ctl_val(&self) -> Option<i32> {
        None
    }

    /// Control Value
    ///
    /// Module Control Value
    fn set_ctl_val(&mut self, value: i32) {}

    /// Timestamp
    ///
    /// Time in seconds since 2000 epoch
    fn tms(&self) -> Option<u32> {
        None
    }

    /// Output Current
    ///
    /// Output Current
    fn out_a(&self) -> Option<i16> {
        None
    }

    /// Output Voltage
    ///
    /// Output Voltage
    fn out_v(&self) -> Option<i16> {
        None
    }

    /// Output Energy
    ///
    /// Output Energy
    fn out_wh(&self) -> Option<u32> {
        None
    }

    /// Output Power
    ///
    /// Output Power
    fn out_pw(&self) -> Option<i16> {
        None
    }

    /// Temp
    ///
    /// Module Temperature
    fn tmp(&self) -> Option<i16> {
        None
    }

    /// Input Current
    ///
    /// Input Current
    fn in_a(&self) -> Option<i16> {
        None
    }

    /// Input Voltage
    ///
    /// Input Voltage
    fn in_v(&self) -> Option<i16> {
        None
    }

    /// Input Energy
    ///
    /// Input Energy
    fn in_wh(&self) -> Option<u32> {
        None
    }

    /// Input Power
    ///
    /// Input Power
    fn in_w(&self) -> Option<i16> {
        None
    }
}

pub enum Stat {
    Off = 1,
    Sleeping = 2,
    Starting = 3,
    Mppt = 4,
    Throttled = 5,
    ShuttingDown = 6,
    Fault = 7,
    Standby = 8,
    Test = 9,
    Other = 10,
}

pub enum StatVend {}

pub enum Ctl {}

pub enum CtlVend {}
