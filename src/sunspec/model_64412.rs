pub type Model64412 = DerExploitation;

/// Operations that represent a hacked DER device
pub struct DerExploitation {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length.
    l: u16,
    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    da_manipulation: Option<DaManipulation>,
    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    falsify_device_identity: Option<FalsifyDeviceIdentity>,
    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    meas_p_always_nameplate: Option<MeasPAlwaysNameplate>,
    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    meas_q_always_minimum: Option<MeasQAlwaysMinimum>,
    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    meas_q_always_maximum: Option<MeasQAlwaysMaximum>,
    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    meas_q_always_zero: Option<MeasQAlwaysZero>,
    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    meas_zero_p: Option<MeasZeroP>,
    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    meas_invert_q: Option<MeasInvertQ>,
    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    meas_low_v: Option<MeasLowV>,
    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    meas_high_v: Option<MeasHighV>,
    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    meas_low_l1v: Option<MeasLowL1v>,
    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    meas_high_l1v: Option<MeasHighL1v>,
    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    meas_low_f: Option<MeasLowF>,
    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    meas_high_f: Option<MeasHighF>,
    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    meas_low_amps: Option<MeasLowAmps>,
    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    meas_high_amps: Option<MeasHighAmps>,
    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    meas_high_s: Option<MeasHighS>,
    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    meas_low_s: Option<MeasLowS>,
    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    meas_high_q: Option<MeasHighQ>,
    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    meas_low_q: Option<MeasLowQ>,
    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    meas_low_pf: Option<MeasLowPf>,
    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    meas_low_reversed_pf: Option<MeasLowReversedPf>,
    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    nameplate_high_p: Option<NameplateHighP>,
    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    nameplate_low_p: Option<NameplateLowP>,
    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    nameplate_high_s: Option<NameplateHighS>,
    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    nameplate_low_s: Option<NameplateLowS>,
    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    nameplate_high_q: Option<NameplateHighQ>,
    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    nameplate_low_q: Option<NameplateLowQ>,
    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    nameplate_high_nom_v: Option<NameplateHighNomV>,
    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    nameplate_low_nom_v: Option<NameplateLowNomV>,
    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    nameplate_low_amps: Option<NameplateLowAmps>,
    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    nameplate_low_varmaxinj: Option<NameplateLowVarmaxinj>,
    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    nameplate_low_varmaxabs: Option<NameplateLowVarmaxabs>,
    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    nameplate_low_pf: Option<NameplateLowPf>,
    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    settings_high_nom_v: Option<SettingsHighNomV>,
    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    settings_low_amps: Option<SettingsLowAmps>,
    /// Settings High P
    ///
    /// Set the DER settings power to be high
    settings_high_p: Option<SettingsHighP>,
    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    settings_low_p: Option<SettingsLowP>,
    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    settings_high_va_max: Option<SettingsHighVaMax>,
    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    settings_high_varmaxinj: Option<SettingsHighVarmaxinj>,
    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    settings_high_varmaxabs: Option<SettingsHighVarmaxabs>,
    /// Change Common Model ID
    ///
    /// Change the common model ID
    change_common_model_id: Option<ChangeCommonModelId>,
    /// Change Common Model Length
    ///
    /// Change the common model length
    change_common_model_length: Option<ChangeCommonModelLength>,
}

