pub fn blend_color(r: u8, g: u8, b: u8, alpha: f32) -> u32 {
    let r = (r as f32 * alpha) as u32;
    let g = (g as f32 * alpha) as u32;
    let b = (b as f32 * alpha) as u32;
    (r << 16) | (g << 8) | b
}

pub fn distance_squared(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x2 - x1).powi(2) + (y2 - y1).powi(2)
}
