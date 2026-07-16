pub type Model712 = DerWattVar;

/// DER Watt-Var model.
pub struct DerWattVar {
    /// Model ID
    ///
    /// DER Watt-Var model ID.
    id: u16,
    /// Model Length
    ///
    /// DER Watt-Var model length.
    l: u16,
    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    ena: Ena,
    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    adpt_crv_req: u16,
    /// Set Active Curve Result
    ///
    /// Result of last set active curve operation.
    adpt_crv_rslt: AdptCrvRslt,
    /// Number Of Points
    ///
    /// Number of curve points supported.
    n_pt: u16,
    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    n_crv: u16,
    /// Reversion Timeout
    ///
    /// Reversion time in seconds. 0 = No reversion time.
    rvrt_tms: Option<u32>,
    /// Reversion Time Left
    ///
    /// Reversion time remaining in seconds.
    rvrt_rem: Option<u32>,
    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    rvrt_crv: Option<u16>,
    /// Active Power Scale Factor
    ///
    /// Scale factor for curve active power points.
    w_sf: u16,
    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    dept_ref_sf: u16,
}

trait DerWattVarTrait {
    /// Model ID
    ///
    /// DER Watt-Var model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER Watt-Var model length.
    fn l(&self) -> u16;

    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    fn ena(&self) -> Ena;

    /// DER Watt-Var Module Enable
    ///
    /// DER Watt-Var control enable.
    fn set_ena(&mut self, value: Ena);

    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    fn adpt_crv_req(&self) -> u16;

    /// Set Active Curve Request
    ///
    /// Set active curve. 0 = No active curve.
    fn set_adpt_crv_req(&mut self, value: u16);

    /// Set Active Curve Result
    ///
    /// Result of last set active curve operation.
    fn adpt_crv_rslt(&self) -> AdptCrvRslt;

    /// Number Of Points
    ///
    /// Number of curve points supported.
    fn n_pt(&self) -> u16;

    /// Stored Curve Count
    ///
    /// Number of stored curves supported.
    fn n_crv(&self) -> u16;

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

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn rvrt_crv(&self) -> Option<u16> {
        None
    }

    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    fn set_rvrt_crv(&mut self, value: u16) {}

    /// Active Power Scale Factor
    ///
    /// Scale factor for curve active power points.
    fn w_sf(&self) -> u16;

    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    fn dept_ref_sf(&self) -> u16;
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

pub enum AdptCrvRslt {
    /// Update In Progress
    ///
    /// Curve update in progress.
    InProgress = 0,
    /// Update Complete
    ///
    /// Curve update completed successfully.
    Completed = 1,
    /// Update Failed
    ///
    /// Curve update failed.
    Failed = 2,
}
