pub type Model808 = FlowBatteryModule;

pub struct FlowBatteryModule {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Module Points To Be Determined
    module_tbd: u16,
}

trait FlowBatteryModuleTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Module Points To Be Determined
    fn module_tbd(&self) -> u16;
}
