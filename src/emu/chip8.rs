use crate::emu::consts::START_ADDR;
use crate::emu::virtual_machine::VirtualMachine;

pub struct Chip8 {
    memory: [u8; 4096],
    stack: [u16; 16],
    sp: u8,      // Stack pointer
    i: u16,      // Index register
    pc: u16,     // Program counter
    v: [u8; 16], // Vx registers
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; 4096],
            stack: [0; 16],
            sp: 0,
            i: 0,
            pc: START_ADDR as u16,
            v: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn start(&mut self, rom_path: &str) {
        self.load_fonts();
        self.load_rom(rom_path);
        println!("{:?}", self.memory);
    }
}

impl VirtualMachine for Chip8 {
    fn get_memory(&mut self) -> &mut [u8] {
        &mut self.memory
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emu::consts::FONTSET_START_ADDR;
    use crate::emu::font::FONTSET;

    #[test]
    fn verify_font_load_into_memory() {
        let mut chip8 = Chip8::new();
        chip8.load_fonts();

        for (i, val) in FONTSET.iter().enumerate() {
            assert_eq!(chip8.memory[FONTSET_START_ADDR + i], val.clone());
        }
    }
}
