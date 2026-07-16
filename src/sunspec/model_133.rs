pub type Model133 = Schedule;

/// Basic Scheduling
pub struct Schedule {
    /// Model ID
    ///
    /// Model identifier
    id: u16,
    /// Model Length
    ///
    /// Model length
    l: u16,
    /// ActSchd
    ///
    /// Bitfield of active schedules
    act_schd: u32,
    /// ModEna
    ///
    /// Is basic scheduling active.
    mod_ena: u16,
    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    n_schd: u16,
    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    n_pts: u16,
}

trait ScheduleTrait {
    /// Model ID
    ///
    /// Model identifier
    fn id(&self) -> u16;

    /// Model Length
    ///
    /// Model length
    fn l(&self) -> u16;

    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn act_schd(&self) -> u32;

    /// ActSchd
    ///
    /// Bitfield of active schedules
    fn set_act_schd(&mut self, value: u32);

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn mod_ena(&self) -> u16;

    /// ModEna
    ///
    /// Is basic scheduling active.
    fn set_mod_ena(&mut self, value: u16);

    /// NSchd
    ///
    /// Number of schedules supported (recommend min. 4, max 32)
    fn n_schd(&self) -> u16;

    /// NPts
    ///
    /// Number of schedule entries supported (maximum of 10).
    fn n_pts(&self) -> u16;
}
