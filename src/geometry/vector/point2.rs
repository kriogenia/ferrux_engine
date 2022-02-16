use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub struct Point2 {
	pub x: f32,
	pub y: f32
}

impl Point2 {

	/// Projects the vector to the given space. It assumes the starting point of the space to be the
	/// origin so it's optimized to work with space like screens that are always positive ranges of
	/// pixels. For example: a 640x480 screen is a (0,0) - (640, 480) space.
	pub fn project(mut self, width: f32, height: f32) -> Self {
		self.x = (self.x + 1.0) * 0.5 * width;
		self.y = (self.y + 1.0) * 0.5 * height;
		self
	}

}

#[cfg(test)]
mod tests {
	use crate::geometry::Point3;
	use crate::geometry::vector::point2::Point2;

	#[test]
	fn project() {
		let x = 240.0;
		let y = 480.0;
		let point = Point2 { x: 0.1, y: -0.5 };
		let expected = Point2 { x: (point.x + 1.0) * 0.5 * x, y: (point.y + 1.0) * 0.5 * y };
		assert_eq!(point.project(x, y), expected);
	}

}