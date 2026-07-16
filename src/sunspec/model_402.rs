pub type Model402 = StringCombinerAdvanced;

/// An advanced string combiner
pub struct StringCombinerAdvanced {
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
    /// Power scale factor
    dcw_sf: Option<u16>,
    /// Energy scale factor
    dc_wh_sf: u16,
    /// Rating
    ///
    /// Maximum DC Current Rating
    dca_max: Option<u16>,
    /// N
    ///
    /// Number of Inputs
    n: Option<u16>,
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
    /// Watts
    ///
    /// Output power
    dcw: Option<i16>,
    /// PR
    ///
    /// DC Performance ratio value
    dcpr: Option<u16>,
    /// Watt-hours
    ///
    /// Output energy
    dc_wh: u32,
}

trait StringCombinerAdvancedTrait {
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

    /// Power scale factor
    fn dcw_sf(&self) -> Option<u16> {
        None
    }

    /// Energy scale factor
    fn dc_wh_sf(&self) -> u16;

    /// Rating
    ///
    /// Maximum DC Current Rating
    fn dca_max(&self) -> Option<u16> {
        None
    }

    /// N
    ///
    /// Number of Inputs
    fn n(&self) -> Option<u16> {
        None
    }

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

    /// Watts
    ///
    /// Output power
    fn dcw(&self) -> Option<i16> {
        None
    }

    /// PR
    ///
    /// DC Performance ratio value
    fn dcpr(&self) -> Option<u16> {
        None
    }

    /// Watt-hours
    ///
    /// Output energy
    fn dc_wh(&self) -> u32;
}
