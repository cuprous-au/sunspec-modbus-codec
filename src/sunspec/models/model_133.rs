use core::ffi::c_void;
use crate::serialisation;
use crate::sunspec::{PointType, ReadablePoint};
use crate::sunspec::points::PointReference;

pub const SIZE: u16 = 8;

pub static POINTS: [ReadablePoint; 7] = [
    ReadablePoint {
        reference: PointReference::Static { value: 133 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 6 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model133 { point: Point::ActSchd },
        size: 2,
        data_type: PointType::Bitfield32,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model133 { point: Point::ModEna },
        size: 1,
        data_type: PointType::Bitfield16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model133 { point: Point::NSchd },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model133 { point: Point::NPts },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 0 },
        size: 1,
        data_type: PointType::Pad,
        writeable: false,
    },
];

#[derive(Debug)]
pub enum Point {
    ActSchd,
    ModEna,
    NSchd,
    NPts,
}

pub fn write_point(model: &dyn ModelAdapter, point: &Point, buffer: &mut [u16], offset: u16, limit: u16) {
    match point {
        Point::ActSchd => {
            serialisation::write_u32(model.act_schd(), buffer, offset, limit);
        },
        Point::ModEna => {
            serialisation::write_u16(model.mod_ena(), buffer);
        },
        Point::NSchd => {
            serialisation::write_u16(model.n_schd(), buffer);
        },
        Point::NPts => {
            serialisation::write_u16(model.n_pts(), buffer);
        },
    }
}

pub trait ModelAdapter {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn act_schd(&self) -> u32;

    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn set_act_schd(&mut self, value: u32);

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn set_mod_ena(&mut self, value: u16);

    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    fn n_schd(&self) -> u16;

    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    fn n_pts(&self) -> u16;
}

#[repr(C)]
pub struct Model133CallbackAdapter {
    context: *mut c_void,
    act_schd_callback: extern "C" fn(*const c_void) -> u32,
    set_act_schd_callback: extern "C" fn(u32, *mut c_void),
    mod_ena_callback: extern "C" fn(*const c_void) -> u16,
    set_mod_ena_callback: extern "C" fn(u16, *mut c_void),
    n_schd_callback: extern "C" fn(*const c_void) -> u16,
    n_pts_callback: extern "C" fn(*const c_void) -> u16,
}

impl ModelAdapter for Model133CallbackAdapter {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn act_schd(&self) -> u32 {
        (self.act_schd_callback)(self.context)
    }

    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn set_act_schd(&mut self, value: u32) {
        (self.set_act_schd_callback)(value, self.context);
    }

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn mod_ena(&self) -> u16 {
        (self.mod_ena_callback)(self.context)
    }

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn set_mod_ena(&mut self, value: u16) {
        (self.set_mod_ena_callback)(value, self.context);
    }

    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    fn n_schd(&self) -> u16 {
        (self.n_schd_callback)(self.context)
    }

    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    fn n_pts(&self) -> u16 {
        (self.n_pts_callback)(self.context)
    }
}

#[repr(C)]
pub struct Model133StatefulAdapter {
    act_schd: u32,
    mod_ena: u16,
    n_schd: u16,
    n_pts: u16,
}

impl ModelAdapter for Model133StatefulAdapter {
    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn act_schd(&self) -> u32 {
        self.act_schd
    }

    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn set_act_schd(&mut self, value: u32) {
        self.act_schd = value;
    }

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn mod_ena(&self) -> u16 {
        self.mod_ena
    }

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn set_mod_ena(&mut self, value: u16) {
        self.mod_ena = value;
    }

    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    fn n_schd(&self) -> u16 {
        self.n_schd
    }

    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    fn n_pts(&self) -> u16 {
        self.n_pts
    }
}