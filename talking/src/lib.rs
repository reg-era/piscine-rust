pub fn talking(text: &str) -> &str {
    fn is_uppercase(text: &str) -> bool {
        let letters: String = text.chars().filter(|c| c.is_alphabetic()).collect();
        letters.len() > 0 && letters.chars().all(|c| c.is_uppercase())
    }

    if text.trim().is_empty() {
        return "Just say something!";
    } else if is_uppercase(text) && text.ends_with("?") {
        return "Quiet, I am thinking!";
    } else if is_uppercase(text) {
        return "There is no need to yell, calm down!";
    } else if text.ends_with("?") {
        return "Sure.";
    } else {
        return "Interesting";
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", talking("JUST DO IT!"));
        println!("{:?}", talking("Hello how are you?"));
        println!("{:?}", talking("WHAT'S GOING ON?"));
        println!("{:?}", talking("something"));
        println!("{:?}", talking(""));
    }
}
