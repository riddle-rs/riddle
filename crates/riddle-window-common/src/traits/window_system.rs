use crate::*;

use riddle_common::eventpub::EventPub;

pub trait WindowSystem {
    type WindowHandle: traits::WindowHandle;
    fn event_pub(&self) -> &EventPub<SystemEvent<Self::WindowHandle>>;
}
