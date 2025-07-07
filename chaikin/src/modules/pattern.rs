use super::point::Point;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Pattern {
    pub path: Vec<Point>,
    pub fixed_point: Vec<Point>,
    pub final_path: Vec<Vec<Point>>,

    pub animation_start: bool,
    pub frame: usize,

    pub draging: bool,
    pub dragable_point: Option<(usize, usize)>,
}

impl Pattern {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            fixed_point: Vec::new(),
            final_path: Vec::new(),

            animation_start: false,
            frame: 0,

            draging: false,
            dragable_point: None,
        }
    }

    pub fn append_point(&mut self, (x, y): (f32, f32)) {
        self.path.push(Point::new(x, y));
        self.fixed_point.push(Point::new(x, y));
    }

    pub fn draw_path(&self) {
        if self.path.len() >= 1 {
            let mut prev_point: &Point = &self.path[0];
            for point in self.path.iter() {
                draw_line(prev_point.x, prev_point.y, point.x, point.y, 2.2, RED);
                prev_point = point;
            }
            for point in self.fixed_point.iter() {
                draw_circle(point.x, point.y, point.radius, BLUE);
            }
        }
    }

    pub fn check_point_colision(&mut self, area: (f32, f32)) {
        for (index, point) in self.fixed_point.iter().enumerate() {
            if point.check_collision(area.0, area.1) {
                self.draging = true;
                for (path_index, path_point) in self.path.iter().enumerate() {
                    if point.x == path_point.x && point.y == path_point.y {
                        self.dragable_point = Some((index, path_index));
                        break;
                    }
                }
                break;
            }
        }
    }

    pub fn chaikin(&mut self) {
        let mut original = self.path.clone();

        for _ in 0..7 {
            self.final_path.push(original.clone());

            let mut new_path = Vec::new();
            for i in 0..original.len() - 1 {
                let new_25_point = original[i].make_25_point(&original[i + 1]);
                let new_75_point = original[i].make_75_point(&original[i + 1]);

                if i == 0 {
                    new_path.push(original[i].clone());
                    new_path.push(new_75_point);
                    continue;
                }

                new_path.push(new_25_point);

                if i == original.len() - 2 {
                    new_path.push(original[i + 1].clone());
                } else {
                    new_path.push(new_75_point);
                }
            }

            original = new_path;
        }

        self.animation_start = true;
    }
}
