use std::fs;
use crate::FerruxViewport;
use crate::actors::examples::get_3d_cube;
use crate::actors::Actor;
use log::{error, info};
use crate::engine::EngineCamera;
use crate::environment::error::EnvironmentError;

/// Represents the environment that is drawn in the screen. It holds all the actors to draw.
pub struct Environment {
    actors: Vec<Box<dyn Actor>>,
}

impl Environment {
    /// Returns a new instance of the environment
    pub(crate) fn new(file: &str) -> Result<Self, EnvironmentError> {
        info!("Creating environment");
        let cube1 = get_3d_cube();

        let _ = fs::read_to_string(file)
          .map_err(|e| {
              error!("{}", e);
              EnvironmentError::BadFile(file)
          })?;

        Ok(Environment {
            actors: vec![Box::new(cube1)],
        })
    }

    /// Draws the environment and actors in the given canvas
    ///
    /// # Arguments
    /// * `canvas` - [EngineCanvas] to draw the actors on
    pub fn draw(&self, viewport: &mut FerruxViewport, camera: &EngineCamera) {
        for actor in &self.actors {
            actor.draw(viewport, camera);
        }
    }

    /// Procs an update of all the actors
    pub fn update(&mut self, delta: u128) {
        for actor in &mut self.actors {
            actor.update(delta);
        }
    }
}
