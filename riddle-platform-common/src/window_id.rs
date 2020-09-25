/// A platform system independent window identifier.
///
/// This allows other crates to reference windows without needing to
/// specify what platform crate is being used.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct WindowId {
    riddle_window_id: u32,
}

impl WindowId {
    pub fn new(id: u32) -> Self {
        Self {
            riddle_window_id: id,
        }
    }
}
