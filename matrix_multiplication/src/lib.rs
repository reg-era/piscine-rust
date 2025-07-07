#[derive(Debug, PartialEq, Eq)]
pub struct Matrix((i32, i32), (i32, i32));

pub fn multiply(m: Matrix, multiplier: i32) -> Matrix {
    Matrix(
        (m.0 .0 * multiplier, m.0 .1 * multiplier),
        (m.1 .0 * multiplier, m.1 .1 * multiplier),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix((1, 3), (4, 5));
        println!("Original matrix {:?}", matrix);
        println!("Matrix after multiply {:?}", multiply(matrix, 3));
    }

    #[test]
    fn test_multiply_1() {
        let m = Matrix((1, 2), (3, 4));
        let val = 5;
        assert_eq!(multiply(m, val), Matrix((5, 10), (15, 20)));
    }
    #[test]
    fn test_multiply_2() {
        let m = Matrix((1, 2), (3, 4));
        let val = -5;
        assert_eq!(multiply(m, val), Matrix((-5, -10), (-15, -20)));
    }
    #[test]
    fn test_multiply_3() {
        let m = Matrix((1, 2), (3, 4));
        let val = 0;
        assert_eq!(multiply(m, val), Matrix((0, 0), (0, 0)));
    }
}
