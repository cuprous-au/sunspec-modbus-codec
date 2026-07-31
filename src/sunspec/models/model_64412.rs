use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};

pub const SIZE: u16 = 45;

pub static POINTS: [ReadablePoint; 45] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64412 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 43 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::DaManipulation,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::FalsifyDeviceIdentity,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasPAlwaysNameplate,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasQAlwaysMinimum,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasQAlwaysMaximum,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasQAlwaysZero,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasZeroP,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasInvertQ,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasLowV,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasHighV,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasLowL1V,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasHighL1V,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasLowF,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasHighF,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasLowAmps,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasHighAmps,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasHighS,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasLowS,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasHighQ,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasLowQ,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasLowPf,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::MeasLowReversedPf,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateHighP,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateLowP,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateHighS,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateLowS,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateHighQ,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateLowQ,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateHighNomV,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateLowNomV,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateLowAmps,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateLowVarmaxinj,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateLowVarmaxabs,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::NameplateLowPf,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::SettingsHighNomV,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::SettingsLowAmps,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::SettingsHighP,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::SettingsLowP,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::SettingsHighVaMax,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::SettingsHighVarmaxinj,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::SettingsHighVarmaxabs,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::ChangeCommonModelId,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64412 {
            point: Point::ChangeCommonModelLength,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    DaManipulation,
    FalsifyDeviceIdentity,
    MeasPAlwaysNameplate,
    MeasQAlwaysMinimum,
    MeasQAlwaysMaximum,
    MeasQAlwaysZero,
    MeasZeroP,
    MeasInvertQ,
    MeasLowV,
    MeasHighV,
    MeasLowL1V,
    MeasHighL1V,
    MeasLowF,
    MeasHighF,
    MeasLowAmps,
    MeasHighAmps,
    MeasHighS,
    MeasLowS,
    MeasHighQ,
    MeasLowQ,
    MeasLowPf,
    MeasLowReversedPf,
    NameplateHighP,
    NameplateLowP,
    NameplateHighS,
    NameplateLowS,
    NameplateHighQ,
    NameplateLowQ,
    NameplateHighNomV,
    NameplateLowNomV,
    NameplateLowAmps,
    NameplateLowVarmaxinj,
    NameplateLowVarmaxabs,
    NameplateLowPf,
    SettingsHighNomV,
    SettingsLowAmps,
    SettingsHighP,
    SettingsLowP,
    SettingsHighVaMax,
    SettingsHighVarmaxinj,
    SettingsHighVarmaxabs,
    ChangeCommonModelId,
    ChangeCommonModelLength,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::DaManipulation => {
            if let Some(value) = model.da_manipulation() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::FalsifyDeviceIdentity => {
            if let Some(value) = model.falsify_device_identity() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasPAlwaysNameplate => {
            if let Some(value) = model.meas_p_always_nameplate() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasQAlwaysMinimum => {
            if let Some(value) = model.meas_q_always_minimum() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasQAlwaysMaximum => {
            if let Some(value) = model.meas_q_always_maximum() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasQAlwaysZero => {
            if let Some(value) = model.meas_q_always_zero() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasZeroP => {
            if let Some(value) = model.meas_zero_p() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasInvertQ => {
            if let Some(value) = model.meas_invert_q() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasLowV => {
            if let Some(value) = model.meas_low_v() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasHighV => {
            if let Some(value) = model.meas_high_v() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasLowL1V => {
            if let Some(value) = model.meas_low_l1_v() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasHighL1V => {
            if let Some(value) = model.meas_high_l1_v() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasLowF => {
            if let Some(value) = model.meas_low_f() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasHighF => {
            if let Some(value) = model.meas_high_f() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasLowAmps => {
            if let Some(value) = model.meas_low_amps() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasHighAmps => {
            if let Some(value) = model.meas_high_amps() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasHighS => {
            if let Some(value) = model.meas_high_s() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasLowS => {
            if let Some(value) = model.meas_low_s() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasHighQ => {
            if let Some(value) = model.meas_high_q() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasLowQ => {
            if let Some(value) = model.meas_low_q() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasLowPf => {
            if let Some(value) = model.meas_low_pf() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::MeasLowReversedPf => {
            if let Some(value) = model.meas_low_reversed_pf() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateHighP => {
            if let Some(value) = model.nameplate_high_p() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateLowP => {
            if let Some(value) = model.nameplate_low_p() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateHighS => {
            if let Some(value) = model.nameplate_high_s() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateLowS => {
            if let Some(value) = model.nameplate_low_s() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateHighQ => {
            if let Some(value) = model.nameplate_high_q() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateLowQ => {
            if let Some(value) = model.nameplate_low_q() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateHighNomV => {
            if let Some(value) = model.nameplate_high_nom_v() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateLowNomV => {
            if let Some(value) = model.nameplate_low_nom_v() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateLowAmps => {
            if let Some(value) = model.nameplate_low_amps() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateLowVarmaxinj => {
            if let Some(value) = model.nameplate_low_varmaxinj() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateLowVarmaxabs => {
            if let Some(value) = model.nameplate_low_varmaxabs() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::NameplateLowPf => {
            if let Some(value) = model.nameplate_low_pf() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SettingsHighNomV => {
            if let Some(value) = model.settings_high_nom_v() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SettingsLowAmps => {
            if let Some(value) = model.settings_low_amps() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SettingsHighP => {
            if let Some(value) = model.settings_high_p() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SettingsLowP => {
            if let Some(value) = model.settings_low_p() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SettingsHighVaMax => {
            if let Some(value) = model.settings_high_va_max() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SettingsHighVarmaxinj => {
            if let Some(value) = model.settings_high_varmaxinj() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SettingsHighVarmaxabs => {
            if let Some(value) = model.settings_high_varmaxabs() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::ChangeCommonModelId => {
            if let Some(value) = model.change_common_model_id() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::ChangeCommonModelLength => {
            if let Some(value) = model.change_common_model_length() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
    }
}

pub trait ModelAdapter {
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
    fn meas_low_l1_v(&self) -> Option<MeasLowL1v> {
        None
    }

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn set_meas_low_l1_v(&mut self, value: MeasLowL1v) {}

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn meas_high_l1_v(&self) -> Option<MeasHighL1v> {
        None
    }

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn set_meas_high_l1_v(&mut self, value: MeasHighL1v) {}

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

#[repr(C)]
pub struct Model64412CallbackAdapter {
    da_manipulation_callback: Option<extern "C" fn() -> DaManipulation>,
    set_da_manipulation_callback: Option<extern "C" fn(DaManipulation)>,
    falsify_device_identity_callback: Option<extern "C" fn() -> FalsifyDeviceIdentity>,
    set_falsify_device_identity_callback: Option<extern "C" fn(FalsifyDeviceIdentity)>,
    meas_p_always_nameplate_callback: Option<extern "C" fn() -> MeasPAlwaysNameplate>,
    set_meas_p_always_nameplate_callback: Option<extern "C" fn(MeasPAlwaysNameplate)>,
    meas_q_always_minimum_callback: Option<extern "C" fn() -> MeasQAlwaysMinimum>,
    set_meas_q_always_minimum_callback: Option<extern "C" fn(MeasQAlwaysMinimum)>,
    meas_q_always_maximum_callback: Option<extern "C" fn() -> MeasQAlwaysMaximum>,
    set_meas_q_always_maximum_callback: Option<extern "C" fn(MeasQAlwaysMaximum)>,
    meas_q_always_zero_callback: Option<extern "C" fn() -> MeasQAlwaysZero>,
    set_meas_q_always_zero_callback: Option<extern "C" fn(MeasQAlwaysZero)>,
    meas_zero_p_callback: Option<extern "C" fn() -> MeasZeroP>,
    set_meas_zero_p_callback: Option<extern "C" fn(MeasZeroP)>,
    meas_invert_q_callback: Option<extern "C" fn() -> MeasInvertQ>,
    set_meas_invert_q_callback: Option<extern "C" fn(MeasInvertQ)>,
    meas_low_v_callback: Option<extern "C" fn() -> MeasLowV>,
    set_meas_low_v_callback: Option<extern "C" fn(MeasLowV)>,
    meas_high_v_callback: Option<extern "C" fn() -> MeasHighV>,
    set_meas_high_v_callback: Option<extern "C" fn(MeasHighV)>,
    meas_low_l1_v_callback: Option<extern "C" fn() -> MeasLowL1v>,
    set_meas_low_l1_v_callback: Option<extern "C" fn(MeasLowL1v)>,
    meas_high_l1_v_callback: Option<extern "C" fn() -> MeasHighL1v>,
    set_meas_high_l1_v_callback: Option<extern "C" fn(MeasHighL1v)>,
    meas_low_f_callback: Option<extern "C" fn() -> MeasLowF>,
    set_meas_low_f_callback: Option<extern "C" fn(MeasLowF)>,
    meas_high_f_callback: Option<extern "C" fn() -> MeasHighF>,
    set_meas_high_f_callback: Option<extern "C" fn(MeasHighF)>,
    meas_low_amps_callback: Option<extern "C" fn() -> MeasLowAmps>,
    set_meas_low_amps_callback: Option<extern "C" fn(MeasLowAmps)>,
    meas_high_amps_callback: Option<extern "C" fn() -> MeasHighAmps>,
    set_meas_high_amps_callback: Option<extern "C" fn(MeasHighAmps)>,
    meas_high_s_callback: Option<extern "C" fn() -> MeasHighS>,
    set_meas_high_s_callback: Option<extern "C" fn(MeasHighS)>,
    meas_low_s_callback: Option<extern "C" fn() -> MeasLowS>,
    set_meas_low_s_callback: Option<extern "C" fn(MeasLowS)>,
    meas_high_q_callback: Option<extern "C" fn() -> MeasHighQ>,
    set_meas_high_q_callback: Option<extern "C" fn(MeasHighQ)>,
    meas_low_q_callback: Option<extern "C" fn() -> MeasLowQ>,
    set_meas_low_q_callback: Option<extern "C" fn(MeasLowQ)>,
    meas_low_pf_callback: Option<extern "C" fn() -> MeasLowPf>,
    set_meas_low_pf_callback: Option<extern "C" fn(MeasLowPf)>,
    meas_low_reversed_pf_callback: Option<extern "C" fn() -> MeasLowReversedPf>,
    set_meas_low_reversed_pf_callback: Option<extern "C" fn(MeasLowReversedPf)>,
    nameplate_high_p_callback: Option<extern "C" fn() -> NameplateHighP>,
    set_nameplate_high_p_callback: Option<extern "C" fn(NameplateHighP)>,
    nameplate_low_p_callback: Option<extern "C" fn() -> NameplateLowP>,
    set_nameplate_low_p_callback: Option<extern "C" fn(NameplateLowP)>,
    nameplate_high_s_callback: Option<extern "C" fn() -> NameplateHighS>,
    set_nameplate_high_s_callback: Option<extern "C" fn(NameplateHighS)>,
    nameplate_low_s_callback: Option<extern "C" fn() -> NameplateLowS>,
    set_nameplate_low_s_callback: Option<extern "C" fn(NameplateLowS)>,
    nameplate_high_q_callback: Option<extern "C" fn() -> NameplateHighQ>,
    set_nameplate_high_q_callback: Option<extern "C" fn(NameplateHighQ)>,
    nameplate_low_q_callback: Option<extern "C" fn() -> NameplateLowQ>,
    set_nameplate_low_q_callback: Option<extern "C" fn(NameplateLowQ)>,
    nameplate_high_nom_v_callback: Option<extern "C" fn() -> NameplateHighNomV>,
    set_nameplate_high_nom_v_callback: Option<extern "C" fn(NameplateHighNomV)>,
    nameplate_low_nom_v_callback: Option<extern "C" fn() -> NameplateLowNomV>,
    set_nameplate_low_nom_v_callback: Option<extern "C" fn(NameplateLowNomV)>,
    nameplate_low_amps_callback: Option<extern "C" fn() -> NameplateLowAmps>,
    set_nameplate_low_amps_callback: Option<extern "C" fn(NameplateLowAmps)>,
    nameplate_low_varmaxinj_callback: Option<extern "C" fn() -> NameplateLowVarmaxinj>,
    set_nameplate_low_varmaxinj_callback: Option<extern "C" fn(NameplateLowVarmaxinj)>,
    nameplate_low_varmaxabs_callback: Option<extern "C" fn() -> NameplateLowVarmaxabs>,
    set_nameplate_low_varmaxabs_callback: Option<extern "C" fn(NameplateLowVarmaxabs)>,
    nameplate_low_pf_callback: Option<extern "C" fn() -> NameplateLowPf>,
    set_nameplate_low_pf_callback: Option<extern "C" fn(NameplateLowPf)>,
    settings_high_nom_v_callback: Option<extern "C" fn() -> SettingsHighNomV>,
    set_settings_high_nom_v_callback: Option<extern "C" fn(SettingsHighNomV)>,
    settings_low_amps_callback: Option<extern "C" fn() -> SettingsLowAmps>,
    set_settings_low_amps_callback: Option<extern "C" fn(SettingsLowAmps)>,
    settings_high_p_callback: Option<extern "C" fn() -> SettingsHighP>,
    set_settings_high_p_callback: Option<extern "C" fn(SettingsHighP)>,
    settings_low_p_callback: Option<extern "C" fn() -> SettingsLowP>,
    set_settings_low_p_callback: Option<extern "C" fn(SettingsLowP)>,
    settings_high_va_max_callback: Option<extern "C" fn() -> SettingsHighVaMax>,
    set_settings_high_va_max_callback: Option<extern "C" fn(SettingsHighVaMax)>,
    settings_high_varmaxinj_callback: Option<extern "C" fn() -> SettingsHighVarmaxinj>,
    set_settings_high_varmaxinj_callback: Option<extern "C" fn(SettingsHighVarmaxinj)>,
    settings_high_varmaxabs_callback: Option<extern "C" fn() -> SettingsHighVarmaxabs>,
    set_settings_high_varmaxabs_callback: Option<extern "C" fn(SettingsHighVarmaxabs)>,
    change_common_model_id_callback: Option<extern "C" fn() -> ChangeCommonModelId>,
    set_change_common_model_id_callback: Option<extern "C" fn(ChangeCommonModelId)>,
    change_common_model_length_callback: Option<extern "C" fn() -> ChangeCommonModelLength>,
    set_change_common_model_length_callback: Option<extern "C" fn(ChangeCommonModelLength)>,
}

impl ModelAdapter for Model64412CallbackAdapter {
    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    fn da_manipulation(&self) -> Option<DaManipulation> {
        self.da_manipulation_callback.map(|callback| (callback)())
    }

    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    fn set_da_manipulation(&mut self, value: DaManipulation) {
        if let Some(callback) = self.set_da_manipulation_callback {
            (callback)(value);
        };
    }

    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    fn falsify_device_identity(&self) -> Option<FalsifyDeviceIdentity> {
        self.falsify_device_identity_callback
            .map(|callback| (callback)())
    }

    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    fn set_falsify_device_identity(&mut self, value: FalsifyDeviceIdentity) {
        if let Some(callback) = self.set_falsify_device_identity_callback {
            (callback)(value);
        };
    }

    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    fn meas_p_always_nameplate(&self) -> Option<MeasPAlwaysNameplate> {
        self.meas_p_always_nameplate_callback
            .map(|callback| (callback)())
    }

    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    fn set_meas_p_always_nameplate(&mut self, value: MeasPAlwaysNameplate) {
        if let Some(callback) = self.set_meas_p_always_nameplate_callback {
            (callback)(value);
        };
    }

    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    fn meas_q_always_minimum(&self) -> Option<MeasQAlwaysMinimum> {
        self.meas_q_always_minimum_callback
            .map(|callback| (callback)())
    }

    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    fn set_meas_q_always_minimum(&mut self, value: MeasQAlwaysMinimum) {
        if let Some(callback) = self.set_meas_q_always_minimum_callback {
            (callback)(value);
        };
    }

    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    fn meas_q_always_maximum(&self) -> Option<MeasQAlwaysMaximum> {
        self.meas_q_always_maximum_callback
            .map(|callback| (callback)())
    }

    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    fn set_meas_q_always_maximum(&mut self, value: MeasQAlwaysMaximum) {
        if let Some(callback) = self.set_meas_q_always_maximum_callback {
            (callback)(value);
        };
    }

    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    fn meas_q_always_zero(&self) -> Option<MeasQAlwaysZero> {
        self.meas_q_always_zero_callback
            .map(|callback| (callback)())
    }

    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    fn set_meas_q_always_zero(&mut self, value: MeasQAlwaysZero) {
        if let Some(callback) = self.set_meas_q_always_zero_callback {
            (callback)(value);
        };
    }

    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    fn meas_zero_p(&self) -> Option<MeasZeroP> {
        self.meas_zero_p_callback.map(|callback| (callback)())
    }

    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    fn set_meas_zero_p(&mut self, value: MeasZeroP) {
        if let Some(callback) = self.set_meas_zero_p_callback {
            (callback)(value);
        };
    }

    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    fn meas_invert_q(&self) -> Option<MeasInvertQ> {
        self.meas_invert_q_callback.map(|callback| (callback)())
    }

    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    fn set_meas_invert_q(&mut self, value: MeasInvertQ) {
        if let Some(callback) = self.set_meas_invert_q_callback {
            (callback)(value);
        };
    }

    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    fn meas_low_v(&self) -> Option<MeasLowV> {
        self.meas_low_v_callback.map(|callback| (callback)())
    }

    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    fn set_meas_low_v(&mut self, value: MeasLowV) {
        if let Some(callback) = self.set_meas_low_v_callback {
            (callback)(value);
        };
    }

    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    fn meas_high_v(&self) -> Option<MeasHighV> {
        self.meas_high_v_callback.map(|callback| (callback)())
    }

    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    fn set_meas_high_v(&mut self, value: MeasHighV) {
        if let Some(callback) = self.set_meas_high_v_callback {
            (callback)(value);
        };
    }

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn meas_low_l1_v(&self) -> Option<MeasLowL1v> {
        self.meas_low_l1_v_callback.map(|callback| (callback)())
    }

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn set_meas_low_l1_v(&mut self, value: MeasLowL1v) {
        if let Some(callback) = self.set_meas_low_l1_v_callback {
            (callback)(value);
        };
    }

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn meas_high_l1_v(&self) -> Option<MeasHighL1v> {
        self.meas_high_l1_v_callback.map(|callback| (callback)())
    }

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn set_meas_high_l1_v(&mut self, value: MeasHighL1v) {
        if let Some(callback) = self.set_meas_high_l1_v_callback {
            (callback)(value);
        };
    }

    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    fn meas_low_f(&self) -> Option<MeasLowF> {
        self.meas_low_f_callback.map(|callback| (callback)())
    }

    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    fn set_meas_low_f(&mut self, value: MeasLowF) {
        if let Some(callback) = self.set_meas_low_f_callback {
            (callback)(value);
        };
    }

    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    fn meas_high_f(&self) -> Option<MeasHighF> {
        self.meas_high_f_callback.map(|callback| (callback)())
    }

    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    fn set_meas_high_f(&mut self, value: MeasHighF) {
        if let Some(callback) = self.set_meas_high_f_callback {
            (callback)(value);
        };
    }

    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    fn meas_low_amps(&self) -> Option<MeasLowAmps> {
        self.meas_low_amps_callback.map(|callback| (callback)())
    }

    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    fn set_meas_low_amps(&mut self, value: MeasLowAmps) {
        if let Some(callback) = self.set_meas_low_amps_callback {
            (callback)(value);
        };
    }

    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    fn meas_high_amps(&self) -> Option<MeasHighAmps> {
        self.meas_high_amps_callback.map(|callback| (callback)())
    }

    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    fn set_meas_high_amps(&mut self, value: MeasHighAmps) {
        if let Some(callback) = self.set_meas_high_amps_callback {
            (callback)(value);
        };
    }

    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    fn meas_high_s(&self) -> Option<MeasHighS> {
        self.meas_high_s_callback.map(|callback| (callback)())
    }

    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    fn set_meas_high_s(&mut self, value: MeasHighS) {
        if let Some(callback) = self.set_meas_high_s_callback {
            (callback)(value);
        };
    }

    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    fn meas_low_s(&self) -> Option<MeasLowS> {
        self.meas_low_s_callback.map(|callback| (callback)())
    }

    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    fn set_meas_low_s(&mut self, value: MeasLowS) {
        if let Some(callback) = self.set_meas_low_s_callback {
            (callback)(value);
        };
    }

    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    fn meas_high_q(&self) -> Option<MeasHighQ> {
        self.meas_high_q_callback.map(|callback| (callback)())
    }

    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    fn set_meas_high_q(&mut self, value: MeasHighQ) {
        if let Some(callback) = self.set_meas_high_q_callback {
            (callback)(value);
        };
    }

    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    fn meas_low_q(&self) -> Option<MeasLowQ> {
        self.meas_low_q_callback.map(|callback| (callback)())
    }

    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    fn set_meas_low_q(&mut self, value: MeasLowQ) {
        if let Some(callback) = self.set_meas_low_q_callback {
            (callback)(value);
        };
    }

    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    fn meas_low_pf(&self) -> Option<MeasLowPf> {
        self.meas_low_pf_callback.map(|callback| (callback)())
    }

    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    fn set_meas_low_pf(&mut self, value: MeasLowPf) {
        if let Some(callback) = self.set_meas_low_pf_callback {
            (callback)(value);
        };
    }

    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    fn meas_low_reversed_pf(&self) -> Option<MeasLowReversedPf> {
        self.meas_low_reversed_pf_callback
            .map(|callback| (callback)())
    }

    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    fn set_meas_low_reversed_pf(&mut self, value: MeasLowReversedPf) {
        if let Some(callback) = self.set_meas_low_reversed_pf_callback {
            (callback)(value);
        };
    }

    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    fn nameplate_high_p(&self) -> Option<NameplateHighP> {
        self.nameplate_high_p_callback.map(|callback| (callback)())
    }

    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    fn set_nameplate_high_p(&mut self, value: NameplateHighP) {
        if let Some(callback) = self.set_nameplate_high_p_callback {
            (callback)(value);
        };
    }

    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    fn nameplate_low_p(&self) -> Option<NameplateLowP> {
        self.nameplate_low_p_callback.map(|callback| (callback)())
    }

    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    fn set_nameplate_low_p(&mut self, value: NameplateLowP) {
        if let Some(callback) = self.set_nameplate_low_p_callback {
            (callback)(value);
        };
    }

    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    fn nameplate_high_s(&self) -> Option<NameplateHighS> {
        self.nameplate_high_s_callback.map(|callback| (callback)())
    }

    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    fn set_nameplate_high_s(&mut self, value: NameplateHighS) {
        if let Some(callback) = self.set_nameplate_high_s_callback {
            (callback)(value);
        };
    }

    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    fn nameplate_low_s(&self) -> Option<NameplateLowS> {
        self.nameplate_low_s_callback.map(|callback| (callback)())
    }

    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    fn set_nameplate_low_s(&mut self, value: NameplateLowS) {
        if let Some(callback) = self.set_nameplate_low_s_callback {
            (callback)(value);
        };
    }

    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    fn nameplate_high_q(&self) -> Option<NameplateHighQ> {
        self.nameplate_high_q_callback.map(|callback| (callback)())
    }

    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    fn set_nameplate_high_q(&mut self, value: NameplateHighQ) {
        if let Some(callback) = self.set_nameplate_high_q_callback {
            (callback)(value);
        };
    }

    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    fn nameplate_low_q(&self) -> Option<NameplateLowQ> {
        self.nameplate_low_q_callback.map(|callback| (callback)())
    }

    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    fn set_nameplate_low_q(&mut self, value: NameplateLowQ) {
        if let Some(callback) = self.set_nameplate_low_q_callback {
            (callback)(value);
        };
    }

    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    fn nameplate_high_nom_v(&self) -> Option<NameplateHighNomV> {
        self.nameplate_high_nom_v_callback
            .map(|callback| (callback)())
    }

    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    fn set_nameplate_high_nom_v(&mut self, value: NameplateHighNomV) {
        if let Some(callback) = self.set_nameplate_high_nom_v_callback {
            (callback)(value);
        };
    }

    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    fn nameplate_low_nom_v(&self) -> Option<NameplateLowNomV> {
        self.nameplate_low_nom_v_callback
            .map(|callback| (callback)())
    }

    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    fn set_nameplate_low_nom_v(&mut self, value: NameplateLowNomV) {
        if let Some(callback) = self.set_nameplate_low_nom_v_callback {
            (callback)(value);
        };
    }

    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    fn nameplate_low_amps(&self) -> Option<NameplateLowAmps> {
        self.nameplate_low_amps_callback
            .map(|callback| (callback)())
    }

    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    fn set_nameplate_low_amps(&mut self, value: NameplateLowAmps) {
        if let Some(callback) = self.set_nameplate_low_amps_callback {
            (callback)(value);
        };
    }

    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    fn nameplate_low_varmaxinj(&self) -> Option<NameplateLowVarmaxinj> {
        self.nameplate_low_varmaxinj_callback
            .map(|callback| (callback)())
    }

    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    fn set_nameplate_low_varmaxinj(&mut self, value: NameplateLowVarmaxinj) {
        if let Some(callback) = self.set_nameplate_low_varmaxinj_callback {
            (callback)(value);
        };
    }

    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    fn nameplate_low_varmaxabs(&self) -> Option<NameplateLowVarmaxabs> {
        self.nameplate_low_varmaxabs_callback
            .map(|callback| (callback)())
    }

    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    fn set_nameplate_low_varmaxabs(&mut self, value: NameplateLowVarmaxabs) {
        if let Some(callback) = self.set_nameplate_low_varmaxabs_callback {
            (callback)(value);
        };
    }

    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    fn nameplate_low_pf(&self) -> Option<NameplateLowPf> {
        self.nameplate_low_pf_callback.map(|callback| (callback)())
    }

    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    fn set_nameplate_low_pf(&mut self, value: NameplateLowPf) {
        if let Some(callback) = self.set_nameplate_low_pf_callback {
            (callback)(value);
        };
    }

    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    fn settings_high_nom_v(&self) -> Option<SettingsHighNomV> {
        self.settings_high_nom_v_callback
            .map(|callback| (callback)())
    }

    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    fn set_settings_high_nom_v(&mut self, value: SettingsHighNomV) {
        if let Some(callback) = self.set_settings_high_nom_v_callback {
            (callback)(value);
        };
    }

    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    fn settings_low_amps(&self) -> Option<SettingsLowAmps> {
        self.settings_low_amps_callback.map(|callback| (callback)())
    }

    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    fn set_settings_low_amps(&mut self, value: SettingsLowAmps) {
        if let Some(callback) = self.set_settings_low_amps_callback {
            (callback)(value);
        };
    }

    /// Settings High P
    ///
    /// Set the DER settings power to be high
    fn settings_high_p(&self) -> Option<SettingsHighP> {
        self.settings_high_p_callback.map(|callback| (callback)())
    }

    /// Settings High P
    ///
    /// Set the DER settings power to be high
    fn set_settings_high_p(&mut self, value: SettingsHighP) {
        if let Some(callback) = self.set_settings_high_p_callback {
            (callback)(value);
        };
    }

    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    fn settings_low_p(&self) -> Option<SettingsLowP> {
        self.settings_low_p_callback.map(|callback| (callback)())
    }

    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    fn set_settings_low_p(&mut self, value: SettingsLowP) {
        if let Some(callback) = self.set_settings_low_p_callback {
            (callback)(value);
        };
    }

    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    fn settings_high_va_max(&self) -> Option<SettingsHighVaMax> {
        self.settings_high_va_max_callback
            .map(|callback| (callback)())
    }

    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    fn set_settings_high_va_max(&mut self, value: SettingsHighVaMax) {
        if let Some(callback) = self.set_settings_high_va_max_callback {
            (callback)(value);
        };
    }

    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    fn settings_high_varmaxinj(&self) -> Option<SettingsHighVarmaxinj> {
        self.settings_high_varmaxinj_callback
            .map(|callback| (callback)())
    }

    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    fn set_settings_high_varmaxinj(&mut self, value: SettingsHighVarmaxinj) {
        if let Some(callback) = self.set_settings_high_varmaxinj_callback {
            (callback)(value);
        };
    }

    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    fn settings_high_varmaxabs(&self) -> Option<SettingsHighVarmaxabs> {
        self.settings_high_varmaxabs_callback
            .map(|callback| (callback)())
    }

    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    fn set_settings_high_varmaxabs(&mut self, value: SettingsHighVarmaxabs) {
        if let Some(callback) = self.set_settings_high_varmaxabs_callback {
            (callback)(value);
        };
    }

    /// Change Common Model ID
    ///
    /// Change the common model ID
    fn change_common_model_id(&self) -> Option<ChangeCommonModelId> {
        self.change_common_model_id_callback
            .map(|callback| (callback)())
    }

    /// Change Common Model ID
    ///
    /// Change the common model ID
    fn set_change_common_model_id(&mut self, value: ChangeCommonModelId) {
        if let Some(callback) = self.set_change_common_model_id_callback {
            (callback)(value);
        };
    }

    /// Change Common Model Length
    ///
    /// Change the common model length
    fn change_common_model_length(&self) -> Option<ChangeCommonModelLength> {
        self.change_common_model_length_callback
            .map(|callback| (callback)())
    }

    /// Change Common Model Length
    ///
    /// Change the common model length
    fn set_change_common_model_length(&mut self, value: ChangeCommonModelLength) {
        if let Some(callback) = self.set_change_common_model_length_callback {
            (callback)(value);
        };
    }
}
