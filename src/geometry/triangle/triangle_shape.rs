use crate::geometry::Point3;
use crate::geometry::triangle::triangle_parsing_error::TriangleParsingError;
use crate::geometry::vector::point_parsing_error::PointParsingError;

/// Three-dimensional triangle composed with three [Point3]
#[derive(Debug)]
pub struct Triangle {
	points: [Point3; 3]
}

impl Triangle {

	/// Builds a new triangle with the given points
	///
	/// # Arguments
	/// * `point_a`, `point_b`, `point_c` - Points of the triangle
	///
	pub fn new(point_a: Point, point_b: Point, point_c: Point) -> Self {
		Self {
			points: [
				Point3 { x: point_a.0, y: point_a.1, z: point_a.2 },
				Point3 { x: point_b.0, y: point_b.1, z: point_b.2 },
				Point3 { x: point_c.0, y: point_c.1, z: point_c.2 }
			]
		}
	}

}

impl TryFrom<String> for Triangle {
	type Error = TriangleParsingError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let mut parsed: Vec<&str> = value.split(",").collect();

		if parsed.len() != 3 {
			return Err(TriangleParsingError::InvalidPointNumber);
		}

		let p1 = parsed[0].to_string().try_into()?;
		let p2 = parsed[1].to_string().try_into()?;
		let p3 = parsed[2].to_string().try_into()?;

		Ok(Triangle { points: [ p1, p2, p3 ] })
	}
}

type Point = (f32, f32, f32);

#[test]
fn valid_parsing() {
	let triangle = Triangle::try_from("1.0 0 -3.5,0 1 2,3 5 2".to_string()).unwrap();
	assert_eq!(triangle.points.len(), 3);
	assert_eq!(triangle.points[0], Point3::try_from("1.0 0 -3.5".to_string()).unwrap());
	assert_eq!(triangle.points[1], Point3::try_from("0 1 2".to_string()).unwrap());
	assert_eq!(triangle.points[2], Point3::try_from("3 5 2".to_string()).unwrap());
}

#[test]
fn invalid_parsing() {
	assert_eq!(Triangle::try_from("1.0 0 -3.5,0 1 2".to_string()).unwrap_err(),
	           TriangleParsingError::InvalidPointNumber);
	assert_eq!(Triangle::try_from(".0 0 -3.5,0 1 2,3 5 2,0 0 0".to_string()).unwrap_err(),
	           TriangleParsingError::InvalidPointNumber);
	assert_eq!(Triangle::try_from("0 0 a,0 1 2,3 5 2".to_string()).unwrap_err(),
	           TriangleParsingError::InvalidPoint(PointParsingError::InvalidFloat));
}