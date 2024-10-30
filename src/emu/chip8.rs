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