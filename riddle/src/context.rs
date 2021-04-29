use crate::*;

use std::borrow::Borrow;

/// Riddle main thread context. It can be accessed before the main event loop
/// is started via [`RiddleLib::context`] and is provided to the event loop update
/// closure.
///
/// A context is needed for creating some resources, like `Window`s.
pub struct RiddleContext<'a> {
	pub(crate) window_ctx: platform::PlatformContext<'a>,
	pub(crate) state: &'a RiddleState,
	pub(crate) event: Event,
}

impl<'a> RiddleContext<'a> {
	/// Issue a quit request. The main loop will terminate once the quit message
	/// is processed.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{*, platform::*, renderer::*, common::Color};
	/// # fn main() -> Result<(), RiddleError> {
	/// let rdl = RiddleLib::new()?;
	///
	/// rdl.run(move |rdl| {
	///     // Quit issued
	///     rdl.quit();
	///     // The program will continue to execute until the quit event is handled
	///     // by the main event loop.
	/// })
	/// # }
	/// ```
	pub fn quit(&self) {
		self.window_ctx.quit().unwrap();
	}

	/// Get the event associated with this context.
	///
	/// If the context was created by [`RiddleLib::context`] the event will be
	/// [`Event::PreRunPlaceholder`].
	///
	/// [`Event::ProcessFrame`] should be used to execute "per-frame" logic.
	pub fn event(&self) -> &Event {
		&self.event
	}

	/// The Riddle state, allowing systems to be queried.
	pub fn state(&self) -> &RiddleState {
		&self.state
	}

	/// The audio system.
	#[cfg(feature = "riddle-audio")]
	pub fn audio(&self) -> &audio::AudioSystem {
		&self.state.audio()
	}

	/// The input system.
	pub fn input(&self) -> &input::InputSystem {
		&self.state.input()
	}

	/// The time system.
	pub fn time(&self) -> &time::TimeSystem {
		&self.state.time()
	}

	/// The platform system.
	pub fn platform(&self) -> &platform::PlatformSystem {
		&self.state.platform()
	}
}

impl<'a> Borrow<platform::PlatformContext<'a>> for RiddleContext<'a> {
	fn borrow(&self) -> &platform::PlatformContext<'a> {
		&self.window_ctx
	}
}

impl<'a> Borrow<platform::PlatformContext<'a>> for &RiddleContext<'a> {
	fn borrow(&self) -> &platform::PlatformContext<'a> {
		&self.window_ctx
	}
}
