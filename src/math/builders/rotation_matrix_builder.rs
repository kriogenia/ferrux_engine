use crate::math::Matrix4;

const DEFAULT_SPEED: f32 = 1.0;
const DEFAULT_THETA: f32 = 0.0;

/// Builder to construct rotation matrices
pub struct RotationMatrixBuilder {
    speed: f32,
    theta: f32,
    axis: RotationAxis,
}

impl RotationMatrixBuilder {
    /// Returns an instance of a builder
    pub fn new() -> Self {
        Self {
            speed: DEFAULT_SPEED,
            theta: DEFAULT_THETA,
            axis: RotationAxis::X,
        }
    }

    pub fn in_axis(mut self, axis: RotationAxis) -> Self {
        self.axis = axis;
        self
    }

    /// Sets the speed of the rotation
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Sets the theta value of the rotation
    pub fn with_theta(mut self, theta: f32) -> Self {
        self.theta = theta;
        self
    }

    /// Builds the rotation matrix derived from the entered parameters and consumes the builder
    pub fn build(&self) -> Matrix4 {
        let mut matrix = Matrix4::default();
        self.axis
            .populate_matrix(&mut matrix, self.theta * self.speed);
        matrix
    }
}

pub enum RotationAxis {
    X,
    #[allow(dead_code)]
    Y,
    Z,
}

impl RotationAxis {
    fn populate_matrix(&self, matrix: &mut Matrix4, magnitude: f32) {
        let cos = magnitude.cos();
        let sin = magnitude.sin();
        match self {
            RotationAxis::X => {
                matrix.matrix[0][0] = 1.0;
                matrix.matrix[1][1] = cos;
                matrix.matrix[1][2] = sin;
                matrix.matrix[2][1] = -sin;
                matrix.matrix[2][2] = cos;
                matrix.matrix[3][3] = 1.0;
            }
            RotationAxis::Y => {
                matrix.matrix[0][0] = 1.0;
                matrix.matrix[1][1] = 1.0;
                matrix.matrix[2][2] = cos;
                matrix.matrix[2][3] = sin;
                matrix.matrix[3][2] = -sin;
                matrix.matrix[3][3] = cos;
            }
            RotationAxis::Z => {
                matrix.matrix[0][0] = cos;
                matrix.matrix[0][1] = sin;
                matrix.matrix[1][0] = -sin;
                matrix.matrix[1][1] = cos;
                matrix.matrix[2][2] = 1.0;
                matrix.matrix[3][3] = 1.0;
            }
        }
    }
}
