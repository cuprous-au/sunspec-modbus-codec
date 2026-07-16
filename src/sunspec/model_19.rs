use heapless::String;

/// Include this model to configure a Point-to-Point Protocol link
pub struct Model19 {
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
    /// Interface name
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
    /// Authentication
    ///
    /// Authentication method
    auth: Option<Auth>,
    /// Username
    ///
    /// Username for authentication
    usr_nam: Option<String<24>>,
    /// Password
    ///
    /// Password for authentication
    pw: Option<String<12>>,
}

trait Model19Trait {
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
    /// Interface name
    fn nam(&self) -> Option<String<8>> {
        None
    }

    /// Name
    ///
    /// Interface name
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

    /// Authentication
    ///
    /// Authentication method
    fn auth(&self) -> Option<Auth> {
        None
    }

    /// Username
    ///
    /// Username for authentication
    fn usr_nam(&self) -> Option<String<24>> {
        None
    }

    /// Password
    ///
    /// Password for authentication
    fn pw(&self) -> Option<String<12>> {
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

pub enum Auth {
    None = 0,
    Pap = 1,
    Chap = 2,
}
