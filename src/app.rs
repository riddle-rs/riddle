use crate::*;

pub struct RiddleApp {
    pub(crate) state: RiddleState,
    main_thread_state: MainThreadState,
}

impl RiddleApp {
    pub fn new() -> Result<Self, RiddleError> {
        let (state, main_thread_state) = RiddleState::new()?;
        Ok(Self {
            state,
            main_thread_state,
        })
    }

    pub fn run<F>(self, update: F) -> !
    where
        F: FnMut(&RiddleContext) -> () + 'static,
    {
        let RiddleApp {
            state,
            main_thread_state,
        } = self;
        main_thread_state.run(state, update);
    }

    pub fn context(&self) -> RiddleContext {
        let platform_ctx = self.main_thread_state.platform.borrow_context().unwrap();
        RiddleContext {
            state: &self.state,
            window_ctx: platform_ctx,
            event: Event::PreRunPlaceholder,
        }
    }

    pub fn state(&self) -> &RiddleState {
        &self.state
    }
}
