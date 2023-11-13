use core::{panic, fmt};
use std::fmt::Debug;
use crate::bus::Bus;

pub const PROGRAM_START: u16 = 0x200;

pub struct Cpu {
    vx: [u8; 16],
    pc: u16,
    i: u16,
    prev_pc: u16,
    ret_stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            pc: PROGRAM_START,
            i: 0,
            prev_pc: 0,
            ret_stack: Vec::<u16>::new(),
        }
    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let hi = bus.ram_read_byte(self.pc) as u16;
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
        let instruction:u16 = (hi << 8) | lo;

        println!("Instruction Read instruction {:#X}, hi: {:#X}, lo: {:#X}", instruction, hi, lo);


        let nnn = instruction & 0x0FFF;
        let nn = (instruction & 0x0FF) as u8;
        let n = (instruction & 0x000F) as u8;
        let x = ((instruction & 0x0F00) >> 8) as u8;
        let y = ((instruction & 0x00F0) >> 4) as u8;

        println!("nnn={:?}, nn={:?}, n={:?}, x={}, y={}", nnn, nn, n, x, y);

        if self.prev_pc == self.pc {
            panic!("Please increment PC!");
        }

        self.prev_pc = self.pc;

        match (instruction & 0xF000) >> 12{
            0x0 => {
                match nn {
                    0xE0 => {
                        // clears the screen
                        bus.clear_screen();
                        self.pc += 2;
                    },
                    0xEE => {
                        // returns from subroutine
                        let adrr = self.ret_stack.pop().unwrap();
                        self.pc = adrr;
                    },
                    _=> panic!("Unknown 0x00** instruction {:#X}:{:#X}", self.pc, instruction),
                }
            },
            0x1 => {
                // jumps to address NNN
                self.pc = nnn;
            },
            0x2 => {
                // calls subroutine at address NNN
                self.ret_stack.push(self.pc + 2);
                self.pc = nnn;
            },
            0x3 => {
                // skips next instruction if Reg VX equals NN
                let vx = self.read_reg_vx(x);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6 => {
                // sets Reg VX to NN
                self.write_reg_vx(x, nn);
                self.pc += 2;
            },
            0x7 => {
                // adds NN to Reg VX
                let vx = self.read_reg_vx(x);
                self.write_reg_vx(x, vx.wrapping_add(nn));
                self.pc += 2;
            },
            0x8 => {
                let vx = self.read_reg_vx(x);
                let vy = self.read_reg_vx(y);

                match n {
                    0 => {
                        // sets Reg VX to value of Reg VY
                        self.write_reg_vx(x, vy);
                    },
                    2 => {
                        // sets Reg VX to Reg VX AND Reg VY
                        self.write_reg_vx(x, vx & vy);
                    },
                    3 => {
                        // sets Reg VX to Reg VX XOR Reg VY
                        self.write_reg_vx(x, vx ^ vy);
                    },
                    4 => {
                        // adds Reg VY to Reg VX. Reg VF is set to 1 when there's a carry, and to 0 when there isn't
                        let sum:u16 = vx as u16 + vy as u16;
                        self.write_reg_vx(x, sum as u8);
                        if sum > 0xFF {
                            self.write_reg_vx(0xF, 1);
                        }
                    },
                    5 => {
                        // Reg VY is subtracted from Reg VX. Reg VF is set to 0 when there's a borrow, and 1 when there isn't
                        let diff:i8 = vx as i8 - vy as i8;
                        self.write_reg_vx(x, diff as u8);
                        if diff < 0 {
                            self.write_reg_vx(0xF, 1);
                        } else {
                            self.write_reg_vx(0xF, 0);
                        }
                    },
                    6 => {
                        // Vx=Vy=Vy>>1
                        self.write_reg_vx(0xF, vy & 0x1);
                        self.write_reg_vx(y, vy >> 1);
                        self.write_reg_vx(x, vy >> 1);
                    },
                    _=> panic!("Unknown 0x8** instruction {:#X}:{:#X}", self.pc, instruction),
                }
                self.pc += 2;
            },
            0xD => {
                // draws sprite at (VX, VY) with width 8 and height N
                self.debug_draw_sprite(bus, x, y, n);
                self.pc += 2;
            },
            0xE => {
                match nn {
                    0xA1 => {
                        // if(key()!=VX) then skip the next instruction
                        let key = self.read_reg_vx(x);
                        if bus.key_pressed(key){
                            self.pc += 2
                        } else {
                            self.pc += 4
                        }
                    },
                    0x9E => {
                        panic!();
                        // if(key()==VX) then skip the next instruction
                        let key = self.read_reg_vx(x);
                        if bus.key_pressed(key){
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    _=> panic!("Unknown 0xEX** instruction {:#X}:{:#X}", self.pc, instruction),
                };
            },
            0xA => {
                // sets I to NNN
                self.i = nnn;
                self.pc += 2;
            },
            0xF => {
                // adds Reg VX to I
                let vx = self.read_reg_vx(x);
                self.i += vx as u16;
                self.pc += 2;
            },
            _=> panic!("Unknown instruction {:#X}:{:#X}", self.pc, instruction),
        }
    }

    fn debug_draw_sprite(&mut self, bus: &mut Bus, x:u8, y:u8, height: u8) {
        println!("Drawing sprite at ({}, {})", x, y);
        let mut should_set_vf = false;
        
        for y in 0..height {
            let byte = bus.ram_read_byte(self.i + y as u16);
            if bus.debug_draw_byte(byte, x, y) {
                should_set_vf = true;
            }
        }

        if should_set_vf {
            self.write_reg_vx(0xF, 1);
        } else {
            self.write_reg_vx(0xF, 0);
        }

        bus.present_screen();
    }

    pub fn write_reg_vx(&mut self, index: u8, value: u8) {
        self.vx[index as usize] = value;
    }

    pub fn read_reg_vx(&mut self, index: u8) -> u8{
        self.vx[index as usize]
    }

}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pc: {:#X}, i: {:#X}, vx: {:?}", self.pc, self.i,  self.vx)
    }
}