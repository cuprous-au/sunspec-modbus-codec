pub type Model703 = DerEnterService;

/// Enter service model.
pub struct DerEnterService {
    /// Model ID
    ///
    /// Enter service model ID.
    id: u16,
    /// Model Length
    ///
    /// Enter service model length.
    l: u16,
    /// Permit Enter Service
    ///
    /// Permit enter service.
    es: Option<Es>,
    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    esv_hi: Option<u16>,
    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    esv_lo: Option<u16>,
    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    es_hz_hi: Option<u32>,
    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    es_hz_lo: Option<u32>,
    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    es_dly_tms: Option<u32>,
    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    es_rnd_tms: Option<u32>,
    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    es_rmp_tms: Option<u32>,
    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    es_dly_rem_tms: Option<u32>,
    /// Voltage Scale Factor
    ///
    /// Voltage percentage scale factor.
    v_sf: Option<u16>,
    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    hz_sf: Option<u16>,
}

trait DerEnterServiceTrait {
    /// Model ID
    ///
    /// Enter service model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Enter service model length.
    fn l(&self) -> u16;

    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn es(&self) -> Option<Es> {
        None
    }

    /// Permit Enter Service
    ///
    /// Permit enter service.
    fn set_es(&mut self, value: Es) {}

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn esv_hi(&self) -> Option<u16> {
        None
    }

    /// Enter Service Voltage High
    ///
    /// Enter service voltage high threshold as percent of normal voltage.
    fn set_esv_hi(&mut self, value: u16) {}

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn esv_lo(&self) -> Option<u16> {
        None
    }

    /// Enter Service Voltage Low
    ///
    /// Enter service voltage low threshold as percent of normal voltage.
    fn set_esv_lo(&mut self, value: u16) {}

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn es_hz_hi(&self) -> Option<u32> {
        None
    }

    /// Enter Service Frequency High
    ///
    /// Enter service frequency high threshold.
    fn set_es_hz_hi(&mut self, value: u32) {}

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn es_hz_lo(&self) -> Option<u32> {
        None
    }

    /// Enter Service Frequency Low
    ///
    /// Enter service frequency low threshold.
    fn set_es_hz_lo(&mut self, value: u32) {}

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn es_dly_tms(&self) -> Option<u32> {
        None
    }

    /// Enter Service Delay Time
    ///
    /// Enter service delay time in seconds.
    fn set_es_dly_tms(&mut self, value: u32) {}

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn es_rnd_tms(&self) -> Option<u32> {
        None
    }

    /// Enter Service Random Delay
    ///
    /// Enter service random delay in seconds.
    fn set_es_rnd_tms(&mut self, value: u32) {}

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn es_rmp_tms(&self) -> Option<u32> {
        None
    }

    /// Enter Service Ramp Time
    ///
    /// Enter service ramp time in seconds.
    fn set_es_rmp_tms(&mut self, value: u32) {}

    /// Enter Service Delay Remaining
    ///
    /// Enter service delay time remaining in seconds.
    fn es_dly_rem_tms(&self) -> Option<u32> {
        None
    }

    /// Voltage Scale Factor
    ///
    /// Voltage percentage scale factor.
    fn v_sf(&self) -> Option<u16> {
        None
    }

    /// Frequency Scale Factor
    ///
    /// Frequency scale factor.
    fn hz_sf(&self) -> Option<u16> {
        None
    }
}

pub enum Es {
    Disabled = 0,
    Enabled = 1,
}
