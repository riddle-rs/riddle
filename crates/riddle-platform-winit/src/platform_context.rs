use crate::{event::InternalEvent, *};

use std::ops::Deref;

pub struct PlatformContext<'a> {
    pub(crate) main_thread_state: &'a PlatformMainThreadState,
    pub(crate) event_loop: Option<&'a winit::event_loop::EventLoopWindowTarget<InternalEvent>>,
    pub(crate) triggering_event: PlatformEvent,
}

impl<'a> PlatformContext<'a> {
    pub(crate) fn with_event_loop<T, F>(&self, f: F) -> Result<T, WindowError>
    where
        F: FnOnce(
            &winit::event_loop::EventLoopWindowTarget<InternalEvent>,
        ) -> Result<T, WindowError>,
    {
        match self.event_loop {
            Some(el) => f(el),
            None => {
                let el_ref = self.main_thread_state.event_loop.borrow();
                let el = el_ref.deref();
                match el {
                    Some(el) => f(el),
                    None => Err(WindowError::Unknown),
                }
            }
        }
    }

    pub fn quit(&self) -> Result<(), WindowError> {
        self.main_thread_state
            .system
            .event_proxy
            .lock()
            .unwrap()
            .send_event(InternalEvent::QuitRequested)
            .map_err(|_| WindowError::Unknown)
    }

    pub fn event(&self) -> &PlatformEvent {
        &self.triggering_event
    }

    pub fn system(&self) -> &PlatformSystem {
        &self.main_thread_state.system
    }
}
