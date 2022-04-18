use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(PartialEq)]
pub enum GeometryError {
	MissingValue(String),
	WrongNumber(String)
}

impl GeometryError {
    fn message(&self) -> String {
        match self {
			Self::MissingValue(line) => format!("Missing value on line: {}", line),
			Self::WrongNumber(line) => format!("Invalid number on line: {}", line),
        }
    }
}

impl Error for GeometryError {}

impl Debug for GeometryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for GeometryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}