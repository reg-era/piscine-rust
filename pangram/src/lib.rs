pub fn is_pangram(s: &str) -> bool {
    let mut count = 0;
    let mut found = String::new();
    for c in s.chars() {
        if c.is_ascii_alphabetic() && !found.contains(&c.to_lowercase().to_string()) {
            count += 1;
            found.push_str(&c.to_lowercase().to_string());
        }
    }

    if count == 26 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{}",
            is_pangram("the quick brown fox jumps over the lazy dog!")
        );
        println!("{}", is_pangram("this is not a pangram!"));
    }
}
