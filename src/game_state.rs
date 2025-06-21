use crate::particles::Particle;
use crate::sound::{play_noise_boom, play_pitched_tone, saw_wave, square_wave};
use crate::space_objects::{BigStar, Star};
use crate::utils::{blend_color, distance_squared};
use minifb::{Key, Window};
use rand::Rng;

pub struct GameState {
    pub ship_x: i32,
    pub ship_y: i32,

    pub stars: Vec<Star>,
    pub big_stars: Vec<BigStar>,
    pub particles: Vec<Particle>,

    pub screen_shake_timer: i32,
    pub shake_timer: f32,
    pub shake_duration: f32,
    pub space_cooldown_timer: f32,

    pub collision_count: usize,
    pub missed_count: usize,
    pub total_seconds: usize,
    pub redemption_flash_timer: f32,
    pub near_stars: Vec<Star>,
    pub far_stars: Vec<Star>,
}

impl GameState {
    pub fn new(width: usize, height: usize, num_stars: usize) -> Self {
        let mut rng = rand::rng();
        let stars: Vec<Star> = (0..num_stars)
            .map(|_| Star {
                x: rng.random_range(-1.0..1.0),
                y: rng.random_range(-1.0..1.0),
                z: rng.random_range(0.1..1.0),
            })
            .collect();

        let near_stars: Vec<Star> = (0..(num_stars / 2))
            .map(|_| Star {
                x: rng.random_range(-1.0..1.0),
                y: rng.random_range(-1.0..1.0),
                z: rng.random_range(0.1..1.0),
            })
            .collect();

        let far_stars: Vec<Star> = (0..(num_stars / 2))
            .map(|_| Star {
                x: rng.random_range(-1.0..1.0),
                y: rng.random_range(-1.0..1.0),
                z: rng.random_range(1.0..2.5),
            })
            .collect();

        Self {
            ship_x: (width / 2) as i32,
            ship_y: (height / 2) as i32,
            stars,
            big_stars: Vec::new(),
            particles: Vec::new(),
            screen_shake_timer: 0,
            shake_timer: 0.0,
            shake_duration: 0.0,
            space_cooldown_timer: 0.0,
            collision_count: 0,
            missed_count: 0,
            total_seconds: 0,
            redemption_flash_timer: 0.0,
            near_stars: near_stars,
            far_stars: far_stars,
        }
    }

    pub fn update(
        &mut self,
        _window: &Window,
        buffer: &mut [u32],
        width: usize,
        height: usize,
        stream_handle: &rodio::OutputStreamHandle,
    ) {
        // Clear screen
        buffer.fill(0);

        // Cooldown timer (e.g. for spacebar)
        if self.space_cooldown_timer > 0.0 {
            self.space_cooldown_timer -= 1.0 / 60.0; // assume 60 FPS
        }

        self.update_starfield(buffer, width, height); // (if you’ve made this)
        let missed: usize = self.update_big_stars(buffer, width, height);
        if missed > 0 {
            self.missed_count += missed;
            crate::sound::play_pitched_tone(50.0, 0.25, crate::sound::square_wave, stream_handle);
            println!(
                "Missed {} star(s)! Total missed: {}",
                missed, self.missed_count
            );
        }

        self.update_particles(buffer, width, height);

        // Update and draw starfield
        let mut rng = rand::rng();

        for star in self.stars.iter_mut() {
            star.z -= 0.01;

            if star.z <= 0.01 {
                star.x = rng.random_range(-1.0..1.0);
                star.y = rng.random_range(-1.0..1.0);
                star.z = 1.0;
            }

            // Project 3D -> 2D
            let sx: isize = (star.x / star.z * width as f32 / 2.0 + width as f32 / 2.0) as isize;
            let sy: isize = (star.y / star.z * height as f32 / 2.0 + height as f32 / 2.0) as isize;

            // Draw if on screen
            if sx >= 0 && sx < width as isize && sy >= 0 && sy < height as isize {
                let brightness: u32 = ((1.0 - star.z) * 255.0) as u32;
                let color: u32 = (brightness << 16) | (brightness << 8) | brightness;
                buffer[sy as usize * width + sx as usize] = color;
            }
        }

        // Count seconds passed (assuming 60 FPS)
        self.total_seconds += 1;

        if self.redemption_flash_timer > 0.0 {
            self.redemption_flash_timer -= 1.0 / 60.0;
        }
    }

