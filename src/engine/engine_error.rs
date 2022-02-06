use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub enum EngineError {
	AdapterNotFound,
	CloseInvocation,
	Rendering
}

impl EngineError {
	/// Informative Error message
	fn message(&self) -> &str {
		match self {
			Self::AdapterNotFound => "GPU adapter not found",
			Self::CloseInvocation => "Close invoked",
			Self::Rendering => "Rendering has failed"
		}
	}
}

/** Error */

impl Error for EngineError {}

/** Printing */

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