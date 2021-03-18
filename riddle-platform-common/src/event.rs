use crate::*;

#[derive(Eq, PartialEq, Clone)]
pub enum PlatformEvent {
	WindowClose(WindowId),
	WindowResize(WindowId),

	CursorMove {
		window: WindowId,
		position: LogicalPosition,
	},
	MouseButtonDown {
		window: WindowId,
		button: MouseButton,
	},
	MouseButtonUp {
		window: WindowId,
		button: MouseButton,
	},
	KeyDown {
		window: WindowId,
		platform_scancode: u32,
		scancode: Scancode,
		vkey: Option<VirtualKey>,
	},
	KeyUp {
		window: WindowId,
		platform_scancode: u32,
		scancode: Scancode,
		vkey: Option<VirtualKey>,
	},

	EventQueueEmpty,
	Unknown,
}
