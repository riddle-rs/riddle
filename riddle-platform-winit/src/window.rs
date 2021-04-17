use crate::{common::*, *};

use riddle_common::eventpub::*;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use riddle_platform_common::traits::WindowCommon;
use std::borrow::Borrow;

/// A platform native window.
///
/// Construct windows using [`WindowBuilder`].
///
/// The window is kept open for the lifetime of this object, once it is
/// dropped the window will be closed during the next event loop pump.
///
/// # Example
///
/// ```no_run
/// # use riddle::{*, platform::*};
/// # fn main() -> Result<(), RiddleError> {
/// let rdl =  RiddleLib::new()?;
///
/// // Create a window
/// let mut window = Some(WindowBuilder::new().build(rdl.context())?);
///
/// rdl.run(move |rdl| {
///     if window.is_some() {
///         // Drop all handles to the window. It will be closed before the next invocation
///         // of this closure.
///         window.take();
///     } else {
///         // The window is closed. Exit.
///         rdl.quit();
///     }
/// #   std::thread::sleep(std::time::Duration::from_millis(500));
/// })
/// # }
/// ```
pub struct Window {
	weak_self: WindowWeak,
	window_system: PlatformSystemHandle,
	winit_window: winit::window::Window,
	event_sub: EventSub<PlatformEvent>,
	event_pub: EventPub<PlatformEvent>,

	id: WindowId,
}

define_handles!(<Window>::weak_self, pub WindowHandle, pub WindowWeak);

impl Window {
	fn new_shared(ctx: &PlatformContext, args: &WindowBuilder) -> Result<WindowHandle> {
		let system = ctx.system().clone_handle();

		#[cfg(target_os = "windows")]
		let winit_builder = {
			use winit::platform::windows::WindowBuilderExtWindows;
			winit::window::WindowBuilder::new()
				// Required to prevent cpal fighting with winit over ole threading model and panicing
				// Tracking Issues:
				// https://github.com/RustAudio/cpal/pull/348
				// https://github.com/rust-windowing/winit/pull/1524
				.with_drag_and_drop(false)
		};

		#[cfg(not(target_os = "windows"))]
		let winit_builder = winit::window::WindowBuilder::new();

		let winit_builder = winit_builder
			.with_inner_size(winit::dpi::LogicalSize::new(
				args.width as f32,
				args.height as f32,
			))
			.with_title(args.title.clone())
			.with_resizable(args.resizeable);

		let winit_window = ctx.with_event_loop(|el| {
			winit_builder
				.build(el)
				.map_err(|_| PlatformError::WindowInitFailure)
		})?;

		#[cfg(target_arch = "wasm32")]
		{
			use winit::platform::web::WindowExtWebSys;

			let canvas = winit_window.canvas();

			let window = web_sys::window().unwrap();
			let document = window.document().unwrap();
			let body = document.body().unwrap();

			body.append_child(&canvas)
				.expect("Append canvas to HTML body");
		}

		args.configure_window(&winit_window);

		let event_sub = EventSub::new_with_filter(Self::event_filter);
		ctx.system().event_pub().attach(&event_sub);

		let window = WindowHandle::new(|weak_self| Self {
			weak_self,
			window_system: system.clone(),
			winit_window,
			event_sub,
			event_pub: EventPub::new(),
			id: system.with_window_map_mut(|wmap| wmap.take_next_window_id()),
		});

		ctx.system()
			.with_window_map_mut(|wmap| wmap.register_window(window.clone()));

		Ok(window)
	}

	/// Get the size of the drawable area of the window in pixels
	pub fn physical_size(&self) -> (u32, u32) {
		let size = self.winit_window.inner_size();
		(size.width, size.height)
	}

	/// Get the size of the drawable area of the window in logical units.
	pub fn logical_size(&self) -> LogicalSize {
		let physical_size = self.winit_window.inner_size();
		let logical_size: winit::dpi::LogicalSize<u32> =
			physical_size.to_logical(self.winit_window.scale_factor());
		dimensions::logical_size_from_winit(logical_size)
	}

	/// Get the scale factor of the window, based on how the window manager
	/// configures hidpi scaling
	pub fn scale_factor(&self) -> f64 {
		self.winit_window.scale_factor()
	}

	/// Set the window title
	pub fn set_title(&self, title: &str) {
		self.winit_window.set_title(title)
	}

