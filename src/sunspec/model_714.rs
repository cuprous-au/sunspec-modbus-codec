pub type Model714 = DerMeasureDc;

/// DER DC measurement.
pub struct DerMeasureDc {
    /// Model ID
    ///
    /// DER DC measurement model ID.
    id: u16,
    /// Model Length
    ///
    /// DER DC measurement model length.
    l: u16,
    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    prt_alrms: Option<u32>,
    /// Number Of Ports
    ///
    /// Number of DC ports.
    n_prt: Option<u16>,
    /// DC Current
    ///
    /// Total DC current for all ports.
    dca: Option<i16>,
    /// DC Power
    ///
    /// Total DC power for all ports.
    dcw: Option<i16>,
    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for all ports.
    dc_wh_inj: Option<u64>,
    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    dc_wh_abs: Option<u64>,
    /// DC Current Scale Factor
    ///
    /// DC current scale factor.
    dca_sf: Option<u16>,
    /// DC Voltage Scale Factor
    ///
    /// DC voltage scale factor.
    dcv_sf: Option<u16>,
    /// DC Power Scale Factor
    ///
    /// DC power scale factor.
    dcw_sf: Option<u16>,
    /// DC Energy Scale Factor
    ///
    /// DC energy scale factor.
    dcwh_sf: Option<u16>,
    /// Temperature Scale Factor
    ///
    /// Temperature Scale Factor.
    tmp_sf: Option<u16>,
}

trait DerMeasureDcTrait {
    /// Model ID
    ///
    /// DER DC measurement model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER DC measurement model length.
    fn l(&self) -> u16;

    /// Port Alarms
    ///
    /// Bitfield of ports with active alarms. Bit is 1 if port has an active alarm. Bit 0 is first port.
    fn prt_alrms(&self) -> Option<u32> {
        None
    }

    /// Number Of Ports
    ///
    /// Number of DC ports.
    fn n_prt(&self) -> Option<u16> {
        None
    }

    /// DC Current
    ///
    /// Total DC current for all ports.
    fn dca(&self) -> Option<i16> {
        None
    }

    /// DC Power
    ///
    /// Total DC power for all ports.
    fn dcw(&self) -> Option<i16> {
        None
    }

    /// DC Energy Injected
    ///
    /// Total cumulative DC energy injected for all ports.
    fn dc_wh_inj(&self) -> Option<u64> {
        None
    }

    /// DC Energy Absorbed
    ///
    /// Total cumulative DC energy absorbed for all ports.
    fn dc_wh_abs(&self) -> Option<u64> {
        None
    }

    /// DC Current Scale Factor
    ///
    /// DC current scale factor.
    fn dca_sf(&self) -> Option<u16> {
        None
    }

    /// DC Voltage Scale Factor
    ///
    /// DC voltage scale factor.
    fn dcv_sf(&self) -> Option<u16> {
        None
    }

    /// DC Power Scale Factor
    ///
    /// DC power scale factor.
    fn dcw_sf(&self) -> Option<u16> {
        None
    }

    /// DC Energy Scale Factor
    ///
    /// DC energy scale factor.
    fn dcwh_sf(&self) -> Option<u16> {
        None
    }

    /// Temperature Scale Factor
    ///
    /// Temperature Scale Factor.
    fn tmp_sf(&self) -> Option<u16> {
        None
    }
}
