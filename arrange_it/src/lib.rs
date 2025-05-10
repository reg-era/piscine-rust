pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: Vec<String> = vec![];
    let mut count = 1;

    while count - 1 < phrase.split(" ").count() {
        for word in phrase.split(" ") {
            let index = &count.to_string();
            if word.contains(index) {
                res.push(word.replace(index, ""));
            }
        }
        count += 1;
    }

    res.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", arrange_phrase("is2 Thi1s T4est 3a"));
    }
}
