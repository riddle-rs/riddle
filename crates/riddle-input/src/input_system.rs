use crate::*;

use riddle_common::{
    define_handles,
    eventpub::{EventPub, EventSub},
};
use riddle_platform_common::{LogicalPosition, PlatformEvent, WindowId};

use std::collections::HashMap;

struct WindowInputState {
    mouse: MouseState,
    keyboard: KeyboardState,
}

pub struct InputSystem {
    weak_self: InputSystemWeak,

    window_states: std::sync::Mutex<HashMap<WindowId, WindowInputState>>,
    gamepad_states: std::sync::Mutex<GamePadStateMap>,

    event_sub: EventSub<PlatformEvent>,

    outgoing_input_events: std::sync::Mutex<Vec<InputEvent>>,
}

define_handles!(<InputSystem>::weak_self, pub InputSystemHandle, pub InputSystemWeak);

impl InputSystem {
    pub fn new(
        sys_events: &EventPub<PlatformEvent>,
    ) -> Result<(InputSystemHandle, InputMainThreadState), InputError> {
        let event_sub = EventSub::new_with_filter(Self::event_filter);
        sys_events.attach(&event_sub);

        let gilrs = gilrs::Gilrs::new().map_err(|_| InputError::Unknown)?;

        let system = InputSystemHandle::new(|weak_self| InputSystem {
            weak_self,
            window_states: std::sync::Mutex::new(HashMap::new()),
            gamepad_states: std::sync::Mutex::new(GamePadStateMap::new()),
            event_sub,
            outgoing_input_events: std::sync::Mutex::new(vec![]),
        });

        let main_thread = InputMainThreadState {
            system: system.clone(),
            gilrs,
        };

        Ok((system, main_thread))
    }

    pub fn update(&self) {
        self.process_platform_events();
    }

    pub fn take_input_events(&self) -> Vec<InputEvent> {
        std::mem::replace(&mut self.outgoing_input_events.lock().unwrap(), vec![])
    }

    pub fn mouse_pos(&self, window: WindowId) -> LogicalPosition {
        self.with_window_state(window, |w| w.mouse.position())
    }

    pub fn keyboard_modifiers(&self, window: WindowId) -> KeyboardModifiers {
        self.with_window_state(window, |w| w.keyboard.modifiers())
    }

    pub fn is_gamepad_button_down(&self, gamepad: GamePadId, button: GamePadButton) -> bool {
        self.gamepad_states
            .lock()
            .unwrap()
            .is_button_down(gamepad, button)
    }

    fn with_window_state<'a, R, F>(&'a self, window: WindowId, f: F) -> R
    where
        F: FnOnce(&WindowInputState) -> R,
    {
        let mut ms = self.window_states.lock().unwrap();
        if !ms.contains_key(&window) {
            ms.insert(window, Default::default());
        }
        f(ms.get(&window).unwrap())
    }

    fn with_window_state_mut<'a, R, F>(&'a self, window: WindowId, f: F) -> R
    where
        F: FnOnce(&mut WindowInputState) -> R,
    {
        let mut ms = self.window_states.lock().unwrap();
        if !ms.contains_key(&window) {
            ms.insert(window, Default::default());
        }
        f(ms.get_mut(&window).unwrap())
    }

    fn cursor_moved(&self, window: WindowId, logical_position: LogicalPosition) {
        self.with_window_state_mut(window, |w| w.mouse.set_position(logical_position));
        self.send_input_event(InputEvent::CursorMove {
            window,
            position: logical_position,
        });
    }

    fn mouse_down(&self, window: WindowId, button: MouseButton) {
        self.with_window_state_mut(window, |w| w.mouse.button_down(button));
        self.send_input_event(InputEvent::MouseButtonDown { window, button });
    }

    fn mouse_up(&self, window: WindowId, button: MouseButton) {
        self.with_window_state_mut(window, |w| w.mouse.button_up(button));
        self.send_input_event(InputEvent::MouseButtonUp { window, button });
    }

    fn key_down(&self, window: WindowId, scancode: Scancode, vkey: Option<VirtualKey>) {
        let modifiers = self.with_window_state_mut(window, |w| {
            w.keyboard.key_down(scancode, vkey);
            w.keyboard.modifiers()
        });
        self.send_input_event(InputEvent::KeyDown {
            window,
            scancode,
            vkey,
            modifiers,
        })
    }

    fn key_up(&self, window: WindowId, scancode: Scancode, vkey: Option<VirtualKey>) {
        let modifiers = self.with_window_state_mut(window, |w| {
            w.keyboard.key_up(scancode, vkey);
            w.keyboard.modifiers()
        });
        self.send_input_event(InputEvent::KeyUp {
            window,
            scancode,
            vkey,
            modifiers,
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

    fn send_input_event(&self, event: InputEvent) {
        self.outgoing_input_events.lock().unwrap().push(event);
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

pub struct InputMainThreadState {
    system: InputSystemHandle,
    gilrs: gilrs::Gilrs,
}

impl InputMainThreadState {
    pub fn update(&mut self) {
        while let Some(gilrs::Event { event, id, .. }) = self.gilrs.next_event() {
            use std::convert::TryFrom;
            match event {
                gilrs::EventType::ButtonPressed(button, _) => {
                    if let Ok(button) = GamePadButton::try_from(button) {
                        self.system
                            .gamepad_states
                            .lock()
                            .unwrap()
                            .button_down(id.into(), button.clone());
                        self.system.send_input_event(InputEvent::GamePadButtonDown {
                            gamepad: id.into(),
                            button,
                        });
                    }
                }
                gilrs::EventType::ButtonRepeated(_, _) => {}
                gilrs::EventType::ButtonReleased(button, _) => {
                    if let Ok(button) = GamePadButton::try_from(button) {
                        self.system
                            .gamepad_states
                            .lock()
                            .unwrap()
                            .button_up(id.into(), button.clone());
                        self.system.send_input_event(InputEvent::GamePadButtonUp {
                            gamepad: id.into(),
                            button,
                        });
                    }
                }
                gilrs::EventType::ButtonChanged(_, _, _) => {}
                gilrs::EventType::AxisChanged(axis, value, _) => {
                    if let Ok(axis) = GamePadAxis::try_from(axis) {
                        self.system
                            .send_input_event(InputEvent::GamePadAxisChanged {
                                gamepad: id.into(),
                                axis,
                                value,
                            });
                    }
                }
                gilrs::EventType::Connected => {
                    self.system
                        .send_input_event(InputEvent::GamePadConnected(id.into()));
                }
                gilrs::EventType::Disconnected => {
                    self.system
                        .send_input_event(InputEvent::GamePadDisconnected(id.into()));
                }
                _ => {}
            }
        }

        self.system.update();
    }
}
