use chip8_core::Emulator;
use std::{fs::File, io::Read};

fn main() {
    let mut chip8 = Emulator::new();
    let mut rom = File::open("ibm.ch8").expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Buffer error");
    chip8.load(&buffer);
    loop {
        chip8.tick();
    }
}
