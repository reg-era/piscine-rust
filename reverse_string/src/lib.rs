pub fn rev_str(input: &str) -> String {
    let mut collect = String::new();
    for c in input.chars().rev() {
        collect.push(c)
    }
    return collect;
}
