pub fn sum(a: &[i32]) -> i32 {
    let mut count = 0;
    for n in a {
        count += n;
    }
    count
}

pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: [i32; 10] = std::array::from_fn(|i| (i + 1) as i32);
        let b = [5; 10];

        println!("ddd {:?}", a);

        println!("The sum of the elements in {:?} is {}", a, sum(&a));
        println!("The sum of the elements in {:?} is {}", b, sum(&b));
        println!(
            "Array of {} elements filled with 10 = {:?}",
            thirtytwo_tens().len(),
            thirtytwo_tens()
        );
    }
}
