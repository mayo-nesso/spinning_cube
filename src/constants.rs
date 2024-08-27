pub const CUBE_WIDTH: usize = 25;
pub const HALF_CUBE_WIDTH: usize = (CUBE_WIDTH as f32 / 2.0) as usize;
pub const CANVAS_WIDTH: usize = 80;
pub const CANVAS_HEIGHT: usize = 40;
pub const ASPECT_RATIO: f32 = CANVAS_WIDTH as f32 / CANVAS_HEIGHT as f32;
pub const BACKGROUND_ASCII_CODE: char = ' ';
pub const DISTANCE_FROM_CAMERA: f32 = 53.0 + HALF_CUBE_WIDTH as f32;
pub const PROJECTION_SCALE: f32 = DISTANCE_FROM_CAMERA / 2.0;
pub const RESOLUTION_STEP: f32 = 0.6;


