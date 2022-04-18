use crate::math::Matrix4;

/// Multiplies the vector with the given matrix using the provided Z value
///
/// # Arguments
/// * `vector` - Vector to multiply
/// * `matrix` - Matrix to multiply
///
pub fn vector_dot_matrix(vector: Vector, matrix: &Matrix4) -> Vector {
    let x =
        vector.0 * matrix[0][0] + vector.1 * matrix[1][0] + vector.2 * matrix[2][0] + matrix[3][0];
    let y =
        vector.0 * matrix[0][1] + vector.1 * matrix[1][1] + vector.2 * matrix[2][1] + matrix[3][1];
    let z =
        vector.0 * matrix[0][2] + vector.1 * matrix[1][2] + vector.2 * matrix[2][2] + matrix[3][2];
    let w =
        vector.0 * matrix[0][3] + vector.1 * matrix[1][3] + vector.2 * matrix[2][3] + matrix[3][3];

    if w != 0.0 {
        (x / w, y / w, z / w)
    } else {
        (x, y, z)
    }
}

type Vector = (f32, f32, f32);

#[cfg(test)]
mod tests {
    use ferrux_projection_matrix::ProjectionMatrixBuilder;
    use crate::math::{vector_dot_matrix, Matrix4};

    #[test]
    fn empty_matrix_multiplication() {
        let matrix = Matrix4::default();
        assert_eq!((0.0, 0.0, 0.0), vector_dot_matrix((1.0, 1.0, 1.0), &matrix));
    }

    #[test]
    fn vector_matrix_multiplication() {
        let point = (1.0, 1.0, 1.0);
        let matrix = ProjectionMatrixBuilder::new()
            .set_height(1)
            .set_width(1)
            .set_fov(90.0)
            .set_far(2.0)
            .set_near(1.0)
            .build();

        let result = vector_dot_matrix(point, &Matrix4::new(matrix));
        let expected = (1.0, 1.0, 0.0);
        assert!((result.0 - expected.0).abs() < 0.0001);
        assert!((result.1 - expected.1).abs() < 0.0001);
        assert!((result.2 - expected.2).abs() < 0.0001);
    }
}
