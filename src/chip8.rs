use core::panic;
use log::{debug, error, info, warn};
use std::fs::File;
use std::io::Read;
use rand::Rng;
mod cpu_unit;
mod graphics_unit;
mod memory_unit;
mod input_unit;

const CHIP8_FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
    0xF0, 0x80, 0xF0, 0x80, 0x80, //F
];

pub struct Chip8 {
    // total memory: 4K
    memory_unit: memory_unit::MemoryUnit,
    delay_timer: u8,
    // CPU emulator
    cpu_unit: cpu_unit::CpuUnit,
    graphics_unit: graphics_unit::GraphicsUnit,
    pub input_unit: input_unit::InputUnit,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut memory = memory_unit::MemoryUnit::new();
        let cpu = cpu_unit::CpuUnit::new();
        let gpu = graphics_unit::GraphicsUnit::new();
        let input= input_unit::InputUnit::new();

        // load font set
        for i in 1..80 {
            memory.memory[i] = CHIP8_FONTSET[i];
        }

        Chip8 {
            memory_unit: memory,
            delay_timer: 0,
            cpu_unit: cpu,
            graphics_unit: gpu,
            input_unit: input,
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch Opcode
        let pc = self.cpu_unit.pc as usize;
        let opcode =
            (self.memory_unit.memory[pc] as u16) << 8_u8 | self.memory_unit.memory[pc + 1] as u16;
        // Decode Opcode
        let firstnib = opcode & 0xF000;
        // Execute Opcode
        //instructions::InstructionSet::new(self);
        info!("Executing {opcode:x} - {firstnib:x}");

        match firstnib {
            0x0000 => match opcode & 0x000F {
                0x0000 => {
                    // CLS:  Clear the display.
                    self.graphics_unit.screen.fill(0);
                    self.cpu_unit.pc += 2;
                }
                0x000E => {
                    // RET: Return from a subroutine 
                    self.cpu_unit.sp -= 1;
                    self.cpu_unit.pc = self.cpu_unit.stack[self.cpu_unit.sp as usize];
                    self.cpu_unit.pc += 2;
                }
                _ => panic!("Instruction {opcode:x} not implemented"),
            },
            0x1000 => {
                // 1nnn - JP addr: Jump to location nnn.
                // check infinite loop
                //if self.cpu_unit.pc == (opcode & 0x0FFF) {
                //    panic!("Infinite loop detected");
                //}
                self.cpu_unit.pc = opcode & 0x0FFF;
            },
            0x2000 => {
                // 2nnn - CALL addr:  Call subroutine at nnn.
                self.cpu_unit.stack[self.cpu_unit.sp as usize] = self.cpu_unit.pc;
                self.cpu_unit.sp += 1;
                self.cpu_unit.pc = opcode & 0x0FFF;
            }
            0x6000 => {
                // 6xkk - LD Vx, byte: Set Vx = kk.
                let register = (opcode & 0x0F00) >> 8;
                self.cpu_unit.v[register as usize] = (opcode & 0x00FF) as u8;
                self.cpu_unit.pc += 2;
            }
            0x3000 => {
                // 3xkk - SE Vx, byte: Skip next instruction if Vx = kk.
                let register: u16 = (opcode & 0x0F00) >> 8;
                let value: u8 = (opcode & 0x00FF) as u8;
                if self.cpu_unit.v[register as usize] == value {
                    self.cpu_unit.pc += 4;
                } else {
                    self.cpu_unit.pc += 2;
                }
            }
            0xa000 => {
                // Annn - LD I, addr:  Set I = nnn.
                self.cpu_unit.i = opcode & 0x0FFF;
                self.cpu_unit.pc += 2;
            },
            0xc000 => { // Cxkk - RND Vx, byte: Set Vx = random byte AND kk.
                let register = (opcode & 0x0F00) >> 8;
                let random_number: u8 = rand::random::<u8>();
                self.cpu_unit.v[register as usize] = (random_number & 0xFF) & ((opcode & 0x00FF) as u8);
                self.cpu_unit.pc +=2;
            },
            0xd000 => {
                // Dxyn - DRW Vx, Vy, nibble: Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // set x,y from registers
                let x = self.cpu_unit.v[((opcode & 0x0F00) >> 8) as usize] & 63; // wrap around as per specification
                let y = self.cpu_unit.v[((opcode & 0x00F0) >> 4) as usize] & 31;
                debug!("Drawing at x: {x} y: {y}", x = x, y = y);

                let height: u16 = opcode & 0x000F;
                // reset VF collision flag
                self.cpu_unit.v[0xF] = 0;

                for y_p in 0..height {
                    // get the Nth byte of sprite data
                    let sprite = self.memory_unit.memory[(self.cpu_unit.i + y_p) as usize];

                    // check each of the 8 pixels in the sprite
                    for x_p in 0..8_u16 {
                        let pixel = sprite & (0x80 >> x_p);
                        // if the pixel is set, set the flag on
                        if pixel != 0 {
                            let pos = (x as u16) + x_p + (((y as u16) + y_p) * 64);
                            if pos >= 64 * 32 {
                                continue;
                            }
                            if self.graphics_unit.screen[pos as usize] == 1 {
                                self.cpu_unit.v[0xF] = 1;
                            }
                            self.graphics_unit.screen[pos as usize] ^= 1;
                        }
                    }
                }

                self.graphics_unit.draw();
                // debug display
                for i in 0..self.graphics_unit.screen.len() {
                    if i % 64 == 0 {
                        debug!("");
                    }
                    if self.graphics_unit.screen[i] == 0 {
                        debug!("⣿");
                    } else {
                        debug!(" ");
                    }
                }

                self.cpu_unit.pc += 2;
            }
            0xe000 => {
                match opcode & 0x00FF {
                    0x00a1 => { // SKNP Vx: Skip next instruction if key with the value of Vx is not pressed.
                         let register = ((opcode & 0x0F00) >> 8) as u8;

                         if self.input_unit.keypad[self.cpu_unit.v[register as usize] as usize] == 0 {
                            self.cpu_unit.pc += 4;
                         }
                         self.cpu_unit.pc += 2;
                    },
                    _ => panic!("Instruction {opcode:x} not implemented"),
                }
            },
            0x7000 => {
                // 7xkk - ADD Vx, byte: Set Vx = Vx + kk.
                let register = ((opcode & 0x0F00) >> 8) as u8;
                self.cpu_unit.v[register as usize] += (opcode & 0x00FF) as u8;
                self.cpu_unit.pc += 2;
            }, 
            0x8000 => {
                match opcode & 0x00FF { // 
                    0x0002 => { // 8xy2 - AND Vx, Vy: Set Vx = Vx AND Vy.
                        let register_x = ((opcode & 0x0F00) >> 8) as u8;
                        let register_y = ((opcode & 0x00F0) >> 4) as u8;
                        self.cpu_unit.v[register_x as usize] &= register_y; 
                    }
                    _ => panic!("Instruction {opcode:x} not implemented"),
                }
            },
            0x9000 => {
                // 9xy0 - SNE Vx, Vy: skip next instruction if Vx != Vy
                let register_x:u8 = ((opcode & 0x0F00) >>8) as u8;
                let register_y:u8 = ((opcode & 0x00F0) >>4) as u8;

                if self.cpu_unit.v[register_x as usize] != self.cpu_unit.v[register_y as usize] {
                    self.cpu_unit.pc +=4;
                }
                else{
                    self.cpu_unit.pc += 2;
                }

            },
            0xF000 => {
                match opcode & 0x00FF {
                    0x0007 => { // Fx07 - LD Vx, DT: Set Vx = delay timer value.
                        let register = ((opcode & 0x0F00) >> 8) as u8;
                        self.cpu_unit.v[register as usize] = self.delay_timer;
                        self.cpu_unit.pc +=2;
                    }, 
                    0x0015 => { // Fx15 - LD DT, Vx: Set delay timer = Vx.
                        let register = ((opcode & 0x0F00) >>8) as u8;
                        self.delay_timer = self.cpu_unit.v[register as usize];
                        self.cpu_unit.pc +=2;
                    },
                    0x0029 => { // LD F, Vx: Set I = location of sprite for digit Vx.
                        let register = ((opcode & 0x0F00) >> 8) as u8;
                        self.cpu_unit.i = (self.cpu_unit.v[register as usize] * 0x5) as u16; // multiply by 5 to get the font
                        self.cpu_unit.pc += 2;
                    }
                    0x0033 => { // Fx33 - LD B, Vx: Store BCD representation of Vx in memory locations I, I+1, and I+2.
                        self.memory_unit.memory[self.cpu_unit.i as usize] = self.cpu_unit.v[((opcode & 0x0F00) >> 8) as usize] / 100;
                        self.memory_unit.memory[self.cpu_unit.i as usize +1] = self.cpu_unit.v[((opcode & 0x0F00) >> 8) as usize] / 10 % 10;
                        self.memory_unit.memory[self.cpu_unit.i as usize +2] = self.cpu_unit.v[((opcode & 0x0F00) >> 8) as usize] % 10;
                        self.cpu_unit.pc +=2;
                    },
                    0x0065 => {//Fx65 - LD Vx, [I]: Read registers V0 through Vx from memory starting at location I.
                        let register = ((opcode & 0x0F00) >> 8) as u8;
                        for r in 0..register{
                            self.cpu_unit.v[r as usize] = self.memory_unit.memory[(self.cpu_unit.i as usize) + (r as usize)];
                        }

                        self.cpu_unit.pc += 2;
                    }
                    _ => panic!("Instruction {opcode:x} not implemented"),
                    
                }
            }

            _ => panic!("Instruction {firstnib:x} not implemented"),
        }

        // Update timer
        if self.delay_timer > 0{
            self.delay_timer -=1;
        }

        // print all registers
        /*for i in 0..16 {
            print!("Register {i}: {v}", i = i, v = self.cpu_unit.v[i]);
        }*/
    }

    pub fn load_rom(&mut self, filename: &str) {
        let rom = File::open(filename);
        let mut buffer = Vec::new();
        rom.unwrap().read_to_end(&mut buffer);

        // copy buffer to memory
        let offset = 512 as usize;
        for i in 0..buffer.len() {
            self.memory_unit.memory[offset + i] = buffer[i];
        }
    }
}
