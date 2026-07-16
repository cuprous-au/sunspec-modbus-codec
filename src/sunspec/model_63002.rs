pub struct Model63002 {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
}

trait Model63002Trait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;
}
