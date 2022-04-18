pub use triangle3::Triangle3;

mod triangle3;
pub mod triangle_parsing_error;

use crate::geometry::vector::Point3;
use crate::geometry::vector::ops::{Cross, Normalizable};

/// Three-dimensional triangle composed with three [Point3] references
#[derive(Debug)]
pub struct Triangle<'m>(pub &'m Point3, pub &'m Point3, pub &'m Point3);

impl<'m> Triangle<'m> {
/*
    /// Applies a rotation to the triangle
    ///
    /// # Arguments
    /// * `matrix` - Rotation matrix to perform the action
    ///
    pub fn rotate(&mut self, matrix: &Matrix4) {
        self.0
            .translate(vector_dot_matrix((self.0.x, self.0.y, self.0.z), matrix));
        self.1
            .translate(vector_dot_matrix((self.1.x, self.1.y, self.1.z), matrix));
        self.2
            .translate(vector_dot_matrix((self.2.x, self.2.y, self.2.z), matrix));
    }
*/
    /// Returns the normal vector of the triangle
    pub fn normal(&self) -> Point3 {
        let line_0_1 = self.1 - self.0;
        let line_0_2 = self.2 - self.0;
        (&line_0_1).cross(&line_0_2).normal()
    }

    /// Returns a vector of the same plane as the triangle
    pub fn plain_component(&self) -> &'m Point3 {
        self.0
    }

}
/*
impl Projectable for Triangle3 {
    fn get_projection(&self, matrix: &Matrix4, offset: f32) -> Self {
        Triangle3(
            self.0.get_projection(matrix, offset),
            self.1.get_projection(matrix, offset),
            self.2.get_projection(matrix, offset),
        )
    }
}
*/

#[cfg(test)]
mod tests {
    use crate::geometry::triangle::Triangle;
    use crate::geometry::vector::Point3;
    
    #[test]
    fn normal() {
        let point_a = Point3 { x: 0.0, y: 0.0, z: 0.0 };
        let point_b = Point3 { x: 3.0, y: 2.0, z: 1.0 };
        let point_c = Point3 { x: 1.0, y: 2.0, z: 3.0 };
		let triangle = Triangle(&point_a, &point_b, &point_c);

        let expected = Point3 { x: 0.41, y: -0.82, z: 0.41 };
        let normal = triangle.normal();
        
        assert!((normal.x - expected.x).abs() < 0.01);
        assert!((normal.y - expected.y).abs() < 0.01);
        assert!((normal.z - expected.z).abs() < 0.01);
    }

	#[test]
	fn plain_component() {
		let point_a = Point3 { x: 0.0, y: 0.0, z: 0.0 };
        let point_b = Point3 { x: 3.0, y: 2.0, z: 1.0 };
        let point_c = Point3 { x: 1.0, y: 2.0, z: 3.0 };
		let triangle = Triangle(&point_a, &point_b, &point_c);

		assert_eq!(&point_a, triangle.plain_component());
	}

}
