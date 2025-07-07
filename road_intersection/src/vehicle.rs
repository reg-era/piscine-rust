use crate::{Light, LightState, TrafficLight};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

pub const TILE_SIZE: u32 = 50;

#[derive(Clone)]
pub struct Vehicles {
    pub uuid: usize,
    pub vehicles: Vec<Vehicle>,
}

#[derive(Clone)]
pub struct Vehicle {
    pub id: usize,
    pub rect: Rect,
    pub spawn: Side,
    pub color: Color,
    pub stop: bool,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Side {
    North,
    East,
    South,
    West,
}

impl Vehicles {
    pub fn new() -> Self {
        Self {
            uuid: 0,
            vehicles: Vec::new(),
        }
    }
    fn random_vehicle() -> Color {
        let colors = [Color::WHITE, Color::CYAN, Color::YELLOW];
        colors[rand::random_range(0..colors.len())]
    }
    pub fn spawn<'a>(&mut self, rect: Rect, side: Side) -> Option<usize> {
        let spirit_car = Rect::new(rect.x, rect.y, TILE_SIZE, TILE_SIZE);

        for cars in self.vehicles.iter() {
            if spirit_car.has_intersection(cars.rect) {
                return None;
            }
        }

        let id = self.uuid;
        self.uuid += 1;

        let vehicle = Vehicle {
            id,
            rect,
            spawn: side,
            color: Self::random_vehicle(),
            stop: false,
        };

        self.vehicles.push(vehicle);
        Some(id)
    }
    pub fn draw(&mut self, canvas: &mut Canvas<Window>, traffic_light: &mut TrafficLight) {
        let mut to_remove_from_stack = Vec::new();

        self.vehicles = self
            .vehicles
            .iter()
            .filter_map(|vehicle| {
                if vehicle.has_left_screen() {
                    // Schedule removal from stack
                    to_remove_from_stack.push((vehicle.spawn.clone(), vehicle.id));
                    None
                } else {
                    canvas.set_draw_color(vehicle.color);
                    canvas.fill_rect(vehicle.rect).unwrap();
                    Some(vehicle.clone())
                }
            })
            .collect();

        // Remove vehicle ID from traffic light stack
        for (side, id) in to_remove_from_stack {
            traffic_light.remove_from_stack(side, id);
        }
    }

    pub fn move_cars(&mut self, delta: f32, lights: &[Light]) {
        let copy_vehicle = self.vehicles.clone();
        for (id, vehicle) in &mut self.vehicles.iter_mut().enumerate() {
            // if vehicle.stop {
            // continue;
            // }

            match &vehicle.spawn {
                Side::East => {
                    if vehicle.color == Color::YELLOW {
                        if lights[1].state == LightState::Red
                            && vehicle.rect.x >= 290
                            && vehicle.rect.x < 300
                        {
                            continue;
                        }
                    } else {
                        if lights[1].state == LightState::Red
                            && vehicle.rect.x >= 290
                            && vehicle.rect.x < 300
                            && vehicle.rect.y == 300
                        {
                            continue;
                        }
                    }
                }
                Side::North => {
                    if vehicle.color == Color::YELLOW {
                        if lights[0].state == LightState::Red
                            && vehicle.rect.y >= 190
                            && vehicle.rect.y < 200
                        {
                            continue;
                        }
                    } else {
                        if lights[0].state == LightState::Red
                            && vehicle.rect.y >= 190
                            && vehicle.rect.y < 200
                            && vehicle.rect.x == 350
                        {
                            continue;
                        }
                    }
                }
                Side::South => {
                    if vehicle.color == Color::YELLOW {
                        if lights[3].state == LightState::Red
                            && vehicle.rect.y > 350
                            && vehicle.rect.y <= 360
                        {
                            continue;
                        }
                    } else {
                        if lights[3].state == LightState::Red
                            && vehicle.rect.y > 350
                            && vehicle.rect.y <= 360
                            && vehicle.rect.x == 400
                        {
                            continue;
                        }
                    }
                }
                Side::West => {
                    if vehicle.color == Color::YELLOW {
                        if lights[2].state == LightState::Red
                            && vehicle.rect.x > 450
                            && vehicle.rect.x <= 460
                        {
                            continue;
                        }
                    } else {
                        if lights[2].state == LightState::Red
                            && vehicle.rect.x > 450
                            && vehicle.rect.x <= 460
                            && vehicle.rect.y == 250
                        {
                            continue;
                        }
                    }
                }
            }
            let (new_x, new_y) = vehicle.move_vehicle((300. * delta) as i32);

            let old_x = vehicle.rect.x;
            let old_y = vehicle.rect.y;

            let margin_offset = 5;

            let mut margin_x = new_x;
            let mut margin_y = new_y;

            if new_y < old_y {
                margin_y -= margin_offset;
            } else if new_y > old_y {
                margin_y += margin_offset;
            }

            if new_x < old_x {
                margin_x -= margin_offset;
            } else if new_x > old_x {
                margin_x += margin_offset;
            }

            let spirit_car = Rect::new(margin_x, margin_y, TILE_SIZE, TILE_SIZE);
            let mut is_safe = true;
            for (car_id, cars) in copy_vehicle.iter().enumerate() {
                if (car_id != id) && spirit_car.has_intersection(cars.rect) {
                    is_safe = false;
                    break;
                }
            }

            if is_safe {
                vehicle.rect.set_x(new_x);
                vehicle.rect.set_y(new_y);
            }
        }
    }
}

