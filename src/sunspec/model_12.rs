use heapless::String;

/// Include to support an IPv4 protocol stack on this interface
pub struct Model12 {
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
    /// Config Status
    ///
    /// Configuration status
    cfg_st: CfgSt,
    /// Change Status
    ///
    /// A configuration change is pending
    chg_st: u16,
    /// Config Capability
    ///
    /// Identify capable sources of configuration
    cap: u16,
    /// IPv4 Config
    ///
    /// Configuration method used.
    cfg: Cfg,
    /// Control
    ///
    /// Configure use of services
    ctl: Ctl,
    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    addr: String<16>,
    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    msk: String<16>,
    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    gw: Option<String<16>>,
    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    dns1: Option<String<16>>,
    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    dns2: Option<String<16>>,
    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    ntp1: Option<String<24>>,
    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    ntp2: Option<String<24>>,
    /// Domain
    ///
    /// Domain name (24 chars max)
    dom_nam: Option<String<24>>,
    /// Host Name
    ///
    /// Host name (24 chars max)
    host_nam: Option<String<24>>,
}

trait Model12Trait {
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

    /// Config Status
    ///
    /// Configuration status
    fn cfg_st(&self) -> CfgSt;

    /// Change Status
    ///
    /// A configuration change is pending
    fn chg_st(&self) -> u16;

    /// Config Capability
    ///
    /// Identify capable sources of configuration
    fn cap(&self) -> u16;

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn cfg(&self) -> Cfg;

    /// IPv4 Config
    ///
    /// Configuration method used.
    fn set_cfg(&mut self, value: Cfg);

    /// Control
    ///
    /// Configure use of services
    fn ctl(&self) -> Ctl;

    /// Control
    ///
    /// Configure use of services
    fn set_ctl(&mut self, value: Ctl);

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn addr(&self) -> String<16>;

    /// IP
    ///
    /// IPv4 numeric address as a dotted string xxx.xxx.xxx.xxx
    fn set_addr(&mut self, value: String<16>);

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn msk(&self) -> String<16>;

    /// Netmask
    ///
    /// IPv4 numeric netmask as a dotted string xxx.xxx.xxx.xxx
    fn set_msk(&mut self, value: String<16>);

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn gw(&self) -> Option<String<16>> {
        None
    }

    /// Gateway
    ///
    /// IPv4 numeric gateway address as a dotted string xxx.xxx.xxx.xxx
    fn set_gw(&mut self, value: String<16>) {}

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns1(&self) -> Option<String<16>> {
        None
    }

    /// DNS1
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns1(&mut self, value: String<16>) {}

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn dns2(&self) -> Option<String<16>> {
        None
    }

    /// DNS2
    ///
    /// IPv4 numeric DNS address as a dotted string xxx.xxx.xxx.xxx
    fn set_dns2(&mut self, value: String<16>) {}

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp1(&self) -> Option<String<24>> {
        None
    }

    /// NTP1
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp1(&mut self, value: String<24>) {}

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn ntp2(&self) -> Option<String<24>> {
        None
    }

    /// NTP2
    ///
    /// IPv4 numeric NTP address as a dotted string xxx.xxx.xxx.xxx
    fn set_ntp2(&mut self, value: String<24>) {}

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn dom_nam(&self) -> Option<String<24>> {
        None
    }

    /// Domain
    ///
    /// Domain name (24 chars max)
    fn set_dom_nam(&mut self, value: String<24>) {}

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn host_nam(&self) -> Option<String<24>> {
        None
    }

    /// Host Name
    ///
    /// Host name (24 chars max)
    fn set_host_nam(&mut self, value: String<24>) {}
}

pub enum CfgSt {
    NotConfigured = 0,
    ValidSetting = 1,
    ValidHw = 2,
}

pub enum Cfg {
    Static = 0,
    Dhcp = 1,
    Bootp = 2,
    Zeroconf = 3,
}

pub enum Ctl {
    EnableDns = 0,
    EnableNtp = 1,
}
