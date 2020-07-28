use crate::*;

use std::hash::Hash;

pub trait WindowId: Eq + PartialEq + Hash {}

pub trait Window {
    type Id: WindowId;

    fn logical_to_physical<L: Into<LogicalVec2>>(&self, vec2: L) -> (u32, u32);
    fn window_id(&self) -> Self::Id;
}

pub trait WindowHandle: Window {
    type Window: Window;
}

impl<T: Window> Window for std::rc::Rc<T> {
    type Id = T::Id;

    fn logical_to_physical<L: Into<LogicalVec2>>(&self, vec2: L) -> (u32, u32) {
        (**self).logical_to_physical(vec2)
    }

    fn window_id(&self) -> Self::Id {
        (**self).window_id()
    }
}

impl<T: Window + Clone> WindowHandle for T {
    type Window = T;
}
