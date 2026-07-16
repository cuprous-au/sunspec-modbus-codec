use heapless::String;

pub type Model122 = Status;

/// Inverter Controls Extended Measurements and Status
pub struct Status {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// PVConn
    ///
    /// PV inverter present/available status.
    pv_conn: u16,
    /// StorConn
    ///
    /// Storage inverter present/available status.
    stor_conn: u16,
    /// ECPConn
    ///
    /// ECP connection status: disconnected=0 connected=1.
    ecp_conn: u16,
    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    act_wh: Option<u64>,
    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    act_v_ah: Option<u64>,
    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    act_v_arh_q1: Option<u64>,
    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    act_v_arh_q2: Option<u64>,
    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output in quadrant 3.
    act_v_arh_q3: Option<u64>,
    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output in quadrant 4.
    act_v_arh_q4: Option<u64>,
    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    v_ar_aval: Option<i16>,
    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    v_ar_aval_sf: Option<u16>,
    /// WAval
    ///
    /// Amount of Watts available.
    w_aval: Option<u16>,
    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    w_aval_sf: Option<u16>,
    /// StSetLimMsk
    ///
    /// Setpoint limit(s) reached.
    ///
    /// Bits shall be automatically cleared on read.
    st_set_lim_msk: Option<u32>,
    /// StActCtl
    ///
    /// Which inverter controls are currently active.
    st_act_ctl: Option<u32>,
    /// TmSrc
    ///
    /// Source of time synchronization.
    tm_src: Option<String<8>>,
    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    tms: Option<u32>,
    /// RtSt
    ///
    /// Active ride-through status.
    rt_st: Option<u16>,
    /// Ris
    ///
    /// Isolation resistance.
    ris: Option<u16>,
    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    ris_sf: Option<u16>,
}

trait StatusTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// PVConn
    ///
    /// PV inverter present/available status.
    fn pv_conn(&self) -> u16;

    /// StorConn
    ///
    /// Storage inverter present/available status.
    fn stor_conn(&self) -> u16;

    /// ECPConn
    ///
    /// ECP connection status: disconnected=0 connected=1.
    fn ecp_conn(&self) -> u16;

    /// ActWh
    ///
    /// AC lifetime active (real) energy output.
    fn act_wh(&self) -> Option<u64> {
        None
    }

    /// ActVAh
    ///
    /// AC lifetime apparent energy output.
    fn act_v_ah(&self) -> Option<u64> {
        None
    }

    /// ActVArhQ1
    ///
    /// AC lifetime reactive energy output in quadrant 1.
    fn act_v_arh_q1(&self) -> Option<u64> {
        None
    }

    /// ActVArhQ2
    ///
    /// AC lifetime reactive energy output in quadrant 2.
    fn act_v_arh_q2(&self) -> Option<u64> {
        None
    }

    /// ActVArhQ3
    ///
    /// AC lifetime negative energy output in quadrant 3.
    fn act_v_arh_q3(&self) -> Option<u64> {
        None
    }

    /// ActVArhQ4
    ///
    /// AC lifetime reactive energy output in quadrant 4.
    fn act_v_arh_q4(&self) -> Option<u64> {
        None
    }

    /// VArAval
    ///
    /// Amount of VARs available without impacting watts output.
    fn v_ar_aval(&self) -> Option<i16> {
        None
    }

    /// VArAval_SF
    ///
    /// Scale factor for available VARs.
    fn v_ar_aval_sf(&self) -> Option<u16> {
        None
    }

    /// WAval
    ///
    /// Amount of Watts available.
    fn w_aval(&self) -> Option<u16> {
        None
    }

    /// WAval_SF
    ///
    /// Scale factor for available Watts.
    fn w_aval_sf(&self) -> Option<u16> {
        None
    }

    /// StSetLimMsk
    ///
    /// Setpoint limit(s) reached.
    ///
    /// Bits shall be automatically cleared on read.
    fn st_set_lim_msk(&self) -> Option<u32> {
        None
    }

    /// StActCtl
    ///
    /// Which inverter controls are currently active.
    fn st_act_ctl(&self) -> Option<u32> {
        None
    }

    /// TmSrc
    ///
    /// Source of time synchronization.
    fn tm_src(&self) -> Option<String<8>> {
        None
    }

    /// Tms
    ///
    /// Seconds since 01-01-2000 00:00 UTC
    fn tms(&self) -> Option<u32> {
        None
    }

    /// RtSt
    ///
    /// Active ride-through status.
    fn rt_st(&self) -> Option<u16> {
        None
    }

    /// Ris
    ///
    /// Isolation resistance.
    fn ris(&self) -> Option<u16> {
        None
    }

    /// Ris_SF
    ///
    /// Scale factor for isolation resistance.
    fn ris_sf(&self) -> Option<u16> {
        None
    }
}
