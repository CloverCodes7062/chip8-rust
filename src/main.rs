use std::fs::File;
use std::io::Read;
use chip8::Chip8;
use minifb::{Key, Window, WindowOptions, Scale, ScaleMode, KeyRepeat};

mod ram;
mod chip8;
mod cpu;
mod display;
mod keyboard;
mod bus;

fn main() {
    let mut file = File::open("data/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    println!();
    let WIDTH = 640;
    let HEIGHT = 320;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    for i in buffer.iter_mut() {
        *i = 0xffff0000;
    }
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

    while window.is_open() && !window.is_key_down(Key::Escape) {

        chip8.run_instruction();
        let chip8_buffer = chip8.get_display_buffer();

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
