use heapless::String;

/// Include this model for serial interface configuration support
pub struct Model17 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Name
    ///
    /// Interface name (8 chars)
    nam: Option<String<8>>,
    /// Rate
    ///
    /// Interface baud rate in bits per second
    rte: u32,
    /// Bits
    ///
    /// Number of data bits per character
    bits: u16,
    /// Parity
    ///
    /// Parity setting
    pty: Pty,
    /// Duplex
    ///
    /// Duplex mode
    dup: Option<Dup>,
    /// Flow Control
    ///
    /// Flow Control Method
    flw: Option<Flw>,
    /// Interface Type
    ///
    /// Interface type
    typ: Option<Typ>,
    /// Protocol
    ///
    /// Serial protocol selection
    pcol: Option<Pcol>,
}

trait Model17Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Name
    ///
    /// Interface name (8 chars)
    fn nam(&self) -> Option<String<8>> {
        None
    }

    /// Name
    ///
    /// Interface name (8 chars)
    fn set_nam(&mut self, value: String<8>) {}

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn rte(&self) -> u32;

    /// Rate
    ///
    /// Interface baud rate in bits per second
    fn set_rte(&mut self, value: u32);

    /// Bits
    ///
    /// Number of data bits per character
    fn bits(&self) -> u16;

    /// Bits
    ///
    /// Number of data bits per character
    fn set_bits(&mut self, value: u16);

    /// Parity
    ///
    /// Parity setting
    fn pty(&self) -> Pty;

    /// Parity
    ///
    /// Parity setting
    fn set_pty(&mut self, value: Pty);

    /// Duplex
    ///
    /// Duplex mode
    fn dup(&self) -> Option<Dup> {
        None
    }

    /// Duplex
    ///
    /// Duplex mode
    fn set_dup(&mut self, value: Dup) {}

    /// Flow Control
    ///
    /// Flow Control Method
    fn flw(&self) -> Option<Flw> {
        None
    }

    /// Flow Control
    ///
    /// Flow Control Method
    fn set_flw(&mut self, value: Flw) {}

    /// Interface Type
    ///
    /// Interface type
    fn typ(&self) -> Option<Typ> {
        None
    }

    /// Protocol
    ///
    /// Serial protocol selection
    fn pcol(&self) -> Option<Pcol> {
        None
    }
}

pub enum Pty {
    None = 0,
    Odd = 1,
    Even = 2,
}

pub enum Dup {
    Full = 0,
    Half = 1,
}

pub enum Flw {
    None = 0,
    Hw = 1,
    Xonxoff = 2,
}

pub enum Typ {
    Unknown = 0,
    Rs232 = 1,
    Rs485 = 2,
}

pub enum Pcol {
    Unknown = 0,
    Modbus = 1,
    Vendor = 2,
}
