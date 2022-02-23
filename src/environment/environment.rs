use ferrux_canvas::canvas::Canvas;
use crate::actors::examples::get_3d_cube;
use crate::actors::Actor;
use log::info;
use crate::engine::EngineCamera;

/// Represents the environment that is drawn in the screen. It holds all the actors to draw.
pub struct Environment {
    actors: Vec<Box<dyn Actor>>,
}

impl Environment {
    /// Returns a new instance of the environment
    pub fn new() -> Self {
        info!("Creating environment");
        //let tri1 = Triangle2D::new();
        let cube1 = get_3d_cube();

        Environment {
            actors: vec![/*Box::new(tri1), */ Box::new(cube1)],
        }
    }

    /// Draws the environment and actors in the given canvas
    ///
    /// # Arguments
    /// * `canvas` - [EngineCanvas] to draw the actors on
    pub fn draw(&self, canvas: &mut dyn Canvas, camera: &EngineCamera) {
        for actor in &self.actors {
            actor.draw(canvas, camera);
        }
    }

    /// Procs an update of all the actors
    pub fn update(&mut self, delta: u128) {
        for actor in &mut self.actors {
            actor.update(delta);
        }
    }
}
