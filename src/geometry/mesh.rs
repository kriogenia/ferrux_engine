use std::cell::RefCell;
use std::rc::Rc;

use crate::geometry::geometry_error::GeometryError;
use crate::math::Matrix4;

use super::Rotation;
use super::triangle::Triangle;
use super::vector::Point3;
use super::util::parse_next;

/// Mesh of triangles
///
/// # Properties
/// * `points` - List of points of the mesh
/// * `triangles` - List of triangles of the mesh
///
#[derive(Debug)]
pub struct Mesh {
	/// List of points conforming the mesh
	points: Vec<Rc<RefCell<Point3>>>,
    /// List of triangles conforming the mesh
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    /// Returns a new empty mesh
    ///
    /// # Arguments
    /// * `triangles` - List of triangles of the mesh
    ///
    fn new(points: Vec<Rc<RefCell<Point3>>>, triangles: Vec<Triangle>) -> Self {
        Self { points, triangles }
    }
}

impl Rotation for Mesh {
	fn rotate(&mut self, rotation: &Matrix4) {
		for point in &self.points {
			(*point).borrow_mut().rotate(rotation);
		}
	}
}

impl TryFrom<String> for Mesh {
    type Error = GeometryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
		let mut points = Vec::new();
		let mut triangles = Vec::new();

		// TODO this can be optimized using a single loop

		// Populate point list				// TODO make function of parser
		for line in value.lines() {
			let mut iter =  line.split_whitespace();
			match iter.next() {
				Some("v") => {
					let x = parse_next(iter.next(), line)?;
					let y = parse_next(iter.next(), line)?;
					let z = parse_next(iter.next(), line)?;
					points.push(Rc::new(RefCell::new(Point3 { x, y, z })));
				},
				Some("f") => break,
				_ => {}
			}
		}
		// Populate triangle list
		for line in value.lines() {
			let mut iter =  line.split_whitespace();
			if let Some("f") = iter.next() {
				let first= parse_next::<usize>(iter.next(), line)? - 1;
				let second = parse_next::<usize>(iter.next(), line)? - 1;
				let third = parse_next::<usize>(iter.next(), line)? - 1;
				triangles.push(Triangle(points[first].clone(), points[second].clone(), points[third].clone()));
			}
		}

        Ok(Mesh::new(points, triangles))
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
