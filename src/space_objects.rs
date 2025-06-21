pub struct Star {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct BigStar {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub hit: bool,
    pub was_missed: bool,
}

impl BigStar {
    pub fn is_off_screen(&self, width: usize, height: usize) -> bool {
        // Project 3D coordinates to 2D screen space
        let sx: f32 = self.x / self.z * width as f32 / 2.0 + width as f32 / 2.0;
        let sy: f32 = self.y / self.z * height as f32 / 2.0 + height as f32 / 2.0;
        // Check if projected coordinates are outside screen bounds
        sx < 0.0 || sx > width as f32 || sy < 0.0 || sy > height as f32
    }
}
