use crate::*;

use std::sync::Mutex;

/// The Riddle time system core state.
///
/// Manages tracking framerate and timer state.
///
/// It is possible to manage the audio system state independantly - the most important
/// thing to note is that [`ext::TimeSystemExt::process_frame`] must be called once per
/// "frame". The default Riddle integration calls that method whenever the
/// `riddle::Event::ProcessFrame` event is fired.
#[derive(Clone)]
pub struct TimeSystem {
	internal: std::sync::Arc<TimeSystemInternal>,
}

pub(crate) struct TimeSystemInternal {
	frame_time: Mutex<FrameTime>,
	timers: TimerSet,
}

impl TimeSystem {
	/// Get the current FPS as calculated based on previous frame durations.
	pub fn fps(&self) -> f32 {
		self.internal.frame_time.lock().unwrap().fps
	}

	/// Get the current delta t as calculated based on previous frame durations.
	pub fn delta_secs(&self) -> f32 {
		1.0 / self.internal.frame_time.lock().unwrap().fps
	}

	/// Get the reference time for this frame. Captured during [`ext::TimeSystemExt::process_frame`].
	pub fn frame_instant(&self) -> instant::Instant {
		self.internal.frame_time.lock().unwrap().frame_instant
	}

	/// Register a timer with a callback which will be fired when the time elpases.
	///
	/// The returned handle may be dropped without cancelling the timer.
	///
	/// # Example
	///
	/// ```
	/// # use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
	/// # use riddle_time::*; doctest::simple(|time_system| {
	/// let val = Arc::new(AtomicBool::new(false));
	///
	/// time_system.register_timer(std::time::Duration::from_millis(200), {
	///     let val = val.clone();
	///     move || { val.store(true, Ordering::Relaxed); }
	/// });
	///
	/// // A while later
	/// # doctest::pump_for_secs(time_system, 1);
	/// assert_eq!(true, val.load(Ordering::Relaxed));
	/// # });
	/// ```
	pub fn register_timer<F>(&self, duration: std::time::Duration, callback: F) -> TimerHandle
	where
		F: FnOnce() + Send + 'static,
	{
		self.internal
			.timers
			.register_timer(duration, Box::new(callback))
	}
}

impl ext::TimeSystemExt for TimeSystem {
	fn new() -> Self {
		let internal = TimeSystemInternal {
			frame_time: Mutex::new(FrameTime::new()),
			timers: TimerSet::new(),
		};
		Self {
			internal: std::sync::Arc::new(internal),
		}
	}

	fn process_frame(&self) {
		let mut locked_time = self.internal.frame_time.lock().unwrap();
		locked_time.update();
		let delta = locked_time.frame_delta;
		drop(locked_time);

		self.internal.timers.update(delta);
	}
}

struct FrameTime {
	frame_instant: instant::Instant,
	frame_delta: instant::Duration,
	fps: f32,
}

impl FrameTime {
	fn new() -> Self {
		Self {
			frame_instant: instant::Instant::now(),
			frame_delta: Default::default(),
			fps: 0.0,
		}
	}

	fn update(&mut self) {
		let now = instant::Instant::now();
		self.frame_delta = now.duration_since(self.frame_instant);
		self.fps = 1.0 / self.frame_delta.as_secs_f32().max(0.0001);
		self.frame_instant = now;
	}
}
