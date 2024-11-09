pub mod consts;

use consts::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

pub struct VirtualMachine {
    pub is_running: bool,
    pub memory: [u8; MEM_SIZE],
    pub video_memory: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub _stack: Vec<u16>,
    pub program_counter: usize,
    pub i_register: u16,
    pub variable_registers: [u8; 16],
    pub _delay_timer: u8,
    pub _sound_timer: u8,
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
            _stack: Vec::with_capacity(STACK_SIZE),
            program_counter: PROG_MEM_START_ADDR,
            i_register: 0,
            variable_registers: [0; 16],
            _delay_timer: 0,
            _sound_timer: 0,
        }
    }

    pub fn load_program(&mut self, path: PathBuf) {
        self.memory[PROG_MEM_START_ADDR..].fill(0);
        self.video_memory = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

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
        let nnn = ((nibbles[1] as u16) << 8) | kk as u16;

        match nibbles {
            // Clear screen
            [0x0, 0x0, 0xE, 0x0] => {
                self.video_memory = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
            }
            // Jump to address
            [0x1, _, _, _] => {
                self.program_counter = nnn as usize;
            }
            // Set register VX
            [0x6, _, _, _] => {
                self.variable_registers[x] = kk;
            }
            // Add value to register VX
            [0x7, _, _, _] => {
                self.variable_registers[x] += kk;
            }
            // Set index register I
            [0xA, _, _, _] => {
                self.i_register = nnn;
            }
            // Draw to screen
            [0xD, _, _, _] => {
                let x_position =
                    self.variable_registers[x] as usize % DISPLAY_WIDTH;
                let y_position =
                    self.variable_registers[y] as usize % DISPLAY_HEIGHT;

                let i_register_value = self.i_register as usize;
                let sprite_bytes = self.memory
                    [i_register_value..(i_register_value + (n as usize))]
                    .into_iter()
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
            // Unknown instruction
            [_, _, _, _] => (),
        };
    }
}
