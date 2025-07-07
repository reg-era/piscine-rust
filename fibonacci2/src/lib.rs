pub fn fibonacci(n: u32) -> u32 {
    let mut p1 = 0;
    let mut p2 = 1;

    if n == p1 || n == p2 {
        return n;
    }

    for _ in 2..=n {
        let temp = p1 + p2;
        p1 = p2;
        p2 = temp;
    }

    p2
}
