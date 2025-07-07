pub fn capitalize_first(input: &str) -> String {
    let mut catch_first = true;
    input
        .chars()
        .map(|c| {
            if catch_first {
                catch_first = false;
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}

pub fn title_case(input: &str) -> String {
    let mut res = String::new();
    let mut upper_now = true;
    for c in input.chars() {
        if upper_now && c.is_alphabetic() {
            res.push_str(&c.to_uppercase().to_string());
            upper_now = false;
            continue;
        } else if c == ' ' || c == '\t' {
            upper_now = true;
        }
        res.push(c);
    }
    res
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_uppercase().to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_capitalize_first() {
    //     assert_eq!(capitalize_first("hello"), "Hello");
    //     assert_eq!(capitalize_first("this is working"), "This is working");
    // }

    #[test]
    fn test_title_case() {
        // assert_eq!(title_case("this is a title"), "This Is A Title");
        assert_eq!(
            title_case("hello my\t\tname is carl"),
            "Hello My\t\tName Is Carl"
        );
    }

    // #[test]
    // fn test_change_case() {
    //     assert_eq!(change_case("PROgraming"), "proGRAMING");
    //     assert_eq!(change_case("heLLo THere"), "HEllO thERE");
    // }

    // #[test]
    // fn test_empty() {
    //     assert_eq!(capitalize_first(""), "");
    //     assert_eq!(title_case(""), "");
    //     assert_eq!(change_case(""), "");
    // }
}
