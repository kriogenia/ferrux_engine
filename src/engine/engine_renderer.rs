use ferrux_canvas::canvas::Canvas;
use ferrux_canvas::canvas::winit::WinitCanvas;
use crate::engine::{EngineConfig, EngineError};
use crate::math::builders::ProjectionMatrixBuilder;
use crate::math::Matrix4;
use log::{error, info};
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::geometry::triangle::Triangle2;

/// Canvas to manage what is drawn in the screen
pub struct EngineRenderer {
    canvas: WinitCanvas,
    projection_matrix: Matrix4,
    z_offset: f32,
}

impl EngineRenderer {
    /// Returns a new canvas
    ///
    /// # Arguments
    /// * `window` - Borrowed winit [Window] to draw on
    ///
    /// # Errors
    /// If no adapter for the GPU is found a [EngineError::AdapterNotFound] is thrown
    ///
    pub fn new(window: &Window, config: EngineConfig) -> Result<Self, EngineError> {
        info!("Starting engine canvas");
        let canvas = WinitCanvas::new(window).map_err(|_| EngineError::AdapterNotFound)?;

        let projection_matrix = ProjectionMatrixBuilder::new()
            .set_screen_position(config.screen_position)
            .set_view_limit(config.view_limit)
            .set_fov(config.fov)
            .set_width(config.width as usize)
            .set_height(config.height as usize)
            .build();

        Ok(Self {
            canvas,
            projection_matrix,
            z_offset: config.z_offset,
        })
    }

    /// Width of the screen
    pub fn width(&self) -> u32 {
        self.canvas.width()
    }

    /// Height of the screen
    pub fn height(&self) -> u32 {
        self.canvas.height()
    }

    /// Offset in the Z-axis
    pub fn offset(&self) -> f32 {
        self.z_offset
    }

    /// Draws the three lines compounding a triangle in the canvas
    ///
    /// # Arguments
    /// `triangle` - 2D Triangle to draw
    ///
    pub fn draw_triangle(&mut self, triangle: Triangle2) {
        self.canvas.draw_triangle((triangle.0.x as u32, triangle.0.y as u32),
                                  (triangle.1.x as u32, triangle.1.y as u32),
                                  (triangle.2.x as u32, triangle.2.y as u32));
    }

    /// Renders the current canvas in the screen and clears it
    ///
    /// # Errors
    /// [EngineError::Rendering] if something goes wrong
    ///
    pub fn render(&mut self) -> Result<(), EngineError> {
        self.canvas.render().map_err(|e| {
            error!("canvas::render failed: {:?}", e);
            EngineError::Rendering
        })?;
        self.canvas.reset_frame();
        Ok(())
    }

    /// Resizes the canvas
    ///
    /// # Arguments
    /// * `size` - New size
    ///
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.canvas.resize(size.width, size.height);
    }

    pub fn projection_matrix(&self) -> &Matrix4 {
        &self.projection_matrix
    }

}