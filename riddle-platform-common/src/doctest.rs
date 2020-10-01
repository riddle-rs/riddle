//! **DO NOT RELY ON THIS MODULE**
//!
//! **TEST CODE ONLY**
//!
//! This module contains utilities for use during doctest builds. To be removed from the public API when
//! [rust#67295](https://github.com/rust-lang/rust/pull/67295) is resolved.

use crate::*;

pub struct MockWindow {}

impl traits::WindowExt for MockWindow {
    fn logical_to_physical<L: Into<LogicalVec2>>(&self, vec2: L) -> (u32, u32) {
        let v = vec2.into();
        (v.x * 2, v.y * 2)
    }
}
