use crate::engine::EngineConfig;
use crate::geometry::vector::Point3;
use crate::math::Matrix4;
use ferrux_projection_matrix::ProjectionMatrixBuilder;

pub struct EngineCamera {
	position: Point3,
	projection_matrix: Matrix4,
	z_offset: f32,
	light: Point3
}

impl EngineCamera {
	pub fn new(config: &EngineConfig) -> Self {
		let matrix = ProjectionMatrixBuilder::new()
			.set_near(config.screen_position)
			.set_far(config.view_limit)
			.set_fov(config.fov)
			.set_width(config.width as usize)
			.set_height(config.height as usize)
			.build();

		Self {
			position: Point3 { x: 0.0, y: 0.0, z: 0.0},
			projection_matrix: Matrix4::new(matrix),
			z_offset: config.z_offset,
			light: config.light_direction.clone(),
		}
	}

	/// Returns the current camera position
	pub fn position(&self) -> &Point3 {
		&self.position
	}

	/// Returns the offset in the Z-axis
	pub fn offset(&self) -> f32 {
		self.z_offset
	}

	/// Returns the projection matrix of the current rendering
	pub fn projection_matrix(&self) -> &Matrix4 {
		&self.projection_matrix
	}

	/// Returns the light vector
	pub fn light(&self) -> &Point3 {
		&self.light
	}

}