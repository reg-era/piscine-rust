pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut res = vec![String::new(); names.len()];
    let mut i = 0;
    for word in names {
        let mut new_word = String::new();
        for sp in word.split(" ") {
            new_word.push(sp.chars().nth(0).expect("wiiw"));
            new_word.push_str(". ");
        }
        let (splited, _) = new_word.split_at(new_word.len() - 1);
        res[i] = String::from(splited);
        i += 1;
    }
    res
}