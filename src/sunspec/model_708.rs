pub type Model708 = DerTripHv;

/// DER high voltage trip model.
pub struct DerTripHv {
    /// Model ID
    ///
    /// DER high voltage trip model ID.
    id: u16,
    /// Model Length
    ///
    /// DER high voltage trip model length.
    l: u16,
    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
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
    n_crv_set: u16,
    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    v_sf: u16,
    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    tms_sf: u16,
}

trait DerTripHvTrait {
    /// Model ID
    ///
    /// DER high voltage trip model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER high voltage trip model length.
    fn l(&self) -> u16;

    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
    fn ena(&self) -> Ena;

    /// DER Trip HV Module Enable
    ///
    /// DER high voltage trip control enable.
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
    fn n_crv_set(&self) -> u16;

    /// Voltage Scale Factor
    ///
    /// Scale factor for curve voltage points.
    fn v_sf(&self) -> u16;

    /// Time Point Scale Factor
    ///
    /// Scale factor for curve time points.
    fn tms_sf(&self) -> u16;
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
