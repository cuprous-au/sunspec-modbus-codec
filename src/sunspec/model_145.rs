pub type Model145 = ExtSettings;

/// Inverter controls extended settings
pub struct ExtSettings {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    nom_rmp_up_rte: Option<u16>,
    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    nom_rmp_dn_rte: Option<u16>,
    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    emg_rmp_up_rte: Option<u16>,
    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    emg_rmp_dn_rte: Option<u16>,
    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    conn_rmp_up_rte: Option<u16>,
    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    conn_rmp_dn_rte: Option<u16>,
    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    a_gra: Option<u16>,
    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    rmp_sf: Option<u16>,
}

trait ExtSettingsTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    fn nom_rmp_up_rte(&self) -> Option<u16> {
        None
    }

    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    fn set_nom_rmp_up_rte(&mut self, value: u16) {}

    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    fn nom_rmp_dn_rte(&self) -> Option<u16> {
        None
    }

    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    fn set_nom_rmp_dn_rte(&mut self, value: u16) {}

    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    fn emg_rmp_up_rte(&self) -> Option<u16> {
        None
    }

    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    fn set_emg_rmp_up_rte(&mut self, value: u16) {}

    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    fn emg_rmp_dn_rte(&self) -> Option<u16> {
        None
    }

    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    fn set_emg_rmp_dn_rte(&mut self, value: u16) {}

    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    fn conn_rmp_up_rte(&self) -> Option<u16> {
        None
    }

    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    fn set_conn_rmp_up_rte(&mut self, value: u16) {}

    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    fn conn_rmp_dn_rte(&self) -> Option<u16> {
        None
    }

    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    fn set_conn_rmp_dn_rte(&mut self, value: u16) {}

    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    fn a_gra(&self) -> Option<u16> {
        None
    }

    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    fn set_a_gra(&mut self, value: u16) {}

    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    fn rmp_sf(&self) -> Option<u16> {
        None
    }
}
