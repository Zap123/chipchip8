mod chip8; 

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("ibmlogo.c8");
    loop {
        chip8.emulate_cycle();
    }

}
