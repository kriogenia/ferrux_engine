use std::ops::Sub;
use crate::geometry::projectable::Projectable;
use crate::geometry::vector::point_parsing_error::PointParsingError;
use crate::geometry::vector::Point2;
use crate::geometry::vector::vector::Vector;
use crate::math::vector_dot_matrix;
use crate::math::Matrix4;

/// Three-dimensional vector
///
/// # Properties
/// * `x`, `y`, `z` - Axes of the vector
///
#[derive(Debug, PartialEq)]
pub struct Point3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Point3 {

	/// Moves the point to the given position
	///
	/// # Arguments
	/// * `new_pos` - New position like (x, y, z)
	///
	pub fn translate(&mut self, new_pos: (f32, f32, f32)) {
		self.x = new_pos.0;
		self.y = new_pos.1;
		self.z = new_pos.2;
	}

}

// To generate 2D projected version of the point
impl Projectable<Point2> for Point3 {
	fn get_projection(&self, matrix: &Matrix4, offset: f32, width: f32, height: f32) -> Point2 {
		let (x, y, _) = vector_dot_matrix((self.x, self.y, self.z + offset), matrix);
		Point2 { x, y }.project(width, height)
	}
}

// Parsing of the point from a string
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

// Vector subtraction
impl<'a> Sub<&'a Point3> for &'a Point3 {
	type Output = Point3;

	fn sub(self, rhs: &'a Point3) -> Self::Output {
		Point3 {
			x: self.x - rhs.x,
			y: self.y - rhs.y,
			z: self.z - rhs.z
		}
	}
}

// Rest of vector operations
impl<'a> Vector<&'a Point3> for &'a Point3 {
	type SelfOutput = Point3;

	fn cross(self, rhs: &'a Point3) -> Self::SelfOutput {
		Point3 {
			x: self.y * rhs.z - self.z * rhs.y,
			y: self.z * rhs.x - self.x * rhs.z,
			z: self.x * rhs.y - self.y * rhs.x
		}
	}

	fn dot(self) -> Self::SelfOutput {
		todo!()
	}

	fn module(self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}

	fn normalized(self) -> Self::SelfOutput {
		let module = self.module();
		Point3 {
			x: self.x / module,
			y: self.y / module,
			z: self.z / module
		}
	}
}

#[cfg(test)]
mod test {
	use crate::geometry::projectable::Projectable;
	use crate::geometry::vector::point_parsing_error::PointParsingError;
	use crate::geometry::vector::{Point2, Point3, Vector};
	use crate::math::builders::ProjectionMatrixBuilder;

	#[test]
	fn valid_parsing() {
		let point = Point3::try_from("1.0 0 -3.5".to_string()).unwrap();
		assert_eq!(
			point,
			Point3 {
				x: 1.0,
				y: 0.0,
				z: -3.5
			}
		);
	}

	#[test]
	fn invalid_parsing() {
		assert_eq!(
			Point3::try_from("0 0".to_string()).unwrap_err(),
			PointParsingError::InvalidAxisNumber
		);
		assert_eq!(
			Point3::try_from("0 0 0 0".to_string()).unwrap_err(),
			PointParsingError::InvalidAxisNumber
		);
		assert_eq!(
			Point3::try_from("0 0 a".to_string()).unwrap_err(),
			PointParsingError::InvalidFloat
		);
	}

	#[test]
	fn module() {
		let point = Point3 { x: 0.0, y: 3.0, z: 4.0 };
		assert_eq!(point.module(), 5.0);
	}

	#[test]
	fn normalize() {
		let point = Point3 { x: 0.0, y: 3.0, z: 4.0 };
		let expected = Point3 { x: 0.0, y: 0.6, z: 0.8 };
		assert_eq!(point.normalized(), expected);
	}

	#[test]
	fn get_projection() {
		let point = Point3 {
			x: 1.0,
			y: 1.0,
			z: 0.0,
		};
		let matrix = ProjectionMatrixBuilder::new()
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

	#[test]
	fn sub() {
		let point_a = Point3 { x: 2.0, y: 0.0, z: -10.0, };
		let point_b = Point3 { x: 1.0, y: 1.0, z: -11.0, };
		let expected = Point3 { x: 1.0, y: -1.0, z: 1.0, };

		assert_eq!(&point_a - &point_b, expected);
	}

	#[test]
	fn cross() {
		let point_a = Point3 { x: 2.0, y: 0.0, z: -10.0, };
		let point_b = Point3 { x: 1.0, y: 1.0, z: -11.0, };
		let expected = Point3 { x: 10.0, y: 12.0, z: 2.0, };

		assert_eq!(&point_a.cross(&point_b), &expected);
	}

}