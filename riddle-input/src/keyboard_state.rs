use crate::*;

pub(crate) struct KeyboardState {
    scankeys: [bool; 300],
    vkeys: [bool; 300],
}

impl KeyboardState {
    pub fn modifiers(&self) -> KeyboardModifiers {
        KeyboardModifiers {
            shift: self.vkeys[VirtualKey::LeftShift as usize]
                || self.vkeys[VirtualKey::RightShift as usize],
            ctrl: self.vkeys[VirtualKey::LeftControl as usize],
            alt: self.vkeys[VirtualKey::LeftAlt as usize]
                || self.vkeys[VirtualKey::RightAlt as usize],
        }
    }

    pub fn key_down(&mut self, scancode: Scancode, vkey: Option<VirtualKey>) {
        self.scankeys[scancode as usize] = true;
        if let Some(vkey) = vkey {
            self.vkeys[vkey as usize] = true;
        }
    }

    pub fn key_up(&mut self, scancode: Scancode, vkey: Option<VirtualKey>) {
        self.scankeys[scancode as usize] = false;
        if let Some(vkey) = vkey {
            self.vkeys[vkey as usize] = false;
        }
    }
}

impl Default for KeyboardState {
    fn default() -> Self {
        KeyboardState {
            scankeys: [false; 300],
            vkeys: [false; 300],
        }
    }
}

/// A snapshot of which keyboard modifiers are currently pressed.
///
/// Mostly used to provide context for [`InputEvent`] keyboard events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyboardModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
}
