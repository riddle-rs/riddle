use crate::*;

pub trait AudioSystemExt {
    /// Create the audio system, connected to the default audio output device.
    fn new_shared() -> Result<(AudioSystemHandle, AudioMainThreadState)>;

    /// Update the system's state.
    ///
    /// Updates all [`ClipPlayer`] fades. This must be called periodically for the [`AudioSystem`]
    /// to function. **Do not** call this if the `riddle` crate is being used.
    ///
    /// # Example
    /// ```no_run
    /// # use riddle_audio::{ext::*, *}; fn main() -> Result<(), AudioError> {
    /// let (audio_system, _audio_main_thread_state) = AudioSystem::new_shared()?;
    ///
    /// // Tick the audio system every 100ms
    /// let start_time = std::time::Instant::now();
    /// while std::time::Instant::now() - start_time < std::time::Duration::from_secs(2) {
    ///     audio_system.process_frame();
    ///     std::thread::sleep(std::time::Duration::from_millis(100));
    /// }
    /// # Ok(()) }
    /// ```
    fn process_frame(&self);
}
