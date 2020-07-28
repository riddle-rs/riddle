use crate::*;

pub struct AudioSystem {
    pub(super) device: rodio::Device,
}

impl AudioSystem {
    pub fn new() -> Result<AudioSystem, AudioError> {
        let device = rodio::default_output_device().ok_or(AudioError::UnknownError)?;
        Ok(AudioSystem { device })
    }
}
