use crate::*;

use std::collections::{HashMap, HashSet};

pub(crate) struct GamePadState {
    buttons: HashSet<GamePadButton>,
}

impl GamePadState {
    pub fn new() -> Self {
        Self {
            buttons: HashSet::new(),
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
}

pub(crate) struct GamePadStateMap {
    gamepads: HashMap<GamePadId, GamePadState>,
}

impl GamePadStateMap {
    pub fn new() -> Self {
        Self {
            gamepads: HashMap::new(),
        }
    }

    pub fn is_button_down(&self, pad: GamePadId, button: GamePadButton) -> bool {
        if let Some(pad_state) = self.gamepads.get(&pad) {
            pad_state.is_button_down(button)
        } else {
            false
        }
    }

    pub fn button_down(&mut self, pad: GamePadId, button: GamePadButton) {
        self.get_pad_mut(pad).button_down(button);
    }

    pub fn button_up(&mut self, pad: GamePadId, button: GamePadButton) {
        self.get_pad_mut(pad).button_up(button);
    }

    fn get_pad_mut(&mut self, pad: GamePadId) -> &mut GamePadState {
        if !self.gamepads.contains_key(&pad) {
            self.gamepads.insert(pad.clone(), GamePadState::new());
        }
        self.gamepads.get_mut(&pad).unwrap()
    }
}
