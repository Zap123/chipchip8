mod chip8; 
use std::{thread, time::Duration}; 

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("roms/ibmlogo.c8");
    
    // 60 hz loop
    let period = Duration::from_secs_f64(1.0/60.00);
    loop {
        chip8.emulate_cycle();

        thread::sleep(period);
    }

}
