pub fn to_url(s: &str) -> String {
    s.chars().map(|c| if c == ' ' { "%20".to_owned() }else{ c.to_string() }).collect()
}
