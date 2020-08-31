use crate::*;

use rodio::source::Source;
use std::rc::Rc;

pub struct ClipPlayer {
    audio: Rc<AudioSystem>,
    clip: Clip,
    sink: Option<Rc<rodio::Sink>>,
}

impl ClipPlayer {
    pub(crate) fn new(audio: Rc<AudioSystem>, clip: Clip) -> Self {
        Self {
            audio: audio.clone(),
            clip: clip,
            sink: None,
        }
    }

    fn play(&mut self, mode: PlayMode) -> Result<(), AudioError> {
        let sink: Rc<rodio::Sink> = rodio::Sink::new(&self.audio.device).into();
        let source = rodio::decoder::Decoder::new(std::io::Cursor::new(self.clip.data.clone()))
            .map_err(|_| AudioError::UnknownError)?;

        match mode {
            PlayMode::OneShot => sink.append(source),
            PlayMode::Loop => sink.append(source.repeat_infinite()),
        }

        self.sink = Some(sink);

        Ok(())
    }

    pub fn set_volume(&mut self, volume: f32) {
        match &self.sink {
            Some(sink) => {
                //TODO: Use the fader
                sink.set_volume(volume)
            }
            _ => (),
        }
    }

    pub fn pause(&mut self) {
        match &self.sink {
            Some(sink) => sink.pause(),
            _ => (),
        }
    }

    pub fn resume(&mut self) {
        match &self.sink {
            Some(sink) => {
                if sink.is_paused() {
                    sink.play()
                }
            }
            _ => (),
        }
    }

    pub fn stop(self) -> () {}
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

    pub fn play(&self, audio: Rc<AudioSystem>, clip: Clip) -> Result<ClipPlayer, AudioError> {
        let mut player = ClipPlayer::new(audio, clip);
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
