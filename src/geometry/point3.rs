use crate::geometry::{Matrix4, MatrixBuilder};

#[derive(Debug)]
pub struct Point3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Point3 {

	fn multiply_matrix(&self, matrix: &Matrix4) -> Option<Point3> {
		let x = self.x * matrix[0][0] + self.y * matrix[1][0] + self.z * matrix[2][0] + matrix[3][0];
		let y = self.x * matrix[0][1] + self.y * matrix[1][1] + self.z * matrix[2][1] + matrix[3][1];
		let z = self.x * matrix[0][2] + self.y * matrix[1][2] + self.z * matrix[2][2] + matrix[3][2];
		let w = self.x * matrix[0][3] + self.y * matrix[1][3] + self.z * matrix[2][3] + matrix[3][3];

		if w != 0.0 {
			Some(Point3 {
				x: x/w,
				y: y/w,
				z: z/w,
			})
		} else { None }
	}

}

#[test]
fn empty_matrix_multiplication() {
	let point = Point3 { x: 1.0, y: 1.0, z: 1.0 };
	let matrix = Matrix4::default();
	assert!(point.multiply_matrix(&matrix).is_none());
}

#[test]
fn vector_matrix_multiplication() {
	let point = Point3 { x: 1.0, y: 1.0, z: 1.0 };
	let matrix = MatrixBuilder::new()
		.set_height(1)
		.set_width(1)
		.set_fov(90.0)
		.set_view_limit(2.0)
		.set_screen_position(1.0)
		.build();

	let result = point.multiply_matrix(&matrix).unwrap();
	let expected = Point3 { x: 1.0, y: 1.0, z: 0.0 };
	assert!((result.x - expected.x).abs() < 0.0001);
	assert!((result.y - expected.y).abs() < 0.0001);
	assert!((result.z - expected.z).abs() < 0.0001);
}