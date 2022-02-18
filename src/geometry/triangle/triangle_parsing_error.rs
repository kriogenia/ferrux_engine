use super::super::vector::point_parsing_error::PointParsingError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq)]
pub enum TriangleParsingError {
    InvalidPointNumber,
    InvalidPoint(PointParsingError),
}

impl TriangleParsingError {
    fn message(&self) -> String {
        match self {
            Self::InvalidPointNumber => "Invalid number of points. It must be three.".to_string(),
            Self::InvalidPoint(point_error) => {
                format!("One of the points is invalid. {}", point_error)
            }
        }
    }
}

impl Debug for TriangleParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Display for TriangleParsingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl Error for TriangleParsingError {}

impl From<PointParsingError> for TriangleParsingError {
    fn from(error: PointParsingError) -> Self {
        Self::InvalidPoint(error)
    }
}
