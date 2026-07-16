pub type Model401 = StringCombinerCurrent;

/// A basic string combiner
pub struct StringCombinerCurrent {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Current scale factor
    dca_sf: u16,
    /// Amp-hour scale factor
    dc_ahr_sf: Option<u16>,
    /// Voltage scale factor
    dcv_sf: Option<u16>,
    /// Rating
    ///
    /// Maximum DC Current Rating
    dca_max: u16,
    /// N
    ///
    /// Number of Inputs
    n: u16,
    /// Event
    ///
    /// Events
    evt: u32,
    /// Vendor Event
    ///
    /// Vendor defined events
    evt_vnd: Option<u32>,
    /// Amps
    ///
    /// Total measured current
    dca: i16,
    /// Amp-hours
    ///
    /// Total metered Amp-hours
    dc_ahr: Option<u32>,
    /// Voltage
    ///
    /// Output Voltage
    dcv: Option<u16>,
    /// Temp
    ///
    /// Internal operating temperature
    tmp: Option<i16>,
}

trait StringCombinerCurrentTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Current scale factor
    fn dca_sf(&self) -> u16;

    /// Amp-hour scale factor
    fn dc_ahr_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage scale factor
    fn dcv_sf(&self) -> Option<u16> {
        None
    }

    /// Rating
    ///
    /// Maximum DC Current Rating
    fn dca_max(&self) -> u16;

    /// N
    ///
    /// Number of Inputs
    fn n(&self) -> u16;

    /// Event
    ///
    /// Events
    fn evt(&self) -> u32;

    /// Vendor Event
    ///
    /// Vendor defined events
    fn evt_vnd(&self) -> Option<u32> {
        None
    }

    /// Amps
    ///
    /// Total measured current
    fn dca(&self) -> i16;

    /// Amp-hours
    ///
    /// Total metered Amp-hours
    fn dc_ahr(&self) -> Option<u32> {
        None
    }

    /// Voltage
    ///
    /// Output Voltage
    fn dcv(&self) -> Option<u16> {
        None
    }

    /// Temp
    ///
    /// Internal operating temperature
    fn tmp(&self) -> Option<i16> {
        None
    }
}
