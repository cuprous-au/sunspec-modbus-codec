use crate::buffer::{self, ModbusBuffer};
use crate::sunspec::points::PointReference;
use crate::sunspec::{PointType, ReadablePoint};
use core::ffi::{c_char, c_void, CStr};

pub const SIZE: u16 = 70;

pub static POINTS: [ReadablePoint; 7] = [
    ReadablePoint {
        reference: PointReference::Static { value: 64415 },
        size: 1,
        data_type: PointType::Uint16,
        writeable: false,
    },
    ReadablePoint {
        reference: PointReference::Model64415 {
            point: Point::ModelLength,
        },
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
    ModelLength,
    LogEventModeEnable,
    HttpMessageModeEnable,
    Comm004Certificate,
    SubscribedResourceUrl,
    SubscribtionEnable,
}

pub fn model_length<'a>(model: &dyn ModelAdapter) -> u16 {
    70
}

pub fn write_point<'a, 'b>(
    model: &dyn ModelAdapter,
    point: &Point,
    buffer: ModbusBuffer<'b>,
    offset: u16,
    limit: u16,
) {
    match point {
        Point::ModelLength => {
            buffer::write_u16(model_length(model) - 2, buffer);
        }
        Point::LogEventModeEnable => {
            if let Some(value) = model.log_event_mode_enable() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::HttpMessageModeEnable => {
            if let Some(value) = model.http_message_mode_enable() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::Comm004Certificate => {
            if let Some(value) = model.comm_004_certificate() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SubscribedResourceUrl => {
            if let Some(value) = model.subscribed_resource_url() {
                buffer::write_string(value, buffer, offset, limit);
            } else {
                buffer::zero(buffer, offset);
            }
        }
        Point::SubscribtionEnable => {
            if let Some(value) = model.subscribtion_enable() {
                buffer::write_u16(value as u16, buffer);
            } else {
                buffer::zero(buffer, offset);
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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

#[derive(Clone, Copy)]
#[repr(u16)]
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
    context: *mut c_void,
    log_event_mode_enable_callback: Option<extern "C" fn(*const c_void) -> LogEventEna>,
    set_log_event_mode_enable_callback: Option<extern "C" fn(LogEventEna, *mut c_void)>,
    http_message_mode_enable_callback: Option<extern "C" fn(*const c_void) -> HttpMsg>,
    set_http_message_mode_enable_callback: Option<extern "C" fn(HttpMsg, *mut c_void)>,
    comm_004_certificate_callback: Option<extern "C" fn(*const c_void) -> Comm004Cert>,
    set_comm_004_certificate_callback: Option<extern "C" fn(Comm004Cert, *mut c_void)>,
    subscribed_resource_url_callback: Option<extern "C" fn(*const c_void) -> *const c_char>,
    set_subscribed_resource_url_callback: Option<extern "C" fn(*const c_char, *mut c_void)>,
    subscribtion_enable_callback: Option<extern "C" fn(*const c_void) -> SubscriptionEna>,
    set_subscribtion_enable_callback: Option<extern "C" fn(SubscriptionEna, *mut c_void)>,
}

impl ModelAdapter for Model64415CallbackAdapter {
    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn log_event_mode_enable(&self) -> Option<LogEventEna> {
        self.log_event_mode_enable_callback
            .map(|callback| (callback)(self.context))
    }

    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn set_log_event_mode_enable(&mut self, value: LogEventEna) {
        if let Some(callback) = self.set_log_event_mode_enable_callback {
            (callback)(value, self.context);
        };
    }

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn http_message_mode_enable(&self) -> Option<HttpMsg> {
        self.http_message_mode_enable_callback
            .map(|callback| (callback)(self.context))
    }

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn set_http_message_mode_enable(&mut self, value: HttpMsg) {
        if let Some(callback) = self.set_http_message_mode_enable_callback {
            (callback)(value, self.context);
        };
    }

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn comm_004_certificate(&self) -> Option<Comm004Cert> {
        self.comm_004_certificate_callback
            .map(|callback| (callback)(self.context))
    }

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn set_comm_004_certificate(&mut self, value: Comm004Cert) {
        if let Some(callback) = self.set_comm_004_certificate_callback {
            (callback)(value, self.context);
        };
    }

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn subscribed_resource_url(&self) -> Option<&CStr> {
        self.subscribed_resource_url_callback
            .map(|callback| unsafe { CStr::from_ptr((callback)(self.context)) })
    }

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn set_subscribed_resource_url(&mut self, value: &CStr) {
        if let Some(callback) = self.set_subscribed_resource_url_callback {
            (callback)(value.as_ptr(), self.context);
        };
    }

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn subscribtion_enable(&self) -> Option<SubscriptionEna> {
        self.subscribtion_enable_callback
            .map(|callback| (callback)(self.context))
    }

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn set_subscribtion_enable(&mut self, value: SubscriptionEna) {
        if let Some(callback) = self.set_subscribtion_enable_callback {
            (callback)(value, self.context);
        };
    }
}

#[repr(C)]
pub struct Model64415StatefulAdapter {
    log_event_mode_enable: LogEventEna,
    http_message_mode_enable: HttpMsg,
    comm_004_certificate: Comm004Cert,
    subscribed_resource_url: [c_char; 128],
    subscribtion_enable: SubscriptionEna,
}

impl ModelAdapter for Model64415StatefulAdapter {
    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn log_event_mode_enable(&self) -> Option<LogEventEna> {
        Some(self.log_event_mode_enable)
    }

    /// LogEvent Mode Enable
    ///
    /// Enable or disable the LogEvent mode
    fn set_log_event_mode_enable(&mut self, value: LogEventEna) {
        self.log_event_mode_enable = value;
    }

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn http_message_mode_enable(&self) -> Option<HttpMsg> {
        Some(self.http_message_mode_enable)
    }

    /// HTTP Message Mode Enable
    ///
    /// Enable or disable the HTTP Message mode
    fn set_http_message_mode_enable(&mut self, value: HttpMsg) {
        self.http_message_mode_enable = value;
    }

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn comm_004_certificate(&self) -> Option<Comm004Cert> {
        Some(self.comm_004_certificate)
    }

    /// COMM-004 Certificate
    ///
    /// Select COMM-004 certificate type
    fn set_comm_004_certificate(&mut self, value: Comm004Cert) {
        self.comm_004_certificate = value;
    }

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn subscribed_resource_url(&self) -> Option<&CStr> {
        Some(unsafe { CStr::from_ptr(self.subscribed_resource_url.as_ptr()) })
    }

    /// Subscribed Resource URL
    ///
    /// The URL of the resource to subscribe to
    fn set_subscribed_resource_url(&mut self, value: &CStr) {
        for (dest, src) in self
            .subscribed_resource_url
            .iter_mut()
            .zip(value.to_bytes_with_nul().iter())
        {
            *dest = *src as c_char;
        }
    }

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn subscribtion_enable(&self) -> Option<SubscriptionEna> {
        Some(self.subscribtion_enable)
    }

    /// Subscribtion Enable
    ///
    /// Enable or disable the Subscription mode
    fn set_subscribtion_enable(&mut self, value: SubscriptionEna) {
        self.subscribtion_enable = value;
    }
}
