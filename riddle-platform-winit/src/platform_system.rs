use crate::{common::*, event::InternalEvent, *};

use riddle_common::eventpub::EventPub;
use winit::event_loop::EventLoopWindowTarget;

use std::{cell::RefCell, sync::Mutex};

/// The winit platform system core state, along with [`PlatformMainThreadState`].
///
/// Mostly used to lookup [`Window`] by [`WindowId`], and subscribe to [`PlatformEvent`]s.
#[derive(Clone)]
pub struct PlatformSystem {
	pub(crate) internal: std::sync::Arc<PlatformSystemInternal>,
}

pub(crate) struct PlatformSystemInternal {
	pub(crate) event_proxy: Mutex<winit::event_loop::EventLoopProxy<InternalEvent>>,
	window_map: Mutex<WindowMap>,
	event_pub: EventPub<PlatformEvent>,
}

impl PlatformSystem {
	/// Get the [`PlatformEvent`] publisher, so that other systems can consume events.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{*, common::eventpub::*, platform::*};
	/// # fn main() -> Result<(), RiddleError> {
	/// let rdl =  RiddleLib::new()?;
	/// let subscriber: EventSub<PlatformEvent> = EventSub::new();
	///
	/// // Attach subscriber to the platform event stream
	/// rdl.state().platform().event_pub().attach(&subscriber);
	/// # Ok(()) }
	/// ```
	pub fn event_pub(&self) -> &EventPub<PlatformEvent> {
		&self.internal.event_pub
	}

	/// Get a [`Window`] associated with a [`WindowId`], if one exists.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{*, common::eventpub::*, platform::*};
	/// # fn main() -> Result<(), RiddleError> {
	/// let rdl =  RiddleLib::new()?;
	/// let window = WindowBuilder::new().build(rdl.context())?;
	/// let window_id = window.id();
	/// assert!(Window::eq(&window,
	///        &rdl.state().platform().lookup_window(window_id).unwrap()));
	/// # Ok(()) }
	/// ```
	pub fn lookup_window(&self, window_id: WindowId) -> Option<Window> {
		self.internal
			.window_map
			.lock()
			.unwrap()
			.lookup_window(window_id)
	}

	#[inline]
	pub(crate) fn with_window_map<R, F: FnOnce(&WindowMap) -> R>(&self, f: F) -> R {
		f(&self.internal.window_map.lock().unwrap())
	}

	#[inline]
	pub(crate) fn with_window_map_mut<R, F: FnOnce(&mut WindowMap) -> R>(&self, f: F) -> R {
		let mut window_map_lock = self.internal.window_map.lock().unwrap();
		f(&mut window_map_lock)
	}

	fn update_windows(&self) {
		let windows = self.internal.window_map.lock().unwrap().windows();
		for window in windows {
			window.update()
		}
	}
}

impl ext::PlatformSystemExt for PlatformSystem {
	fn new_system_pair() -> (PlatformSystem, PlatformMainThreadState) {
		let event_loop = winit::event_loop::EventLoop::with_user_event();
		let event_proxy = event_loop.create_proxy();

		let internal = PlatformSystemInternal {
			event_proxy: Mutex::new(event_proxy),

			window_map: WindowMap::new().into(),

			event_pub: EventPub::new(),
		};
		let system = Self {
			internal: std::sync::Arc::new(internal),
		};

		let main_thread_state = PlatformMainThreadState {
			system: system.clone(),
			event_loop: RefCell::new(Some(event_loop)),
		};
		(system, main_thread_state)
	}
}

pub struct PlatformMainThreadState {
	pub(crate) system: PlatformSystem,
	pub(crate) event_loop: RefCell<Option<winit::event_loop::EventLoop<InternalEvent>>>,
}

impl PlatformMainThreadState {
	/// Starts the main even loop for this window system.
	///
	/// # Panics
	///
	/// If run has already been invoked, then this function will panic.
	pub fn run<Err: std::fmt::Debug, F>(self, main_loop: F) -> !
	where
		F: FnMut(PlatformContext) -> std::result::Result<(), Err> + 'static,
	{
		let el = std::mem::replace(&mut *self.event_loop.borrow_mut(), None).unwrap();
		let mut main_loop = main_loop;
		let this = self.system.clone();
		el.run(move |event, el, cf| {
			match &event {
				winit::event::Event::UserEvent(InternalEvent::QuitRequested) => {
					*cf = winit::event_loop::ControlFlow::Exit
				}
				_ => *cf = winit::event_loop::ControlFlow::Poll,
			}

			// Dispatch event
			if let Some(system_event) = event::convert_winit_event(&this, event) {
				self.dispatch_event(el, system_event, &mut main_loop);
			}
		})
	}

	pub fn borrow_context(&self) -> PlatformContext {
		PlatformContext {
			main_thread_state: &self,
			event_loop: None,
			triggering_event: PlatformEvent::Unknown,
		}
	}

	fn dispatch_event<Err: std::fmt::Debug, F>(
		&self,
		el: &EventLoopWindowTarget<InternalEvent>,
		system_event: PlatformEvent,
		main_loop: &mut F,
	) where
		F: FnMut(PlatformContext) -> std::result::Result<(), Err> + 'static,
	{
		let system = self.system.clone();
		let ctx = PlatformContext {
			main_thread_state: &self,
			event_loop: Some(el),
			triggering_event: system_event.clone(),
		};

		system.internal.event_pub.dispatch(system_event);
		system.update_windows();

		main_loop(ctx).unwrap();
	}
}
