use std::cell::RefCell;

pub struct TimeSystem {
    frame_time: RefCell<FrameTime>,
}

impl TimeSystem {
    pub fn new() -> Self {
        Self {
            frame_time: RefCell::new(FrameTime::new()),
        }
    }

    pub fn process_frame(&self) {
        self.frame_time.borrow_mut().update();
    }

    pub fn fps(&self) -> f32 {
        self.frame_time.borrow().fps
    }

    pub fn frame_instant(&self) -> std::time::Instant {
        self.frame_time.borrow().frame_instant
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
