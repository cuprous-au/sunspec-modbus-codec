pub type Model120 = Nameplate;

/// Inverter Controls Nameplate Ratings
pub struct Nameplate {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    der_typ: DerTyp,
    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    w_rtg: u16,
    /// WRtg_SF
    ///
    /// Scale factor
    w_rtg_sf: u16,
    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    va_rtg: u16,
    /// VARtg_SF
    ///
    /// Scale factor
    va_rtg_sf: u16,
    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    v_ar_rtg_q1: i16,
    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    v_ar_rtg_q2: i16,
    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    v_ar_rtg_q3: i16,
    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    v_ar_rtg_q4: i16,
    /// VArRtg_SF
    ///
    /// Scale factor
    v_ar_rtg_sf: u16,
    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    a_rtg: u16,
    /// ARtg_SF
    ///
    /// Scale factor
    a_rtg_sf: u16,
    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// EEI sign convention.
    pf_rtg_q1: i16,
    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// EEI sign convention.
    pf_rtg_q2: i16,
    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// EEI sign convention.
    pf_rtg_q3: i16,
    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// EEI sign convention.
    pf_rtg_q4: i16,
    /// PFRtg_SF
    ///
    /// Scale factor
    pf_rtg_sf: u16,
    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    wh_rtg: Option<u16>,
    /// WHRtg_SF
    ///
    /// Scale factor
    wh_rtg_sf: Option<u16>,
    /// AhrRtg
    ///
    /// The usable capacity of the battery. Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    ahr_rtg: Option<u16>,
    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    ahr_rtg_sf: Option<u16>,
    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    max_cha_rte: Option<u16>,
    /// MaxChaRte_SF
    ///
    /// Scale factor
    max_cha_rte_sf: Option<u16>,
    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    max_dis_cha_rte: Option<u16>,
    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    max_dis_cha_rte_sf: Option<u16>,
}

trait NameplateTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// DERTyp
    ///
    /// Type of DER device. Default value is 4 to indicate PV device.
    fn der_typ(&self) -> DerTyp;

    /// WRtg
    ///
    /// Continuous power output capability of the inverter.
    fn w_rtg(&self) -> u16;

    /// WRtg_SF
    ///
    /// Scale factor
    fn w_rtg_sf(&self) -> u16;

    /// VARtg
    ///
    /// Continuous Volt-Ampere capability of the inverter.
    fn va_rtg(&self) -> u16;

    /// VARtg_SF
    ///
    /// Scale factor
    fn va_rtg_sf(&self) -> u16;

    /// VArRtgQ1
    ///
    /// Continuous VAR capability of the inverter in quadrant 1.
    fn v_ar_rtg_q1(&self) -> i16;

    /// VArRtgQ2
    ///
    /// Continuous VAR capability of the inverter in quadrant 2.
    fn v_ar_rtg_q2(&self) -> i16;

    /// VArRtgQ3
    ///
    /// Continuous VAR capability of the inverter in quadrant 3.
    fn v_ar_rtg_q3(&self) -> i16;

    /// VArRtgQ4
    ///
    /// Continuous VAR capability of the inverter in quadrant 4.
    fn v_ar_rtg_q4(&self) -> i16;

    /// VArRtg_SF
    ///
    /// Scale factor
    fn v_ar_rtg_sf(&self) -> u16;

    /// ARtg
    ///
    /// Maximum RMS AC current level capability of the inverter.
    ///
    /// Sum of all connected phases.  Current rating under nominal voltage under nominal power factor.
    fn a_rtg(&self) -> u16;

    /// ARtg_SF
    ///
    /// Scale factor
    fn a_rtg_sf(&self) -> u16;

    /// PFRtgQ1
    ///
    /// Minimum power factor capability of the inverter in quadrant 1.
    ///
    /// EEI sign convention.
    fn pf_rtg_q1(&self) -> i16;

    /// PFRtgQ2
    ///
    /// Minimum power factor capability of the inverter in quadrant 2.
    ///
    /// EEI sign convention.
    fn pf_rtg_q2(&self) -> i16;

    /// PFRtgQ3
    ///
    /// Minimum power factor capability of the inverter in quadrant 3.
    ///
    /// EEI sign convention.
    fn pf_rtg_q3(&self) -> i16;

    /// PFRtgQ4
    ///
    /// Minimum power factor capability of the inverter in quadrant 4.
    ///
    /// EEI sign convention.
    fn pf_rtg_q4(&self) -> i16;

    /// PFRtg_SF
    ///
    /// Scale factor
    fn pf_rtg_sf(&self) -> u16;

    /// WHRtg
    ///
    /// Nominal energy rating of storage device.
    fn wh_rtg(&self) -> Option<u16> {
        None
    }

    /// WHRtg_SF
    ///
    /// Scale factor
    fn wh_rtg_sf(&self) -> Option<u16> {
        None
    }

    /// AhrRtg
    ///
    /// The usable capacity of the battery. Maximum charge minus minimum charge from a technology capability perspective (Amp-hour capacity rating).
    fn ahr_rtg(&self) -> Option<u16> {
        None
    }

    /// AhrRtg_SF
    ///
    /// Scale factor for amp-hour rating.
    fn ahr_rtg_sf(&self) -> Option<u16> {
        None
    }

    /// MaxChaRte
    ///
    /// Maximum rate of energy transfer into the storage device.
    fn max_cha_rte(&self) -> Option<u16> {
        None
    }

    /// MaxChaRte_SF
    ///
    /// Scale factor
    fn max_cha_rte_sf(&self) -> Option<u16> {
        None
    }

    /// MaxDisChaRte
    ///
    /// Maximum rate of energy transfer out of the storage device.
    fn max_dis_cha_rte(&self) -> Option<u16> {
        None
    }

    /// MaxDisChaRte_SF
    ///
    /// Scale factor
    fn max_dis_cha_rte_sf(&self) -> Option<u16> {
        None
    }
}

pub enum DerTyp {
    Pv = 4,
    PvStor = 82,
}
