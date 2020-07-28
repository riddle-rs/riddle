use crate::*;

use rodio::source::Source;
use std::rc::Rc;

pub struct ClipPlayer {
    _audio: Rc<AudioSystem>,
    clip: Clip,
    sink: rodio::Sink,
}

impl ClipPlayer {
    pub(crate) fn new(audio: Rc<AudioSystem>, clip: Clip) -> Self {
        Self {
            _audio: audio.clone(),
            clip: clip,
            sink: rodio::Sink::new(&audio.device),
        }
    }

    fn play(&mut self, mode: PlayMode) -> Result<(), AudioError> {
        let source = rodio::decoder::Decoder::new(std::io::Cursor::new(self.clip.data.clone()))
            .map_err(|_| AudioError::UnknownError)?;

        match mode {
            PlayMode::OneShot => self.sink.append(source),
            PlayMode::Loop => self.sink.append(source.repeat_infinite()),
        }

        Ok(())
    }

    pub fn detach(self) {
        self.sink.detach();
    }

    pub fn pause(&mut self) {
        self.sink.pause();
    }

    pub fn resume(&mut self) {
        if self.sink.is_paused() {
            self.sink.play()
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
