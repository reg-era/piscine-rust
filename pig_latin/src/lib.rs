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
            // println!("sss {} {} {:?}", c,clock,iter);
            if is_vowel(c) {
                if c == 'u' && iter[i - 1] == 'q' && !text.starts_with(&"qu")  {
                    new_pref.push(c);
                } else {
                    // println!("break in {} {}", c,iter[i - 1]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", pig_latin(&String::from("igloo")));
        println!("{}", pig_latin(&String::from("apple")));
        println!("{}", pig_latin(&String::from("hello")));
        println!("{}", pig_latin(&String::from("square")));
        println!("{}", pig_latin(&String::from("xenon")));
        println!("{}", pig_latin(&String::from("chair")));
        println!("{}", pig_latin(&String::from("queen")));
    }
}
