pub mod scalar;
pub use scalar::*;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let m: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
        println!("{:?}", m);
        println!("{:?}", Matrix::<i32>::identity(4));
        println!("{:?}", Matrix::<f64>::zero(3, 4));
    }
}
