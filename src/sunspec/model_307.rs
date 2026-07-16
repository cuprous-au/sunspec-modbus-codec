pub type Model307 = BaseMet;

/// Base Meteorological Model
pub struct BaseMet {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Ambient Temperature
    tmp_amb: Option<i16>,
    /// Relative Humidity
    rh: Option<i16>,
    /// Barometric Pressure
    pres: Option<i16>,
    /// Wind Speed
    wnd_spd: Option<i16>,
    /// Wind Direction
    wnd_dir: Option<i16>,
    /// Rainfall
    rain: Option<i16>,
    /// Snow Depth
    snw: Option<i16>,
    /// Precipitation Type
    ///
    /// Precipitation Type (WMO 4680 SYNOP code reference)
    ppt: Option<i16>,
    /// Electric Field
    elec_fld: Option<i16>,
    /// Surface Wetness
    sur_wet: Option<i16>,
    /// Soil Wetness
    soil_wet: Option<i16>,
}

trait BaseMetTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Ambient Temperature
    fn tmp_amb(&self) -> Option<i16> {
        None
    }

    /// Relative Humidity
    fn rh(&self) -> Option<i16> {
        None
    }

    /// Barometric Pressure
    fn pres(&self) -> Option<i16> {
        None
    }

    /// Wind Speed
    fn wnd_spd(&self) -> Option<i16> {
        None
    }

    /// Wind Direction
    fn wnd_dir(&self) -> Option<i16> {
        None
    }

    /// Rainfall
    fn rain(&self) -> Option<i16> {
        None
    }

    /// Snow Depth
    fn snw(&self) -> Option<i16> {
        None
    }

    /// Precipitation Type
    ///
    /// Precipitation Type (WMO 4680 SYNOP code reference)
    fn ppt(&self) -> Option<i16> {
        None
    }

    /// Electric Field
    fn elec_fld(&self) -> Option<i16> {
        None
    }

    /// Surface Wetness
    fn sur_wet(&self) -> Option<i16> {
        None
    }

    /// Soil Wetness
    fn soil_wet(&self) -> Option<i16> {
        None
    }
}
