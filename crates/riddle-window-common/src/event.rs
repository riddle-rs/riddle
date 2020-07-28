use crate::{traits::WindowHandle, *};

#[derive(Eq, PartialEq, Clone)]
pub enum WindowEvent<W: WindowHandle> {
    WindowClose(W),
    WindowResize(W),
}

#[derive(Eq, PartialEq, Clone)]
pub enum InputEvent<W: WindowHandle> {
    CursorMove {
        window: W,
        position: LogicalPosition,
    },
    MouseButtonDown {
        window: W,
    },
    MouseButtonUp {
        window: W,
    },
    KeyDown {
        window: W,
        scancode: Scancode,
    },
    KeyUp {
        window: W,
        scancode: Scancode,
    },
}

#[derive(Eq, PartialEq, Clone)]
pub enum SystemEvent<W: WindowHandle> {
    Window(WindowEvent<W>),
    Input(InputEvent<W>),
    ProcessFrame,
    Unknown,
}

impl<W: WindowHandle> From<WindowEvent<W>> for SystemEvent<W> {
    fn from(e: WindowEvent<W>) -> Self {
        SystemEvent::Window(e)
    }
}

impl<W: WindowHandle> From<InputEvent<W>> for SystemEvent<W> {
    fn from(e: InputEvent<W>) -> Self {
        SystemEvent::Input(e)
    }
}
