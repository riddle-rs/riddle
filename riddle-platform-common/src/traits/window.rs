use crate::*;

/// Functionality Window types need in order to be able to interact with platform common.
pub trait WindowExt {
    /// Convert a logical vec2 in to a physical pixel unit (x,y) pair
    fn logical_to_physical<L: Into<LogicalVec2>>(&self, vec2: L) -> (u32, u32);
}
