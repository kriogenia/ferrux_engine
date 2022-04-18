pub use point3::Point3;

pub(crate) mod ops;
mod point3;

use crate::geometry::vector::ops::{Dot, Module, Normalizable};
use std::ops::{Add, Sub};

/// N-dimensional vectors. It should be implemented with the vector reference if the intention is
/// to not consume the vector. Implementations of this trait are **Point2** and **Point3**.
///
/// Operations of certain dimension vectors (like the cross-product of R3) are not forced with this
/// trait
///
pub trait Vector<T = Self>: Add<T> + Sub<T> + Dot<T> + Normalizable<T>
where
    T: Module,
{
}
