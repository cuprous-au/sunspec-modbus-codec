use crate::buffer::{self, ModbusBuffer};
use core::cmp::min;
use core::ffi::c_void;

pub const SIZE: u16 = 45;

static POINTS: [PointDetails<()>; 45] = [
    PointDetails {
        point: |()| Point::ModelId,
        size: 1,
        start_address: 0,
    },
    PointDetails {
        point: |()| Point::ModelLength,
        size: 1,
        start_address: 1,
    },
    PointDetails {
        point: |()| Point::DaManipulation,
        size: 1,
        start_address: 2,
    },
    PointDetails {
        point: |()| Point::FalsifyDeviceIdentity,
        size: 1,
        start_address: 3,
    },
    PointDetails {
        point: |()| Point::MeasPAlwaysNameplate,
        size: 1,
        start_address: 4,
    },
    PointDetails {
        point: |()| Point::MeasQAlwaysMinimum,
        size: 1,
        start_address: 5,
    },
    PointDetails {
        point: |()| Point::MeasQAlwaysMaximum,
        size: 1,
        start_address: 6,
    },
    PointDetails {
        point: |()| Point::MeasQAlwaysZero,
        size: 1,
        start_address: 7,
    },
    PointDetails {
        point: |()| Point::MeasZeroP,
        size: 1,
        start_address: 8,
    },
    PointDetails {
        point: |()| Point::MeasInvertQ,
        size: 1,
        start_address: 9,
    },
    PointDetails {
        point: |()| Point::MeasLowV,
        size: 1,
        start_address: 10,
    },
    PointDetails {
        point: |()| Point::MeasHighV,
        size: 1,
        start_address: 11,
    },
    PointDetails {
        point: |()| Point::MeasLowL1V,
        size: 1,
        start_address: 12,
    },
    PointDetails {
        point: |()| Point::MeasHighL1V,
        size: 1,
        start_address: 13,
    },
    PointDetails {
        point: |()| Point::MeasLowF,
        size: 1,
        start_address: 14,
    },
    PointDetails {
        point: |()| Point::MeasHighF,
        size: 1,
        start_address: 15,
    },
    PointDetails {
        point: |()| Point::MeasLowAmps,
        size: 1,
        start_address: 16,
    },
    PointDetails {
        point: |()| Point::MeasHighAmps,
        size: 1,
        start_address: 17,
    },
    PointDetails {
        point: |()| Point::MeasHighS,
        size: 1,
        start_address: 18,
    },
    PointDetails {
        point: |()| Point::MeasLowS,
        size: 1,
        start_address: 19,
    },
    PointDetails {
        point: |()| Point::MeasHighQ,
        size: 1,
        start_address: 20,
    },
    PointDetails {
        point: |()| Point::MeasLowQ,
        size: 1,
        start_address: 21,
    },
    PointDetails {
        point: |()| Point::MeasLowPf,
        size: 1,
        start_address: 22,
    },
    PointDetails {
        point: |()| Point::MeasLowReversedPf,
        size: 1,
        start_address: 23,
    },
    PointDetails {
        point: |()| Point::NameplateHighP,
        size: 1,
        start_address: 24,
    },
    PointDetails {
        point: |()| Point::NameplateLowP,
        size: 1,
        start_address: 25,
    },
    PointDetails {
        point: |()| Point::NameplateHighS,
        size: 1,
        start_address: 26,
    },
    PointDetails {
        point: |()| Point::NameplateLowS,
        size: 1,
        start_address: 27,
    },
    PointDetails {
        point: |()| Point::NameplateHighQ,
        size: 1,
        start_address: 28,
    },
    PointDetails {
        point: |()| Point::NameplateLowQ,
        size: 1,
        start_address: 29,
    },
    PointDetails {
        point: |()| Point::NameplateHighNomV,
        size: 1,
        start_address: 30,
    },
    PointDetails {
        point: |()| Point::NameplateLowNomV,
        size: 1,
        start_address: 31,
    },
    PointDetails {
        point: |()| Point::NameplateLowAmps,
        size: 1,
        start_address: 32,
    },
    PointDetails {
        point: |()| Point::NameplateLowVarmaxinj,
        size: 1,
        start_address: 33,
    },
    PointDetails {
        point: |()| Point::NameplateLowVarmaxabs,
        size: 1,
        start_address: 34,
    },
    PointDetails {
        point: |()| Point::NameplateLowPf,
        size: 1,
        start_address: 35,
    },
    PointDetails {
        point: |()| Point::SettingsHighNomV,
        size: 1,
        start_address: 36,
    },
    PointDetails {
        point: |()| Point::SettingsLowAmps,
        size: 1,
        start_address: 37,
    },
    PointDetails {
        point: |()| Point::SettingsHighP,
        size: 1,
        start_address: 38,
    },
    PointDetails {
        point: |()| Point::SettingsLowP,
        size: 1,
        start_address: 39,
    },
    PointDetails {
        point: |()| Point::SettingsHighVaMax,
        size: 1,
        start_address: 40,
    },
    PointDetails {
        point: |()| Point::SettingsHighVarmaxinj,
        size: 1,
        start_address: 41,
    },
    PointDetails {
        point: |()| Point::SettingsHighVarmaxabs,
        size: 1,
        start_address: 42,
    },
    PointDetails {
        point: |()| Point::ChangeCommonModelId,
        size: 1,
        start_address: 43,
    },
    PointDetails {
        point: |()| Point::ChangeCommonModelLength,
        size: 1,
        start_address: 44,
    },
];

#[derive(Debug)]
pub enum Point {
    ModelId,
    ModelLength,
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

#[derive(Debug)]
struct PointDetails<GroupIndexArgs> {
    point: fn(GroupIndexArgs) -> Point,
    start_address: u16,
    size: u16,
}

pub fn model_length(model: &dyn ModelAdapter) -> u16 {
    45
}

pub fn read_into_buffer<'a>(
    model: &dyn ModelAdapter,
    buffer: &mut ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    let until = offset + limit;
    let mut cursor = 0;

    POINTS
        .iter()
        .map(|p| (p.start_address, p.size, (p.point)(())))
        .skip_while(|(start, size, _)| offset >= start + size)
        .take_while(|(start, _, _)| until > *start)
        .for_each(|(start, size, point)| {
            write_point(
                model,
                &point,
                buffer.slice(cursor, limit - cursor),
                offset.saturating_sub(start),
                until - start,
            );
            cursor += min(size, until - start);
        });
}

