pub fn rotate(input: &str, key: i8) -> String {
    let mut res = String::new();

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            let mut revc = c as i32 + key as i32;
            if c.is_ascii_lowercase() {
                if key < 0 {
                    revc += 26
                }
                if revc > 'z' as i32 {
                    revc -= 26;
                }
                res.push(char::from(revc as u8));
            } else {
                if key < 0 {
                    revc += 26
                }
                if revc > 'Z' as i32 {
                    revc -= 26;
                }
                res.push(char::from(revc as u8));
            }
        } else {
            res.push(c);
        }
    }

    res
}
