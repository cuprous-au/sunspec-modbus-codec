pub type Model64413 = PvSimCurves;

/// Current-Voltage and Power-Voltage Profiles for PV Simulation.
pub struct PvSimCurves {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length.
    l: u16,
    /// IV length
    ///
    /// Number of points in the IV curve.
    iv_len: Option<u16>,
    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    irr: Option<u16>,
    irr_sf: Option<u16>,
}

trait PvSimCurvesTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length.
    fn l(&self) -> u16;

    /// IV length
    ///
    /// Number of points in the IV curve.
    fn iv_len(&self) -> Option<u16> {
        None
    }

    /// POA Irradiance
    ///
    /// Plane of Array Irradiance
    fn irr(&self) -> Option<u16> {
        None
    }

    fn irr_sf(&self) -> Option<u16> {
        None
    }
}
