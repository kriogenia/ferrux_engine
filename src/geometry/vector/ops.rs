/// The dot-product operation
pub trait Dot<T = Self> {

	type Output;

	/// Performs the dot-product operation
	fn dot(self, rhs: T) -> Self::Output;

}

/// The cross-product operation
pub trait Cross<T = Self> {

	type Output;

	/// Performs the cross-product operation
	fn cross(self, rhs: T) -> Self::Output;

}

/// Entity with a length module
pub trait Module {

	/// Returns the length module of the vector
	fn module(self) -> f32;

}


pub trait Normalizable<T>
	where T: Module
{

	type Output;

	/// Returns a normalized copy of the vector
	fn normal(self) -> Self::Output;

}