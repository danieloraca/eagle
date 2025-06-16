#[derive(Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: u32,
    pub initial_life: u32,
}
