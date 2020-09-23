use crate::*;

pub enum Event {
    Platform(platform::PlatformEvent),
    Input(input::InputEvent),
    PreRunPlaceholder,
    ProcessFrame,
}
