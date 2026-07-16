pub type Model715 = DerCtl;

/// DER Control
pub struct DerCtl {
    /// Model ID
    ///
    /// DER control model ID.
    id: u16,
    /// Model Length
    ///
    /// DER control model length.
    l: u16,
    /// Control Mode
    ///
    /// DER control mode. Enumeration.
    loc_rem_ctl: Option<LocRemCtl>,
    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    der_hb: Option<u32>,
    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    controller_hb: Option<u32>,
    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    alarm_reset: Option<u16>,
    /// Set Operation
    ///
    /// Commands to PCS.
    op_ctl: Option<OpCtl>,
}

trait DerCtlTrait {
    /// Model ID
    ///
    /// DER control model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER control model length.
    fn l(&self) -> u16;

    /// Control Mode
    ///
    /// DER control mode. Enumeration.
    fn loc_rem_ctl(&self) -> Option<LocRemCtl> {
        None
    }

    /// DER Heartbeat
    ///
    /// Value is incremented every second by the DER with periodic resets to zero.
    fn der_hb(&self) -> Option<u32> {
        None
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn controller_hb(&self) -> Option<u32> {
        None
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second by the controller with periodic resets to zero.
    fn set_controller_hb(&mut self, value: u32) {}

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn alarm_reset(&self) -> Option<u16> {
        None
    }

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    fn set_alarm_reset(&mut self, value: u16) {}

    /// Set Operation
    ///
    /// Commands to PCS.
    fn op_ctl(&self) -> Option<OpCtl> {
        None
    }

    /// Set Operation
    ///
    /// Commands to PCS.
    fn set_op_ctl(&mut self, value: OpCtl) {}
}

pub enum LocRemCtl {
    /// Remote Control
    Remote = 0,
    /// Local Control
    ///
    /// Local mode is required for manual/maintenance operations. Once invoked, it must be explicitly exited for the inverter to be controlled remotely.
    Local = 1,
}

pub enum OpCtl {
    /// Stop the DER
    Stop = 0,
    /// Start the DER
    Start = 1,
    /// Enter Standby Mode
    EnterStandby = 2,
    /// Exit Standby Mode
    ExitStandby = 3,
}
