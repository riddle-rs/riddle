use crate::*;

use std::rc::Rc;

pub struct RiddleApp {
    pub(crate) state: Rc<RiddleState>,
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
        window::WindowSystem::run(window_system, move |window_ctx| {
            let ctx = RiddleContext {
                window_ctx,
                state: self.state.clone(),
            };

            match ctx.event() {
                window::SystemEvent::ProcessFrame => {
                    self.state.time.process_frame();
                }
                _ => (),
            };

            self.state.input.update();
            update(&ctx);
        })
    }

    pub fn context(&self) -> RiddleContext {
        let state = self.state.clone();
        let window_ctx = window::WindowSystem::borrow_context(&self.state.window).unwrap();
        RiddleContext { state, window_ctx }
    }
}
