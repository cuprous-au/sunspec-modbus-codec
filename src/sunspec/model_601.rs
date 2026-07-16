use heapless::String;

pub type Model601 = TrackerController;

/// Monitors and controls multiple trackers
pub struct TrackerController {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Controller
    ///
    /// Descriptive name for this control unit
    nam: Option<String<16>>,
    /// Type
    ///
    /// Type of tracker
    typ: Typ,
    /// Date
    ///
    /// Local date in YYYYMMDD format
    dt_loc: Option<String<10>>,
    /// Time
    ///
    /// 24 hour local time stamp to second
    tm_loc: Option<String<6>>,
    /// Day
    ///
    /// Number of the day in the year (1-366)
    day: Option<u16>,
    /// Manual Elevation
    ///
    /// Global manual override target position of elevation in degrees from horizontal. Unimplemented for single axis azimuth tracker type
    glbl_el_ctl: Option<i32>,
    /// Manual Azimuth
    ///
    /// Global manual override target position of azimuth in degrees from true north towards east. Unimplemented for single axis azimuth tracker type
    glbl_az_ctl: Option<i32>,
    /// Global Mode
    ///
    /// Global Control register operates on all trackers. Normal operation is automatic. Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
    ///
    /// The global controls all trackers
    glbl_ctl: Option<GlblCtl>,
    /// Global Alarm
    ///
    /// Global tracker alarm conditions
    ///
    /// Combined tracker alarm conditions.  See individual trackers for alarms
    glbl_alm: Option<u16>,
    /// SF
    ///
    /// Scale Factor for targets and position measurements in degrees
    dgr_sf: u16,
    /// Trackers
    ///
    /// Number of trackers being controlled. Size of repeating block.
    n: u16,
}

trait TrackerControllerTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Controller
    ///
    /// Descriptive name for this control unit
    fn nam(&self) -> Option<String<16>> {
        None
    }

    /// Type
    ///
    /// Type of tracker
    fn typ(&self) -> Typ;

    /// Date
    ///
    /// Local date in YYYYMMDD format
    fn dt_loc(&self) -> Option<String<10>> {
        None
    }

    /// Time
    ///
    /// 24 hour local time stamp to second
    fn tm_loc(&self) -> Option<String<6>> {
        None
    }

    /// Day
    ///
    /// Number of the day in the year (1-366)
    fn day(&self) -> Option<u16> {
        None
    }

    /// Manual Elevation
    ///
    /// Global manual override target position of elevation in degrees from horizontal. Unimplemented for single axis azimuth tracker type
    fn glbl_el_ctl(&self) -> Option<i32> {
        None
    }

    /// Manual Elevation
    ///
    /// Global manual override target position of elevation in degrees from horizontal. Unimplemented for single axis azimuth tracker type
    fn set_glbl_el_ctl(&mut self, value: i32) {}

    /// Manual Azimuth
    ///
    /// Global manual override target position of azimuth in degrees from true north towards east. Unimplemented for single axis azimuth tracker type
    fn glbl_az_ctl(&self) -> Option<i32> {
        None
    }

    /// Manual Azimuth
    ///
    /// Global manual override target position of azimuth in degrees from true north towards east. Unimplemented for single axis azimuth tracker type
    fn set_glbl_az_ctl(&mut self, value: i32) {}

    /// Global Mode
    ///
    /// Global Control register operates on all trackers. Normal operation is automatic. Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
    ///
    /// The global controls all trackers
    fn glbl_ctl(&self) -> Option<GlblCtl> {
        None
    }

    /// Global Mode
    ///
    /// Global Control register operates on all trackers. Normal operation is automatic. Operator can override the position by setting the ElCtl, AzCtl and enabling Manual operation. Entering calibration mode will revert to automatic operation after calibration is complete.
    ///
    /// The global controls all trackers
    fn set_glbl_ctl(&mut self, value: GlblCtl) {}

    /// Global Alarm
    ///
    /// Global tracker alarm conditions
    ///
    /// Combined tracker alarm conditions.  See individual trackers for alarms
    fn glbl_alm(&self) -> Option<u16> {
        None
    }

    /// SF
    ///
    /// Scale Factor for targets and position measurements in degrees
    fn dgr_sf(&self) -> u16;

    /// Trackers
    ///
    /// Number of trackers being controlled. Size of repeating block.
    fn n(&self) -> u16;
}

pub enum Typ {
    Unknown = 0,
    Fixed = 1,
    Horizontal = 2,
    Tilted = 3,
    Azimuth = 4,
    Dual = 5,
    Other = 99,
}

pub enum GlblCtl {
    Automatic = 0,
    Manual = 1,
    Calibrate = 2,
}
