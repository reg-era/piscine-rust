pub fn fibonacci(n: u32) -> u32 {
    let mut p1 = 0;
    let mut p2 = 1;

    if n == p1 || n == p2 {
        return n;
    }

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
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(22), 17711);
        assert_eq!(fibonacci(20), 6765);
    }
}
