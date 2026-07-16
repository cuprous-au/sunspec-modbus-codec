use heapless::String;

pub type Model805 = LithiumIonModule;

pub struct LithiumIonModule {
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
    /// Index of the string containing the module.
    ///
    /// Indices are one-based.
    str_idx: u16,
    /// Module Index
    ///
    /// Index of the module within the string.
    ///
    /// Indices are one-based.
    mod_idx: u16,
    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    n_cell: u16,
    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    so_c: Option<u16>,
    /// Depth of Discharge
    ///
    /// Depth of discharge for the module.
    ///
    /// Measurement.
    do_d: Option<u16>,
    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    so_h: Option<u16>,
    /// Cycle Count
    ///
    /// Count of cycles executed.
    n_cyc: Option<u32>,
    /// Module Voltage
    ///
    /// Voltage of the module.
    ///
    /// Measurement.
    v: u16,
    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    ///
    /// Measurement.
    cell_v_max: u16,
    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum voltage.
    cell_v_max_cell: Option<u16>,
    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    ///
    /// Measurement.
    cell_v_min: u16,
    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum voltage.
    cell_v_min_cell: Option<u16>,
    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    ///
    /// Calculation based on measurements.
    cell_v_avg: u16,
    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    ///
    /// Measurement.
    cell_tmp_max: i16,
    /// Max Cell Temperature Cell
    ///
    /// Cell with the maximum cell temperature.
    cell_tmp_max_cell: Option<u16>,
    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    ///
    /// Measurement.
    cell_tmp_min: i16,
    /// Min Cell Temperature Cell
    ///
    /// Cell with the minimum cell temperature.
    cell_tmp_min_cell: Option<u16>,
    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    ///
    /// Calculation based on measurements.
    cell_tmp_avg: i16,
    /// Balanced Cell Count
    ///
    /// Number of cells currently being balanced in the module.
    n_cell_bal: Option<u16>,
    /// Serial Number
    ///
    /// Serial number for the module.
    sn: Option<String<32>>,
    /// Scale factor for module state of charge.
    so_c_sf: Option<u16>,
    /// Scale factor for module state of health.
    so_h_sf: Option<u16>,
    /// Scale factor for module depth of discharge.
    do_d_sf: Option<u16>,
    /// Scale factor for module voltage.
    v_sf: u16,
    /// Scale factor for cell voltage.
    cell_v_sf: u16,
    /// Scale factor for module temperature.
    tmp_sf: u16,
}

trait LithiumIonModuleTrait {
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
    /// Index of the string containing the module.
    ///
    /// Indices are one-based.
    fn str_idx(&self) -> u16;

    /// Module Index
    ///
    /// Index of the module within the string.
    ///
    /// Indices are one-based.
    fn mod_idx(&self) -> u16;

    /// Module Cell Count
    ///
    /// Count of all cells in the module.
    fn n_cell(&self) -> u16;

    /// Module SoC
    ///
    /// Module state of charge, expressed as a percentage.
    fn so_c(&self) -> Option<u16> {
        None
    }

    /// Depth of Discharge
    ///
    /// Depth of discharge for the module.
    ///
    /// Measurement.
    fn do_d(&self) -> Option<u16> {
        None
    }

    /// Module SoH
    ///
    /// Module state of health, expressed as a percentage.
    fn so_h(&self) -> Option<u16> {
        None
    }

    /// Cycle Count
    ///
    /// Count of cycles executed.
    fn n_cyc(&self) -> Option<u32> {
        None
    }

    /// Module Voltage
    ///
    /// Voltage of the module.
    ///
    /// Measurement.
    fn v(&self) -> u16;

    /// Max Cell Voltage
    ///
    /// Maximum voltage for all cells in the module.
    ///
    /// Measurement.
    fn cell_v_max(&self) -> u16;

    /// Max Cell Voltage Cell
    ///
    /// Cell with the maximum voltage.
    fn cell_v_max_cell(&self) -> Option<u16> {
        None
    }

    /// Min Cell Voltage
    ///
    /// Minimum voltage for all cells in the module.
    ///
    /// Measurement.
    fn cell_v_min(&self) -> u16;

    /// Min Cell Voltage Cell
    ///
    /// Cell with the minimum voltage.
    fn cell_v_min_cell(&self) -> Option<u16> {
        None
    }

    /// Average Cell Voltage
    ///
    /// Average voltage for all cells in the module.
    ///
    /// Calculation based on measurements.
    fn cell_v_avg(&self) -> u16;

    /// Max Cell Temperature
    ///
    /// Maximum temperature for all cells in the module.
    ///
    /// Measurement.
    fn cell_tmp_max(&self) -> i16;

    /// Max Cell Temperature Cell
    ///
    /// Cell with the maximum cell temperature.
    fn cell_tmp_max_cell(&self) -> Option<u16> {
        None
    }

    /// Min Cell Temperature
    ///
    /// Minimum temperature for all cells in the module.
    ///
    /// Measurement.
    fn cell_tmp_min(&self) -> i16;

    /// Min Cell Temperature Cell
    ///
    /// Cell with the minimum cell temperature.
    fn cell_tmp_min_cell(&self) -> Option<u16> {
        None
    }

    /// Average Cell Temperature
    ///
    /// Average temperature for all cells in the module.
    ///
    /// Calculation based on measurements.
    fn cell_tmp_avg(&self) -> i16;

    /// Balanced Cell Count
    ///
    /// Number of cells currently being balanced in the module.
    fn n_cell_bal(&self) -> Option<u16> {
        None
    }

    /// Serial Number
    ///
    /// Serial number for the module.
    fn sn(&self) -> Option<String<32>> {
        None
    }

    /// Scale factor for module state of charge.
    fn so_c_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for module state of health.
    fn so_h_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for module depth of discharge.
    fn do_d_sf(&self) -> Option<u16> {
        None
    }

    /// Scale factor for module voltage.
    fn v_sf(&self) -> u16;

    /// Scale factor for cell voltage.
    fn cell_v_sf(&self) -> u16;

    /// Scale factor for module temperature.
    fn tmp_sf(&self) -> u16;
}
