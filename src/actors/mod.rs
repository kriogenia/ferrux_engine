pub trait Actor {
	fn update(&mut self);
	fn draw(&self, pixel: &mut [u8], coordinates: (i16, i16));
}

pub mod example;
pub mod environment;
