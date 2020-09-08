use crate::*;

use riddle_common::eventpub::{EventPub, EventSub};
use riddle_platform_common as window;
use window::{LogicalPosition, PlatformEvent, WindowId};

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

struct WindowMouseState {
    logical_position: LogicalPosition,
    pressed: bool,
}

struct WindowInputState {
    mouse: WindowMouseState,
    keyboard: KeyboardState,
}

pub struct InputSystem {
    window_states: RefCell<HashMap<WindowId, WindowInputState>>,
    event_sub: EventSub<PlatformEvent>,

    outgoing_input_events: RefCell<Vec<InputEvent>>,
}

impl InputSystem {
    pub fn new(sys_events: &EventPub<PlatformEvent>) -> Result<Self, InputError> {
        let event_sub = EventSub::new_with_filter(Self::event_filter);
        sys_events.attach(&event_sub);

        Ok(InputSystem {
            window_states: RefCell::new(HashMap::new()),
            event_sub,
            outgoing_input_events: RefCell::new(vec![]),
        })
    }

    pub fn update(&self) {
        for event in self.event_sub.collect() {
            match event {
                window::PlatformEvent::CursorMove { window, position } => {
                    self.cursor_moved(window, position);
                }
                window::PlatformEvent::MouseButtonUp { window } => {
                    self.mouse_up(window);
                }
                window::PlatformEvent::MouseButtonDown { window } => {
                    self.mouse_down(window);
                }
                window::PlatformEvent::KeyUp {
                    window,
                    scancode,
                    vkey,
                    ..
                } => {
                    self.key_up(window, scancode, vkey);
                }
                window::PlatformEvent::KeyDown {
                    window,
                    scancode,
                    vkey,
                    ..
                } => {
                    self.key_down(window, scancode, vkey);
                }
                _ => (),
            }
        }
    }

    pub fn take_input_events(&self) -> Vec<InputEvent> {
        std::mem::replace(&mut self.outgoing_input_events.borrow_mut(), vec![])
    }

    pub fn mouse_pos(&self, window: WindowId) -> LogicalPosition {
        let state = self.get_mouse_state(window);
        state.logical_position
    }

    pub fn keyboard_modifiers(&self, window: WindowId) -> KeyboardModifiers {
        let state = self.get_keyboard_state(window);
        state.modifiers()
    }

    fn get_window_state<'a>(&'a self, window: WindowId) -> Ref<'a, WindowInputState> {
        let mut ms = self.window_states.borrow_mut();
        if !ms.contains_key(&window) {
            ms.insert(window, Default::default());
        }
        let ms = self.window_states.borrow();
        Ref::map(ms, |ms| ms.get(&window).unwrap())
    }

    fn get_window_state_mut<'a>(&'a self, window: WindowId) -> RefMut<'a, WindowInputState> {
        let mut ms = self.window_states.borrow_mut();
        if !ms.contains_key(&window) {
            ms.insert(window, Default::default());
        }
        RefMut::map(ms, |ms| ms.get_mut(&window).unwrap())
    }

    fn get_mouse_state<'a>(&'a self, window: WindowId) -> RefMut<'a, WindowMouseState> {
        RefMut::map(self.get_window_state_mut(window), |state| &mut state.mouse)
    }

    fn get_keyboard_state_mut<'a>(&'a self, window: WindowId) -> RefMut<'a, KeyboardState> {
        RefMut::map(self.get_window_state_mut(window), |state| {
            &mut state.keyboard
        })
    }

    fn get_keyboard_state<'a>(&'a self, window: WindowId) -> Ref<'a, KeyboardState> {
        Ref::map(self.get_window_state(window), |state| &state.keyboard)
    }

    fn cursor_moved(&self, window: WindowId, logical_position: LogicalPosition) {
        let mut state = self.get_mouse_state(window);
        state.logical_position = logical_position;
        self.send_input_event(InputEvent::CursorMove {
            window,
            position: logical_position,
        });
    }

    fn mouse_down(&self, window: WindowId) {
        let mut state = self.get_mouse_state(window);
        state.pressed = true;
    }

    fn mouse_up(&self, window: WindowId) {
        let mut state = self.get_mouse_state(window);
        state.pressed = false;
    }

    fn key_down(&self, window: WindowId, scancode: Scancode, vkey: Option<VirtualKey>) {
        let mut state = self.get_keyboard_state_mut(window);
        state.key_down(scancode, vkey);
        self.send_input_event(InputEvent::KeyDown {
            window,
            scancode,
            vkey,
            modifiers: state.modifiers(),
        })
    }

    fn key_up(&self, window: WindowId, scancode: Scancode, vkey: Option<VirtualKey>) {
        let mut state = self.get_keyboard_state_mut(window);
        state.key_up(scancode, vkey);
        self.send_input_event(InputEvent::KeyUp {
            window,
            scancode,
            vkey,
            modifiers: state.modifiers(),
        })
    }

    fn event_filter(event: &PlatformEvent) -> bool {
        match event {
            window::PlatformEvent::CursorMove { .. } => true,
            window::PlatformEvent::MouseButtonUp { .. } => true,
            window::PlatformEvent::MouseButtonDown { .. } => true,
            window::PlatformEvent::KeyUp { .. } => true,
            window::PlatformEvent::KeyDown { .. } => true,
            _ => false,
        }
    }

    fn send_input_event(&self, event: InputEvent) {
        self.outgoing_input_events.borrow_mut().push(event);
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

impl Default for WindowInputState {
    fn default() -> Self {
        Self {
            mouse: Default::default(),
            keyboard: Default::default(),
        }
    }
}
