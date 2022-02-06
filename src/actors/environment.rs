use crate::actors::Actor;
use crate::actors::example::BouncingBox;

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

	pub fn draw(&self, frame: &mut [u8]) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);
			for actor in &self.actors {
				actor.draw(pixel, ((i % self.width) as i16, (i / self.width) as i16));
			}
		}
	}

}