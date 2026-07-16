pub type Model803 = LithiumIonBank;

pub struct LithiumIonBank {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// String Count
    ///
    /// Number of strings in the bank.
    n_str: u16,
    /// Connected String Count
    ///
    /// Number of strings with contactor closed.
    n_str_con: u16,
    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    ///
    /// Measurement.
    mod_tmp_max: i16,
    /// Max Module Temperature String
    ///
    /// String containing the module with maximum temperature.
    mod_tmp_max_str: Option<u16>,
    /// Max Module Temperature Module
    ///
    /// Module with maximum temperature.
    mod_tmp_max_mod: Option<u16>,
    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    ///
    /// Measurement.
    mod_tmp_min: i16,
    /// Min Module Temperature String
    ///
    /// String containing the module with minimum temperature.
    mod_tmp_min_str: Option<u16>,
    /// Min Module Temperature Module
    ///
    /// Module with minimum temperature.
    mod_tmp_min_mod: Option<u16>,
    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    ///
    /// Calculation based on measurements.
    mod_tmp_avg: Option<i16>,
    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Measurement.
    str_v_max: Option<u16>,
    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    str_v_max_str: Option<u16>,
    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Measurement.
    str_v_min: Option<u16>,
    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    str_v_min_str: Option<u16>,
    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Calculation based on measurements.
    str_v_avg: Option<u16>,
    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Measurement.
    str_a_max: Option<i16>,
    /// Max String Current String
    ///
    /// String with the maximum current.
    str_a_max_str: Option<u16>,
    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Measurement.
    str_a_min: Option<i16>,
    /// Min String Current String
    ///
    /// String with the minimum current.
    str_a_min_str: Option<u16>,
    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Calculation based on measurements.
    str_a_avg: Option<i16>,
    /// Battery Cell Balancing Count
    ///
    /// Total number of cells that are currently being balanced.
    n_cell_bal: Option<u16>,
    /// Scale factor for cell voltage.
    cell_v_sf: u16,
    /// Scale factor for module temperatures.
    mod_tmp_sf: u16,
    /// Scale factor for string currents.
    a_sf: u16,
    /// Scale factor for string state of health.
    so_h_sf: Option<u16>,
    /// Scale factor for string state of charge.
    so_c_sf: u16,
    /// Scale factor for string voltage.
    v_sf: Option<u16>,
}

trait LithiumIonBankTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// String Count
    ///
    /// Number of strings in the bank.
    fn n_str(&self) -> u16;

    /// Connected String Count
    ///
    /// Number of strings with contactor closed.
    fn n_str_con(&self) -> u16;

    /// Max Module Temperature
    ///
    /// Maximum temperature for all modules in the bank.
    ///
    /// Measurement.
    fn mod_tmp_max(&self) -> i16;

    /// Max Module Temperature String
    ///
    /// String containing the module with maximum temperature.
    fn mod_tmp_max_str(&self) -> Option<u16> {
        None
    }

    /// Max Module Temperature Module
    ///
    /// Module with maximum temperature.
    fn mod_tmp_max_mod(&self) -> Option<u16> {
        None
    }

    /// Min Module Temperature
    ///
    /// Minimum temperature for all modules in the bank.
    ///
    /// Measurement.
    fn mod_tmp_min(&self) -> i16;

    /// Min Module Temperature String
    ///
    /// String containing the module with minimum temperature.
    fn mod_tmp_min_str(&self) -> Option<u16> {
        None
    }

    /// Min Module Temperature Module
    ///
    /// Module with minimum temperature.
    fn mod_tmp_min_mod(&self) -> Option<u16> {
        None
    }

    /// Average Module Temperature
    ///
    /// Average temperature for all modules in the bank.
    ///
    /// Calculation based on measurements.
    fn mod_tmp_avg(&self) -> Option<i16> {
        None
    }

    /// Max String Voltage
    ///
    /// Maximum string voltage for all strings in the bank.
    ///
    /// Measurement.
    fn str_v_max(&self) -> Option<u16> {
        None
    }

    /// Max String Voltage String
    ///
    /// String with maximum voltage.
    fn str_v_max_str(&self) -> Option<u16> {
        None
    }

    /// Min String Voltage
    ///
    /// Minimum string voltage for all strings in the bank.
    ///
    /// Measurement.
    fn str_v_min(&self) -> Option<u16> {
        None
    }

    /// Min String Voltage String
    ///
    /// String with minimum voltage.
    fn str_v_min_str(&self) -> Option<u16> {
        None
    }

    /// Average String Voltage
    ///
    /// Average string voltage for all strings in the bank.
    ///
    /// Calculation based on measurements.
    fn str_v_avg(&self) -> Option<u16> {
        None
    }

    /// Max String Current
    ///
    /// Maximum current of any string in the bank.
    ///
    /// Measurement.
    fn str_a_max(&self) -> Option<i16> {
        None
    }

    /// Max String Current String
    ///
    /// String with the maximum current.
    fn str_a_max_str(&self) -> Option<u16> {
        None
    }

    /// Min String Current
    ///
    /// Minimum current of any string in the bank.
    ///
    /// Measurement.
    fn str_a_min(&self) -> Option<i16> {
        None
    }

    /// Min String Current String
    ///
    /// String with the minimum current.
    fn str_a_min_str(&self) -> Option<u16> {
        None
    }

    /// Average String Current
    ///
    /// Average string current for all strings in the bank.
    ///
    /// Calculation based on measurements.
    fn str_a_avg(&self) -> Option<i16> {
        None
    }

    /// Battery Cell Balancing Count
    ///
    /// Total number of cells that are currently being balanced.
    fn n_cell_bal(&self) -> Option<u16> {
        None
    }

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for module temperatures.
    fn mod_tmp_sf(&self) -> u16;

    /// Scale factor for string currents.
    fn a_sf(&self) -> u16;

    /// Scale factor for string state of health.
    fn so_h_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for string state of charge.
    fn so_c_sf(&self) -> u16;

    /// Scale factor for string voltage.
    fn v_sf(&self) -> Option<u16> {
        None
    }
}
