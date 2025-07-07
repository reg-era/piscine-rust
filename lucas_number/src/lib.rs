pub fn lucas_number(n: u32) -> u32 {
    if n == 0 {
        return 2;
    } else if n == 1 {
        return 1;
    }

    let mut a = 2;
    let mut b = 1;
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }

    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "The element in the position {} in Lucas Numbres is {}",
            2,
            lucas_number(2)
        );
        println!(
            "The element in the position {} in Lucas Numbres is {}",
            5,
            lucas_number(5)
        );
        println!(
            "The element in the position {} in Lucas Numbres is {}",
            10,
            lucas_number(10)
        );
        println!(
            "The element in the position {} in Lucas Numbres is {}",
            13,
            lucas_number(13)
        );
    }
}
