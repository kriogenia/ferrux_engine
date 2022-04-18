use std::ops::{Add, Sub};
use crate::geometry::projectable::Projectable;
use crate::geometry::vector::ops::{Cross, Dot, Module, Normalizable};
use crate::geometry::vector::point_parsing_error::PointParsingError;
use crate::geometry::vector::Vector;
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

	/// Applies the offset to the point
	///
	/// # Arguments
	/// * `offset` - Offset to apply
	///
	pub fn apply_offset(mut self, offset: f32) -> Self {
		self.z += offset;
		self
	}

}

// To generate 2D projected version of the point
impl Projectable for Point3 {
	fn get_projection(&self, matrix: &Matrix4, offset: f32) -> Self {
		let (x, y, z) = vector_dot_matrix((self.x, self.y, self.z + offset), matrix);
		Point3 { x, y, z }
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

// Vector addition
impl<'a> Add<&'a Point3> for &'a Point3 {
	type Output = Point3;

	fn add(self, rhs: &'a Point3) -> Self::Output {
		Point3 {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
			z: self.z + rhs.z
		}
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

// Vector dot-product
impl<'a> Dot<&'a Point3> for &'a Point3 {
	type Output = f32;

	fn dot(self, rhs: &'a Point3) -> Self::Output {
		self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
	}
}

// Vector cross-product
impl<'a> Cross<&'a Point3> for &'a Point3 {
	type Output = Point3;

	fn cross(self, rhs: &'a Point3) -> Self::Output {
		Point3 {
			x: self.y * rhs.z - self.z * rhs.y,
			y: self.z * rhs.x - self.x * rhs.z,
			z: self.x * rhs.y - self.y * rhs.x
		}
	}
}

// Vector module
impl<'a> Module for &'a Point3 {
	fn module(self) -> f32 {
		(self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
	}
}

// Vector normal
impl<'a> Normalizable<&'a Point3> for &'a Point3 {
	type Output = Point3;

	fn normal(self) -> Self::Output {
		let module = self.module();
		Point3 {
			x: self.x / module,
			y: self.y / module,
			z: self.z / module
		}
	}
}

impl<'a> Vector<&'a Point3> for &'a Point3 {}

impl Clone for Point3 {
	fn clone(&self) -> Self {
		Self {
			x: self.x,
			y: self.y,
			z: self.z
		}
	}
}

#[cfg(test)]
mod test {
	use crate::geometry::projectable::Projectable;
	use crate::geometry::vector::point_parsing_error::PointParsingError;
	use crate::geometry::vector::Point3;
	use crate::geometry::vector::ops::*;
use crate::math::Matrix4;
	use ferrux_projection_matrix::ProjectionMatrixBuilder;

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
	fn normal() {
		let point = Point3 { x: 0.0, y: 3.0, z: 4.0 };
		let expected = Point3 { x: 0.0, y: 0.6, z: 0.8 };
		assert_eq!(point.normal(), expected);
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
			.set_far(2.0)
			.set_near(1.0)
			.build();

		let result = point.get_projection(&Matrix4::new(matrix), 1.0);

		let expected = Point3 { x: 1.0, y: 1.0, z: 0.0 };
		assert!((result.x - expected.x).abs() < 0.001);
		assert!((result.y - expected.y).abs() < 0.001);
		assert!((result.z - expected.z).abs() < 0.001);
	}

	#[test]
	fn add() {
		let point_a = Point3 { x: 2.0, y: 0.0, z: -10.0, };
		let point_b = Point3 { x: 1.0, y: 1.0, z: -11.0, };
		let expected = Point3 { x: 3.0, y: 1.0, z: -21.0, };

		assert_eq!(&point_a + &point_b, expected);
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

	#[test]
	fn dot() {
		let point_a = Point3 { x: 2.0, y: 0.0, z: -10.0, };
		let point_b = Point3 { x: 1.0, y: 1.0, z: -1.0, };

		assert_eq!(12.0, point_a.dot(&point_b));
	}

}