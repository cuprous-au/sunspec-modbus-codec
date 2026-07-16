pub type Model127 = FreqWattParam;

/// Parameterized Frequency-Watt
pub struct FreqWattParam {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    w_gra: u16,
    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    hz_str: i16,
    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    hz_stop: i16,
    /// HysEna
    ///
    /// Enable hysteresis
    hys_ena: u16,
    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    mod_ena: u16,
    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    hz_stop_w_gra: Option<u16>,
    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    w_gra_sf: Option<u16>,
    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    hz_str_stop_sf: Option<u16>,
    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    rmp_inc_dec_sf: Option<u16>,
}

trait FreqWattParamTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn w_gra(&self) -> u16;

    /// WGra
    ///
    /// The slope of the reduction in the maximum allowed watts output as a function of frequency.
    fn set_w_gra(&mut self, value: u16);

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn hz_str(&self) -> i16;

    /// HzStr
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which a snapshot of the instantaneous power output is taken to act as the CAPPED power level (PM) and above which reduction in power output occurs.
    fn set_hz_str(&mut self, value: i16);

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn hz_stop(&self) -> i16;

    /// HzStop
    ///
    /// The frequency deviation from nominal frequency (ECPNomHz) at which curtailed power output may return to normal and the cap on the power level value is removed.
    fn set_hz_stop(&mut self, value: i16);

    /// HysEna
    ///
    /// Enable hysteresis
    fn hys_ena(&self) -> u16;

    /// HysEna
    ///
    /// Enable hysteresis
    fn set_hys_ena(&mut self, value: u16);

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is Parameterized Frequency-Watt control active.
    fn set_mod_ena(&mut self, value: u16);

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn hz_stop_w_gra(&self) -> Option<u16> {
        None
    }

    /// HzStopWGra
    ///
    /// The maximum time-based rate of change at which power output returns to normal after having been capped by an over frequency event.
    fn set_hz_stop_w_gra(&mut self, value: u16) {}

    /// WGra_SF
    ///
    /// Scale factor for output gradient.
    fn w_gra_sf(&self) -> Option<u16> {
        None
    }

    /// HzStrStop_SF
    ///
    /// Scale factor for frequency deviations.
    fn hz_str_stop_sf(&self) -> Option<u16> {
        None
    }

    /// RmpIncDec_SF
    ///
    /// Scale factor for increment and decrement ramps.
    fn rmp_inc_dec_sf(&self) -> Option<u16> {
        None
    }
}