    pub fn shake_offsets(&mut self) -> (f32, f32) {
        if self.shake_timer <= 0.0 {
            return (0.0, 0.0);
        }

        let progress: f32 = 1.0 - (self.shake_timer / self.shake_duration);
        let amplitude: f32 = 5.0 * (1.0 - progress);
        let frequency: f32 = 30.0;

        self.shake_timer -= 1.0 / 60.0;

        let shake_x: f32 = (progress * frequency * std::f32::consts::TAU).sin() * amplitude;
        let shake_y: f32 = ((progress * frequency + 0.5) * std::f32::consts::TAU).sin() * amplitude;

        (shake_x, shake_y)
    }

    pub fn check_collisions(
        &mut self,
        stream_handle: &rodio::OutputStreamHandle,
        width: usize,
        height: usize,
        num_particles: usize,
        offset_x: f32,
        offset_y: f32,
    ) {
        let mut rng = rand::rng();
        let ship_x: f32 = self.ship_x as f32 + offset_x;
        let ship_y: f32 = self.ship_y as f32 + offset_y;

        // Step 1: Find indices of stars to explode
        let mut to_explode = vec![];

        for (i, star) in self.big_stars.iter().enumerate() {
            let px: f32 = star.x / star.z * width as f32 / 2.0 + width as f32 / 2.0;
            let py: f32 = star.y / star.z * height as f32 / 2.0 + height as f32 / 2.0;

            if !star.hit && distance_squared(px, py, ship_x, ship_y) < 12.0 {
                to_explode.push(i);
            }
        }

        // Step 2: Mutate stars and GameState after borrow ends
        for i in to_explode {
            let star: &mut BigStar = &mut self.big_stars[i];
            star.hit = true;

            self.reset_shake();
            play_pitched_tone(110.0, 0.25, saw_wave, stream_handle);
            self.collision_count += 1;

            if self.missed_count > 0 {
                self.missed_count -= 1;
                self.redemption_flash_timer = 0.3;
            }
            println!(
                "Collision! Total: {}, Missed reduced to: {}",
                self.collision_count, self.missed_count
            );

            for _ in 0..num_particles {
                let life: u32 = rng.random_range(50..100);
                self.particles.push(crate::particles::Particle {
                    x: ship_x,
                    y: ship_y,
                    vx: rng.random_range(-4.0..4.0),
                    vy: rng.random_range(-4.0..4.0),
                    life,
                    initial_life: life,
                });
            }
        }

        self.big_stars.retain(|s: &BigStar| !s.hit);
    }

