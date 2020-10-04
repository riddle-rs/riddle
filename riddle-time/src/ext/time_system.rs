use crate::*;

pub trait TimeSystemExt {
    /// Create a new time system. The time the system is created is used as the time
    /// of the 0th frame.
    fn new() -> TimeSystemHandle;

    /// Update the time system state, marking the beginning of a the next frame.
    ///
    /// The instant that this method is called is taken as the reference time for
    /// the frame that is about to be executed.
    ///
    /// Timers will also be triggered during this function call if they are due
    /// to trigger.
    ///
    /// **Do not** call this function directly if you are using this through the
    /// `riddle` crate.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_time::*; doctest::simple(|time_system| {
    /// let frame_1 = time_system.frame_instant();
    ///
    /// // A while later
    /// # doctest::pump_for_secs(time_system, 1);
    /// let frame_n = time_system.frame_instant();
    ///
    /// assert_eq!(true, frame_n - frame_1 > std::time::Duration::from_secs(0));
    /// # });
    /// ```
    fn process_frame(&self);
}
