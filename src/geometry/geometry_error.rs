use crate::geometry::triangle::triangle_parsing_error::TriangleParsingError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(PartialEq)]
pub enum GeometryError {
    InvalidMesh,
    InvalidPoint(TriangleParsingError),
    InvalidTriangle(TriangleParsingError),
	MissingValue(String),
	WrongNumber(String)
}

impl GeometryError {
    fn message(&self) -> String {
        match self {
            Self::InvalidMesh => "Invalid mesh. At least one triangle is required.".to_string(),
            Self::InvalidPoint(error) => format!("Invalid triangle: {}", error),
            Self::InvalidTriangle(error) => format!("Invalid triangle: {}", error),
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

impl From<TriangleParsingError> for GeometryError {
    fn from(error: TriangleParsingError) -> Self {
        match error {
            TriangleParsingError::InvalidPoint(_) => Self::InvalidPoint(error),
            TriangleParsingError::InvalidPointNumber => Self::InvalidTriangle(error),
        }
    }
}