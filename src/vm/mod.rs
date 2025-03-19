pub mod consts;
pub mod errors;

use consts::*;
use errors::FdeError;
use notan::random::rand;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub struct VirtualMachine {
    // Quality of life
    pub is_running: bool,
    pub is_waiting_for_key: bool,
    pub current_key_press: Option<u8>,
    pub last_cycle_result: Result<(), FdeError>,

    // Necessary fields
    pub memory: [u8; MEM_SIZE],
    pub video_memory: [[u8; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    pub program_counter: usize,
    pub stack: Vec<usize>,
    pub i_register: usize,
    pub variable_registers: [u8; 16],
    pub keypad: [bool; 16],
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
            is_waiting_for_key: false,
            current_key_press: None,
            last_cycle_result: Ok(()),
            memory,
            video_memory: [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            program_counter: PROG_MEM_START_ADDR,
            stack: Vec::with_capacity(STACK_SIZE),
            i_register: 0,
            variable_registers: [0; 16],
            keypad: [false; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn pause(&mut self) {
        self.is_running = false;
    }

    pub fn load_program(&mut self, path: &Path) {
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
        if self.program_counter + 1 >= self.memory.len() {
            self.last_cycle_result = Err(FdeError::StackOverflow);
            return;
        }

        let opcode = self.memory
            [self.program_counter..self.program_counter + 2]
            .try_into();

        self.last_cycle_result = match opcode {
            Ok(opcode_bytes) => {
                self.program_counter += 2;
                self.decode_execute_instruction(opcode_bytes)
            }
            Err(_) => Err(FdeError::OpcodeFetch),
        }
    }

    fn decode_execute_instruction(
        &mut self,
        opcode_bytes: [u8; 2],
    ) -> Result<(), FdeError> {
        let nibbles: [u8; 4] = opcode_bytes
            .iter()
            .flat_map(|byte| [(*byte >> 4) & 0xF, *byte & 0xF])
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| FdeError::NibblesFetch {
                _opcode_bytes: opcode_bytes,
            })?;

        let x = nibbles[1] as usize;
        let y = nibbles[2] as usize;
        let n = nibbles[3];
        let kk = opcode_bytes[1];
        let nnn = (x << 8) | kk as usize;

        match nibbles {
            // Clear screen
            [0x0, 0x0, 0xE, 0x0] => {
                self.video_memory = [[0; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
                Ok(())
            }
            // Return from subroutine
            [0x0, 0x0, 0xE, 0xE] => {
                self.program_counter =
                    self.stack.pop().ok_or(FdeError::SubroutineReturn)?;
                Ok(())
            }
            // Jump to address
            [0x1, _, _, _] => {
                self.program_counter = nnn;
                Ok(())
            }
            // Call subroutine
            [0x2, _, _, _] => {
                self.stack.push(self.program_counter);
                self.program_counter = nnn;
                Ok(())
            }
            // Skip if VX = kk
            [0x3, _, _, _] => {
                if self.variable_registers[x] == kk {
                    self.program_counter += 2;
                }
                Ok(())
            }
            // Skip if VX != kk
            [0x4, _, _, _] => {
                if self.variable_registers[x] != kk {
                    self.program_counter += 2;
                }
                Ok(())
            }
            // Skip if VX == VY
            [0x5, _, _, 0x0] => {
                if self.variable_registers[x] == self.variable_registers[y] {
                    self.program_counter += 2;
                }
                Ok(())
            }
            // Set register VX
            [0x6, _, _, _] => {
                self.variable_registers[x] = kk;
                Ok(())
            }
            // Add value to register VX
            [0x7, _, _, _] => {
                let (addition_result, _) =
                    self.variable_registers[x].overflowing_add(kk);

                self.variable_registers[x] = addition_result;
                Ok(())
            }
            // Set VX = VY
            [0x8, _, _, 0x0] => {
                self.variable_registers[x] = self.variable_registers[y];
                Ok(())
            }
            // Set VX = VX | VY
            [0x8, _, _, 0x1] => {
                self.variable_registers[x] |= self.variable_registers[y];
                Ok(())
            }
            // Set VX = VX & VY
            [0x8, _, _, 0x2] => {
                self.variable_registers[x] &= self.variable_registers[y];
                Ok(())
            }
            // Set VX = VX ^ VY
            [0x8, _, _, 0x3] => {
                self.variable_registers[x] ^= self.variable_registers[y];
                Ok(())
            }
            // Set VX = VX + VY
            [0x8, _, _, 0x4] => {
                let (sum, is_overflow) = self.variable_registers[x]
                    .overflowing_add(self.variable_registers[y]);

                self.variable_registers[x] = sum;
                self.variable_registers[0xF] = is_overflow as u8;
                Ok(())
            }
            // Set VX = VX - VY
            [0x8, _, _, 0x5] => {
                let (diff, is_underflow) = self.variable_registers[x]
                    .overflowing_sub(self.variable_registers[y]);

                self.variable_registers[x] = diff;
                self.variable_registers[0xF] = !is_underflow as u8;
                Ok(())
            }
            // Set VX = VX >> 1
            [0x8, _, _, 0x6] => {
                // TODO: handle ambiguous behaviour
                self.variable_registers[0xF] = self.variable_registers[x] & 0x1;
                self.variable_registers[x] >>= 1;
                Ok(())
            }
            // Set VX = VY - VX
            [0x8, _, _, 0x7] => {
                let (diff, is_underflow) = self.variable_registers[y]
                    .overflowing_sub(self.variable_registers[x]);

                self.variable_registers[x] = diff;
                self.variable_registers[0xF] = !is_underflow as u8;
                Ok(())
            }
            // Set VX = VX << 1
            [0x8, _, _, 0xE] => {
                // TODO: handle ambiguous behaviour
                self.variable_registers[0xF] =
                    (self.variable_registers[x] & 0x80) >> 7;
                self.variable_registers[x] <<= 1;
                Ok(())
            }
            // Skip if VY != VY
            [0x9, _, _, 0x0] => {
                if self.variable_registers[x] != self.variable_registers[y] {
                    self.program_counter += 2;
                }
                Ok(())
            }
            // Set index register I
            [0xA, _, _, _] => {
                self.i_register = nnn;
                Ok(())
            }
            // Jump to nnn + V0
            [0xB, _, _, _] => {
                // TODO: handle ambiguous behaviour
                self.program_counter =
                    self.variable_registers[0] as usize + nnn;
                Ok(())
            }
            // Set VX = random byte && kk
            [0xC, _, _, _] => {
                self.variable_registers[x] = rand::random::<u8>() & kk;
                Ok(())
            }
            // Draw to screen
            [0xD, _, _, _] => {
                let base_x_position =
                    self.variable_registers[x] as usize % DISPLAY_WIDTH;
                let base_y_position =
                    self.variable_registers[y] as usize % DISPLAY_HEIGHT;

                let sprite_bytes = self.memory
                    [self.i_register..self.i_register + (n as usize)]
                    .iter()
                    .enumerate();

                self.variable_registers[0xF] = 0;

                for (row_index, sprite_byte) in sprite_bytes {
                    let pixel_y_position = base_y_position + row_index;

                    if pixel_y_position == DISPLAY_HEIGHT {
                        break;
                    }

                    let sprite_pixels = (0u8..8)
                        .rev()
                        .map(|bit_index| (*sprite_byte >> bit_index) & 1)
                        .enumerate();

                    for (column_index, sprite_pixel) in sprite_pixels {
                        let pixel_x_position = base_x_position + column_index;

                        if pixel_x_position == DISPLAY_WIDTH {
                            break;
                        }

                        let display_pixel = &mut self.video_memory
                            [pixel_y_position][pixel_x_position];

                        if sprite_pixel == 1 && *display_pixel == 1 {
                            self.variable_registers[0xF] = 1;
                        }

                        *display_pixel ^= sprite_pixel;
                    }
                }
                Ok(())
            }
            // Skip if VX key is pressed
            [0xE, _, 0x9, 0xE] => {
                let key = self.variable_registers[x] as usize;
                if self.keypad[key] {
                    self.program_counter += 2;
                }
                Ok(())
            }
            // Skip if VX key is NOT pressed
            [0xE, _, 0xA, 0x1] => {
                let key = self.variable_registers[x] as usize;
                if !self.keypad[key] {
                    self.program_counter += 2;
                }
                Ok(())
            }
            // Set VX = delay timer value
            [0xF, _, 0x0, 0x7] => {
                self.variable_registers[x] = self.delay_timer;
                Ok(())
            }
            // Wait for a key press, store value in VX
            [0xF, _, 0x0, 0xA] => {
                self.is_waiting_for_key = true;

                if let Some(val) = self.current_key_press {
                    self.variable_registers[x] = val;
                    self.is_waiting_for_key = false;
                    self.current_key_press = None;
                    return Ok(());
                }

                self.program_counter -= 2;
                Ok(())
            }
            // Set delay timer = VX
            [0xF, _, 0x1, 0x5] => {
                self.delay_timer = self.variable_registers[x];
                Ok(())
            }
            // Set sound timer = VX
            [0xF, _, 0x1, 0x8] => {
                self.sound_timer = self.variable_registers[x];
                Ok(())
            }
            // Set I = I + VX
            [0xF, _, 0x1, 0xE] => {
                // TODO: handle ambiguous behaviour
                self.i_register += self.variable_registers[x] as usize;
                Ok(())
            }
            // Set I = location of sprite for VX
            [0xF, _, 0x2, 0x9] => {
                let digit = self.variable_registers[x];
                self.i_register = FONTSET_START_ADDR + (5 * digit) as usize;
                Ok(())
            }
            // Store BCD of VX in I, I+1, I+2
            [0xF, _, 0x3, 0x3] => {
                let mut value = self.variable_registers[x];

                self.memory[self.i_register + 2] = value % 10;
                value /= 10;

                self.memory[self.i_register + 1] = value % 10;
                value /= 10;

                self.memory[self.i_register] = value % 10;
                Ok(())
            }
            // Store V0 through VX starting at I
            [0xF, _, 0x5, 0x5] => {
                self.memory[self.i_register..=self.i_register + x]
                    .copy_from_slice(&self.variable_registers[..=x]);
                Ok(())
            }
            // Read V0 through VX starting at I
            [0xF, _, 0x6, 0x5] => {
                self.variable_registers[..=x].copy_from_slice(
                    &self.memory[self.i_register..=self.i_register + x],
                );
                Ok(())
            }
            // Unknown instruction
            [_, _, _, _] => {
                Err(FdeError::UnknownInstruction { _nibbles: nibbles })
            }
        }
    }
}
