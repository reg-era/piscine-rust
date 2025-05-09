pub fn first_subword(mut s: String) -> String {
    let mut new_str = String::new();
    let mut i = 0;
    for c in s.chars() {
        if (i != 0 && c.is_ascii_uppercase()) || c == '_' {
            break;
        }
        new_str.push(c);
        i += 1;
    }
    s = new_str;
    return s;
}
