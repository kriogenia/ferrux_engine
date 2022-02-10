use crate::engine::EngineCanvas;

/// Entities than can be drawn in the canvas
pub trait Drawable {
	/// Draws the entity in the given [EngineCanvas]
	///
	/// # Arguments
	/// * `canvas` - Canvas to draw the Drawable
	fn draw(&self, canvas: &mut EngineCanvas);
}

/// Updatable entities
pub trait Actor: Drawable {
	/// Updates the entity state
	fn update(&mut self);
}