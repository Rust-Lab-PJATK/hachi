pub mod consts;

use consts::*;
use notan::random::rand;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub struct VirtualMachine {
    pub is_running: bool,
    pub memory: [u8; MEM_SIZE],
    pub video_memory: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub program_counter: usize,
    pub stack: Vec<u16>,
    pub i_register: usize,
    pub variable_registers: [u8; 16],
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl VirtualMachine {
    pub fn new() -> Self {
        let memory: [u8; MEM_SIZE] = [0; FONTSET_START_ADDR]
            .into_iter()
            .chain(FONTSET)
            .chain([0; MEM_SIZE - FONTSET_SIZE - FONTSET_START_ADDR])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        VirtualMachine {
            is_running: false,
            memory,
            video_memory: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            program_counter: PROG_MEM_START_ADDR,
            stack: Vec::with_capacity(STACK_SIZE),
            i_register: 0,
            variable_registers: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load_program(&mut self, path: PathBuf) {
        self.is_running = false;
        self.video_memory = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
        self.program_counter = PROG_MEM_START_ADDR;
        self.memory[PROG_MEM_START_ADDR..].fill(0);

        let file = File::open(path).unwrap();
        let file_size = file.metadata().unwrap().len() as usize;

        let mut buffer: Vec<u8> = Vec::with_capacity(file_size);
        let mut buf_reader = BufReader::new(file);

        buf_reader.read_to_end(&mut buffer).unwrap();
        self.memory[PROG_MEM_START_ADDR..PROG_MEM_START_ADDR + buffer.len()]
            .copy_from_slice(&buffer);

        self.is_running = true;
    }

    pub fn fde_cycle(&mut self) {
        let opcode_bytes: [u8; 2] = self.memory
            [self.program_counter..self.program_counter + 2]
            .try_into()
            .unwrap();

        self.program_counter += 2;

        let nibbles: [u8; 4] = opcode_bytes
            .iter()
            .flat_map(|byte| [(*byte >> 4) & 0xF, *byte & 0xF])
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        let x = nibbles[1] as usize;
        let y = nibbles[2] as usize;
        let n = nibbles[3];
        let kk = opcode_bytes[1];
        let nnn = (x << 8) | kk as usize;

        match nibbles {
            // Clear screen
            [0x0, 0x0, 0xE, 0x0] => {
                self.video_memory = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
            }
            // Return from subroutine
            [0x0, 0x0, 0xE, 0xE] => {
                self.program_counter = self.stack.pop().unwrap() as usize;
            }
            // Jump to address
            [0x1, _, _, _] => {
                self.program_counter = nnn;
            }
            // Call subroutine
            [0x2, _, _, _] => {
                self.stack.push(self.program_counter as u16);
                self.program_counter = nnn;
            }
            // Skip if VX = kk
            [0x3, _, _, _] => {
                if self.variable_registers[x] == kk {
                    self.program_counter += 2;
                }
            }
            // Skip if VX != kk
            [0x4, _, _, _] => {
                if self.variable_registers[x] != kk {
                    self.program_counter += 2;
                }
            }
            // Skip if VX == VY
            [0x5, _, _, 0x0] => {
                if self.variable_registers[x] == self.variable_registers[y] {
                    self.program_counter += 2;
                }
            }
            // Set register VX
            [0x6, _, _, _] => {
                self.variable_registers[x] = kk;
            }
            // Add value to register VX
            [0x7, _, _, _] => {
                self.variable_registers[x] += kk;
            }
            // Set VX = VY
            [0x8, _, _, 0x0] => {
                self.variable_registers[x] = self.variable_registers[y];
            }
            // Set VX = VX | VY
            [0x8, _, _, 0x1] => {
                self.variable_registers[x] |= self.variable_registers[y];
            }
            // Set VX = VX & VY
            [0x8, _, _, 0x2] => {
                self.variable_registers[x] &= self.variable_registers[y];
            }
            // Set VX = VX ^ VY
            [0x8, _, _, 0x3] => {
                self.variable_registers[x] ^= self.variable_registers[y];
            }
            // Set VX = VX + VY
            [0x8, _, _, 0x4] => {
                let add_option = self.variable_registers[x]
                    .checked_add(self.variable_registers[y]);

                match add_option {
                    Some(sum) => {
                        self.variable_registers[0xF] = 0;
                        self.variable_registers[x] = sum;
                    }
                    None => {
                        // overflow
                        self.variable_registers[0xF] = 1;
                        let overflowed_sum = self.variable_registers[x] as u16
                            + self.variable_registers[y] as u16;
                        self.variable_registers[x] =
                            (overflowed_sum & 0xFF) as u8;
                    }
                }
            }
            // Set VX = VX - VY
            [0x8, _, _, 0x5] => {
                if self.variable_registers[x] > self.variable_registers[y] {
                    self.variable_registers[0xF] = 1;
                    self.variable_registers[x] -= self.variable_registers[y];
                    return;
                }

                // underflow
                self.variable_registers[0xF] = 0;
                let underflowed_difference = self.variable_registers[x]
                    + (0xFF - self.variable_registers[y]);
                self.variable_registers[x] = underflowed_difference;
            }
            // Set VX = VX >> 1
            [0x8, _, _, 0x6] => {
                self.variable_registers[0xF] = self.variable_registers[x] & 0x1;
                self.variable_registers[x] >>= 1;
            }
            // Set VX = VY - VX
            [0x8, _, _, 0x7] => {
                if self.variable_registers[y] > self.variable_registers[x] {
                    self.variable_registers[0xF] = 1;
                    self.variable_registers[x] =
                        self.variable_registers[y] - self.variable_registers[x];
                    return;
                }

                // underflow
                self.variable_registers[0xF] = 0;
                let underflowed_difference = self.variable_registers[y]
                    + (0xFF - self.variable_registers[x]);
                self.variable_registers[x] = underflowed_difference;
            }
            // Set VX = VX << 1
            [0x8, _, _, 0xE] => {
                self.variable_registers[0xF] =
                    (self.variable_registers[x] & 0x80) >> 7;
                self.variable_registers[x] <<= 1;
            }
            // Skip if VY != VY
            [0x9, _, _, 0x0] => {
                if self.variable_registers[x] != self.variable_registers[y] {
                    self.program_counter += 2;
                }
            }
            // Set index register I
            [0xA, _, _, _] => {
                self.i_register = nnn;
            }
            // Jump to nnn + V0
            [0xB, _, _, _] => {
                self.program_counter =
                    self.variable_registers[0] as usize + nnn;
            }
            // Set VX = random byte && kk
            [0xC, _, _, _] => {
                self.variable_registers[x] = rand::random::<u8>() & kk;
            }
            // Draw to screen
            [0xD, _, _, _] => {
                let x_position =
                    self.variable_registers[x] as usize % DISPLAY_WIDTH;
                let y_position =
                    self.variable_registers[y] as usize % DISPLAY_HEIGHT;

                let sprite_bytes = self.memory
                    [self.i_register..self.i_register + (n as usize)]
                    .iter()
                    .enumerate();

                self.variable_registers[0xF] = 0;

                for (row_index, sprite_byte) in sprite_bytes {
                    let sprite_pixels = (0u8..8)
                        .rev()
                        .map(|bit_index| (*sprite_byte >> bit_index) & 1)
                        .enumerate();

                    for (column_index, sprite_pixel) in sprite_pixels {
                        let display_pixel = &mut self.video_memory
                            [y_position + row_index][x_position + column_index];

                        if sprite_pixel == 1 && *display_pixel == 1 {
                            self.variable_registers[0xF] = 1;
                        }

                        *display_pixel ^= sprite_pixel;
                    }
                }
            }
            // Skip if VX key is pressed
            [0xE, _, 0x9, 0xE] => {
                // TODO: when keypad is implemented
            }
            // Skip if VX key is NOT pressed
            [0xE, _, 0xA, 0x1] => {
                // TODO: when keypad is implemented
            }
            // Set VX = delay timer value
            [0xF, _, 0x0, 0x7] => {
                self.variable_registers[x] = self.delay_timer;
            }
            // Set delay timer = VX
            [0xF, _, 0x1, 0x5] => {
                self.delay_timer = self.variable_registers[x];
            }
            // Set sound timer = VX
            [0xF, _, 0x1, 0x8] => {
                self.sound_timer = self.variable_registers[x];
            }
            // Set I = I + VX
            [0xF, _, 0x1, 0xE] => {
                self.i_register += self.variable_registers[x] as usize;
            }
            // Set I = location of sprite for VX
            [0xF, _, 0x2, 0x9] => {
                let digit = self.variable_registers[x];
                self.i_register = FONTSET_START_ADDR + (5 * digit) as usize;
            }
            // Store BCD of VX in I, I+1, I+2
            [0xF, _, 0x3, 0x3] => {
                let mut value = self.variable_registers[x];

                self.memory[self.i_register + 2] = value % 10;
                value /= 10;

                self.memory[self.i_register + 1] = value % 10;
                value /= 10;

                self.memory[self.i_register] = value % 10;
            }
            // Store V0 through VX starting at I
            [0xF, _, 0x5, 0x5] => {
                for i in 0..=self.variable_registers[x] as usize {
                    self.memory[self.i_register + i] =
                        self.variable_registers[i];
                }
            }
            // Read V0 through VX starting at I
            [0xF, _, 0x6, 0x5] => {
                for i in 0..=self.variable_registers[x] as usize {
                    self.variable_registers[i] =
                        self.memory[self.i_register + i];
                }
            }
            // Unknown instruction
            [_, _, _, _] => (),
        };
    }
}
