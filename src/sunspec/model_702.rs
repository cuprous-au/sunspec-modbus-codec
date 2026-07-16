pub type Model702 = DerCapacity;

/// DER capacity model.
pub struct DerCapacity {
    /// Model ID
    ///
    /// DER capacity model ID.
    id: u16,
    /// Model Length
    ///
    /// DER capacity model length.
    l: u16,
    /// Active Power Max Rating
    ///
    /// Maximum active power rating at unity power factor in watts.
    w_max_rtg: Option<u16>,
    /// Active Power (Over-Excited) Rating
    ///
    /// Active power rating at specified over-excited power factor in watts.
    w_ovr_ext_rtg: Option<u16>,
    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    w_ovr_ext_rtg_pf: Option<u16>,
    /// Active Power (Under-Excited) Rating
    ///
    /// Active power rating at specified under-excited power factor in watts.
    w_und_ext_rtg: Option<u16>,
    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    w_und_ext_rtg_pf: Option<u16>,
    /// Apparent Power Max Rating
    ///
    /// Maximum apparent power rating in voltamperes.
    va_max_rtg: Option<u16>,
    /// Reactive Power Injected Rating
    ///
    /// Maximum injected reactive power rating in vars.
    var_max_inj_rtg: Option<u16>,
    /// Reactive Power Absorbed Rating
    ///
    /// Maximum absorbed reactive power rating in vars.
    var_max_abs_rtg: Option<u16>,
    /// Charge Rate Max Rating
    ///
    /// Maximum active power charge rate in watts.
    w_cha_rte_max_rtg: Option<u16>,
    /// Discharge Rate Max Rating
    ///
    /// Maximum active power discharge rate in watts.
    w_dis_cha_rte_max_rtg: Option<u16>,
    /// Charge Rate Max VA Rating
    ///
    /// Maximum apparent power charge rate in voltamperes.
    va_cha_rte_max_rtg: Option<u16>,
    /// Discharge Rate Max VA Rating
    ///
    /// Maximum apparent power discharge rate in voltamperes.
    va_dis_cha_rte_max_rtg: Option<u16>,
    /// AC Voltage Nominal Rating
    ///
    /// AC voltage nominal rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    v_nom_rtg: Option<u16>,
    /// AC Voltage Max Rating
    ///
    /// AC voltage maximum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    v_max_rtg: Option<u16>,
    /// AC Voltage Min Rating
    ///
    /// AC voltage minimum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    v_min_rtg: Option<u16>,
    /// AC Current Max Rating
    ///
    /// AC current maximum rating in amps.
    a_max_rtg: Option<u16>,
    /// PF Over-Excited Rating (Unused)
    ///
    /// Unused. Please use WOvrExtRtgPF.
    ///
    /// This point is duplicative of WOvrExtRtgPF.
    pf_ovr_ext_rtg: Option<u16>,
    /// PF Under-Excited Rating (Unused)
    ///
    /// Unused. Please use WUndExtRtgPF.
    ///
    /// This point is duplicative of WUndExtRtgPF.
    pf_und_ext_rtg: Option<u16>,
    /// Reactive Susceptance
    ///
    /// Reactive susceptance that remains connected to the Area EPS in the cease to energize and trip state.
    react_suscept_rtg: Option<u16>,
    /// Normal Operating Category
    ///
    /// Normal operating performance category as specified in IEEE 1547-2018.
    nor_op_cat_rtg: Option<NorOpCatRtg>,
    /// Abnormal Operating Category
    ///
    /// Abnormal operating performance category as specified in IEEE 1547-2018.
    abn_op_cat_rtg: Option<AbnOpCatRtg>,
    /// Supported Control Modes
    ///
    /// Supported control mode functions.
    ctrl_modes: Option<u32>,
    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    int_island_cat_rtg: Option<u16>,
    /// Active Power Max Setting
    ///
    /// Maximum active power setting used to adjust maximum active power setting.
    w_max: Option<u16>,
    /// Active Power (Over-Excited) Setting
    ///
    /// Active power setting at specified over-excited power factor in watts.
    w_max_ovr_ext: Option<u16>,
    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    w_ovr_ext_pf: Option<u16>,
    /// Active Power (Under-Excited) Setting
    ///
    /// Active power setting at specified under-excited power factor in watts.
    w_max_und_ext: Option<u16>,
    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    w_und_ext_pf: Option<u16>,
    /// Apparent Power Max Setting
    ///
    /// Maximum apparent power setting used to adjust maximum apparent power rating.
    va_max: Option<u16>,
    /// Reactive Power Injected Setting
    ///
    /// Maximum injected reactive power setting used to adjust maximum injected reactive power rating.
    var_max_inj: Option<u16>,
    /// Reactive Power Absorbed Setting
    ///
    /// Maximum absorbed reactive power setting used to adjust maximum absorbed reactive power rating.
    var_max_abs: Option<u16>,
    /// Charge Rate Max Setting
    ///
    /// Maximum active power charge rate setting used to adjust maximum active power charge rate rating.
    w_cha_rte_max: Option<u16>,
    /// Discharge Rate Max Setting
    ///
    /// Maximum active power discharge rate setting used to adjust maximum active power discharge rate rating.
    w_dis_cha_rte_max: Option<u16>,
    /// Charge Rate Max VA Setting
    ///
    /// Maximum apparent power charge rate setting used to adjust maximum apparent power charge rate rating.
    va_cha_rte_max: Option<u16>,
    /// Discharge Rate Max VA Setting
    ///
    /// Maximum apparent power discharge rate setting used to adjust maximum apparent power discharge rate rating.
    va_dis_cha_rte_max: Option<u16>,
    /// Nominal AC Voltage Setting
    ///
    /// Nominal AC voltage setting.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    v_nom: Option<u16>,
    /// AC Voltage Max Setting
    ///
    /// AC voltage maximum setting used to adjust AC voltage maximum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    v_max: Option<u16>,
    /// AC Voltage Min Setting
    ///
    /// AC voltage minimum setting used to adjust AC voltage minimum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    v_min: Option<u16>,
    /// AC Current Max Setting
    ///
    /// Maximum AC current setting used to adjust maximum AC current rating.
    a_max: Option<u16>,
    /// PF Over-Excited Setting (Unused)
    ///
    /// Unused. Please use WOvrExtPF.
    ///
    /// This point is duplicative of WOvrExtPF.
    pf_ovr_ext: Option<u16>,
    /// PF Under-Excited Setting (Unused)
    ///
    /// Unused. Please use WUndExtPF.
    ///
    /// This point is duplicative of WUndExtPF.
    pf_und_ext: Option<u16>,
    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    int_island_cat: Option<u16>,
    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    w_sf: Option<u16>,
    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    pf_sf: Option<u16>,
    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    va_sf: Option<u16>,
    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    var_sf: Option<u16>,
    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    v_sf: Option<u16>,
    /// Current Scale Factor
    ///
    /// Current scale factor.
    a_sf: Option<u16>,
    /// Susceptance Scale Factor
    ///
    /// Susceptance scale factor.
    s_sf: Option<u16>,
}

