pub type Model804 = LithiumIonString;

pub struct LithiumIonString {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Indices are one-based.
    idx: u16,
    /// Module Count
    ///
    /// Count of modules in the string.
    n_mod: u16,
    /// String Status
    ///
    /// Current status of the string.
    st: u32,
    /// Connection Failure Reason
    con_fail: Option<ConFail>,
    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    n_cell_bal: Option<u16>,
    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Measurement.
    so_c: u16,
    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Measurement.
    do_d: Option<u16>,
    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    n_cyc: Option<u32>,
    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Measurement.
    so_h: Option<u16>,
    /// String Current
    ///
    /// String current measurement.
    ///
    /// Measurement.
    a: i16,
    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Measurement.
    v: Option<u16>,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    cell_v_max: u16,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    cell_v_max_mod: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    cell_v_min: u16,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    cell_v_min_mod: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    cell_v_avg: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Measurement.
    mod_tmp_max: i16,
    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    mod_tmp_max_mod: u16,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Measurement.
    mod_tmp_min: i16,
    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    mod_tmp_min_mod: u16,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    mod_tmp_avg: i16,
    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    con_st: Option<u32>,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    evt1: u32,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    ///
    /// Reserved for future use.
    evt2: Option<u32>,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    evt_vnd1: Option<u32>,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    evt_vnd2: Option<u32>,
    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    set_ena: Option<SetEna>,
    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    set_con: Option<SetCon>,
    /// Scale factor for string state of charge.
    so_c_sf: u16,
    /// Scale factor for string state of health.
    so_h_sf: Option<u16>,
    /// Scale factor for string depth of discharge.
    do_d_sf: Option<u16>,
    /// Scale factor for string current.
    a_sf: u16,
    /// Scale factor for string voltage.
    v_sf: Option<u16>,
    /// Scale factor for cell voltage.
    cell_v_sf: u16,
    /// Scale factor for module temperature.
    mod_tmp_sf: u16,
}

trait LithiumIonStringTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// String Index
    ///
    /// Index of the string within the bank.
    ///
    /// Indices are one-based.
    fn idx(&self) -> u16;

    /// Module Count
    ///
    /// Count of modules in the string.
    fn n_mod(&self) -> u16;

    /// String Status
    ///
    /// Current status of the string.
    fn st(&self) -> u32;

    /// Connection Failure Reason
    fn con_fail(&self) -> Option<ConFail> {
        None
    }

    /// String Cell Balancing Count
    ///
    /// Number of cells currently being balanced in the string.
    fn n_cell_bal(&self) -> Option<u16> {
        None
    }

    /// String State of Charge
    ///
    /// Battery string state of charge, expressed as a percentage.
    ///
    /// Measurement.
    fn so_c(&self) -> u16;

    /// String Depth of Discharge
    ///
    /// Depth of discharge for the string, expressed as a percentage.
    ///
    /// Measurement.
    fn do_d(&self) -> Option<u16> {
        None
    }

    /// String Cycle Count
    ///
    /// Number of discharge cycles executed upon the string.
    fn n_cyc(&self) -> Option<u32> {
        None
    }

    /// String State of Health
    ///
    /// Battery string state of health, expressed as a percentage.
    ///
    /// Measurement.
    fn so_h(&self) -> Option<u16> {
        None
    }

    /// String Current
    ///
    /// String current measurement.
    ///
    /// Measurement.
    fn a(&self) -> i16;

    /// String Voltage
    ///
    /// String voltage measurement.
    ///
    /// Measurement.
    fn v(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn cell_v_max(&self) -> u16;

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with maximum cell voltage.
    fn cell_v_max_mod(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn cell_v_min(&self) -> u16;

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with minimum cell voltage.
    fn cell_v_min_mod(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn cell_v_avg(&self) -> u16;

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the string.
    ///
    /// Measurement.
    fn mod_tmp_max(&self) -> i16;

    /// Max Module Temperature Module
    ///
    /// Module with the maximum temperature.
    fn mod_tmp_max_mod(&self) -> u16;

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the string.
    ///
    /// Measurement.
    fn mod_tmp_min(&self) -> i16;

    /// Min Module Temperature Module
    ///
    /// Module with the minimum temperature.
    fn mod_tmp_min_mod(&self) -> u16;

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn mod_tmp_avg(&self) -> i16;

    /// Contactor Status
    ///
    /// Status of the contactor(s) for the string.
    fn con_st(&self) -> Option<u32> {
        None
    }

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn evt1(&self) -> u32;

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    ///
    /// Reserved for future use.
    fn evt2(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn evt_vnd1(&self) -> Option<u32> {
        None
    }

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn evt_vnd2(&self) -> Option<u32> {
        None
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn set_ena(&self) -> Option<SetEna> {
        None
    }

    /// Enable/Disable String
    ///
    /// Enables and disables the string. Should reset to 0 upon completion.
    fn set_set_ena(&mut self, value: SetEna) {}

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn set_con(&self) -> Option<SetCon> {
        None
    }

    /// Connect/Disconnect String
    ///
    /// Connects and disconnects the string.
    ///
    /// Should reset to 0 upon completion.
    fn set_set_con(&mut self, value: SetCon) {}

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16;

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for string depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for string current.
    fn a_sf(&self) -> u16;

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for module temperature.
    fn mod_tmp_sf(&self) -> u16;
}

pub enum ConFail {
    NoFailure = 0,
    ButtonPushed = 1,
    StrGroundFault = 2,
    OutsideVoltageRange = 3,
    StringNotEnabled = 4,
    FuseOpen = 5,
    ContactorFailure = 6,
    PrechargeFailure = 7,
    /// See Evt1 for more information.
    StringFault = 8,
}

pub enum SetEna {}

pub enum SetCon {
    ConnectString = 1,
    DisconnectString = 2,
}
