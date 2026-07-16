use heapless::String;

/// Include this block to allow for a proxy server
pub struct Model14 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// name
    ///
    /// Interface name (8 chars)
    nam: Option<String<8>>,
    /// Capabilities
    ///
    /// Proxy configuration capabilities
    cap: u16,
    /// Config
    ///
    /// Set proxy address type
    cfg: Cfg,
    /// Type
    ///
    /// Enumerate value. Proxy server type
    typ: u16,
    /// Address
    ///
    /// IPv4 or IPv6 proxy hostname or dotted address (40 chars)
    addr: String<40>,
    /// Port
    ///
    /// Proxy port number
    port: u16,
    /// Username
    ///
    /// Proxy user name
    user: Option<String<24>>,
    /// Password
    ///
    /// Proxy password
    pw: Option<String<24>>,
}

trait Model14Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// name
    ///
    /// Interface name (8 chars)
    fn nam(&self) -> Option<String<8>> {
        None
    }

    /// name
    ///
    /// Interface name (8 chars)
    fn set_nam(&mut self, value: String<8>) {}

    /// Capabilities
    ///
    /// Proxy configuration capabilities
    fn cap(&self) -> u16;

    /// Capabilities
    ///
    /// Proxy configuration capabilities
    fn set_cap(&mut self, value: u16);

    /// Config
    ///
    /// Set proxy address type
    fn cfg(&self) -> Cfg;

    /// Config
    ///
    /// Set proxy address type
    fn set_cfg(&mut self, value: Cfg);

    /// Type
    ///
    /// Enumerate value. Proxy server type
    fn typ(&self) -> u16;

    /// Type
    ///
    /// Enumerate value. Proxy server type
    fn set_typ(&mut self, value: u16);

    /// Address
    ///
    /// IPv4 or IPv6 proxy hostname or dotted address (40 chars)
    fn addr(&self) -> String<40>;

    /// Address
    ///
    /// IPv4 or IPv6 proxy hostname or dotted address (40 chars)
    fn set_addr(&mut self, value: String<40>);

    /// Port
    ///
    /// Proxy port number
    fn port(&self) -> u16;

    /// Port
    ///
    /// Proxy port number
    fn set_port(&mut self, value: u16);

    /// Username
    ///
    /// Proxy user name
    fn user(&self) -> Option<String<24>> {
        None
    }

    /// Username
    ///
    /// Proxy user name
    fn set_user(&mut self, value: String<24>) {}

    /// Password
    ///
    /// Proxy password
    fn pw(&self) -> Option<String<24>> {
        None
    }

    /// Password
    ///
    /// Proxy password
    fn set_pw(&mut self, value: String<24>) {}
}

pub enum Cfg {}
