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
