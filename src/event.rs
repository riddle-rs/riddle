use crate::*;

pub enum Event {
    PlatformEvent(platform::PlatformEvent),
    InputEvent(input::InputEvent),
    PreRunNullEvent,
    ProcessFrame,
}
