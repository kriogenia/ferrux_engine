use std::ops::Index;
use log::info;

const SIZE: usize = 4;

/// Dimension four matrix to use with the vector's display calculation
#[derive(Debug)]
pub struct Matrix4 {
	matrix: [[f32; SIZE]; SIZE]
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

/**********************************************************************/

/// Builder to construct projection matrices
pub struct MatrixBuilder {
	screen_position: f32,
	view_limit: f32,
	fov: f32,
	width: usize,
	height: usize
}

impl MatrixBuilder {

	/// Returns an instance of a builder
	pub fn new() -> Self {
		Self {
			screen_position: 0.0,           // TODO change with defaults
			view_limit: 0.0,
			fov: 0.0,
			width: 0,
			height: 0
		}
	}

	/// Sets the spatial screen position in the z axis
	pub fn set_screen_position(mut self, position: f32) -> Self {
		self.screen_position = position;
		self
	}

	/// Sets the view limit of the rendering in the z axis
	pub fn set_view_limit(mut self, limit: f32) -> Self {
		self.view_limit = limit;
		self
	}

	/// Sets the field of view in grades
	pub fn set_fov(mut self, fov: f32) -> Self {
		self.fov = fov;
		self
	}

	/// Sets the width of the screen
	pub fn set_width(mut self, width: usize) -> Self {
		self.width = width;
		self
	}

	/// Sets the height of the screen
	pub fn set_height(mut self, height: usize) -> Self {
		self.height = height;
		self
	}

	/// Builds the projection matrix derived from the entered parameters and consumes the builder
	pub fn build(self) -> Matrix4 {
		// TODO check valid creation and return error
		info!("Building new projection matrix");
		let mut matrix = Matrix4::default();
		let aspect_ratio = self.width as f32 / self.height as f32;
		info!("Aspect ratio: {}", aspect_ratio);
		let fov_rad: f32 = 1.0 / (self.fov * 0.5 / 180.0 * 3.14159).tan();
		info!("Calculated fov rad: {}", fov_rad);
		let distance = self.view_limit - self.screen_position;
		info!("Calculated distance: {}", distance);

		matrix.matrix[0][0] = aspect_ratio * fov_rad;
		matrix.matrix[1][1] = fov_rad;
		matrix.matrix[2][2] = self.view_limit * distance;
		matrix.matrix[3][2] = (-self.view_limit * self.screen_position) / distance;
		matrix.matrix[2][3] = 1.0;

		matrix
	}

}

// TODO test matrix building