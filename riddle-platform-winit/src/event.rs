use crate::{common::*, *};

#[derive(Debug)]
pub(crate) enum InternalEvent {
	QuitRequested,
}

pub(crate) fn convert_winit_event(
	system: &PlatformSystem,
	event: winit::event::Event<InternalEvent>,
) -> Option<PlatformEvent> {
	match event {
		winit::event::Event::WindowEvent { window_id, event } => {
			let window = system.with_window_map(|wmap| wmap.lookup_winit_window(window_id));
			window.and_then(|window| convert_winit_window_event(&window, event))
		}
		winit::event::Event::MainEventsCleared => PlatformEvent::EventQueueEmpty.into(),
		_ => None,
	}
}

fn convert_winit_window_event(
	window: &Window,
	event: winit::event::WindowEvent,
) -> Option<PlatformEvent> {
	match event {
		winit::event::WindowEvent::CloseRequested => Some(PlatformEvent::WindowClose(window.id())),
		winit::event::WindowEvent::Resized(_) => Some(PlatformEvent::WindowResize(window.id())),
		winit::event::WindowEvent::CursorMoved { position, .. } => {
			Some(PlatformEvent::CursorMove {
				window: window.id(),
				position: dimensions::logical_pos_from_winit(
					position.to_logical(window.scale_factor()),
				),
			})
		}
		winit::event::WindowEvent::MouseInput { state, button, .. } => match state {
			winit::event::ElementState::Pressed => Some(PlatformEvent::MouseButtonDown {
				window: window.id(),
				button: winit_mousebutton_to_mousebutton(button),
			}),
			winit::event::ElementState::Released => Some(PlatformEvent::MouseButtonUp {
				window: window.id(),
				button: winit_mousebutton_to_mousebutton(button),
			}),
		},
		winit::event::WindowEvent::KeyboardInput {
			input:
				winit::event::KeyboardInput {
					scancode,
					state,
					virtual_keycode,
					..
				},
			..
		} => match state {
			winit::event::ElementState::Pressed => Some(PlatformEvent::KeyDown {
				window: window.id(),
				platform_scancode: scancode,
				scancode: scancode.into(),
				vkey: virtual_keycode.and_then(winit_vkey_to_vkey),
			}),
			winit::event::ElementState::Released => Some(PlatformEvent::KeyUp {
				window: window.id(),
				platform_scancode: scancode,
				scancode: scancode.into(),
				vkey: virtual_keycode.and_then(winit_vkey_to_vkey),
			}),
		},
		winit::event::WindowEvent::ReceivedCharacter(c) => Some(PlatformEvent::TextInput {
			window: window.id(),
			text: c.into(),
		}),
		_ => None,
	}
}

