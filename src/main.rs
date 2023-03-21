use std::fs;

#[allow(dead_code)]
const START_ADDRESS:u16 = 0x200;

#[allow(dead_code)]
struct Chip8{
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16],
    video: [u32; 64*32],
    opcode: u16
}

#[allow(dead_code)]
impl Chip8 {
    
    fn new() -> Self{
        Self {
            registers: [0; 16],
            memory: [0; 4096],
            index: 0,
            pc: START_ADDRESS,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            video: [0; 64*32],
            opcode: 0
            }
    }
   
    fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>{
        let contents = fs::read(path)?;
        let length = contents.len();
        for i in 1..length {
            self.memory[START_ADDRESS as usize + i] = contents[i];
        }
        // self.memory = fs::read(path)
        Ok(())
    }
}


fn main() {
    let chip = Chip8::new();
    println!("Hello, world!");
}
