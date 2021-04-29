use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RiddleError {
	#[cfg(feature = "riddle-audio")]
	#[error(transparent)]
	AudioError(#[from] audio::AudioError),

	#[cfg(feature = "riddle-font")]
	#[error(transparent)]
	FontError(#[from] font::FontError),

	#[error(transparent)]
	ImageError(#[from] image::ImageError),

	#[error(transparent)]
	InputError(#[from] input::InputError),

	#[error(transparent)]
	RendererError(#[from] renderer::RendererError),

	#[cfg(feature = "riddle-renderer-wgpu")]
	#[error(transparent)]
	WgpuRendererError(#[from] renderer::WgpuRendererError),

	#[error(transparent)]
	WindowError(#[from] platform::PlatformError),
}
