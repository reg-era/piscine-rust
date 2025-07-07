#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&'a self) -> &'a [u32] {
        &self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        if self.numbers.len() == 0 {
            return None;
        }
        let last = self.numbers.len() - 1;
        Some(self.numbers[last])
    }

    pub fn highest(&self) -> Option<u32> {
        if self.numbers.len() == 0 {
            return None;
        }
        let mut max = 0;
        for nb in self.numbers {
            if *nb > max {
                max = *nb;
            }
        }
        Some(max)
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut max = [0 as u32; 3];
        for &nb in self.numbers {
            if nb > max[0] {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = nb;
            } else if nb > max[1] {
                max[2] = max[1];
                max[1] = nb;
            } else if nb > max[2] {
                max[2] = nb;
            }
        }
        let mut res = Vec::new();
        for m in max {
            if m != 0 {
                res.push(m);
            }
        }
        res
    }
}
