use crate::geometry::vector::Point3;

const DEFAULT_TITLE: &str = "Rust 3D Engine";
const DEFAULT_WIDTH: u32 = 960;
const DEFAULT_HEIGHT: u32 = 640;
const DEFAULT_FOV: f32 = 90.0;
const DEFAULT_SCREEN_POSITION: f32 = 0.1;
const DEFAULT_VIEW_LIMIT: f32 = 1000.0;
const DEFAULT_Z_OFFSET: f32 = 3.0;
const DEFAULT_LIGHT: Point3 = Point3 { x: 0.0, y: 0.0, z: -1.0 };

// TODO check invalid values
/// Entity holding all the configurable options of the engine
pub struct EngineConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub screen_position: f32,
    pub view_limit: f32,
    pub z_offset: f32,
    pub light_direction: Point3
}

impl EngineConfig {

    /// Sets the window title
    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Sets the window width
    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Sets the window height
    pub fn set_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    /// Sets the field of view
    pub fn set_fov(mut self, fov: f32) -> Self {
        self.fov = fov;
        self
    }

    /// Sets the screen position
    pub fn set_screen_position(mut self, screen_position: f32) -> Self {
        self.screen_position = screen_position;
        self
    }

    /// Sets the max view limit
    pub fn set_view_limit(mut self, view_limit: f32) -> Self {
        self.view_limit = view_limit;
        self
    }

    /// Set the global translation on the Z-axis
    pub fn set_z_offset(mut self, z_offset: f32) -> Self {
        self.z_offset = z_offset;
        self
    }

    /// Set the direction of the global ilumination
    pub fn set_light_direction(mut self, light_direction: Point3) -> Self {
        self.light_direction = light_direction;
        self
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            title: DEFAULT_TITLE.to_string(),
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            fov: DEFAULT_FOV,
            screen_position: DEFAULT_SCREEN_POSITION,
            view_limit: DEFAULT_VIEW_LIMIT,
            z_offset: DEFAULT_Z_OFFSET,
            light_direction: DEFAULT_LIGHT,
        }
    }
}
