use crate::FerruxViewport;
use crate::engine::engine_error::EngineError;
use crate::engine::EngineConfig;
use crate::environment::Environment;
use ferrux_viewport::viewport::ViewportFactory;
use log::{error, info};
use std::time::SystemTime;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;
use crate::engine::engine_camera::EngineCamera;

type Error<'a> = EngineError<'a>;

/// Graphics engine. It holds the displayed window, the canvas to print in the window and the
/// environment with the meshes to display.
pub struct Rust3DEngine {
    input: WinitInputHelper,
    window: Window,
    viewport: FerruxViewport,
    camera: EngineCamera,
    environment: Environment,
    time: SystemTime,
}

impl Rust3DEngine {
    /// Returns a working engine with the given attributes
    ///
    /// # Arguments
    /// * `event_loop` - Loop of events to control the window. You could use the loop inside a
    /// [EngineLoop]
    /// * `config` - Configuration of the engine
    ///
    /// # Error
    /// In case that no valid adapter for the GPU is found a [EngineError::AdapterNotFound] is thrown
    ///
    /// # Example
    /// Create an [EngineLoop] and provided its event loop
    ///
    /// ```no_run
    /// use ferrux_engine::engine::{EngineConfig, EngineLoop};
    /// use ferrux_engine::engine::Rust3DEngine;
    ///
    /// let engine_loop = EngineLoop::new();
    /// let mut engine = Rust3DEngine::new(engine_loop.event_loop(), EngineConfig::default()).unwrap();
    /// ```
    ///
    pub fn new<'a>(event_loop: &EventLoop<()>, config: EngineConfig<'a>) -> Result<Self, Error<'a>> {
        info!("Building window");
        let window = {
            let size = LogicalSize::new(config.width, config.height);
            WindowBuilder::new()
                .with_title(config.title)
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(event_loop)
                .unwrap()
        };
		// TODO extract depth to config
		let viewport = ViewportFactory::winit(&window, 1000).map_err(|e| {
            error!("{:?}", e);
            EngineError::AdapterNotFound
        })?;

        let environment = Environment::new(config.file)?;

        Ok(Self {
            input: WinitInputHelper::new(),
            window,
            viewport,
            camera: EngineCamera::new(&config),
            environment,
            time: SystemTime::now(),
        })
    }

    /// Draws the current frame
    ///
    /// # Error
    /// If some problem in the rendering happens a [EngineError::Rendering] is thrown
    ///
    pub fn draw(&mut self) -> Result<(), EngineError> {
        self.environment.draw(&mut self.viewport, &self.camera);
        self.viewport.render().map_err(|e| {
            error!("{:?}", e);
            EngineError::Rendering
        })?;
        self.viewport.reset_buffer();
        Ok(())
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
                self.viewport.resize(size.width, size.height);
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
