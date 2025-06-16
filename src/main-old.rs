use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_PARTICLES: usize = 40;
const NUM_STARS: usize = 1000;

const FONT: [[u8; 5]; 10] = [
    [0b01110, 0b10001, 0b10011, 0b10101, 0b01110], // 0
    [0b00100, 0b01100, 0b00100, 0b00100, 0b01110], // 1
    [0b01110, 0b10001, 0b00010, 0b00100, 0b11111], // 2
    [0b11110, 0b00001, 0b00110, 0b00001, 0b11110], // 3
    [0b00010, 0b00110, 0b01010, 0b11111, 0b00010], // 4
    [0b11111, 0b10000, 0b11110, 0b00001, 0b11110], // 5
    [0b01110, 0b10000, 0b11110, 0b10001, 0b01110], // 6
    [0b11111, 0b00001, 0b00010, 0b00100, 0b01000], // 7
    [0b01110, 0b10001, 0b01110, 0b10001, 0b01110], // 8
    [0b01110, 0b10001, 0b01111, 0b00001, 0b01110], // 9
];

struct Star {
    x: f32,
    y: f32,
    z: f32,
}

struct BigStar {
    x: f32,
    y: f32,
    z: f32,
    hit: bool,
}

#[derive(Clone)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: u32,
    initial_life: u32,
}

fn blend_color(r: u8, g: u8, b: u8, alpha: f32) -> u32 {
    let r = (r as f32 * alpha) as u32;
    let g = (g as f32 * alpha) as u32;
    let b = (b as f32 * alpha) as u32;
    (r << 16) | (g << 8) | b
}

fn distance_squared(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x2 - x1).powi(2) + (y2 - y1).powi(2)
}

fn play_sound(file_path: &str, stream_handle: &rodio::OutputStreamHandle) {
    if let Ok(sink) = Sink::try_new(stream_handle) {
        if let Ok(file) = File::open(file_path) {
            if let Ok(source) = Decoder::new(BufReader::new(file)) {
                sink.append(source);
                sink.detach();
            }
        }
    }
}

fn draw_digit(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    digit: usize,
    color: u32,
    scale: usize,
) {
    let font = FONT[digit];
    for (dy, row) in font.iter().enumerate() {
        for dx in 0..5 {
            if (row >> (4 - dx)) & 1 == 1 {
                // Draw a scale x scale block
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x + dx * scale + sx;
                        let py = y + dy * scale + sy;
                        if px < width && py < buffer.len() / width {
                            buffer[py * width + px] = color;
                        }
                    }
                }
            }
        }
    }
}

fn draw_number(
    buffer: &mut [u32],
    width: usize,
    x: usize,
    y: usize,
    number: usize,
    color: u32,
    scale: usize,
) {
    let digits: Vec<_> = number.to_string().chars().collect();
    for (i, d) in digits.iter().enumerate() {
        if let Some(digit) = d.to_digit(10) {
            draw_digit(
                buffer,
                width,
                x + i * (5 * scale + scale),
                y,
                digit as usize,
                color,
                scale,
            );
        }
    }
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

    let mut rng = rand::rng();
    let mut screen_shake_timer = 0;
    let mut shake_timer = 0.0;
    let mut shake_duration = 0.0;
    let mut collision_count = 0;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Initialize stars
    let mut stars: Vec<Star> = (0..NUM_STARS)
        .map(|_| Star {
            x: rng.random_range(-1.0..1.0),
            y: rng.random_range(-1.0..1.0),
            z: rng.random_range(0.1..1.0),
        })
        .collect();

    let mut big_stars: Vec<BigStar> = Vec::new();

    // Ship initial position
    let mut ship_x = (WIDTH / 2) as i32;
    let mut ship_y = (HEIGHT / 2) as i32;

    // Particle container
    let mut particles: Vec<Particle> = Vec::new();

    let mut buffer = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear screen (black)
        buffer.fill(0);

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

        // Update and draw big stars
        for big_star in big_stars.iter_mut() {
            big_star.z -= rng.random_range(0.002..0.006); // slower than normal stars

            if big_star.z <= 0.1 {
                // Respawn far away if it gets too close
                big_star.x = rng.random_range(-1.5..1.5);
                big_star.y = rng.random_range(-1.5..1.5);
                big_star.z = 2.5 + rng.random_range(0.0..1.0);
            }

            let sx = (big_star.x / big_star.z * WIDTH as f32 / 2.0 + WIDTH as f32 / 2.0) as isize;
            let sy = (big_star.y / big_star.z * HEIGHT as f32 / 2.0 + HEIGHT as f32 / 2.0) as isize;

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
                play_sound("assets/boom.wav", &stream_handle);

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
        if window.is_key_down(Key::Space) {
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

        let draw_x = ship_x as f32 + shake_offset_x;
        let draw_y = ship_y as f32 + shake_offset_y;

        //draw ship
        for dy in -2..=2 {
            for dx in -2..=2 {
                let px = draw_x as i32 + dx;
                let py = draw_y as i32 + dy;
                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    buffer[py as usize * WIDTH + px as usize] = 0xFFAA00;
                }
            }
        }

        draw_number(&mut buffer, WIDTH, 10, 10, collision_count, 0xffffff, 4); // white color

        // --- Update window buffer ---
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

        // if shake_timer > 0.0 {
        //     let progress = 1.0 - (shake_timer / shake_duration);
        //     let amplitude = (1.0 - progress) * 5.0;
        //     shake_offset_x = rng.gen_range(-amplitude..=amplitude);
        //     shake_offset_y = rng.gen_range(-amplitude..=amplitude);
        //     shake_timer -= 0.016; // assuming ~60 FPS
        // }

        // let draw_ship_x = (ship_x as f32 + shake_offset_x) as isize;
        // let draw_ship_y = (ship_y as f32 + shake_offset_y) as isize;

        // if draw_ship_x >= 0
        //     && draw_ship_x < WIDTH as isize
        //     && draw_ship_y >= 0
        //     && draw_ship_y < HEIGHT as isize
        // {
        //     buffer[draw_ship_y as usize * WIDTH + draw_ship_x as usize] = 0xFFFFFF;
        // }

        // window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
