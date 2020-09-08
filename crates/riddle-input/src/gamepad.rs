#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GamePadId {
    id: gilrs::GamepadId,
}

impl From<gilrs::GamepadId> for GamePadId {
    fn from(id: gilrs::GamepadId) -> Self {
        Self { id }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GamePadButton {
    South,
    North,
    East,
    West,
}

impl std::convert::TryFrom<gilrs::Button> for GamePadButton {
    type Error = gilrs::Button;

    fn try_from(button: gilrs::Button) -> Result<Self, gilrs::Button> {
        Ok(match button {
            gilrs::Button::South => GamePadButton::South,
            gilrs::Button::North => GamePadButton::North,
            gilrs::Button::East => GamePadButton::East,
            gilrs::Button::West => GamePadButton::West,
            _ => Err(button)?,
        })
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum GamePadAxis {
    LeftStickX,
    LeftStickY,
}

impl std::convert::TryFrom<gilrs::Axis> for GamePadAxis {
    type Error = gilrs::Axis;

    fn try_from(axis: gilrs::Axis) -> Result<Self, gilrs::Axis> {
        Ok(match axis {
            gilrs::Axis::LeftStickX => GamePadAxis::LeftStickX,
            gilrs::Axis::LeftStickY => GamePadAxis::LeftStickY,
            _ => Err(axis)?,
        })
    }
}
