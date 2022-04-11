use crate::geometry::triangle::triangle_parsing_error::TriangleParsingError;
use crate::geometry::triangle::Triangle2;
use crate::geometry::Projectable;
use crate::geometry::vector::Point3;
use crate::geometry::vector::ops::{Cross, Normalizable};
use crate::math::{vector_dot_matrix, Matrix4};

/// Three-dimensional triangle composed with three [Point3]
#[derive(Debug)]
pub struct Triangle3(pub Point3, pub Point3, pub Point3);

impl Triangle3 {

    /// Applies a rotation to the triangle
    ///
    /// # Arguments
    /// * `matrix` - Rotation matrix to perform the action
    ///
    pub fn rotate(&mut self, matrix: &Matrix4) {
        self.0
            .translate(vector_dot_matrix((self.0.x, self.0.y, self.0.z), &matrix));
        self.1
            .translate(vector_dot_matrix((self.1.x, self.1.y, self.1.z), &matrix));
        self.2
            .translate(vector_dot_matrix((self.2.x, self.2.y, self.2.z), &matrix));
    }

    /// Returns the normal vector of the triangle
    pub fn normal(&self) -> Point3 {
        let line_0_1 = &self.1 - &self.0;
        let line_0_2 = &self.2 - &self.0;
        (&line_0_1).cross(&line_0_2).normal()
    }

    /// Returns a vector of the same plane as the triangle
    pub fn plain_component(&self) -> Point3 {
        self.0.clone()
    }

}

impl Projectable<Triangle2> for Triangle3 {
    fn get_projection(&self, matrix: &Matrix4, offset: f32) -> Triangle2 {
        Triangle2(
            self.0.get_projection(matrix, offset),
            self.1.get_projection(matrix, offset),
            self.2.get_projection(matrix, offset),
        )
    }
}

impl TryFrom<String> for Triangle3 {
    type Error = TriangleParsingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parsed: Vec<&str> = value.split(",").collect();

        if parsed.len() != 3 {
            return Err(TriangleParsingError::InvalidPointNumber);
        }

        let p1 = parsed[0].to_string().try_into()?;
        let p2 = parsed[1].to_string().try_into()?;
        let p3 = parsed[2].to_string().try_into()?;

        Ok(Triangle3(p1, p2, p3))
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::projectable::Projectable;
    use crate::geometry::triangle::triangle_parsing_error::TriangleParsingError;
    use crate::geometry::triangle::{Triangle2, Triangle3};
    use crate::geometry::vector::point_parsing_error::PointParsingError;
    use crate::geometry::vector::{Point2, Point3};
    use crate::math::builders::ProjectionMatrixBuilder;

    #[test]
    fn valid_parsing() {
        let triangle = Triangle3::try_from("1.0 0 -3.5,0 1 2,3 5 2".to_string()).unwrap();
        assert_eq!(
            triangle.0,
            Point3 {
                x: 1.0,
                y: 0.0,
                z: -3.5
            }
        );
        assert_eq!(
            triangle.1,
            Point3 {
                x: 0.0,
                y: 1.0,
                z: 2.0
            }
        );
        assert_eq!(
            triangle.2,
            Point3 {
                x: 3.0,
                y: 5.0,
                z: 2.0
            }
        );
    }

    #[test]
    fn invalid_parsing() {
        assert_eq!(
            Triangle3::try_from("1.0 0 -3.5,0 1 2".to_string()).unwrap_err(),
            TriangleParsingError::InvalidPointNumber
        );
        assert_eq!(
            Triangle3::try_from(".0 0 -3.5,0 1 2,3 5 2,0 0 0".to_string()).unwrap_err(),
            TriangleParsingError::InvalidPointNumber
        );
        assert_eq!(
            Triangle3::try_from("0 0 a,0 1 2,3 5 2".to_string()).unwrap_err(),
            TriangleParsingError::InvalidPoint(PointParsingError::InvalidFloat)
        );
    }

    #[test]
    fn normal() {
        let triangle = Triangle3(
            Point3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Point3 {
                x: 3.0,
                y: 2.0,
                z: 1.0,
            },
            Point3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        );
        let expected = Point3 { x: 0.41, y: -0.82, z: 0.41 };
        let normal = triangle.normal();
        println!("{:?}", &normal);
        assert!((normal.x - expected.x).abs() < 0.01);
        assert!((normal.y - expected.y).abs() < 0.01);
        assert!((normal.z - expected.z).abs() < 0.01);
    }


    #[test]
    fn projection() {
        let triangle = Triangle3(
            Point3 {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            Point3 {
                x: -1.0,
                y: -1.0,
                z: 0.0,
            },
            Point3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
        );
        let matrix = ProjectionMatrixBuilder::new()
            .set_height(1)
            .set_width(1)
            .set_fov(90.0)
            .set_view_limit(2.0)
            .set_screen_position(1.0)
            .build();

        let result = triangle.get_projection(&matrix, 1.0, 240.0, 480.0);

        let expected = Triangle2(
            Point2 { x: 240.0, y: 480.0 },
            Point2 { x: 0.0, y: 0.0 },
            Point2 { x: 120.0, y: 360.0 },
        );

        assert!((result.0.x - expected.0.x).abs() < 0.001);
        assert!((result.0.y - expected.0.y).abs() < 0.001);
        assert!((result.1.x - expected.1.x).abs() < 0.001);
        assert!((result.1.y - expected.1.y).abs() < 0.001);
        assert!((result.2.x - expected.2.x).abs() < 0.001);
        assert!((result.2.y - expected.2.y).abs() < 0.001);
    }
}
