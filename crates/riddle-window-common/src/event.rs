use crate::*;

#[derive(Eq, PartialEq, Clone)]
pub enum WindowEvent {
    WindowClose(WindowId),
    WindowResize(WindowId),
}

#[derive(Eq, PartialEq, Clone)]
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
    },
    KeyUp {
        window: WindowId,
        scancode: Scancode,
    },
}

#[derive(Eq, PartialEq, Clone)]
pub enum SystemEvent {
    Window(WindowEvent),
    Input(InputEvent),
    ProcessFrame,
    Unknown,
}

impl From<WindowEvent> for SystemEvent {
    fn from(e: WindowEvent) -> Self {
        SystemEvent::Window(e)
    }
}

impl From<InputEvent> for SystemEvent {
    fn from(e: InputEvent) -> Self {
        SystemEvent::Input(e)
    }
}
