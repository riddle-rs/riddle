use crate::*;

use riddle_common::eventpub::{EventPub, EventSub};
use riddle_platform_common::{LogicalPosition, PlatformEvent, WindowId};

use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

struct WindowInputState {
    mouse: MouseState,
    keyboard: KeyboardState,
}

pub struct InputSystem {
    window_states: RefCell<HashMap<WindowId, WindowInputState>>,
    event_sub: EventSub<PlatformEvent>,

    gilrs: RefCell<gilrs::Gilrs>,

    outgoing_input_events: RefCell<Vec<InputEvent>>,
}

impl InputSystem {
    pub fn new(sys_events: &EventPub<PlatformEvent>) -> Result<Self, InputError> {
        let event_sub = EventSub::new_with_filter(Self::event_filter);
        sys_events.attach(&event_sub);

        let gilrs = gilrs::Gilrs::new().map_err(|_| InputError::Unknown)?;

        Ok(InputSystem {
            window_states: RefCell::new(HashMap::new()),
            event_sub,
            gilrs: RefCell::new(gilrs),
            outgoing_input_events: RefCell::new(vec![]),
        })
    }

    pub fn update(&self) {
        self.process_platform_events();
        self.process_gilrs_events();
    }

    pub fn take_input_events(&self) -> Vec<InputEvent> {
        std::mem::replace(&mut self.outgoing_input_events.borrow_mut(), vec![])
    }

    pub fn mouse_pos(&self, window: WindowId) -> LogicalPosition {
        let state = self.get_mouse_state(window);
        state.position()
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

    fn get_mouse_state<'a>(&'a self, window: WindowId) -> RefMut<'a, MouseState> {
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
        state.set_position(logical_position);
        self.send_input_event(InputEvent::CursorMove {
            window,
            position: logical_position,
        });
    }

    fn mouse_down(&self, window: WindowId, button: MouseButton) {
        let mut state = self.get_mouse_state(window);
        state.button_down(button);
        self.send_input_event(InputEvent::MouseButtonDown { window, button });
    }

    fn mouse_up(&self, window: WindowId, button: MouseButton) {
        let mut state = self.get_mouse_state(window);
        state.button_up(button);
        self.send_input_event(InputEvent::MouseButtonUp { window, button });
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
            PlatformEvent::CursorMove { .. } => true,
            PlatformEvent::MouseButtonUp { .. } => true,
            PlatformEvent::MouseButtonDown { .. } => true,
            PlatformEvent::KeyUp { .. } => true,
            PlatformEvent::KeyDown { .. } => true,
            _ => false,
        }
    }

    fn process_platform_events(&self) {
        for event in self.event_sub.collect() {
            match event {
                PlatformEvent::CursorMove { window, position } => {
                    self.cursor_moved(window, position);
                }
                PlatformEvent::MouseButtonUp { window, button } => {
                    self.mouse_up(window, button);
                }
                PlatformEvent::MouseButtonDown { window, button } => {
                    self.mouse_down(window, button);
                }
                PlatformEvent::KeyUp {
                    window,
                    scancode,
                    vkey,
                    ..
                } => {
                    self.key_up(window, scancode, vkey);
                }
                PlatformEvent::KeyDown {
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

    fn process_gilrs_events(&self) {
        while let Some(gilrs::Event { event, id, .. }) = self.gilrs.borrow_mut().next_event() {
            use std::convert::TryFrom;
            match event {
                gilrs::EventType::ButtonPressed(button, _) => {
                    if let Ok(button) = GamePadButton::try_from(button) {
                        self.send_input_event(InputEvent::GamePadButtonDown {
                            gamepad: id.into(),
                            button,
                        });
                    }
                }
                gilrs::EventType::ButtonRepeated(_, _) => {}
                gilrs::EventType::ButtonReleased(button, _) => {
                    if let Ok(button) = GamePadButton::try_from(button) {
                        self.send_input_event(InputEvent::GamePadButtonUp {
                            gamepad: id.into(),
                            button,
                        });
                    }
                }
                gilrs::EventType::ButtonChanged(_, _, _) => {}
                gilrs::EventType::AxisChanged(axis, value, _) => {
                    if let Ok(axis) = GamePadAxis::try_from(axis) {
                        self.send_input_event(InputEvent::GamePadAxisChanged {
                            gamepad: id.into(),
                            axis,
                            value,
                        });
                    }
                }
                gilrs::EventType::Connected => {
                    self.send_input_event(InputEvent::GamePadConnected(id.into()));
                }
                gilrs::EventType::Disconnected => {
                    self.send_input_event(InputEvent::GamePadDisconnected(id.into()));
                }
                _ => {}
            }
        }
    }

    fn send_input_event(&self, event: InputEvent) {
        self.outgoing_input_events.borrow_mut().push(event);
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
