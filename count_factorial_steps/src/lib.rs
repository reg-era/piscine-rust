pub fn count_factorial_steps(factorial: u64) -> u64 {
    let mut count = 1;
    let mut acc = 1;
    while acc < factorial {
        acc = acc * count;
        count += 1;
    }

    if acc == factorial {
        count - 1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "The factorial steps of 720 = {}",
            count_factorial_steps(720)
        );
        println!("The factorial steps of 13 = {}", count_factorial_steps(13));
        println!("The factorial steps of 6 = {}", count_factorial_steps(6));
    }
}
