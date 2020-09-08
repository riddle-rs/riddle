use crate::*;

pub struct RiddleApp {
    pub(crate) state: RiddleState,
}

impl RiddleApp {
    pub fn new() -> Result<Self, RiddleError> {
        Ok(Self {
            state: RiddleState::new()?.into(),
        })
    }

    pub fn run<F>(self, mut update: F) -> !
    where
        F: FnMut(&RiddleContext) -> () + 'static,
    {
        let window_system = self.state.window.clone();
        platform::PlatformSystem::run(window_system, move |platform_ctx| {
            match platform_ctx.event() {
                platform::PlatformEvent::EventQueueEmpty => {
                    self.state.time.process_frame();

                    #[cfg(feature = "riddle-audio")]
                    self.state.audio.process_frame();
                }
                _ => (),
            };

            self.state.input.update();

            let event = match platform_ctx.event() {
                platform::PlatformEvent::EventQueueEmpty => Event::ProcessFrame,
                _ => Event::PlatformEvent(platform_ctx.event().clone()),
            };

            let mut ctx = RiddleContext {
                window_ctx: platform_ctx,
                state: &self.state,
                event,
            };
            update(&ctx);

            let input_events = self.state.input.take_input_events();
            for input_event in input_events {
                ctx.event = Event::InputEvent(input_event);
                update(&ctx);
            }
        })
    }

    pub fn context(&self) -> RiddleContext {
        let state = &self.state;
        let platform_ctx = platform::PlatformSystem::borrow_context(&self.state.window).unwrap();
        RiddleContext {
            state,
            window_ctx: platform_ctx,
            event: Event::PreRunNullEvent,
        }
    }

    pub fn state(&self) -> &RiddleState {
        &self.state
    }
}
