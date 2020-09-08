#[cfg(feature = "riddle-audio")]
use crate::audio::AudioSystem;
use crate::{input::InputSystem, platform::PlatformSystem, time::TimeSystem, *};

use std::rc::Rc;

#[derive(Clone)]
pub struct RiddleState {
    pub window: Rc<PlatformSystem>,
    pub input: Rc<InputSystem>,
    pub time: Rc<TimeSystem>,

    #[cfg(feature = "riddle-audio")]
    pub audio: Rc<AudioSystem>,
}

impl RiddleState {
    pub fn new() -> Result<Self, RiddleError> {
        let window = PlatformSystem::new();
        let input = InputSystem::new(window.event_pub())?;
        let time = TimeSystem::new();

        #[cfg(feature = "riddle-audio")]
        let audio = AudioSystem::new()?;

        Ok(RiddleState {
            window: window.into(),
            input: input.into(),
            time: time.into(),

            #[cfg(feature = "riddle-audio")]
            audio: audio.into(),
        })
    }

    pub fn window(&self) -> Rc<PlatformSystem> {
        self.window.clone()
    }

    pub fn input(&self) -> Rc<InputSystem> {
        self.input.clone()
    }

    pub fn time(&self) -> Rc<TimeSystem> {
        self.time.clone()
    }

    pub fn audio(&self) -> Rc<AudioSystem> {
        self.audio.clone()
    }
}
