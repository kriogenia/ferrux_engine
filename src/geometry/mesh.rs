use crate::geometry::triangle::Triangle;

/// Mesh of triangles
pub struct Mesh {
	triangles: Vec<Triangle>
}

impl Mesh {

	/// Returns a new empty mesh
	fn new() -> Self {
		Self {
			triangles: vec![]
		}
	}

}