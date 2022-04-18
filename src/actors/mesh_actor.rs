use crate::FerruxViewport;
use crate::actors::actor::Drawable;
use crate::actors::Actor;
use crate::engine::EngineCamera;
use crate::geometry::Mesh;
use crate::geometry::Projectable;
//use crate::math::builders::{RotationAxis, RotationMatrixBuilder};
use crate::geometry::vector::ops::{Dot, Normalizable};

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
    fn draw(&self, viewport: &mut FerruxViewport, camera: &EngineCamera) {
        let offset = camera.offset();

        let light = camera.light().normal();
        for triangle in &self.mesh.triangles {
            let plain = triangle.plain_component().clone().apply_offset(offset);
            let normal = triangle.normal();
            if normal.dot(&(&plain - camera.position())) < 0.0 {
                let brightness = (light.dot(&normal) * (u8::MAX as f32)) as u8;
                let color = [255, 255, 255, brightness];

                let projection = triangle.get_projection(camera.projection_matrix(), offset);

				viewport.fill_triangle(
				(projection.0.x, projection.0.y, projection.0.z),
				(projection.1.x, projection.1.y, projection.1.z),
				(projection.2.x, projection.2.y, projection.2.z), 
					&color);

            }
        }
    }
}

impl Actor for MeshActor {
    fn update(&mut self, _delta: u128) {
		/*
        let matrix_x = RotationMatrixBuilder::new()
            .in_axis(RotationAxis::X)
            .with_speed(0.005)
            .with_theta(delta as f32 * 0.1)
            .build();
        let matrix_z = RotationMatrixBuilder::new()
            .in_axis(RotationAxis::Z)
            .with_speed(-0.0025)
            .with_theta(delta as f32 * 0.1)
            .build();

        for triangle in &mut self.mesh.triangles {
            triangle.rotate(&matrix_x);
            triangle.rotate(&matrix_z)
        } */
    }
}
