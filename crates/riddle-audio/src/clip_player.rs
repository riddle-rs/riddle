use crate::*;

use rodio::{decoder::Decoder, source::Source, Sink};
use std::{io::Cursor, sync::Arc, time::Duration};

const QUICK_FADE_DURATION_SECONDS: f32 = 0.2;

pub struct ClipPlayer {
    audio: AudioSystemHandle,
    clip: Clip,
    sink: Option<Arc<Sink>>,

    volume: f32,
}

impl ClipPlayer {
    pub(crate) fn new(audio: &AudioSystem, clip: Clip) -> Result<Self, AudioError> {
        Ok(Self {
            audio: audio.clone_handle().ok_or(AudioError::UnknownError)?,
            clip: clip,
            sink: None,
            volume: 1.0,
        })
    }

    fn play(&mut self, mode: PlayMode) -> Result<(), AudioError> {
        let sink: Arc<Sink> = Sink::new(&self.audio.device).into();
        sink.set_volume(self.volume);
        let source = Decoder::new(Cursor::new(self.clip.data.clone()))
            .map_err(|_| AudioError::UnknownError)?;

        match mode {
            PlayMode::OneShot => sink.append(source),
            PlayMode::Loop => sink.append(source.repeat_infinite()),
        }

        self.sink = Some(sink);

        Ok(())
    }

    pub fn fade_volume(&mut self, volume: f32, duration: Duration) {
        self.volume = volume;
        self.fade_volume_with_type(self.volume, duration, FadeType::AlterVolume);
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.fade_volume(volume, Duration::from_secs_f32(QUICK_FADE_DURATION_SECONDS))
    }

    pub fn pause(&mut self) {
        self.fade_volume_with_type(
            0.0,
            Duration::from_secs_f32(QUICK_FADE_DURATION_SECONDS),
            FadeType::Pause,
        );
    }

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

#[derive(Copy, Clone)]
pub enum PlayMode {
    OneShot,
    Loop,
}

pub struct ClipPlayerBuilder {
    mode: PlayMode,
}

impl ClipPlayerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_mode(&mut self, mode: PlayMode) -> &mut Self {
        self.mode = mode;
        self
    }

    pub fn play(&self, audio: &AudioSystem, clip: Clip) -> Result<ClipPlayer, AudioError> {
        let mut player = ClipPlayer::new(audio, clip)?;
        player.play(self.mode)?;
        Ok(player)
    }
}

impl Default for ClipPlayerBuilder {
    fn default() -> Self {
        Self {
            mode: PlayMode::OneShot,
        }
    }
}
