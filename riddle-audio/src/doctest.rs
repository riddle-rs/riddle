//! **DO NOT RELY ON THIS MODULE**
//!
//! **TEST CODE ONLY**
//!
//! This module contains utilities for use during doctest builds. To be removed from the public API when
//! [rust#67295](https://github.com/rust-lang/rust/pull/67295) is resolved.

use crate::{ext::*, *};

use std::time::{Duration, Instant};

pub fn simple<R, F: FnOnce(&AudioSystem) -> Result<R>>(f: F) {
	let (audio_system, _main_thread_state) = AudioSystem::new_system_pair().unwrap();
	let _r = f(&audio_system).unwrap();
	let start_time = Instant::now();
	while Instant::now() - start_time < Duration::from_secs(2) {
		audio_system.process_frame();
	}
}

pub fn pump_for_secs(audio_system: &AudioSystem, secs: u64) {
	let start_time = Instant::now();
	while Instant::now() - start_time < Duration::from_secs(secs) {
		audio_system.process_frame();
	}
}
