#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {
            center: Point::new(x, y),
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    pub fn intersect(&self, other: Circle) -> bool {
        self.center.distance(other.center) < self.radius + other.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    pub fn distance(&self, other: Point) -> f64 {
        f64::sqrt((other.0 - self.0).powi(2) + (other.1 - self.1).powi(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[inline]
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_new_circle() {
        let circle = Circle::new(500.0, 400.0, 150.0);
        assert!(approx_eq(circle.radius, 150.0));
        assert!(approx_eq(circle.center.0, 500.0));
        assert!(approx_eq(circle.center.1, 400.0));
    }

    #[test]
    fn test_distance() {
        let a = Point(0.0, 1.0);
        let b = Point(0.0, 0.0);
        assert!(approx_eq(a.distance(b), 1.0));

        let a = Point(1.0, 0.0);
        let b = Point(0.0, 0.0);
        assert!(approx_eq(a.distance(b), 1.0));

        let a = Point(1.0, 1.0);
        let b = Point(0.0, 0.0);
        assert!(approx_eq(a.distance(b), f64::sqrt(2.0)));
    }

    #[test]
    fn test_area() {
        let circle = Circle::new(500.0, 400.0, 150.0);
        assert!(approx_eq(circle.area(), 70685.83470577035));
    }

    #[test]
    fn test_intersection() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle::new(80.0, 115.0, 30.0);
        assert!(!circle.intersect(circle1));

        let circle = Circle::new(100.0, 300.0, 150.0);
        let circle1 = Circle::new(80.0, 115.0, 100.0);
        assert!(circle.intersect(circle1));
    }
}
