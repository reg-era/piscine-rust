#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(PartialEq)]
enum ColorePattern {
    ColorR,
    ColorG,
    ColorB,
    ColorA,
}

impl Color {
    fn find(&self, value: u8) -> ColorePattern {
        if value == self.r {
            ColorePattern::ColorR
        } else if value == self.g {
            ColorePattern::ColorG
        } else if value == self.b {
            ColorePattern::ColorB
        } else {
            ColorePattern::ColorA
        }
    }

    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let first_match = self.find(first);
        let second_match = self.find(second);

        match first_match {
            ColorePattern::ColorR => self.r = second,
            ColorePattern::ColorG => self.g = second,
            ColorePattern::ColorB => self.b = second,
            ColorePattern::ColorA => self.a = second,
        };

        match second_match {
            ColorePattern::ColorR => self.r = first,

            ColorePattern::ColorG => self.g = first,

            ColorePattern::ColorB => self.b = first,

            ColorePattern::ColorA => self.a = first,
        };

        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = Color {
            r: 255,
            g: 200,
            b: 10,
            a: 30,
        };

        println!("{:?}", c.swap(c.r, c.b));
        println!("{:?}", c.swap(c.r, c.g));
        println!("{:?}", c.swap(c.r, c.a));
        println!();
        println!("{:?}", c.swap(c.g, c.r));
        println!("{:?}", c.swap(c.g, c.b));
        println!("{:?}", c.swap(c.g, c.a));
        println!();
        println!("{:?}", c.swap(c.b, c.r));
        println!("{:?}", c.swap(c.b, c.g));
        println!("{:?}", c.swap(c.b, c.a));
        println!();
        println!("{:?}", c.swap(c.a, c.r));
        println!("{:?}", c.swap(c.a, c.b));
        println!("{:?}", c.swap(c.a, c.g));
    }
}
// Color { r: 10, g: 200, b: 255, a: 30 }
// Color { r: 200, g: 255, b: 10, a: 30 }
// Color { r: 30, g: 200, b: 10, a: 255 }
//
// Color { r: 200, g: 255, b: 10, a: 30 }
// Color { r: 255, g: 10, b: 200, a: 30 }
// Color { r: 255, g: 30, b: 10, a: 200 }
//
// Color { r: 10, g: 200, b: 255, a: 30 }
// Color { r: 255, g: 10, b: 200, a: 30 }
// Color { r: 255, g: 200, b: 30, a: 10 }
//
// Color { r: 30, g: 200, b: 10, a: 255 }
// Color { r: 255, g: 200, b: 30, a: 10 }
// Color { r: 255, g: 30, b: 10, a: 200 }
