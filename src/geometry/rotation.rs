use crate::math::Matrix4;

/// Defines object that can be rotated in the three axes
pub trait Rotation {
	/// Applies the rotation matrix to the object
	/// 
	/// # Arguments
	/// * `rotation` - Matrix to calculate the rotation value
	/// 
	fn rotate(&mut self, rotation: &Matrix4);
}