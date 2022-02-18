use crate::engine::engine_error::EngineError;
use crate::engine::{EngineCanvas, EngineConfig};
use crate::environment::Environment;
use log::{error, info};
use std::time::SystemTime;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

type Error = EngineError;

/// Graphics engine. It holds the displayed window, the canvas to print in the window and the
/// environment with the meshes to display.
pub struct Rust3DEngine {
    input: WinitInputHelper,
    window: Window,
    canvas: EngineCanvas,
    environment: Environment,
    time: SystemTime,
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
    ///
    /// ```no_run
    /// use rust_3d_engine::engine::{EngineConfig, EngineLoop};
    /// use rust_3d_engine::engine::Rust3DEngine;
    ///
    /// let engine_loop = EngineLoop::new();
    /// let mut engine = Rust3DEngine::new(engine_loop.event_loop(), EngineConfig::default()).unwrap();
    /// ```
    ///
    pub fn new(event_loop: &EventLoop<()>, config: EngineConfig) -> Result<Self, Error> {
        info!("Building window");
        let window = {
            let size = LogicalSize::new(config.width, config.height);
            WindowBuilder::new()
                .with_title(&config.title)
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(event_loop)
                .unwrap()
        };

        let canvas = EngineCanvas::new(&window, config)?;

        Ok(Self {
            input: WinitInputHelper::new(),
            window,
            canvas,
            environment: Environment::new(),
            time: SystemTime::now(),
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
        }

        match self.time.elapsed() {
            Ok(difference) => {
                if difference.as_millis() > 0 {
                    info!("{} FPS", 1000 / difference.as_millis());
                }
                // Update internal state and request a redraw
                self.environment.update(difference.as_millis());
                self.time = SystemTime::now();
                self.window.request_redraw();
            }
            Err(_) => {
                error!("Error calculating delta time");
            }
        }
        Ok(())
    }
}
