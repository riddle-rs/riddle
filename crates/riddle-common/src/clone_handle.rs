use std::rc::{Rc, Weak};

pub trait CloneHandle {
    #[inline]
    fn clone_handle(&self) -> Option<Rc<Self>> {
        std::rc::Weak::upgrade(&self.clone_weak_handle())
    }

    fn clone_weak_handle(&self) -> Weak<Self>;
}