pub(crate) fn winit_vkey_to_vkey(vk: winit::event::VirtualKeyCode) -> Option<VirtualKey> {
	match vk {
		winit::event::VirtualKeyCode::Key1 => Some(VirtualKey::One),
		winit::event::VirtualKeyCode::Key2 => Some(VirtualKey::Two),
		winit::event::VirtualKeyCode::Key3 => Some(VirtualKey::Three),
		winit::event::VirtualKeyCode::Key4 => Some(VirtualKey::Four),
		winit::event::VirtualKeyCode::Key5 => Some(VirtualKey::Five),
		winit::event::VirtualKeyCode::Key6 => Some(VirtualKey::Six),
		winit::event::VirtualKeyCode::Key7 => Some(VirtualKey::Seven),
		winit::event::VirtualKeyCode::Key8 => Some(VirtualKey::Eight),
		winit::event::VirtualKeyCode::Key9 => Some(VirtualKey::Nine),
		winit::event::VirtualKeyCode::Key0 => Some(VirtualKey::Zero),
		winit::event::VirtualKeyCode::A => Some(VirtualKey::A),
		winit::event::VirtualKeyCode::B => Some(VirtualKey::B),
		winit::event::VirtualKeyCode::C => Some(VirtualKey::C),
		winit::event::VirtualKeyCode::D => Some(VirtualKey::D),
		winit::event::VirtualKeyCode::E => Some(VirtualKey::E),
		winit::event::VirtualKeyCode::F => Some(VirtualKey::F),
		winit::event::VirtualKeyCode::G => Some(VirtualKey::G),
		winit::event::VirtualKeyCode::H => Some(VirtualKey::H),
		winit::event::VirtualKeyCode::I => Some(VirtualKey::I),
		winit::event::VirtualKeyCode::J => Some(VirtualKey::J),
		winit::event::VirtualKeyCode::K => Some(VirtualKey::K),
		winit::event::VirtualKeyCode::L => Some(VirtualKey::L),
		winit::event::VirtualKeyCode::M => Some(VirtualKey::M),
		winit::event::VirtualKeyCode::N => Some(VirtualKey::N),
		winit::event::VirtualKeyCode::O => Some(VirtualKey::O),
		winit::event::VirtualKeyCode::P => Some(VirtualKey::P),
		winit::event::VirtualKeyCode::Q => Some(VirtualKey::Q),
		winit::event::VirtualKeyCode::R => Some(VirtualKey::R),
		winit::event::VirtualKeyCode::S => Some(VirtualKey::S),
		winit::event::VirtualKeyCode::T => Some(VirtualKey::T),
		winit::event::VirtualKeyCode::U => Some(VirtualKey::U),
		winit::event::VirtualKeyCode::V => Some(VirtualKey::V),
		winit::event::VirtualKeyCode::W => Some(VirtualKey::W),
		winit::event::VirtualKeyCode::X => Some(VirtualKey::X),
		winit::event::VirtualKeyCode::Y => Some(VirtualKey::Y),
		winit::event::VirtualKeyCode::Z => Some(VirtualKey::Z),
		winit::event::VirtualKeyCode::Escape => Some(VirtualKey::Escape),
		winit::event::VirtualKeyCode::F1 => Some(VirtualKey::F1),
		winit::event::VirtualKeyCode::F2 => Some(VirtualKey::F2),
		winit::event::VirtualKeyCode::F3 => Some(VirtualKey::F3),
		winit::event::VirtualKeyCode::F4 => Some(VirtualKey::F4),
		winit::event::VirtualKeyCode::F5 => Some(VirtualKey::F5),
		winit::event::VirtualKeyCode::F6 => Some(VirtualKey::F6),
		winit::event::VirtualKeyCode::F7 => Some(VirtualKey::F7),
		winit::event::VirtualKeyCode::F8 => Some(VirtualKey::F8),
		winit::event::VirtualKeyCode::F9 => Some(VirtualKey::F9),
		winit::event::VirtualKeyCode::F10 => Some(VirtualKey::F10),
		winit::event::VirtualKeyCode::F11 => Some(VirtualKey::F11),
		winit::event::VirtualKeyCode::F12 => Some(VirtualKey::F12),
		winit::event::VirtualKeyCode::Scroll => Some(VirtualKey::ScrollLock),
		winit::event::VirtualKeyCode::Insert => Some(VirtualKey::Insert),
		winit::event::VirtualKeyCode::Home => Some(VirtualKey::Home),
		winit::event::VirtualKeyCode::Delete => Some(VirtualKey::Delete),
		winit::event::VirtualKeyCode::End => Some(VirtualKey::End),
		winit::event::VirtualKeyCode::PageDown => Some(VirtualKey::PageDown),
		winit::event::VirtualKeyCode::PageUp => Some(VirtualKey::PageUp),
		winit::event::VirtualKeyCode::Left => Some(VirtualKey::Left),
		winit::event::VirtualKeyCode::Up => Some(VirtualKey::Up),
		winit::event::VirtualKeyCode::Right => Some(VirtualKey::Right),
		winit::event::VirtualKeyCode::Down => Some(VirtualKey::Down),
		winit::event::VirtualKeyCode::Back => Some(VirtualKey::Backspace),
		winit::event::VirtualKeyCode::Return => Some(VirtualKey::Enter),
		winit::event::VirtualKeyCode::Space => Some(VirtualKey::Space),
		winit::event::VirtualKeyCode::Numlock => Some(VirtualKey::NumLock),
		winit::event::VirtualKeyCode::Numpad0 => Some(VirtualKey::KeyPad0),
		winit::event::VirtualKeyCode::Numpad1 => Some(VirtualKey::KeyPad1),
		winit::event::VirtualKeyCode::Numpad2 => Some(VirtualKey::KeyPad2),
		winit::event::VirtualKeyCode::Numpad3 => Some(VirtualKey::KeyPad3),
		winit::event::VirtualKeyCode::Numpad4 => Some(VirtualKey::KeyPad4),
		winit::event::VirtualKeyCode::Numpad5 => Some(VirtualKey::KeyPad5),
		winit::event::VirtualKeyCode::Numpad6 => Some(VirtualKey::KeyPad6),
		winit::event::VirtualKeyCode::Numpad7 => Some(VirtualKey::KeyPad7),
		winit::event::VirtualKeyCode::Numpad8 => Some(VirtualKey::KeyPad8),
		winit::event::VirtualKeyCode::Numpad9 => Some(VirtualKey::KeyPad9),
		winit::event::VirtualKeyCode::NumpadEnter => Some(VirtualKey::KeyPadEnter),
		winit::event::VirtualKeyCode::NumpadEquals => Some(VirtualKey::KeyPadEqual),
		winit::event::VirtualKeyCode::NumpadSubtract => Some(VirtualKey::KeyPadMinus),
		winit::event::VirtualKeyCode::LShift => Some(VirtualKey::LeftShift),
		winit::event::VirtualKeyCode::RShift => Some(VirtualKey::RightShift),
		winit::event::VirtualKeyCode::LControl => Some(VirtualKey::LeftControl),
		winit::event::VirtualKeyCode::LBracket => Some(VirtualKey::LeftBrace),
		winit::event::VirtualKeyCode::RBracket => Some(VirtualKey::RightBrace),
		winit::event::VirtualKeyCode::Minus => Some(VirtualKey::Minus),
		winit::event::VirtualKeyCode::Equals => Some(VirtualKey::Equal),
		winit::event::VirtualKeyCode::Semicolon => Some(VirtualKey::Semicolon),
		winit::event::VirtualKeyCode::Apostrophe => Some(VirtualKey::Apostrope),
		winit::event::VirtualKeyCode::Grave => Some(VirtualKey::Grave),
		winit::event::VirtualKeyCode::Comma => Some(VirtualKey::Comma),
		winit::event::VirtualKeyCode::Period => Some(VirtualKey::Dot),
		winit::event::VirtualKeyCode::Slash => Some(VirtualKey::Slash),
		_ => None,
	}
}

fn winit_mousebutton_to_mousebutton(button: winit::event::MouseButton) -> MouseButton {
	match button {
		winit::event::MouseButton::Left => MouseButton::Left,
		winit::event::MouseButton::Right => MouseButton::Right,
		winit::event::MouseButton::Middle => MouseButton::Middle,
		winit::event::MouseButton::Other(b) => MouseButton::Other(b as u32),
	}
}
