use crate::geometry::point3::Point3;

/// Three-dimensional triangle composed with three [Point3]
pub struct Triangle {
	points: [Point3; 3]
}

impl Triangle {

	/// Builds a new triangle with the given points
	///
	/// # Arguments
	/// * `point_a`, `point_b`, `point_c` - Points of the triangle
	///
	pub fn new(point_a: Point, point_b: Point, point_c: Point) -> Self {
		Self {
			points: [
				Point3 { x: point_a.0, y: point_a.1, z: point_a.2 },
				Point3 { x: point_b.0, y: point_b.1, z: point_b.2 },
				Point3 { x: point_c.0, y: point_c.1, z: point_c.2 }
			]
		}
	}

}

type Point = (f32, f32, f32);