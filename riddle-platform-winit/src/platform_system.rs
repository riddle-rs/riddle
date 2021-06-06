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

			// Workaround for winit lacking IME events
			if let Some(text_event) = self.simulate_textinput_event(&event) {
				self.dispatch_event(el, text_event, &mut main_loop);
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

	//Workaround for winit supporting IME events (https://github.com/rust-windowing/winit/issues/1497)
	fn simulate_textinput_event(
		&self,
		event: &winit::event::Event<InternalEvent>,
	) -> Option<PlatformEvent> {
		#[allow(deprecated)]
		let (window_id, vkey, modifiers) = if let winit::event::Event::WindowEvent {
			window_id,
			event:
				winit::event::WindowEvent::KeyboardInput {
					input:
						winit::event::KeyboardInput {
							state: winit::event::ElementState::Pressed,
							virtual_keycode: Some(vkey),
							modifiers,
							..
						},
					..
				},
		} = &event
		{
			(*window_id, *vkey, *modifiers)
		} else {
			return None;
		};

		let text = String::from(keyevent_to_textinput(
			event::winit_vkey_to_vkey(vkey),
			modifiers,
		)?);

		let window = self
			.system
			.with_window_map(|wmap| wmap.lookup_winit_window(window_id))?;

		Some(PlatformEvent::TextInput {
			window: window.id(),
			text,
		})
	}
}

