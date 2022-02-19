use crate::actors::actor::Drawable;
use crate::actors::Actor;
use crate::engine::EngineCanvas;
use crate::geometry::Mesh;
use crate::geometry::Projectable;
use crate::math::builders::{RotationAxis, RotationMatrixBuilder};

/// Implementation of an actor with a mesh
pub struct MeshActor {
    mesh: Mesh,
}

impl MeshActor {
    /// Creates a new actor with the given mesh
    ///
    /// # Arguments
    /// * `mesh` - Mesh of the actor
    ///
    pub fn new(mesh: Mesh) -> Self {
        Self { mesh }
    }
}

impl Drawable for MeshActor {
    fn draw(&self, canvas: &mut EngineCanvas) {
        let offset = canvas.offset();
        let width = canvas.width() as f32;
        let height = canvas.height() as f32;

        for triangle in &self.mesh.triangles {
            if triangle.normal().z < 0.0 {
                let projection = triangle.get_projection(canvas.projection_matrix(),
                                                         offset, width, height);
                canvas.draw_triangle(projection);
            }
        }
    }
}

impl Actor for MeshActor {
    fn update(&mut self, delta: u128) {
        let matrix_x = RotationMatrixBuilder::new()
            .in_axis(RotationAxis::X)
            .with_speed(0.02)
            .with_theta(delta as f32 * 0.1)
            .build();
        let matrix_z = RotationMatrixBuilder::new()
            .in_axis(RotationAxis::Z)
            .with_speed(-0.02)
            .with_theta(delta as f32 * 0.1)
            .build();

        for triangle in &mut self.mesh.triangles {
            triangle.rotate(&matrix_x);
            triangle.rotate(&matrix_z)
        }
    }
}
