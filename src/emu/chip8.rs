pub struct Chip8 {
    memory: [u8; 4096], // 4KB of memory
    stack: [u16; 16], // 16 x 8 bit registers (V0 - VF)
    sp: u8, // Stack pointer
    i: u16, // Index register
    pc: u16, // Program counter
    v: [u8; 16], // Stack pointer
    delay_timer: u8,
    sound_timer: u8
}