/// To be included first for a complete interface description
pub struct Model10 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Interface Status
    ///
    /// Overall interface status
    st: St,
    /// Interface Control
    ///
    /// Overall interface control (TBD)
    ctl: Option<u16>,
    /// Physical Access Type
    ///
    /// Type of physical media
    typ: Option<Typ>,
}

trait Model10Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Interface Status
    ///
    /// Overall interface status
    fn st(&self) -> St;

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn ctl(&self) -> Option<u16> {
        None
    }

    /// Interface Control
    ///
    /// Overall interface control (TBD)
    fn set_ctl(&mut self, value: u16) {}

    /// Physical Access Type
    ///
    /// Type of physical media
    fn typ(&self) -> Option<Typ> {
        None
    }
}

pub enum St {
    Down = 0,
    Up = 1,
    Fault = 2,
}

pub enum Typ {
    Unknown = 0,
    Internal = 1,
    TwistedPair = 2,
    Fiber = 3,
    Wireless = 4,
}
