pub fn score(word: &str) -> u64 {
    let mut res = 0;
    for c in word.to_uppercase().to_string().chars() {
        res += match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        };
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", score("a"));
        println!("{}", score("ã ê Á?"));
        println!("{}", score("ThiS is A Test"));
    }
}
