use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub(crate) struct TimerSet {
    active_timers: RefCell<Vec<Timer>>,
}

impl TimerSet {
    pub fn new() -> Self {
        Self {
            active_timers: RefCell::new(vec![]),
        }
    }

    pub fn update(&self, dt: std::time::Duration) {
        let mut timers = std::mem::take(&mut *self.active_timers.borrow_mut());
        for timer in timers.iter_mut() {
            timer.update(dt);
        }
        timers.retain(|t| t.pending());
        self.active_timers.borrow_mut().append(&mut timers);
    }

    pub fn register_timer(
        &self,
        duration: std::time::Duration,
        callback: Box<dyn FnOnce()>,
    ) -> TimerHandle {
        let timer = Timer::new(duration, callback);
        let handle = TimerHandle {
            shared_state: Rc::downgrade(&timer.shared_state),
        };
        self.active_timers.borrow_mut().push(timer);
        handle
    }
}

struct Timer {
    time_remaining: std::time::Duration,
    callback: Option<Box<dyn FnOnce()>>,
    shared_state: Rc<SharedTimerState>,
}

impl Timer {
    pub fn new(duration: std::time::Duration, callback: Box<dyn FnOnce()>) -> Self {
        Self {
            time_remaining: duration,
            callback: Some(callback),
            shared_state: Rc::new(SharedTimerState::default()),
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
        *self.shared_state.pending.borrow_mut() = false;
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SharedTimerState {
    pending: RefCell<bool>,
    cancelled: RefCell<bool>,
}

impl SharedTimerState {
    fn cancelled(&self) -> bool {
        *self.cancelled.borrow()
    }

    fn pending(&self) -> bool {
        *self.pending.borrow()
    }
}

impl Default for SharedTimerState {
    fn default() -> Self {
        Self {
            pending: RefCell::new(true),
            cancelled: RefCell::new(false),
        }
    }
}

pub struct TimerHandle {
    shared_state: Weak<SharedTimerState>,
}

impl TimerHandle {
    pub fn cancel(&self) {
        if let Some(state) = Weak::upgrade(&self.shared_state) {
            *state.cancelled.borrow_mut() = true;
        }
    }

    pub fn pending(&self) -> bool {
        match Weak::upgrade(&self.shared_state) {
            Some(state) => state.pending(),
            _ => false,
        }
    }
}
