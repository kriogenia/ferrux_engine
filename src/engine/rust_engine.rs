use log::info;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use crate::engine::engine_error::EngineError;
use crate::engine::EngineCanvas;
use crate::environment::Environment;

type Error = EngineError;

/// Graphics engine. It holds the displayed window, the canvas to print in the window and the
/// environment with the meshes to display.
pub struct Rust3DEngine {
	input: WinitInputHelper,
	window: Window,
	canvas: EngineCanvas,
	environment: Environment
}

impl Rust3DEngine {

	/// Returns a working engine with the given attributes
	///
	/// # Arguments
	/// * `title` - Title of the opened window
	/// * `width` - Width of the window to run
	/// * `height` - Height of the window to run
	/// * `event_loop` - Loop of events to control the window. You could use the loop inside a
	/// [EngineLoop]
	///
	/// # Error
	/// In case that no valid adapter for the GPU is found a [EngineError::AdapterNotFound] is thrown
	///
	/// # Example
	/// Create an [EngineLoop] and provided its event loop
	/// ```
	/// let engine_loop = EngineLoop::new();
	///	let mut engine = Rust3DEngine::new("Rust 3D Engine".to_string(), 480, 960,
	///                                   engine_loop.event_loop())?;
	/// ```
	///
	pub fn new(title: String, width: f64, height: f64, event_loop: &EventLoop<()>) -> Result<Self, Error> {
		info!("Building window");
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

	/// Draws the current frame
	///
	/// # Error
	/// If some problem in the rendering happens a [EngineError::Rendering] is thrown
	///
	pub fn draw(&mut self) -> Result<(), EngineError> {
		self.environment.draw(&mut self.canvas);
		self.canvas.render()
	}

	/// Processed the event and makes the pertinent updates
	///
	/// # Arguments
	/// `event` - Event to read
	///
	/// # Error
	/// If the *Escape* key is pressed or the window is a closed a [EngineError::CloseInvocation]
	/// will be returned.
	///
	pub fn update(&mut self, event: &Event<()>) -> Result<(), EngineError> {
			// Handle input events
			if self.input.update(event) {
				// Close events
				if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.quit() {
					info!("Quitting engine");
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