pub fn write_point<'a>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'a>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelId => {
            buffer::write_u16(64412, buffer);
        }
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::DaManipulation => {
            if let Some(value) = model.da_manipulation() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::FalsifyDeviceIdentity => {
            if let Some(value) = model.falsify_device_identity() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasPAlwaysNameplate => {
            if let Some(value) = model.meas_p_always_nameplate() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasQAlwaysMinimum => {
            if let Some(value) = model.meas_q_always_minimum() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasQAlwaysMaximum => {
            if let Some(value) = model.meas_q_always_maximum() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasQAlwaysZero => {
            if let Some(value) = model.meas_q_always_zero() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasZeroP => {
            if let Some(value) = model.meas_zero_p() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasInvertQ => {
            if let Some(value) = model.meas_invert_q() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasLowV => {
            if let Some(value) = model.meas_low_v() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasHighV => {
            if let Some(value) = model.meas_high_v() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasLowL1V => {
            if let Some(value) = model.meas_low_l1_v() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasHighL1V => {
            if let Some(value) = model.meas_high_l1_v() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasLowF => {
            if let Some(value) = model.meas_low_f() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasHighF => {
            if let Some(value) = model.meas_high_f() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasLowAmps => {
            if let Some(value) = model.meas_low_amps() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasHighAmps => {
            if let Some(value) = model.meas_high_amps() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasHighS => {
            if let Some(value) = model.meas_high_s() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasLowS => {
            if let Some(value) = model.meas_low_s() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasHighQ => {
            if let Some(value) = model.meas_high_q() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasLowQ => {
            if let Some(value) = model.meas_low_q() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasLowPf => {
            if let Some(value) = model.meas_low_pf() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::MeasLowReversedPf => {
            if let Some(value) = model.meas_low_reversed_pf() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateHighP => {
            if let Some(value) = model.nameplate_high_p() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateLowP => {
            if let Some(value) = model.nameplate_low_p() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateHighS => {
            if let Some(value) = model.nameplate_high_s() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateLowS => {
            if let Some(value) = model.nameplate_low_s() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateHighQ => {
            if let Some(value) = model.nameplate_high_q() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateLowQ => {
            if let Some(value) = model.nameplate_low_q() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateHighNomV => {
            if let Some(value) = model.nameplate_high_nom_v() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateLowNomV => {
            if let Some(value) = model.nameplate_low_nom_v() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateLowAmps => {
            if let Some(value) = model.nameplate_low_amps() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateLowVarmaxinj => {
            if let Some(value) = model.nameplate_low_varmaxinj() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateLowVarmaxabs => {
            if let Some(value) = model.nameplate_low_varmaxabs() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::NameplateLowPf => {
            if let Some(value) = model.nameplate_low_pf() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SettingsHighNomV => {
            if let Some(value) = model.settings_high_nom_v() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SettingsLowAmps => {
            if let Some(value) = model.settings_low_amps() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SettingsHighP => {
            if let Some(value) = model.settings_high_p() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SettingsLowP => {
            if let Some(value) = model.settings_low_p() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SettingsHighVaMax => {
            if let Some(value) = model.settings_high_va_max() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SettingsHighVarmaxinj => {
            if let Some(value) = model.settings_high_varmaxinj() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::SettingsHighVarmaxabs => {
            if let Some(value) = model.settings_high_varmaxabs() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ChangeCommonModelId => {
            if let Some(value) = model.change_common_model_id() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
            }
        }
        Point::ChangeCommonModelLength => {
            if let Some(value) = model.change_common_model_length() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, 1);
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[repr(C)]
pub struct Model64412CallbackAdapter {
    context: *mut c_void,
    da_manipulation_callback: Option<extern "C" fn(*const c_void) -> DaManipulation>,
    set_da_manipulation_callback: Option<extern "C" fn(DaManipulation, *mut c_void)>,
    falsify_device_identity_callback: Option<extern "C" fn(*const c_void) -> FalsifyDeviceIdentity>,
    set_falsify_device_identity_callback: Option<extern "C" fn(FalsifyDeviceIdentity, *mut c_void)>,
    meas_p_always_nameplate_callback: Option<extern "C" fn(*const c_void) -> MeasPAlwaysNameplate>,
    set_meas_p_always_nameplate_callback: Option<extern "C" fn(MeasPAlwaysNameplate, *mut c_void)>,
    meas_q_always_minimum_callback: Option<extern "C" fn(*const c_void) -> MeasQAlwaysMinimum>,
    set_meas_q_always_minimum_callback: Option<extern "C" fn(MeasQAlwaysMinimum, *mut c_void)>,
    meas_q_always_maximum_callback: Option<extern "C" fn(*const c_void) -> MeasQAlwaysMaximum>,
    set_meas_q_always_maximum_callback: Option<extern "C" fn(MeasQAlwaysMaximum, *mut c_void)>,
    meas_q_always_zero_callback: Option<extern "C" fn(*const c_void) -> MeasQAlwaysZero>,
    set_meas_q_always_zero_callback: Option<extern "C" fn(MeasQAlwaysZero, *mut c_void)>,
    meas_zero_p_callback: Option<extern "C" fn(*const c_void) -> MeasZeroP>,
    set_meas_zero_p_callback: Option<extern "C" fn(MeasZeroP, *mut c_void)>,
    meas_invert_q_callback: Option<extern "C" fn(*const c_void) -> MeasInvertQ>,
    set_meas_invert_q_callback: Option<extern "C" fn(MeasInvertQ, *mut c_void)>,
    meas_low_v_callback: Option<extern "C" fn(*const c_void) -> MeasLowV>,
    set_meas_low_v_callback: Option<extern "C" fn(MeasLowV, *mut c_void)>,
    meas_high_v_callback: Option<extern "C" fn(*const c_void) -> MeasHighV>,
    set_meas_high_v_callback: Option<extern "C" fn(MeasHighV, *mut c_void)>,
    meas_low_l1_v_callback: Option<extern "C" fn(*const c_void) -> MeasLowL1v>,
    set_meas_low_l1_v_callback: Option<extern "C" fn(MeasLowL1v, *mut c_void)>,
    meas_high_l1_v_callback: Option<extern "C" fn(*const c_void) -> MeasHighL1v>,
    set_meas_high_l1_v_callback: Option<extern "C" fn(MeasHighL1v, *mut c_void)>,
    meas_low_f_callback: Option<extern "C" fn(*const c_void) -> MeasLowF>,
    set_meas_low_f_callback: Option<extern "C" fn(MeasLowF, *mut c_void)>,
    meas_high_f_callback: Option<extern "C" fn(*const c_void) -> MeasHighF>,
    set_meas_high_f_callback: Option<extern "C" fn(MeasHighF, *mut c_void)>,
    meas_low_amps_callback: Option<extern "C" fn(*const c_void) -> MeasLowAmps>,
    set_meas_low_amps_callback: Option<extern "C" fn(MeasLowAmps, *mut c_void)>,
    meas_high_amps_callback: Option<extern "C" fn(*const c_void) -> MeasHighAmps>,
    set_meas_high_amps_callback: Option<extern "C" fn(MeasHighAmps, *mut c_void)>,
    meas_high_s_callback: Option<extern "C" fn(*const c_void) -> MeasHighS>,
    set_meas_high_s_callback: Option<extern "C" fn(MeasHighS, *mut c_void)>,
    meas_low_s_callback: Option<extern "C" fn(*const c_void) -> MeasLowS>,
    set_meas_low_s_callback: Option<extern "C" fn(MeasLowS, *mut c_void)>,
    meas_high_q_callback: Option<extern "C" fn(*const c_void) -> MeasHighQ>,
    set_meas_high_q_callback: Option<extern "C" fn(MeasHighQ, *mut c_void)>,
    meas_low_q_callback: Option<extern "C" fn(*const c_void) -> MeasLowQ>,
    set_meas_low_q_callback: Option<extern "C" fn(MeasLowQ, *mut c_void)>,
    meas_low_pf_callback: Option<extern "C" fn(*const c_void) -> MeasLowPf>,
    set_meas_low_pf_callback: Option<extern "C" fn(MeasLowPf, *mut c_void)>,
    meas_low_reversed_pf_callback: Option<extern "C" fn(*const c_void) -> MeasLowReversedPf>,
    set_meas_low_reversed_pf_callback: Option<extern "C" fn(MeasLowReversedPf, *mut c_void)>,
    nameplate_high_p_callback: Option<extern "C" fn(*const c_void) -> NameplateHighP>,
    set_nameplate_high_p_callback: Option<extern "C" fn(NameplateHighP, *mut c_void)>,
    nameplate_low_p_callback: Option<extern "C" fn(*const c_void) -> NameplateLowP>,
    set_nameplate_low_p_callback: Option<extern "C" fn(NameplateLowP, *mut c_void)>,
    nameplate_high_s_callback: Option<extern "C" fn(*const c_void) -> NameplateHighS>,
    set_nameplate_high_s_callback: Option<extern "C" fn(NameplateHighS, *mut c_void)>,
    nameplate_low_s_callback: Option<extern "C" fn(*const c_void) -> NameplateLowS>,
    set_nameplate_low_s_callback: Option<extern "C" fn(NameplateLowS, *mut c_void)>,
    nameplate_high_q_callback: Option<extern "C" fn(*const c_void) -> NameplateHighQ>,
    set_nameplate_high_q_callback: Option<extern "C" fn(NameplateHighQ, *mut c_void)>,
    nameplate_low_q_callback: Option<extern "C" fn(*const c_void) -> NameplateLowQ>,
    set_nameplate_low_q_callback: Option<extern "C" fn(NameplateLowQ, *mut c_void)>,
    nameplate_high_nom_v_callback: Option<extern "C" fn(*const c_void) -> NameplateHighNomV>,
    set_nameplate_high_nom_v_callback: Option<extern "C" fn(NameplateHighNomV, *mut c_void)>,
    nameplate_low_nom_v_callback: Option<extern "C" fn(*const c_void) -> NameplateLowNomV>,
    set_nameplate_low_nom_v_callback: Option<extern "C" fn(NameplateLowNomV, *mut c_void)>,
    nameplate_low_amps_callback: Option<extern "C" fn(*const c_void) -> NameplateLowAmps>,
    set_nameplate_low_amps_callback: Option<extern "C" fn(NameplateLowAmps, *mut c_void)>,
    nameplate_low_varmaxinj_callback: Option<extern "C" fn(*const c_void) -> NameplateLowVarmaxinj>,
    set_nameplate_low_varmaxinj_callback: Option<extern "C" fn(NameplateLowVarmaxinj, *mut c_void)>,
    nameplate_low_varmaxabs_callback: Option<extern "C" fn(*const c_void) -> NameplateLowVarmaxabs>,
    set_nameplate_low_varmaxabs_callback: Option<extern "C" fn(NameplateLowVarmaxabs, *mut c_void)>,
    nameplate_low_pf_callback: Option<extern "C" fn(*const c_void) -> NameplateLowPf>,
    set_nameplate_low_pf_callback: Option<extern "C" fn(NameplateLowPf, *mut c_void)>,
    settings_high_nom_v_callback: Option<extern "C" fn(*const c_void) -> SettingsHighNomV>,
    set_settings_high_nom_v_callback: Option<extern "C" fn(SettingsHighNomV, *mut c_void)>,
    settings_low_amps_callback: Option<extern "C" fn(*const c_void) -> SettingsLowAmps>,
    set_settings_low_amps_callback: Option<extern "C" fn(SettingsLowAmps, *mut c_void)>,
    settings_high_p_callback: Option<extern "C" fn(*const c_void) -> SettingsHighP>,
    set_settings_high_p_callback: Option<extern "C" fn(SettingsHighP, *mut c_void)>,
    settings_low_p_callback: Option<extern "C" fn(*const c_void) -> SettingsLowP>,
    set_settings_low_p_callback: Option<extern "C" fn(SettingsLowP, *mut c_void)>,
    settings_high_va_max_callback: Option<extern "C" fn(*const c_void) -> SettingsHighVaMax>,
    set_settings_high_va_max_callback: Option<extern "C" fn(SettingsHighVaMax, *mut c_void)>,
    settings_high_varmaxinj_callback: Option<extern "C" fn(*const c_void) -> SettingsHighVarmaxinj>,
    set_settings_high_varmaxinj_callback: Option<extern "C" fn(SettingsHighVarmaxinj, *mut c_void)>,
    settings_high_varmaxabs_callback: Option<extern "C" fn(*const c_void) -> SettingsHighVarmaxabs>,
    set_settings_high_varmaxabs_callback: Option<extern "C" fn(SettingsHighVarmaxabs, *mut c_void)>,
    change_common_model_id_callback: Option<extern "C" fn(*const c_void) -> ChangeCommonModelId>,
    set_change_common_model_id_callback: Option<extern "C" fn(ChangeCommonModelId, *mut c_void)>,
    change_common_model_length_callback:
        Option<extern "C" fn(*const c_void) -> ChangeCommonModelLength>,
    set_change_common_model_length_callback:
        Option<extern "C" fn(ChangeCommonModelLength, *mut c_void)>,
}

impl ModelAdapter for Model64412CallbackAdapter {
    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    fn da_manipulation(&self) -> Option<DaManipulation> {
        self.da_manipulation_callback
            .map(|callback| (callback)(self.context))
    }

    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    fn set_da_manipulation(&mut self, value: DaManipulation) {
        if let Some(callback) = self.set_da_manipulation_callback {
            (callback)(value, self.context);
        };
    }

    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    fn falsify_device_identity(&self) -> Option<FalsifyDeviceIdentity> {
        self.falsify_device_identity_callback
            .map(|callback| (callback)(self.context))
    }

    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    fn set_falsify_device_identity(&mut self, value: FalsifyDeviceIdentity) {
        if let Some(callback) = self.set_falsify_device_identity_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    fn meas_p_always_nameplate(&self) -> Option<MeasPAlwaysNameplate> {
        self.meas_p_always_nameplate_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    fn set_meas_p_always_nameplate(&mut self, value: MeasPAlwaysNameplate) {
        if let Some(callback) = self.set_meas_p_always_nameplate_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    fn meas_q_always_minimum(&self) -> Option<MeasQAlwaysMinimum> {
        self.meas_q_always_minimum_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    fn set_meas_q_always_minimum(&mut self, value: MeasQAlwaysMinimum) {
        if let Some(callback) = self.set_meas_q_always_minimum_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    fn meas_q_always_maximum(&self) -> Option<MeasQAlwaysMaximum> {
        self.meas_q_always_maximum_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    fn set_meas_q_always_maximum(&mut self, value: MeasQAlwaysMaximum) {
        if let Some(callback) = self.set_meas_q_always_maximum_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    fn meas_q_always_zero(&self) -> Option<MeasQAlwaysZero> {
        self.meas_q_always_zero_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    fn set_meas_q_always_zero(&mut self, value: MeasQAlwaysZero) {
        if let Some(callback) = self.set_meas_q_always_zero_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    fn meas_zero_p(&self) -> Option<MeasZeroP> {
        self.meas_zero_p_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    fn set_meas_zero_p(&mut self, value: MeasZeroP) {
        if let Some(callback) = self.set_meas_zero_p_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    fn meas_invert_q(&self) -> Option<MeasInvertQ> {
        self.meas_invert_q_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    fn set_meas_invert_q(&mut self, value: MeasInvertQ) {
        if let Some(callback) = self.set_meas_invert_q_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    fn meas_low_v(&self) -> Option<MeasLowV> {
        self.meas_low_v_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    fn set_meas_low_v(&mut self, value: MeasLowV) {
        if let Some(callback) = self.set_meas_low_v_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    fn meas_high_v(&self) -> Option<MeasHighV> {
        self.meas_high_v_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    fn set_meas_high_v(&mut self, value: MeasHighV) {
        if let Some(callback) = self.set_meas_high_v_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn meas_low_l1_v(&self) -> Option<MeasLowL1v> {
        self.meas_low_l1_v_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn set_meas_low_l1_v(&mut self, value: MeasLowL1v) {
        if let Some(callback) = self.set_meas_low_l1_v_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn meas_high_l1_v(&self) -> Option<MeasHighL1v> {
        self.meas_high_l1_v_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn set_meas_high_l1_v(&mut self, value: MeasHighL1v) {
        if let Some(callback) = self.set_meas_high_l1_v_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    fn meas_low_f(&self) -> Option<MeasLowF> {
        self.meas_low_f_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    fn set_meas_low_f(&mut self, value: MeasLowF) {
        if let Some(callback) = self.set_meas_low_f_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    fn meas_high_f(&self) -> Option<MeasHighF> {
        self.meas_high_f_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    fn set_meas_high_f(&mut self, value: MeasHighF) {
        if let Some(callback) = self.set_meas_high_f_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    fn meas_low_amps(&self) -> Option<MeasLowAmps> {
        self.meas_low_amps_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    fn set_meas_low_amps(&mut self, value: MeasLowAmps) {
        if let Some(callback) = self.set_meas_low_amps_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    fn meas_high_amps(&self) -> Option<MeasHighAmps> {
        self.meas_high_amps_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    fn set_meas_high_amps(&mut self, value: MeasHighAmps) {
        if let Some(callback) = self.set_meas_high_amps_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    fn meas_high_s(&self) -> Option<MeasHighS> {
        self.meas_high_s_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    fn set_meas_high_s(&mut self, value: MeasHighS) {
        if let Some(callback) = self.set_meas_high_s_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    fn meas_low_s(&self) -> Option<MeasLowS> {
        self.meas_low_s_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    fn set_meas_low_s(&mut self, value: MeasLowS) {
        if let Some(callback) = self.set_meas_low_s_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    fn meas_high_q(&self) -> Option<MeasHighQ> {
        self.meas_high_q_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    fn set_meas_high_q(&mut self, value: MeasHighQ) {
        if let Some(callback) = self.set_meas_high_q_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    fn meas_low_q(&self) -> Option<MeasLowQ> {
        self.meas_low_q_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    fn set_meas_low_q(&mut self, value: MeasLowQ) {
        if let Some(callback) = self.set_meas_low_q_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    fn meas_low_pf(&self) -> Option<MeasLowPf> {
        self.meas_low_pf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    fn set_meas_low_pf(&mut self, value: MeasLowPf) {
        if let Some(callback) = self.set_meas_low_pf_callback {
            (callback)(value, self.context);
        };
    }

    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    fn meas_low_reversed_pf(&self) -> Option<MeasLowReversedPf> {
        self.meas_low_reversed_pf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    fn set_meas_low_reversed_pf(&mut self, value: MeasLowReversedPf) {
        if let Some(callback) = self.set_meas_low_reversed_pf_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    fn nameplate_high_p(&self) -> Option<NameplateHighP> {
        self.nameplate_high_p_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    fn set_nameplate_high_p(&mut self, value: NameplateHighP) {
        if let Some(callback) = self.set_nameplate_high_p_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    fn nameplate_low_p(&self) -> Option<NameplateLowP> {
        self.nameplate_low_p_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    fn set_nameplate_low_p(&mut self, value: NameplateLowP) {
        if let Some(callback) = self.set_nameplate_low_p_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    fn nameplate_high_s(&self) -> Option<NameplateHighS> {
        self.nameplate_high_s_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    fn set_nameplate_high_s(&mut self, value: NameplateHighS) {
        if let Some(callback) = self.set_nameplate_high_s_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    fn nameplate_low_s(&self) -> Option<NameplateLowS> {
        self.nameplate_low_s_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    fn set_nameplate_low_s(&mut self, value: NameplateLowS) {
        if let Some(callback) = self.set_nameplate_low_s_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    fn nameplate_high_q(&self) -> Option<NameplateHighQ> {
        self.nameplate_high_q_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    fn set_nameplate_high_q(&mut self, value: NameplateHighQ) {
        if let Some(callback) = self.set_nameplate_high_q_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    fn nameplate_low_q(&self) -> Option<NameplateLowQ> {
        self.nameplate_low_q_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    fn set_nameplate_low_q(&mut self, value: NameplateLowQ) {
        if let Some(callback) = self.set_nameplate_low_q_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    fn nameplate_high_nom_v(&self) -> Option<NameplateHighNomV> {
        self.nameplate_high_nom_v_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    fn set_nameplate_high_nom_v(&mut self, value: NameplateHighNomV) {
        if let Some(callback) = self.set_nameplate_high_nom_v_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    fn nameplate_low_nom_v(&self) -> Option<NameplateLowNomV> {
        self.nameplate_low_nom_v_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    fn set_nameplate_low_nom_v(&mut self, value: NameplateLowNomV) {
        if let Some(callback) = self.set_nameplate_low_nom_v_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    fn nameplate_low_amps(&self) -> Option<NameplateLowAmps> {
        self.nameplate_low_amps_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    fn set_nameplate_low_amps(&mut self, value: NameplateLowAmps) {
        if let Some(callback) = self.set_nameplate_low_amps_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    fn nameplate_low_varmaxinj(&self) -> Option<NameplateLowVarmaxinj> {
        self.nameplate_low_varmaxinj_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    fn set_nameplate_low_varmaxinj(&mut self, value: NameplateLowVarmaxinj) {
        if let Some(callback) = self.set_nameplate_low_varmaxinj_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    fn nameplate_low_varmaxabs(&self) -> Option<NameplateLowVarmaxabs> {
        self.nameplate_low_varmaxabs_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    fn set_nameplate_low_varmaxabs(&mut self, value: NameplateLowVarmaxabs) {
        if let Some(callback) = self.set_nameplate_low_varmaxabs_callback {
            (callback)(value, self.context);
        };
    }

    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    fn nameplate_low_pf(&self) -> Option<NameplateLowPf> {
        self.nameplate_low_pf_callback
            .map(|callback| (callback)(self.context))
    }

    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    fn set_nameplate_low_pf(&mut self, value: NameplateLowPf) {
        if let Some(callback) = self.set_nameplate_low_pf_callback {
            (callback)(value, self.context);
        };
    }

    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    fn settings_high_nom_v(&self) -> Option<SettingsHighNomV> {
        self.settings_high_nom_v_callback
            .map(|callback| (callback)(self.context))
    }

    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    fn set_settings_high_nom_v(&mut self, value: SettingsHighNomV) {
        if let Some(callback) = self.set_settings_high_nom_v_callback {
            (callback)(value, self.context);
        };
    }

    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    fn settings_low_amps(&self) -> Option<SettingsLowAmps> {
        self.settings_low_amps_callback
            .map(|callback| (callback)(self.context))
    }

    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    fn set_settings_low_amps(&mut self, value: SettingsLowAmps) {
        if let Some(callback) = self.set_settings_low_amps_callback {
            (callback)(value, self.context);
        };
    }

    /// Settings High P
    ///
    /// Set the DER settings power to be high
    fn settings_high_p(&self) -> Option<SettingsHighP> {
        self.settings_high_p_callback
            .map(|callback| (callback)(self.context))
    }

    /// Settings High P
    ///
    /// Set the DER settings power to be high
    fn set_settings_high_p(&mut self, value: SettingsHighP) {
        if let Some(callback) = self.set_settings_high_p_callback {
            (callback)(value, self.context);
        };
    }

    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    fn settings_low_p(&self) -> Option<SettingsLowP> {
        self.settings_low_p_callback
            .map(|callback| (callback)(self.context))
    }

    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    fn set_settings_low_p(&mut self, value: SettingsLowP) {
        if let Some(callback) = self.set_settings_low_p_callback {
            (callback)(value, self.context);
        };
    }

    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    fn settings_high_va_max(&self) -> Option<SettingsHighVaMax> {
        self.settings_high_va_max_callback
            .map(|callback| (callback)(self.context))
    }

    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    fn set_settings_high_va_max(&mut self, value: SettingsHighVaMax) {
        if let Some(callback) = self.set_settings_high_va_max_callback {
            (callback)(value, self.context);
        };
    }

    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    fn settings_high_varmaxinj(&self) -> Option<SettingsHighVarmaxinj> {
        self.settings_high_varmaxinj_callback
            .map(|callback| (callback)(self.context))
    }

    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    fn set_settings_high_varmaxinj(&mut self, value: SettingsHighVarmaxinj) {
        if let Some(callback) = self.set_settings_high_varmaxinj_callback {
            (callback)(value, self.context);
        };
    }

    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    fn settings_high_varmaxabs(&self) -> Option<SettingsHighVarmaxabs> {
        self.settings_high_varmaxabs_callback
            .map(|callback| (callback)(self.context))
    }

    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    fn set_settings_high_varmaxabs(&mut self, value: SettingsHighVarmaxabs) {
        if let Some(callback) = self.set_settings_high_varmaxabs_callback {
            (callback)(value, self.context);
        };
    }

    /// Change Common Model ID
    ///
    /// Change the common model ID
    fn change_common_model_id(&self) -> Option<ChangeCommonModelId> {
        self.change_common_model_id_callback
            .map(|callback| (callback)(self.context))
    }

    /// Change Common Model ID
    ///
    /// Change the common model ID
    fn set_change_common_model_id(&mut self, value: ChangeCommonModelId) {
        if let Some(callback) = self.set_change_common_model_id_callback {
            (callback)(value, self.context);
        };
    }

    /// Change Common Model Length
    ///
    /// Change the common model length
    fn change_common_model_length(&self) -> Option<ChangeCommonModelLength> {
        self.change_common_model_length_callback
            .map(|callback| (callback)(self.context))
    }

    /// Change Common Model Length
    ///
    /// Change the common model length
    fn set_change_common_model_length(&mut self, value: ChangeCommonModelLength) {
        if let Some(callback) = self.set_change_common_model_length_callback {
            (callback)(value, self.context);
        };
    }
}

#[repr(C)]
pub struct Model64412StatefulAdapter {
    da_manipulation: DaManipulation,
    falsify_device_identity: FalsifyDeviceIdentity,
    meas_p_always_nameplate: MeasPAlwaysNameplate,
    meas_q_always_minimum: MeasQAlwaysMinimum,
    meas_q_always_maximum: MeasQAlwaysMaximum,
    meas_q_always_zero: MeasQAlwaysZero,
    meas_zero_p: MeasZeroP,
    meas_invert_q: MeasInvertQ,
    meas_low_v: MeasLowV,
    meas_high_v: MeasHighV,
    meas_low_l1_v: MeasLowL1v,
    meas_high_l1_v: MeasHighL1v,
    meas_low_f: MeasLowF,
    meas_high_f: MeasHighF,
    meas_low_amps: MeasLowAmps,
    meas_high_amps: MeasHighAmps,
    meas_high_s: MeasHighS,
    meas_low_s: MeasLowS,
    meas_high_q: MeasHighQ,
    meas_low_q: MeasLowQ,
    meas_low_pf: MeasLowPf,
    meas_low_reversed_pf: MeasLowReversedPf,
    nameplate_high_p: NameplateHighP,
    nameplate_low_p: NameplateLowP,
    nameplate_high_s: NameplateHighS,
    nameplate_low_s: NameplateLowS,
    nameplate_high_q: NameplateHighQ,
    nameplate_low_q: NameplateLowQ,
    nameplate_high_nom_v: NameplateHighNomV,
    nameplate_low_nom_v: NameplateLowNomV,
    nameplate_low_amps: NameplateLowAmps,
    nameplate_low_varmaxinj: NameplateLowVarmaxinj,
    nameplate_low_varmaxabs: NameplateLowVarmaxabs,
    nameplate_low_pf: NameplateLowPf,
    settings_high_nom_v: SettingsHighNomV,
    settings_low_amps: SettingsLowAmps,
    settings_high_p: SettingsHighP,
    settings_low_p: SettingsLowP,
    settings_high_va_max: SettingsHighVaMax,
    settings_high_varmaxinj: SettingsHighVarmaxinj,
    settings_high_varmaxabs: SettingsHighVarmaxabs,
    change_common_model_id: ChangeCommonModelId,
    change_common_model_length: ChangeCommonModelLength,
}

impl ModelAdapter for Model64412StatefulAdapter {
    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    fn da_manipulation(&self) -> Option<DaManipulation> {
        Some(self.da_manipulation)
    }

    /// DA Manipulation
    ///
    /// Modify the device ID of the DER
    fn set_da_manipulation(&mut self, value: DaManipulation) {
        self.da_manipulation = value;
    }

    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    fn falsify_device_identity(&self) -> Option<FalsifyDeviceIdentity> {
        Some(self.falsify_device_identity)
    }

    /// Falsify Device Identity
    ///
    /// Change the DER manufacturer and model
    fn set_falsify_device_identity(&mut self, value: FalsifyDeviceIdentity) {
        self.falsify_device_identity = value;
    }

    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    fn meas_p_always_nameplate(&self) -> Option<MeasPAlwaysNameplate> {
        Some(self.meas_p_always_nameplate)
    }

    /// Meas P Always Nameplate
    ///
    /// Set the DER meas to always be at nameplate power
    fn set_meas_p_always_nameplate(&mut self, value: MeasPAlwaysNameplate) {
        self.meas_p_always_nameplate = value;
    }

    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    fn meas_q_always_minimum(&self) -> Option<MeasQAlwaysMinimum> {
        Some(self.meas_q_always_minimum)
    }

    /// Meas Q Always Minimum
    ///
    /// Set the DER to always be at minimum reactive power
    fn set_meas_q_always_minimum(&mut self, value: MeasQAlwaysMinimum) {
        self.meas_q_always_minimum = value;
    }

    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    fn meas_q_always_maximum(&self) -> Option<MeasQAlwaysMaximum> {
        Some(self.meas_q_always_maximum)
    }

    /// Meas Q Always Maximum
    ///
    /// Set the DER to always be at maximum reactive power
    fn set_meas_q_always_maximum(&mut self, value: MeasQAlwaysMaximum) {
        self.meas_q_always_maximum = value;
    }

    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    fn meas_q_always_zero(&self) -> Option<MeasQAlwaysZero> {
        Some(self.meas_q_always_zero)
    }

    /// Meas Q Always Zero
    ///
    /// Set the DER to always be at zero reactive power
    fn set_meas_q_always_zero(&mut self, value: MeasQAlwaysZero) {
        self.meas_q_always_zero = value;
    }

    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    fn meas_zero_p(&self) -> Option<MeasZeroP> {
        Some(self.meas_zero_p)
    }

    /// Meas Zero P
    ///
    /// Set the DER to always be at zero P, Q, and S
    fn set_meas_zero_p(&mut self, value: MeasZeroP) {
        self.meas_zero_p = value;
    }

    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    fn meas_invert_q(&self) -> Option<MeasInvertQ> {
        Some(self.meas_invert_q)
    }

    /// Meas Invert Q
    ///
    /// Set the DER to reverse the Q measurement data
    fn set_meas_invert_q(&mut self, value: MeasInvertQ) {
        self.meas_invert_q = value;
    }

    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    fn meas_low_v(&self) -> Option<MeasLowV> {
        Some(self.meas_low_v)
    }

    /// Meas Low V
    ///
    /// Set the DER to always measure low voltage
    fn set_meas_low_v(&mut self, value: MeasLowV) {
        self.meas_low_v = value;
    }

    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    fn meas_high_v(&self) -> Option<MeasHighV> {
        Some(self.meas_high_v)
    }

    /// Meas High V
    ///
    /// Set the DER to always measure high voltage
    fn set_meas_high_v(&mut self, value: MeasHighV) {
        self.meas_high_v = value;
    }

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn meas_low_l1_v(&self) -> Option<MeasLowL1v> {
        Some(self.meas_low_l1_v)
    }

    /// Meas Low L1 V
    ///
    /// Set the DER to always measure low line 1 voltage
    fn set_meas_low_l1_v(&mut self, value: MeasLowL1v) {
        self.meas_low_l1_v = value;
    }

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn meas_high_l1_v(&self) -> Option<MeasHighL1v> {
        Some(self.meas_high_l1_v)
    }

    /// Meas High L1 V
    ///
    /// Set the DER to always measure high line 1 voltage
    fn set_meas_high_l1_v(&mut self, value: MeasHighL1v) {
        self.meas_high_l1_v = value;
    }

    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    fn meas_low_f(&self) -> Option<MeasLowF> {
        Some(self.meas_low_f)
    }

    /// Meas Low F
    ///
    /// Set the DER to always measure low frequency
    fn set_meas_low_f(&mut self, value: MeasLowF) {
        self.meas_low_f = value;
    }

    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    fn meas_high_f(&self) -> Option<MeasHighF> {
        Some(self.meas_high_f)
    }

    /// Meas High F
    ///
    /// Set the DER to always measure high frequency
    fn set_meas_high_f(&mut self, value: MeasHighF) {
        self.meas_high_f = value;
    }

    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    fn meas_low_amps(&self) -> Option<MeasLowAmps> {
        Some(self.meas_low_amps)
    }

    /// Meas Low Amps
    ///
    /// Set the DER to always measure low current
    fn set_meas_low_amps(&mut self, value: MeasLowAmps) {
        self.meas_low_amps = value;
    }

    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    fn meas_high_amps(&self) -> Option<MeasHighAmps> {
        Some(self.meas_high_amps)
    }

    /// Meas High Amps
    ///
    /// Set the DER to always measure high current
    fn set_meas_high_amps(&mut self, value: MeasHighAmps) {
        self.meas_high_amps = value;
    }

    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    fn meas_high_s(&self) -> Option<MeasHighS> {
        Some(self.meas_high_s)
    }

    /// Meas High S
    ///
    /// Set the DER to always measure high apparent power
    fn set_meas_high_s(&mut self, value: MeasHighS) {
        self.meas_high_s = value;
    }

    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    fn meas_low_s(&self) -> Option<MeasLowS> {
        Some(self.meas_low_s)
    }

    /// Meas Low S
    ///
    /// Set the DER to always measure low apparent power
    fn set_meas_low_s(&mut self, value: MeasLowS) {
        self.meas_low_s = value;
    }

    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    fn meas_high_q(&self) -> Option<MeasHighQ> {
        Some(self.meas_high_q)
    }

    /// Meas High Q
    ///
    /// Set the DER to always measure high reactive power
    fn set_meas_high_q(&mut self, value: MeasHighQ) {
        self.meas_high_q = value;
    }

    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    fn meas_low_q(&self) -> Option<MeasLowQ> {
        Some(self.meas_low_q)
    }

    /// Meas Low Q
    ///
    /// Set the DER to always measure low reactive power
    fn set_meas_low_q(&mut self, value: MeasLowQ) {
        self.meas_low_q = value;
    }

    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    fn meas_low_pf(&self) -> Option<MeasLowPf> {
        Some(self.meas_low_pf)
    }

    /// Meas Low PF
    ///
    /// Set the DER to always measure low power factor
    fn set_meas_low_pf(&mut self, value: MeasLowPf) {
        self.meas_low_pf = value;
    }

    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    fn meas_low_reversed_pf(&self) -> Option<MeasLowReversedPf> {
        Some(self.meas_low_reversed_pf)
    }

    /// Meas Low Reversed PF
    ///
    /// Set the DER to always measure low reversed power factor
    fn set_meas_low_reversed_pf(&mut self, value: MeasLowReversedPf) {
        self.meas_low_reversed_pf = value;
    }

    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    fn nameplate_high_p(&self) -> Option<NameplateHighP> {
        Some(self.nameplate_high_p)
    }

    /// Nameplate High P
    ///
    /// Set the DER nameplate power to be high
    fn set_nameplate_high_p(&mut self, value: NameplateHighP) {
        self.nameplate_high_p = value;
    }

    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    fn nameplate_low_p(&self) -> Option<NameplateLowP> {
        Some(self.nameplate_low_p)
    }

    /// Nameplate Low P
    ///
    /// Set the DER nameplate power to be low
    fn set_nameplate_low_p(&mut self, value: NameplateLowP) {
        self.nameplate_low_p = value;
    }

    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    fn nameplate_high_s(&self) -> Option<NameplateHighS> {
        Some(self.nameplate_high_s)
    }

    /// Nameplate High S
    ///
    /// Set the DER nameplate apparent power to be high
    fn set_nameplate_high_s(&mut self, value: NameplateHighS) {
        self.nameplate_high_s = value;
    }

    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    fn nameplate_low_s(&self) -> Option<NameplateLowS> {
        Some(self.nameplate_low_s)
    }

    /// Nameplate Low S
    ///
    /// Set the DER nameplate apparent power to be low
    fn set_nameplate_low_s(&mut self, value: NameplateLowS) {
        self.nameplate_low_s = value;
    }

    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    fn nameplate_high_q(&self) -> Option<NameplateHighQ> {
        Some(self.nameplate_high_q)
    }

    /// Nameplate High Q
    ///
    /// Set the DER nameplate reactive power to be high
    fn set_nameplate_high_q(&mut self, value: NameplateHighQ) {
        self.nameplate_high_q = value;
    }

    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    fn nameplate_low_q(&self) -> Option<NameplateLowQ> {
        Some(self.nameplate_low_q)
    }

    /// Nameplate Low Q
    ///
    /// Set the DER nameplate reactive power to be low
    fn set_nameplate_low_q(&mut self, value: NameplateLowQ) {
        self.nameplate_low_q = value;
    }

    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    fn nameplate_high_nom_v(&self) -> Option<NameplateHighNomV> {
        Some(self.nameplate_high_nom_v)
    }

    /// Nameplate High Nom V
    ///
    /// Set the DER nameplate voltage to be high
    fn set_nameplate_high_nom_v(&mut self, value: NameplateHighNomV) {
        self.nameplate_high_nom_v = value;
    }

    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    fn nameplate_low_nom_v(&self) -> Option<NameplateLowNomV> {
        Some(self.nameplate_low_nom_v)
    }

    /// Nameplate Low Nom V
    ///
    /// Set the DER nameplate voltage to be low
    fn set_nameplate_low_nom_v(&mut self, value: NameplateLowNomV) {
        self.nameplate_low_nom_v = value;
    }

    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    fn nameplate_low_amps(&self) -> Option<NameplateLowAmps> {
        Some(self.nameplate_low_amps)
    }

    /// Nameplate Low Amps
    ///
    /// Set the DER nameplate current to be low
    fn set_nameplate_low_amps(&mut self, value: NameplateLowAmps) {
        self.nameplate_low_amps = value;
    }

    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    fn nameplate_low_varmaxinj(&self) -> Option<NameplateLowVarmaxinj> {
        Some(self.nameplate_low_varmaxinj)
    }

    /// Nameplate Low Varmaxinj
    ///
    /// Set the DER nameplate VarMaxInj to be low
    fn set_nameplate_low_varmaxinj(&mut self, value: NameplateLowVarmaxinj) {
        self.nameplate_low_varmaxinj = value;
    }

    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    fn nameplate_low_varmaxabs(&self) -> Option<NameplateLowVarmaxabs> {
        Some(self.nameplate_low_varmaxabs)
    }

    /// Nameplate Low Varmaxabs
    ///
    /// Set the DER nameplate VarMaxAbs to be low
    fn set_nameplate_low_varmaxabs(&mut self, value: NameplateLowVarmaxabs) {
        self.nameplate_low_varmaxabs = value;
    }

    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    fn nameplate_low_pf(&self) -> Option<NameplateLowPf> {
        Some(self.nameplate_low_pf)
    }

    /// Nameplate Low PF
    ///
    /// Set the DER nameplate power factor to be low
    fn set_nameplate_low_pf(&mut self, value: NameplateLowPf) {
        self.nameplate_low_pf = value;
    }

    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    fn settings_high_nom_v(&self) -> Option<SettingsHighNomV> {
        Some(self.settings_high_nom_v)
    }

    /// Settings High Nom V
    ///
    /// Set the DER settings voltage to be high
    fn set_settings_high_nom_v(&mut self, value: SettingsHighNomV) {
        self.settings_high_nom_v = value;
    }

    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    fn settings_low_amps(&self) -> Option<SettingsLowAmps> {
        Some(self.settings_low_amps)
    }

    /// Settings Low Amps
    ///
    /// Set the DER settings current to be low
    fn set_settings_low_amps(&mut self, value: SettingsLowAmps) {
        self.settings_low_amps = value;
    }

    /// Settings High P
    ///
    /// Set the DER settings power to be high
    fn settings_high_p(&self) -> Option<SettingsHighP> {
        Some(self.settings_high_p)
    }

    /// Settings High P
    ///
    /// Set the DER settings power to be high
    fn set_settings_high_p(&mut self, value: SettingsHighP) {
        self.settings_high_p = value;
    }

    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    fn settings_low_p(&self) -> Option<SettingsLowP> {
        Some(self.settings_low_p)
    }

    /// Settings Low P
    ///
    /// Set the DER settings power to be low
    fn set_settings_low_p(&mut self, value: SettingsLowP) {
        self.settings_low_p = value;
    }

    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    fn settings_high_va_max(&self) -> Option<SettingsHighVaMax> {
        Some(self.settings_high_va_max)
    }

    /// Settings High VAMax
    ///
    /// Set the DER settings VAMax to be high
    fn set_settings_high_va_max(&mut self, value: SettingsHighVaMax) {
        self.settings_high_va_max = value;
    }

    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    fn settings_high_varmaxinj(&self) -> Option<SettingsHighVarmaxinj> {
        Some(self.settings_high_varmaxinj)
    }

    /// Settings High Varmaxinj
    ///
    /// Set the DER settings VarMaxInj to be high
    fn set_settings_high_varmaxinj(&mut self, value: SettingsHighVarmaxinj) {
        self.settings_high_varmaxinj = value;
    }

    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    fn settings_high_varmaxabs(&self) -> Option<SettingsHighVarmaxabs> {
        Some(self.settings_high_varmaxabs)
    }

    /// Settings High Varmaxabs
    ///
    /// Set the DER settings VarMaxAbs to be high
    fn set_settings_high_varmaxabs(&mut self, value: SettingsHighVarmaxabs) {
        self.settings_high_varmaxabs = value;
    }

    /// Change Common Model ID
    ///
    /// Change the common model ID
    fn change_common_model_id(&self) -> Option<ChangeCommonModelId> {
        Some(self.change_common_model_id)
    }

    /// Change Common Model ID
    ///
    /// Change the common model ID
    fn set_change_common_model_id(&mut self, value: ChangeCommonModelId) {
        self.change_common_model_id = value;
    }

    /// Change Common Model Length
    ///
    /// Change the common model length
    fn change_common_model_length(&self) -> Option<ChangeCommonModelLength> {
        Some(self.change_common_model_length)
    }

    /// Change Common Model Length
    ///
    /// Change the common model length
    fn set_change_common_model_length(&mut self, value: ChangeCommonModelLength) {
        self.change_common_model_length = value;
    }
}
