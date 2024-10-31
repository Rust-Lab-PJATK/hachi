use std::fs::File;
use std::io::{BufReader, Read};
use crate::emu;

pub fn load_rom(rom_path: &str, memory: &mut [u8; 4096]) -> () {
    let mut buffer: Vec<u8> = Vec::new();
    let mut buf_reader = BufReader::new(File::open(rom_path).unwrap());
    buf_reader.read_to_end(&mut buffer).unwrap();

    for (i, val) in buffer.iter().enumerate() {
        memory[(emu::consts::START_ADDR as usize) + i] = val.clone();
    }
}