#[derive(Debug)]

pub struct Ram {
    mem: [u8; 4096],
}

impl Ram {
    pub fn new() -> Self {
        let mut ram = Ram { mem: [0; 4096] };
        let sprites: [[u8; 5]; 16] = [
            [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
            [0x20, 0x60, 0x20, 0x20, 0x70], // 1
            [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
            [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
            [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
            [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
            [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
            [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
            [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
            [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
            [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
            [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
            [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
            [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
            [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
            [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
        ];

        let mut i = 0;
        for sprite in sprites.iter() {
            for byte in sprite.iter() {
                ram.mem[i] = *byte;
                i += 1;
            }
        }

        ram
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8{
        self.mem[address as usize]
    }

    pub fn print_ram(&self) {
        for i in 0..self.mem.len() {
            print!("{} ", self.mem[i]);
        }
        print!("\n");
    }

}
