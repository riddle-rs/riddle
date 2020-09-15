use std::sync::Mutex;

pub(crate) struct TimerSet {
    active_timers: Mutex<Vec<Timer>>,
}

impl TimerSet {
    pub fn new() -> Self {
        Self {
            active_timers: std::sync::Mutex::new(vec![]),
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
            shared_state: std::sync::Arc::downgrade(&timer.shared_state),
        };
        self.active_timers.lock().unwrap().push(timer);
        handle
    }
}

struct Timer {
    time_remaining: std::time::Duration,
    callback: Option<Box<dyn FnOnce() + Send>>,
    shared_state: std::sync::Arc<SharedTimerState>,
}

impl Timer {
    pub fn new(duration: std::time::Duration, callback: Box<dyn FnOnce() + Send>) -> Self {
        Self {
            time_remaining: duration,
            callback: Some(callback),
            shared_state: std::sync::Arc::new(SharedTimerState::default()),
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
        *self.shared_state.pending.lock().unwrap() = false;
    }
}

#[derive(Debug)]
struct SharedTimerState {
    pending: std::sync::Mutex<bool>,
    cancelled: std::sync::Mutex<bool>,
}

impl SharedTimerState {
    fn cancelled(&self) -> bool {
        *self.cancelled.lock().unwrap()
    }

    fn pending(&self) -> bool {
        *self.pending.lock().unwrap()
    }
}

impl Default for SharedTimerState {
    fn default() -> Self {
        Self {
            pending: std::sync::Mutex::new(true),
            cancelled: std::sync::Mutex::new(false),
        }
    }
}

pub struct TimerHandle {
    shared_state: std::sync::Weak<SharedTimerState>,
}

impl TimerHandle {
    pub fn cancel(&self) {
        if let Some(state) = std::sync::Weak::upgrade(&self.shared_state) {
            *state.cancelled.lock().unwrap() = true;
        }
    }

    pub fn pending(&self) -> bool {
        match std::sync::Weak::upgrade(&self.shared_state) {
            Some(state) => state.pending(),
            _ => false,
        }
    }
}
