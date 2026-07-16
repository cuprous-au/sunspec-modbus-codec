/// Security model for PKI
pub struct Model9 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Cert_UID
    ///
    /// User ID for this certificate
    cert_uid: u16,
    /// Cert_Role
    ///
    /// Role for this certificate
    cert_role: u16,
    /// Format
    ///
    /// Format of this certificate
    fmt: Fmt,
    /// Type
    ///
    /// Type of this certificate
    typ: Typ,
    /// Total Length
    ///
    /// Total Length of the Certificate
    ///
    /// In registers, zero padded.
    tot_ln: u16,
    /// Fragment length
    ///
    /// Length of this fragment
    ///
    /// Maximum fragment length is 80 registers
    frg_ln: u16,
    /// Frag1
    ///
    /// First word of this fragment
    frg1: u16,
    frg2: u16,
    frg3: u16,
    frg4: u16,
    frg5: u16,
    frg6: u16,
    frg7: u16,
    frg8: u16,
    frg9: u16,
    frg10: u16,
    frg11: u16,
    frg12: u16,
    frg13: u16,
    frg14: u16,
    frg15: u16,
    frg16: u16,
    frg17: u16,
    frg18: u16,
    frg19: u16,
    frg20: u16,
    frg21: u16,
    frg22: u16,
    frg23: u16,
    frg24: u16,
    frg25: u16,
    frg26: u16,
    frg27: u16,
    frg28: u16,
    frg29: u16,
    frg30: u16,
    frg31: u16,
    frg32: u16,
    frg33: u16,
    frg34: u16,
    frg35: u16,
    frg36: u16,
    frg37: u16,
    frg38: u16,
    frg39: u16,
    frg40: u16,
    frg41: u16,
    frg42: u16,
    frg43: u16,
    frg44: u16,
    frg45: u16,
    frg46: u16,
    frg47: u16,
    frg48: u16,
    frg49: u16,
    frg50: u16,
    frg51: u16,
    frg52: u16,
    frg53: u16,
    frg54: u16,
    frg55: u16,
    frg56: u16,
    frg57: u16,
    frg58: u16,
    frg59: u16,
    frg60: u16,
    frg61: u16,
    frg62: u16,
    frg63: u16,
    frg64: u16,
    frg65: u16,
    frg66: u16,
    frg67: u16,
    frg68: u16,
    frg69: u16,
    frg70: u16,
    frg71: u16,
    frg72: u16,
    frg73: u16,
    frg74: u16,
    frg75: u16,
    frg78: u16,
    frg79: u16,
    /// Frag80
    ///
    /// Last word of this fragment
    frg80: u16,
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
    /// UID
    ///
    /// User ID for the request signature
    uid: u16,
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
    /// Number of registers to follow for the certificate
    n: u16,
}