    pub fn handle_input(
        &mut self,
        window: &Window,
        stream_handle: &rodio::OutputStreamHandle,
        width: usize,
        height: usize,
        num_particles: usize,
    ) {
        let mut rng = rand::rng();

        // Movement
        if window.is_key_down(Key::Right) && self.ship_x < width as i32 - 3 {
            self.ship_x += 3;
        }
        if window.is_key_down(Key::Left) && self.ship_x > 0 {
            self.ship_x -= 3;
        }
        if window.is_key_down(Key::Down) && self.ship_y < height as i32 - 3 {
            self.ship_y += 3;
        }
        if window.is_key_down(Key::Up) && self.ship_y > 0 {
            self.ship_y -= 3;
        }

        // Spacebar action
        if window.is_key_down(Key::Space) && self.space_cooldown_timer <= 0.0 {
            self.space_cooldown_timer = 0.1;

            play_pitched_tone(500.0, 0.35, square_wave, stream_handle);

            let mut to_explode: Vec<usize> = vec![];

            for (i, star) in self.big_stars.iter_mut().enumerate() {
                let px: f32 = star.x / star.z * width as f32 / 2.0 + width as f32 / 2.0;
                let py: f32 = star.y / star.z * height as f32 / 2.0 + height as f32 / 2.0;

                if !star.hit
                    && distance_squared(px, py, self.ship_x as f32, self.ship_y as f32) < 900.0
                {
                    to_explode.push(i);
                }
            }

            for i in to_explode {
                let star: &mut BigStar = &mut self.big_stars[i];
                star.hit = true;
                self.reset_shake();
                play_noise_boom(0.1, stream_handle);

                self.collision_count += 1;
                println!(
                    "Manual explosion! Total collisions: {}",
                    self.collision_count
                );

                for _ in 0..num_particles {
                    let life: u32 = rng.random_range(50..100);
                    self.particles.push(crate::particles::Particle {
                        x: self.ship_x as f32,
                        y: self.ship_y as f32,
                        vx: rng.random_range(-4.0..4.0),
                        vy: rng.random_range(-4.0..4.0),
                        life,
                        initial_life: life,
                    });
                }
            }
        }
    }

    pub fn update_starfield(&mut self, buffer: &mut [u32], width: usize, height: usize) {
        draw_stars(&mut self.far_stars, buffer, width, height, 0.003);
        draw_stars(&mut self.near_stars, buffer, width, height, 0.01);
    }

