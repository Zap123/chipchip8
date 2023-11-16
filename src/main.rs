extern crate sdl2;
use std::{thread, time::Duration};

use sdl2::event::Event;
mod chip8;

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("roms/ibmlogo.c8");

    // 60 hz loop
    let period = Duration::from_secs_f64(1.0 / 60.00);
    'running: loop {
        chip8.emulate_cycle();

        for event in chip8.input_unit.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        thread::sleep(period);
    }
}
