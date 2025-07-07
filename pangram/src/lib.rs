pub fn is_pangram(s: &str) -> bool {
    let all_letter = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    let mut find_letter: Vec<char> = Vec::new();

    s.to_uppercase()
        .chars()
        .filter(|c| {
            if all_letter.contains(&c) && !find_letter.contains(&c) {
                find_letter.push(*c);
                return true;
            }
            false
        })
        .count()
        == 26
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
