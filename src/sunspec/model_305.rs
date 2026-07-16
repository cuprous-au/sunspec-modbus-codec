use heapless::String;

pub type Model305 = Location;

/// Include to support location measurements
pub struct Location {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Tm
    ///
    /// UTC 24 hour time stamp to millisecond hhmmss.sssZ format
    tm: Option<String<12>>,
    /// Date
    ///
    /// UTC Date string YYYYMMDD format
    date: Option<String<8>>,
    /// Location
    ///
    /// Location string (40 chars max)
    loc: Option<String<40>>,
    /// Lat
    ///
    /// Latitude with seven degrees of precision
    lat: Option<i32>,
    /// Long
    ///
    /// Longitude with seven degrees of precision
    long: Option<i32>,
    /// Altitude
    ///
    /// Altitude measurement in meters
    alt: Option<i32>,
}

trait LocationTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Tm
    ///
    /// UTC 24 hour time stamp to millisecond hhmmss.sssZ format
    fn tm(&self) -> Option<String<12>> {
        None
    }

    /// Date
    ///
    /// UTC Date string YYYYMMDD format
    fn date(&self) -> Option<String<8>> {
        None
    }

    /// Location
    ///
    /// Location string (40 chars max)
    fn loc(&self) -> Option<String<40>> {
        None
    }

    /// Lat
    ///
    /// Latitude with seven degrees of precision
    fn lat(&self) -> Option<i32> {
        None
    }

    /// Long
    ///
    /// Longitude with seven degrees of precision
    fn long(&self) -> Option<i32> {
        None
    }

    /// Altitude
    ///
    /// Altitude measurement in meters
    fn alt(&self) -> Option<i32> {
        None
    }
}
