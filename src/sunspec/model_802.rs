pub type Model802 = Battery;

pub struct Battery {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Nameplate Charge Capacity
    ///
    /// Nameplate charge capacity in amp-hours.
    ah_rtg: u16,
    /// Nameplate Energy Capacity
    ///
    /// Nameplate energy capacity in DC watt-hours.
    wh_rtg: u16,
    /// Nameplate Max Charge Rate
    ///
    /// Maximum rate of energy transfer into the storage device in DC watts.
    w_cha_rte_max: u16,
    /// Nameplate Max Discharge Rate
    ///
    /// Maximum rate of energy transfer out of the storage device in DC watts.
    w_dis_cha_rte_max: u16,
    /// Self Discharge Rate
    ///
    /// Self discharge rate. Percentage of capacity (WHRtg) discharged per day.
    dis_cha_rte: Option<u16>,
    /// Nameplate Max SoC
    ///
    /// Manufacturer maximum state of charge, expressed as a percentage.
    so_c_max: Option<u16>,
    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    so_c_min: Option<u16>,
    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    soc_rsv_max: Option<u16>,
    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    so_c_rsv_min: Option<u16>,
    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Measurement.
    so_c: u16,
    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Measurement.
    do_d: Option<u16>,
    /// State of Health
    ///
    /// Percentage of battery life remaining.
    so_h: Option<u16>,
    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    n_cyc: Option<u32>,
    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    cha_st: Option<ChaSt>,
    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Maps to DRCC.LocRemCtl in IEC 61850.
    loc_rem_ctl: LocRemCtl,
    /// Battery Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    hb: Option<u16>,
    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    ctrl_hb: Option<u16>,
    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    alm_rst: u16,
    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Maps to DBAT.BatTyp in 61850.
    typ: Typ,
    /// State of the Battery Bank
    ///
    /// State of the battery bank. Enumeration.
    ///
    /// Must be reconciled with State in IEC 61850.
    state: State,
    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state. Enumeration.
    state_vnd: Option<StateVnd>,
    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Number of days since 1/1/2000.
    warr_dt: Option<u32>,
    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.
    evt1: u32,
    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.
    ///
    /// Reserved for future use.
    evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    evt_vnd1: u32,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    evt_vnd2: u32,
    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Maps to ZBAT.V in IEC 61850.
    v: u16,
    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    v_max: Option<u16>,
    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    v_min: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Measurement.
    cell_v_max: Option<u16>,
    /// Max Cell Voltage String
    ///
    /// String containing the cell with maximum voltage.
    cell_v_max_str: Option<u16>,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum voltage.
    cell_v_max_mod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the bank.
    ///
    /// Measurement.
    cell_v_min: Option<u16>,
    /// Min Cell Voltage String
    ///
    /// String containing the cell with minimum voltage.
    cell_v_min_str: Option<u16>,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum voltage.
    cell_v_min_mod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average cell voltage for all cells in the bank.
    ///
    /// Calculation based on measurements.
    cell_v_avg: Option<u16>,
    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Measurement.
    a: i16,
    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    a_cha_max: Option<u16>,
    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    a_dis_cha_max: Option<u16>,
    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// DC Measurement.
    w: i16,
    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter. Enumeration.
    ///
    /// Used in special states such as manual battery charging.
    req_inv_state: Option<ReqInvState>,
    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Used in special states such as string balancing.
    req_w: Option<i16>,
    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    set_op: SetOp,
    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    set_inv_state: SetInvState,
    /// Scale factor for charge capacity.
    ah_rtg_sf: u16,
    /// Scale factor for energy capacity.
    wh_rtg_sf: u16,
    /// Scale factor for maximum charge and discharge rate.
    w_cha_dis_cha_max_sf: u16,
    /// Scale factor for self discharge rate.
    dis_cha_rte_sf: Option<u16>,
    /// Scale factor for state of charge values.
    so_c_sf: u16,
    /// Scale factor for depth of discharge.
    do_d_sf: Option<u16>,
    /// Scale factor for state of health.
    so_h_sf: Option<u16>,
    /// Scale factor for DC bus voltage.
    v_sf: u16,
    /// Scale factor for cell voltage.
    cell_v_sf: u16,
    /// Scale factor for DC current.
    a_sf: u16,
    /// Scale factor for instantaneous DC charge/discharge current.
    a_max_sf: u16,
    /// Scale factor for AC power request.
    w_sf: Option<u16>,
}

