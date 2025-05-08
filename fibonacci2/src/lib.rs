pub fn fibonacci(n: u32) -> u32 {
    let mut p1 = 0;
    let mut p2 = 1;

    let mut step = 0;
    while step < n - 1 {
        let temp = p1 + p2;
        p1 = p2;
        p2 = temp;

        step += 1;
    }

    return p2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(22), 17711);
        assert_eq!(fibonacci(20), 6765);
    }
}
