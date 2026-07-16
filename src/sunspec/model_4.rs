/// Compute a digital signature over a specified set of data registers
pub struct Model4 {
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
    /// Status of last read operation
    sts: Sts,
    /// X
    ///
    /// Number of values from the request
    ///
    /// A max of 50 values are allocated
    x: u16,
    /// Value1
    ///
    /// Copy of value from register Off1.
    ///
    /// Unused values shall return 0xFFFF (unimplemented)
    val1: u16,
    val2: u16,
    val3: u16,
    val4: u16,
    val5: u16,
    val6: u16,
    val7: u16,
    val8: u16,
    val9: u16,
    val10: u16,
    val11: u16,
    val12: u16,
    val13: u16,
    val14: u16,
    val15: u16,
    val16: u16,
    val17: u16,
    val18: u16,
    val19: u16,
    val20: u16,
    val21: u16,
    val22: u16,
    val23: u16,
    val24: u16,
    val25: u16,
    val26: u16,
    val27: u16,
    val28: u16,
    val29: u16,
    val30: u16,
    val31: u16,
    val32: u16,
    val33: u16,
    val34: u16,
    val35: u16,
    val36: u16,
    val37: u16,
    val38: u16,
    val39: u16,
    val40: u16,
    val41: u16,
    val42: u16,
    val43: u16,
    val44: u16,
    val45: u16,
    val46: u16,
    val47: u16,
    val48: u16,
    val49: u16,
    val50: u16,
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

trait Model4Trait {
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
    /// Status of last read operation
    fn sts(&self) -> Sts;

    /// X
    ///
    /// Number of values from the request
    ///
    /// A max of 50 values are allocated
    fn x(&self) -> u16;

    /// Value1
    ///
    /// Copy of value from register Off1.
    ///
    /// Unused values shall return 0xFFFF (unimplemented)
    fn val1(&self) -> u16;

    fn val2(&self) -> u16;

    fn val3(&self) -> u16;

    fn val4(&self) -> u16;

    fn val5(&self) -> u16;

    fn val6(&self) -> u16;

    fn val7(&self) -> u16;

    fn val8(&self) -> u16;

    fn val9(&self) -> u16;

    fn val10(&self) -> u16;

    fn val11(&self) -> u16;

    fn val12(&self) -> u16;

    fn val13(&self) -> u16;

    fn val14(&self) -> u16;

    fn val15(&self) -> u16;

    fn val16(&self) -> u16;

    fn val17(&self) -> u16;

    fn val18(&self) -> u16;

    fn val19(&self) -> u16;

    fn val20(&self) -> u16;

    fn val21(&self) -> u16;

    fn val22(&self) -> u16;

    fn val23(&self) -> u16;

    fn val24(&self) -> u16;

    fn val25(&self) -> u16;

    fn val26(&self) -> u16;

    fn val27(&self) -> u16;

    fn val28(&self) -> u16;

    fn val29(&self) -> u16;

    fn val30(&self) -> u16;

    fn val31(&self) -> u16;

    fn val32(&self) -> u16;

    fn val33(&self) -> u16;

    fn val34(&self) -> u16;

    fn val35(&self) -> u16;

    fn val36(&self) -> u16;

    fn val37(&self) -> u16;

    fn val38(&self) -> u16;

    fn val39(&self) -> u16;

    fn val40(&self) -> u16;

    fn val41(&self) -> u16;

    fn val42(&self) -> u16;

    fn val43(&self) -> u16;

    fn val44(&self) -> u16;

    fn val45(&self) -> u16;

    fn val46(&self) -> u16;

    fn val47(&self) -> u16;

    fn val48(&self) -> u16;

    fn val49(&self) -> u16;

    fn val50(&self) -> u16;

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
}

pub enum Sts {
    Success = 0,
    Ds = 1,
    /// One or more registers were not writable by this role
    Acl = 2,
    /// Offset out of range or missing from multi-register value
    Off = 3,
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
