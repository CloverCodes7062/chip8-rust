use chip8::Chip8;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::io::Read;
use std::fs::File;
use std::time::{Instant, Duration, self};
use crate::display::Display;

mod bus;
mod chip8;
mod cpu;
mod display;
mod keyboard;
mod ram;

fn get_chip8_keycode_for(key: Option<Key>) -> Option<u8> {
    match key {
        Some(Key::Key1) => Some(0x1),
        Some(Key::Key2) => Some(0x2),
        Some(Key::Key3) => Some(0x3),
        Some(Key::Key4) => Some(0xC),

        Some(Key::Q) => Some(0x4),
        Some(Key::W) => Some(0x5),
        Some(Key::E) => Some(0x6),
        Some(Key::R) => Some(0xD),

        Some(Key::A) => Some(0x7),
        Some(Key::S) => Some(0x8),
        Some(Key::D) => Some(0x9),
        Some(Key::F) => Some(0xE),

        Some(Key::Z) => Some(0xA),
        Some(Key::X) => Some(0x0),
        Some(Key::C) => Some(0xB),
        Some(Key::V) => Some(0xF),

        _ => None,
    }
}

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("Failed to read rom fil/File Not Found");

    let WIDTH = 640;
    let HEIGHT = 320;

    // A buffer than contains the color of each pixel of the screen in ARGB format
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Rust - Chip8 Emulator | ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let keys_pressed = window.get_keys_pressed(KeyRepeat::Yes);
        let mut key = None;

        if keys_pressed.len() > 0 {
            key = Some(keys_pressed[0]);
        }

        let diff_update_time = Instant::now() - last_key_update_time;
        let chip8_key = get_chip8_keycode_for(key);

        if chip8_key.is_some() || diff_update_time >= Duration::from_millis(250) {
            last_key_update_time = Instant::now();
            chip8.set_key_pressed(chip8_key);
        }

        let diff_update_time = Instant::now() - last_instruction_run_time;
        if diff_update_time > Duration::from_millis(1) {
            chip8.run_instruction();
            last_instruction_run_time = Instant::now();
        }

        let chip8_buffer = chip8.get_display_buffer();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let index = Display::get_index_from_coords(x / 10, y / 10);
                let pixel = chip8_buffer[index];
                let color_pixel = match pixel {
                    0 => 0x0,
                    1 => 0xffffff,
                    _ => unreachable!(),
                };
                buffer[y * WIDTH + x] = color_pixel;
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}