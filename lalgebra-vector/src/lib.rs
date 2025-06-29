pub mod scalar;
pub use scalar::Scalar;

pub use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar<T>>(pub Vec<T>);

impl<T: Scalar<T>> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut iter = self.0.iter().zip(other.0.iter());
        iter.next()
            .map(|(a, b)| *a * *b)
            .map(|first| iter.fold(first, |acc, (a, b)| acc + (*a * *b)))
    }
}

// Optional: implement Add for Vector<T>
impl<T: Scalar<T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            panic!("Cannot add vectors of different lengths");
        }
        let data = self
            .0
            .iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();
        Vector(data)
    }
}

use std::fmt::{self, Debug, Display};

// TODO: TestProperties to a lib
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Kind {
    Method,   // Makes the message firstInput.MethodName(inputs[1], input[2], ..])
    Operator, // Makes the message inputs[0] OperatorName inputs[1] ex: 1 + 2
    Function, // Makes the message FunctionName(inputs[0], inputs[1], inputs[2], ..)
    Value,
}

pub struct Inputs<'a>(pub &'a [Input]);

impl<'a> Display for Inputs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.0.iter().take(self.0.len() - 1) {
            write!(f, "{:?}, ", item)?;
        }
        write!(f, "{:?}", self.0[self.0.len() - 1])
    }
}

pub type Input = Box<dyn Debug>;

#[derive(Debug)]
pub struct TestProperties {
    pub name: &'static str,
    pub kind: Kind,
}

impl TestProperties {
    pub fn assert_with_message<U: std::fmt::Debug + std::cmp::PartialEq>(
        &self,
        inputs: &[Input],
        actual: U,
        expected: U,
    ) {
        let message_name = match (inputs.len(), self.kind) {
            (0, Kind::Function) => format!("{}()", self.name),
            (0, Kind::Value) => format!("{}", self.name),
            (0, _) => String::new(),
            (1, Kind::Method) => format!("{:?}.{}()", inputs[0], self.name),
            (1, Kind::Function) => format!("{}({:?})", self.name, inputs[0]),
            (1, Kind::Operator) => format!("{} {:?}", self.name, inputs[0]),
            (_, Kind::Function) => format!("{}({:?})", self.name, inputs),
            (_, Kind::Operator) => {
                format!("{:?} {} {}", inputs[0], self.name, Inputs(&inputs[1..]))
            }
            (_, Kind::Method) => {
                format!("{:?}.{}({})", inputs[0], self.name, Inputs(&inputs[1..]))
            }
            (_, Kind::Value) => {
                format!("{}.{}", Inputs(&inputs), self.name)
            }
        };
        assert_eq!(
            actual, expected,
            "\n\t`{}` == {:?}, expected == {:?}",
            message_name, actual, expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
        let meta_test = TestProperties {
            name: "dot",
            kind: Kind::Method,
        };
        let expected: i64 = 3;
        meta_test.assert_with_message(
            &[Box::new(vector_1.clone()), Box::new(vector_2.clone())],
            vector_1.dot(&vector_2),
            Some(expected),
        );

        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2]);
        meta_test.assert_with_message(
            &[Box::new(vector_1.clone()), Box::new(vector_2.clone())],
            vector_1.dot(&vector_2),
            None,
        );
    }

    #[test]
    fn addition() {
        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
        let test_meta = TestProperties {
            name: "+",
            kind: Kind::Operator,
        };
        test_meta.assert_with_message(
            &[Box::new(vector_1.clone()), Box::new(vector_2.clone())],
            vector_1 + vector_2,
            Some(Vector(vec![5i64, 1, -6])),
        );

        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![2, 4, -2, -1]);
        test_meta.assert_with_message(
            &[Box::new(vector_1.clone()), Box::new(vector_2.clone())],
            vector_1 + vector_2,
            None,
        );
    }
}
