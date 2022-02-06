use crate::actors::Actor;

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

	fn draw(&self, pixel: &mut [u8], coordinates: (i16, i16)) {
		let x = coordinates.0;
		let y = coordinates.1;

		if x >= self.box_x
			&& x < self.box_x + BOX_SIZE
			&& y >= self.box_y
			&& y < self.box_y + BOX_SIZE {
			pixel.copy_from_slice(&[0x5e, 0x48, 0xe8, 0xff]);
		}
	}
}