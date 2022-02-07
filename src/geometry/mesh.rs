use crate::geometry::triangle::Triangle;

pub struct Mesh {
	triangles: Vec<Triangle>
}

impl Mesh {

	fn new() -> Self {
		Self {
			triangles: vec![]
		}
	}

}