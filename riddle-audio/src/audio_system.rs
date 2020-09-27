use crate::*;

use rodio::Device;
use std::{collections::HashMap, sync::Mutex, time::Instant};

/// The Riddle audio system core state.
///
/// Manages underlying audio device and riddle audio objects' states. The recommended
/// way to use this type is to let the `riddle` crate initialize and manage it for you.
///
/// It is possible to manage the audio system state independantly - the most important
/// thing to note is that [`AudioSystem::process_frame`] must be called periodically
/// for [`ClipPlayer`] to work properly. This is **not** something that needs doing if
/// using the `riddle` crate to manage the [`AudioSystem`] automatically.
pub struct AudioSystem {
    weak_self: AudioSystemWeak,
    pub(super) device: Device,

    fades: Mutex<HashMap<FadeKey, Fade>>,
}

define_handles!(<AudioSystem>::weak_self, pub AudioSystemHandle, pub AudioSystemWeak);

impl AudioSystem {
    /// Create the audio system, connected to the default audio output device.
    pub fn new() -> Result<AudioSystemHandle> {
        let device = rodio::default_output_device().ok_or(AudioError::InitFailed {
            cause: "Failed to get rodio output device",
        })?;
        Ok(AudioSystemHandle::new(|weak_self| AudioSystem {
            weak_self,
            device,
            fades: Mutex::new(HashMap::new()),
        }))
    }

    /// Update the system's state.
    ///
    /// Updates all [`ClipPlayer`] fades. This must be called periodically for the [`AudioSystem`]
    /// to function. **Do not** call this if the `riddle` crate is being used.
    ///
    /// # Example
    /// ```no_run
    /// # use riddle_audio::*; fn main() -> Result<(), AudioError> {
    /// let audio_system = AudioSystem::new()?;
    ///
    /// // Tick the audio system every 100ms
    /// let start_time = std::time::Instant::now();
    /// while std::time::Instant::now() - start_time < std::time::Duration::from_secs(2) {
    ///     audio_system.process_frame();
    ///     std::thread::sleep(std::time::Duration::from_millis(100));
    /// }
    /// # Ok(()) }
    /// ```
    pub fn process_frame(&self) {
        let now = Instant::now();
        self.tick_fades(now);
    }
    pub(crate) fn register_fade(&self, fade: Fade) {
        let mut fades = self.fades.lock().unwrap();
        let existing = fades.remove(&fade.key());
        match existing {
            Some(old) => fades.insert(fade.key(), Fade::merge_pair(old, fade)),
            None => fades.insert(fade.key(), fade),
        };
    }

    fn tick_fades(&self, now: Instant) {
        let mut fades = self.fades.lock().unwrap();
        fades.retain(|_, f| f.update(now));
    }
}
