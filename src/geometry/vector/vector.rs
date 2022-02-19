use std::ops::Sub;

/// N-dimensional vectors. It should implemented with the vector reference if the intention is
/// to not consume the vector. Implementation of this trait are **Point2** and **Point3**.
pub trait Vector<T = Self>
	: Sub<T>
{

	/// Expected output of the operations returning a Vector. It's not assumed and should be declared
	/// to allow de-referencing. For example, the implementation of Vector for Point3 is implemented
	/// for &Point3 but the return value of the operations is Point3.
	type SelfOutput;

	/// Performs the cross-product with a Vector of the same type
	fn cross(self, rhs: T) -> Self::SelfOutput;

	/// Performs the dot-product with a Vector of the same type
	fn dot(self) -> Self::SelfOutput;

	/// Returns the length module of the vector
	fn module(self) -> f32;

	/// Returns a normalized copy of the vector
	fn normalized(self) -> Self::SelfOutput;

}
