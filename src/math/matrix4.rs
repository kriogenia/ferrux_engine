use std::ops::Index;

const SIZE: usize = 4;

/// Dimension four matrix to use with the vector's display calculation
#[derive(Debug)]
pub struct Matrix4 {
	pub(crate) matrix: [[f32; SIZE]; SIZE]
}

impl Default for Matrix4 {
	fn default() -> Self {
		Self {
			matrix: [[0.0; SIZE]; SIZE]
		}
	}
}

impl Index<usize> for Matrix4 {
	type Output = [f32];

	fn index(&self, index: usize) -> &Self::Output {
		&self.matrix[index]
	}

}