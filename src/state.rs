use crate::{
    audio::AudioSystem,
    input::InputSystem,
    time::TimeSystem,
    window::{Window, WindowSystem},
    *,
};

use std::rc::Rc;

pub(crate) struct RiddleState {
    pub window: Rc<WindowSystem>,
    pub input: Rc<InputSystem<Rc<Window>>>,
    pub audio: Rc<AudioSystem>,
    pub time: Rc<TimeSystem>,
}

impl RiddleState {
    pub fn new() -> Result<Self, RiddleError> {
        let window = WindowSystem::new();
        let input = InputSystem::new(window.event_pub())?;
        let audio = AudioSystem::new()?;
        let time = TimeSystem::new();

        Ok(RiddleState {
            window: window.into(),
            input: input.into(),
            audio: audio.into(),
            time: time.into(),
        })
    }
}
