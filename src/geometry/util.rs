use std::str::FromStr;

use super::geometry_error::GeometryError;

pub(crate) fn parse_next<T: FromStr>(slice: Option<&str>, line: &str) -> Result<T, GeometryError> {
    slice
        .ok_or_else(|| GeometryError::MissingValue(line.to_string()))?
        .parse()
        .map_err(|_| GeometryError::WrongNumber(line.to_string()))
}

#[cfg(test)]
mod tests {
    use super::parse_next;
    use crate::geometry::geometry_error::GeometryError;

    #[test]
    fn parse_next_valid() {
        assert_eq!(parse_next::<f32>(Some("1.0"), "1.0").unwrap(), 1.0);
        assert_eq!(parse_next::<i32>(Some("1"), "1").unwrap(), 1);
    }

    #[test]
    fn parse_next_none() {
        match parse_next::<u32>(None, "line").unwrap_err() {
            GeometryError::MissingValue(line) => assert_eq!(line, "line"),
            _ => panic!(),
        }
    }

    #[test]
    fn parse_next_invalid_number() {
        match parse_next::<u32>(Some("-1.0"), "line").unwrap_err() {
            GeometryError::WrongNumber(line) => assert_eq!(line, "line"),
            _ => panic!(),
        }
    }
}
