pub fn rev_str(input: &str) -> String {
    let mut collect = String::new();
    input.chars().rev().for_each(|c| collect.push(c));
    collect
}