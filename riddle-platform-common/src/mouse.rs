/// Mouse button identifiers.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u32),
}
