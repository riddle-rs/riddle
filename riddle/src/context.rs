use crate::*;

use std::borrow::Borrow;

/// An riddle execution context. A context is always associated with the event that caused
/// the context to be created.
///
/// A context is used for creating root resources like [`Window`].
pub struct RiddleContext<'a> {
    pub(crate) window_ctx: platform::PlatformContext<'a>,
    pub(crate) state: &'a RiddleState,
    pub(crate) event: Event,
}

impl<'a> RiddleContext<'a> {
    /// Issue quit request to the window context.
    ///
    /// # Panic
    ///
    /// If the underlying quit function results in an Err, panic. A quit was being requested anyway, probably
    //  best to err on the side of termination.
    pub fn quit(&self) {
        self.window_ctx.quit().unwrap();
    }

    /// Get the event associated with this context
    pub fn event(&self) -> &Event {
        &self.event
    }

    pub fn state(&self) -> &RiddleState {
        &self.state
    }

    #[cfg(feature = "riddle-audio")]
    pub fn audio(&self) -> &audio::AudioSystem {
        &self.state.audio
    }

    pub fn input(&self) -> &input::InputSystem {
        &self.state.input
    }

    pub fn time(&self) -> &time::TimeSystem {
        &self.state.time
    }
}

impl<'a> Borrow<platform::PlatformContext<'a>> for RiddleContext<'a> {
    fn borrow(&self) -> &platform::PlatformContext<'a> {
        &self.window_ctx
    }
}
