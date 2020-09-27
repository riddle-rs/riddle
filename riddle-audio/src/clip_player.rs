use crate::*;

use rodio::{decoder::Decoder, source::Source, Sink};
use std::{io::Cursor, sync::Arc, time::Duration};

const QUICK_FADE_DURATION_SECONDS: f32 = 0.2;

/// Handles playback of a [`Clip`] with support for pausing, resuming, volume adjustment.
///
/// Instances can be built using a [`ClipPlayerBuilder`].
///
/// # Example
///
/// ```no_run
/// # use riddle_audio::*; doctest::simple(|audio_system| {
/// let bytes = include_bytes!("../../example_assets/boop.wav");
/// let clip = Clip::new(&bytes[..])?;
///
/// // Play the clip
/// let mut player = ClipPlayerBuilder::new(&audio_system).play(&clip)?;
/// player.set_volume(0.5);
/// # Ok(player) });
/// ```
pub struct ClipPlayer {
    audio: AudioSystemHandle,
    clip: Clip,
    sink: Option<Arc<Sink>>,

    volume: f32,
}

impl ClipPlayer {
    pub(crate) fn new(audio: &AudioSystem, clip: &Clip, volume: f32) -> Self {
        Self {
            audio: audio.clone_handle(),
            clip: clip.clone(),
            sink: None,
            volume,
        }
    }

    fn play(&mut self, mode: PlayMode, paused: bool) -> Result<()> {
        let sink: Arc<Sink> = Sink::new(&self.audio.device).into();

        if paused {
            sink.pause();
            sink.set_volume(0.0);
        } else {
            sink.set_volume(self.volume);
        }

        let source = Decoder::new(Cursor::new(self.clip.data.clone()))
            .map_err(|_| AudioError::ClipDecodeError)?;

        match mode {
            PlayMode::OneShot => sink.append(source),
            PlayMode::Loop => sink.append(source.repeat_infinite()),
        }

        self.sink = Some(sink);

        Ok(())
    }

    /// Fade the volume from the current volume to the targat volume over time.
    ///
    /// Once the volume has been changed the nominal volume will be immediately set to the new
    /// goal volume, as that is the volume that the player will be set to if it gets paused
    /// and resumed.
    ///
    /// The observed volume will change over time as the `AudioSubsystem` progresses the fade.
    ///
    /// If another volume fade is started while one is in progress the existing one is replaced
    /// by the new one, starting from whatever the current volume is.
    ///
    /// Since [`ClipPlayer::set_volume`], [`ClipPlayer::stop`], [`ClipPlayer::pause`]
    /// and [`ClipPlayer::resume`] calls also trigger a fade to avoid popping,
    /// calling any of those methods will also replace any current fade.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle_audio::*; doctest::simple(|audio_system| {
    /// # let bytes = include_bytes!("../../example_assets/boop.wav");
    /// # let clip = Clip::new(&bytes[..])?;
    /// // The player starts with all volumes at 1.0
    /// let mut player = ClipPlayerBuilder::new(&audio_system).play(&clip)?;
    /// assert_eq!(1.0, player.get_nominal_volume());
    /// assert_eq!(1.0, player.get_observed_volume());
    ///
    /// // The nominal volume changes immediately, the observed volume hasn't changed
    /// player.fade_volume(0.0, std::time::Duration::from_secs(1));
    /// assert_eq!(0.0, player.get_nominal_volume());
    /// assert_eq!(1.0, player.get_observed_volume());
    ///
    /// // A few seconds later
    /// # doctest::pump_for_secs(audio_system, 2);
    /// // The fade has completed and the nominal and observed volumes agree again
    /// assert_eq!(0.0, player.get_nominal_volume());
    /// assert_eq!(0.0, player.get_observed_volume());
    /// # Ok(player) });
    /// ```
    pub fn fade_volume(&mut self, volume: f32, duration: Duration) {
        self.volume = volume;
        self.fade_volume_with_type(self.volume, duration, FadeType::AlterVolume);
    }

    /// Set the volume of playback immediately.
    ///
    /// This performs a very quick fade to the destination volume, to avoid popping
    /// audio artefacts.
    ///
    /// See the example in [`ClipPlayer::fade_volume`] for more details of how volume
    /// changes over time.
    pub fn set_volume(&mut self, volume: f32) {
        self.fade_volume(volume, Duration::from_secs_f32(QUICK_FADE_DURATION_SECONDS))
    }

    /// Get the nominal volume of the player, which may not match the volume the player is currently
    /// playing at this second.
    ///
    /// This is the volume last set via [`ClipPlayer::set_volume`] or [`ClipPlayer::fade_volume`].
    /// This is the volume the player will be at if it is paused and resumed.
    ///
    /// See the example in [`ClipPlayer::fade_volume`] for more details of how volume
    /// changes over time.
    pub fn get_nominal_volume(&self) -> f32 {
        self.volume
    }

    /// Get the observed volume of the player, which is always equal to exactly the volume of playback.
    ///
    /// This is the volume of playback at this moment in time, which could be either equal to the
    /// nominal volume, or another volume if there is a fade running or if the player has been
    /// paused (which causes an observed fade to 0 volume).
    ///
    /// See the example in [`ClipPlayer::fade_volume`] for more details of how volume
    /// changes over time.
    pub fn get_observed_volume(&self) -> f32 {
        self.sink
            .as_ref()
            .map(|sink| sink.volume())
            .unwrap_or(self.volume)
    }

