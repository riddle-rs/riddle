#[cfg(feature = "riddle-audio")]
use crate::audio::{ext::*, AudioMainThreadState, AudioSystem, AudioSystemHandle};
use crate::{
    input::{ext::*, InputMainThreadState, InputSystem, InputSystemHandle},
    platform::{ext::*, PlatformMainThreadState, PlatformSystem, PlatformSystemHandle},
    time::{ext::*, TimeSystem, TimeSystemHandle},
    *,
};

/// Riddle subsystem state handles
///
/// Provides access to all the thread-safe state associated with riddle systems.
#[derive(Clone)]
pub struct RiddleState {
    platform: PlatformSystemHandle,
    input: InputSystemHandle,
    time: TimeSystemHandle,

    #[cfg(feature = "riddle-audio")]
    audio: AudioSystemHandle,
}

impl RiddleState {
    pub(crate) fn new() -> Result<(Self, MainThreadState)> {
        let (platform_system, platform_main_thread) = PlatformSystem::new();
        let (input_system, input_main_thread) = InputSystem::new(platform_system.event_pub())?;
        let time = TimeSystem::new();

        #[cfg(feature = "riddle-audio")]
        let (audio, audio_main_thread) = AudioSystem::new()?;

        let riddle_state = RiddleState {
            platform: platform_system,
            input: input_system,
            time: time,

            #[cfg(feature = "riddle-audio")]
            audio: audio,
        };

        let main_thread_state = MainThreadState {
            platform: platform_main_thread,
            input: input_main_thread,

            #[cfg(feature = "riddle-audio")]
            audio: audio_main_thread,
        };

        Ok((riddle_state, main_thread_state))
    }

    /// Platform system state
    pub fn platform(&self) -> &PlatformSystem {
        &self.platform
    }

    /// Input system state
    pub fn input(&self) -> &InputSystem {
        &self.input
    }

    /// Time system state
    pub fn time(&self) -> &TimeSystem {
        &self.time
    }

    /// Audio system state
    #[cfg(feature = "riddle-audio")]
    #[doc(cfg(feature = "riddle-audio"))]
    pub fn audio(&self) -> &AudioSystem {
        &self.audio
    }
}

pub(crate) struct MainThreadState {
    pub(crate) platform: PlatformMainThreadState,
    input: InputMainThreadState,

    #[cfg(feature = "riddle-audio")]
    pub audio: AudioMainThreadState,
}

impl MainThreadState {
    #[inline]
    pub fn run<Err: std::fmt::Debug, F>(self, state: RiddleState, mut update: F) -> !
    where
        F: FnMut(&RiddleContext) -> std::result::Result<(), Err> + 'static,
    {
        let MainThreadState {
            platform,
            mut input,
            audio: _audio,
        } = self;
        platform.run::<Err, _>(move |platform_ctx| {
            match platform_ctx.event() {
                platform::PlatformEvent::EventQueueEmpty => {
                    state.time.process_frame();

                    #[cfg(feature = "riddle-audio")]
                    state.audio.process_frame();
                }
                _ => (),
            };

            input.process_input();

            let event = match platform_ctx.event() {
                platform::PlatformEvent::EventQueueEmpty => Event::ProcessFrame,
                _ => Event::Platform(platform_ctx.event().clone()),
            };

            let mut ctx = RiddleContext {
                window_ctx: platform_ctx,
                state: &state,
                event,
            };

            update(&ctx)?;

            let input_events = state.input.take_input_events();
            for input_event in input_events {
                ctx.event = Event::Input(input_event);
                update(&ctx)?;
            }

            Ok(())
        })
    }
}
