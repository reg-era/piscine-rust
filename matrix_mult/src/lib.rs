pub mod matrix;
pub mod scalar;

pub use matrix::*;
pub use scalar::*;

use std::iter::Sum;
use std::ops::Mul;

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut vec = Vec::new();
        for col in &self.0 {
            vec.push(col[n].clone());
            continue;
        }
        vec
    }
}

impl<T: Scalar<Item = T> + Mul<Output = T> + Add<Output = T> + Sum> Mul for Matrix<T> {
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows_self = self.0.len();
        let cols_self = self.0[0].len();
        let rows_rhs = rhs.0.len();
        let cols_rhs = rhs.0[0].len();

        if cols_self != rows_rhs {
            return None;
        }

        let mut res = Vec::new();

        for i in 0..rows_self {
            let mut ins_res = Vec::new();
            for j in 0..cols_rhs {
                let mut sum = Vec::new();
                for k in 0..cols_self {
                    sum.push(self.0[i][k].clone() * rhs.0[k][j].clone());
                }
                ins_res.push(sum.into_iter().sum());
            }
            res.push(ins_res);
        }

        Some(Self(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        println!("{:?}", matrix.col(0));
        println!("{:?}", matrix.row(1));

        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
        let mult = matrix_1.clone() * matrix_2.clone();
        println!("{:?}", mult);
        println!("{:?}", matrix_1.number_of_cols());
        println!("{:?}", matrix_2.number_of_rows());
    }
}
