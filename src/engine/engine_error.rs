use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Errors than can occur with the [Rust3DEngine]
pub enum EngineError {
    /// No GPU adapter has been found to run the pixel buffer
    AdapterNotFound,
    /// Closing of the engine invoked
    CloseInvocation,
    /// Error triggered during a render
    Rendering,
}

impl EngineError {
    fn message(&self) -> &str {
        match self {
            Self::AdapterNotFound => "GPU adapter not found",
            Self::CloseInvocation => "Close invoked",
            Self::Rendering => "Rendering has failed",
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
