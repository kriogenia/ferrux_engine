use crate::geometry::Matrix4;

pub struct Point3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

impl Point3 {

	fn multiply_matrix(&self, matrix: Matrix4) -> Option<Point3> {
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

// TODO test multiplication