use crate::math::Matrix4;

/// Builder to construct rotation matrices
pub struct RotationMatrixBuilder {
	theta: f32
}

impl RotationMatrixBuilder {

	/// Returns an instance of a builder
	pub fn new() -> Self {
		Self {
			theta: 0.0
		}
	}

	/// Sets the theta value of the rotation
	pub fn with_theta(mut self, theta: f32) -> Self {
		self.theta = theta;
		self
	}

	/// Builds the rotation matrix derived from the entered parameters and consumes the builder
	pub fn build(&self) -> Matrix4 {
		let mut matrix = Matrix4::default();
		matrix.matrix[0][0] = 1.0;
		matrix.matrix[1][1] = (self.theta * 0.5).cos();
		matrix.matrix[1][2] = (self.theta * 0.5).sin();
		matrix.matrix[2][1] = -(self.theta * 0.5).sin();
		matrix.matrix[2][2] = (self.theta * 0.5).cos();
		matrix.matrix[3][3] = 1.0;
		matrix
	}

}