use crate::*;

use std::{borrow::Borrow, rc::Rc};

/// An riddle execution context. A context is always associated with the event that caused
/// the context to be created.
///
/// A context is used for creating root resources like [`Window`].
pub struct RiddleContext<'a> {
    pub(crate) window_ctx: window::WindowContext<'a>,
    pub(crate) state: &'a RiddleState,
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
    pub fn event(&self) -> &window::SystemEvent {
        &self.window_ctx.event()
    }

    pub fn state(&self) -> &RiddleState {
        &self.state
    }

    #[cfg(feature = "riddle-audio")]
    pub fn audio(&self) -> Rc<audio::AudioSystem> {
        self.state.audio.clone()
    }

    pub fn input(&self) -> &input::InputSystem {
        &self.state.input
    }

    pub fn time(&self) -> &time::TimeSystem {
        &self.state.time
    }
}

#[cfg(feature = "riddle-audio")]
impl<'a> Borrow<Rc<audio::AudioSystem>> for RiddleContext<'a> {
    fn borrow(&self) -> &Rc<audio::AudioSystem> {
        &self.state.audio
    }
}

impl<'a> Borrow<window::WindowContext<'a>> for RiddleContext<'a> {
    fn borrow(&self) -> &window::WindowContext<'a> {
        &self.window_ctx
    }
}
