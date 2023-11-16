extern crate sdl2;
use sdl2::{pixels::Color, rect::Point, Sdl};

const SCREEN_SIZE: u16 = 64 * 32;

pub struct GraphicsUnit {
    pub screen: [u8; SCREEN_SIZE as usize],
    pub driver: Sdl,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl<'a> GraphicsUnit {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Chip Chip 8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.set_scale(10.0, 10.0);
        canvas.clear();
        canvas.present();

        GraphicsUnit {
            screen: [0; SCREEN_SIZE as usize],
            driver: sdl_context,
            canvas: canvas,
        }
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        let coordinates_to_draw = self
            .screen
            .iter()
            .enumerate()
            .filter(|(_, &val)| val == 1)
            .map(|(i, _)| {
                let x = (i % 64) as i32;
                let y = (i / 64) as i32;
                Point::new(x, y)
            }).collect::<Vec<Point>>();

        self.canvas.draw_points(&coordinates_to_draw[..]).unwrap();

        self.canvas.present();
    }
}
