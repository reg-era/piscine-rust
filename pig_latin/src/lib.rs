pub fn pig_latin(text: &str) -> String {
    fn is_vowel(c: char) -> bool {
        c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
    }

    if text.starts_with(|c| is_vowel(c)) {
        return String::from(format!("{}{}", text, "ay"));
    } else {
        let mut new_pref = String::new();
        let iter: Vec<char> = text.chars().collect();
        let mut clock = false;
        for (i, c) in iter.clone().into_iter().enumerate() {
            if is_vowel(c) {
                if c == 'u' && iter[i - 1] == 'q' && !text.starts_with(&"qu")  {
                    new_pref.push(c);
                } else {
                    break;
                }
            } else {
                new_pref.push(c);
            }
        }

        String::from(format!(
            "{}{}{}",
            text.strip_prefix(&new_pref).unwrap(),
            new_pref,
            "ay"
        ))
    }
}