use crate::*;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum WgpuRendererError {
	#[error("Error initializing graphics API: {0}")]
	ApiInit(&'static str),

	#[error("Error beginning render: {0}")]
	BeginRender(&'static str),

	#[error("Shader loading error: {0}")]
	ShaderLoad(&'static str),

	#[error(transparent)]
	Image(#[from] image::ImageError),

	#[error(transparent)]
	Common(#[from] CommonError),
}
