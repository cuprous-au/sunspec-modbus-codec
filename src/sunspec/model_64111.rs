pub struct Model64111 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Port Number
    port: u16,
    v_sf: u16,
    a_sf: u16,
    p_sf: u16,
    ah_sf: u16,
    kwh_sf: u16,
    /// Battery Voltage
    batt_v: u16,
    /// Array Voltage
    array_v: u16,
    /// Output Current
    output_a: u16,
    /// Array Current
    input_a: u16,
    /// Operating State
    charger_st: ChargerSt,
    /// Output Wattage
    output_w: u16,
    /// Today's Minimum Battery Voltage
    today_min_bat_v: u16,
    /// Today's Maximum Battery Voltage
    today_max_bat_v: u16,
    /// VOC
    vocv: u16,
    /// Today's Maximum VOC
    today_max_voc: u16,
    /// Today's kWh
    todayk_wh_output: u16,
    /// Today's AH
    today_ah_output: u16,
    /// Lifetime kWh
    life_time_kwh_out: u16,
    /// Lifetime kAH
    life_time_ah_out: u16,
    /// Lifetime Maximum Output Wattage
    life_time_max_out: u16,
    /// Lifetime Maximum Battery Voltage
    life_time_max_batt: u16,
    /// Lifetime Maximum VOC Voltage
    life_time_max_voc: u16,
}

trait Model64111Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Port Number
    fn port(&self) -> u16;

    fn v_sf(&self) -> u16;

    fn a_sf(&self) -> u16;

    fn p_sf(&self) -> u16;

    fn ah_sf(&self) -> u16;

    fn kwh_sf(&self) -> u16;

    /// Battery Voltage
    fn batt_v(&self) -> u16;

    /// Array Voltage
    fn array_v(&self) -> u16;

    /// Output Current
    fn output_a(&self) -> u16;

    /// Array Current
    fn input_a(&self) -> u16;

    /// Operating State
    fn charger_st(&self) -> ChargerSt;

    /// Output Wattage
    fn output_w(&self) -> u16;

    /// Today's Minimum Battery Voltage
    fn today_min_bat_v(&self) -> u16;

    /// Today's Maximum Battery Voltage
    fn today_max_bat_v(&self) -> u16;

    /// VOC
    fn vocv(&self) -> u16;

    /// Today's Maximum VOC
    fn today_max_voc(&self) -> u16;

    /// Today's kWh
    fn todayk_wh_output(&self) -> u16;

    /// Today's AH
    fn today_ah_output(&self) -> u16;

    /// Lifetime kWh
    fn life_time_kwh_out(&self) -> u16;

    /// Lifetime kAH
    fn life_time_ah_out(&self) -> u16;

    /// Lifetime Maximum Output Wattage
    fn life_time_max_out(&self) -> u16;

    /// Lifetime Maximum Battery Voltage
    fn life_time_max_batt(&self) -> u16;

    /// Lifetime Maximum VOC Voltage
    fn life_time_max_voc(&self) -> u16;
}

pub enum ChargerSt {
    Off = 0,
    Float = 1,
    Bulk = 2,
    Absorb = 3,
    Eq = 4,
}
