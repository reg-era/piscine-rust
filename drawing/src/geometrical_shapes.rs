use rand::Rng;
use raster::{Color, Image};
use std::f64::consts::{FRAC_PI_2, PI};

// drawable and displayable traits

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color {
        let mut rng = rand::rng();
        Color {
            r: rng.random_range(0..=255),
            g: rng.random_range(0..=255),
            b: rng.random_range(0..=255),
            a: 255,
        }
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// Point struct

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(w: i32, h: i32) -> Point {
        let mut rng = rand::rng();
        let x = rng.random_range(0..=w);
        let y = rng.random_range(0..=h);
        Self { x, y }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }
}

// Line struct

pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Self { a, b }
    }

    pub fn random(w: i32, h: i32) -> Point {
        let mut rng = rand::rng();
        let nb1 = rng.random_range(0..=w);
        let nb2 = rng.random_range(0..=h);
        Point::new(nb1, nb2)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let Point {
            x: mut x0,
            y: mut y0,
        } = self.a;
        let Point { x: x1, y: y1 } = self.b;

        let ax = (x1 - x0).abs();
        let ay = if x0 < x1 { 1 } else { -1 };
        let bx = -(y1 - y0).abs();
        let by = if y0 < y1 { 1 } else { -1 };

        let mut err = ax + bx;

        loop {
            image.display(x0, y0, self.color());

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2: i32 = 2 * err;

            if e2 >= bx {
                err += bx;
                x0 += ay;
            }

            if e2 <= ax {
                err += ax;
                y0 += by;
            }
        }
    }
    fn color(&self) -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }
}

// Triangle struct

pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(pt_1: &Point, pt_2: &Point, pt_3: &Point) -> Self {
        Self {
            a: pt_1.clone(),
            b: pt_2.clone(),
            c: pt_3.clone(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        Line::new(self.a.clone(), self.b.clone()).draw(image);
        Line::new(self.b.clone(), self.c.clone()).draw(image);
        Line::new(self.c.clone(), self.a.clone()).draw(image);
    }
}

// Rectangle struct

pub struct Rectangle {
    a: Point,
    c: Point,
}

impl Rectangle {
    pub fn new(pt_1: &Point, pt_2: &Point) -> Self {
        Self {
            a: pt_1.clone(),
            c: pt_2.clone(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let b = Point {
            x: self.a.x,
            y: self.c.y,
        };
        let d = Point {
            x: self.c.x,
            y: self.a.y,
        };

        Line::new(self.a.clone(), b.clone()).draw(image);
        Line::new(b.clone(), self.c.clone()).draw(image);
        Line::new(self.c.clone(), d.clone()).draw(image);
        Line::new(d.clone(), self.a.clone()).draw(image);
    }
}

// Circle struct

pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    #[allow(dead_code)]
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(w: i32, h: i32) -> Circle {
        let mut rng = rand::rng();
        let center = Point::random(w, h);
        let radius = rng.random_range(1..=w.max(h));
        Self { center, radius }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let cx = self.center.x;
        let cy = self.center.y;
        let mut x = 0;
        let mut y = self.radius;

        let color = self.color();

        while x <= y {
            // 8 symmetric points
            image.display(cx + x, cy + y, color.clone());
            image.display(cx - x, cy + y, color.clone());
            image.display(cx + x, cy - y, color.clone());
            image.display(cx - x, cy - y, color.clone());
            image.display(cx + y, cy + x, color.clone());
            image.display(cx - y, cy + x, color.clone());
            image.display(cx + y, cy - x, color.clone());
            image.display(cx - y, cy - x, color.clone());

            x += 1;
            let d: i32 = ((x).pow(2) + (y).pow(2)).isqrt();
            if d > self.radius {
                y -= 1;
            }
        }
    }
}

// Pentagon struct

pub struct Pentagon {
    center: Point,
    radius: i32,
}

#[allow(dead_code)]
impl Pentagon {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(w: i32, h: i32) -> Pentagon {
        let mut rng = rand::rng();
        let center = Point::random(w, h);
        let radius = rng.random_range(1..=w.max(h));
        Self { center, radius }
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let angle_step = (2.0 * std::f64::consts::PI) / 5.0;

        let mut prev_point = Point::new(
            ((self.center.x as f64) + (self.radius as f64) * (4.0 * angle_step).cos()) as i32,
            ((self.center.y as f64) + (self.radius as f64) * (4.0 * angle_step).sin()) as i32,
        );

        for i in 0..5 {
            let angle = i as f64 * angle_step;
            let x = ((self.center.x as f64) + (self.radius as f64) * angle.cos()) as i32;
            let y = ((self.center.y as f64) + (self.radius as f64) * angle.sin()) as i32;

            Line::new(prev_point, Point::new(x, y)).draw(image);
            prev_point = Point::new(x, y);
        }
    }
}

// Cube struct

pub struct Cube {
    arm: i32,
    face: Rectangle,
}

#[allow(dead_code)]
impl Cube {
    pub fn new(start: Point, arm: i32) -> Self {
        Self {
            arm,
            face: Rectangle::new(
                &Point::new(start.x, start.y),
                &Point::new(start.x + arm, start.y + arm),
            ),
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut Image) {
        self.face.draw(image);

        let dimension = self.arm / 2;

        let back_face = Rectangle::new(
            &Point::new(self.face.a.x - dimension, self.face.a.y - dimension),
            &Point::new(self.face.c.x - dimension, self.face.c.y - dimension),
        );
        back_face.draw(image);

        Line::new(self.face.a.clone(), back_face.a.clone()).draw(image);
        Line::new(self.face.c.clone(), back_face.c.clone()).draw(image);

        let face_b = Point {
            x: self.face.a.x,
            y: self.face.c.y,
        };

        let back_face_b = Point {
            x: back_face.a.x,
            y: back_face.c.y,
        };

        Line::new(face_b, back_face_b).draw(image);

        let face_d = Point {
            x: self.face.c.x,
            y: self.face.a.y,
        };

        let back_face_d = Point {
            x: back_face.c.x,
            y: back_face.a.y,
        };

        Line::new(face_d, back_face_d).draw(image);
    }
}

// Pyramid struct

pub struct Pyramid {
    arm: i32,
    start: Point,
}

#[allow(dead_code)]
impl Pyramid {
    pub fn new(start: Point, arm: i32) -> Self {
        Self { start, arm }
    }
}

impl Drawable for Pyramid {
    fn draw(&self, image: &mut Image) {
        let b = Point {
            x: self.start.x + self.arm,
            y: self.start.y,
        };
        let c = Point {
            x: self.start.x + self.arm - (self.arm / 2),
            y: self.start.y - (self.arm / 2),
        };
        let d = Point {
            x: self.start.x - self.arm + (self.arm / 2),
            y: self.start.y - (self.arm / 2),
        };

        let head = Point {
            x: self.start.x + self.arm / 4,
            y: self.start.y - (self.arm as f64 * 1.5) as i32,
        };

        Line::new(self.start.clone(), b.clone()).draw(image);
        Line::new(b.clone(), c.clone()).draw(image);
        Line::new(c.clone(), d.clone()).draw(image);
        Line::new(d.clone(), self.start.clone()).draw(image);

        Line::new(head.clone(), self.start.clone()).draw(image);
        Line::new(head.clone(), b).draw(image);
        Line::new(head.clone(), c).draw(image);
        Line::new(head.clone(), d).draw(image);
    }
}

// Star struct

pub struct Star {
    center: Point,
    radius: i32,
}

#[allow(dead_code)]
impl Star {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(w: i32, h: i32) -> Star {
        let mut rng = rand::rng();
        let center = Point::random(w, h);
        let radius = rng.random_range(1..=w.max(h));
        Self { center, radius }
    }
}

impl Drawable for Star {
    fn draw(&self, image: &mut Image) {
        let big_angle_step = (2.0 * PI) / 5.0;
        let small_angle_step = (2.0 * PI) / 5.0;

        let mut small_prev_point = Point::new(
            self.center.x
                + ((self.radius as f64) / 3.0 * (4.0 * small_angle_step + FRAC_PI_2 / 2.0).cos())
                    as i32,
            ((self.center.y as f64)
                + (self.radius as f64) / 3.0 * (4.0 * small_angle_step + FRAC_PI_2 / 2.0).sin())
                as i32,
        );

        for i in 0..5 {
            let big_angle = i as f64 * big_angle_step;
            let x = ((self.center.x as f64) + (self.radius as f64) * big_angle.cos()) as i32;
            let y = ((self.center.y as f64) + (self.radius as f64) * big_angle.sin()) as i32;

            Line::new(small_prev_point, Point::new(x, y)).draw(image);
            let big_prev_point = Point::new(x, y);

            let small_angle = i as f64 * small_angle_step + FRAC_PI_2 / 2.0;
            let x =
                ((self.center.x as f64) + (self.radius as f64) / 3.0 * small_angle.cos()) as i32;
            let y =
                ((self.center.y as f64) + (self.radius as f64) / 3.0 * small_angle.sin()) as i32;

            Line::new(big_prev_point, Point::new(x, y)).draw(image);
            small_prev_point = Point::new(x, y);
        }
    }
}
