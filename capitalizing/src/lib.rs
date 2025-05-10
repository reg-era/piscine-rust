pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if res.len() == 0 {
            res.push_str(&c.to_uppercase().to_string());
            continue;
        }
        res.push(c)
    }
    res
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    let mut upper_now = true;
    for c in input.chars() {
        if upper_now {
            res.push_str(&c.to_uppercase().to_string());
            upper_now = false;
            continue;
        } else if c == ' ' {
            upper_now = true;
        }
        res.push(c);
    }
    res
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();
    for c in input.chars() {
        if c.is_ascii_lowercase() {
            res.push_str(&c.to_uppercase().to_string());
        } else {
            res.push_str(&c.to_lowercase().to_string());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", capitalize_first("joe is missing"));
        println!("{}", title_case("jill is leaving A"));
        println!("{}", change_case("heLLo THere"));
    }
}
