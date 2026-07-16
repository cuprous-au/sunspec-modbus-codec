pub type Model128 = ReactiveCurrent;

/// Dynamic Reactive Current
pub struct ReactiveCurrent {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    ar_gra_mod: ArGraMod,
    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    ar_gra_sag: u16,
    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    ar_gra_swell: u16,
    /// ModEna
    ///
    /// Activate dynamic reactive current model
    mod_ena: u16,
    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fil_tms: Option<u16>,
    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    db_v_min: Option<u16>,
    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    db_v_max: Option<u16>,
    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    blk_zn_v: Option<u16>,
    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    hys_blk_zn_v: Option<u16>,
    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    blk_zn_tmms: Option<u16>,
    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    hold_tmms: Option<u16>,
    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    ar_gra_sf: u16,
    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    v_ref_pct_sf: Option<u16>,
}

trait ReactiveCurrentTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn ar_gra_mod(&self) -> ArGraMod;

    /// ArGraMod
    ///
    /// Indicates if gradients trend toward zero at the edges of the deadband or trend toward zero at the center of the deadband.
    fn set_ar_gra_mod(&mut self, value: ArGraMod);

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_sag(&self) -> u16;

    /// ArGraSag
    ///
    /// The gradient used to increase capacitive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_sag(&mut self, value: u16);

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn ar_gra_swell(&self) -> u16;

    /// ArGraSwell
    ///
    /// The gradient used to increase inductive dynamic current. A value of 0 indicates no additional reactive current support.
    fn set_ar_gra_swell(&mut self, value: u16);

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Activate dynamic reactive current model
    fn set_mod_ena(&mut self, value: u16);

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn fil_tms(&self) -> Option<u16> {
        None
    }

    /// FilTms
    ///
    /// The time window used to calculate the moving average voltage.
    fn set_fil_tms(&mut self, value: u16) {}

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn db_v_min(&self) -> Option<u16> {
        None
    }

    /// DbVMin
    ///
    /// The lower delta voltage limit for which negative voltage deviations less than this value no dynamic vars are produced.
    fn set_db_v_min(&mut self, value: u16) {}

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn db_v_max(&self) -> Option<u16> {
        None
    }

    /// DbVMax
    ///
    /// The upper delta voltage limit for which positive voltage deviations less than this value no dynamic current produced.
    fn set_db_v_max(&mut self, value: u16) {}

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn blk_zn_v(&self) -> Option<u16> {
        None
    }

    /// BlkZnV
    ///
    /// Block zone voltage which defines a lower voltage boundary below which no dynamic current is produced.
    fn set_blk_zn_v(&mut self, value: u16) {}

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn hys_blk_zn_v(&self) -> Option<u16> {
        None
    }

    /// HysBlkZnV
    ///
    /// Hysteresis voltage used with BlkZnV.
    fn set_hys_blk_zn_v(&mut self, value: u16) {}

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn blk_zn_tmms(&self) -> Option<u16> {
        None
    }

    /// BlkZnTmms
    ///
    /// Block zone time the time before which reactive current support remains active regardless of how low the voltage drops.
    fn set_blk_zn_tmms(&mut self, value: u16) {}

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn hold_tmms(&self) -> Option<u16> {
        None
    }

    /// HoldTmms
    ///
    /// Hold time during which reactive current support continues after the average voltage has entered the dead zone.
    fn set_hold_tmms(&mut self, value: u16) {}

    /// ArGra_SF
    ///
    /// Scale factor for the gradients.
    fn ar_gra_sf(&self) -> u16;

    /// VRefPct_SF
    ///
    /// Scale factor for the voltage zone and limit settings.
    fn v_ref_pct_sf(&self) -> Option<u16> {
        None
    }
}

pub enum ArGraMod {
    Edge = 0,
    Center = 1,
}
