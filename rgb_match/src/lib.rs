#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let mut dada = [self.r, self.g, self.b, self.a];
        let mut x = 0;
        let mut y = 0;
        for (i, da) in dada.iter().enumerate() {
            if *da == first {
                x = i;
            }
            if *da == second {
                y = i;
            }
        }
        dada.swap(x,y);
        self = Color {
            r: dada[0],
            g: dada[1],
            b: dada[2],
            a: dada[3],
        };
        self
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
