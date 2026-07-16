pub struct Model64101 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    eltek_country_code: Option<u16>,
    eltek_feeding_phase: Option<u16>,
    eltek_apd_method: Option<u16>,
    eltek_apd_power_ref: Option<u16>,
    eltek_rps_method: Option<u16>,
    eltek_rps_q_ref: Option<u16>,
    eltek_rps_cos_phi_ref: Option<i16>,
}

trait Model64101Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    fn eltek_country_code(&self) -> Option<u16> {
        None
    }

    fn eltek_feeding_phase(&self) -> Option<u16> {
        None
    }

    fn eltek_apd_method(&self) -> Option<u16> {
        None
    }

    fn eltek_apd_power_ref(&self) -> Option<u16> {
        None
    }

    fn eltek_rps_method(&self) -> Option<u16> {
        None
    }

    fn eltek_rps_q_ref(&self) -> Option<u16> {
        None
    }

    fn eltek_rps_cos_phi_ref(&self) -> Option<i16> {
        None
    }
}
