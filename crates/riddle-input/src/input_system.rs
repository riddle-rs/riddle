use crate::*;

use riddle_common::eventpub::{EventPub, EventSub};
use riddle_window_common::{traits::*, *};

use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    hash::Hash,
};

struct InputStateKey<Id: WindowId> {
    window_id: Id,
}

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

pub struct InputSystem<W: WindowHandle> {
    window_states: RefCell<HashMap<W::Id, WindowInputState>>,
    event_sub: EventSub<SystemEvent<W>>,
}

impl<W> InputSystem<W>
where
    W: Window + Clone + 'static,
{
    pub fn new(sys_events: &EventPub<SystemEvent<W>>) -> Result<Self, InputError> {
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
                    InputEvent::CursorMove { window, position } => {
                        self.cursor_moved(&window, position);
                    }
                    InputEvent::MouseButtonUp { window } => {
                        self.mouse_up(&window);
                    }
                    InputEvent::MouseButtonDown { window } => {
                        self.mouse_down(&window);
                    }
                    InputEvent::KeyUp { window, scancode } => {
                        self.key_up(&window, scancode);
                    }
                    InputEvent::KeyDown { window, scancode } => {
                        self.key_down(&window, scancode);
                    }
                },
                _ => (),
            }
        }
    }

    pub fn mouse_pos(&self, window: &W) -> LogicalPosition {
        let state = self.get_mouse_state(window);
        state.logical_position
    }

    fn get_window_state<'a>(&'a self, window: &W) -> RefMut<'a, WindowInputState> {
        let mut ms = self.window_states.borrow_mut();
        if !ms.contains_key(&window.window_id()) {
            ms.insert(window.window_id(), Default::default());
        }
        RefMut::map(ms, |ms| ms.get_mut(&window.window_id()).unwrap())
    }

    fn get_mouse_state<'a>(&'a self, window: &W) -> RefMut<'a, WindowMouseState> {
        RefMut::map(self.get_window_state(window), |state| &mut state.mouse)
    }

    fn get_keyboard_state<'a>(&'a self, window: &W) -> RefMut<'a, WindowKeyboardState> {
        RefMut::map(self.get_window_state(window), |state| &mut state.keyboard)
    }

    fn cursor_moved(&self, window: &W, logical_position: LogicalPosition) {
        let mut state = self.get_mouse_state(&window);
        state.logical_position = logical_position;
    }

    fn mouse_down(&self, window: &W) {
        let mut state = self.get_mouse_state(&window);
        state.pressed = true;
    }

    fn mouse_up(&self, window: &W) {
        let mut state = self.get_mouse_state(&window);
        state.pressed = false;
    }

    fn key_down(&self, window: &W, scancode: Scancode) {
        let mut state = self.get_keyboard_state(&window);
        state.key_states[scancode as usize] = true;
    }

    fn key_up(&self, window: &W, scancode: Scancode) {
        let mut state = self.get_keyboard_state(&window);
        state.key_states[scancode as usize] = false;
    }

    fn event_filter(event: &SystemEvent<W>) -> bool {
        match event {
            SystemEvent::Input(_) => true,
            _ => false,
        }
    }
}

impl<Id: WindowId> PartialEq for InputStateKey<Id> {
    fn eq(&self, other: &Self) -> bool {
        self.window_id == other.window_id
    }
}

impl<Id: WindowId> Eq for InputStateKey<Id> {}

impl<Id: WindowId> Hash for InputStateKey<Id> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Id::hash(&self.window_id, state);
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
