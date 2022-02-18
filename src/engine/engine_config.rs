const DEFAULT_TITLE: &str = "Rust 3D Engine";
const DEFAULT_WIDTH: u32 = 960;
const DEFAULT_HEIGHT: u32 = 640;
const DEFAULT_FOV: f32 = 90.0;
const DEFAULT_SCREEN_POSITION: f32 = 0.1;
const DEFAULT_VIEW_LIMIT: f32 = 1000.0;
const DEFAULT_Z_OFFSET: f32 = 3.0;

// TODO check invalid values
pub struct EngineConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub screen_position: f32,
    pub view_limit: f32,
    pub z_offset: f32,
}

impl EngineConfig {
    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn set_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn set_fov(mut self, fov: f32) -> Self {
        self.fov = fov;
        self
    }

    pub fn set_screen_position(mut self, screen_position: f32) -> Self {
        self.screen_position = screen_position;
        self
    }

    pub fn set_view_limit(mut self, view_limit: f32) -> Self {
        self.view_limit = view_limit;
        self
    }

    pub fn set_z_offset(mut self, z_offset: f32) -> Self {
        self.z_offset = z_offset;
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
        }
    }
}
