pub mod matrix;
pub mod scalar;

pub use matrix::*;
pub use scalar::*;

pub use std::ops::Add;
pub use std::ops::Sub;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix(vec![vec![8, 1], vec![9, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        println!("{:?}", matrix + matrix_2);

        let matrix = Matrix(vec![vec![1, 3], vec![2, 5]]);
        let matrix_2 = Matrix(vec![vec![3, 1], vec![1, 1]]);
        println!("{:?}", matrix - matrix_2);

        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        println!("{:?}", matrix - matrix_2);

        let matrix = Matrix(vec![vec![1, 3], vec![9, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        println!("{:?}", matrix + matrix_2);
    }
}
