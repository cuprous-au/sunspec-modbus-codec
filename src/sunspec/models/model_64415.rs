use crate::serialisation;
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, CStr};

pub const SIZE: u16 = 70;

pub static POINTS: [ReadablePoint; 7] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64415 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Static { value: 68 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64415 {
            point: Point::LogEventModeEnable,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64415 {
            point: Point::HttpMessageModeEnable,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64415 {
            point: Point::Comm004Certificate,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64415 {
            point: Point::SubscribedResourceUrl,
        },
        size: 64,
        data_type: PointType::String,
        writeable: true,
    },
    ReadablePoint {
        reference: PointReference::Model64415 {
            point: Point::SubscribtionEnable,
        },
        size: 1,
        data_type: PointType::Enum16,
        writeable: true,
    },
];

#[derive(Debug)]
pub enum Point {
    LogEventModeEnable,
    HttpMessageModeEnable,
    Comm004Certificate,
    SubscribedResourceUrl,
    SubscribtionEnable,
}

pub fn write_point(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: &mut [u16],
    offset: u16,
    limit: u16,
) {
    match point {
        Point::LogEventModeEnable => {
            if let Some(value) = model.log_event_mode_enable() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::HttpMessageModeEnable => {
            if let Some(value) = model.http_message_mode_enable() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::Comm004Certificate => {
            if let Some(value) = model.comm_004_certificate() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
        Point::SubscribedResourceUrl => {
            if let Some(value) = model.subscribed_resource_url() {
                serialisation::write_string(value, buffer, offset, limit);
            }
        }
        Point::SubscribtionEnable => {
            if let Some(value) = model.subscribtion_enable() {
                serialisation::write_u16(value as u16, buffer);
            }
        }
    }
}

pub trait ModelAdapter {
    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn log_event_mode_enable(&self) -> Option<LogEventEna> {
        None
    }

    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn set_log_event_mode_enable(&mut self, value: LogEventEna) {}

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn http_message_mode_enable(&self) -> Option<HttpMsg> {
        None
    }

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn set_http_message_mode_enable(&mut self, value: HttpMsg) {}

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn comm_004_certificate(&self) -> Option<Comm004Cert> {
        None
    }

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn set_comm_004_certificate(&mut self, value: Comm004Cert) {}

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn subscribed_resource_url(&self) -> Option<&CStr> {
        None
    }

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn set_subscribed_resource_url(&mut self, value: &CStr) {}

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn subscribtion_enable(&self) -> Option<SubscriptionEna> {
        None
    }

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn set_subscribtion_enable(&mut self, value: SubscriptionEna) {}
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

#[repr(C)]
pub struct Model64415CallbackAdapter {
    log_event_mode_enable_callback: Option<extern "C" fn() -> LogEventEna>,
    set_log_event_mode_enable_callback: Option<extern "C" fn(LogEventEna)>,
    http_message_mode_enable_callback: Option<extern "C" fn() -> HttpMsg>,
    set_http_message_mode_enable_callback: Option<extern "C" fn(HttpMsg)>,
    comm_004_certificate_callback: Option<extern "C" fn() -> Comm004Cert>,
    set_comm_004_certificate_callback: Option<extern "C" fn(Comm004Cert)>,
    subscribed_resource_url_callback: Option<extern "C" fn() -> *const c_char>,
    set_subscribed_resource_url_callback: Option<extern "C" fn(*const c_char)>,
    subscribtion_enable_callback: Option<extern "C" fn() -> SubscriptionEna>,
    set_subscribtion_enable_callback: Option<extern "C" fn(SubscriptionEna)>,
}

impl ModelAdapter for Model64415CallbackAdapter {
    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn log_event_mode_enable(&self) -> Option<LogEventEna> {
        self.log_event_mode_enable_callback
            .map(|callback| (callback)())
    }

    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn set_log_event_mode_enable(&mut self, value: LogEventEna) {
        if let Some(callback) = self.set_log_event_mode_enable_callback {
            (callback)(value);
        };
    }

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn http_message_mode_enable(&self) -> Option<HttpMsg> {
        self.http_message_mode_enable_callback
            .map(|callback| (callback)())
    }

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn set_http_message_mode_enable(&mut self, value: HttpMsg) {
        if let Some(callback) = self.set_http_message_mode_enable_callback {
            (callback)(value);
        };
    }

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn comm_004_certificate(&self) -> Option<Comm004Cert> {
        self.comm_004_certificate_callback
            .map(|callback| (callback)())
    }

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn set_comm_004_certificate(&mut self, value: Comm004Cert) {
        if let Some(callback) = self.set_comm_004_certificate_callback {
            (callback)(value);
        };
    }

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn subscribed_resource_url(&self) -> Option<&CStr> {
        self.subscribed_resource_url_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)()) })
    }

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn set_subscribed_resource_url(&mut self, value: &CStr) {
        if let Some(callback) = self.set_subscribed_resource_url_callback {
            (callback)(value.as_ptr());
        };
    }

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn subscribtion_enable(&self) -> Option<SubscriptionEna> {
        self.subscribtion_enable_callback
            .map(|callback| (callback)())
    }

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn set_subscribtion_enable(&mut self, value: SubscriptionEna) {
        if let Some(callback) = self.set_subscribtion_enable_callback {
            (callback)(value);
        };
    }
}
