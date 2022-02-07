use crate::actors::Actor;
use crate::engine::EngineCanvas;

pub struct Triangle2D {
	point_a: (f32, f32),
	point_b: (f32, f32),
	point_c: (f32, f32)
}

impl Triangle2D {

	pub fn new() -> Self {
		Self {
			point_a: (0.1, 0.1),
			point_b: (0.7, 0.1),
			point_c: (0.4, 0.4)
		}
	}

}

impl Actor for Triangle2D {
	fn update(&mut self) {
		//todo!()
	}

	fn draw(&self, canvas: &mut EngineCanvas) {
		let half_width = canvas.width() as f32 / 2.0;
		let half_height = canvas.height() as f32 / 2.0;
		let points: Vec<(isize, isize)> = vec![self.point_a, self.point_b, self.point_c].into_iter().map(|point| {
			let x = (point.0 * half_width + half_width) as isize;
			let y = (point.1 * half_height + half_height) as isize;
			(x,y)
		}).collect();

		canvas.draw_line(points[0], points[1]);
		canvas.draw_line(points[1], points[2]);
		canvas.draw_line(points[0], points[2]);

	}
}