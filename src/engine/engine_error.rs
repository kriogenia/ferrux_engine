use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub enum EngineError {
	AdapterNotFound,
	CloseInvocation,
	Rendering
}

impl EngineError {
	fn message(&self) -> &str {
		match self {
			Self::AdapterNotFound => "GPU adapter not found",
			Self::CloseInvocation => "Close invoked",
			Self::Rendering => "Rendering has failed"
		}
	}
}

impl Error for EngineError {}

impl Debug for EngineError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Display for EngineError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}