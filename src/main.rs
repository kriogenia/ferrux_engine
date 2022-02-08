use rust_3d_engine::engine::{EngineError, EngineLoop, Rust3DEngine};

/// Default width for the window
const WIDTH: u32 = 960;
/// Default height for the window
const HEIGHT: u32 = 640;

fn main() -> Result<(), EngineError> {
	env_logger::init();

	let engine_loop = EngineLoop::new();

	let title = "Rust 3D Engine".to_string();
	let engine = Rust3DEngine::new(title,
	                                   WIDTH as f64,
	                                   HEIGHT as f64,
	                                   engine_loop.event_loop())?;

	engine_loop.run(engine);

	Ok(())
}