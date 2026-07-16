/// Include a digital signature over the response
pub struct Model7 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Request Sequence
    ///
    /// Sequence number from the request
    rq_seq: u16,
    /// Status
    ///
    /// Status of last write operation
    sts: Sts,
    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    ts: u32,
    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    ms: u16,
    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Shall be advanced for each response
    seq: u16,
    /// Alarm
    ///
    /// Bitmask alarm code
    alm: Alm,
    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    alg: Alg,
    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    n: u16,
}

trait Model7Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Request Sequence
    ///
    /// Sequence number from the request
    fn rq_seq(&self) -> u16;

    /// Status
    ///
    /// Status of last write operation
    fn sts(&self) -> Sts;

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn ts(&self) -> u32;

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn ms(&self) -> u16;

    /// Sequence
    ///
    /// Sequence number of response
    ///
    /// Shall be advanced for each response
    fn seq(&self) -> u16;

    /// Alarm
    ///
    /// Bitmask alarm code
    fn alm(&self) -> Alm;

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn alg(&self) -> Alg;

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn n(&self) -> u16;

    /// N
    ///
    /// Number of registers comprising the digital signature.
    ///
    /// The value of N must be at least 4 (64 bits)
    fn set_n(&mut self, value: u16);
}

pub enum Sts {
    Success = 0,
    /// The signature was not valid
    Ds = 1,
    /// One or more registers were not writable by this role
    Acl = 2,
    /// Offset out of range or missing from multi-register value
    Off = 3,
    /// Value is out of acceptable range
    Val = 4,
}

pub enum Alm {
    None = 0,
    /// Tampered
    Alm = 1,
}

pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}
