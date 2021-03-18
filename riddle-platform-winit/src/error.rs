use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlatformError {
	#[error("Message Dispatch Error")]
	MessageDispatchError,

	#[error("Invalid context state")]
	InvalidContextState,

	#[error("Window creation failure")]
	WindowInitFailure,

	#[error("Unknown Window Error")]
	Unknown,
}
