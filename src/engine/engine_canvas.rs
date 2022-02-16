use bresenham::Bresenham;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::engine::EngineError;
use crate::geometry::{Matrix4, MatrixBuilder};

const DEFAULT_OFFSET: f32 = 3.0;

/// Canvas to manage what is drawn in the screen
pub struct EngineCanvas {
	pixels: Pixels,
	canvas: Vec<Vec<Pixel>>,
	projection_matrix: Matrix4,
	width: usize,
	height: usize,
	z_offset: f32
}

impl EngineCanvas {

	/// Returns a new canvas
	///
	/// # Arguments
	/// * `window` - Borrowed winit [Window] to draw on
	///
	/// # Errors
	/// If no adapter for the GPU is found a [EngineError::AdapterNotFound] is thrown
	///
	pub fn new(window: &Window) -> Result<Self, EngineError> {
		info!("Starting engine canvas");
		let window_size = window.inner_size();
		let width = window_size.width as usize;
		let height = window_size.height as usize;
		info!("Width: {}. Height: {}", &width, &height);

		info!("Starting pixel buffer");
		let pixels = {
			let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
			Pixels::new(window_size.width, window_size.height, surface_texture)
				.map_err(|_| EngineError::AdapterNotFound)?
		};

		let projection_matrix = MatrixBuilder::new()
			.set_screen_position(0.1)
			.set_view_limit(1000.0)
			.set_fov(90.0)
			.set_width(width)
			.set_height(height)
			.build();

		Ok(Self {
			pixels,
			projection_matrix,
			canvas: vec![vec![Pixel::Blank; height as usize]; width as usize],
			width,
			height,
			z_offset: DEFAULT_OFFSET
		})
	}

	/// Width of the screen
	pub fn width(&self) -> usize {
		self.width
	}

	/// Height of the screen
	pub fn height(&self) -> usize {
		self.height
	}

	/// Offset in the Z-axis
	pub fn offset(&self) -> f32 {
		self.z_offset
	}

	fn clear(&mut self) {
		self.canvas = vec![vec![Pixel::Blank; self.height]; self.width]
	}

	fn push_pixel(&mut self, x: usize, y: usize) {
		if x < self.width &&  y < self.height {
			self.canvas[x][y] = Pixel::White;
		}
	}

	/// Draws a line between the two specified points in the canvas
	///
	/// # Arguments
	/// * `start` - Starting point
	/// * `end` - Ending point
	///
	pub fn draw_line(&mut self, start: Point, end: Point) {
		for (x,y) in Bresenham::new(start, end) {
			self.push_pixel(x as usize, y as usize);
		}
	}

	/// Renders the current canvas in the screen and clears it
	///
	/// # Errors
	/// [EngineError::Rendering] if something goes wrong
	///
	pub fn render(&mut self) -> Result<(), EngineError> {
		for (i, pixel) in self.pixels.get_frame().chunks_exact_mut(4).enumerate() {
			pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);

			match self.canvas[i % self.width][i / self.width] {
				Pixel::White => pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]),
				_ => {}
			}
		}

		self.clear();

		self.pixels
			.render()
			.map_err(|e| {
				error!("pixels.render() failed: {:?}", e);
				EngineError::Rendering
			})
	}

	/// Resizes the canvas
	///
	/// # Arguments
	/// * `size` - New size
	///
	pub fn resize(&mut self, size: PhysicalSize<u32>) {
		self.width = size.width as usize;
		self.height = size.height as usize;
		self.clear();
		self.pixels.resize_surface(size.width, size.height);
	}

	pub fn projection_matrix(&self) -> &Matrix4 {
		&self.projection_matrix
	}

}

#[derive(Clone, Copy)]
enum Pixel {
	Blank,
	White
}

type Point = (isize, isize);