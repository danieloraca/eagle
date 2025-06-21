use minifb::{Key, Window, WindowOptions};
use rodio::OutputStream;

mod particles;

mod draw_text;
use draw_text::{draw_number, draw_text};

mod sound;
mod utils;

mod space_objects;

mod game_state;
use game_state::GameState;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const NUM_PARTICLES: usize = 40;
const NUM_STARS: usize = 1000;
const MAX_ESCAPED: usize = 10;

fn main() {
    let mut window: Window = Window::new(
        "Starfield + Ship + Particles - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_target_fps(60);

    let mut game: GameState = GameState::new(WIDTH, HEIGHT, NUM_STARS);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut buffer: Vec<u32> = vec![0u32; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        game.update(&window, &mut buffer, WIDTH, HEIGHT, &stream_handle);
        game.check_and_shake(&stream_handle, WIDTH, HEIGHT, NUM_PARTICLES);

        // Remove hit big stars
        game.big_stars.retain(|star| !star.hit);

        // Handle keyboard
        game.handle_input(&window, &stream_handle, WIDTH, HEIGHT, NUM_PARTICLES);

        // --- Update and draw particles ---
        game.update_particles(&mut buffer, WIDTH, HEIGHT);

        let (shake_offset_x, shake_offset_y) = game.shake_offsets();

        let draw_x: isize = (game.ship_x as f32 + shake_offset_x).round() as isize;
        let draw_y: isize = (game.ship_y as f32 + shake_offset_y).round() as isize;

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

        draw_number(
            &mut buffer,
            WIDTH,
            10,
            10,
            game.collision_count,
            0xffffff,
            4,
        ); // white color
        draw_number(&mut buffer, WIDTH, 10, 40, game.missed_count, 0xff0000, 4); // red color

        draw_text(&mut buffer, WIDTH, 10, 580, "eagle", 0xFF00FF00, 2);

        let elapsed_seconds: usize = game.total_seconds / 60;
        draw_text(&mut buffer, WIDTH, 650, 15, "Time(s):", 0xdddddd, 1);
        draw_number(&mut buffer, WIDTH, 700, 10, elapsed_seconds, 0xaaffaa, 3);

        if game.missed_count >= MAX_ESCAPED {
            println!("Too many missed stars — game over!");

            draw_text(&mut buffer, WIDTH, 250, 250, "FAIL!", 0x225599, 16);

            // Update the window so the player can see it
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

            std::thread::sleep(std::time::Duration::from_secs(3));

            break;
        }

        if game.redemption_flash_timer > 0.0 {
            game.flash(&mut buffer, draw_x, draw_y, WIDTH, HEIGHT, "green");
        }

        // --- Update window buffer ---
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