/// Workaround for winit not supporting IME events yet. This hard-codes my personal keyboard layout
/// and locale (English (Ireland) - Irish)
fn keyevent_to_textinput(
	vkey: Option<VirtualKey>,
	modifiers: winit::event::ModifiersState,
) -> Option<char> {
	if modifiers.ctrl() || modifiers.alt() {
		return None;
	}

	let vkey = vkey?;
	match (modifiers.shift(), vkey) {
		(true, VirtualKey::One) => Some('!'),
		(true, VirtualKey::Two) => Some('"'),
		(true, VirtualKey::Three) => Some('£'),
		(true, VirtualKey::Four) => Some('$'),
		(true, VirtualKey::Five) => Some('%'),
		(true, VirtualKey::Six) => Some('^'),
		(true, VirtualKey::Seven) => Some('&'),
		(true, VirtualKey::Eight) => Some('*'),
		(true, VirtualKey::Nine) => Some('('),
		(true, VirtualKey::Zero) => Some(')'),
		(true, VirtualKey::Minus) => Some('_'),
		(true, VirtualKey::Equal) => Some('+'),
		(true, VirtualKey::Q) => Some('Q'),
		(true, VirtualKey::W) => Some('W'),
		(true, VirtualKey::E) => Some('E'),
		(true, VirtualKey::R) => Some('R'),
		(true, VirtualKey::T) => Some('T'),
		(true, VirtualKey::Y) => Some('Y'),
		(true, VirtualKey::U) => Some('U'),
		(true, VirtualKey::I) => Some('I'),
		(true, VirtualKey::O) => Some('O'),
		(true, VirtualKey::P) => Some('P'),
		(true, VirtualKey::LeftBrace) => Some('{'),
		(true, VirtualKey::RightBrace) => Some('}'),
		(true, VirtualKey::A) => Some('A'),
		(true, VirtualKey::D) => Some('D'),
		(true, VirtualKey::S) => Some('S'),
		(true, VirtualKey::F) => Some('F'),
		(true, VirtualKey::G) => Some('G'),
		(true, VirtualKey::H) => Some('H'),
		(true, VirtualKey::J) => Some('J'),
		(true, VirtualKey::K) => Some('K'),
		(true, VirtualKey::L) => Some('L'),
		(true, VirtualKey::Semicolon) => Some(':'),
		(true, VirtualKey::Apostrope) => Some('@'),
		(true, VirtualKey::Grave) => Some('~'),
		(true, VirtualKey::Backslash) => Some('?'),
		(true, VirtualKey::Z) => Some('Z'),
		(true, VirtualKey::X) => Some('X'),
		(true, VirtualKey::C) => Some('C'),
		(true, VirtualKey::V) => Some('V'),
		(true, VirtualKey::B) => Some('B'),
		(true, VirtualKey::N) => Some('N'),
		(true, VirtualKey::M) => Some('M'),
		(true, VirtualKey::Comma) => Some('<'),
		(true, VirtualKey::Dot) => Some('>'),
		(true, VirtualKey::Slash) => Some('?'),
		(true, VirtualKey::NonUsBackslash) => Some('|'),
		(false, VirtualKey::One) => Some('1'),
		(false, VirtualKey::Two) => Some('2'),
		(false, VirtualKey::Three) => Some('3'),
		(false, VirtualKey::Four) => Some('4'),
		(false, VirtualKey::Five) => Some('5'),
		(false, VirtualKey::Six) => Some('6'),
		(false, VirtualKey::Seven) => Some('7'),
		(false, VirtualKey::Eight) => Some('8'),
		(false, VirtualKey::Nine) => Some('9'),
		(false, VirtualKey::Zero) => Some('0'),
		(false, VirtualKey::Minus) => Some('-'),
		(false, VirtualKey::Equal) => Some('='),
		(false, VirtualKey::Q) => Some('q'),
		(false, VirtualKey::W) => Some('w'),
		(false, VirtualKey::E) => Some('e'),
		(false, VirtualKey::R) => Some('r'),
		(false, VirtualKey::T) => Some('t'),
		(false, VirtualKey::Y) => Some('y'),
		(false, VirtualKey::U) => Some('u'),
		(false, VirtualKey::I) => Some('i'),
		(false, VirtualKey::O) => Some('o'),
		(false, VirtualKey::P) => Some('p'),
		(false, VirtualKey::LeftBrace) => Some('['),
		(false, VirtualKey::RightBrace) => Some(']'),
		(false, VirtualKey::A) => Some('a'),
		(false, VirtualKey::D) => Some('d'),
		(false, VirtualKey::S) => Some('s'),
		(false, VirtualKey::F) => Some('f'),
		(false, VirtualKey::G) => Some('g'),
		(false, VirtualKey::H) => Some('h'),
		(false, VirtualKey::J) => Some('j'),
		(false, VirtualKey::K) => Some('k'),
		(false, VirtualKey::L) => Some('l'),
		(false, VirtualKey::Semicolon) => Some(';'),
		(false, VirtualKey::Apostrope) => Some('\''),
		(false, VirtualKey::Grave) => Some('#'),
		(false, VirtualKey::Backslash) => Some('/'),
		(false, VirtualKey::Z) => Some('z'),
		(false, VirtualKey::X) => Some('x'),
		(false, VirtualKey::C) => Some('c'),
		(false, VirtualKey::V) => Some('v'),
		(false, VirtualKey::B) => Some('b'),
		(false, VirtualKey::N) => Some('n'),
		(false, VirtualKey::M) => Some('m'),
		(false, VirtualKey::Comma) => Some(','),
		(false, VirtualKey::Dot) => Some('.'),
		(false, VirtualKey::Slash) => Some('/'),
		(false, VirtualKey::NonUsBackslash) => Some('\\'),
		(_, VirtualKey::Tab) => Some('\t'),
		(_, VirtualKey::Enter) => Some('\n'),
		(_, VirtualKey::KeyPadAsterick) => Some('*'),
		(_, VirtualKey::Space) => Some(' '),
		(_, VirtualKey::KeyPad7) => Some('7'),
		(_, VirtualKey::KeyPad8) => Some('8'),
		(_, VirtualKey::KeyPad9) => Some('9'),
		(_, VirtualKey::KeyPadMinus) => Some('-'),
		(_, VirtualKey::KeyPad4) => Some('4'),
		(_, VirtualKey::KeyPad5) => Some('5'),
		(_, VirtualKey::KeyPad6) => Some('6'),
		(_, VirtualKey::KeyPadPlus) => Some('+'),
		(_, VirtualKey::KeyPad1) => Some('1'),
		(_, VirtualKey::KeyPad2) => Some('2'),
		(_, VirtualKey::KeyPad3) => Some('3'),
		(_, VirtualKey::KeyPad0) => Some('0'),
		(_, VirtualKey::KeyPadDot) => Some('.'),
		(_, VirtualKey::KeyPadEnter) => Some('\n'),
		(_, VirtualKey::KeyPadEqual) => Some('='),
		_ => None,
	}
}
