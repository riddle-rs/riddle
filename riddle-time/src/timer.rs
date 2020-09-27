use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex, Weak,
};

pub(crate) struct TimerSet {
    active_timers: Mutex<Vec<Timer>>,
}

impl TimerSet {
    pub fn new() -> Self {
        Self {
            active_timers: Mutex::new(vec![]),
        }
    }

    pub fn update(&self, dt: std::time::Duration) {
        let mut timers = std::mem::take(&mut *self.active_timers.lock().unwrap());
        for timer in timers.iter_mut() {
            timer.update(dt);
        }
        timers.retain(|t| t.pending());
        self.active_timers.lock().unwrap().append(&mut timers);
    }

    pub fn register_timer(
        &self,
        duration: std::time::Duration,
        callback: Box<dyn FnOnce() + Send>,
    ) -> TimerHandle {
        let timer = Timer::new(duration, callback);
        let handle = TimerHandle {
            shared_state: Arc::downgrade(&timer.shared_state),
        };
        self.active_timers.lock().unwrap().push(timer);
        handle
    }
}

struct Timer {
    time_remaining: std::time::Duration,
    callback: Option<Box<dyn FnOnce() + Send>>,
    shared_state: Arc<SharedTimerState>,
}

impl Timer {
    pub fn new(duration: std::time::Duration, callback: Box<dyn FnOnce() + Send>) -> Self {
        Self {
            time_remaining: duration,
            callback: Some(callback),
            shared_state: Arc::new(SharedTimerState::default()),
        }
    }

    pub fn pending(&self) -> bool {
        !self.shared_state.cancelled() && self.time_remaining > std::time::Duration::from_secs(0)
    }

    fn update(&mut self, dt: std::time::Duration) {
        if self.pending() {
            match self.time_remaining.checked_sub(dt) {
                Some(remaining) => {
                    self.time_remaining = remaining;
                }
                None => {
                    self.complete();
                }
            }
        };
    }

    fn complete(&mut self) {
        if let Some(callback) = self.callback.take() {
            callback();
        }
        self.time_remaining = std::time::Duration::default();
        self.shared_state.pending.store(false, Ordering::Relaxed);
    }
}

#[derive(Debug)]
struct SharedTimerState {
    pending: AtomicBool,
    cancelled: AtomicBool,
}

impl SharedTimerState {
    fn cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    fn pending(&self) -> bool {
        self.pending.load(Ordering::Relaxed)
    }
}

impl Default for SharedTimerState {
    fn default() -> Self {
        Self {
            pending: AtomicBool::new(true),
            cancelled: AtomicBool::new(false),
        }
    }
}

/// A handle to a timer created by [`crate::TimeSystem::register_timer`].
///
/// It can be used to detect whether the timer has triggered, and to cancel
/// the timer if it hasn't been triggered yet.
pub struct TimerHandle {
    shared_state: Weak<SharedTimerState>,
}

impl TimerHandle {
    /// Cancel a pending timer.
    ///
    /// If the timer has already fired, or been cancelled, this is a no-op.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};
    /// # use riddle_time::*; doctest::simple(|time_system| {
    /// let timer_handle = time_system.register_timer(Duration::from_millis(200),
    ///     move || { panic!("The timer should never fire"); }
    /// );
    ///
    /// timer_handle.cancel();
    ///
    /// # doctest::pump_for_secs(time_system, 1);
    /// // The panic! never fires because the timer was cancelled.
    /// # });
    /// ```
    pub fn cancel(&self) {
        if let Some(state) = Weak::upgrade(&self.shared_state) {
            state.cancelled.store(true, Ordering::Relaxed);
        }
    }

    /// Check whether a timer has yet to fire.
    ///
    /// If the timer was cancelled before it was fired, this will remain `true`.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::{sync::{Arc, atomic::{AtomicBool, Ordering}}, time::Duration};
    /// # use riddle_time::*; doctest::simple(|time_system| {    ///
    /// let timer_handle = time_system.register_timer(Duration::from_millis(200), || {});
    ///
    /// assert_eq!(true, timer_handle.pending());
    ///
    /// // A while later
    /// # doctest::pump_for_secs(time_system, 1);
    /// // The timer has fired and is no longer pending
    /// assert_eq!(false, timer_handle.pending());
    /// # });
    /// ```
    pub fn pending(&self) -> bool {
        match Weak::upgrade(&self.shared_state) {
            Some(state) => state.pending(),
            _ => false,
        }
    }
}
