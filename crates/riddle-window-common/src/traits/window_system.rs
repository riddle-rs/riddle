use crate::*;

use riddle_common::eventpub::EventPub;

pub trait WindowSystem {
    fn event_pub(&self) -> &EventPub<SystemEvent>;
}
