use crate::emu;
use crate::emu::consts::FONTSET_START_ADDR;
use crate::emu::font::FONTSET;
use std::fs::File;
use std::io::{BufReader, Read};

pub trait VirtualMachine {
    fn get_memory(&mut self) -> &mut [u8];

    fn load_fonts(&mut self) -> () {
        for (i, val) in FONTSET.iter().enumerate() {
            self.get_memory()[FONTSET_START_ADDR + i] = val.clone();
        }
    }

    fn load_rom(&mut self, rom_path: &str) -> () {
        let mut buffer: Vec<u8> = Vec::new();
        let mut buf_reader = BufReader::new(File::open(rom_path).unwrap());
        buf_reader.read_to_end(&mut buffer).unwrap();

        for (i, val) in buffer.iter().enumerate() {
            self.get_memory()[emu::consts::START_ADDR + i] = val.clone();
        }
    }
}
