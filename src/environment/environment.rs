use log::info;
use crate::actors::Actor;
use crate::actors::examples::Triangle2D;
use crate::engine::EngineCanvas;

/// Represents the environment that is drawn in the screen. It holds all the actors to draw.
pub struct Environment {
	actors: Vec<Box<dyn Actor>>
}

impl Environment {

	/// Returns a new instance of the environment
	pub fn new() -> Self {
		info!("Creating environment");
		let tri1 = Triangle2D::new();

		Environment {
			actors: vec![Box::new(tri1)]
		}
	}

	/// Procs an update of all the actors
	pub fn update(&mut self) {
		for actor in &mut self.actors {
			actor.update();
		}
	}

	/// Draws the environment and actors in the given canvas
	///
	/// # Arguments
	/// * `canvas` - [EngineCanvas] to draw the actors on
	pub fn draw(&self, canvas: &mut EngineCanvas) {
		for actor in &self.actors {
			actor.draw(canvas);
		}
	}

}