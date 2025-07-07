pub fn str_len(s: &str) -> usize {
    let mut count = 0;
    for _ in s.chars() {
        count += 1;
    }
    count
}