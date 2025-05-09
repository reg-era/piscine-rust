pub fn to_url(s: &str) -> String {
    let mut new_string = String::new();

    for c in s.chars() {
        if c == ' ' {
            new_string.push_str("%20");
        }else{
            new_string.push(c);
        }
    }

    new_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "Hello, world!";
        println!("'{}' parsed as an URL becomes '{}'", s, to_url(s));
    }
}
