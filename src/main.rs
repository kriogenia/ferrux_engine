use ferrux_engine::engine::{EngineConfig, EngineError, EngineLoop, Rust3DEngine};

fn main() -> Result<(), EngineError<'static>> {
    env_logger::init();

    let config = EngineConfig::default()
        .with_title("Ferruxe Engine")
        .with_width(960)
        .with_height(960)
        .with_fov(90.0)
        .with_z_offset(15.0)
		.using_file("resources/spaceship.obj");

    let engine_loop = EngineLoop::new();

    let engine = Rust3DEngine::new(engine_loop.event_loop(), config)?;

    engine_loop.run(engine);

    Ok(())
}
