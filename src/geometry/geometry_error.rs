use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(PartialEq)]
pub enum GeometryError {
	EmptyMesh,
	MissingValue(String),
	WrongNumber(String)
}

impl GeometryError {
    fn message(&self) -> String {
        match self {
			Self::EmptyMesh => "The specified mesh is missing points or triangles".to_string(),
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