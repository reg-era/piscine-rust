pub fn stars(n: u32) -> String {
    String::from("*".repeat((2_i32).pow(n)as usize))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", stars(1));
        println!("{}", stars(4));
        println!("{}", stars(5));
    }
}
