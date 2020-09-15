use crate::*;

use riddle_common::define_handles;

use std::sync::Mutex;

pub struct TimeSystem {
    weak_self: TimeSystemWeak,
    frame_time: Mutex<FrameTime>,
    timers: TimerSet,
}

define_handles!(<TimeSystem>::weak_self, pub TimeSystemHandle, pub TimeSystemWeak);

impl TimeSystem {
    pub fn new() -> TimeSystemHandle {
        TimeSystemHandle::new(|weak_self| Self {
            weak_self,
            frame_time: Mutex::new(FrameTime::new()),
            timers: TimerSet::new(),
        })
    }

    pub fn process_frame(&self) {
        let mut locked_time = self.frame_time.lock().unwrap();
        locked_time.update();
        let delta = locked_time.frame_delta;
        drop(locked_time);

        self.timers.update(delta);
    }

    pub fn fps(&self) -> f32 {
        self.frame_time.lock().unwrap().fps
    }

    pub fn frame_instant(&self) -> std::time::Instant {
        self.frame_time.lock().unwrap().frame_instant
    }

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