trait BatteryTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Nameplate Charge Capacity
    ///
    /// Nameplate charge capacity in amp-hours.
    fn ah_rtg(&self) -> u16;

    /// Nameplate Energy Capacity
    ///
    /// Nameplate energy capacity in DC watt-hours.
    fn wh_rtg(&self) -> u16;

    /// Nameplate Max Charge Rate
    ///
    /// Maximum rate of energy transfer into the storage device in DC watts.
    fn w_cha_rte_max(&self) -> u16;

    /// Nameplate Max Discharge Rate
    ///
    /// Maximum rate of energy transfer out of the storage device in DC watts.
    fn w_dis_cha_rte_max(&self) -> u16;

    /// Self Discharge Rate
    ///
    /// Self discharge rate. Percentage of capacity (WHRtg) discharged per day.
    fn dis_cha_rte(&self) -> Option<u16> {
        None
    }

    /// Nameplate Max SoC
    ///
    /// Manufacturer maximum state of charge, expressed as a percentage.
    fn so_c_max(&self) -> Option<u16> {
        None
    }

    /// Nameplate Min SoC
    ///
    /// Manufacturer minimum state of charge, expressed as a percentage.
    fn so_c_min(&self) -> Option<u16> {
        None
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn soc_rsv_max(&self) -> Option<u16> {
        None
    }

    /// Max Reserve Percent
    ///
    /// Setpoint for maximum reserve for storage as a percentage of the nominal maximum storage.
    fn set_soc_rsv_max(&mut self, value: u16) {}

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn so_c_rsv_min(&self) -> Option<u16> {
        None
    }

    /// Min Reserve Percent
    ///
    /// Setpoint for minimum reserve for storage as a percentage of the nominal maximum storage.
    fn set_so_c_rsv_min(&mut self, value: u16) {}

    /// State of Charge
    ///
    /// State of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn so_c(&self) -> u16;

    /// Depth of Discharge
    ///
    /// Depth of discharge, expressed as a percentage.
    ///
    /// Measurement.
    fn do_d(&self) -> Option<u16> {
        None
    }

    /// State of Health
    ///
    /// Percentage of battery life remaining.
    fn so_h(&self) -> Option<u16> {
        None
    }

    /// Cycle Count
    ///
    /// Number of cycles executed in the battery.
    fn n_cyc(&self) -> Option<u32> {
        None
    }

    /// Charge Status
    ///
    /// Charge status of storage device. Enumeration.
    fn cha_st(&self) -> Option<ChaSt> {
        None
    }

    /// Control Mode
    ///
    /// Battery control mode. Enumeration.
    ///
    /// Maps to DRCC.LocRemCtl in IEC 61850.
    fn loc_rem_ctl(&self) -> LocRemCtl;

    /// Battery Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn hb(&self) -> Option<u16> {
        None
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn ctrl_hb(&self) -> Option<u16> {
        None
    }

    /// Controller Heartbeat
    ///
    /// Value is incremented every second with periodic resets to zero.
    fn set_ctrl_hb(&mut self, value: u16) {}

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn alm_rst(&self) -> u16;

    /// Alarm Reset
    ///
    /// Used to reset any latched alarms. 1 = Reset.
    ///
    /// Battery should reset to 0 when reset is complete.
    fn set_alm_rst(&mut self, value: u16);

    /// Battery Type
    ///
    /// Type of battery. Enumeration.
    ///
    /// Maps to DBAT.BatTyp in 61850.
    fn typ(&self) -> Typ;

    /// State of the Battery Bank
    ///
    /// State of the battery bank. Enumeration.
    ///
    /// Must be reconciled with State in IEC 61850.
    fn state(&self) -> State;

    /// Vendor Battery Bank State
    ///
    /// Vendor specific battery bank state. Enumeration.
    fn state_vnd(&self) -> Option<StateVnd> {
        None
    }

    /// Warranty Date
    ///
    /// Date the device warranty expires.
    ///
    /// Number of days since 1/1/2000.
    fn warr_dt(&self) -> Option<u32> {
        None
    }

    /// Battery Event 1 Bitfield
    ///
    /// Alarms and warnings.
    fn evt1(&self) -> u32;

    /// Battery Event 2 Bitfield
    ///
    /// Alarms and warnings.
    ///
    /// Reserved for future use.
    fn evt2(&self) -> u32;

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn evt_vnd1(&self) -> u32;

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn evt_vnd2(&self) -> u32;

    /// External Battery Voltage
    ///
    /// DC Bus Voltage.
    ///
    /// Maps to ZBAT.V in IEC 61850.
    fn v(&self) -> u16;

    /// Max Battery Voltage
    ///
    /// Instantaneous maximum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn v_max(&self) -> Option<u16> {
        None
    }

    /// Min Battery Voltage
    ///
    /// Instantaneous minimum battery voltage.
    ///
    /// If not implemented, must implement AChaMax and ADisChaMax.
    fn v_min(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn cell_v_max(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage String
    ///
    /// String containing the cell with maximum voltage.
    fn cell_v_max_str(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum voltage.
    fn cell_v_max_mod(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the bank.
    ///
    /// Measurement.
    fn cell_v_min(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage String
    ///
    /// String containing the cell with minimum voltage.
    fn cell_v_min_str(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum voltage.
    fn cell_v_min_mod(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average cell voltage for all cells in the bank.
    ///
    /// Calculation based on measurements.
    fn cell_v_avg(&self) -> Option<u16> {
        None
    }

    /// Total DC Current
    ///
    /// Total DC current flowing to/from the battery bank.
    ///
    /// Measurement.
    fn a(&self) -> i16;

    /// Max Charge Current
    ///
    /// Instantaneous maximum DC charge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn a_cha_max(&self) -> Option<u16> {
        None
    }

    /// Max Discharge Current
    ///
    /// Instantaneous maximum DC discharge current.
    ///
    /// Calculation which is always unsigned (i.e. magnitude only). If not implemented, must implement VMax and VMin.
    fn a_dis_cha_max(&self) -> Option<u16> {
        None
    }

    /// Total Power
    ///
    /// Total power flowing to/from the battery bank.
    ///
    /// DC Measurement.
    fn w(&self) -> i16;

    /// Inverter State Request
    ///
    /// Request from battery to start or stop the inverter. Enumeration.
    ///
    /// Used in special states such as manual battery charging.
    fn req_inv_state(&self) -> Option<ReqInvState> {
        None
    }

    /// Battery Power Request
    ///
    /// AC Power requested by battery.
    ///
    /// Used in special states such as string balancing.
    fn req_w(&self) -> Option<i16> {
        None
    }

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn set_op(&self) -> SetOp;

    /// Set Operation
    ///
    /// Instruct the battery bank to perform an operation such as connecting. Enumeration.
    fn set_set_op(&mut self, value: SetOp);

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_inv_state(&self) -> SetInvState;

    /// Set Inverter State
    ///
    /// Set the current state of the inverter.
    ///
    /// Information needed by battery for some operations.
    fn set_set_inv_state(&mut self, value: SetInvState);

    /// Scale factor for charge capacity.
    fn ah_rtg_sf(&self) -> u16;

    /// Scale factor for energy capacity.
    fn wh_rtg_sf(&self) -> u16;

    /// Scale factor for maximum charge and discharge rate.
    fn w_cha_dis_cha_max_sf(&self) -> u16;

    /// Scale factor for self discharge rate.
    fn dis_cha_rte_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for state of charge values.
    fn so_c_sf(&self) -> u16;

    /// Scale factor for depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for state of health.
    fn so_h_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for DC bus voltage.
    fn v_sf(&self) -> u16;

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for DC current.
    fn a_sf(&self) -> u16;

    /// Scale factor for instantaneous DC charge/discharge current.
    fn a_max_sf(&self) -> u16;

    /// Scale factor for AC power request.
    fn w_sf(&self) -> Option<u16> {
        None
    }
}

pub enum ChaSt {
    Off = 1,
    Empty = 2,
    Discharging = 3,
    Charging = 4,
    Full = 5,
    Holding = 6,
    Testing = 7,
}

pub enum LocRemCtl {
    /// Value of 0 matches LocRemCtl in IEC 61850.
    Remote = 0,
    /// Value of 1 matches LocRemCtl in IEC 61850.
    Local = 1,
}

pub enum Typ {
    NotApplicableUnknown = 0,
    LeadAcid = 1,
    NickelMetalHydrate = 2,
    NickelCadmium = 3,
    LithiumIon = 4,
    CarbonZinc = 5,
    ZincChloride = 6,
    Alkaline = 7,
    RechargeableAlkaline = 8,
    SodiumSulfur = 9,
    Flow = 10,
    Other = 99,
}

pub enum State {
    Disconnected = 1,
    Initializing = 2,
    Connected = 3,
    Standby = 4,
    SocProtection = 5,
    Suspending = 6,
    Fault = 99,
}

pub enum StateVnd {}

pub enum ReqInvState {
    NoRequest = 0,
    /// Battery is notified of inverter state change through SetInvState.
    Start = 1,
    /// Battery is notified of inverter state change through SetInvState.
    Stop = 2,
}

pub enum SetOp {
    Connect = 1,
    Disconnect = 2,
}

pub enum SetInvState {
    InverterStopped = 1,
    InverterStandby = 2,
    InverterStarted = 3,
}
