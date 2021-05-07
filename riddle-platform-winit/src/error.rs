use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
	#[error("Message Dispatch Error")]
	MessageDispatch,

	#[error("Invalid context state")]
	InvalidContext,

	#[error("Window creation failure")]
	WindowInit,
}
