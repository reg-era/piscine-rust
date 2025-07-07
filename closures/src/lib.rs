pub fn first_fifty_even_square() -> Vec<i32> {
    let mut res = Vec::new();
    let is_even = |n| n % 2 == 0;
    let mut i = 1;
    while res.len() < 50 {
        if is_even(i) {
            res.push(i * i);
        }
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Hello, world!");
        let v1 = first_fifty_even_square();

        println!("All elements in {:?}, len = {}", v1, v1.len());
    }
}
