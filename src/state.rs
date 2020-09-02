#[cfg(feature = "riddle-audio")]
use crate::audio::AudioSystem;
use crate::{
    input::InputSystem,
    time::TimeSystem,
    window::{Window, WindowSystem},
    *,
};

use std::rc::Rc;

pub(crate) struct RiddleState {
    pub window: Rc<WindowSystem>,
    pub input: Rc<InputSystem<Rc<Window>>>,
    pub time: Rc<TimeSystem>,

    #[cfg(feature = "riddle-audio")]
    pub audio: Rc<AudioSystem>,
}

impl RiddleState {
    pub fn new() -> Result<Self, RiddleError> {
        let window = WindowSystem::new();
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
}
