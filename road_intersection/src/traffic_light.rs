pub use std::collections::HashMap;

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::Side;

#[derive(PartialEq)]
pub enum LightState {
    Red,
    Green,
}
pub struct Light {
    rect: Rect,
    pub state: LightState,
}

pub struct TrafficLight {
    pub lights: [Light; 4],
    pub timer: f32,
    pub stack: HashMap<Side, Vec<usize>>,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            lights: [
                Light {
                    rect: Rect::new(300, 200, 50, 50),
                    state: LightState::Red,
                },
                Light {
                    rect: Rect::new(300, 350, 50, 50),
                    state: LightState::Red,
                },
                Light {
                    rect: Rect::new(450, 200, 50, 50),
                    state: LightState::Red,
                },
                Light {
                    rect: Rect::new(450, 350, 50, 50),
                    state: LightState::Red,
                },
            ],
            timer: 0.0,
            stack: HashMap::new(),
        }
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for light in &self.lights {
            match light.state {
                LightState::Green => canvas.set_draw_color(Color::GREEN),
                LightState::Red => canvas.set_draw_color(Color::RED),
            };
            canvas.draw_rect(light.rect).unwrap();
        }
    }

    pub fn add_to_stack(&mut self, side: Side, vehicle_id: usize) {
        self.stack.entry(side).or_default().push(vehicle_id);
    }

    pub fn remove_from_stack(&mut self, side: Side, vehicle_id: usize) {
        if let Some(vehicles) = self.stack.get_mut(&side) {
            vehicles.retain(|&id| id != vehicle_id);
        }
    }

    pub fn controle_lights(&mut self, delta: f32) {
        self.timer += delta;

        if self.timer < 2.0 {
            return; // only switch every 2 seconds
        }

        self.timer = 0.0;
        if self.stack.is_empty() {
            return; // keep current state
        }

        let mut max_side = None;
        let mut max_count = 0;

        for (side, queue) in &self.stack {
            if queue.len() > max_count {
                max_count = queue.len();
                max_side = Some(side);
            }
        }

        if let Some(side) = max_side {
            for (i, light) in self.lights.iter_mut().enumerate() {
                let current_side = match i {
                    0 => Side::North,
                    1 => Side::East,
                    2 => Side::West,
                    3 => Side::South,
                    _ => continue,
                };

                light.state = if current_side == *side {
                    LightState::Green
                } else {
                    LightState::Red
                };
            }
        }
    }

    pub fn light_rg(&mut self, delta: f32) {
        self.timer += delta;

        // Set all lights to red by default
        for light in &mut self.lights {
            light.state = LightState::Red;
        }

        // 5 seconds green + 2 seconds red between each
        if self.timer < 5.0 {
            self.lights[0].state = LightState::Green; // North
        } else if self.timer >= 7.0 && self.timer < 12.0 {
            self.lights[1].state = LightState::Green; // East
        } else if self.timer >= 14.0 && self.timer < 19.0 {
            self.lights[2].state = LightState::Green; // South
        } else if self.timer >= 21.0 && self.timer < 26.0 {
            self.lights[3].state = LightState::Green; // West
        }

        // Restart the full cycle after 28 seconds
        if self.timer >= 28.0 {
            self.timer = 0.0;
        }
    }
}
