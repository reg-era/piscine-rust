#[derive(Debug, Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            None
        } else {
            let old = self.clone();
            let next_value = if self.v % 2 == 0 {
                self.v / 2
            } else {
                3 * self.v + 1
            };
            self.v = next_value;
            Some(old)
        }
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    let collatz_iter = Collatz::new(n);
    collatz_iter.count()
}
