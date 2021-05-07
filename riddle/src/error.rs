use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RiddleError {
	#[cfg(feature = "riddle-audio")]
	#[error(transparent)]
	Audio(#[from] audio::AudioError),

	#[cfg(feature = "riddle-font")]
	#[error(transparent)]
	Font(#[from] font::FontError),

	#[error(transparent)]
	Image(#[from] image::ImageError),

	#[error(transparent)]
	Input(#[from] input::InputError),

	#[cfg(feature = "riddle-renderer-wgpu")]
	#[error(transparent)]
	WgpuRenderer(#[from] renderer::WgpuRendererError),

	#[error(transparent)]
	Window(#[from] platform::PlatformError),
}
