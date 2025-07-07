use std::ops::Add;

#[derive(Debug)]
pub struct StepIterator<T> {
    current: T,
    end: T,
    step: T,
    done: bool,
}

impl<T> StepIterator<T>
where
    T: Copy + Add<Output = T> + PartialOrd,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        StepIterator {
            current: beg,
            end,
            step,
            done: false,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Copy + Add<Output = T> + PartialOrd,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current;

        if self.current >= self.end {
            self.done = true;
        } else {
            let next = self.current + self.step;
            if next > self.end {
                self.done = true;
            }
            self.current = next;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut step_iterator = StepIterator::new(0, 100, 10);
        assert_eq!(step_iterator.next(), Some(0));
        assert_eq!(step_iterator.next(), Some(10));
    }

    #[test]
    fn until_the_end() {
        for (i, v) in StepIterator::new(0, 100, 10).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i * 10, v);
        }
    }

    #[test]
    fn test_with_floats() {
        for (i, v) in StepIterator::new(0.0, 10.0, 0.5).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i as f64 * 0.5, v);
        }
    }

    #[test]
    fn test_with_floats_with_imperfect_range() {
        for (i, v) in StepIterator::new(0.3, 10.0, 0.5).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i as f64 * 0.5 + 0.3, v);
        }
    }
}
