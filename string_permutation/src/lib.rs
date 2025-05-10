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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
        let word1 = "thougth";

        println!(
            "Is {:?} a permutation of {:?}? = {}",
            word,
            word1,
            is_permutation(word, word1)
        );
    }

    #[test]
    fn test_basic() {
        assert!(is_permutation("abcde", "edbca"));
        assert!(!is_permutation("avcde", "edbca"));
        assert!(!is_permutation("cde", "edbca"));
        assert!(is_permutation("code", "deco"));
        assert!(!is_permutation("code", "deeco"));
        assert!(!is_permutation("codde", "deeco"));
    }

    #[test]
    fn test_repeating_characters() {
        assert!(is_permutation("aab", "baa"));
    }

    #[test]
    fn test_one_char() {
        assert!(!is_permutation("a", "b"));
        assert!(is_permutation("a", "a"));
    }

    #[test]
    fn test_empty() {
        assert!(is_permutation("", ""));
    }

    #[test]
    fn test_special_characters() {
        assert!(is_permutation("!#%@", "@%#!"));
    }

    #[test]
    fn test_unicode() {
        assert!(is_permutation("hello♥", "♥oelhl"));
        assert!(!is_permutation("♥", "♥♥"));
    }
}
