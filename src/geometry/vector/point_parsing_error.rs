use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseFloatError;

#[derive(PartialEq)]
pub enum PointParsingError {
    InvalidAxisNumber,
    InvalidFloat,
}

impl PointParsingError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidAxisNumber => "Invalid number of axis. It must be three.",
            Self::InvalidFloat => "One of the floats of the point is invalid.",
        }
    }
}

impl Debug for PointParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for PointParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for PointParsingError {}

impl From<ParseFloatError> for PointParsingError {
    fn from(_: ParseFloatError) -> Self {
        Self::InvalidFloat
    }
}
