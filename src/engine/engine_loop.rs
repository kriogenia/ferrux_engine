use log::error;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use crate::engine::Rust3DEngine;

pub struct EngineLoop {
	event_loop: EventLoop<()>,
}

impl EngineLoop {

	pub fn new() -> Self {
		EngineLoop {
			event_loop: EventLoop::new()
		}
	}

	pub fn event_loop(&self) -> &EventLoop<()> {
		&self.event_loop
	}

	pub fn run(self, mut engine: Rust3DEngine) {
		self.event_loop.run(move |event, _, control_flow| {
			if let Event::RedrawRequested(_) = event {
				if engine.draw().is_err() {
					*control_flow = ControlFlow::Exit
				}
			}
			engine.update(&event).unwrap_or_else(|e| {
				error!("Error during update: {}", e);
				*control_flow = ControlFlow::Exit;
			});
		});
	}
}