use heapless::String;

/// Include this model to support a cellular interface link
pub struct Model18 {
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
    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    imei: Option<u32>,
    /// APN
    ///
    /// Access Point Name for the interface
    apn: Option<String<8>>,
    /// Number
    ///
    /// Phone number for the interface
    num: Option<String<12>>,
    /// PIN
    ///
    /// Personal Identification Number for the interface
    pin: Option<String<12>>,
}

trait Model18Trait {
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

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn imei(&self) -> Option<u32> {
        None
    }

    /// IMEI
    ///
    /// International Mobile Equipment Identifier for the interface
    fn set_imei(&mut self, value: u32) {}

    /// APN
    ///
    /// Access Point Name for the interface
    fn apn(&self) -> Option<String<8>> {
        None
    }

    /// APN
    ///
    /// Access Point Name for the interface
    fn set_apn(&mut self, value: String<8>) {}

    /// Number
    ///
    /// Phone number for the interface
    fn num(&self) -> Option<String<12>> {
        None
    }

    /// Number
    ///
    /// Phone number for the interface
    fn set_num(&mut self, value: String<12>) {}

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn pin(&self) -> Option<String<12>> {
        None
    }

    /// PIN
    ///
    /// Personal Identification Number for the interface
    fn set_pin(&mut self, value: String<12>) {}
}
