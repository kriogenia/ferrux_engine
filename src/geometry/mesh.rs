use crate::geometry::geometry_error::GeometryError;
use crate::geometry::triangle::Triangle3;

/// Mesh of triangles
///
/// # Properties
/// * `triangles` - List of triangles of the mesh
///
#[derive(Debug)]
pub struct Mesh {
    /// List of triangles conforming the mesh
    pub triangles: Vec<Triangle3>,
}

impl Mesh {
    /// Returns a new empty mesh
    ///
    /// # Arguments
    /// * `triangles` - List of triangles of the mesh
    ///
    fn new(triangles: Vec<Triangle3>) -> Self {
        Self { triangles }
    }
}

impl TryFrom<String> for Mesh {
    type Error = GeometryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed: Vec<&str> = value.split(";").collect();

        if parsed.len() == 0 || (parsed.len() == 1 && parsed[0].is_empty()) {
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

#[cfg(test)]
mod tests {
    use super::Mesh;
    use crate::geometry::geometry_error::GeometryError;
    use crate::geometry::triangle::triangle_parsing_error::TriangleParsingError;
    use crate::geometry::vector::point_parsing_error::PointParsingError;

    #[test]
    fn valid_parsing() {
        let mesh = Mesh::try_from("1.0 0 -3.5,0 1 2,3 5 2;2 1 0,1 0 2,2 2 2".to_string()).unwrap();
        assert_eq!(mesh.triangles.len(), 2);
    }

    #[test]
    fn invalid_parsing() {
        assert_eq!(
            Mesh::try_from("".to_string()).unwrap_err(),
            GeometryError::InvalidMesh
        );
        assert_eq!(
            Mesh::try_from("a 0 -3.5,1 0 1,0 0 0".to_string()).unwrap_err(),
            GeometryError::InvalidPoint(TriangleParsingError::InvalidPoint(
                PointParsingError::InvalidFloat
            ))
        );
        assert_eq!(
            Mesh::try_from("0 0".to_string()).unwrap_err(),
            GeometryError::InvalidTriangle(TriangleParsingError::InvalidPointNumber)
        );
    }
}
