use crate::actors::Actor;
use crate::actors::example::BouncingBox;
use crate::engine::EngineCanvas;

pub struct Environment {
	width: usize,
	actors: Vec<Box<dyn Actor>>
}

impl Environment {

	pub fn new(width: usize) -> Self {
		let bbox1 = BouncingBox::new(24, 16, 1, 1);
		let bbox2 = BouncingBox::new(96, 32, 2, 2);

		Environment {
			width,
			actors: vec![Box::new(bbox1), Box::new(bbox2)]
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