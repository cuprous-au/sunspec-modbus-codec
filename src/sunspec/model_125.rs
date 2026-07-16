pub type Model125 = Pricing;

/// Pricing Signal  
pub struct Pricing {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    mod_ena: u16,
    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    sig_type: Option<SigType>,
    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    sig: i16,
    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    win_tms: Option<u16>,
    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    rvt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    rmp_tms: Option<u16>,
    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    sig_sf: u16,
}

trait PricingTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is price-based charge/discharge mode active?
    fn set_mod_ena(&mut self, value: u16);

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn sig_type(&self) -> Option<SigType> {
        None
    }

    /// SigType
    ///
    /// Meaning of the pricing signal. When a Price schedule is used, type must match the schedule range variable description.
    fn set_sig_type(&mut self, value: SigType) {}

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn sig(&self) -> i16;

    /// Sig
    ///
    /// Utility/ESP specific pricing signal. Content depends on pricing signal type. When H/M/L type is specified. Low=0; Med=1; High=2.
    fn set_sig(&mut self, value: i16);

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for charge/discharge pricing change.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn rvt_tms(&self) -> Option<u16> {
        None
    }

    /// RvtTms
    ///
    /// Timeout period for charge/discharge pricing change.
    fn set_rvt_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current charge or discharge level to new level.
    fn set_rmp_tms(&mut self, value: u16) {}

    /// Sig_SF
    ///
    /// Pricing signal scale factor.
    fn sig_sf(&self) -> u16;
}

pub enum SigType {
    Unknown = 0,
    Absolute = 1,
    Relative = 2,
    Multiplier = 3,
    Level = 4,
}
