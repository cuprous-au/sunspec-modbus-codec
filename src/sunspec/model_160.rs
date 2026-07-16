pub type Model160 = Mppt;

pub struct Mppt {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Current Scale Factor
    dca_sf: Option<u16>,
    /// Voltage Scale Factor
    dcv_sf: Option<u16>,
    /// Power Scale Factor
    dcw_sf: Option<u16>,
    /// Energy Scale Factor
    dcwh_sf: Option<u16>,
    /// Global Events
    evt: Option<u32>,
    /// Number of Modules
    n: Option<u16>,
    /// Timestamp Period
    tms_per: Option<u16>,
}

trait MpptTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Current Scale Factor
    fn dca_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage Scale Factor
    fn dcv_sf(&self) -> Option<u16> {
        None
    }

    /// Power Scale Factor
    fn dcw_sf(&self) -> Option<u16> {
        None
    }

    /// Energy Scale Factor
    fn dcwh_sf(&self) -> Option<u16> {
        None
    }

    /// Global Events
    fn evt(&self) -> Option<u32> {
        None
    }

    /// Number of Modules
    fn n(&self) -> Option<u16> {
        None
    }

    /// Timestamp Period
    fn tms_per(&self) -> Option<u16> {
        None
    }
}
