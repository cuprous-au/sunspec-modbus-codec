pub type Model131 = WattPf;

/// Watt-Power Factor
pub struct WattPf {
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
    /// Is watt-PF mode active.
    mod_ena: u16,
    /// WinTms
    ///
    /// Time window for watt-PF change.
    win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
    rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    n_crv: u16,
    /// NPt
    ///
    /// Max number of points in array.
    n_pt: u16,
    /// W_SF
    ///
    /// Scale factor for percent WMax.
    w_sf: u16,
    /// PF_SF
    ///
    /// Scale factor for PF.
    pf_sf: u16,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    rmp_inc_dec_sf: Option<u16>,
}

trait WattPfTrait {
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
    /// Is watt-PF mode active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is watt-PF mode active.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for watt-PF change.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for watt-PF change.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for watt-PF curve selection.
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
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16;

    /// NPt
    ///
    /// Max number of points in array.
    fn n_pt(&self) -> u16;

    /// W_SF
    ///
    /// Scale factor for percent WMax.
    fn w_sf(&self) -> u16;

    /// PF_SF
    ///
    /// Scale factor for PF.
    fn pf_sf(&self) -> u16;

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        None
    }
}
