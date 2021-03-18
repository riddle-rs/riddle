use crate::*;

use riddle_common::eventpub::EventPub;
use riddle_platform_common::PlatformEvent;

pub trait InputSystemExt {
	/// Create the input system, initializing any input device libraries needed.
	///
	/// This will be instantiated automatically if `riddle` is being used.
	///
	/// Returns a pair of objects, one of which is the system state which is like
	/// most other riddle system states - it is thread safe, stored in `RiddleState`
	/// and is the main means by which client code interacts with the system.
	///
	/// The other return value stores the portion of this object's state that should
	/// stay on the main thread - [`InputMainThreadState`].
	fn new_shared(
		sys_events: &EventPub<PlatformEvent>,
	) -> Result<(InputSystemHandle, InputMainThreadState)>;

	/// Collect any buffered [`InputEvent`]s emitted by the input system.
	///
	/// This clears the system's buffer, so should only be called from a single location.
	///
	/// **Do not** call this if you are using `riddle`. `riddle` manages taking these
	/// events and passing them to the main application closure.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_input::{ext::*, *}; use riddle_common::eventpub::*; use riddle_platform_common::*;
	/// # fn main() -> Result<(), InputError> {
	/// let platform_events: EventPub<PlatformEvent> = EventPub::new();
	/// let (input_system, mut main_thread_state) = InputSystem::new_shared(&platform_events)?;
	///
	/// // Platform dispatches an event, and input processes it
	/// platform_events.dispatch(PlatformEvent::MouseButtonDown{
	///     window: WindowId::new(0),
	///     button: MouseButton::Left});
	/// main_thread_state.process_input();
	///
	/// // Having processed the incoming platform event, there is now an InputEvent available
	/// let input_events: Vec<InputEvent> = input_system.take_input_events();
	///
	/// assert_eq!(vec![InputEvent::MouseButtonDown{
	///     window: WindowId::new(0),
	///     button: MouseButton::Left
	/// }], input_events);
	/// # Ok(()) }
	/// ```
	fn take_input_events(&self) -> Vec<InputEvent>;
}
