use crate::actors::Actor;
use crate::actors::examples::Triangle2D;
use crate::engine::EngineCanvas;

pub struct Environment {
	actors: Vec<Box<dyn Actor>>
}

impl Environment {

	pub fn new() -> Self {
		let tri1 = Triangle2D::new();

		Environment {
			actors: vec![Box::new(tri1)]
		}
	}

	pub fn update(&mut self) {
		for actor in &mut self.actors {
			actor.update();
		}
	}

	pub fn draw(&self, canvas: &mut EngineCanvas) {
		for actor in &self.actors {
			actor.draw(canvas);
		}
	}

}