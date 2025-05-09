pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
    let mut count = 0;
    for c in v.chars() {
        if c == pat {
            return count;
        }
        count += 1;
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", is_empty(""));
        println!("{}", is_ascii("rust"));
        println!("{}", contains("rust", "ru"));
        println!("{:?}", split_at("rust", 2));
        println!("{}", find("rust", 'u'));
    }
}
