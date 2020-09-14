#[cfg(feature = "riddle-audio")]
use crate::audio::{AudioSystem, AudioSystemHandle};
use crate::{
    input::InputSystem,
    platform::{PlatformMainThreadState, PlatformSystem, PlatformSystemHandle},
    time::TimeSystem,
    *,
};

use std::rc::Rc;

#[derive(Clone)]
pub struct RiddleState {
    pub platform: PlatformSystemHandle,
    pub input: Rc<InputSystem>,
    pub time: Rc<TimeSystem>,

    #[cfg(feature = "riddle-audio")]
    pub audio: AudioSystemHandle,
}

impl RiddleState {
    pub(crate) fn new() -> Result<(Self, MainThreadState), RiddleError> {
        let (platform_system, platform_main_thread) = PlatformSystem::new();
        let input = InputSystem::new(platform_system.event_pub())?;
        let time = TimeSystem::new();

        #[cfg(feature = "riddle-audio")]
        let audio = AudioSystem::new()?;

        let riddle_state = RiddleState {
            platform: platform_system,
            input: input.into(),
            time: time.into(),

            #[cfg(feature = "riddle-audio")]
            audio: audio,
        };

        let main_thread_state = MainThreadState {
            platform: platform_main_thread,
        };

        Ok((riddle_state, main_thread_state))
    }

    pub fn platform(&self) -> &PlatformSystem {
        &self.platform
    }

    pub fn input(&self) -> Rc<InputSystem> {
        self.input.clone()
    }

    pub fn time(&self) -> Rc<TimeSystem> {
        self.time.clone()
    }

    pub fn audio(&self) -> &AudioSystem {
        &self.audio
    }
}

pub(crate) struct MainThreadState {
    pub(crate) platform: PlatformMainThreadState,
}

impl MainThreadState {
    #[inline]
    pub fn run<F>(self, state: RiddleState, mut update: F) -> !
    where
        F: FnMut(&RiddleContext) -> () + 'static,
    {
        let MainThreadState { platform } = self;
        platform.run(move |platform_ctx| {
            match platform_ctx.event() {
                platform::PlatformEvent::EventQueueEmpty => {
                    state.time.process_frame();

                    #[cfg(feature = "riddle-audio")]
                    state.audio.process_frame();
                }
                _ => (),
            };

            state.input.update();

            let event = match platform_ctx.event() {
                platform::PlatformEvent::EventQueueEmpty => Event::ProcessFrame,
                _ => Event::Platform(platform_ctx.event().clone()),
            };

            let mut ctx = RiddleContext {
                window_ctx: platform_ctx,
                state: &state,
                event,
            };
            update(&ctx);

            let input_events = state.input.take_input_events();
            for input_event in input_events {
                ctx.event = Event::Input(input_event);
                update(&ctx);
            }
        })
    }
}
