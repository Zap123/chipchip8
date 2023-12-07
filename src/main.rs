extern crate sdl2;
use std::{thread, time::Duration};

use sdl2::{event::Event, keyboard::Keycode};
mod chip8;

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("roms/pong2.c8");

    // 60 hz loop
    let period = Duration::from_secs_f64(1.0 / 60.00);
    'running: loop {
        chip8.emulate_cycle();
        /*
            Keypad                   Keyboard
            +-+-+-+-+                +-+-+-+-+
            |1|2|3|C|                |1|2|3|4|
            +-+-+-+-+                +-+-+-+-+
            |4|5|6|D|                |Q|W|E|R|
            +-+-+-+-+       =>       +-+-+-+-+
            |7|8|9|E|                |A|S|D|F|
            +-+-+-+-+                +-+-+-+-+
            |A|0|B|F|                |Z|X|C|V|
            +-+-+-+-+                +-+-+-+-+


        */

        for event in chip8.input_unit.driver.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Num1), ..} => {

                    }
                _ => {}
            }
        }

        thread::sleep(period);
    }
}
