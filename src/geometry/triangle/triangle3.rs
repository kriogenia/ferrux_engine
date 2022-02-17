use crate::geometry::{Point3, Projectable};
use crate::geometry::triangle::Triangle2;
use crate::geometry::triangle::triangle_parsing_error::TriangleParsingError;
use crate::math::{Matrix4, vector_dot_matrix};

/// Three-dimensional triangle composed with three [Point3]
#[derive(Debug)]
pub struct Triangle3(
	pub Point3,
	pub Point3,
	pub Point3
);

impl Triangle3 {
	pub fn rotate(&mut self, matrix: &Matrix4) {
		self.0.update(vector_dot_matrix((self.0.x, self.0.y, self.0.z), &matrix));
		self.1.update(vector_dot_matrix((self.1.x, self.1.y, self.1.z), &matrix));
		self.2.update(vector_dot_matrix((self.2.x, self.2.y, self.2.z), &matrix));
	}
}

impl Projectable<Triangle2> for Triangle3 {
	fn get_projection(&self, matrix: &Matrix4, offset: f32, width: f32, height: f32) -> Triangle2 {
		Triangle2(
			self.0.get_projection(matrix, offset, width, height),
			self.1.get_projection(matrix, offset, width, height),
			self.2.get_projection(matrix, offset, width, height),
		)
	}
}

impl TryFrom<String> for Triangle3 {
	type Error = TriangleParsingError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let parsed: Vec<&str> = value.split(",").collect();

		if parsed.len() != 3 {
			return Err(TriangleParsingError::InvalidPointNumber);
		}

		let p1 = parsed[0].to_string().try_into()?;
		let p2 = parsed[1].to_string().try_into()?;
		let p3 = parsed[2].to_string().try_into()?;

		Ok(Triangle3(p1, p2, p3))
	}
}

#[cfg(test)]
mod tests {
	use crate::geometry::{MatrixBuilder, Point3};
	use crate::geometry::projectable::Projectable;
	use crate::geometry::triangle::{Triangle2, Triangle3};
	use crate::geometry::triangle::triangle_parsing_error::TriangleParsingError;
	use crate::geometry::vector::Point2;
	use crate::geometry::vector::point_parsing_error::PointParsingError;

	#[test]
	fn valid_parsing() {
		let triangle = Triangle3::try_from("1.0 0 -3.5,0 1 2,3 5 2".to_string()).unwrap();
		assert_eq!(triangle.0, Point3 { x: 1.0, y: 0.0, z: -3.5 });
		assert_eq!(triangle.1, Point3 { x: 0.0, y: 1.0, z: 2.0 });
		assert_eq!(triangle.2, Point3 { x: 3.0, y: 5.0, z: 2.0 });
	}

	#[test]
	fn invalid_parsing() {
		assert_eq!(Triangle3::try_from("1.0 0 -3.5,0 1 2".to_string()).unwrap_err(),
		           TriangleParsingError::InvalidPointNumber);
		assert_eq!(Triangle3::try_from(".0 0 -3.5,0 1 2,3 5 2,0 0 0".to_string()).unwrap_err(),
		           TriangleParsingError::InvalidPointNumber);
		assert_eq!(Triangle3::try_from("0 0 a,0 1 2,3 5 2".to_string()).unwrap_err(),
		           TriangleParsingError::InvalidPoint(PointParsingError::InvalidFloat));
	}

	#[test]
	fn projection() {
		let triangle = Triangle3(
			Point3 { x: 1.0, y: 1.0, z: 0.0 },
			Point3 { x: -1.0, y: -1.0, z: 0.0 },
			Point3 { x: 0.0, y: 0.5, z: 0.0 }
			);
		let matrix = MatrixBuilder::new()
			.set_height(1)
			.set_width(1)
			.set_fov(90.0)
			.set_view_limit(2.0)
			.set_screen_position(1.0)
			.build();

		let result = triangle.get_projection(&matrix, 1.0, 240.0, 480.0);

		let expected = Triangle2(
			Point2 { x: 240.0, y: 480.0 },
			Point2 { x: 0.0, y: 0.0 },
			Point2 { x: 120.0, y: 360.0 },
		);

		dbg!(&result);

		assert!((result.0.x - expected.0.x).abs() < 0.001);
		assert!((result.0.y - expected.0.y).abs() < 0.001);
		assert!((result.1.x - expected.1.x).abs() < 0.001);
		assert!((result.1.y - expected.1.y).abs() < 0.001);
		assert!((result.2.x - expected.2.x).abs() < 0.001);
		assert!((result.2.y - expected.2.y).abs() < 0.001);
	}

}