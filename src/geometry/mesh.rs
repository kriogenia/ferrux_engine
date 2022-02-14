use crate::geometry::geometry_error::GeometryError;
use crate::geometry::triangle::Triangle;
use crate::geometry::triangle::triangle_parsing_error::TriangleParsingError;
use crate::geometry::vector::point_parsing_error::PointParsingError;

/// Mesh of triangles
#[derive(Debug)]
pub struct Mesh {
	triangles: Vec<Triangle>
}

impl Mesh {

	/// Returns a new empty mesh
	fn new(triangles: Vec<Triangle>) -> Self {
		Self { triangles }
	}

	fn len(&self) -> usize {
		self.triangles.len()
	}

}

impl TryFrom<String> for Mesh {
	type Error = GeometryError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		let mut parsed: Vec<&str> = value.split(";").collect();

		if parsed.len() == 0 {
			return Err(GeometryError::InvalidMesh);
		}

		let mut triangles = Vec::new();

		for slice in parsed {
			let triangle = slice.to_string().try_into()?;
			triangles.push(triangle);
		}

		Ok(Mesh::new(triangles))
	}
}


#[test]
fn valid_parsing() {
	let mesh = Mesh::try_from("1.0 0 -3.5,0 1 2,3 5 2;2 1 0,1 0 2,2 2 2".to_string()).unwrap();
	assert_eq!(mesh.len(), 2);
}

#[test]
fn invalid_parsing() {
	assert_eq!(Mesh::try_from("".to_string()).unwrap_err(), GeometryError::InvalidMesh);
	assert_eq!(Mesh::try_from("a 0 -3.5".to_string()).unwrap_err(),
	           GeometryError::InvalidPoint(TriangleParsingError::InvalidPoint(PointParsingError::InvalidFloat)));
	assert_eq!(Mesh::try_from("0 0".to_string()).unwrap_err(),
	           GeometryError::InvalidTriangle(TriangleParsingError::InvalidPointNumber));
}