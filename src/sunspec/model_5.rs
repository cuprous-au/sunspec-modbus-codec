/// Include a digital signature along with the control data
pub struct Model5 {
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
    /// Offset1
    ///
    /// Offset of control register to write value to
    off1: u16,
    /// Value1
    ///
    /// Value to write to control register at offset
    val1: u16,
    off2: u16,
    val2: u16,
    off3: u16,
    val3: u16,
    off4: u16,
    val4: u16,
    off5: u16,
    val5: u16,
    off6: u16,
    val6: u16,
    off7: u16,
    val7: u16,
    off8: u16,
    val8: u16,
    off9: u16,
    val9: u16,
    off10: u16,
    val10: u16,
    off11: u16,
    val11: u16,
    off12: u16,
    val12: u16,
    off13: u16,
    val13: u16,
    off14: u16,
    val14: u16,
    off15: u16,
    val15: u16,
    off16: u16,
    val16: u16,
    off17: u16,
    val17: u16,
    off18: u16,
    val18: u16,
    off19: u16,
    val19: u16,
    off20: u16,
    val20: u16,
    off21: u16,
    val21: u16,
    off22: u16,
    val22: u16,
    off23: u16,
    val23: u16,
    off24: u16,
    val24: u16,
    off25: u16,
    val25: u16,
    off26: u16,
    val26: u16,
    off27: u16,
    val27: u16,
    off28: u16,
    val28: u16,
    off29: u16,
    val29: u16,
    off30: u16,
    val30: u16,
    off31: u16,
    val31: u16,
    off32: u16,
    val32: u16,
    off33: u16,
    val33: u16,
    off34: u16,
    val34: u16,
    off35: u16,
    val35: u16,
    off36: u16,
    val36: u16,
    off37: u16,
    val37: u16,
    off38: u16,
    val38: u16,
    off39: u16,
    val39: u16,
    off40: u16,
    val40: u16,
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

trait Model5Trait {
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

    /// Offset1
    ///
    /// Offset of control register to write value to
    fn off1(&self) -> u16;

    /// Offset1
    ///
    /// Offset of control register to write value to
    fn set_off1(&mut self, value: u16);

    /// Value1
    ///
    /// Value to write to control register at offset
    fn val1(&self) -> u16;

    /// Value1
    ///
    /// Value to write to control register at offset
    fn set_val1(&mut self, value: u16);

    fn off2(&self) -> u16;

    fn set_off2(&mut self, value: u16);

    fn val2(&self) -> u16;

    fn set_val2(&mut self, value: u16);

    fn off3(&self) -> u16;

    fn set_off3(&mut self, value: u16);

    fn val3(&self) -> u16;

    fn set_val3(&mut self, value: u16);

    fn off4(&self) -> u16;

    fn set_off4(&mut self, value: u16);

    fn val4(&self) -> u16;

    fn set_val4(&mut self, value: u16);

    fn off5(&self) -> u16;

    fn set_off5(&mut self, value: u16);

    fn val5(&self) -> u16;

    fn set_val5(&mut self, value: u16);

    fn off6(&self) -> u16;

    fn set_off6(&mut self, value: u16);

    fn val6(&self) -> u16;

    fn set_val6(&mut self, value: u16);

    fn off7(&self) -> u16;

    fn set_off7(&mut self, value: u16);

    fn val7(&self) -> u16;

    fn set_val7(&mut self, value: u16);

    fn off8(&self) -> u16;

    fn set_off8(&mut self, value: u16);

    fn val8(&self) -> u16;

    fn set_val8(&mut self, value: u16);

    fn off9(&self) -> u16;

    fn set_off9(&mut self, value: u16);

    fn val9(&self) -> u16;

    fn set_val9(&mut self, value: u16);

    fn off10(&self) -> u16;

    fn set_off10(&mut self, value: u16);

    fn val10(&self) -> u16;

    fn set_val10(&mut self, value: u16);

    fn off11(&self) -> u16;

    fn set_off11(&mut self, value: u16);

    fn val11(&self) -> u16;

    fn set_val11(&mut self, value: u16);

    fn off12(&self) -> u16;

    fn set_off12(&mut self, value: u16);

    fn val12(&self) -> u16;

    fn set_val12(&mut self, value: u16);

    fn off13(&self) -> u16;

    fn set_off13(&mut self, value: u16);

    fn val13(&self) -> u16;

    fn set_val13(&mut self, value: u16);

    fn off14(&self) -> u16;

    fn set_off14(&mut self, value: u16);

