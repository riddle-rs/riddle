use crate::*;

use rodio::{decoder::Decoder, Source};
use std::{io::Cursor, io::Read, sync::Arc};

/// Stores the raw data of an audio file.
///
/// To play the clip construct a [`ClipPlayer`] using a [`ClipPlayerBuilder`].
///
/// # Example
/// ```
/// # use riddle_audio::*; fn main() -> Result<(), AudioError> {
/// let clip_bytes = include_bytes!("../../example_assets/boop.wav");
/// let clip = Clip::new(&clip_bytes[..])?;
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct Clip {
    pub(crate) data: ClipData,
    duration: std::time::Duration,
}

impl Clip {
    /// Reads the entirety of the data reader in to memory.
    ///
    /// An [`AudioError::ClipDecodeError`] value will be returned if the data isn't a known format
    /// or if the duration of the clip could not be determined.
    pub fn new<R>(mut data: R) -> Result<Clip>
    where
        R: Read,
    {
        let mut owned_data: Vec<u8> = vec![];
        data.read_to_end(&mut owned_data)
            .map_err(|e| CommonError::IOError(e))?;

        let source = Decoder::new(Cursor::new(owned_data.clone()))
            .map_err(|_| AudioError::ClipDecodeError)?;
        let duration = source.total_duration().ok_or(AudioError::ClipDecodeError)?;

        Ok(Self {
            data: ClipData::new(owned_data),
            duration,
        })
    }

    /// Get the run time of the clip.
    ///
    /// # Example
    ///
    /// ```
    /// # use riddle_audio::*; fn main() -> Result<(), AudioError> {
    /// let clip_bytes = include_bytes!("../../example_assets/boop.wav");
    /// let clip = Clip::new(&clip_bytes[..])?;
    ///
    /// assert!(clip.duration() > std::time::Duration::from_secs(0));
    /// # Ok(()) }
    /// ```
    pub fn duration(&self) -> std::time::Duration {
        self.duration
    }
}

#[derive(Clone)]
pub(crate) struct ClipData {
    data: Arc<Vec<u8>>,
}

impl ClipData {
    fn new(data: Vec<u8>) -> Self {
        Self { data: data.into() }
    }
}

impl AsRef<[u8]> for ClipData {
    fn as_ref(&self) -> &[u8] {
        Arc::as_ref(&self.data).as_ref()
    }
}
