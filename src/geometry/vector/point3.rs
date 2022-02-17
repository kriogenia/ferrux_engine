use crate::math::Matrix4;
use crate::geometry::projectable::Projectable;
use crate::geometry::vector::Point2;
use crate::geometry::vector::point_parsing_error::PointParsingError;
use crate::math::vector_dot_matrix;

/// Three-dimensional vector
///
/// # Properties
/// * `x`, `y`, `z` - Axes of the vector
///
#[derive(Debug, PartialEq)]
pub struct Point3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Point3 {
	pub fn update(&mut self, new_pos: (f32, f32, f32)) {
		self.x = new_pos.0;
		self.y = new_pos.1;
		self.z = new_pos.2;
	}
}

impl Projectable<Point2> for Point3 {
	fn get_projection(&self, matrix: &Matrix4, offset: f32, width: f32, height: f32) -> Point2 {
		let (x, y, _) = vector_dot_matrix((self.x, self.y, self.z + offset), matrix);
		Point2 { x, y }.project(width, height)
	}
}

impl TryFrom<String> for Point3 {
	type Error = PointParsingError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let parsed: Vec<&str> = value.split(" ").collect();

		if parsed.len() != 3 {
			return Err(PointParsingError::InvalidAxisNumber);
		}

		let x = parsed[0].parse::<f32>()?;
		let y = parsed[1].parse::<f32>()?;
		let z = parsed[2].parse::<f32>()?;

		Ok(Point3 { x, y, z })
	}
}

#[cfg(test)]
mod test {
	use crate::geometry::{MatrixBuilder, Point3};
	use crate::geometry::projectable::Projectable;
	use crate::geometry::vector::Point2;
	use crate::geometry::vector::point_parsing_error::PointParsingError;

	#[test]
	fn valid_parsing() {
		let point = Point3::try_from("1.0 0 -3.5".to_string()).unwrap();
		assert_eq!(point, Point3{ x: 1.0, y: 0.0, z: -3.5 });
	}

	#[test]
	fn invalid_parsing() {
		assert_eq!(Point3::try_from("0 0".to_string()).unwrap_err(),
		           PointParsingError::InvalidAxisNumber);
		assert_eq!(Point3::try_from("0 0 0 0".to_string()).unwrap_err(),
		           PointParsingError::InvalidAxisNumber);
		assert_eq!(Point3::try_from("0 0 a".to_string()).unwrap_err(),
		           PointParsingError::InvalidFloat);
	}

	#[test]
	fn get_projection() {
		let point = Point3 { x: 1.0, y: 1.0, z: 0.0 };
		let matrix = MatrixBuilder::new()
			.set_height(1)
			.set_width(1)
			.set_fov(90.0)
			.set_view_limit(2.0)
			.set_screen_position(1.0)
			.build();

		let result = point.get_projection(&matrix, 1.0, 240.0, 480.0);

		let expected = Point2 { x: 240.0, y: 480.0 };
		assert!((result.x - expected.x).abs() < 0.001);
		assert!((result.y - expected.y).abs() < 0.001);
	}

}

