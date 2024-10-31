use crate::emu::consts::START_ADDR;
use crate::emu::font::load_fonts;
use crate::emu::rom::load_rom;

pub struct Chip8 {
    memory: [u8; 4096],
    stack: [u16; 16],
    sp: u8, // Stack pointer
    i: u16, // Index register
    pc: u16, // Program counter
    v: [u8; 16], // Vx registers
    delay_timer: u8,
    sound_timer: u8
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: [0; 4096],
            stack: [0; 16],
            sp: 0,
            i: 0,
            pc: START_ADDR,
            v: [0; 16],
            delay_timer: 0,
            sound_timer: 0
        }
    }

    pub fn start(&mut self, rom_path: &str) {
        load_fonts(&mut self.memory);
        load_rom(rom_path, &mut self.memory);
        println!("{:?}", self.memory);
    }
}

#[cfg(test)]
mod tests {
    use crate::emu::chip8::Chip8;

    #[test]
    pub fn test_chip8() {
        let mut chip8 = Chip8::new();
        chip8.start("/Users/dolczyk/Downloads/ibm.ch8");
    }
}
