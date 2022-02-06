use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use crate::engine::engine_error::EngineError;
use crate::environment::Environment;

type Error = EngineError;

pub struct Rust3DEngine {
	input: WinitInputHelper,
	window: Window,
	pixels: Pixels,
	environment: Environment
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
			environment: Environment::new(width as usize)
		})
	}

	pub fn draw(&mut self) -> Result<(), EngineError> {
		self.environment.draw(self.pixels.get_frame());
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
				self.environment.update();
				self.window.request_redraw();
			}
		Ok(())
	}

}