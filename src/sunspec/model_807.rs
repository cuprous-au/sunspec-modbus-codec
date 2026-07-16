pub type Model807 = FlowBatteryString;

pub struct FlowBatteryString {
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
    /// Number of modules in this string.
    n_mod: u16,
    /// Connected Module Count
    ///
    /// Number of electrically connected modules in this string.
    n_mod_con: u16,
    /// Max Module Voltage
    ///
    /// Maximum voltage for all modules in the string.
    ///
    /// Measurement.
    mod_v_max: u16,
    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    mod_v_max_mod: Option<u16>,
    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Measurement.
    mod_v_min: u16,
    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    mod_v_min_mod: Option<u16>,
    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Calculation based on measurements.
    mod_v_avg: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    cell_v_max: Option<u16>,
    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with the maximum voltage.
    cell_v_max_mod: Option<u16>,
    /// Max Cell Voltage Stack
    ///
    /// Stack containing the cell with the maximum voltage.
    cell_v_max_stk: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    cell_v_min: Option<u16>,
    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with the minimum voltage.
    cell_v_min_mod: Option<u16>,
    /// Min Cell Voltage Stack
    ///
    /// Stack containing the cell with the minimum voltage.
    cell_v_min_stk: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    cell_v_avg: Option<u16>,
    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    tmp_max: i16,
    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    tmp_max_mod: Option<u16>,
    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    tmp_min: i16,
    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    tmp_min_mod: Option<u16>,
    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    tmp_avg: i16,
    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    evt1: u32,
    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    evt2: u32,
    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    evt_vnd1: u32,
    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    evt_vnd2: u32,
    mod_v_sf: u16,
    /// Scale factor for voltage.
    cell_v_sf: u16,
    /// Scale factor for temperature.
    tmp_sf: u16,
    /// Scale factor for state of charge.
    so_c_sf: u16,
    /// Scale factor for open circuit voltage.
    ocv_sf: u16,
}

trait FlowBatteryStringTrait {
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
    /// Number of modules in this string.
    fn n_mod(&self) -> u16;

    /// Connected Module Count
    ///
    /// Number of electrically connected modules in this string.
    fn n_mod_con(&self) -> u16;

    /// Max Module Voltage
    ///
    /// Maximum voltage for all modules in the string.
    ///
    /// Measurement.
    fn mod_v_max(&self) -> u16;

    /// Max Module Voltage Module
    ///
    /// Module with the maximum voltage.
    fn mod_v_max_mod(&self) -> Option<u16> {
        None
    }

    /// Min Module Voltage
    ///
    /// Minimum voltage for all modules in the string.
    ///
    /// Measurement.
    fn mod_v_min(&self) -> u16;

    /// Min Module Voltage Module
    ///
    /// Module with the minimum voltage.
    fn mod_v_min_mod(&self) -> Option<u16> {
        None
    }

    /// Average Module Voltage
    ///
    /// Average voltage for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn mod_v_avg(&self) -> u16;

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the string.
    ///
    /// Measurement.
    fn cell_v_max(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage Module
    ///
    /// Module containing the cell with the maximum voltage.
    fn cell_v_max_mod(&self) -> Option<u16> {
        None
    }

    /// Max Cell Voltage Stack
    ///
    /// Stack containing the cell with the maximum voltage.
    fn cell_v_max_stk(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the string.
    ///
    /// Measurement.
    fn cell_v_min(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage Module
    ///
    /// Module containing the cell with the minimum voltage.
    fn cell_v_min_mod(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage Stack
    ///
    /// Stack containing the cell with the minimum voltage.
    fn cell_v_min_stk(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the string.
    ///
    /// Calculation based on measurements.
    fn cell_v_avg(&self) -> Option<u16> {
        None
    }

    /// Max Temperature
    ///
    /// Maximum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    fn tmp_max(&self) -> i16;

    /// Max Temperature Module
    ///
    /// Module with the maximum temperature.
    fn tmp_max_mod(&self) -> Option<u16> {
        None
    }

    /// Min Temperature
    ///
    /// Minimum electrolyte temperature for all modules in the string.
    ///
    /// Measurement.
    fn tmp_min(&self) -> i16;

    /// Min Temperature Module
    ///
    /// Module with the minimum temperature.
    fn tmp_min_mod(&self) -> Option<u16> {
        None
    }

    /// Average Temperature
    ///
    /// Average electrolyte temperature for all modules in the string.
    ///
    /// Calculation based on measurements.
    fn tmp_avg(&self) -> i16;

    /// String Event 1
    ///
    /// Alarms, warnings and status values.
    fn evt1(&self) -> u32;

    /// String Event 2
    ///
    /// Alarms, warnings and status values.
    fn evt2(&self) -> u32;

    /// Vendor Event Bitfield 1
    ///
    /// Vendor defined events.
    fn evt_vnd1(&self) -> u32;

    /// Vendor Event Bitfield 2
    ///
    /// Vendor defined events.
    fn evt_vnd2(&self) -> u32;

    fn mod_v_sf(&self) -> u16;

    /// Scale factor for voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for temperature.
    fn tmp_sf(&self) -> u16;

    /// Scale factor for state of charge.
    fn so_c_sf(&self) -> u16;

    /// Scale factor for open circuit voltage.
    fn ocv_sf(&self) -> u16;
}
