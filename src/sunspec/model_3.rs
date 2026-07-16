/// Request a digital signature over a specified set of data registers
pub struct Model3 {
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
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    x: u16,
    /// Offset1
    ///
    /// Offset of value to read
    off1: u16,
    off2: u16,
    off3: u16,
    off4: u16,
    off5: u16,
    off6: u16,
    off7: u16,
    off8: u16,
    off9: u16,
    off10: u16,
    off11: u16,
    off12: u16,
    off13: u16,
    off14: u16,
    off15: u16,
    off16: u16,
    off17: u16,
    off18: u16,
    off19: u16,
    off20: u16,
    off21: u16,
    off22: u16,
    off23: u16,
    off24: u16,
    off25: u16,
    off26: u16,
    off27: u16,
    off28: u16,
    off29: u16,
    off30: u16,
    off31: u16,
    off32: u16,
    off33: u16,
    off34: u16,
    off35: u16,
    off36: u16,
    off37: u16,
    off38: u16,
    off39: u16,
    off40: u16,
    off41: u16,
    off42: u16,
    off43: u16,
    off44: u16,
    off45: u16,
    off46: u16,
    off47: u16,
    off48: u16,
    off49: u16,
    off50: u16,
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
    /// Digital Signature ID
    ///
    /// User's role id 0-5
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

trait Model3Trait {
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
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    fn x(&self) -> u16;

    /// X
    ///
    /// Number of registers being requested
    ///
    /// A max of 50 registers are allowed
    fn set_x(&mut self, value: u16);

    /// Offset1
    ///
    /// Offset of value to read
    fn off1(&self) -> u16;

    /// Offset1
    ///
    /// Offset of value to read
    fn set_off1(&mut self, value: u16);

    fn off2(&self) -> u16;

    fn set_off2(&mut self, value: u16);

    fn off3(&self) -> u16;

    fn set_off3(&mut self, value: u16);

    fn off4(&self) -> u16;

    fn set_off4(&mut self, value: u16);

    fn off5(&self) -> u16;

    fn set_off5(&mut self, value: u16);

    fn off6(&self) -> u16;

    fn set_off6(&mut self, value: u16);

    fn off7(&self) -> u16;

    fn set_off7(&mut self, value: u16);

    fn off8(&self) -> u16;

    fn set_off8(&mut self, value: u16);

    fn off9(&self) -> u16;

    fn set_off9(&mut self, value: u16);

    fn off10(&self) -> u16;

    fn set_off10(&mut self, value: u16);

    fn off11(&self) -> u16;

    fn set_off11(&mut self, value: u16);

    fn off12(&self) -> u16;

    fn set_off12(&mut self, value: u16);

    fn off13(&self) -> u16;

    fn set_off13(&mut self, value: u16);

    fn off14(&self) -> u16;

    fn set_off14(&mut self, value: u16);

    fn off15(&self) -> u16;

    fn set_off15(&mut self, value: u16);

    fn off16(&self) -> u16;

    fn set_off16(&mut self, value: u16);

    fn off17(&self) -> u16;

    fn set_off17(&mut self, value: u16);

    fn off18(&self) -> u16;

    fn set_off18(&mut self, value: u16);

    fn off19(&self) -> u16;

    fn set_off19(&mut self, value: u16);

    fn off20(&self) -> u16;

    fn set_off20(&mut self, value: u16);

    fn off21(&self) -> u16;

    fn set_off21(&mut self, value: u16);

    fn off22(&self) -> u16;

    fn set_off22(&mut self, value: u16);

    fn off23(&self) -> u16;

    fn set_off23(&mut self, value: u16);

    fn off24(&self) -> u16;

    fn set_off24(&mut self, value: u16);

    fn off25(&self) -> u16;

    fn set_off25(&mut self, value: u16);

    fn off26(&self) -> u16;

    fn set_off26(&mut self, value: u16);

    fn off27(&self) -> u16;

    fn set_off27(&mut self, value: u16);

    fn off28(&self) -> u16;

    fn set_off28(&mut self, value: u16);

    fn off29(&self) -> u16;

    fn set_off29(&mut self, value: u16);

    fn off30(&self) -> u16;

    fn set_off30(&mut self, value: u16);

    fn off31(&self) -> u16;

    fn set_off31(&mut self, value: u16);

    fn off32(&self) -> u16;

    fn set_off32(&mut self, value: u16);

    fn off33(&self) -> u16;

    fn set_off33(&mut self, value: u16);

    fn off34(&self) -> u16;

    fn set_off34(&mut self, value: u16);

    fn off35(&self) -> u16;

    fn set_off35(&mut self, value: u16);

    fn off36(&self) -> u16;

    fn set_off36(&mut self, value: u16);

    fn off37(&self) -> u16;

    fn set_off37(&mut self, value: u16);

    fn off38(&self) -> u16;

    fn set_off38(&mut self, value: u16);

    fn off39(&self) -> u16;

    fn set_off39(&mut self, value: u16);

    fn off40(&self) -> u16;

    fn set_off40(&mut self, value: u16);

    fn off41(&self) -> u16;

    fn set_off41(&mut self, value: u16);

    fn off42(&self) -> u16;

    fn set_off42(&mut self, value: u16);

    fn off43(&self) -> u16;

    fn set_off43(&mut self, value: u16);

    fn off44(&self) -> u16;

    fn set_off44(&mut self, value: u16);

    fn off45(&self) -> u16;

    fn set_off45(&mut self, value: u16);

    fn off46(&self) -> u16;

    fn set_off46(&mut self, value: u16);

    fn off47(&self) -> u16;

    fn set_off47(&mut self, value: u16);

    fn off48(&self) -> u16;

    fn set_off48(&mut self, value: u16);

    fn off49(&self) -> u16;

    fn set_off49(&mut self, value: u16);

    fn off50(&self) -> u16;

    fn set_off50(&mut self, value: u16);

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
    /// Digital Signature ID
    ///
    /// User's role id 0-5
    fn role(&self) -> u16;

    /// Role
    ///
    /// Digital Signature ID
    ///
    /// User's role id 0-5
    fn set_role(&mut self, value: u16);

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

pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}
