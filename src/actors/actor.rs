use crate::engine::EngineCanvas;

pub trait Actor {
	fn update(&mut self);
	fn draw(&self, canvas: &mut EngineCanvas);
}