trait DerExploitationTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length.
    fn l(&self) -> u16;

    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    fn da_manipulation(&self) -> Option<DaManipulation> {
        None
    }

    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    fn set_da_manipulation(&mut self, value: DaManipulation) {}

    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    fn falsify_device_identity(&self) -> Option<FalsifyDeviceIdentity> {
        None
    }

    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    fn set_falsify_device_identity(&mut self, value: FalsifyDeviceIdentity) {}

    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    fn meas_p_always_nameplate(&self) -> Option<MeasPAlwaysNameplate> {
        None
    }

    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    fn set_meas_p_always_nameplate(&mut self, value: MeasPAlwaysNameplate) {}

    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    fn meas_q_always_minimum(&self) -> Option<MeasQAlwaysMinimum> {
        None
    }

    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    fn set_meas_q_always_minimum(&mut self, value: MeasQAlwaysMinimum) {}

    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    fn meas_q_always_maximum(&self) -> Option<MeasQAlwaysMaximum> {
        None
    }

    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    fn set_meas_q_always_maximum(&mut self, value: MeasQAlwaysMaximum) {}

    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    fn meas_q_always_zero(&self) -> Option<MeasQAlwaysZero> {
        None
    }

    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    fn set_meas_q_always_zero(&mut self, value: MeasQAlwaysZero) {}

    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    fn meas_zero_p(&self) -> Option<MeasZeroP> {
        None
    }

    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    fn set_meas_zero_p(&mut self, value: MeasZeroP) {}

    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    fn meas_invert_q(&self) -> Option<MeasInvertQ> {
        None
    }

    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    fn set_meas_invert_q(&mut self, value: MeasInvertQ) {}

    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    fn meas_low_v(&self) -> Option<MeasLowV> {
        None
    }

    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    fn set_meas_low_v(&mut self, value: MeasLowV) {}

    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    fn meas_high_v(&self) -> Option<MeasHighV> {
        None
    }

    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    fn set_meas_high_v(&mut self, value: MeasHighV) {}

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn meas_low_l1v(&self) -> Option<MeasLowL1v> {
        None
    }

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn set_meas_low_l1v(&mut self, value: MeasLowL1v) {}

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn meas_high_l1v(&self) -> Option<MeasHighL1v> {
        None
    }

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn set_meas_high_l1v(&mut self, value: MeasHighL1v) {}

    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    fn meas_low_f(&self) -> Option<MeasLowF> {
        None
    }

    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    fn set_meas_low_f(&mut self, value: MeasLowF) {}

    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    fn meas_high_f(&self) -> Option<MeasHighF> {
        None
    }

    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    fn set_meas_high_f(&mut self, value: MeasHighF) {}

    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    fn meas_low_amps(&self) -> Option<MeasLowAmps> {
        None
    }

    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    fn set_meas_low_amps(&mut self, value: MeasLowAmps) {}

    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    fn meas_high_amps(&self) -> Option<MeasHighAmps> {
        None
    }

    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    fn set_meas_high_amps(&mut self, value: MeasHighAmps) {}

    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    fn meas_high_s(&self) -> Option<MeasHighS> {
        None
    }

    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    fn set_meas_high_s(&mut self, value: MeasHighS) {}

    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    fn meas_low_s(&self) -> Option<MeasLowS> {
        None
    }

    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    fn set_meas_low_s(&mut self, value: MeasLowS) {}

    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    fn meas_high_q(&self) -> Option<MeasHighQ> {
        None
    }

    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    fn set_meas_high_q(&mut self, value: MeasHighQ) {}

    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    fn meas_low_q(&self) -> Option<MeasLowQ> {
        None
    }

    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    fn set_meas_low_q(&mut self, value: MeasLowQ) {}

    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    fn meas_low_pf(&self) -> Option<MeasLowPf> {
        None
    }

    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    fn set_meas_low_pf(&mut self, value: MeasLowPf) {}

    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    fn meas_low_reversed_pf(&self) -> Option<MeasLowReversedPf> {
        None
    }

    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    fn set_meas_low_reversed_pf(&mut self, value: MeasLowReversedPf) {}

    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    fn nameplate_high_p(&self) -> Option<NameplateHighP> {
        None
    }

    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    fn set_nameplate_high_p(&mut self, value: NameplateHighP) {}

    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    fn nameplate_low_p(&self) -> Option<NameplateLowP> {
        None
    }

    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    fn set_nameplate_low_p(&mut self, value: NameplateLowP) {}

    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    fn nameplate_high_s(&self) -> Option<NameplateHighS> {
        None
    }

    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    fn set_nameplate_high_s(&mut self, value: NameplateHighS) {}

    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    fn nameplate_low_s(&self) -> Option<NameplateLowS> {
        None
    }

    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    fn set_nameplate_low_s(&mut self, value: NameplateLowS) {}

    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    fn nameplate_high_q(&self) -> Option<NameplateHighQ> {
        None
    }

    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    fn set_nameplate_high_q(&mut self, value: NameplateHighQ) {}

    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    fn nameplate_low_q(&self) -> Option<NameplateLowQ> {
        None
    }

    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    fn set_nameplate_low_q(&mut self, value: NameplateLowQ) {}

    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    fn nameplate_high_nom_v(&self) -> Option<NameplateHighNomV> {
        None
    }

    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    fn set_nameplate_high_nom_v(&mut self, value: NameplateHighNomV) {}

    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    fn nameplate_low_nom_v(&self) -> Option<NameplateLowNomV> {
        None
    }

    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    fn set_nameplate_low_nom_v(&mut self, value: NameplateLowNomV) {}

    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    fn nameplate_low_amps(&self) -> Option<NameplateLowAmps> {
        None
    }

    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    fn set_nameplate_low_amps(&mut self, value: NameplateLowAmps) {}

    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    fn nameplate_low_varmaxinj(&self) -> Option<NameplateLowVarmaxinj> {
        None
    }

    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    fn set_nameplate_low_varmaxinj(&mut self, value: NameplateLowVarmaxinj) {}

    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    fn nameplate_low_varmaxabs(&self) -> Option<NameplateLowVarmaxabs> {
        None
    }

    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    fn set_nameplate_low_varmaxabs(&mut self, value: NameplateLowVarmaxabs) {}

    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    fn nameplate_low_pf(&self) -> Option<NameplateLowPf> {
        None
    }

    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    fn set_nameplate_low_pf(&mut self, value: NameplateLowPf) {}

    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    fn settings_high_nom_v(&self) -> Option<SettingsHighNomV> {
        None
    }

    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    fn set_settings_high_nom_v(&mut self, value: SettingsHighNomV) {}

    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    fn settings_low_amps(&self) -> Option<SettingsLowAmps> {
        None
    }

    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    fn set_settings_low_amps(&mut self, value: SettingsLowAmps) {}

    /// Settings High P
    ///
    /// Set the DER settings power to be high
    fn settings_high_p(&self) -> Option<SettingsHighP> {
        None
    }

    /// Settings High P
    ///
    /// Set the DER settings power to be high
    fn set_settings_high_p(&mut self, value: SettingsHighP) {}

    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    fn settings_low_p(&self) -> Option<SettingsLowP> {
        None
    }

    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    fn set_settings_low_p(&mut self, value: SettingsLowP) {}

    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    fn settings_high_va_max(&self) -> Option<SettingsHighVaMax> {
        None
    }

    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    fn set_settings_high_va_max(&mut self, value: SettingsHighVaMax) {}

    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    fn settings_high_varmaxinj(&self) -> Option<SettingsHighVarmaxinj> {
        None
    }

    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    fn set_settings_high_varmaxinj(&mut self, value: SettingsHighVarmaxinj) {}

    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    fn settings_high_varmaxabs(&self) -> Option<SettingsHighVarmaxabs> {
        None
    }

    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    fn set_settings_high_varmaxabs(&mut self, value: SettingsHighVarmaxabs) {}

    /// Change Common Model ID
    ///
    /// Change the common model ID
    fn change_common_model_id(&self) -> Option<ChangeCommonModelId> {
        None
    }

    /// Change Common Model ID
    ///
    /// Change the common model ID
    fn set_change_common_model_id(&mut self, value: ChangeCommonModelId) {}

    /// Change Common Model Length
    ///
    /// Change the common model length
    fn change_common_model_length(&self) -> Option<ChangeCommonModelLength> {
        None
    }

    /// Change Common Model Length
    ///
    /// Change the common model length
    fn set_change_common_model_length(&mut self, value: ChangeCommonModelLength) {}
}

pub enum DaManipulation {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum FalsifyDeviceIdentity {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasPAlwaysNameplate {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasQAlwaysMinimum {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasQAlwaysMaximum {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasQAlwaysZero {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasZeroP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasInvertQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasLowV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasHighV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasLowL1v {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasHighL1v {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasLowF {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasHighF {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasHighAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasHighS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasLowS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasHighQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasLowQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasLowPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum MeasLowReversedPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateHighP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateLowP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateHighS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateLowS {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateHighQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateLowQ {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateHighNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateLowNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateLowVarmaxinj {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateLowVarmaxabs {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum NameplateLowPf {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum SettingsHighNomV {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum SettingsLowAmps {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum SettingsHighP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum SettingsLowP {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum SettingsHighVaMax {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum SettingsHighVarmaxinj {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum SettingsHighVarmaxabs {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum ChangeCommonModelId {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}

pub enum ChangeCommonModelLength {
    /// Data Unaffected
    ///
    /// Modbus Falsification Disabled
    Off = 0,
    /// Data Falsification
    ///
    /// Modbus Falsification Enabled
    On = 1,
}
