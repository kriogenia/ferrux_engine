use crate::actors::Actor;
use crate::engine::EngineCanvas;

const BOX_SIZE: i16 = 64;

pub struct BouncingBox {
	box_x: i16,
	box_y: i16,
	velocity_x: i16,
	velocity_y: i16,
}

impl BouncingBox {

	pub fn new(box_x: i16, box_y: i16, velocity_x: i16, velocity_y: i16) -> Self {
		Self { box_x, box_y, velocity_x, velocity_y	}
	}

}

impl Actor for BouncingBox {

	fn update(&mut self) {
		if self.box_x <= 0 || self.box_x + BOX_SIZE > 960 as i16 {
			self.velocity_x *= -1;
		}
		if self.box_y <= 0 || self.box_y + BOX_SIZE > 640 as i16 {
			self.velocity_y *= -1;
		}

		self.box_x += self.velocity_x;
		self.box_y += self.velocity_y;
	}

	fn draw(&self, canvas: &mut EngineCanvas) {
		for x in self.box_x..self.box_x + BOX_SIZE {
			for y in self.box_y..self.box_y + BOX_SIZE {
				canvas.push_pixel(x as usize, y as usize);
			}
		}
	}
}