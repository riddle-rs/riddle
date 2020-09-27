/*!

**DO NOT RELY ON THIS MODULE**

**TEST CODE ONLY**

This module contains utilities for use during doctest builds. To be removed from the public API when
[rust#67295](https://github.com/rust-lang/rust/pull/67295) is resolved.

*/
use crate::*;

use std::time::{Duration, Instant};

pub fn simple<R, F: FnOnce(&TimeSystem) -> R>(f: F) {
    let time_system = TimeSystem::new();
    let _r = f(&time_system);
    let start_time = Instant::now();
    while Instant::now() - start_time < Duration::from_secs(2) {
        std::thread::sleep(std::time::Duration::from_millis(100));
        time_system.process_frame();
    }
}

pub fn pump_for_secs(time_system: &TimeSystem, secs: u64) {
    let start_time = Instant::now();
    while Instant::now() - start_time < Duration::from_secs(secs) {
        std::thread::sleep(std::time::Duration::from_millis(100));
        time_system.process_frame();
    }
}
