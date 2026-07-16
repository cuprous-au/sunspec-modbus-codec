pub type Model134 = FreqWatt;

/// Curve-Based Frequency-Watt
pub struct FreqWatt {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    act_crv: u16,
    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    mod_ena: u16,
    /// WinTms
    ///
    /// Time window for freq-watt change.
    win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    n_crv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    n_pt: u16,
    /// Hz_SF
    ///
    /// Scale factor for frequency.
    hz_sf: u16,
    /// W_SF
    ///
    /// Scale factor for percent WRef.
    w_sf: u16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    rmp_inc_dec_sf: Option<u16>,
}

trait FreqWattTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn act_crv(&self) -> u16;

    /// ActCrv
    ///
    /// Index of active curve. 0=no active curve.
    fn set_act_crv(&mut self, value: u16);

    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is curve-based Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for freq-watt change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for freq-watt change.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for freq-watt curve selection.
    fn set_rvrt_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    fn set_rmp_tms(&mut self, value: u16) {}

    /// NCrv
    ///
    /// Number of curves supported (recommend min. 4).
    fn n_crv(&self) -> u16;

    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16;

    /// Hz_SF
    ///
    /// Scale factor for frequency.
    fn hz_sf(&self) -> u16;

    /// W_SF
    ///
    /// Scale factor for percent WRef.
    fn w_sf(&self) -> u16;

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        None
    }
}