    /// Pauses playback of the clip.
    ///
    /// This performs a very quick fade to zero volume, to avoid popping
    /// audio artefacts, and then pauses playback.
    ///
    /// The nominal volume of the player won't change, but the observed volume will drop to zero.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle_audio::*; doctest::simple(|audio_system| {
    /// # let bytes = include_bytes!("../../example_assets/boop.wav");
    /// # let clip = Clip::new(&bytes[..])?;
    /// // The player starts with all volumes at 1.0
    /// let mut player = ClipPlayerBuilder::new(&audio_system).play(&clip)?;
    /// assert_eq!(1.0, player.get_nominal_volume());
    /// assert_eq!(1.0, player.get_observed_volume());
    ///
    /// // Pausing doesn't change the nominal volume
    /// player.pause();
    /// assert_eq!(1.0, player.get_nominal_volume());
    /// assert_eq!(1.0, player.get_observed_volume());
    ///
    /// // A short moment later
    /// # doctest::pump_for_secs(audio_system, 1);
    /// // The pause has completed and the observed volume is now 0.0
    /// assert_eq!(1.0, player.get_nominal_volume());
    /// assert_eq!(0.0, player.get_observed_volume());
    /// # Ok(player) });
    /// ```
    pub fn pause(&mut self) {
        self.fade_volume_with_type(
            0.0,
            Duration::from_secs_f32(QUICK_FADE_DURATION_SECONDS),
            FadeType::Pause,
        );
    }

    // Resumes playback if paused.
    //
    // Immediately starts playback and performs a quick fade back up to the players
    // nominal volume.
    //
    // The nominal volume of the player won't change, but the observed volume with fade
    // from 0 to the nominal value over time.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use riddle_audio::*; doctest::simple(|audio_system| {
    /// # let bytes = include_bytes!("../../example_assets/boop.wav");
    /// # let clip = Clip::new(&bytes[..])?;
    /// // The paused player starts with an observed volume of 0.0
    /// let mut player = ClipPlayerBuilder::new(&audio_system).paused(&clip)?;
    /// assert_eq!(1.0, player.get_nominal_volume());
    /// assert_eq!(0.0, player.get_observed_volume());
    ///
    /// // Resuming doesn't change the nominal volume
    /// player.resume();
    /// assert_eq!(1.0, player.get_nominal_volume());
    /// assert_eq!(0.0, player.get_observed_volume());
    ///
    /// // A short moment later
    /// # doctest::pump_for_secs(audio_system, 1);
    /// // The resume has completed and the observed volume is now 1.0
    /// assert_eq!(1.0, player.get_nominal_volume());
    /// assert_eq!(1.0, player.get_observed_volume());
    /// # Ok(player) });
    /// ```
    pub fn resume(&mut self) {
        match &self.sink {
            Some(sink) => {
                if sink.is_paused() {
                    sink.play();
                    self.fade_volume_with_type(
                        self.volume,
                        Duration::from_secs_f32(QUICK_FADE_DURATION_SECONDS),
                        FadeType::Resume,
                    );
                }
            }
            _ => (),
        }
    }

    /// Stops playback.
    ///
    /// This is equivalent to calling [`ClipPlayer::pause`] and then dropping the player
    /// after the fade is complete
    pub fn stop(mut self) -> () {
        self.pause();
    }

    fn fade_volume_with_type(&mut self, volume: f32, duration: Duration, fade_type: FadeType) {
        match &self.sink {
            Some(sink) => {
                let fade = Fade::new(sink.clone(), volume, duration, fade_type);
                self.audio.register_fade(fade);
            }
            _ => (),
        }
    }
}

/// Enum describing what the player should do at the end of the clip
#[derive(Copy, Clone)]
pub enum PlayMode {
    /// Stop playing at the end of the clip
    OneShot,

    /// Return to the beginning of the clip and play it again
    Loop,
}

/// Builder for [`ClipPlayer`]
///
/// A builder instance may be used to construct multiple players.
///
/// # Example
///
/// ```no_run
/// # use riddle_audio::*; doctest::simple(|audio_system| {
/// let bytes = include_bytes!("../../example_assets/boop.wav");
/// let clip = Clip::new(&bytes[..])?;
///
/// // Play the clip
/// let player = ClipPlayerBuilder::new(&audio_system)
///     .with_volume(0.5)
///     .with_mode(PlayMode::Loop)
///     .play(&clip)?;
/// # Ok(player) });
/// ```
pub struct ClipPlayerBuilder {
    mode: PlayMode,
    audio: AudioSystemHandle,
    volume: f32,
}

impl ClipPlayerBuilder {
    /// Make a new builder.
    ///
    /// Defaults:
    ///
    /// * mode: [`PlayMode::OneShot`]
    /// * volume: 1.0.
    pub fn new(audio: &AudioSystem) -> Self {
        Self {
            mode: PlayMode::OneShot,
            audio: audio.clone_handle(),
            volume: 1.0,
        }
    }

    /// Set the playback mode of the player. Defaults to [`PlayMode::OneShot`].
    pub fn with_mode(&mut self, mode: PlayMode) -> &mut Self {
        self.mode = mode;
        self
    }

    /// Set the playback volume of the player. Defaults to 1.0.
    pub fn with_volume(&mut self, volume: f32) -> &mut Self {
        self.volume = volume;
        self
    }

    /// Build the ClipPlayer, and start playing the clip immediately.
    pub fn play(&self, clip: &Clip) -> Result<ClipPlayer> {
        let mut player = ClipPlayer::new(&self.audio, clip, self.volume);
        player.play(self.mode, false)?;
        Ok(player)
    }

    /// Build the ClipPlayer in the paused state. [`ClipPlayer::resume`] will need
    /// to be called on the player to start playback.
    pub fn paused(&self, clip: &Clip) -> Result<ClipPlayer> {
        let mut player = ClipPlayer::new(&self.audio, clip, self.volume);
        player.play(self.mode, true)?;
        Ok(player)
    }
}
