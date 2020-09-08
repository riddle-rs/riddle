use crate::*;

use riddle_platform_common::{LogicalPosition, WindowId};

pub enum InputEvent {
    CursorMove {
        window: WindowId,
        position: LogicalPosition,
    },
    MouseButtonDown {
        window: WindowId,
    },
    MouseButtonUp {
        window: WindowId,
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
