use heapless::String;

pub type Model64415 = CsipControl;

/// CSIP Client Control for Alarms and Error tests
pub struct CsipControl {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length.
    l: u16,
    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    log_event_ena: Option<LogEventEna>,
    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    http_msg: Option<HttpMsg>,
    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    comm004_cert: Option<Comm004Cert>,
    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    subscribed_resource: Option<String<128>>,
    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    subscription_ena: Option<SubscriptionEna>,
}

trait CsipControlTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length.
    fn l(&self) -> u16;

    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn log_event_ena(&self) -> Option<LogEventEna> {
        None
    }

    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn set_log_event_ena(&mut self, value: LogEventEna) {}

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn http_msg(&self) -> Option<HttpMsg> {
        None
    }

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn set_http_msg(&mut self, value: HttpMsg) {}

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn comm004_cert(&self) -> Option<Comm004Cert> {
        None
    }

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn set_comm004_cert(&mut self, value: Comm004Cert) {}

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn subscribed_resource(&self) -> Option<String<128>> {
        None
    }

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn set_subscribed_resource(&mut self, value: String<128>) {}

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn subscription_ena(&self) -> Option<SubscriptionEna> {
        None
    }

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn set_subscription_ena(&mut self, value: SubscriptionEna) {}
}

pub enum LogEventEna {
    /// Disabled
    ///
    /// LogEvent Mode Disabled
    Disabled = 0,
    /// Enabled
    ///
    /// LogEvent Mode Enabled
    Enabled = 1,
}

pub enum HttpMsg {
    /// Disabled
    ///
    /// HTTP Message Mode Disabled
    Disabled = 0,
    /// Enabled
    ///
    /// HTTP Message Mode Enabled
    Enabled = 1,
}

pub enum Comm004Cert {
    /// DEFAULT
    ///
    /// Default Certificate
    DefaultCertificate = 0,
    /// COMM-004A
    ///
    /// Chain Length Two Certificate
    Comm004a = 1,
    /// COMM-004B
    ///
    /// Chain Length Three Certificate
    Comm004b = 2,
    /// COMM-004C
    ///
    /// Chain Length Four Certificate
    Comm004c = 3,
    /// COMM-004D
    ///
    /// Invalid MICA Extended Key Critical Value
    Comm004d = 4,
    /// COMM-004E
    ///
    /// Invalid MICA Name Non-Critical Value
    Comm004e = 5,
    /// COMM-004F
    ///
    /// Invalid MICA Policy Mapping Non-Critical Value
    Comm004f = 6,
    /// COMM-004G
    ///
    /// Self-signed device certificate
    Comm004g = 7,
}

pub enum SubscriptionEna {
    /// Disabled
    ///
    /// Subscription Mode Disabled
    Disabled = 0,
    /// Enabled
    ///
    /// Subscription Mode Enabled
    Enabled = 1,
}
