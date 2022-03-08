use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

/// Errors than can occur within the engine environment
pub(crate) enum EnvironmentError<'a> {
    /// A file to load in the environment is missing or invalid
    BadFile(&'a str),
}

impl<'a> EnvironmentError<'a> {
    fn message(&self) -> &str {
        match self {
            Self::BadFile(file) => file
        }
    }
}

impl<'a> Error for EnvironmentError<'a> {}

impl<'a> Debug for EnvironmentError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl<'a> Display for EnvironmentError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}