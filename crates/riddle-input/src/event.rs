use crate::*;

use riddle_platform_common::{LogicalPosition, WindowId};

pub enum InputEvent {
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
        scancode: Scancode,
        vkey: Option<VirtualKey>,
        modifiers: KeyboardModifiers,
    },
    KeyUp {
        window: WindowId,
        scancode: Scancode,
        vkey: Option<VirtualKey>,
        modifiers: KeyboardModifiers,
    },
}
