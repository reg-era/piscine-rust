pub use super::*;

pub use std::ops::Add;
pub use std::ops::Sub;

#[derive(Clone,Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(Vec::new())
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut r = Vec::new();
        for _ in 0..row {
            let mut c = Vec::new();
            for _ in 0..col {
                c.push(<T as Scalar>::zero());
            }
            r.push(c);
        }
        Matrix(r)
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut r = Vec::new();
        for i in 0..n {
            let mut c = Vec::new();
            for j in 0..n {
                if i == j {
                    c.push(<T as Scalar>::one());
                } else {
                    c.push(<T as Scalar>::zero());
                }
            }
            r.push(c);
        }
        Matrix(r)
    }
}

impl<T: Scalar<Item = T> + Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut col = Vec::new();

        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }

            let mut row = Vec::new();

            for j in 0..self.0[i].len() {
                row.push(self.0[i][j].clone() + rhs.0[i][j].clone());
            }
            col.push(row);
        }

        Some(Matrix(col))
    }
}

impl<T: Scalar<Item = T> + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut col = Vec::new();

        for i in 0..self.0.len() {
            if self.0[i].len() != rhs.0[i].len() {
                return None;
            }

            let mut row = Vec::new();

            for j in 0..self.0[i].len() {
                row.push(self.0[i][j].clone() - rhs.0[i][j].clone());
            }
            col.push(row);
        }

        Some(Matrix(col))
    }
}