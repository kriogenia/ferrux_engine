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

		if points.is_empty() || triangles.is_empty() {
			Err(GeometryError::EmptyMesh)
		} else {
			Ok(Mesh::new(points, triangles))
		}

    }
}

#[cfg(test)]
mod tests {
    use super::Mesh;
    use crate::geometry::geometry_error::GeometryError;

    #[test]
    fn valid_parsing() {
        let mesh = Mesh::try_from("
			v 1.0 0.0 0.0
			v 0.0 1.0 0.0
			v 0.0 0.0 1.0
			v 0.0 0.0 0.0
			f 1 2 3
			f 2 3 4
			f 3 4 1
			f 4 1 2
			".to_string()).unwrap();
        assert_eq!(mesh.triangles.len(), 4);
		assert_eq!(mesh.triangles[0].1, mesh.triangles[1].0);
		assert_eq!(mesh.triangles[1].1, mesh.triangles[2].0);
		assert_eq!(mesh.triangles[2].1, mesh.triangles[3].0);
		assert_eq!(mesh.triangles[3].1, mesh.triangles[0].0);
    }

    #[test]
    fn invalid_parsing() {
        assert_eq!(
            Mesh::try_from("".to_string()).unwrap_err(),
            GeometryError::EmptyMesh
        );
        assert_eq!(
            Mesh::try_from("v 1.0 0.0 a".to_string()).unwrap_err(),
            GeometryError::WrongNumber("v 1.0 0.0 a".to_string())
        );
        assert_eq!(
            Mesh::try_from("v 0 0".to_string()).unwrap_err(),
            GeometryError::MissingValue("v 0 0".to_string())
        );
    }
}
