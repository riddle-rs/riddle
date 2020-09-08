use crate::*;

pub trait WindowExt {
    fn logical_to_physical<L: Into<LogicalVec2>>(&self, vec2: L) -> (u32, u32);
}

impl<T: WindowExt> WindowExt for std::rc::Rc<T> {
    fn logical_to_physical<L: Into<LogicalVec2>>(&self, vec2: L) -> (u32, u32) {
        (**self).logical_to_physical(vec2)
    }
}
