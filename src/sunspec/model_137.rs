pub type Model137 = Lvrtc;

/// LVRT must remain connected
pub struct Lvrtc {
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
    /// LVRT control mode. Enable active curve.
    mod_ena: u16,
    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    win_tms: Option<u16>,
    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    rvrt_tms: Option<u16>,
    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
    rmp_tms: Option<u16>,
    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    n_crv: u16,
    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    n_pt: u16,
    /// Tms_SF
    ///
    /// Scale factor for duration.
    tms_sf: u16,
    /// V_SF
    ///
    /// Scale factor for percent VRef.
    v_sf: u16,
}

trait LvrtcTrait {
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
    /// LVRT control mode. Enable active curve.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// LVRT control mode. Enable active curve.
    fn set_mod_ena(&mut self, value: u16);

    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    fn win_tms(&self) -> Option<u16> {
        None
    }

    /// WinTms
    ///
    /// Time window for LVRT change.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_win_tms(&mut self, value: u16) {}

    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    fn rvrt_tms(&self) -> Option<u16> {
        None
    }

    /// RvrtTms
    ///
    /// Timeout period for LVRT curve selection.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_rvrt_tms(&mut self, value: u16) {}

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
    fn rmp_tms(&self) -> Option<u16> {
        None
    }

    /// RmpTms
    ///
    /// Ramp time for moving from current mode to new mode.
    ///
    /// Setting is ignored for LVRT controls.
    fn set_rmp_tms(&mut self, value: u16) {}

    /// NCrv
    ///
    /// Number of curves supported (recommend 4).
    fn n_crv(&self) -> u16;

    /// NPt
    ///
    /// Number of curve points supported (maximum of 20).
    fn n_pt(&self) -> u16;

    /// Tms_SF
    ///
    /// Scale factor for duration.
    fn tms_sf(&self) -> u16;

    /// V_SF
    ///
    /// Scale factor for percent VRef.
    fn v_sf(&self) -> u16;
}
