use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use crate::engine::engine_error::EngineError;

type Error = EngineError;

pub struct Rust3DEngine {
	input: WinitInputHelper,
	window: Window,
	pixels: Pixels,
	world: World
}

impl Rust3DEngine {

	pub fn new(title: String, width: f64, height: f64, event_loop: &EventLoop<()>) -> Result<Self, Error> {
		let window = {
			let size = LogicalSize::new(width, height);
			WindowBuilder::new()
				.with_title(title)
				.with_inner_size(size)
				.with_min_inner_size(size)
				.build(event_loop)
				.unwrap()
		};

		let pixels = {
			let window_size = window.inner_size();
			let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
			Pixels::new(window_size.width, window_size.height, surface_texture)
				.map_err(|_| EngineError::AdapterNotFound)?
		};

		Ok(Self {
			input: WinitInputHelper::new(),
			window,
			pixels,
			world: World::new()
		})
	}

	pub fn draw(&mut self) -> Result<(), EngineError> {
		self.world.draw(self.pixels.get_frame());
		self.pixels
			.render()
			.map_err(|e| {
				error!("pixels.render() failed: {:?}", e);
				EngineError::Rendering
			})
	}

	pub fn update(&mut self, event: &Event<()>) -> Result<(), EngineError> {
			// Handle input events
			if self.input.update(event) {
				// Close events
				if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
					return Err(EngineError::CloseInvocation);
				}

				// Resize the window
				if let Some(size) = self.input.window_resized() {
					self.pixels.resize_surface(size.width, size.height);
				}

				// Update internal state and request a redraw
				self.world.update();
				self.window.request_redraw();
			}
		Ok(())
	}

}

const BOX_SIZE: i16 = 64;

struct World {
	box_x: i16,
	box_y: i16,
	velocity_x: i16,
	velocity_y: i16,
}

impl World {
	fn new() -> Self {
		Self {
			box_x: 24,
			box_y: 16,
			velocity_x: 1,
			velocity_y: 1,
		}
	}

	fn update(&mut self) {
		if self.box_x <= 0 || self.box_x + BOX_SIZE > 960 as i16 {
			self.velocity_x *= -1;
		}
		if self.box_y <= 0 || self.box_y + BOX_SIZE > 640 as i16 {
			self.velocity_y *= -1;
		}

		self.box_x += self.velocity_x;
		self.box_y += self.velocity_y;
	}

	fn draw(&self, frame: &mut [u8]) {
		for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
			let x = (i % 960 as usize) as i16;
			let y = (i / 960 as usize) as i16;

			let inside_the_box = x >= self.box_x
				&& x < self.box_x + BOX_SIZE
				&& y >= self.box_y
				&& y < self.box_y + BOX_SIZE;

			let rgba = if inside_the_box {
				[0x5e, 0x48, 0xe8, 0xff]
			} else {
				[0x48, 0xb2, 0xe8, 0xff]
			};

			pixel.copy_from_slice(&rgba);
		}
	}
}