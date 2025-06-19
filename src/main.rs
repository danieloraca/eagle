use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rodio::{OutputStream, Sink};

mod particles;
use particles::Particle;

mod draw_text;
use draw_text::draw_number;

mod sound;
use sound::{play_noise_boom, play_pitched_tone, saw_wave, square_wave};

mod utils;
use utils::{blend_color, distance_squared};

mod space_objects;
use space_objects::{BigStar, Star};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_PARTICLES: usize = 40;
const NUM_STARS: usize = 1000;
const MAX_ESCAPED: usize = 10;
const MAX_HITS: usize = 10;

fn draw_stars() -> Vec<Star> {
    let mut rng = rand::rng();

    // Initialize stars
    let stars: Vec<Star> = (0..NUM_STARS)
        .map(|_| Star {
            x: rng.random_range(-1.0..1.0),
            y: rng.random_range(-1.0..1.0),
            z: rng.random_range(0.1..1.0),
        })
        .collect();

    stars
}

fn main() {
    let mut window = Window::new(
        "Starfield + Ship + Particles - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // let beep = SineWave {
    //     freq: 440.0, // A4 note
    //     sample_rate: 44100,
    //     duration_samples: 44100 / 2, // 0.5 seconds
    //     t: 0,
    // };

    // sink.append(beep);
    // sink.sleep_until_end();

    let mut rng = rand::rng();
    let mut screen_shake_timer = 0;
    let mut shake_timer = 0.0;
    let mut space_cooldown_timer = 0.0f32;
    let mut shake_duration = 0.0;
    let mut collision_count = 0;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Initialize stars
    let mut stars: Vec<Star> = draw_stars();

    let mut big_stars: Vec<BigStar> = Vec::new();

    // Ship initial position
    let mut ship_x = (WIDTH / 2) as i32;
    let mut ship_y = (HEIGHT / 2) as i32;

    // Particle container
    let mut particles: Vec<Particle> = Vec::new();
    let mut missed_count = 0;

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear screen (black)
        buffer.fill(0);
        if space_cooldown_timer > 0.0 {
            space_cooldown_timer -= 1.0 / 60.0; // assuming 60 FPS
        }

        // --- Update starfield: move stars closer and draw them ---
        for star in stars.iter_mut() {
            star.z -= 0.01;

            if star.z <= 0.01 {
                star.x = rng.random_range(-1.0..1.0);
                star.y = rng.random_range(-1.0..1.0);
                star.z = 1.0;
            }

            // Project 3D star coords into 2D screen space
            let sx = (star.x / star.z * WIDTH as f32 / 2.0 + WIDTH as f32 / 2.0) as isize;
            let sy = (star.y / star.z * HEIGHT as f32 / 2.0 + HEIGHT as f32 / 2.0) as isize;

            // Draw star if inside screen bounds
            if sx >= 0 && sx < WIDTH as isize && sy >= 0 && sy < HEIGHT as isize {
                let brightness = ((1.0 - star.z) * 255.0) as u32;
                let color = (brightness << 16) | (brightness << 8) | brightness;
                buffer[sy as usize * WIDTH + sx as usize] = color;
            }
        }

        let mut shake_x = 0;
        let mut shake_y = 0;
        if screen_shake_timer > 0 {
            shake_x = rng.random_range(-2..=2);
            shake_y = rng.random_range(-2..=2);
            screen_shake_timer -= 1;
        }

        // Occasionally add a new big slow star
        if rng.random_ratio(1, 200) {
            big_stars.push(BigStar {
                x: rng.random_range(-1.5..1.5),
                y: rng.random_range(-1.5..1.5),
                z: 2.5 + rng.random_range(0.0..1.0), // starts "farther" than normal stars
                hit: false,
            });
        }

        let mut missed_this_frame = 0;
        // Update and draw big stars
        for big_star in big_stars.iter_mut() {
            big_star.z -= rng.random_range(0.002..0.006); // slower than normal stars

            // Check if the star is off-screen or too close
            let is_off_screen = big_star.is_off_screen(WIDTH, HEIGHT);
            let is_too_close = big_star.z <= 0.1;

            if is_off_screen || is_too_close {
                if !big_star.hit {
                    missed_this_frame += 1; // Increment temporary counter
                }
                // Respawn far away
                big_star.x = rng.random_range(-1.5..1.5);
                big_star.y = rng.random_range(-1.5..1.5);
                big_star.z = 2.5 + rng.random_range(0.0..1.0);
                big_star.hit = false;
                continue; // Skip drawing for respawned stars
            }

            let sx = (big_star.x / big_star.z * WIDTH as f32 / 2.0 + WIDTH as f32 / 2.0) as isize;
            let sy = (big_star.y / big_star.z * HEIGHT as f32 / 2.0 + HEIGHT as f32 / 2.0) as isize;

            let ship_pos_x = ship_x as f32;
            let ship_pos_y = ship_y as f32;

            let px = big_star.x / big_star.z * WIDTH as f32 / 2.0 + WIDTH as f32 / 2.0;
            let py = big_star.y / big_star.z * HEIGHT as f32 / 2.0 + HEIGHT as f32 / 2.0;
            let dist2 = distance_squared(px, py, ship_pos_x, ship_pos_y);
            let proximity_threshold_sq = 900.0;

            if dist2 < proximity_threshold_sq && !big_star.hit {
                let highlight_color = 0xFF0000; // red border

                for oy in -4..=4 {
                    for ox in -4..=4 {
                        if (ox as isize).abs() == 4 || (oy as isize).abs() == 4 {
                            let bx = sx + ox;
                            let by = sy + oy;
                            if bx >= 0 && bx < WIDTH as isize && by >= 0 && by < HEIGHT as isize {
                                let idx = by as usize * WIDTH + bx as usize;
                                buffer[idx] = highlight_color;
                            }
                        }
                    }
                }
            }

            if sx >= 1 && sx < WIDTH as isize - 1 && sy >= 1 && sy < HEIGHT as isize - 1 {
                let color = blend_color(
                    rng.random_range(150..250),
                    rng.random_range(100..200),
                    rng.random_range(0..250),
                    rng.random_range(0.7..1.0),
                );

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let px = sx + dx;
                        let py = sy + dy;
                        if px >= 0 && px < WIDTH as isize && py >= 0 && py < HEIGHT as isize {
                            let index = py as usize * WIDTH + px as usize;
                            buffer[index] = color;
                        }
                    }
                }
            }
        }

        let sx = (ship_x + shake_x) as f32;
        let sy = (ship_y + shake_y) as f32;

        // --- Check collisions with big stars ---
        for big_star in &mut big_stars {
            let px = big_star.x / big_star.z * WIDTH as f32 / 2.0 + WIDTH as f32 / 2.0;
            let py = big_star.y / big_star.z * HEIGHT as f32 / 2.0 + HEIGHT as f32 / 2.0;

            if !big_star.hit && distance_squared(px, py, sx, sy) < 12.0 {
                big_star.hit = true;
                screen_shake_timer = 10;
                shake_duration = 0.5; // in seconds
                shake_timer = shake_duration;
                // play_noise_boom(1.1, &stream_handle);
                // play_pitched_tone(220.0, 0.4, triangle_wave, &stream_handle); // power hum
                play_pitched_tone(110.0, 0.25, saw_wave, &stream_handle); // crunchy explosion

                collision_count += 1;
                println!("Collision! Total collisions: {}", collision_count);

                for _ in 0..NUM_PARTICLES {
                    let life = rng.random_range(50..100);
                    particles.push(Particle {
                        x: sx,
                        y: sy,
                        vx: rng.random_range(-4.0..4.0),
                        vy: rng.random_range(-4.0..4.0),
                        life,
                        initial_life: life,
                    });
                }
            }
        }

        // Update missed_count after processing all big stars
        missed_count += missed_this_frame;
        if missed_this_frame > 0 {
            println!("Missed {} star(s)! Total missed: {}", missed_this_frame, missed_count);
        }

        // Remove hit big stars
        big_stars.retain(|star| !star.hit);

        // --- Move ship with arrow keys ---
        if window.is_key_down(Key::Right) && ship_x < WIDTH as i32 - 3 {
            ship_x += 3;
        }
        if window.is_key_down(Key::Left) && ship_x > 0 {
            ship_x -= 3;
        }
        if window.is_key_down(Key::Down) && ship_y < HEIGHT as i32 - 3 {
            ship_y += 3;
        }
        if window.is_key_down(Key::Up) && ship_y > 0 {
            ship_y -= 3;
        }

        // --- Spawn particles when pressing Space ---
        if window.is_key_down(Key::Space) && space_cooldown_timer <= 0.0 {
            space_cooldown_timer = 0.1;
            play_pitched_tone(500.0, 0.35, square_wave, &stream_handle); // laser zap
            //play_pitched_tone(220.0, 0.4, triangle_wave, &stream_handle); // power hum

            // Explode any nearby big stars
            for big_star in big_stars.iter_mut() {
                let px = big_star.x / big_star.z * WIDTH as f32 / 2.0 + WIDTH as f32 / 2.0;
                let py = big_star.y / big_star.z * HEIGHT as f32 / 2.0 + HEIGHT as f32 / 2.0;

                if !big_star.hit && distance_squared(px, py, ship_x as f32, ship_y as f32) < 900.0 {
                    big_star.hit = true;
                    screen_shake_timer = 10;
                    shake_duration = 0.5;
                    shake_timer = shake_duration;

                    play_noise_boom(0.1, &stream_handle);

                    collision_count += 1;
                    println!("Manual explosion! Total collisions: {}", collision_count);

                    for _ in 0..NUM_PARTICLES {
                        let life = rng.random_range(50..100);
                        particles.push(Particle {
                            x: ship_x as f32,
                            y: ship_y as f32,
                            vx: rng.random_range(-4.0..4.0),
                            vy: rng.random_range(-4.0..4.0),
                            life,
                            initial_life: life,
                        });
                    }
                }
            }

            for _ in 0..NUM_PARTICLES {
                let life = rng.random_range(20..100);
                particles.push(Particle {
                    x: ship_x as f32,
                    y: ship_y as f32,
                    vx: rng.random_range(-2.0..2.0),
                    vy: rng.random_range(-2.0..2.0),
                    life,
                    initial_life: life,
                });
            }
        }

        // --- Update and draw particles ---
        particles.retain_mut(|p| {
            p.x += p.vx;
            p.y += p.vy;
            p.life = p.life.saturating_sub(1);

            if p.x >= 1.0 && p.x < (WIDTH - 1) as f32 && p.y >= 1.0 && p.y < (HEIGHT - 1) as f32 {
                let cx = p.x as usize;
                let cy = p.y as usize;
                let base_r = 255.0 * (p.life as f32 / p.initial_life as f32);
                let base_g = 170.0 * (p.life as f32 / p.initial_life as f32);

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
                    let px = cx as isize + dx;
                    let py = cy as isize + dy;
                    if px >= 0 && px < WIDTH as isize && py >= 0 && py < HEIGHT as isize {
                        let index = py as usize * WIDTH + px as usize;
                        let color = blend_color(base_r as u8, base_g as u8, 0, alpha as f32);
                        buffer[index] = color;
                    }
                }
            }

            p.life > 0
        });

        let mut shake_offset_x = 0.0;
        let mut shake_offset_y = 0.0;

        if shake_timer > 0.0 {
            let progress = 1.0 - (shake_timer / shake_duration);
            let amplitude = 5.0 * (1.0 - progress); // decay
            let frequency = 30.0; // higher = faster oscillation
            shake_offset_x = (progress * frequency * std::f32::consts::TAU).sin() * amplitude;
            shake_offset_y =
                ((progress * frequency + 0.5) * std::f32::consts::TAU).sin() * amplitude;

            shake_timer -= 1.0 / 60.0; // assuming 60 FPS
        }

        let draw_x = (ship_x as f32 + shake_offset_x).round() as isize;
        let draw_y = (ship_y as f32 + shake_offset_y).round() as isize;

        //draw ship
        for dy in -1..=1 {
            for dx in -1..=1 {
                if (dx == 0 || dy == 0)
                    && draw_x + dx >= 0
                    && draw_x + dx < WIDTH as isize
                    && draw_y + dy >= 0
                    && draw_y + dy < HEIGHT as isize
                {
                    let index = (draw_y + dy) as usize * WIDTH + (draw_x + dx) as usize;
                    buffer[index] = 0xFFFFFF; // white cross
                }
            }
        }

        draw_number(&mut buffer, WIDTH, 10, 10, collision_count, 0xffffff, 4); // white color
        draw_number(&mut buffer, WIDTH, 10, 40, missed_count, 0xff0000, 4); // red color

        if missed_count >= MAX_ESCAPED {

        }
        // --- Update window buffer ---
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
