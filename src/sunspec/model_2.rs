pub type Model2 = Aggregator;

/// Aggregates a collection of models for a given model id
pub struct Aggregator {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// AID
    ///
    /// Aggregated model id
    aid: u16,
    /// N
    ///
    /// Number of aggregated models
    n: u16,
    /// UN
    ///
    /// Update Number. Incrementing number each time the mapping is changed. If the number is not changed from the last reading the direct access to a specific offset will result in reading the same logical model as before. Otherwise the entire model must be read to refresh the changes
    un: u16,
    /// Status
    ///
    /// Enumerated status code
    st: St,
    /// Vendor Status
    ///
    /// Vendor specific status code
    st_vnd: Option<StVnd>,
    /// Event Code
    ///
    /// Bitmask event code
    evt: u32,
    /// Vendor Event Code
    ///
    /// Vendor specific event code
    evt_vnd: Option<u32>,
    /// Control
    ///
    /// Control register for all aggregated devices
    ctl: Option<Ctl>,
    /// Vendor Control
    ///
    /// Vendor control register for all aggregated devices
    ctl_vnd: Option<CtlVnd>,
    /// Control Value
    ///
    /// Numerical value used as a parameter to the control
    ctl_vl: Option<CtlVl>,
}

trait AggregatorTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// AID
    ///
    /// Aggregated model id
    fn aid(&self) -> u16;

    /// N
    ///
    /// Number of aggregated models
    fn n(&self) -> u16;

    /// UN
    ///
    /// Update Number. Incrementing number each time the mapping is changed. If the number is not changed from the last reading the direct access to a specific offset will result in reading the same logical model as before. Otherwise the entire model must be read to refresh the changes
    fn un(&self) -> u16;

    /// Status
    ///
    /// Enumerated status code
    fn st(&self) -> St;

    /// Vendor Status
    ///
    /// Vendor specific status code
    fn st_vnd(&self) -> Option<StVnd> {
        None
    }

    /// Event Code
    ///
    /// Bitmask event code
    fn evt(&self) -> u32;

    /// Vendor Event Code
    ///
    /// Vendor specific event code
    fn evt_vnd(&self) -> Option<u32> {
        None
    }

    /// Control
    ///
    /// Control register for all aggregated devices
    fn ctl(&self) -> Option<Ctl> {
        None
    }

    /// Vendor Control
    ///
    /// Vendor control register for all aggregated devices
    fn ctl_vnd(&self) -> Option<CtlVnd> {
        None
    }

    /// Control Value
    ///
    /// Numerical value used as a parameter to the control
    fn ctl_vl(&self) -> Option<CtlVl> {
        None
    }
}

pub enum St {
    Off = 1,
    On = 2,
    Full = 3,
    Fault = 4,
}

pub enum StVnd {}

pub enum Ctl {
    None = 0,
    Automatic = 1,
    ForceOff = 2,
    Test = 3,
    Throttle = 4,
}

pub enum CtlVnd {}

pub enum CtlVl {}
