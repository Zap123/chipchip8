use sdl2::EventPump;

pub struct InputUnit {
    pub keypad: [u8; 16],
    pub driver: EventPump,
}

impl InputUnit{
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let driver = sdl_context.event_pump().unwrap();

        InputUnit {
            keypad: [0; 16],
            driver,
        }
    }
}