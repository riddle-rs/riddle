#![feature(arc_new_cyclic)]

mod audio_system;
mod clip;
mod clip_player;
mod error;

use riddle_common::*;

pub use audio_system::*;
pub use clip::*;
pub use clip_player::*;
pub use error::*;

type Result<R> = std::result::Result<R, AudioError>;
