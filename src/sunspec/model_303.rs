pub type Model303 = BomTemp;

/// Include to support variable number of  back of module temperature measurements
pub struct BomTemp {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
}

trait BomTempTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;
}
