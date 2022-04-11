use crate::FerruxViewport;
use crate::engine::EngineCamera;

/// Entities than can be drawn in the canvas
pub trait Drawable {
    /// Draws the entity in the given [EngineCanvas]
    ///
    /// # Arguments
    /// * `viewport` - Viewport to draw the Drawable
    fn draw(&self, viewport: &mut FerruxViewport, camera: &EngineCamera);
}

/// Updatable entities
pub trait Actor: Drawable {
    /// Updates the entity state
    fn update(&mut self, delta: u128);
}
