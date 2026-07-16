pub type Model806 = FlowBattery;

pub struct FlowBattery {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Battery Points To Be Determined
    bat_tbd: u16,
}

trait FlowBatteryTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Battery Points To Be Determined
    fn bat_tbd(&self) -> u16;
}
