use heapless::String;

/// Include to support a wired ethernet port
pub struct Model11 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    spd: u16,
    /// Interface Status Flags
    ///
    /// Interface flags.
    cfg_st: u16,
    /// Link State
    ///
    /// State information for this interface
    st: St,
    /// MAC
    ///
    /// IEEE MAC address of this interface
    mac: Option<[u8; 6]>,
    /// Name
    ///
    /// Interface name (8 chars)
    nam: Option<String<8>>,
    /// Control
    ///
    /// Control flags
    ctl: Option<u16>,
    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    frc_spd: Option<u16>,
}

trait Model11Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Ethernet Link Speed
    ///
    /// Interface speed in Mb/s
    fn spd(&self) -> u16;

    /// Interface Status Flags
    ///
    /// Interface flags.
    fn cfg_st(&self) -> u16;

    /// Link State
    ///
    /// State information for this interface
    fn st(&self) -> St;

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<[u8; 6]> {
        None
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn nam(&self) -> Option<String<8>> {
        None
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn set_nam(&mut self, value: String<8>) {}

    /// Control
    ///
    /// Control flags
    fn ctl(&self) -> Option<u16> {
        None
    }

    /// Control
    ///
    /// Control flags
    fn set_ctl(&mut self, value: u16) {}

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn frc_spd(&self) -> Option<u16> {
        None
    }

    /// Forced Speed
    ///
    /// Forced interface speed in Mb/s when AUTO is disabled
    fn set_frc_spd(&mut self, value: u16) {}
}

pub enum St {
    Unknown = 0,
    Enabled = 1,
    Disabled = 2,
    Testing = 3,
}
