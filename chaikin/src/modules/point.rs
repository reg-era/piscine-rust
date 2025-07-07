#[derive(Clone, Debug)]
pub struct Point {
    pub radius: f32,
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, radius: 5.0 }
    }

    pub fn set_position(&mut self, new_position: (f32, f32)) {
        self.x = new_position.0;
        self.y = new_position.1;
    }

    pub fn check_collision(&self, area_x: f32, area_y: f32) -> bool {
        let distance = ((area_x - self.x).powi(2) + (area_y - self.y).powi(2)).sqrt();

        distance <= self.radius
    }

    pub fn make_25_point(&self, other: &Point) -> Point {
        let p0 = self;
        let p1 = other;
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        Point {
            x: p0.x + dx * 0.25,
            y: p0.y + dy * 0.25,
            radius: 0.,
        }
    }

    pub fn make_75_point(&self, other: &Point) -> Point {
        let p0 = self;
        let p1 = other;
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        Point {
            x: p0.x + dx * 0.75,
            y: p0.y + dy * 0.75,
            radius: 0.,
        }
    }
}
