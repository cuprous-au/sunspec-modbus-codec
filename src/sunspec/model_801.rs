pub type Model801 = Storage;

/// This model has been deprecated.
pub struct Storage {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// Deprecated Model
    ///
    /// This model has been deprecated.
    deprecated: Deprecated,
}

trait StorageTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// Deprecated Model
    ///
    /// This model has been deprecated.
    fn deprecated(&self) -> Deprecated;
}

pub enum Deprecated {}
