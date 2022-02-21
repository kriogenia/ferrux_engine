use crate::engine::EngineRenderer;

/// Entities than can be drawn in the canvas
pub trait Drawable {
    /// Draws the entity in the given [EngineCanvas]
    ///
    /// # Arguments
    /// * `canvas` - Canvas to draw the Drawable
    fn draw(&self, canvas: &mut EngineRenderer);
}

/// Updatable entities
pub trait Actor: Drawable {
    /// Updates the entity state
    fn update(&mut self, delta: u128);
}
