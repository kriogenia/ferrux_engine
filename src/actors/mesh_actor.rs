use crate::actors::Actor;
use crate::actors::actor::Drawable;
use crate::engine::EngineCanvas;
use crate::geometry::Mesh;

/// Implementation of an actor with a mesh
pub struct MeshActor {
	mesh: Mesh
}

impl MeshActor {

	/// Creates a new actor with the given mesh
	///
	/// # Arguments
	/// * `mesh` - Mesh of the actor
	pub fn new(mesh: Mesh) -> Self {
		Self {
			mesh
		}
	}

}

impl Drawable for MeshActor {
	fn draw(&self, canvas: &mut EngineCanvas) {
		// todo calculate and draw
	}
}

impl Actor for MeshActor {
	fn update(&mut self) {}
}