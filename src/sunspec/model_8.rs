/// Security model for PKI
pub struct Model8 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Format
    ///
    /// X.509 format of the certificate. DER or PEM.
    fmt: Fmt,
    /// N
    ///
    /// Number of registers to follow for the certificate
    n: u16,
}

trait Model8Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Format
    ///
    /// X.509 format of the certificate. DER or PEM.
    fn fmt(&self) -> Fmt;

    /// N
    ///
    /// Number of registers to follow for the certificate
    fn n(&self) -> u16;
}

pub enum Fmt {
    None = 0,
    X509Pem = 1,
    X509Der = 2,
}
