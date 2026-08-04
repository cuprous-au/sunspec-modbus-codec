use core::ffi::c_void;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 10;

pub static POINTS: [ReadablePoint; 10] = [
    ReadablePoint {
        reference: PointReference::Static { value: 145 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 8 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model145 { point: Point::RampUpRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model145 { point: Point::NomRmpDnRte },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model145 { point: Point::EmergencyRampUpRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model145 { point: Point::EmergencyRampDownRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model145 { point: Point::ConnectRampUpRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model145 { point: Point::ConnectRampDownRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model145 { point: Point::DefaultRampRate },
        size: 1,
        data_type: PointType::Uint16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model145 { point: Point::RampRateScaleFactor },
        size: 1,
        data_type: PointType::Sunssf,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    RampUpRate,
    NomRmpDnRte,
    EmergencyRampUpRate,
    EmergencyRampDownRate,
    ConnectRampUpRate,
    ConnectRampDownRate,
    DefaultRampRate,
    RampRateScaleFactor,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::RampUpRate => {
            if let Some(value) = model.ramp_up_rate() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::NomRmpDnRte => {
            if let Some(value) = model.nom_rmp_dn_rte() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::EmergencyRampUpRate => {
            if let Some(value) = model.emergency_ramp_up_rate() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::EmergencyRampDownRate => {
            if let Some(value) = model.emergency_ramp_down_rate() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::ConnectRampUpRate => {
            if let Some(value) = model.connect_ramp_up_rate() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::ConnectRampDownRate => {
            if let Some(value) = model.connect_ramp_down_rate() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::DefaultRampRate => {
            if let Some(value) = model.default_ramp_rate() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
        Point::RampRateScaleFactor => {
            if let Some(value) = model.ramp_rate_scale_factor() {
                serialisation::write_u16(value, buffer);
            }
            else {
                buffer.fill(0)
            }
        }
    }
}

pub trait ModelAdapter {
    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    fn ramp_up_rate(&self) -> Option<u16> {
        None
    }

    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    fn set_ramp_up_rate(&mut self, value: u16) {
    }

    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    fn nom_rmp_dn_rte(&self) -> Option<u16> {
        None
    }

    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    fn set_nom_rmp_dn_rte(&mut self, value: u16) {
    }

    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    fn emergency_ramp_up_rate(&self) -> Option<u16> {
        None
    }

    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    fn set_emergency_ramp_up_rate(&mut self, value: u16) {
    }

    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    fn emergency_ramp_down_rate(&self) -> Option<u16> {
        None
    }

    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    fn set_emergency_ramp_down_rate(&mut self, value: u16) {
    }

    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    fn connect_ramp_up_rate(&self) -> Option<u16> {
        None
    }

    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    fn set_connect_ramp_up_rate(&mut self, value: u16) {
    }

    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    fn connect_ramp_down_rate(&self) -> Option<u16> {
        None
    }

    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    fn set_connect_ramp_down_rate(&mut self, value: u16) {
    }

    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    fn default_ramp_rate(&self) -> Option<u16> {
        None
    }

    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    fn set_default_ramp_rate(&mut self, value: u16) {
    }

    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    fn ramp_rate_scale_factor(&self) -> Option<u16> {
        None
    }
}

#[repr(C)]
pub struct Model145CallbackAdapter {
    context: *mut c_void,
    ramp_up_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_ramp_up_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    nom_rmp_dn_rte_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_nom_rmp_dn_rte_callback: Option<extern "C" fn(u16, *mut c_void)>,
    emergency_ramp_up_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_emergency_ramp_up_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    emergency_ramp_down_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_emergency_ramp_down_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    connect_ramp_up_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_connect_ramp_up_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    connect_ramp_down_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_connect_ramp_down_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    default_ramp_rate_callback: Option<extern "C" fn(*const c_void) -> u16>,
    set_default_ramp_rate_callback: Option<extern "C" fn(u16, *mut c_void)>,
    ramp_rate_scale_factor_callback: Option<extern "C" fn(*const c_void) -> u16>,
}

impl ModelAdapter for Model145CallbackAdapter {
    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    fn ramp_up_rate(&self) -> Option<u16> {
        self.ramp_up_rate_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    fn set_ramp_up_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_ramp_up_rate_callback {
        (callback)(value, self.context);
        };
    }

    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    fn nom_rmp_dn_rte(&self) -> Option<u16> {
        self.nom_rmp_dn_rte_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    fn set_nom_rmp_dn_rte(&mut self, value: u16) {
        if let Some(callback) = self.set_nom_rmp_dn_rte_callback {
        (callback)(value, self.context);
        };
    }

    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    fn emergency_ramp_up_rate(&self) -> Option<u16> {
        self.emergency_ramp_up_rate_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    fn set_emergency_ramp_up_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_emergency_ramp_up_rate_callback {
        (callback)(value, self.context);
        };
    }

    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    fn emergency_ramp_down_rate(&self) -> Option<u16> {
        self.emergency_ramp_down_rate_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    fn set_emergency_ramp_down_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_emergency_ramp_down_rate_callback {
        (callback)(value, self.context);
        };
    }

    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    fn connect_ramp_up_rate(&self) -> Option<u16> {
        self.connect_ramp_up_rate_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    fn set_connect_ramp_up_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_connect_ramp_up_rate_callback {
        (callback)(value, self.context);
        };
    }

    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    fn connect_ramp_down_rate(&self) -> Option<u16> {
        self.connect_ramp_down_rate_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    fn set_connect_ramp_down_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_connect_ramp_down_rate_callback {
        (callback)(value, self.context);
        };
    }

    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    fn default_ramp_rate(&self) -> Option<u16> {
        self.default_ramp_rate_callback.map(|callback| {
        (callback)(self.context)
        })
    }

    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    fn set_default_ramp_rate(&mut self, value: u16) {
        if let Some(callback) = self.set_default_ramp_rate_callback {
        (callback)(value, self.context);
        };
    }

    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    fn ramp_rate_scale_factor(&self) -> Option<u16> {
        self.ramp_rate_scale_factor_callback.map(|callback| {
        (callback)(self.context)
        })
    }
}

#[repr(C)]
pub struct Model145StatefulAdapter {
    ramp_up_rate: u16,
    nom_rmp_dn_rte: u16,
    emergency_ramp_up_rate: u16,
    emergency_ramp_down_rate: u16,
    connect_ramp_up_rate: u16,
    connect_ramp_down_rate: u16,
    default_ramp_rate: u16,
    ramp_rate_scale_factor: u16,
}

impl ModelAdapter for Model145StatefulAdapter {
    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    fn ramp_up_rate(&self) -> Option<u16> {
        Some(
        self.ramp_up_rate
        )
    }

    /// Ramp Up Rate
    ///
    /// Ramp up rate as a percentage of max current.
    fn set_ramp_up_rate(&mut self, value: u16) {
        self.ramp_up_rate = value;
    }

    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    fn nom_rmp_dn_rte(&self) -> Option<u16> {
        Some(
        self.nom_rmp_dn_rte
        )
    }

    /// NomRmpDnRte
    ///
    /// Ramp down rate as a percentage of max current.
    fn set_nom_rmp_dn_rte(&mut self, value: u16) {
        self.nom_rmp_dn_rte = value;
    }

    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    fn emergency_ramp_up_rate(&self) -> Option<u16> {
        Some(
        self.emergency_ramp_up_rate
        )
    }

    /// Emergency Ramp Up Rate
    ///
    /// Emergency ramp up rate as a percentage of max current.
    fn set_emergency_ramp_up_rate(&mut self, value: u16) {
        self.emergency_ramp_up_rate = value;
    }

    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    fn emergency_ramp_down_rate(&self) -> Option<u16> {
        Some(
        self.emergency_ramp_down_rate
        )
    }

    /// Emergency Ramp Down Rate
    ///
    /// Emergency ramp down rate as a percentage of max current.
    fn set_emergency_ramp_down_rate(&mut self, value: u16) {
        self.emergency_ramp_down_rate = value;
    }

    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    fn connect_ramp_up_rate(&self) -> Option<u16> {
        Some(
        self.connect_ramp_up_rate
        )
    }

    /// Connect Ramp Up Rate
    ///
    /// Connect ramp up rate as a percentage of max current.
    fn set_connect_ramp_up_rate(&mut self, value: u16) {
        self.connect_ramp_up_rate = value;
    }

    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    fn connect_ramp_down_rate(&self) -> Option<u16> {
        Some(
        self.connect_ramp_down_rate
        )
    }

    /// Connect Ramp Down Rate
    ///
    /// Connect ramp down rate as a percentage of max current.
    fn set_connect_ramp_down_rate(&mut self, value: u16) {
        self.connect_ramp_down_rate = value;
    }

    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    fn default_ramp_rate(&self) -> Option<u16> {
        Some(
        self.default_ramp_rate
        )
    }

    /// Default Ramp Rate
    ///
    /// Ramp rate specified in percent of max current.
    fn set_default_ramp_rate(&mut self, value: u16) {
        self.default_ramp_rate = value;
    }

    /// Ramp Rate Scale Factor
    ///
    /// Ramp Rate Scale Factor
    fn ramp_rate_scale_factor(&self) -> Option<u16> {
        Some(
        self.ramp_rate_scale_factor
        )
    }
}