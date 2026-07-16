pub type Model711 = DerFreqDroop;

/// DER Frequency Droop model.
pub struct DerFreqDroop {
    /// Model ID
    ///
    /// DER Frequency Droop model ID.
    id: u16,
    /// Model Length
    ///
    /// DER Frequency Droop model length.
    l: u16,
    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    ena: Ena,
    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    adpt_ctl_req: u16,
    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    adpt_ctl_rslt: AdptCtlRslt,
    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    n_ctl: u16,
    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    rvrt_tms: Option<u32>,
    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    rvrt_rem: Option<u32>,
    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    rvrt_ctl: Option<u16>,
    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    db_sf: u16,
    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    k_sf: u16,
    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    rsp_tms_sf: u16,
}

trait DerFreqDroopTrait {
    /// Model ID
    ///
    /// DER Frequency Droop model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER Frequency Droop model length.
    fn l(&self) -> u16;

    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn ena(&self) -> Ena;

    /// DER Frequency Droop Module Enable
    ///
    /// DER Frequency-Watt (Frequency-Droop) control enable.
    fn set_ena(&mut self, value: Ena);

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn adpt_ctl_req(&self) -> u16;

    /// Set Active Control Request
    ///
    /// Set active control. 0 = No active control.
    fn set_adpt_ctl_req(&mut self, value: u16);

    /// Set Active Control Result
    ///
    /// Result of last set active control operation.
    fn adpt_ctl_rslt(&self) -> AdptCtlRslt;

    /// Stored Control Count
    ///
    /// Number of stored controls supported.
    fn n_ctl(&self) -> u16;

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn rvrt_tms(&self) -> Option<u32> {
        None
    }

    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    fn set_rvrt_tms(&mut self, value: u32) {}

    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    fn rvrt_rem(&self) -> Option<u32> {
        None
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn rvrt_ctl(&self) -> Option<u16> {
        None
    }

    /// Reversion Control
    ///
    /// Default control after reversion timeout.
    fn set_rvrt_ctl(&mut self, value: u16) {}

    /// Deadband Scale Factor
    ///
    /// Deadband scale factor.
    fn db_sf(&self) -> u16;

    /// Frequency Change Scale Factor
    ///
    /// Frequency change scale factor.
    fn k_sf(&self) -> u16;

    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    fn rsp_tms_sf(&self) -> u16;
}

pub enum Ena {
    /// Disabled
    ///
    /// Function is disabled.
    Disabled = 0,
    /// Enabled
    ///
    /// Function is enabled.
    Enabled = 1,
}

pub enum AdptCtlRslt {
    /// Update In Progress
    ///
    /// Control update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Control update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Control update failed.
    Failed = 2,
}
