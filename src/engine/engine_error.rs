use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use crate::environment::error::EnvironmentError;

/// Errors than can occur with the [Rust3DEngine]
pub enum EngineError<'a> {
    /// No GPU adapter has been found to run the pixel buffer
    AdapterNotFound,
    /// A file to be rendered is missing or invalid
    BadFile(&'a str),
    /// Closing of the engine invoked
    CloseInvocation,
    /// Error triggered during a render
    Rendering,
}

impl<'a> EngineError<'a> {
    fn message(&self) -> String {
        match self {
            Self::AdapterNotFound => "GPU adapter not found".to_string(),
            Self::BadFile(file) => {
                format!("The specified file {file} is missing or invalid")
            },
            Self::CloseInvocation => "Close invoked".to_string(),
            Self::Rendering => "Rendering has failed".to_string(),
        }
    }
}

impl<'a> Error for EngineError<'a> {}

impl<'a> Debug for EngineError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl<'a> Display for EngineError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl<'a> From<EnvironmentError<'a>> for EngineError<'a> {
    fn from(e: EnvironmentError<'a>) -> Self {
        match e {
            EnvironmentError::BadFile(file) => Self::BadFile(file),
        }
    }
}
