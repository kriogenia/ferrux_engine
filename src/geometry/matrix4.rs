const SIZE: usize = 4;

struct Matrix4 {
	matrix: [[f32; SIZE]; SIZE]
}

impl Default for Matrix4 {
	fn default() -> Self {
		Self {
			matrix: [[0.0; SIZE]; SIZE]
		}
	}
}

impl Matrix4 {
	// TODO take out to builder
	fn projection() -> Self {
		let mut matrix = Self::default().matrix;

		let fNear: f32 = 0.1;
		let fFar: f32 = 1000.0;
		let fFov: f32 = 90.0;
		let fAspectRatio: f32 = 960.0/480.0;
		let fFovRad: f32 = 1.0 / (fFov * 0.5 / 180.0 * 3.14159).tan();

		matrix[0][0] = fAspectRatio * fFovRad;
		matrix[1][1] = fFovRad;
		matrix[2][2] = fFar ( (fFar - fNear));
		matrix[3][2] = (-fFar * fNear) / (fFar - fNear);
		matrix[2][3] = 1.0;

		Self {
			matrix
		}
	}

}