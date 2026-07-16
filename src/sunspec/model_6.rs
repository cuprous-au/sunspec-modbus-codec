/// Include a digital signature along with the control data
pub struct Model6 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    x: u16,
    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// X values to follow
    off: u16,
    /// Value1
    ///
    /// Value to write to control register at offset
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
    val51: u16,
    val52: u16,
    val53: u16,
    val54: u16,
    val55: u16,
    val56: u16,
    val57: u16,
    val58: u16,
    val59: u16,
    val60: u16,
    val61: u16,
    val62: u16,
    val63: u16,
    val64: u16,
    val65: u16,
    val66: u16,
    val67: u16,
    val68: u16,
    val69: u16,
    val70: u16,
    val71: u16,
    val72: u16,
    val73: u16,
    val74: u16,
    val75: u16,
    val76: u16,
    val77: u16,
    val78: u16,
    val79: u16,
    val80: u16,
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
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    seq: u16,
    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    role: u16,
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

trait Model6Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn x(&self) -> u16;

    /// X
    ///
    /// Number of (offset, value) pairs being written
    ///
    /// A max of 50 (offset, value) pairs are allocated
    fn set_x(&mut self, value: u16);

    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// X values to follow
    fn off(&self) -> u16;

    /// Offset
    ///
    /// Starting offset for write operation
    ///
    /// X values to follow
    fn set_off(&mut self, value: u16);

    /// Value1
    ///
    /// Value to write to control register at offset
    fn val1(&self) -> u16;

    /// Value1
    ///
    /// Value to write to control register at offset
    fn set_val1(&mut self, value: u16);

    fn val2(&self) -> u16;

    fn set_val2(&mut self, value: u16);

    fn val3(&self) -> u16;

    fn set_val3(&mut self, value: u16);

    fn val4(&self) -> u16;

    fn set_val4(&mut self, value: u16);

    fn val5(&self) -> u16;

    fn set_val5(&mut self, value: u16);

    fn val6(&self) -> u16;

    fn set_val6(&mut self, value: u16);

    fn val7(&self) -> u16;

    fn set_val7(&mut self, value: u16);

    fn val8(&self) -> u16;

    fn set_val8(&mut self, value: u16);

    fn val9(&self) -> u16;

    fn set_val9(&mut self, value: u16);

    fn val10(&self) -> u16;

    fn set_val10(&mut self, value: u16);

    fn val11(&self) -> u16;

    fn set_val11(&mut self, value: u16);

    fn val12(&self) -> u16;

    fn set_val12(&mut self, value: u16);

    fn val13(&self) -> u16;

    fn set_val13(&mut self, value: u16);

    fn val14(&self) -> u16;

    fn set_val14(&mut self, value: u16);

    fn val15(&self) -> u16;

    fn set_val15(&mut self, value: u16);

    fn val16(&self) -> u16;

    fn set_val16(&mut self, value: u16);

    fn val17(&self) -> u16;

    fn set_val17(&mut self, value: u16);

    fn val18(&self) -> u16;

    fn set_val18(&mut self, value: u16);

    fn val19(&self) -> u16;

    fn set_val19(&mut self, value: u16);

    fn val20(&self) -> u16;

    fn set_val20(&mut self, value: u16);

    fn val21(&self) -> u16;

    fn set_val21(&mut self, value: u16);

    fn val22(&self) -> u16;

    fn set_val22(&mut self, value: u16);

    fn val23(&self) -> u16;

    fn set_val23(&mut self, value: u16);

    fn val24(&self) -> u16;

    fn set_val24(&mut self, value: u16);

    fn val25(&self) -> u16;

    fn set_val25(&mut self, value: u16);

    fn val26(&self) -> u16;

    fn set_val26(&mut self, value: u16);

    fn val27(&self) -> u16;

    fn set_val27(&mut self, value: u16);

    fn val28(&self) -> u16;

    fn set_val28(&mut self, value: u16);

    fn val29(&self) -> u16;

    fn set_val29(&mut self, value: u16);

    fn val30(&self) -> u16;

    fn set_val30(&mut self, value: u16);

    fn val31(&self) -> u16;

    fn set_val31(&mut self, value: u16);

    fn val32(&self) -> u16;

    fn set_val32(&mut self, value: u16);

    fn val33(&self) -> u16;

    fn set_val33(&mut self, value: u16);

    fn val34(&self) -> u16;

    fn set_val34(&mut self, value: u16);

    fn val35(&self) -> u16;

    fn set_val35(&mut self, value: u16);

    fn val36(&self) -> u16;

    fn set_val36(&mut self, value: u16);

    fn val37(&self) -> u16;

    fn set_val37(&mut self, value: u16);

    fn val38(&self) -> u16;

    fn set_val38(&mut self, value: u16);

    fn val39(&self) -> u16;

    fn set_val39(&mut self, value: u16);

    fn val40(&self) -> u16;

    fn set_val40(&mut self, value: u16);

    fn val41(&self) -> u16;

    fn set_val41(&mut self, value: u16);

    fn val42(&self) -> u16;

    fn set_val42(&mut self, value: u16);

    fn val43(&self) -> u16;

    fn set_val43(&mut self, value: u16);

    fn val44(&self) -> u16;

    fn set_val44(&mut self, value: u16);

    fn val45(&self) -> u16;

    fn set_val45(&mut self, value: u16);

    fn val46(&self) -> u16;

    fn set_val46(&mut self, value: u16);

    fn val47(&self) -> u16;

    fn set_val47(&mut self, value: u16);

    fn val48(&self) -> u16;

    fn set_val48(&mut self, value: u16);

    fn val49(&self) -> u16;

    fn set_val49(&mut self, value: u16);

    fn val50(&self) -> u16;

    fn set_val50(&mut self, value: u16);

    fn val51(&self) -> u16;

    fn set_val51(&mut self, value: u16);

    fn val52(&self) -> u16;

    fn set_val52(&mut self, value: u16);

    fn val53(&self) -> u16;

    fn set_val53(&mut self, value: u16);

    fn val54(&self) -> u16;

    fn set_val54(&mut self, value: u16);

    fn val55(&self) -> u16;

    fn set_val55(&mut self, value: u16);

    fn val56(&self) -> u16;

    fn set_val56(&mut self, value: u16);

    fn val57(&self) -> u16;

    fn set_val57(&mut self, value: u16);

    fn val58(&self) -> u16;

    fn set_val58(&mut self, value: u16);

    fn val59(&self) -> u16;

    fn set_val59(&mut self, value: u16);

    fn val60(&self) -> u16;

    fn set_val60(&mut self, value: u16);

    fn val61(&self) -> u16;

    fn set_val61(&mut self, value: u16);

    fn val62(&self) -> u16;

    fn set_val62(&mut self, value: u16);

    fn val63(&self) -> u16;

    fn set_val63(&mut self, value: u16);

    fn val64(&self) -> u16;

    fn set_val64(&mut self, value: u16);

    fn val65(&self) -> u16;

    fn set_val65(&mut self, value: u16);

    fn val66(&self) -> u16;

    fn set_val66(&mut self, value: u16);

    fn val67(&self) -> u16;

    fn set_val67(&mut self, value: u16);

    fn val68(&self) -> u16;

    fn set_val68(&mut self, value: u16);

    fn val69(&self) -> u16;

    fn set_val69(&mut self, value: u16);

    fn val70(&self) -> u16;

    fn set_val70(&mut self, value: u16);

    fn val71(&self) -> u16;

    fn set_val71(&mut self, value: u16);

    fn val72(&self) -> u16;

    fn set_val72(&mut self, value: u16);

    fn val73(&self) -> u16;

    fn set_val73(&mut self, value: u16);

    fn val74(&self) -> u16;

    fn set_val74(&mut self, value: u16);

    fn val75(&self) -> u16;

    fn set_val75(&mut self, value: u16);

    fn val76(&self) -> u16;

    fn set_val76(&mut self, value: u16);

    fn val77(&self) -> u16;

    fn set_val77(&mut self, value: u16);

    fn val78(&self) -> u16;

    fn set_val78(&mut self, value: u16);

    fn val79(&self) -> u16;

    fn set_val79(&mut self, value: u16);

    fn val80(&self) -> u16;

    fn set_val80(&mut self, value: u16);

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn ts(&self) -> u32;

    /// Timestamp
    ///
    /// Timestamp value is the number of seconds since January 1, 2000
    fn set_ts(&mut self, value: u32);

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn ms(&self) -> u16;

    /// Milliseconds
    ///
    /// Millisecond counter 0-999
    fn set_ms(&mut self, value: u16);

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn seq(&self) -> u16;

    /// Sequence
    ///
    /// Sequence number of request
    ///
    /// Shall be advanced for each request
    fn set_seq(&mut self, value: u16);

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn role(&self) -> u16;

    /// Role
    ///
    /// Signing key used 0-5
    ///
    /// Each controller is assigned a key index that maps to their access control role
    fn set_role(&mut self, value: u16);

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn alg(&self) -> Alg;

    /// Algorithm
    ///
    /// Algorithm used to compute the digital signature
    ///
    /// For future proof
    fn set_alg(&mut self, value: Alg);

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

pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}
