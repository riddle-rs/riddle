use crate::{event::InternalEvent, *};

use std::ops::Deref;

/// The platform system context provided to the application main thread.
///
/// It is used to allow the application to deal with platform events, and
/// perform actions which must be done on the main thread (eg. creating windows).
///
/// To acquire a context when using `riddle` (the **recommended** approach):
///
/// * ` RiddleLib::context()` returns a `RiddleContext` which implements `Borrow<PlatformContext>`.
/// * ` RiddleLib::run()` passes a `RiddleContext` to the application callback.
///
/// To acquire a context when using this crate directly:
///
/// * [`PlatformMainThreadState::borrow_context()`]
/// * [`PlatformMainThreadState::run`] passes it to application callback
///
/// # Example
///
/// ```no_run
/// use riddle::{*, platform::*};
///
/// fn main() -> Result<(), RiddleError> {
///     let rdl =  RiddleLib::new()?;
///
///     // Get a context before the application starts the main event loop.
///     let window_a: WindowHandle = WindowBuilder::new().build(rdl.context())?;
///     let mut window_b: Option<WindowHandle> = None;
///
///     rdl.run(move |rdl| {
///         if window_b.is_none() {
///             // rdl: RiddleContext, is used to build the second window.
///             window_b = Some(WindowBuilder::new().build(rdl).unwrap());
///         } else {
///             rdl.quit();
///         }
/// #       std::thread::sleep(std::time::Duration::from_secs(1));
///     })
/// }
/// ```
pub struct PlatformContext<'a> {
    pub(crate) main_thread_state: &'a PlatformMainThreadState,
    pub(crate) event_loop: Option<&'a winit::event_loop::EventLoopWindowTarget<InternalEvent>>,
    pub(crate) triggering_event: PlatformEvent,
}

impl<'a> PlatformContext<'a> {
    pub(crate) fn with_event_loop<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&winit::event_loop::EventLoopWindowTarget<InternalEvent>) -> Result<T>,
    {
        match self.event_loop {
            Some(el) => f(el),
            None => {
                let el_ref = self.main_thread_state.event_loop.borrow();
                let el = el_ref.deref();
                match el {
                    Some(el) => f(el),
                    None => Err(PlatformError::InvalidContextState),
                }
            }
        }
    }

    /// Issue a quit request to the underlying platform system.
    ///
    /// The application will quit when that message is processed by the
    /// main event loop.
    pub fn quit(&self) -> Result<()> {
        self.main_thread_state
            .system
            .event_proxy
            .lock()
            .unwrap()
            .send_event(InternalEvent::QuitRequested)
            .map_err(|_| PlatformError::MessageDispatchError)
    }

    /// Get the event associated with the context.
    ///
    /// This will be the platform event that triggered the application closure, or
    /// [`PlatformEvent::Unknown`] if the context was created before the main event loop
    /// has started.
    pub fn event(&self) -> &PlatformEvent {
        &self.triggering_event
    }

    /// The platform system associated with this context.
    pub fn system(&self) -> &PlatformSystem {
        &self.main_thread_state.system
    }
}
