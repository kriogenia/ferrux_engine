use crate::engine::EngineConfig;
use crate::geometry::vector::Point3;
use crate::math::builders::ProjectionMatrixBuilder;
use crate::math::Matrix4;

pub struct EngineCamera {
	position: Point3,
	projection_matrix: Matrix4,
	z_offset: f32,
}

impl EngineCamera {
	pub fn new(config: EngineConfig) -> Self {
		let projection_matrix = ProjectionMatrixBuilder::new()
			.set_screen_position(config.screen_position)
			.set_view_limit(config.view_limit)
			.set_fov(config.fov)
			.set_width(config.width as usize)
			.set_height(config.height as usize)
			.build();

		Self {
			position: Point3 { x: 0.0, y: 0.0, z: 0.0},
			projection_matrix,
			z_offset: config.z_offset,
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

}