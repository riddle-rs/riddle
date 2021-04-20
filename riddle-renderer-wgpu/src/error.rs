use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WGPURendererError {
	#[error("Error initializing graphics API")]
	APIInitError(&'static str),

	#[error("Error beginning render")]
	BeginRenderError(&'static str),

	#[error(transparent)]
	ImageError(#[from] image::ImageError),

	#[error(transparent)]
	CommonError(#[from] CommonError),

	#[error(transparent)]
	RendererCommonError(#[from] RendererError),

	#[error("Unknown Renderer Error")]
	Unknown,
}

impl From<WGPURendererError> for RendererError {
	fn from(err: WGPURendererError) -> Self {
		match err {
			WGPURendererError::APIInitError(_) => RendererError::Unknown,
			WGPURendererError::BeginRenderError(_) => RendererError::Unknown,
			WGPURendererError::ImageError(_) => RendererError::Unknown,
			WGPURendererError::CommonError(_) => RendererError::Unknown,
			WGPURendererError::RendererCommonError(err) => err,
			WGPURendererError::Unknown => RendererError::Unknown,
		}
	}
}
