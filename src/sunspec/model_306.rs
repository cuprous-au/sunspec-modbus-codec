pub type Model306 = RefPoint;

/// Include to support a standard reference point
pub struct RefPoint {
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
    /// Amps
    ///
    /// Current measurement at reference point
    a: Option<u16>,
    /// Voltage
    ///
    /// Voltage measurement at reference point
    v: Option<u16>,
    /// Temperature
    ///
    /// Temperature measurement at reference point
    tmp: Option<u16>,
}

trait RefPointTrait {
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

    /// Amps
    ///
    /// Current measurement at reference point
    fn a(&self) -> Option<u16> {
        None
    }

    /// Voltage
    ///
    /// Voltage measurement at reference point
    fn v(&self) -> Option<u16> {
        None
    }

    /// Temperature
    ///
    /// Temperature measurement at reference point
    fn tmp(&self) -> Option<u16> {
        None
    }
}
