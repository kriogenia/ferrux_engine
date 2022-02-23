use ferrux_canvas::canvas::Canvas;
use crate::engine::EngineCamera;

/// Entities than can be drawn in the canvas
pub trait Drawable {
    /// Draws the entity in the given [EngineCanvas]
    ///
    /// # Arguments
    /// * `canvas` - Canvas to draw the Drawable
    fn draw(&self, canvas: &mut dyn Canvas, camera: &EngineCamera);
}

/// Updatable entities
pub trait Actor: Drawable {
    /// Updates the entity state
    fn update(&mut self, delta: u128);
}
