use crate::geometry::vector::Point3;
use crate::geometry::vector::ops::{Cross, Normalizable};
use crate::math::Matrix4;
use super::Projectable;
use std::cell::RefCell;
use std::rc::Rc;

/// Three-dimensional triangle composed with three [Point3] references
#[derive(Debug)]
pub struct Triangle(pub Rc<RefCell<Point3>>, pub Rc<RefCell<Point3>>, pub Rc<RefCell<Point3>>);

impl Triangle {

    /// Returns the normal vector of the triangle
    pub fn normal(&self) -> Point3 {
        let line_0_1 = &*self.1.borrow() - &*self.0.borrow();
        let line_0_2 = &*self.2.borrow() - &*self.0.borrow();
        (&line_0_1).cross(&line_0_2).normal()
    }

    /// Returns a vector of the same plane as the triangle
    pub fn plain_component(&self) -> Point3 {
        self.0.borrow().clone()
    }

}

pub type TriangleProjection = (Point3, Point3, Point3);
impl Projectable<TriangleProjection> for Triangle {
    fn get_projection(&self, matrix: &Matrix4, offset: f32) -> TriangleProjection {
        (
            self.0.borrow().get_projection(matrix, offset),
            self.1.borrow().get_projection(matrix, offset),
            self.2.borrow().get_projection(matrix, offset),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::geometry::triangle::Triangle;
    use crate::geometry::vector::Point3;
    
	macro_rules! wrap {
		($point:tt) => {
			Rc::new(RefCell::new($point))
		};
	}

    #[test]
    fn normal() {
        let point_a = Point3 { x: 0.0, y: 0.0, z: 0.0 };
        let point_b = Point3 { x: 3.0, y: 2.0, z: 1.0 };
        let point_c = Point3 { x: 1.0, y: 2.0, z: 3.0 };
		let triangle = Triangle(wrap!(point_a), wrap!(point_b), wrap!(point_c));

        let expected = Point3 { x: 0.41, y: -0.82, z: 0.41 };
        let normal = triangle.normal();
        
        assert!((normal.x - expected.x).abs() < 0.01);
        assert!((normal.y - expected.y).abs() < 0.01);
        assert!((normal.z - expected.z).abs() < 0.01);
    }
/*
	#[test]
	fn plain_component() {
		let point_a = Point3 { x: 0.0, y: 0.0, z: 0.0 };
        let point_b = Point3 { x: 3.0, y: 2.0, z: 1.0 };
        let point_c = Point3 { x: 1.0, y: 2.0, z: 3.0 };
		let triangle = Triangle(&point_a, &point_b, &point_c);

		assert_eq!(&point_a, triangle.plain_component());
	}

	#[test]
	fn get_projection() {
		let point_a = Point3 { x: 0.0, y: 0.0, z: 0.0 };
        let point_b = Point3 { x: 3.0, y: 2.0, z: 1.0 };
        let point_c = Point3 { x: 1.0, y: 2.0, z: 3.0 };
		let triangle = Triangle(&point_a, &point_b, &point_c);
		
		let matrix = ferrux_projection_matrix::ProjectionMatrixBuilder::new().build();
		let projection = triangle.get_projection(&Matrix4::new(matrix), 1.0);

		dbg!(projection);
	}
*/
}
