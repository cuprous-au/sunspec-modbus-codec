pub type Model713 = DerStorageCapacity;

/// DER storage capacity.
pub struct DerStorageCapacity {
    /// Model ID
    ///
    /// DER storage capacity model ID.
    id: u16,
    /// Model Length
    ///
    /// DER storage capacity model length.
    l: u16,
    /// Energy Rating
    ///
    /// Energy rating of the DER storage.
    wh_rtg: Option<u16>,
    /// Energy Available
    ///
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    wh_avail: Option<u16>,
    /// State of Charge
    ///
    /// State of charge of the DER storage.
    ///
    /// SOC shall be fixed to 0% for DER without storage capabilities.
    so_c: Option<u16>,
    /// State of Health
    ///
    /// State of health of the DER storage.
    so_h: Option<u16>,
    /// Status
    ///
    /// Storage status.
    sta: Option<Sta>,
    /// Energy Scale Factor
    ///
    /// Scale factor for energy capacity.
    wh_sf: Option<u16>,
    /// Percent Scale Factor
    ///
    /// Scale factor for percentage.
    pct_sf: Option<u16>,
}

trait DerStorageCapacityTrait {
    /// Model ID
    ///
    /// DER storage capacity model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER storage capacity model length.
    fn l(&self) -> u16;

    /// Energy Rating
    ///
    /// Energy rating of the DER storage.
    fn wh_rtg(&self) -> Option<u16> {
        None
    }

    /// Energy Available
    ///
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    fn wh_avail(&self) -> Option<u16> {
        None
    }

    /// State of Charge
    ///
    /// State of charge of the DER storage.
    ///
    /// SOC shall be fixed to 0% for DER without storage capabilities.
    fn so_c(&self) -> Option<u16> {
        None
    }

    /// State of Health
    ///
    /// State of health of the DER storage.
    fn so_h(&self) -> Option<u16> {
        None
    }

    /// Status
    ///
    /// Storage status.
    fn sta(&self) -> Option<Sta> {
        None
    }

    /// Energy Scale Factor
    ///
    /// Scale factor for energy capacity.
    fn wh_sf(&self) -> Option<u16> {
        None
    }

    /// Percent Scale Factor
    ///
    /// Scale factor for percentage.
    fn pct_sf(&self) -> Option<u16> {
        None
    }
}

pub enum Sta {
    /// OK
    ///
    /// No warnings or errors pending.
    Ok = 0,
    /// Warning
    ///
    /// One or more warnings pending.
    Warning = 1,
    /// Error
    ///
    /// One or more errors pending.
    Error = 2,
}
