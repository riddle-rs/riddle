use crate::*;

use riddle_common::define_handles;

use std::sync::Mutex;

/// The Riddle time system core state.
///
/// Manages tracking framerate and timer state.
///
/// It is possible to manage the audio system state independantly - the most important
/// thing to note is that [`TimeSystem::process_frame`] must be called once per "frame".
/// The default Riddle integration calls that method whenever the
/// `riddle::Event::ProcessFrame` event is fired.
pub struct TimeSystem {
    weak_self: TimeSystemWeak,
    frame_time: Mutex<FrameTime>,
    timers: TimerSet,
}

define_handles!(<TimeSystem>::weak_self, pub TimeSystemHandle, pub TimeSystemWeak);

impl TimeSystem {
    /// Create a new time system. The time the system is created is used as the time
    /// of the 0th frame.
    pub fn new() -> TimeSystemHandle {
        TimeSystemHandle::new(|weak_self| Self {
            weak_self,
            frame_time: Mutex::new(FrameTime::new()),
            timers: TimerSet::new(),
        })
    }

    /// Update the time system state, marking the beginning of a the next frame.
    ///
    /// The instant that this method is called is taken as the reference time for
    /// the frame that is about to be executed.
    ///
    /// Timers will also be triggered during this function call if they are due
    /// to trigger.
    ///
    /// **Do not** call this function directly if you are using this through the
    /// `riddle` crate.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_time::*; doctest::simple(|time_system| {
    /// let frame_1 = time_system.frame_instant();
    ///
    /// // A while later
    /// # doctest::pump_for_secs(time_system, 1);
    /// let frame_n = time_system.frame_instant();
    ///
    /// assert_eq!(true, frame_n - frame_1 > std::time::Duration::from_secs(0));
    /// # });
    /// ```
    pub fn process_frame(&self) {
        let mut locked_time = self.frame_time.lock().unwrap();
        locked_time.update();
        let delta = locked_time.frame_delta;
        drop(locked_time);

        self.timers.update(delta);
    }

    /// Get the current FPS as calculated based on previous frame durations.
    pub fn fps(&self) -> f32 {
        self.frame_time.lock().unwrap().fps
    }

    /// Get the current delta t as calculated based on previous frame durations.
    pub fn delta_secs(&self) -> f32 {
        1.0 / self.frame_time.lock().unwrap().fps
    }

    /// Get the reference time for this frame. Captured during [`TimeSystem::process_frame`].
    pub fn frame_instant(&self) -> std::time::Instant {
        self.frame_time.lock().unwrap().frame_instant
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
        self.timers.register_timer(duration, Box::new(callback))
    }
}

struct FrameTime {
    frame_instant: std::time::Instant,
    frame_delta: std::time::Duration,
    fps: f32,
}

impl FrameTime {
    fn new() -> Self {
        Self {
            frame_instant: std::time::Instant::now(),
            frame_delta: Default::default(),
            fps: 0.0,
        }
    }

    fn update(&mut self) {
        let now = std::time::Instant::now();
        self.frame_delta = now.duration_since(self.frame_instant);
        self.fps = 1.0 / self.frame_delta.as_secs_f32().max(0.001);
        self.frame_instant = now;
    }
}
