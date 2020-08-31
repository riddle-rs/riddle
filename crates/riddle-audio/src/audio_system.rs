use crate::*;
use std::cell::RefCell;

pub struct AudioSystem {
    pub(super) device: rodio::Device,

    fades: RefCell<Vec<Fade>>,
}

impl AudioSystem {
    pub fn new() -> Result<AudioSystem, AudioError> {
        let device = rodio::default_output_device().ok_or(AudioError::UnknownError)?;
        Ok(AudioSystem {
            device,
            fades: RefCell::new(vec![]),
        })
    }

    pub fn tick(&self) {}

    pub(crate) fn register_fade(&self, fade: Fade) {
        let mut fades = self.fades.borrow_mut();
        fades.retain(|f| !fade.sinks_eq(f));
        fades.push(fade);
    }

    pub fn tick_fades(&self, now: std::time::Instant) {
        let mut fades = self.fades.borrow_mut();
        fades.retain(|f| f.update(now));
    }
}

pub(crate) struct Fade {
    sink: std::rc::Rc<rodio::Sink>,
    start_volume: f32,
    dest_volume: f32,
    start_time: std::time::Instant,
    duration: std::time::Duration,
}

impl Fade {
    fn new(
        sink: std::rc::Rc<rodio::Sink>,
        dest_volume: f32,
        start_time: std::time::Instant,
        duration: std::time::Duration,
    ) -> Self {
        let start_volume = sink.volume();
        Self {
            sink,
            start_volume,
            dest_volume,
            start_time,
            duration,
        }
    }

    fn sinks_eq(&self, other: &Fade) -> bool {
        std::rc::Rc::ptr_eq(&self.sink, &other.sink)
    }

    fn update(&self, now: std::time::Instant) -> bool {
        let current_duration = now.duration_since(self.start_time);
        let t = current_duration.as_secs_f32() / self.duration.as_secs_f32();
        let new_volume = self.start_volume + ((self.dest_volume - self.start_volume) * t.min(1.0));
        self.sink.set_volume(new_volume);
        t < 1.0
    }
}
