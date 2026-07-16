/// Interface counters
pub struct Model15 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Clear
    ///
    /// Write a "1" to clear all counters
    clr: Option<u16>,
    /// Input Count
    ///
    /// Number of bytes received
    in_cnt: Option<u32>,
    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    in_uc_cnt: Option<u32>,
    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    in_n_uc_cnt: Option<u32>,
    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    in_dsc_cnt: Option<u32>,
    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    in_err_cnt: Option<u32>,
    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    in_unk_cnt: Option<u32>,
    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    out_cnt: Option<u32>,
    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    out_uc_cnt: Option<u32>,
    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    out_n_uc_cnt: Option<u32>,
    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    out_dsc_cnt: Option<u32>,
    /// Output Error Count
    ///
    /// Number of outbound error packets
    out_err_cnt: Option<u32>,
}

trait Model15Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn clr(&self) -> Option<u16> {
        None
    }

    /// Clear
    ///
    /// Write a "1" to clear all counters
    fn set_clr(&mut self, value: u16) {}

    /// Input Count
    ///
    /// Number of bytes received
    fn in_cnt(&self) -> Option<u32> {
        None
    }

    /// Input Unicast Count
    ///
    /// Number of Unicast packets received
    fn in_uc_cnt(&self) -> Option<u32> {
        None
    }

    /// Input Non-Unicast Count
    ///
    /// Number of non-Unicast packets received
    fn in_n_uc_cnt(&self) -> Option<u32> {
        None
    }

    /// Input Discarded Count
    ///
    /// Number of inbound packets received on the interface but discarded
    fn in_dsc_cnt(&self) -> Option<u32> {
        None
    }

    /// Input Error Count
    ///
    /// Number of inbound packets that contain errors (excluding discards)
    fn in_err_cnt(&self) -> Option<u32> {
        None
    }

    /// Input Unknown Count
    ///
    /// Number of inbound packets with unknown protocol
    fn in_unk_cnt(&self) -> Option<u32> {
        None
    }

    /// Output Count
    ///
    /// Total number of bytes transmitted on this interface
    fn out_cnt(&self) -> Option<u32> {
        None
    }

    /// Output Unicast Count
    ///
    /// Number of Unicast packets transmitted
    fn out_uc_cnt(&self) -> Option<u32> {
        None
    }

    /// Output Non-Unicast Count
    ///
    /// Number of Non-Unicast packets transmitted
    fn out_n_uc_cnt(&self) -> Option<u32> {
        None
    }

    /// Output Discarded Count
    ///
    /// Number of Discarded output packets
    fn out_dsc_cnt(&self) -> Option<u32> {
        None
    }

    /// Output Error Count
    ///
    /// Number of outbound error packets
    fn out_err_cnt(&self) -> Option<u32> {
        None
    }
}
