use crate::*;

/// Riddle library initializer
///
/// Stores the state of all initialized riddle systems before the main event
/// loop is started with [`RiddleLib::run`] or [`RiddleLib::run_with_err`] and
/// this object is consumed
pub struct RiddleLib {
	pub(crate) state: RiddleState,
	main_thread_state: MainThreadState,
}

impl RiddleLib {
	/// Initialize Riddle. All available subsystems will be initialized.
	pub fn new() -> Result<Self> {
		let (state, main_thread_state) = RiddleState::new()?;
		Ok(Self {
			state,
			main_thread_state,
		})
	}

	/// Begin the main event loop of the program.
	///
	/// The update closure is given a [`RiddleContext`] which provides
	/// access to state which is only available on the main thread, and
	/// the event that caused the update closure to be called
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{*, platform::*, renderer::*, common::Color};
	/// # fn main() -> Result<(), RiddleError> {
	/// // Initialize the library
	/// let rdl = RiddleLib::new()?;
	///
	/// // Start the main event loop
	/// rdl.run(move |rdl| {
	///     // Application logic
	///     // [..]
	/// #   rdl.quit();
	/// })
	/// # }
	/// ```
	pub fn run<F>(self, mut update: F) -> !
	where
		F: FnMut(&RiddleContext) + 'static,
	{
		self.run_with_err::<RiddleError, _>(move |ctx| {
			update(ctx);
			Ok(())
		});
	}

	/// Begin the main event loop of the program, with an Error result type,
	/// to allow the program to terminate by returning an Error value.
	///
	/// See [`RiddleLib::run`] for more details.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{*, platform::*, renderer::*, common::Color};
	/// # fn main() -> Result<(), RiddleError> {
	/// // Initialize the library
	/// let rdl = RiddleLib::new()?;
	///
	/// // Start the main event loop
	/// rdl.run_with_err::<RiddleError, _>(move |rdl| {
	///     // Application logic
	///     // [..]
	/// #   rdl.quit();
	///     Ok(())
	/// })
	/// # }
	/// ```
	pub fn run_with_err<Err: std::fmt::Debug, F>(self, update: F) -> !
	where
		F: FnMut(&RiddleContext) -> std::result::Result<(), Err> + 'static,
	{
		let RiddleLib {
			state,
			main_thread_state,
		} = self;
		main_thread_state.run(state, update);
	}

	/// Get a [`RiddleContext`] during program initialization, before the main
	/// event loop is started.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{*, platform::*};
	/// # fn main() -> Result<(), RiddleError> {
	/// let rdl =  RiddleLib::new()?;
	///
	/// // Get a context before the application starts the main event loop.
	/// let window: Window = WindowBuilder::new().build(rdl.context())?;
	/// # Ok (()) }
	/// ```
	pub fn context(&self) -> RiddleContext {
		let platform_ctx = self.main_thread_state.platform.borrow_context();
		RiddleContext {
			state: &self.state,
			window_ctx: platform_ctx,
			event: Event::PreRunPlaceholder,
		}
	}

	/// The system state which is thread safe, and which can be cloned for convenient access
	/// throughout the program.
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::*;
	/// # fn main() -> Result<(), RiddleError> {
	/// // Initialize the library
	/// let rdl = RiddleLib::new()?;
	///
	/// let rdl_state = rdl.state().clone();
	/// std::thread::spawn(move || {
	///     // rdl_state is available for accessing input, time, audio, etc.
	/// });
	/// # Ok(()) }
	/// ```
	pub fn state(&self) -> &RiddleState {
		&self.state
	}
}
