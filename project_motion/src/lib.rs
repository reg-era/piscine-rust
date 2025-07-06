#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        const GRAVITY: f32 = 9.8;

        // Stop if at or below ground
        if self.actual_position.y <= 0.0 && self.time > 0.0 {
            return None;
        }

        self.time += 1.0;

        // Raw values
        let t = self.time;
        let raw_vy = self.init_velocity.y - GRAVITY * t;
        let raw_y = self.init_position.y + self.init_velocity.y * t - 0.5 * GRAVITY * t * t;
        let raw_x = self.init_position.x + self.init_velocity.x * t;

        if raw_y < 0.0 {
            return None;
        }

        self.actual_position = Object {
            x: (raw_x * 10.0).round() / 10.0,
            y: (raw_y * 10.0).round() / 10.0,
        };
        self.actual_velocity = Object {
            x: (self.init_velocity.x * 10.0).round() / 10.,
            y: (raw_vy * 10.0).round() / 10.0,
        };

        Some(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_without_acelaration_velocity() {
        let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 45.1 },
                actual_velocity: Object { x: 0.0, y: -9.8 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 30.4 },
                actual_velocity: Object { x: 0.0, y: -19.6 },
                time: 2.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 5.9 },
                actual_velocity: Object { x: 0.0, y: -29.4 },
                time: 3.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_with_velocity() {
        let mut obj = ThrowObject::new(Object { x: 0.0, y: 50.0 }, Object { x: 10.0, y: 10.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 10.0, y: 55.1 },
                actual_velocity: Object { x: 10.0, y: 0.2 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 20.0, y: 50.4 },
                actual_velocity: Object { x: 10.0, y: -9.6 },
                time: 2.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 30.0, y: 35.9 },
                actual_velocity: Object { x: 10.0, y: -19.4 },
                time: 3.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 40.0, y: 11.6 },
                actual_velocity: Object { x: 10.0, y: -29.2 },
                time: 4.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_with_negative_velocity() {
        let mut obj = ThrowObject::new(Object { x: -10.0, y: 50.0 }, Object { x: -10.0, y: -10.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: -10.0, y: 50.0 },
                init_velocity: Object { x: -10.0, y: -10.0 },
                actual_position: Object { x: -20.0, y: 35.1 },
                actual_velocity: Object { x: -10.0, y: -19.8 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: -10.0, y: 50.0 },
                init_velocity: Object { x: -10.0, y: -10.0 },
                actual_position: Object { x: -30.0, y: 10.4 },
                actual_velocity: Object { x: -10.0, y: -29.6 },
                time: 2.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_with_zero() {
        let mut obj = ThrowObject::new(Object { x: 0.0, y: 0.0 }, Object { x: 0.0, y: 0.0 });

        assert_eq!(obj.next(), None);
    }
}
