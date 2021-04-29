use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WgpuRendererError {
	#[error("Error initializing graphics API")]
	ApiInitError(&'static str),

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

impl From<WgpuRendererError> for RendererError {
	fn from(err: WgpuRendererError) -> Self {
		match err {
			WgpuRendererError::ApiInitError(_) => RendererError::Unknown,
			WgpuRendererError::BeginRenderError(_) => RendererError::Unknown,
			WgpuRendererError::ImageError(_) => RendererError::Unknown,
			WgpuRendererError::CommonError(_) => RendererError::Unknown,
			WgpuRendererError::RendererCommonError(err) => err,
			WgpuRendererError::Unknown => RendererError::Unknown,
		}
	}
}