	/// Attach a subscriber to the subset of platform events that relate to this window
	///
	/// # Example
	///
	/// ```no_run
	/// # use riddle::{*, common::eventpub::*, platform::*};
	/// # fn main() -> Result<(), RiddleError> {
	/// let rdl =  RiddleLib::new()?;
	/// let subscriber: EventSub<PlatformEvent> = EventSub::new();
	///
	/// let window = WindowBuilder::new().build(rdl.context())?;
	/// window.subscribe_to_events(&subscriber);
	/// # Ok(()) }
	/// ```
	pub fn subscribe_to_events(&self, sub: &EventSub<PlatformEvent>) {
		self.event_pub.attach(sub);
	}

	/// The window id which is used to identify this window in [`PlatformEvent`]s
	pub fn id(&self) -> WindowId {
		self.id
	}

	pub(crate) fn update(&self) {
		for event in self.event_sub.collect() {
			self.event_pub.dispatch(event);
		}
	}

	pub(crate) fn winit_window_id(&self) -> winit::window::WindowId {
		self.winit_window.id()
	}

	fn event_filter(event: &PlatformEvent) -> bool {
		matches!(event, PlatformEvent::WindowResize(_))
	}
}

impl winit_ext::WinitWindowExt for Window {
	fn winit_window(&self) -> &winit::window::Window {
		&self.winit_window
	}
}

impl WindowCommon for Window {
	fn logical_to_physical<L: Into<LogicalVec2>>(&self, vec2: L) -> (u32, u32) {
		let winit_pos = dimensions::logical_vec_to_winit(vec2.into());
		let physical_size = winit_pos.to_physical(self.scale_factor());
		(physical_size.x, physical_size.y)
	}
}

impl std::cmp::PartialEq for Window {
	fn eq(&self, other: &Self) -> bool {
		self.id() == other.id()
	}
}

impl std::cmp::Eq for Window {}

impl Drop for Window {
	fn drop(&mut self) {
		self.window_system
			.with_window_map_mut(|wmap| wmap.unregister_window(&self));
	}
}

unsafe impl HasRawWindowHandle for Window {
	fn raw_window_handle(&self) -> RawWindowHandle {
		self.winit_window.raw_window_handle()
	}
}

/// Builder for [`Window`] instances.
///
/// Default values:
///
/// * Width: `800`
/// * Height: `600`
/// * Title: `Riddle Window`
/// * Resizeable: `true`
/// * Cursor Visible: `true`
///
/// # Example
///
/// ```no_run
/// use riddle::{*, platform::*};
///
/// fn main() -> Result<(), RiddleError> {
///     let rdl =  RiddleLib::new()?;
///
///     let window: WindowHandle = WindowBuilder::new()
///         .title("A Sample Title")
///         .dimensions(320, 240)
///         .build(rdl.context())?;
///
///     // The window has been created, and is visible now.
///     // [.. the rest of the application ..]
/// # Ok(())
/// }
/// ```
pub struct WindowBuilder {
	width: u32,
	height: u32,
	title: String,
	resizeable: bool,
	cursor_visible: bool,
}

impl Default for WindowBuilder {
	fn default() -> Self {
		WindowBuilder {
			width: 800,
			height: 600,
			title: String::from("Riddle Window"),
			resizeable: true,
			cursor_visible: true,
		}
	}
}

impl WindowBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	/// Set the dimensions of the window, in logical units.
	///
	/// The pixel dimensions of the window may be different if the display's scale factor
	/// does not equal `1`.
	pub fn dimensions(&mut self, width: u32, height: u32) -> &mut Self {
		self.width = width;
		self.height = height;
		self
	}

	/// Set whether the window is resizeable.
	pub fn resizeable(&mut self, resizeable: bool) -> &mut Self {
		self.resizeable = resizeable;
		self
	}

	/// Set the window's title.
	pub fn title<S: Into<String>>(&mut self, title: S) -> &mut Self {
		self.title = title.into();
		self
	}

	/// Set whether the system cursor will be displayed for the created window.
	pub fn cursor_visible(&mut self, cursor_visible: bool) -> &mut Self {
		self.cursor_visible = cursor_visible;
		self
	}

	/// Build the new window, returning a handle to the new window.
	///
	/// The window will be visible as long as the handle, or a clone of it, is alive.
	pub fn build<'a, C>(&self, ctx: C) -> Result<WindowHandle>
	where
		C: Borrow<PlatformContext<'a>>,
	{
		Window::new_shared(ctx.borrow(), self)
	}

	fn configure_window(&self, window: &winit::window::Window) {
		window.set_cursor_visible(self.cursor_visible);
	}
}
