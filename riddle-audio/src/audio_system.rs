use crate::*;

use std::{collections::HashMap, sync::Mutex, time::Instant};

/// The Riddle audio system core state.
///
/// Manages underlying audio device and riddle audio objects' states. The recommended
/// way to use this type is to let the `riddle` crate initialize and manage it for you.
///
/// It is possible to manage the audio system state independantly - the most important
/// thing to note is that [`ext::AudioSystemExt::process_frame`] must be called periodically
/// for [`ClipPlayer`] to work properly. This is **not** something that needs doing if
/// using the `riddle` crate to manage the [`AudioSystem`] automatically.
pub struct AudioSystem {
    weak_self: AudioSystemWeak,
    pub(super) stream_handle: rodio::OutputStreamHandle,
    fades: Mutex<HashMap<FadeKey, Fade>>,
}

define_handles!(<AudioSystem>::weak_self, pub AudioSystemHandle, pub AudioSystemWeak);

impl AudioSystem {
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

impl ext::AudioSystemExt for AudioSystem {
    fn new_shared() -> Result<(AudioSystemHandle, AudioMainThreadState)> {
        let (stream, stream_handle) =
            rodio::OutputStream::try_default().map_err(|_| AudioError::InitFailed {
                cause: "Failed to get rodio output device",
            })?;
        let main_thread_state = AudioMainThreadState { _stream: stream };
        Ok((
            AudioSystemHandle::new(|weak_self| AudioSystem {
                weak_self,
                stream_handle,
                fades: Mutex::new(HashMap::new()),
            }),
            main_thread_state,
        ))
    }

    fn process_frame(&self) {
        let now = Instant::now();
        self.tick_fades(now);
    }
}

pub struct AudioMainThreadState {
    _stream: rodio::OutputStream,
}
