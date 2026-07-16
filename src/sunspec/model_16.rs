use heapless::String;

/// Include this model for a simple IPv4 network stack
pub struct Model16 {
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
    /// Interface name. (8 chars)
    nam: Option<String<8>>,
    /// Config
    ///
    /// Force IPv4 configuration method
    cfg: Cfg,
    /// Control
    ///
    /// Configure use of services
    ctl: u16,
    /// Address
    ///
    /// IP address
    addr: String<16>,
    /// Netmask
    ///
    /// Netmask
    msk: String<16>,
    /// Gateway
    ///
    /// Gateway IP address
    gw: Option<String<16>>,
    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    dns1: Option<String<16>>,
    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    dns2: Option<String<16>>,
    /// MAC
    ///
    /// IEEE MAC address of this interface
    mac: Option<[u8; 6]>,
    /// Link Control
    ///
    /// Link control flags
    lnk_ctl: Option<u16>,
}

trait Model16Trait {
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
    /// Interface name. (8 chars)
    fn nam(&self) -> Option<String<8>> {
        None
    }

    /// Name
    ///
    /// Interface name. (8 chars)
    fn set_nam(&mut self, value: String<8>) {}

    /// Config
    ///
    /// Force IPv4 configuration method
    fn cfg(&self) -> Cfg;

    /// Control
    ///
    /// Configure use of services
    fn ctl(&self) -> u16;

    /// Control
    ///
    /// Configure use of services
    fn set_ctl(&mut self, value: u16);

    /// Address
    ///
    /// IP address
    fn addr(&self) -> String<16>;

    /// Address
    ///
    /// IP address
    fn set_addr(&mut self, value: String<16>);

    /// Netmask
    ///
    /// Netmask
    fn msk(&self) -> String<16>;

    /// Netmask
    ///
    /// Netmask
    fn set_msk(&mut self, value: String<16>);

    /// Gateway
    ///
    /// Gateway IP address
    fn gw(&self) -> Option<String<16>> {
        None
    }

    /// Gateway
    ///
    /// Gateway IP address
    fn set_gw(&mut self, value: String<16>) {}

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn dns1(&self) -> Option<String<16>> {
        None
    }

    /// DNS1
    ///
    /// 32 bit IP address of DNS server
    fn set_dns1(&mut self, value: String<16>) {}

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn dns2(&self) -> Option<String<16>> {
        None
    }

    /// DNS2
    ///
    /// 32 bit IP address of DNS server
    fn set_dns2(&mut self, value: String<16>) {}

    /// MAC
    ///
    /// IEEE MAC address of this interface
    fn mac(&self) -> Option<[u8; 6]> {
        None
    }

    /// Link Control
    ///
    /// Link control flags
    fn lnk_ctl(&self) -> Option<u16> {
        None
    }

    /// Link Control
    ///
    /// Link control flags
    fn set_lnk_ctl(&mut self, value: u16) {}
}

pub enum Cfg {
    Static = 0,
    Dhcp = 1,
}