trait Model9Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Cert_UID
    ///
    /// User ID for this certificate
    fn cert_uid(&self) -> u16;

    /// Cert_UID
    ///
    /// User ID for this certificate
    fn set_cert_uid(&mut self, value: u16);

    /// Cert_Role
    ///
    /// Role for this certificate
    fn cert_role(&self) -> u16;

    /// Cert_Role
    ///
    /// Role for this certificate
    fn set_cert_role(&mut self, value: u16);

    /// Format
    ///
    /// Format of this certificate
    fn fmt(&self) -> Fmt;

    /// Format
    ///
    /// Format of this certificate
    fn set_fmt(&mut self, value: Fmt);

    /// Type
    ///
    /// Type of this certificate
    fn typ(&self) -> Typ;

    /// Type
    ///
    /// Type of this certificate
    fn set_typ(&mut self, value: Typ);

    /// Total Length
    ///
    /// Total Length of the Certificate
    ///
    /// In registers, zero padded.
    fn tot_ln(&self) -> u16;

    /// Total Length
    ///
    /// Total Length of the Certificate
    ///
    /// In registers, zero padded.
    fn set_tot_ln(&mut self, value: u16);

    /// Fragment length
    ///
    /// Length of this fragment
    ///
    /// Maximum fragment length is 80 registers
    fn frg_ln(&self) -> u16;

    /// Fragment length
    ///
    /// Length of this fragment
    ///
    /// Maximum fragment length is 80 registers
    fn set_frg_ln(&mut self, value: u16);

    /// Frag1
    ///
    /// First word of this fragment
    fn frg1(&self) -> u16;

    /// Frag1
    ///
    /// First word of this fragment
    fn set_frg1(&mut self, value: u16);

    fn frg2(&self) -> u16;

    fn set_frg2(&mut self, value: u16);

    fn frg3(&self) -> u16;

    fn set_frg3(&mut self, value: u16);

    fn frg4(&self) -> u16;

    fn set_frg4(&mut self, value: u16);

    fn frg5(&self) -> u16;

    fn set_frg5(&mut self, value: u16);

    fn frg6(&self) -> u16;

    fn set_frg6(&mut self, value: u16);

    fn frg7(&self) -> u16;

    fn set_frg7(&mut self, value: u16);

    fn frg8(&self) -> u16;

    fn set_frg8(&mut self, value: u16);

    fn frg9(&self) -> u16;

    fn set_frg9(&mut self, value: u16);

    fn frg10(&self) -> u16;

    fn set_frg10(&mut self, value: u16);

    fn frg11(&self) -> u16;

    fn set_frg11(&mut self, value: u16);

    fn frg12(&self) -> u16;

    fn set_frg12(&mut self, value: u16);

    fn frg13(&self) -> u16;

    fn set_frg13(&mut self, value: u16);

    fn frg14(&self) -> u16;

    fn set_frg14(&mut self, value: u16);

    fn frg15(&self) -> u16;

    fn set_frg15(&mut self, value: u16);

    fn frg16(&self) -> u16;

    fn set_frg16(&mut self, value: u16);

    fn frg17(&self) -> u16;

    fn set_frg17(&mut self, value: u16);

    fn frg18(&self) -> u16;

    fn set_frg18(&mut self, value: u16);

    fn frg19(&self) -> u16;

    fn set_frg19(&mut self, value: u16);

    fn frg20(&self) -> u16;

    fn set_frg20(&mut self, value: u16);

    fn frg21(&self) -> u16;

    fn set_frg21(&mut self, value: u16);

    fn frg22(&self) -> u16;

    fn set_frg22(&mut self, value: u16);

    fn frg23(&self) -> u16;

    fn set_frg23(&mut self, value: u16);

    fn frg24(&self) -> u16;

    fn set_frg24(&mut self, value: u16);

    fn frg25(&self) -> u16;

    fn set_frg25(&mut self, value: u16);

    fn frg26(&self) -> u16;

    fn set_frg26(&mut self, value: u16);

    fn frg27(&self) -> u16;

    fn set_frg27(&mut self, value: u16);

    fn frg28(&self) -> u16;

    fn set_frg28(&mut self, value: u16);

    fn frg29(&self) -> u16;

    fn set_frg29(&mut self, value: u16);

    fn frg30(&self) -> u16;

    fn set_frg30(&mut self, value: u16);

    fn frg31(&self) -> u16;

    fn set_frg31(&mut self, value: u16);

    fn frg32(&self) -> u16;

    fn set_frg32(&mut self, value: u16);

    fn frg33(&self) -> u16;

    fn set_frg33(&mut self, value: u16);

    fn frg34(&self) -> u16;

    fn set_frg34(&mut self, value: u16);

    fn frg35(&self) -> u16;

    fn set_frg35(&mut self, value: u16);

    fn frg36(&self) -> u16;

    fn set_frg36(&mut self, value: u16);

    fn frg37(&self) -> u16;

    fn set_frg37(&mut self, value: u16);

    fn frg38(&self) -> u16;

    fn set_frg38(&mut self, value: u16);

    fn frg39(&self) -> u16;

    fn set_frg39(&mut self, value: u16);

    fn frg40(&self) -> u16;

    fn set_frg40(&mut self, value: u16);

    fn frg41(&self) -> u16;

    fn set_frg41(&mut self, value: u16);

    fn frg42(&self) -> u16;

    fn set_frg42(&mut self, value: u16);

    fn frg43(&self) -> u16;

    fn set_frg43(&mut self, value: u16);

    fn frg44(&self) -> u16;

    fn set_frg44(&mut self, value: u16);

    fn frg45(&self) -> u16;

    fn set_frg45(&mut self, value: u16);

    fn frg46(&self) -> u16;

    fn set_frg46(&mut self, value: u16);

    fn frg47(&self) -> u16;

    fn set_frg47(&mut self, value: u16);

    fn frg48(&self) -> u16;

    fn set_frg48(&mut self, value: u16);

    fn frg49(&self) -> u16;

    fn set_frg49(&mut self, value: u16);

    fn frg50(&self) -> u16;

    fn set_frg50(&mut self, value: u16);

    fn frg51(&self) -> u16;

    fn set_frg51(&mut self, value: u16);

    fn frg52(&self) -> u16;

    fn set_frg52(&mut self, value: u16);

    fn frg53(&self) -> u16;

    fn set_frg53(&mut self, value: u16);

    fn frg54(&self) -> u16;

    fn set_frg54(&mut self, value: u16);

    fn frg55(&self) -> u16;

    fn set_frg55(&mut self, value: u16);

    fn frg56(&self) -> u16;

    fn set_frg56(&mut self, value: u16);

    fn frg57(&self) -> u16;

    fn set_frg57(&mut self, value: u16);

    fn frg58(&self) -> u16;

    fn set_frg58(&mut self, value: u16);

    fn frg59(&self) -> u16;

    fn set_frg59(&mut self, value: u16);

    fn frg60(&self) -> u16;

    fn set_frg60(&mut self, value: u16);

    fn frg61(&self) -> u16;

    fn set_frg61(&mut self, value: u16);

    fn frg62(&self) -> u16;

    fn set_frg62(&mut self, value: u16);

    fn frg63(&self) -> u16;

    fn set_frg63(&mut self, value: u16);

    fn frg64(&self) -> u16;

    fn set_frg64(&mut self, value: u16);

    fn frg65(&self) -> u16;

    fn set_frg65(&mut self, value: u16);

    fn frg66(&self) -> u16;

    fn set_frg66(&mut self, value: u16);

    fn frg67(&self) -> u16;

    fn set_frg67(&mut self, value: u16);

    fn frg68(&self) -> u16;

    fn set_frg68(&mut self, value: u16);

    fn frg69(&self) -> u16;

    fn set_frg69(&mut self, value: u16);

    fn frg70(&self) -> u16;

    fn set_frg70(&mut self, value: u16);

    fn frg71(&self) -> u16;

    fn set_frg71(&mut self, value: u16);

    fn frg72(&self) -> u16;

    fn set_frg72(&mut self, value: u16);

    fn frg73(&self) -> u16;

    fn set_frg73(&mut self, value: u16);

    fn frg74(&self) -> u16;

    fn set_frg74(&mut self, value: u16);

    fn frg75(&self) -> u16;

    fn set_frg75(&mut self, value: u16);

    fn frg78(&self) -> u16;

    fn set_frg78(&mut self, value: u16);

    fn frg79(&self) -> u16;

    fn set_frg79(&mut self, value: u16);

    /// Frag80
    ///
    /// Last word of this fragment
    fn frg80(&self) -> u16;

    /// Frag80
    ///
    /// Last word of this fragment
    fn set_frg80(&mut self, value: u16);

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

    /// UID
    ///
    /// User ID for the request signature
    fn uid(&self) -> u16;

    /// UID
    ///
    /// User ID for the request signature
    fn set_uid(&mut self, value: u16);

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
    /// Number of registers to follow for the certificate
    fn n(&self) -> u16;

    /// N
    ///
    /// Number of registers to follow for the certificate
    fn set_n(&mut self, value: u16);
}

pub enum Fmt {
    None = 0,
    X509Pem = 1,
    X509Der = 2,
}

pub enum Typ {
    DevKeyPair = 0,
    DevSharedKey = 1,
    OperatorPub = 2,
    OperatorShared = 3,
    CaPub = 4,
}

pub enum Alg {
    /// For test purposes only
    None = 0,
    AesGmac64 = 1,
    Ecc256 = 2,
}
