pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: Vec<String> = Vec::new();
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
