mod vehicle;
mod traffic_light;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Instant;
pub use vehicle::*;
pub use traffic_light::*;

pub struct DeltaTimer {
    last_instant: Instant,
}

impl DeltaTimer {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
        }
    }

    pub fn delta(&mut self) -> f32 {
        let now = Instant::now();
        let delta = now.duration_since(self.last_instant);
        self.last_instant = now;
        delta.as_secs_f32()
    }
}

pub fn draw_road(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::WHITE);

    canvas
        .draw_line(Point::new(400, 600), Point::new(400, 0))
        .unwrap();
    canvas
        .draw_line(Point::new(350, 600), Point::new(350, 0))
        .unwrap();
    canvas
        .draw_line(Point::new(450, 600), Point::new(450, 0))
        .unwrap();

    canvas
        .draw_line(Point::new(0, 300), Point::new(800, 300))
        .unwrap();
    canvas
        .draw_line(Point::new(0, 250), Point::new(800, 250))
        .unwrap();
    canvas
        .draw_line(Point::new(0, 350), Point::new(800, 350))
        .unwrap();
}
