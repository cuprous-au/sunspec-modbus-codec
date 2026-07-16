pub type Model809 = FlowBatteryStack;

pub struct FlowBatteryStack {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Stack Points To Be Determined
    stack_tbd: u16,
}

trait FlowBatteryStackTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Stack Points To Be Determined
    fn stack_tbd(&self) -> u16;
}