    fn val14(&self) -> u16;

    fn set_val14(&mut self, value: u16);

    fn off15(&self) -> u16;

    fn set_off15(&mut self, value: u16);

    fn val15(&self) -> u16;

    fn set_val15(&mut self, value: u16);

    fn off16(&self) -> u16;

    fn set_off16(&mut self, value: u16);

    fn val16(&self) -> u16;

    fn set_val16(&mut self, value: u16);

    fn off17(&self) -> u16;

    fn set_off17(&mut self, value: u16);

    fn val17(&self) -> u16;

    fn set_val17(&mut self, value: u16);

    fn off18(&self) -> u16;

    fn set_off18(&mut self, value: u16);

    fn val18(&self) -> u16;

    fn set_val18(&mut self, value: u16);

    fn off19(&self) -> u16;

    fn set_off19(&mut self, value: u16);

    fn val19(&self) -> u16;

    fn set_val19(&mut self, value: u16);

    fn off20(&self) -> u16;

    fn set_off20(&mut self, value: u16);

    fn val20(&self) -> u16;

    fn set_val20(&mut self, value: u16);

    fn off21(&self) -> u16;

    fn set_off21(&mut self, value: u16);

    fn val21(&self) -> u16;

    fn set_val21(&mut self, value: u16);

    fn off22(&self) -> u16;

    fn set_off22(&mut self, value: u16);

    fn val22(&self) -> u16;

    fn set_val22(&mut self, value: u16);

    fn off23(&self) -> u16;

    fn set_off23(&mut self, value: u16);

    fn val23(&self) -> u16;

    fn set_val23(&mut self, value: u16);

    fn off24(&self) -> u16;

    fn set_off24(&mut self, value: u16);

    fn val24(&self) -> u16;

    fn set_val24(&mut self, value: u16);

    fn off25(&self) -> u16;

    fn set_off25(&mut self, value: u16);

    fn val25(&self) -> u16;

    fn set_val25(&mut self, value: u16);

    fn off26(&self) -> u16;

    fn set_off26(&mut self, value: u16);

    fn val26(&self) -> u16;

    fn set_val26(&mut self, value: u16);

    fn off27(&self) -> u16;

    fn set_off27(&mut self, value: u16);

    fn val27(&self) -> u16;

    fn set_val27(&mut self, value: u16);

    fn off28(&self) -> u16;

    fn set_off28(&mut self, value: u16);

    fn val28(&self) -> u16;

    fn set_val28(&mut self, value: u16);

    fn off29(&self) -> u16;

    fn set_off29(&mut self, value: u16);

    fn val29(&self) -> u16;

    fn set_val29(&mut self, value: u16);

    fn off30(&self) -> u16;

    fn set_off30(&mut self, value: u16);

    fn val30(&self) -> u16;

    fn set_val30(&mut self, value: u16);

    fn off31(&self) -> u16;

    fn set_off31(&mut self, value: u16);

    fn val31(&self) -> u16;

    fn set_val31(&mut self, value: u16);

    fn off32(&self) -> u16;

    fn set_off32(&mut self, value: u16);

    fn val32(&self) -> u16;

    fn set_val32(&mut self, value: u16);

    fn off33(&self) -> u16;

    fn set_off33(&mut self, value: u16);

    fn val33(&self) -> u16;

    fn set_val33(&mut self, value: u16);

    fn off34(&self) -> u16;

    fn set_off34(&mut self, value: u16);

    fn val34(&self) -> u16;

    fn set_val34(&mut self, value: u16);

    fn off35(&self) -> u16;

    fn set_off35(&mut self, value: u16);

    fn val35(&self) -> u16;

    fn set_val35(&mut self, value: u16);

    fn off36(&self) -> u16;

    fn set_off36(&mut self, value: u16);

    fn val36(&self) -> u16;

    fn set_val36(&mut self, value: u16);

    fn off37(&self) -> u16;

    fn set_off37(&mut self, value: u16);

    fn val37(&self) -> u16;

    fn set_val37(&mut self, value: u16);

    fn off38(&self) -> u16;

    fn set_off38(&mut self, value: u16);

    fn val38(&self) -> u16;

    fn set_val38(&mut self, value: u16);

    fn off39(&self) -> u16;

    fn set_off39(&mut self, value: u16);

    fn val39(&self) -> u16;

    fn set_val39(&mut self, value: u16);

    fn off40(&self) -> u16;

    fn set_off40(&mut self, value: u16);

    fn val40(&self) -> u16;

    fn set_val40(&mut self, value: u16);

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
