use crate::*;

pub trait PlatformSystemExt {
	/// Create a new platform system and its main thread state pair.
	///
	/// **Do not** call if using the `riddle` crate as recommended, as ` RiddleLib` manages
	/// the creation and platform lifetime automatically.
	///
	/// # Example
	///
	/// This example is only relevant if you're not using `riddle`
	///
	/// ```no_run
	/// # use riddle_platform_winit::{ext::*, *};
	/// # fn main() -> Result<(), PlatformError> {
	/// let (platform_system, main_thread_state) = PlatformSystem::new_shared();
	/// let window = WindowBuilder::new().build(main_thread_state.borrow_context())?;
	///
	/// main_thread_state.run::<PlatformError, _>(move |ctx| {
	///     match ctx.event() {
	///         PlatformEvent::WindowClose(_) => { ctx.quit(); }
	///         _ => ()
	///     };
	///     Ok(())
	/// })
	/// # }
	/// ```
	fn new_shared() -> (PlatformSystemHandle, PlatformMainThreadState);
}
