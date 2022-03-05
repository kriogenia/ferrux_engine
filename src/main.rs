use rust_3d_engine::engine::{EngineConfig, EngineError, EngineLoop, Rust3DEngine};

fn main() -> Result<(), EngineError> {
    env_logger::init();

    let config = EngineConfig::default()
        .set_title("rust_3d_engine".to_string())
        .set_width(960)
        .set_height(960)
        .set_fov(90.0)
        .set_z_offset(5.0);

    let engine_loop = EngineLoop::new();

    let engine = Rust3DEngine::new(engine_loop.event_loop(), config)?;

    engine_loop.run(engine);

    Ok(())
}
