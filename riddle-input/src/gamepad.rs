/// An identifier mapping to a particular gamepad, used for querying state from [`crate::InputSystem`]
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct GamePadId {
    id: gilrs::GamepadId,
}

impl From<gilrs::GamepadId> for GamePadId {
    fn from(id: gilrs::GamepadId) -> Self {
        Self { id }
    }
}

impl From<GamePadId> for gilrs::GamepadId {
    fn from(gpid: GamePadId) -> Self {
        gpid.id
    }
}

/// GamePad Button identifiers.
///
/// Assumes a layout similar to an Xbox controller, with the exception that
/// the A, B, X, and Y buttons have been renamed South, East, West, North respectively.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum GamePadButton {
    South,
    North,
    East,
    West,

    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,

    LeftStick,
    RightStick,

    LeftShoulder,
    RightShoulder,

    LeftTrigger,
    RightTrigger,

    Start,
    Back,
}

impl std::convert::TryFrom<gilrs::Button> for GamePadButton {
    type Error = gilrs::Button;

    fn try_from(button: gilrs::Button) -> Result<Self, gilrs::Button> {
        Ok(match button {
            gilrs::Button::South => GamePadButton::South,
            gilrs::Button::North => GamePadButton::North,
            gilrs::Button::East => GamePadButton::East,
            gilrs::Button::West => GamePadButton::West,

            gilrs::Button::DPadUp => GamePadButton::DPadUp,
            gilrs::Button::DPadDown => GamePadButton::DPadDown,
            gilrs::Button::DPadLeft => GamePadButton::DPadLeft,
            gilrs::Button::DPadRight => GamePadButton::DPadRight,

            gilrs::Button::LeftThumb => GamePadButton::LeftStick,
            gilrs::Button::RightThumb => GamePadButton::RightStick,

            gilrs::Button::LeftTrigger => GamePadButton::LeftShoulder,
            gilrs::Button::RightTrigger => GamePadButton::RightShoulder,

            gilrs::Button::LeftTrigger2 => GamePadButton::LeftTrigger,
            gilrs::Button::RightTrigger2 => GamePadButton::RightTrigger,

            gilrs::Button::Start => GamePadButton::Start,
            gilrs::Button::Select => GamePadButton::Back,

            _ => return Err(button),
        })
    }
}

impl From<GamePadButton> for gilrs::Button {
    fn from(button: GamePadButton) -> Self {
        match button {
            GamePadButton::South => gilrs::Button::South,
            GamePadButton::North => gilrs::Button::North,
            GamePadButton::East => gilrs::Button::East,
            GamePadButton::West => gilrs::Button::West,
            GamePadButton::DPadUp => gilrs::Button::DPadUp,
            GamePadButton::DPadDown => gilrs::Button::DPadDown,
            GamePadButton::DPadLeft => gilrs::Button::DPadLeft,
            GamePadButton::DPadRight => gilrs::Button::DPadRight,
            GamePadButton::LeftStick => gilrs::Button::LeftThumb,
            GamePadButton::RightStick => gilrs::Button::RightThumb,
            GamePadButton::LeftShoulder => gilrs::Button::LeftTrigger,
            GamePadButton::RightShoulder => gilrs::Button::RightTrigger,
            GamePadButton::LeftTrigger => gilrs::Button::LeftTrigger2,
            GamePadButton::RightTrigger => gilrs::Button::RightTrigger2,
            GamePadButton::Start => gilrs::Button::Start,
            GamePadButton::Back => gilrs::Button::Select,
        }
    }
}

/// GamePad axis identifiers.
///
/// Does not include an axis mapping for the DPad, that is exposed via button presses.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum GamePadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
}

impl std::convert::TryFrom<gilrs::Axis> for GamePadAxis {
    type Error = gilrs::Axis;

    fn try_from(axis: gilrs::Axis) -> Result<Self, gilrs::Axis> {
        Ok(match axis {
            gilrs::Axis::LeftStickX => GamePadAxis::LeftStickX,
            gilrs::Axis::LeftStickY => GamePadAxis::LeftStickY,
            gilrs::Axis::RightStickX => GamePadAxis::RightStickX,
            gilrs::Axis::RightStickY => GamePadAxis::RightStickY,
            _ => return Err(axis),
        })
    }
}
