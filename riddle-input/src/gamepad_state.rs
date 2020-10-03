use crate::*;

use std::collections::{HashMap, HashSet};

pub(crate) struct GamePadState {
    buttons: HashSet<GamePadButton>,
    axis_values: HashMap<GamePadAxis, f32>,
}

impl GamePadState {
    pub fn new() -> Self {
        Self {
            buttons: HashSet::new(),
            axis_values: HashMap::new(),
        }
    }

    fn is_button_down(&self, button: GamePadButton) -> bool {
        self.buttons.contains(&button)
    }

    fn button_down(&mut self, button: GamePadButton) {
        self.buttons.insert(button);
    }

    fn button_up(&mut self, button: GamePadButton) {
        self.buttons.remove(&button);
    }

    fn axis_value(&self, axis: GamePadAxis) -> f32 {
        self.axis_values
            .get(&axis)
            .map(|v| v.clone())
            .unwrap_or_default()
    }

    fn set_axis_value(&mut self, axis: GamePadAxis, value: f32) {
        self.axis_values.insert(axis, value);
    }
}

pub(crate) struct GamePadStateMap {
    gamepads: HashMap<GamePadId, GamePadState>,
    last_active_pad: Option<GamePadId>,
}

impl GamePadStateMap {
    pub fn new() -> Self {
        Self {
            gamepads: HashMap::new(),
            last_active_pad: None,
        }
    }

    pub fn last_active_pad(&self) -> Option<GamePadId> {
        self.last_active_pad
    }

    pub fn is_button_down(&self, pad: GamePadId, button: GamePadButton) -> bool {
        if let Some(pad_state) = self.gamepads.get(&pad) {
            pad_state.is_button_down(button)
        } else {
            false
        }
    }

    pub fn button_down(&mut self, pad: GamePadId, button: GamePadButton) {
        self.last_active_pad = Some(pad);
        self.get_pad_mut(pad).button_down(button);
    }

    pub fn button_up(&mut self, pad: GamePadId, button: GamePadButton) {
        self.last_active_pad = Some(pad);
        self.get_pad_mut(pad).button_up(button);
    }

    pub fn axis_value(&self, pad: GamePadId, axis: GamePadAxis) -> f32 {
        self.gamepads
            .get(&pad)
            .map(|state| state.axis_value(axis))
            .unwrap_or_default()
    }

    pub fn set_axis_value(&mut self, pad: GamePadId, axis: GamePadAxis, value: f32) {
        self.last_active_pad = Some(pad);
        self.get_pad_mut(pad).set_axis_value(axis, value)
    }

    fn get_pad_mut(&mut self, pad: GamePadId) -> &mut GamePadState {
        if !self.gamepads.contains_key(&pad) {
            self.gamepads.insert(pad, GamePadState::new());
        }
        self.gamepads.get_mut(&pad).unwrap()
    }
}
