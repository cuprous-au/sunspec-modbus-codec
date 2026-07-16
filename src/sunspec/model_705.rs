pub type Model705 = DerVoltVar;

/// DER Volt-Var model.
pub struct DerVoltVar {
    /// Model ID
    ///
    /// DER Volt-Var model ID.
    id: u16,
    /// Model Length
    ///
    /// DER Volt-Var model length.
    l: u16,
    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
    ena: Ena,
    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    adpt_crv_req: u16,
    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
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
    /// Reversion Time Remaining
    ///
    /// Reversion time remaining in seconds.
    rvrt_rem: Option<u32>,
    /// Reversion Curve
    ///
    /// Default curve after reversion timeout.
    rvrt_crv: Option<u16>,
    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    v_sf: u16,
    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    dept_ref_sf: u16,
    /// Open-Loop Scale Factor
    ///
    /// Open loop response time scale factor.
    rsp_tms_sf: u16,
}

trait DerVoltVarTrait {
    /// Model ID
    ///
    /// DER Volt-Var model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER Volt-Var model length.
    fn l(&self) -> u16;

    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
    fn ena(&self) -> Ena;

    /// DER Volt-Var Module Enable
    ///
    /// Volt-Var control enable.
    fn set_ena(&mut self, value: Ena);

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn adpt_crv_req(&self) -> u16;

    /// Adopt Curve Request
    ///
    /// Index of curve points to adopt. First curve index is 1.
    fn set_adpt_crv_req(&mut self, value: u16);

    /// Adopt Curve Result
    ///
    /// Result of last adopt curve operation.
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

    /// Reversion Time Remaining
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

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn v_sf(&self) -> u16;

    /// Var Scale Factor
    ///
    /// Scale factor for curve var points.
    fn dept_ref_sf(&self) -> u16;

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
