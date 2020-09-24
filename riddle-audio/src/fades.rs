use std::{sync::Arc, time::Duration, time::Instant};

use rodio::Sink;

pub(crate) struct FadeKey {
    pub sink: Arc<Sink>,
}

impl std::hash::Hash for FadeKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(&*self.sink, state);
    }
}

impl std::cmp::PartialEq for FadeKey {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.sink, &other.sink)
    }
}

impl std::cmp::Eq for FadeKey {}

pub(crate) enum FadeType {
    Pause,
    Resume,
    AlterVolume,
}

pub(crate) struct Fade {
    sink: Arc<Sink>,
    start_volume: f32,
    dest_volume: f32,
    start_time: Instant,
    duration: Duration,
    fade_type: FadeType,
}

impl Fade {
    pub fn new(sink: Arc<Sink>, dest_volume: f32, duration: Duration, fade_type: FadeType) -> Self {
        let start_volume = sink.volume();
        let start_time = Instant::now();
        Self {
            sink,
            start_volume,
            dest_volume,
            start_time,
            duration,
            fade_type,
        }
    }

    pub fn merge_pair(old: Self, new: Self) -> Self {
        use FadeType::*;
        match (&old.fade_type, &new.fade_type) {
            (AlterVolume, _) => new,
            (Pause, _) => old,
            (Resume, _) => old,
        }
    }

    pub fn update(&self, now: Instant) -> bool {
        let current_duration = now.duration_since(self.start_time);
        let t = current_duration.as_secs_f32() / self.duration.as_secs_f32();
        let new_volume = self.start_volume + ((self.dest_volume - self.start_volume) * t.min(1.0));
        self.sink.set_volume(new_volume);

        let finished = t >= 1.0;
        if finished {
            match &self.fade_type {
                FadeType::Pause => {
                    self.sink.pause();
                }
                _ => (),
            }
        }

        !finished
    }

    pub fn key(&self) -> FadeKey {
        FadeKey {
            sink: self.sink.clone(),
        }
    }
}
