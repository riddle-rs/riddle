use thiserror::Error;

#[derive(Debug, Error)]
pub enum InputError {
	#[error("Initialization Error {0}")]
	Init(&'static str),
}
