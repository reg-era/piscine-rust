pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    if !s.starts_with(prefix) {
        None
    }else{
        let new_s = s.trim_start_matches(prefix);
        Some(&new_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", delete_prefix("ab", "abcdefghijklmnop"));
        println!("{:?}", delete_prefix("x", "abcdefghijklmnop"));
    }
}