    pub fn update_big_stars(&mut self, buffer: &mut [u32], width: usize, height: usize) -> usize {
        let mut rng = rand::rng();
        let mut missed_this_frame = 0;

        // Occasionally spawn a new big star
        if rng.random_ratio(1, 200) {
            self.big_stars.push(BigStar {
                x: rng.random_range(-1.5..1.5),
                y: rng.random_range(-1.5..1.5),
                z: 2.5 + rng.random_range(0.0..1.0),
                hit: false,
                was_missed: false,
            });
        }

        for star in self.big_stars.iter_mut() {
            star.z -= rng.random_range(0.002..0.006);

            // Out of bounds or too close
            let off_screen: bool = star.is_off_screen(width, height);
            let too_close: bool = star.z <= 0.1;

            if off_screen || too_close {
                if !star.hit && !star.was_missed {
                    missed_this_frame += 1;
                    star.was_missed = true;
                }
                // Respawn
                star.x = rng.random_range(-1.5..1.5);
                star.y = rng.random_range(-1.5..1.5);
                star.z = 2.5 + rng.random_range(0.0..1.0);
                star.hit = false;
                star.was_missed = false;
                continue;
            }

            let sx: isize = (star.x / star.z * width as f32 / 2.0 + width as f32 / 2.0) as isize;
            let sy: isize = (star.y / star.z * height as f32 / 2.0 + height as f32 / 2.0) as isize;

            let px: f32 = star.x / star.z * width as f32 / 2.0 + width as f32 / 2.0;
            let py: f32 = star.y / star.z * height as f32 / 2.0 + height as f32 / 2.0;
            let dist2: f32 = distance_squared(px, py, self.ship_x as f32, self.ship_y as f32);
            let proximity_sq: f32 = 900.0;

            if dist2 < proximity_sq && !star.hit {
                // Draw red outline box
                let red: u32 = 0xFF0000;
                for oy in -4..=4 {
                    for ox in -4..=4 {
                        if (ox as isize).abs() == 4 || (oy as isize).abs() == 4 {
                            let bx: isize = sx + ox;
                            let by: isize = sy + oy;
                            if bx >= 0 && bx < width as isize && by >= 0 && by < height as isize {
                                let idx: usize = by as usize * width + bx as usize;
                                buffer[idx] = red;
                            }
                        }
                    }
                }
            }

            // Draw the big star (a 3x3 or blended blob)
            if sx >= 1 && sx < width as isize - 1 && sy >= 1 && sy < height as isize - 1 {
                let color: u32 = blend_color(
                    rng.random_range(150..250),
                    rng.random_range(100..200),
                    rng.random_range(0..250),
                    rng.random_range(0.7..1.0),
                );

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let px: isize = sx + dx;
                        let py: isize = sy + dy;
                        if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                            let idx: usize = py as usize * width + px as usize;
                            buffer[idx] = color;
                        }
                    }
                }
            }
        }

        missed_this_frame
    }

    pub fn update_particles(&mut self, buffer: &mut [u32], width: usize, height: usize) {
        self.particles.retain_mut(|p: &mut Particle| {
            p.x += p.vx;
            p.y += p.vy;
            p.life = p.life.saturating_sub(1);

            if p.x >= 1.0 && p.x < (width - 1) as f32 && p.y >= 1.0 && p.y < (height - 1) as f32 {
                let cx: usize = p.x as usize;
                let cy: usize = p.y as usize;
                let base_r: f32 = 255.0 * (p.life as f32 / p.initial_life as f32);
                let base_g: f32 = 170.0 * (p.life as f32 / p.initial_life as f32);

                let positions = [
                    (0, 0, 1.0),   // center
                    (-1, 0, 0.4),  // left
                    (1, 0, 0.4),   // right
                    (0, -1, 0.4),  // up
                    (0, 1, 0.4),   // down
                    (-1, -1, 0.2), // corners
                    (-1, 1, 0.2),
                    (1, -1, 0.2),
                    (1, 1, 0.2),
                ];

                for (dx, dy, alpha) in positions {
                    let px: isize = cx as isize + dx;
                    let py: isize = cy as isize + dy;
                    if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                        let index: usize = py as usize * width + px as usize;
                        let color: u32 = blend_color(base_r as u8, base_g as u8, 0, alpha as f32);
                        buffer[index] = color;
                    }
                }
            }

            p.life > 0
        });
    }

    pub fn reset_shake(&mut self) {
        self.screen_shake_timer = 10;
        self.shake_duration = 0.5;
        self.shake_timer = self.shake_duration;
    }

    pub fn _shake_offsets(&mut self) -> (f32, f32) {
        if self.shake_timer <= 0.0 {
            return (0.0, 0.0);
        }

        let progress: f32 = 1.0 - (self.shake_timer / self.shake_duration);
        let amplitude: f32 = 5.0 * (1.0 - progress);
        let frequency: f32 = 30.0;

        self.shake_timer -= 1.0 / 60.0;

        (
            (progress * frequency * std::f32::consts::TAU).sin() * amplitude,
            ((progress * frequency + 0.5) * std::f32::consts::TAU).sin() * amplitude,
        )
    }
}

fn draw_stars(stars: &mut Vec<Star>, buffer: &mut [u32], width: usize, height: usize, speed: f32) {
    let mut rng = rand::rng();

    for star in stars.iter_mut() {
        star.z -= speed;

        if star.z <= 0.01 {
            star.x = rng.random_range(-1.0..1.0);
            star.y = rng.random_range(-1.0..1.0);
            star.z = 1.0;
        }

        let sx: isize = (star.x / star.z * width as f32 / 2.0 + width as f32 / 2.0) as isize;
        let sy: isize = (star.y / star.z * height as f32 / 2.0 + height as f32 / 2.0) as isize;

        if sx >= 0 && sx < width as isize && sy >= 0 && sy < height as isize {
            let brightness: u32 = ((1.0 - star.z.min(1.0)) * 255.0) as u32;
            let color: u32 = (brightness << 16) | (brightness << 8) | brightness;
            buffer[sy as usize * width + sx as usize] = color;
        }
    }
}