impl Vehicle {
    pub fn has_left_screen(&self) -> bool {
        if self.rect.x > 800 || self.rect.x < 0 {
            return true;
        }
        if self.rect.y > 600 || self.rect.y < 0 {
            return true;
        }
        false
    }

    pub fn move_vehicle(&self, speed: i32) -> (i32, i32) {
        match (&self.spawn, self.color) {
            (Side::East, Color::WHITE) => {
                if self.rect.x < 350 {
                    ((self.rect.x + speed).min(350), self.rect.y)
                } else {
                    (self.rect.x, self.rect.y + speed)
                }
            }
            (Side::East, Color::CYAN) => {
                if self.rect.x < 400 {
                    ((self.rect.x + speed).min(400), self.rect.y)
                } else {
                    (self.rect.x, self.rect.y - speed)
                }
            }
            (Side::East, Color::YELLOW) => (self.rect.x + speed, self.rect.y),
            (Side::North, Color::WHITE) => {
                if self.rect.y < 250 {
                    (self.rect.x, (self.rect.y + speed).min(250))
                } else {
                    (self.rect.x - speed, self.rect.y)
                }
            }
            (Side::North, Color::CYAN) => {
                if self.rect.y < 300 {
                    (self.rect.x, (self.rect.y + speed).min(300))
                } else {
                    (self.rect.x + speed, self.rect.y)
                }
            }
            (Side::North, Color::YELLOW) => (self.rect.x, self.rect.y + speed),
            (Side::West, Color::WHITE) => {
                if self.rect.x > 400 {
                    ((self.rect.x - speed).max(400), self.rect.y)
                } else {
                    (self.rect.x, self.rect.y - speed)
                }
            }
            (Side::West, Color::CYAN) => {
                if self.rect.x > 350 {
                    ((self.rect.x - speed).max(350), self.rect.y)
                } else {
                    (self.rect.x, self.rect.y + speed)
                }
            }
            (Side::West, Color::YELLOW) => (self.rect.x - speed, self.rect.y),
            (Side::South, Color::WHITE) => {
                if self.rect.y > 300 {
                    (self.rect.x, (self.rect.y - speed).max(300))
                } else {
                    (self.rect.x + speed, self.rect.y)
                }
            }
            (Side::South, Color::CYAN) => {
                if self.rect.y > 250 {
                    (self.rect.x, (self.rect.y - speed).max(250))
                } else {
                    (self.rect.x - speed, self.rect.y)
                }
            }
            (Side::South, Color::YELLOW) => (self.rect.x, self.rect.y - speed),
            _ => unreachable!(),
        }
    }
}
