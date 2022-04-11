mod actors;
pub mod engine;
mod environment;
mod geometry;
mod math;

type FerruxViewport<'a> = ferrux_viewport::viewport::WinitViewport<'a, u32>;