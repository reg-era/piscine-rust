pub fn factorial(num: u64) -> u64 {
    if num == 1 || num == 0 {
        return 1;
    }
    let mut fact = num;
    let mut iter = num - 1;
    while iter > 0 {
        fact *= iter;
        iter -= 1;
    }
    return fact;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
        assert_eq!(factorial(19), 121645100408832000);
    }
}
