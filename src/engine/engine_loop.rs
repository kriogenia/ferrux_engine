use crate::engine::Rust3DEngine;
use log::{error, info};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

/// Loop to run and manage the updating and drawing process of the [Rust3DEngine]
pub struct EngineLoop {
    event_loop: EventLoop<()>
}

impl EngineLoop {
    /// Creates and returns a new [EngineLoop]
    pub fn new() -> Self {
        EngineLoop {
            event_loop: EventLoop::new()
        }
    }

    /// Returns the internal [EventLoop]
    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    /// Starts a new thread to run the provided [Rust3DEngine]
    ///
    /// # Arguments
    /// * `engine` - engine to execute
    ///
    /// # Example
    /// Create a [Rust3DEngine] with a [EngineLoop] and start its execution
    ///
    /// ```no_run
    /// use ferrux_engine::engine::{EngineConfig, EngineLoop};
    /// use ferrux_engine::engine::Rust3DEngine;
    ///
    /// let engine_loop = EngineLoop::new();
    ///  let mut engine = Rust3DEngine::new(engine_loop.event_loop(), EngineConfig::default()).unwrap();
    /// engine_loop.run(engine);
    /// ```
    ///
    pub fn run(self, mut engine: Rust3DEngine<'static>) {
        info!("Starting event loop");
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    info!("The close button was pressed; stopping");
                    *control_flow = ControlFlow::Exit
                }
                Event::MainEventsCleared => {
                    engine.update(&event).unwrap_or_else(|e| {
                        error!("Error during update: {}", e);
                        *control_flow = ControlFlow::Exit;
                    });
                }
                Event::RedrawRequested(_) => {
                    if engine.draw().is_err() {
                        *control_flow = ControlFlow::Exit
                    }
                }
                _ => (),
            }
        });
    }
}
