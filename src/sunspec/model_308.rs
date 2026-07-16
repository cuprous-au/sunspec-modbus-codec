pub type Model308 = MiniMet;

/// Include to support a few basic measurements
pub struct MiniMet {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// GHI
    ///
    /// Global Horizontal Irradiance
    ghi: Option<u16>,
    /// Temp
    ///
    /// Back of module temperature measurement
    tmp_bom: Option<i16>,
    /// Ambient Temperature
    tmp_amb: Option<i16>,
    /// Wind Speed
    wnd_spd: Option<u16>,
}

trait MiniMetTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// GHI
    ///
    /// Global Horizontal Irradiance
    fn ghi(&self) -> Option<u16> {
        None
    }

    /// Temp
    ///
    /// Back of module temperature measurement
    fn tmp_bom(&self) -> Option<i16> {
        None
    }

    /// Ambient Temperature
    fn tmp_amb(&self) -> Option<i16> {
        None
    }

    /// Wind Speed
    fn wnd_spd(&self) -> Option<u16> {
        None
    }
}