trait DerCapacityTrait {
    /// Model ID
    ///
    /// DER capacity model ID.
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// DER capacity model length.
    fn l(&self) -> u16;

    /// Active Power Max Rating
    ///
    /// Maximum active power rating at unity power factor in watts.
    fn w_max_rtg(&self) -> Option<u16> {
        None
    }

    /// Active Power (Over-Excited) Rating
    ///
    /// Active power rating at specified over-excited power factor in watts.
    fn w_ovr_ext_rtg(&self) -> Option<u16> {
        None
    }

    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    fn w_ovr_ext_rtg_pf(&self) -> Option<u16> {
        None
    }

    /// Active Power (Under-Excited) Rating
    ///
    /// Active power rating at specified under-excited power factor in watts.
    fn w_und_ext_rtg(&self) -> Option<u16> {
        None
    }

    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    fn w_und_ext_rtg_pf(&self) -> Option<u16> {
        None
    }

    /// Apparent Power Max Rating
    ///
    /// Maximum apparent power rating in voltamperes.
    fn va_max_rtg(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Injected Rating
    ///
    /// Maximum injected reactive power rating in vars.
    fn var_max_inj_rtg(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Absorbed Rating
    ///
    /// Maximum absorbed reactive power rating in vars.
    fn var_max_abs_rtg(&self) -> Option<u16> {
        None
    }

    /// Charge Rate Max Rating
    ///
    /// Maximum active power charge rate in watts.
    fn w_cha_rte_max_rtg(&self) -> Option<u16> {
        None
    }

    /// Discharge Rate Max Rating
    ///
    /// Maximum active power discharge rate in watts.
    fn w_dis_cha_rte_max_rtg(&self) -> Option<u16> {
        None
    }

    /// Charge Rate Max VA Rating
    ///
    /// Maximum apparent power charge rate in voltamperes.
    fn va_cha_rte_max_rtg(&self) -> Option<u16> {
        None
    }

    /// Discharge Rate Max VA Rating
    ///
    /// Maximum apparent power discharge rate in voltamperes.
    fn va_dis_cha_rte_max_rtg(&self) -> Option<u16> {
        None
    }

    /// AC Voltage Nominal Rating
    ///
    /// AC voltage nominal rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn v_nom_rtg(&self) -> Option<u16> {
        None
    }

    /// AC Voltage Max Rating
    ///
    /// AC voltage maximum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn v_max_rtg(&self) -> Option<u16> {
        None
    }

