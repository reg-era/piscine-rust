use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut source = HashMap::new();
    for c1 in s1.chars() {
        source.insert(c1, s1.chars().filter(|c| *c == c1).count());
    }
    for c2 in s2.chars() {
        match source.get(&c2) {
            Some(old) => {
                if s2.chars().filter(|c| *c == c2).count() == *old {
                    continue;
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }
    true
}
