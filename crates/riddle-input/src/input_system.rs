use crate::*;

use riddle_common::eventpub::{EventPub, EventSub};
use riddle_window_common as window;
use window::{LogicalPosition, SystemEvent, WindowId};

use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
};

struct WindowMouseState {
    logical_position: LogicalPosition,
    pressed: bool,
}

struct WindowKeyboardState {
    key_states: [bool; 300],
}

struct WindowInputState {
    mouse: WindowMouseState,
    keyboard: WindowKeyboardState,
}

pub struct InputSystem {
    window_states: RefCell<HashMap<WindowId, WindowInputState>>,
    event_sub: EventSub<SystemEvent>,
}

impl InputSystem {
    pub fn new(sys_events: &EventPub<SystemEvent>) -> Result<Self, InputError> {
        let event_sub = EventSub::new_with_filter(Self::event_filter);
        sys_events.attach(&event_sub);

        Ok(InputSystem {
            window_states: RefCell::new(HashMap::new()),
            event_sub,
        })
    }

    pub fn update(&self) {
        for event in self.event_sub.collect() {
            match event {
                SystemEvent::Input(event) => match event {
                    window::InputEvent::CursorMove { window, position } => {
                        self.cursor_moved(window, position);
                    }
                    window::InputEvent::MouseButtonUp { window } => {
                        self.mouse_up(window);
                    }
                    window::InputEvent::MouseButtonDown { window } => {
                        self.mouse_down(window);
                    }
                    window::InputEvent::KeyUp { window, scancode } => {
                        self.key_up(window, scancode);
                    }
                    window::InputEvent::KeyDown { window, scancode } => {
                        self.key_down(window, scancode);
                    }
                },
                _ => (),
            }
        }
    }

    pub fn mouse_pos(&self, window: WindowId) -> LogicalPosition {
        let state = self.get_mouse_state(window);
        state.logical_position
    }

    fn get_window_state<'a>(&'a self, window: WindowId) -> RefMut<'a, WindowInputState> {
        let mut ms = self.window_states.borrow_mut();
        if !ms.contains_key(&window) {
            ms.insert(window, Default::default());
        }
        RefMut::map(ms, |ms| ms.get_mut(&window).unwrap())
    }

    fn get_mouse_state<'a>(&'a self, window: WindowId) -> RefMut<'a, WindowMouseState> {
        RefMut::map(self.get_window_state(window), |state| &mut state.mouse)
    }

    fn get_keyboard_state<'a>(&'a self, window: WindowId) -> RefMut<'a, WindowKeyboardState> {
        RefMut::map(self.get_window_state(window), |state| &mut state.keyboard)
    }

    fn cursor_moved(&self, window: WindowId, logical_position: LogicalPosition) {
        let mut state = self.get_mouse_state(window);
        state.logical_position = logical_position;
    }

    fn mouse_down(&self, window: WindowId) {
        let mut state = self.get_mouse_state(window);
        state.pressed = true;
    }

    fn mouse_up(&self, window: WindowId) {
        let mut state = self.get_mouse_state(window);
        state.pressed = false;
    }

    fn key_down(&self, window: WindowId, scancode: window::Scancode) {
        let mut state = self.get_keyboard_state(window);
        state.key_states[scancode as usize] = true;
    }

    fn key_up(&self, window: WindowId, scancode: window::Scancode) {
        let mut state = self.get_keyboard_state(window);
        state.key_states[scancode as usize] = false;
    }

    fn event_filter(event: &SystemEvent) -> bool {
        match event {
            SystemEvent::Input(_) => true,
            _ => false,
        }
    }
}

impl Default for WindowMouseState {
    fn default() -> Self {
        Self {
            logical_position: LogicalPosition { x: 0, y: 0 },
            pressed: false,
        }
    }
}

impl Default for WindowKeyboardState {
    fn default() -> Self {
        Self {
            key_states: [false; 300],
        }
    }
}

impl Default for WindowInputState {
    fn default() -> Self {
        Self {
            mouse: Default::default(),
            keyboard: Default::default(),
        }
    }
}
