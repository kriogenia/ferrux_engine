use crate::actors::Actor;
use crate::engine::EngineCanvas;
use crate::geometry::Mesh;

pub struct MeshActor {
	mesh: Mesh
}

impl MeshActor {

	pub fn new(mesh: Mesh) -> Self {
		Self {
			mesh
		}
	}

}

impl Actor for MeshActor {
	fn update(&mut self) {}

	fn draw(&self, canvas: &mut EngineCanvas) {
		// todo calculate and draw
	}
}