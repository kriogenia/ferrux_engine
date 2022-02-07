use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use crate::engine::engine_error::EngineError;
use crate::engine::EngineCanvas;
use crate::environment::Environment;

type Error = EngineError;

pub struct Rust3DEngine {
	input: WinitInputHelper,
	window: Window,
	canvas: EngineCanvas,
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

		let canvas = EngineCanvas::new(&window)?;

		Ok(Self {
			input: WinitInputHelper::new(),
			window,
			canvas,
			environment: Environment::new()
		})
	}

	pub fn draw(&mut self) -> Result<(), EngineError> {
		self.environment.draw(&mut self.canvas);
		self.canvas.render()
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
					self.canvas.resize(size);
				}

				// Update internal state and request a redraw
				self.environment.update();
				self.window.request_redraw();
			}
		Ok(())
	}

}