    /// AC Voltage Min Rating
    ///
    /// AC voltage minimum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn v_min_rtg(&self) -> Option<u16> {
        None
    }

    /// AC Current Max Rating
    ///
    /// AC current maximum rating in amps.
    fn a_max_rtg(&self) -> Option<u16> {
        None
    }

    /// PF Over-Excited Rating (Unused)
    ///
    /// Unused. Please use WOvrExtRtgPF.
    ///
    /// This point is duplicative of WOvrExtRtgPF.
    fn pf_ovr_ext_rtg(&self) -> Option<u16> {
        None
    }

    /// PF Under-Excited Rating (Unused)
    ///
    /// Unused. Please use WUndExtRtgPF.
    ///
    /// This point is duplicative of WUndExtRtgPF.
    fn pf_und_ext_rtg(&self) -> Option<u16> {
        None
    }

    /// Reactive Susceptance
    ///
    /// Reactive susceptance that remains connected to the Area EPS in the cease to energize and trip state.
    fn react_suscept_rtg(&self) -> Option<u16> {
        None
    }

    /// Normal Operating Category
    ///
    /// Normal operating performance category as specified in IEEE 1547-2018.
    fn nor_op_cat_rtg(&self) -> Option<NorOpCatRtg> {
        None
    }

    /// Abnormal Operating Category
    ///
    /// Abnormal operating performance category as specified in IEEE 1547-2018.
    fn abn_op_cat_rtg(&self) -> Option<AbnOpCatRtg> {
        None
    }

    /// Supported Control Modes
    ///
    /// Supported control mode functions.
    fn ctrl_modes(&self) -> Option<u32> {
        None
    }

    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    fn int_island_cat_rtg(&self) -> Option<u16> {
        None
    }

    /// Active Power Max Setting
    ///
    /// Maximum active power setting used to adjust maximum active power setting.
    fn w_max(&self) -> Option<u16> {
        None
    }

    /// Active Power Max Setting
    ///
    /// Maximum active power setting used to adjust maximum active power setting.
    fn set_w_max(&mut self, value: u16) {}

    /// Active Power (Over-Excited) Setting
    ///
    /// Active power setting at specified over-excited power factor in watts.
    fn w_max_ovr_ext(&self) -> Option<u16> {
        None
    }

    /// Active Power (Over-Excited) Setting
    ///
    /// Active power setting at specified over-excited power factor in watts.
    fn set_w_max_ovr_ext(&mut self, value: u16) {}

    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    fn w_ovr_ext_pf(&self) -> Option<u16> {
        None
    }

    /// Specified Over-Excited PF
    ///
    /// Specified over-excited power factor.
    fn set_w_ovr_ext_pf(&mut self, value: u16) {}

    /// Active Power (Under-Excited) Setting
    ///
    /// Active power setting at specified under-excited power factor in watts.
    fn w_max_und_ext(&self) -> Option<u16> {
        None
    }

    /// Active Power (Under-Excited) Setting
    ///
    /// Active power setting at specified under-excited power factor in watts.
    fn set_w_max_und_ext(&mut self, value: u16) {}

    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    fn w_und_ext_pf(&self) -> Option<u16> {
        None
    }

    /// Specified Under-Excited PF
    ///
    /// Specified under-excited power factor.
    fn set_w_und_ext_pf(&mut self, value: u16) {}

    /// Apparent Power Max Setting
    ///
    /// Maximum apparent power setting used to adjust maximum apparent power rating.
    fn va_max(&self) -> Option<u16> {
        None
    }

    /// Apparent Power Max Setting
    ///
    /// Maximum apparent power setting used to adjust maximum apparent power rating.
    fn set_va_max(&mut self, value: u16) {}

    /// Reactive Power Injected Setting
    ///
    /// Maximum injected reactive power setting used to adjust maximum injected reactive power rating.
    fn var_max_inj(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Injected Setting
    ///
    /// Maximum injected reactive power setting used to adjust maximum injected reactive power rating.
    fn set_var_max_inj(&mut self, value: u16) {}

    /// Reactive Power Absorbed Setting
    ///
    /// Maximum absorbed reactive power setting used to adjust maximum absorbed reactive power rating.
    fn var_max_abs(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Absorbed Setting
    ///
    /// Maximum absorbed reactive power setting used to adjust maximum absorbed reactive power rating.
    fn set_var_max_abs(&mut self, value: u16) {}

    /// Charge Rate Max Setting
    ///
    /// Maximum active power charge rate setting used to adjust maximum active power charge rate rating.
    fn w_cha_rte_max(&self) -> Option<u16> {
        None
    }

    /// Charge Rate Max Setting
    ///
    /// Maximum active power charge rate setting used to adjust maximum active power charge rate rating.
    fn set_w_cha_rte_max(&mut self, value: u16) {}

    /// Discharge Rate Max Setting
    ///
    /// Maximum active power discharge rate setting used to adjust maximum active power discharge rate rating.
    fn w_dis_cha_rte_max(&self) -> Option<u16> {
        None
    }

    /// Discharge Rate Max Setting
    ///
    /// Maximum active power discharge rate setting used to adjust maximum active power discharge rate rating.
    fn set_w_dis_cha_rte_max(&mut self, value: u16) {}

    /// Charge Rate Max VA Setting
    ///
    /// Maximum apparent power charge rate setting used to adjust maximum apparent power charge rate rating.
    fn va_cha_rte_max(&self) -> Option<u16> {
        None
    }

    /// Charge Rate Max VA Setting
    ///
    /// Maximum apparent power charge rate setting used to adjust maximum apparent power charge rate rating.
    fn set_va_cha_rte_max(&mut self, value: u16) {}

    /// Discharge Rate Max VA Setting
    ///
    /// Maximum apparent power discharge rate setting used to adjust maximum apparent power discharge rate rating.
    fn va_dis_cha_rte_max(&self) -> Option<u16> {
        None
    }

    /// Discharge Rate Max VA Setting
    ///
    /// Maximum apparent power discharge rate setting used to adjust maximum apparent power discharge rate rating.
    fn set_va_dis_cha_rte_max(&mut self, value: u16) {}

    /// Nominal AC Voltage Setting
    ///
    /// Nominal AC voltage setting.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn v_nom(&self) -> Option<u16> {
        None
    }

    /// Nominal AC Voltage Setting
    ///
    /// Nominal AC voltage setting.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn set_v_nom(&mut self, value: u16) {}

    /// AC Voltage Max Setting
    ///
    /// AC voltage maximum setting used to adjust AC voltage maximum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn v_max(&self) -> Option<u16> {
        None
    }

    /// AC Voltage Max Setting
    ///
    /// AC voltage maximum setting used to adjust AC voltage maximum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn set_v_max(&mut self, value: u16) {}

    /// AC Voltage Min Setting
    ///
    /// AC voltage minimum setting used to adjust AC voltage minimum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn v_min(&self) -> Option<u16> {
        None
    }

    /// AC Voltage Min Setting
    ///
    /// AC voltage minimum setting used to adjust AC voltage minimum rating.
    ///
    /// Voltages are LN for single phase DER (e.g. 120 V nominal), LL for split phase DER (e.g. 240 V nominal), and LL for three phase DER (e.g., 480 V nominal).
    fn set_v_min(&mut self, value: u16) {}

    /// AC Current Max Setting
    ///
    /// Maximum AC current setting used to adjust maximum AC current rating.
    fn a_max(&self) -> Option<u16> {
        None
    }

    /// AC Current Max Setting
    ///
    /// Maximum AC current setting used to adjust maximum AC current rating.
    fn set_a_max(&mut self, value: u16) {}

    /// PF Over-Excited Setting (Unused)
    ///
    /// Unused. Please use WOvrExtPF.
    ///
    /// This point is duplicative of WOvrExtPF.
    fn pf_ovr_ext(&self) -> Option<u16> {
        None
    }

    /// PF Over-Excited Setting (Unused)
    ///
    /// Unused. Please use WOvrExtPF.
    ///
    /// This point is duplicative of WOvrExtPF.
    fn set_pf_ovr_ext(&mut self, value: u16) {}

    /// PF Under-Excited Setting (Unused)
    ///
    /// Unused. Please use WUndExtPF.
    ///
    /// This point is duplicative of WUndExtPF.
    fn pf_und_ext(&self) -> Option<u16> {
        None
    }

    /// PF Under-Excited Setting (Unused)
    ///
    /// Unused. Please use WUndExtPF.
    ///
    /// This point is duplicative of WUndExtPF.
    fn set_pf_und_ext(&mut self, value: u16) {}

    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    fn int_island_cat(&self) -> Option<u16> {
        None
    }

    /// Intentional Island Categories
    ///
    /// Intentional island categories.
    fn set_int_island_cat(&mut self, value: u16) {}

    /// Active Power Scale Factor
    ///
    /// Active power scale factor.
    fn w_sf(&self) -> Option<u16> {
        None
    }

    /// Power Factor Scale Factor
    ///
    /// Power factor scale factor.
    fn pf_sf(&self) -> Option<u16> {
        None
    }

    /// Apparent Power Scale Factor
    ///
    /// Apparent power scale factor.
    fn va_sf(&self) -> Option<u16> {
        None
    }

    /// Reactive Power Scale Factor
    ///
    /// Reactive power scale factor.
    fn var_sf(&self) -> Option<u16> {
        None
    }

    /// Voltage Scale Factor
    ///
    /// Voltage scale factor.
    fn v_sf(&self) -> Option<u16> {
        None
    }

    /// Current Scale Factor
    ///
    /// Current scale factor.
    fn a_sf(&self) -> Option<u16> {
        None
    }

    /// Susceptance Scale Factor
    ///
    /// Susceptance scale factor.
    fn s_sf(&self) -> Option<u16> {
        None
    }
}

pub enum NorOpCatRtg {
    /// Category A
    CatA = 0,
    /// Category B
    CatB = 1,
}

pub enum AbnOpCatRtg {
    /// Category I
    Cat1 = 0,
    /// Category II
    Cat2 = 1,
    /// Category III
    Cat3 = 2,
}
