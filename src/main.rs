use std::fs;
use rand::Rng;

const START_ADDRESS:usize = 0x200;
const FONSET_START_ADDRESS:usize = 0x50;
const FONTSET_SIZE: usize = 80;
static FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[allow(dead_code)]
struct Chip8{
    registers: [u8; 16],
    memory: [u8; 4096],
    index: u16,
    pc: usize,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [u8; 16],
    video: [u32; 64*32],
    opcode: u16,
}

#[allow(dead_code)]
impl Chip8 {
    
    fn new() -> Self{
        let mut new = Self {
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
            };
            
        for i in 1..FONTSET_SIZE{
            new.memory[FONSET_START_ADDRESS + i] = FONTSET[i]
        }
        new
    }
   
    fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>>{
        let contents = fs::read(path)?;
        let length = contents.len();
        for i in 1..length {
            self.memory[START_ADDRESS as usize + i] = contents[i];
        }
        Ok(())
    }
    
    // maybe this needs to be different 
    fn rand_byte() -> i32 {
        rand::thread_rng().gen_range(0..256)
    }
    
    fn op_00e0(&mut self){
        self.video.iter_mut().for_each(|m| *m = 0)
    }
}


fn main() {
    //let chip = Chip8::new();
    println!("Hello, world!");
}
