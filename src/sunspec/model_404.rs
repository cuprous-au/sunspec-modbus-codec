pub type Model404 = StringCombinerAdvancedInputs;

/// An advanced string combiner including voltage and energy measurements
pub struct StringCombinerAdvancedInputs {
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
    dc_wh_sf: Option<u16>,
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
    dcv: Option<i16>,
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
    dcpr: Option<i16>,
    /// Watt-hours
    ///
    /// Output energy
    dc_wh: Option<u32>,
    /// Current scale factor for inputs
    in_dca_sf: Option<u16>,
    /// Amp-hour scale factor for inputs
    in_dc_ahr_sf: Option<u16>,
    /// Voltage scale factor for inputs
    in_dcv_sf: Option<u16>,
    /// Power scale factor for inputs
    in_dcw_sf: Option<u16>,
    /// Energy scale factor for inputs
    in_dc_wh_sf: Option<u16>,
}

trait StringCombinerAdvancedInputsTrait {
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
    fn dc_wh_sf(&self) -> Option<u16> {
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
    fn dcv(&self) -> Option<i16> {
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
    fn dcpr(&self) -> Option<i16> {
        None
    }

    /// Watt-hours
    ///
    /// Output energy
    fn dc_wh(&self) -> Option<u32> {
        None
    }

    /// Current scale factor for inputs
    fn in_dca_sf(&self) -> Option<u16> {
        None
    }

    /// Amp-hour scale factor for inputs
    fn in_dc_ahr_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage scale factor for inputs
    fn in_dcv_sf(&self) -> Option<u16> {
        None
    }

    /// Power scale factor for inputs
    fn in_dcw_sf(&self) -> Option<u16> {
        None
    }

    /// Energy scale factor for inputs
    fn in_dc_wh_sf(&self) -> Option<u16> {
        None
    }
}
