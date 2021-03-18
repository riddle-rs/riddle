use crate::*;

use futures::AsyncReadExt;
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
/// let clip = Clip::load(&clip_bytes[..], ClipFormat::Wav)?;
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct Clip {
	pub(crate) data: ClipData,
	duration: Option<std::time::Duration>,
}

impl Clip {
	/// Reads the entirety of the data reader in to memory.
	///
	/// An [`AudioError::ClipDecodeError`] value will be returned if the data isn't a known format.
	pub fn load<R>(mut data: R, format: ClipFormat) -> Result<Clip>
	where
		R: Read,
	{
		let mut owned_data: Vec<u8> = vec![];
		data.read_to_end(&mut owned_data)?;
		Self::from_owned_bytes(owned_data, format)
	}

	/// Reads the entirety of the data reader in to memory, asynchronously.
	///
	/// An [`AudioError::ClipDecodeError`] value will be returned if the data isn't a known format.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_audio::*; fn main() -> Result<(), AudioError> { futures::executor::block_on(async_main()) }
	/// # async fn async_main() -> Result<(), AudioError> {
	/// let clip_bytes = include_bytes!("../../example_assets/boop.wav");
	/// let clip = Clip::load_async(&clip_bytes[..], ClipFormat::Wav).await?;
	/// # Ok(()) }
	/// ```
	pub async fn load_async<R>(mut data: R, format: ClipFormat) -> Result<Clip>
	where
		R: futures::io::AsyncRead + std::marker::Unpin,
	{
		let mut owned_data: Vec<u8> = vec![];
		data.read_to_end(&mut owned_data).await?;
		Self::from_owned_bytes(owned_data, format)
	}

	fn from_owned_bytes(owned_data: Vec<u8>, format: ClipFormat) -> Result<Clip> {
		let cursor = Cursor::new(owned_data.clone());

		let source = match format {
			ClipFormat::Wav => Decoder::new_wav(cursor),
			ClipFormat::Vorbis => Decoder::new_vorbis(cursor),

			#[cfg(feature = "riddle-mp3")]
			ClipFormat::Mp3 => Decoder::new_mp3(cursor),
		}
		.map_err(|_| AudioError::ClipDecodeError)?;

		let duration = source.total_duration();

		Ok(Self {
			data: ClipData::new(owned_data),
			duration,
		})
	}

	/// Get the run time of the clip, if it can be determined.
	///
	/// # Example
	///
	/// ```
	/// # use riddle_audio::*; fn main() -> Result<(), AudioError> {
	/// let clip_bytes = include_bytes!("../../example_assets/boop.wav");
	/// let clip = Clip::load(&clip_bytes[..], ClipFormat::Wav)?;
	///
	/// assert!(clip.duration().unwrap() > std::time::Duration::from_secs(0));
	/// # Ok(()) }
	/// ```
	pub fn duration(&self) -> Option<std::time::Duration> {
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

/// The set of support audio file formats which [`Clip`] can load
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ClipFormat {
	Wav,
	Vorbis,

	#[cfg(feature = "riddle-mp3")]
	#[doc(cfg(feature = "riddle-mp3"))]
	Mp3,
}
